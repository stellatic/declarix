use std::path::PathBuf;
use std::str::FromStr;
use std::{env, fs::hard_link, os::unix::fs::symlink};
use shared::copy_file;
use shared::Ops;
fn main()  {
    let args: Vec<String> = env::args().collect();
    match Ops::from_str(&args[1]).unwrap() {
        Ops::Symlink => {
            symlink(&args[2], &args[3]).unwrap()
        },
        Ops::Hardlink => {
            hard_link(&args[2], &args[3]).unwrap();
        },
        Ops::Copy => {
            copy_file(&PathBuf::from(&args[2]), &PathBuf::from(&args[3])).unwrap();
        }
        Ops::Create_Dir => {
            std::fs::create_dir(&args[2]).unwrap();
        },
        Ops::Rm_Dir => {
            std::fs::remove_dir(&args[2]).unwrap();
        }, 
        Ops::Rm_File => {
            std::fs::remove_file(&args[2]).unwrap();
        }
        _ => {}
    }
}