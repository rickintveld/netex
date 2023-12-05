use std::env;

use crate::ftp::connection::Connection;

pub fn ftp_connection() -> Connection {
    let ftp_connection = Connection {
        host: env::var("netex.host").expect("Missing netex host"),
        username: env::var("netex.user").expect("Missing netex user"),
        password: env::var("netex.password").expect("Missing netex password"),
        download_dir: env::var("netex.downloadDirectory").expect("Missing netex download dir"),
        port: 21,
        regex: env::var("netex.regex").expect("Missing netex file regex"),
        provider: "netex".to_string(),
    };

    ftp_connection
}
