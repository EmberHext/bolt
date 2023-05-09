use std::path::Path;
use std::process::Command;

use bolt_common::prelude::*;

// downloads the dist from github
pub fn build_dist() {
    // println!("Downloading static files");

    #[cfg(debug_assertions)]
    _clone_repo_dev();

    #[cfg(not(debug_assertions))]
    _clone_repo_release();

    let src = get_home() + "bolt/bolt_tauri/" + "./dist/";
    let dst = get_home() + "bolt/bolt_tauri/" + "../../dist";
    copy_dir(&src, &dst).unwrap();

    // println!("Download complete");
}

pub fn _clone_repo_dev() {
    let shell_command = format!(
        "rsync -a --exclude-from=.gitignore --exclude='.git' ./ {}",
        get_home() + "bolt/"
    );

    run_command(shell_command, "../".to_string());
}

pub fn _clone_repo_release() {
    let url = "https://github.com/hiro-codes/bolt";

    let shell_command = format!("git clone {url} --depth 1");

    run_command(shell_command, get_home());
}

fn copy_dir(src: &str, dst: &str) -> std::io::Result<()> {
    let src = Path::new(&src);
    let dst = Path::new(&dst);

    if src.is_dir() {
        std::fs::create_dir_all(dst)?;

        for entry in std::fs::read_dir(src)? {
            let entry = entry?;
            let path = entry.path();
            let new_path = dst.join(path.file_name().unwrap());

            if entry.file_type()?.is_dir() {
                copy_dir(path.to_str().unwrap(), new_path.to_str().unwrap())?;
            } else {
                std::fs::copy(&path, &new_path)?;
            }
        }
    } else {
        std::fs::copy(src, dst)?;
    }

    Ok(())
}

pub fn run_command(shell_command: String, dir: String) {
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
