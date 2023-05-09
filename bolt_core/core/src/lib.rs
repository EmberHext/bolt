mod session;
mod utils;

use std::sync::{Arc, Mutex};

use bolt_common::prelude::*;

static VERSION: &str = "0.11.11";
static HELP: &str = r#"
Bolt CLI (Build and test APIs)

Usage:
  bolt [OPTIONS]...
  bolt -h | --help
  bolt -v | --version
Options:
  -h --help      Show this screen.
  -v --version   Show version.
  --reset        Reset static files
    "#;

static ADDRESS: &str = "127.0.0.1";

// Create a shared global state variable
lazy_static::lazy_static! {
    static ref CORE_STATE: Arc<Mutex<CoreState>> = Arc::new(Mutex::new(CoreState::new()));
}

pub struct CoreState {
    main_state: MainState,
    active_connections: Vec<String>,
}

impl CoreState {
    pub fn new() -> Self {
        Self {
            main_state: MainState::new(),
            active_connections: vec![],
        }
    }
}

pub fn start(args: Vec<String>, port: u16) {
    let mut args = args;

    args.remove(0);

    let mut is_tauri = false;
    let mut is_headless = false;
    let mut launch = false;
    let mut reset = false;

    match std::env::var_os("BOLT_DEV") {
        Some(_) => {
            reset = true;
        }
        None => {}
    }

    if args.len() > 0 {
        let flag = args[0].as_str();

        match flag {
            "--reset" => reset = true,

            "-h" | "--help" => {
                println!("{}", HELP);
            }

            "-v" | "--version" => {
                println!("bolt {}", VERSION);
            }

            "--tauri" => {
                is_tauri = true;

                launch = true;
            }

            "--headless" => {
                is_headless = true;

                launch = true;
            }

            _ => {
                panic!("unknown flag");
            }
        }
    } else {
        launch = true;
    }

    if reset {
        utils::reset_home();
    }

    if launch {
        utils::verify_home();
        utils::verify_state();

        if !is_tauri {
            utils::verify_dist();
        }

        if !is_tauri && !is_headless {
            std::thread::spawn(move || {
                session::asset::launch_asset_server(port + 1, ADDRESS.to_string());

                std::process::exit(0);
            });
        }

        start_services();

        session::server::launch_core_server(port, ADDRESS.to_string());
    }
}

fn start_services() {
    println!("Starting services");

    std::thread::spawn(move || {
        start_ws_service();
    });
}

fn start_ws_service() {
    std::thread::spawn(|| {
        // comment

        loop {
            let mut core_state = CORE_STATE.lock().unwrap();

            let connections = core_state.main_state.ws_connections.clone();
            let active_connections = core_state.active_connections.clone();

            println!("connections: {:?}", connections.len());
            println!("active: {:?}", active_connections.len());

            for con in connections.clone() {
                if !active_connections.contains(&con.connection_id) {
                    spawn_ws_service(con.clone());
                    core_state.active_connections.push(con.connection_id);
                }
            }

            for active_con in active_connections {
                let exists = connections.iter().any(|x| x.connection_id == active_con);

                if !exists {
                    stop_ws_service(active_con);
                }
            }

            drop(core_state);
            std::thread::sleep(std::time::Duration::from_millis(2000));
        }
    });
}

fn spawn_ws_service(con: WsConnection) {
    println!("started service for {}", con.connection_id);

    let handle = std::thread::Builder::new()
        .name(con.connection_id.clone())
        .spawn(move || {
            loop {
                // comment

                println!("POLL CON {}", con.connection_id);

                std::thread::sleep(std::time::Duration::from_millis(2000));
            }
        })
        .unwrap();
}

fn stop_ws_service(active_con: String) {
    println!("STOP {} service", active_con);
}
