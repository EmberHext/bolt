use std::path::Path;
use std::process::Command;
use std::time::SystemTime;
use std::time::UNIX_EPOCH;

use bolt_common::prelude::*;

// downloads the dist from github
pub fn build_dist() {
    println!("Downloading static files");

    #[cfg(debug_assertions)]
    _fetch_dist_dev();

    #[cfg(not(debug_assertions))]
    _fetch_dist_release();

    println!("Download complete");
}

fn _fetch_dist_release() {
    let download_link = "https://github.com/hiro-codes/bolt/releases/download/v".to_string()
        + crate::VERSION
        + "/dist.zip";

    _download_url(download_link, get_home() + "dist.zip");

    let file_path = get_home() + "dist.zip";
    let extract_path = get_home();

    _unzip(&file_path, &extract_path);
}

fn _fetch_dist_dev() {
    _copy_dir("../bolt_tauri/dist/", &(get_home() + "dist"));
}

fn _download_url(url: String, file_name: String) {
    let response = reqwest::blocking::get(url).unwrap();

    let mut file = std::fs::File::create(file_name).unwrap();

    let mut content = std::io::Cursor::new(response.bytes().unwrap());

    std::io::copy(&mut content, &mut file).unwrap();
}

fn _unzip(file_path: &String, extract_path: &String) {
    let file = std::fs::File::open(file_path).unwrap();
    let reader = std::io::BufReader::new(file);
    let mut archive = zip::ZipArchive::new(reader).unwrap();

    for i in 0..archive.len() {
        let mut file = archive.by_index(i).unwrap();
        let outpath = Path::new(extract_path).join(file.name());

        if (&*file.name()).ends_with('/') {
            std::fs::create_dir_all(&outpath).unwrap();
        } else {
            if let Some(p) = outpath.parent() {
                if !p.exists() {
                    std::fs::create_dir_all(&p).unwrap();
                }
            }

            let mut outfile = std::fs::File::create(&outpath).unwrap();
            std::io::copy(&mut file, &mut outfile).unwrap();
        }
    }
}

pub fn _clone_repo_dev() {
    let shell_command = format!(
        "rsync -a --exclude-from=.gitignore --exclude='.git' ./ {}",
        get_home() + "bolt/"
    );

    _run_command(shell_command, "../".to_string());
}

pub fn _clone_repo_release() {
    let url = "https://github.com/hiro-codes/bolt";

    let shell_command = format!("git clone {url} --depth 1");

    _run_command(shell_command, get_home());

    let shell_command = format!("git checkout release");

    _run_command(shell_command, get_home() + "bolt");
}

fn _copy_dir(src: &str, dst: &str) {
    let src = Path::new(&src);
    let dst = Path::new(&dst);

    if src.is_dir() {
        std::fs::create_dir_all(dst).unwrap();

        for entry in std::fs::read_dir(src).unwrap() {
            let entry = entry.unwrap();
            let path = entry.path();
            let new_path = dst.join(path.file_name().unwrap());

            if entry.file_type().unwrap().is_dir() {
                _copy_dir(path.to_str().unwrap(), new_path.to_str().unwrap());
            } else {
                std::fs::copy(&path, &new_path).unwrap();
            }
        }
    } else {
        std::fs::copy(src, dst).unwrap();
    }
}

pub fn _run_command(shell_command: String, dir: String) {
    let _output = if cfg!(target_os = "windows") {
        Command::new("cmd")
            .args(["/C", &shell_command])
            .current_dir(dir)
            .output()
            .expect(&format!("failed to execute command: {}", &shell_command))
    } else {
        Command::new("sh")
            .arg("-c")
            .arg(&shell_command)
            .current_dir(dir)
            .output()
            .expect(&format!("failed to execute command: {}", &shell_command))
    };
}

pub fn verify_state() {
    let path = get_home() + "state.json";
    if !file_exists(&path) {
        // println!("Creating state file");

        create_state(&path);
    }
}

pub fn file_exists(path: &String) -> bool {
    if Path::new(&path).exists() {
        true
    } else {
        // println!("File {} does not exist", path);
        false
    }
}

pub fn create_state(path: &String) {
    let new_state = MainState::new();

    let new_state = serde_json::to_string(&new_state).unwrap();
    std::fs::write(path, new_state).unwrap();
}

pub fn open_browser(link: String) {
    std::thread::sleep(std::time::Duration::from_secs(2));

    webbrowser::open(&link).unwrap();
}

pub fn reset_home() {
    println!("reseting home");

    let home_path = get_home();

    let _reset = match std::fs::remove_dir_all(home_path) {
        Ok(_) => {
            // println!("Deleted Bolt home")
        }
        Err(_err) => {
            // println!("could not delete Bolt home: {}", err)
        }
    };

    verify_home();
    verify_dist();

    println!("Home has been reset");
}

pub fn get_dist() -> String {
    get_home() + "dist/"
}

pub fn get_home() -> String {
    let path = dirs::home_dir().unwrap().to_str().unwrap().to_string() + "/bolt/";
    path
}

pub fn verify_home() {
    let path = get_home();
    if !dir_exists(&path) {
        // println!("Creating directory {}", path);
        create_home(&path);
    }
}

pub fn verify_dist() {
    let path = get_dist();
    if !dir_exists(&path) {
        // println!("Creating dist");
        build_dist();
    }
}

pub fn dir_exists(path: &String) -> bool {
    if Path::new(&path).exists() {
        true
    } else {
        // println!("Directory {} does not exist", path);
        false
    }
}

pub fn create_home(path: &String) {
    std::fs::create_dir(path).unwrap();
}

pub fn get_timestamp() -> u64 {
    let now = SystemTime::now();
    let since_epoch = now.duration_since(UNIX_EPOCH).expect("Time went backwards");

    since_epoch.as_millis() as u64
}
