use std::fs::File;

use dotenvy::dotenv;
use types::connection::FtpConnection;

mod provider;
mod service;
mod types;

fn main() {
    dotenv().expect(".env file not found");

    let ftp_connection: FtpConnection = provider::netex::ftp_connection();

    let _ = service::file::prepare_provider_directory(&ftp_connection.provider);

    let mut ftp_stream: ftp::FtpStream = service::ftp_stream::create(&ftp_connection);

    let downloads: Vec<File> = service::file::download(&mut ftp_stream, ftp_connection);

    let extractions: Vec<File> = service::gzip::extract(downloads);

    println!("{:?}", extractions);
}
