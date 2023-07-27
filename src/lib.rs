use std::{fs, io};
use std::io::Write;

fn rename(dirEntry: fs::DirEntry) {
    // let mut entries = fs::read_dir(".")?
    //     .map(|res| res.map(|e| e.path()))
    //     .collect::<Result<Vec<_>, io::Error>>()?;
}

#[cfg(test)]
mod tests {
    use std::fs::FileType;
    use std::ops::Add;
    use std::path::{Path, PathBuf};

    use tempfile::{NamedTempFile, tempfile};

    use super::*;

    #[test]
    fn plexify_renames_existed_files() {
        // setup
        let path = vec![
            "temp/Tate_no_Yuusha_no_Nariagari_[01]_[AniLibria]_[720p].mkv.txt",
            "temp/dir1/Tate_no_Yuusha_no_Nariagari_[02]_[AniLibria]_[720p].mkv.txt",
            "temp/dir2/Tate_no_Yuusha_no_Nariagari_[03]_[AniLibria]_[720p].mkv.txt",
            "temp/dir2/dir3/Tate_no_Yuusha_no_Nariagari_[04]_[AniLibria]_[720p].mkv.txt",
        ];
        create_temp_files(&path);
        // when
        // then
        let result_files = enumerate_files_in_path("temp");
        // cleanup
        delete_folder("temp");
    }

    #[test]
    fn method_returns_path_from_file_name() {
        assert_eq!(get_path_from_file_name("temp/Tate_no_Yuusha_no_Nariagari_[01]_[AniLibria]_[720p].mkv.txt"), "temp");
        assert_eq!(get_path_from_file_name("temp/dir1/Tate_no_Yuusha_no_Nariagari_[01]_[AniLibria]_[720p].mkv.txt"), "temp/dir1");
        assert_eq!(get_path_from_file_name("temp/dir1/dir2/Tate_no_Yuusha_no_Nariagari_[01]_[AniLibria]_[720p].mkv.txt"), "temp/dir1/dir2");
    }

    #[test]
    fn method_returns_files_in_path() {
        let files = vec![
            "temp/file1.txt",
            "temp/dir1/file2.txt",
            "temp/dir1/dir2/file3.txt",
            "temp/dir3/file4.txt",
        ];
        create_temp_files(&files);
        let result_files = enumerate_files_in_path("temp");
        assert_eq!(result_files, files);
    }

    fn create_temp_files(paths: &[&str]) {
        for path in paths {
            let dir_path = get_path_from_file_name(path);
            let dir = Path::new(&dir_path);
            match fs::create_dir(&dir) {
                Ok(_) => (),
                Err(e) => println!("Error creating temp directory: {}", e),
            }
            // Создаем и записываем в файл
            let mut file = match fs::File::create(path) {
                Ok(file) => {

                    println!("Successfully created file {}", path);
                    file
                }
                Err(e) => panic!("Couldn't create file: {}", e),
            };
            let body = format!("I'm: {}", path);
            match file.write_all(body.as_bytes()) {
                Ok(_) => (),
                Err(e) => println!("Error writing to file: {}", e),
            }
        }
    }

    fn enumerate_files_in_path(path: &str) -> Vec<String> {
        return vec![];

        // let mut entries = fs::read_dir("temp").unwrap();
        // for entry in entries {
        //     match entry {
        //         Ok(it) => {
        //             if it.file_type().unwrap().is_file() {
        //                 println!("Name: {}", it.path().display())
        //             }
        //         },
        //         Err(e) => (),
        //     }
        // }
    }

    fn get_path_from_file_name(path: &str) -> String {
        let mut full_path = String::new();
        let dirs: Vec<&str> = path.split("/").collect();
        for (i, v) in dirs.iter().enumerate() {
            if i == dirs.len() - 1 {
                break;
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
            Ok(_) => println!("Successfully removed directory {}", path),
            Err(e) => println!("Error removing directory: {}", e),
        }
    }
}