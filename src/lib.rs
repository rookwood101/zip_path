extern crate zip;

use zip::ZipWriter;
use zip::write::FileOptions;
use zip::result::ZipResult;
use zip::result::ZipError;

use std::io::Write;
use std::io::Seek;
use std::io::Error as IoError;
use std::io::ErrorKind as IoErrorKind;
use std::io::copy;

use std::path::Path;

use std::fs::File;

pub struct RecursiveZipWriter<W: Write + Seek> {
    zip_writer: ZipWriter<W>,
}

impl<W: Write + Seek> RecursiveZipWriter<W> {
    pub fn new(inner: W) -> Self {
        RecursiveZipWriter { zip_writer: ZipWriter::new(inner) }
    }

    pub fn add_path_renamed(&mut self, real_path: &Path, zip_path: &Path) -> Result<(), ZipError> {
        if real_path.is_file() {
            self.zip_writer
                .start_file(zip_path.to_string_lossy().into_owned(),
                            FileOptions::default())?;
            let mut file = File::open(real_path).unwrap();
            copy(&mut file, &mut self.zip_writer)?;
            Ok(())
        } else if real_path.is_dir() {
            for listing in real_path.read_dir().unwrap() {
                let file_name = listing.unwrap().file_name();
                self.add_path_renamed(&real_path.join(&file_name), &zip_path.join(&file_name))
                    .unwrap_or(());
            }
            Ok(())
        } else {
            Err(ZipError::Io(IoError::new(IoErrorKind::InvalidInput,
                                          "Cannot add non file/directory.")))
        }
    }

    pub fn add_path(&mut self, real_path: &Path) -> Result<(), ZipError> {
        self.add_path_renamed(real_path, &Path::new(real_path.file_name().unwrap()))
    }

    pub fn finish(&mut self) -> ZipResult<W> {
        self.zip_writer.finish()
    }
}
