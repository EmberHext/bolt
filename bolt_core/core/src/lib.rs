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

#[derive(Clone)]
struct WsService {
    connection_id: String,
    kill: bool,
}

pub struct CoreState {
    main_state: MainState,
    ws_services: Vec<WsService>,
}

impl CoreState {
    pub fn new() -> Self {
        Self {
            main_state: MainState::new(),
            ws_services: vec![],
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
    println!("Starting core services");

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
            let ws_services = core_state.ws_services.clone();

            // println!("connections: {:?}", connections.len());
            // println!("active: {:?}", active_connections.len());

            for ws_con in connections.clone() {
                let exists = ws_services
                    .iter()
                    .any(|x| x.connection_id == ws_con.connection_id);

                if !exists {
                    spawn_ws_service(ws_con.connection_id.clone());

                    core_state.ws_services.push(WsService {
                        connection_id: ws_con.connection_id,
                        kill: false,
                    });
                }
            }

            for service in ws_services {
                let exists = connections
                    .iter()
                    .any(|x| x.connection_id == service.connection_id);

                if !exists {
                    for sv in core_state
                        .ws_services
                        .iter_mut()
                        .filter(|sv_mut| sv_mut.connection_id == service.connection_id)
                    {
                        sv.kill = true;
                    }
                }
            }

            drop(core_state);
            std::thread::sleep(std::time::Duration::from_millis(200));
        }
    });
}

fn spawn_ws_service(connection_id: String) {
    // println!("started service for {}", con.connection_id);

    let _handle = std::thread::Builder::new()
        .name(connection_id.clone())
        .spawn(move || loop {
            let mut core_state = CORE_STATE.lock().unwrap();

            let mut kill_service = false;
            let mut service_index = 0;
            let mut con_index = 0;

            for (index, ws_service) in core_state
                .ws_services
                .iter()
                .enumerate()
                .filter(|(_, sv)| sv.connection_id == connection_id)
            {
                if ws_service.kill {
                    // println!("KILLED {}", conn.connection_id);

                    service_index = index;
                    kill_service = true;
                }
            }

            if kill_service {
                core_state.ws_services.remove(service_index);
                break;
            }

            for (index, con) in core_state.main_state.ws_connections.iter().enumerate() {
                if con.connection_id == connection_id {
                    con_index = index;
                }
            }

            let ws_con = &mut core_state.main_state.ws_connections[con_index];

            println!("POLL CON {}", ws_con.connection_id);

            drop(core_state);
            std::thread::sleep(std::time::Duration::from_millis(1000));
        })
        .unwrap();
}
