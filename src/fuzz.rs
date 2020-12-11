use std::path::{Path, PathBuf};
use std::fs::OpenOptions;

pub fn listdirs(dir: &Path, output_list: &mut Vec<(PathBuf, bool, bool)>) {
    let list = std::fs::read_dir(dir);

    if let Ok(list) = list {

        for entry in list {
            if let Ok(entry) = entry {

                let path = entry.path();

                if let Ok(metadata) = path.symlink_metadata() {

                    if metadata.file_type().is_symlink() {
                        continue;
                    }

                    if metadata.file_type().is_dir() {
                        listdirs(&path, output_list);
                    }

                    if metadata.file_type().is_file() {
                        let can_read = OpenOptions::new().read(true).open(&path).is_ok();

                        let can_write = OpenOptions::new().write(true).open(&path).is_ok();

                        output_list.push((path, can_read, can_write));
                    }
                }
            }
        }
    }
}