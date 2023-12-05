use std::fs::File;

use ::ftp::FtpStream;
use dotenvy::dotenv;

use crate::ftp::connection::Connection;

mod ftp;
mod provider;
mod service;

fn main() {
    dotenv().expect(".env file not found");

    let ftp_connection: Connection = provider::netex::ftp_connection();

    let _ = service::file::prepare_provider_directory(&ftp_connection.provider);

    let mut ftp_stream: FtpStream = service::ftp_stream::create(&ftp_connection);

    let downloads: Vec<File> = service::file::download(&mut ftp_stream, ftp_connection);

    let extractions: Vec<File> = service::gzip::extract(downloads);

    println!("{:?}", extractions);
}
