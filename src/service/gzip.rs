use filepath::FilePath;
use flate2::bufread;
use std::io::copy;
use std::path::PathBuf;
use std::{fs::File, io::BufReader};

pub fn extract(files: Vec<File>) -> std::io::Result<()> {
    for file in files {
        let download: PathBuf = match file.path() {
            Err(error) => panic!("Could not get path for file: {:?}: {:?}", file, error),
            Ok(download) => download,
        };

        let path: String = download.display().to_string();

        println!("Extracting: {}", path);

        let output_file: &str = path
            .strip_suffix(".gz")
            .expect("Could not removed suffix GZ");

        let gzip: BufReader<File> = match File::open(&path) {
            Err(error) => panic!("Could not open file {}: {:?}", &path, error),
            Ok(file) => BufReader::new(file),
        };

        let mut xml: File = match File::create(output_file) {
            Err(error) => panic!("{:?}", error),
            Ok(file) => file,
        };

        let mut decoder: bufread::GzDecoder<BufReader<File>> = bufread::GzDecoder::new(gzip);

        copy(&mut decoder, &mut xml).unwrap();
    }

    Ok(())
}
