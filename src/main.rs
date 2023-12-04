use std::fs::File;

use dotenvy::dotenv;
use types::connection::FtpConnection;

mod provider;
mod service;
mod types;

fn main() {
    dotenv().expect(".env file not found");

    let ftp_connection: FtpConnection = provider::netex::ftp_connection();

    let downloads: Vec<File> = service::file::download(ftp_connection);

    let extractions: Vec<File> = service::gzip::extract(downloads);

    println!("{:?}", extractions);
}
