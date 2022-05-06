use std::fs;
use std::io::Write;
use std::process::Command;
use std::path::Path;

const TTARCHEXT: &[u8; 977920] = include_bytes!("ttarchext.exe");

pub struct Ttarchext {
    ttarchext_exe: String
}

impl Ttarchext {
    pub fn new() -> Self {
        let temp_file = std::env::temp_dir().join(format!("{}_ttarchext.exe", std::process::id()));
        let new = Ttarchext {
            ttarchext_exe: temp_file.to_str().unwrap().to_string(),
        };

        fs::File::create(&new.ttarchext_exe).unwrap().write_all(TTARCHEXT).unwrap();

        new
    }

    pub fn execute(&self, args: &[&str]) {
        let mut command = Command::new(&self.ttarchext_exe);
        command.args(args);
        let output = command.output().unwrap();
        if output.status.code() != Some(0) {
            panic!("{:?}", output);
        }
    }

    pub fn archive_to_folder(&self, archive: &Path, folder: &Path) {
        self.execute(&["67", &archive.to_str().unwrap(), &folder.to_str().unwrap()]);
    }

    pub fn folder_to_archive(&self, folder: &Path, archive: &Path) {
        fs::remove_file(&archive).unwrap();
        self.execute(&["-b", "67", &archive.to_str().unwrap(), &folder.to_str().unwrap()]);
    }
}