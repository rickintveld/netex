use chrono::{DateTime, Duration, Utc};
use ftp::FtpStream;
use regex::Regex;
use std::error::Error;
use std::fs::{self, File};
use std::io::{prelude::*, Cursor};
use std::path::Path;

use crate::types::connection::FtpConnection;

pub fn download(ftp_connection: FtpConnection) -> Vec<File> {
    let file_regex = Regex::new(&ftp_connection.regex).unwrap();
    let date_regex = Regex::new(r"(\d{8})_(\d{8})").unwrap();

    let mut ftp_stream = connect(&ftp_connection);

    login(&mut ftp_stream, &ftp_connection);

    change_directory(&mut ftp_stream, &ftp_connection);

    let files: Vec<String> = read_files_in_directory(&mut ftp_stream);

    let provider: &String = &ftp_connection.provider;
    let _ = prepare_provider_directory(provider);

    let date: DateTime<Utc> = Utc::now() - Duration::days(7);
    let minimum_date = date.format("%Y%m%d").to_string();
    let mut file_date: String = String::from("");

    let mut downloads: Vec<File> = vec![];

    for file in files.iter() {
        if file_regex.is_match(&file) {
            for date in date_regex.captures_iter(file) {
                file_date = date[2].to_string();
            }

            if file_date >= minimum_date {
                println!("Creating local file {}", file);

                let path = format!("{}/{}", &provider, file);

                let mut local_file = match File::create(&path) {
                    Err(error) => panic!("Could not create {}: {}", &path, error),
                    Ok(file) => file,
                };

                println!("Downloading file {}", file);
                let remote_file: Cursor<Vec<u8>> = ftp_stream.simple_retr(file).unwrap();

                let _ = local_file.write_all(&remote_file.into_inner());

                downloads.push(local_file);
            }
        }
    }

    let _ = ftp_stream.quit();

    downloads
}

fn connect(ftp_connection: &FtpConnection) -> FtpStream {
    FtpStream::connect(format!("{}:{}", ftp_connection.host, ftp_connection.port)).unwrap()
}

fn login(ftp_stream: &mut FtpStream, ftp_connection: &FtpConnection) {
    match ftp_stream.login(&ftp_connection.username, &ftp_connection.password) {
        Err(error) => println!("{:?}", error),
        Ok(()) => println!("Succesfully logged in."),
    };
}

fn change_directory(ftp_stream: &mut FtpStream, ftp_connection: &FtpConnection) {
    match ftp_stream.cwd(&ftp_connection.download_dir) {
        Err(error) => println!("{:?}", error),
        Ok(()) => println!("Change to directory: {}", &ftp_connection.download_dir),
    };
}

fn read_files_in_directory(ftp_stream: &mut FtpStream) -> Vec<String> {
    println!("Reading files in folder {}", ftp_stream.pwd().unwrap());

    let files: Vec<String> = match ftp_stream.nlst(Some(".")) {
        Err(error) => panic!("{:?}", error),
        Ok(files) => files,
    };

    files
}

fn prepare_provider_directory(provider: &str) -> Result<(), Box<dyn Error>> {
    let dir_exists = Path::new(provider).exists();

    if dir_exists {
        match fs::remove_dir_all(provider) {
            Err(error) => println!("{:?}", error),
            Ok(()) => println!("All files are removed"),
        };
    }

    match fs::create_dir(provider) {
        Err(error) => println!("Could not create provider dir: {:?}", error),
        Ok(()) => println!("The provider folder is created"),
    };

    Ok(())
}
