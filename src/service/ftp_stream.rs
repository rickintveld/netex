use ftp::FtpStream;

use crate::types::connection::FtpConnection;

pub fn create(ftp_connection: &FtpConnection) -> FtpStream {
    let mut ftp_stream = connect(&ftp_connection);

    login(&mut ftp_stream, &ftp_connection);

    change_directory(&mut ftp_stream, &ftp_connection);

    ftp_stream
}

pub fn read_files_in_directory(ftp_stream: &mut FtpStream) -> Vec<String> {
    println!("Reading files in folder {}", ftp_stream.pwd().unwrap());

    let files: Vec<String> = match ftp_stream.nlst(Some(".")) {
        Err(error) => panic!("{:?}", error),
        Ok(files) => files,
    };

    files
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
