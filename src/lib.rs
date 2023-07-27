use std::{fs, io};
use std::io::Write;

fn rename(dirEntry: fs::DirEntry) {
    // let mut entries = fs::read_dir(".")?
    //     .map(|res| res.map(|e| e.path()))
    //     .collect::<Result<Vec<_>, io::Error>>()?;
}

#[cfg(test)]
mod tests {
    use std::ops::Add;
    use std::path::{Path, PathBuf};
    use super::*;
    use tempfile::{NamedTempFile, tempfile};

    fn create_temp_files(paths: Vec<&str>) {
        for path in paths {


            let dir = Path::new(path);
            match fs::create_dir(&dir) {
                Ok(_) => println!("Successfully created temp directory"),
                Err(e) => println!("Error creating temp directory: {}", e),
            }
            // Создаем и записываем в файл
            let mut file = match fs::File::create(path) {
                Ok(file) => file,
                Err(e) => panic!("Couldn't create file: {}", e),
            };
            let body = format!("I'm: {}", path);
            match file.write_all(body.as_bytes()) {
                Ok(_) => println!("Successfully wrote to file"),
                Err(e) => println!("Error writing to file: {}", e),
            }
        }
    }

    fn get_dirs_from_file_name (path: &str) -> String {
        let mut full_path = String::new();
        let dirs: Vec<&str> = path.split("/").collect();
        for (i, v) in dirs.iter().enumerate() {
            if i == dirs.len() - 1 {
                break
            }
            if full_path.len() != 0 {
                full_path.push_str("/");
            }
            full_path.push_str(v);
        }
        return full_path;
    }

    fn delete_folder(path: &str) {
        match fs::remove_dir_all(path) {
            Ok(_) => println!("Successfully removed directory"),
            Err(e) => println!("Error removing directory: {}", e),
        }
    }

    #[test]
    fn method_renames_file_if_file_exists() {
        let path = vec![
            "temp/Tate_no_Yuusha_no_Nariagari_[01]_[AniLibria]_[720p].mkv.txt",
            "temp/dir1/Tate_no_Yuusha_no_Nariagari_[01]_[AniLibria]_[720p].mkv.txt",
            "temp/dir2/Tate_no_Yuusha_no_Nariagari_[01]_[AniLibria]_[720p].mkv.txt",
            "temp/dir2/dir3/Tate_no_Yuusha_no_Nariagari_[01]_[AniLibria]_[720p].mkv.txt"
        ];
        create_temp_files(path);
        assert_eq!(1, 1);
        // delete_folder("temp");
    }

    #[test]
    fn method_returns_dirs_from_file_name() {
        assert_eq!(get_dirs_from_file_name("temp/Tate_no_Yuusha_no_Nariagari_[01]_[AniLibria]_[720p].mkv.txt"), "temp");
        assert_eq!(get_dirs_from_file_name("temp/dir1/Tate_no_Yuusha_no_Nariagari_[01]_[AniLibria]_[720p].mkv.txt"), "temp/dir1");
        assert_eq!(get_dirs_from_file_name("temp/dir1/dir2/Tate_no_Yuusha_no_Nariagari_[01]_[AniLibria]_[720p].mkv.txt"), "temp/dir1/dir2");
    }
}