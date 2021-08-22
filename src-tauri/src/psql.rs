use std::process::{Command, Child, Stdio};
use std::io::{Read};
use whoami::{username};

const PGSQL_VERSION: &'static str = "11";

pub enum CtlStatusResponse {
  NoServerRunning,
  ServerRunning,
  NoResponse
}

pub enum CtlStartResponse {
  ServerStarted,
  NoResponse
}

pub enum CtlStopResponse {
  NoServerToStop,
  NoResponse
}

#[allow(dead_code)]
fn get_bin() -> String {
  format!("/usr/lib/postgresql/{}/bin/postgres", PGSQL_VERSION)
}
  
fn get_ctl() -> String {
  format!("/usr/lib/postgresql/{}/bin/pg_ctl", PGSQL_VERSION)
}
  
fn get_pgdata() -> String {
  format!("/home/{}/databases/pgsql_data", username())
}

pub fn start_cmd() {
  let mut child = Command::new(get_ctl())
    .arg("start")
    .arg("-D")
    .arg(get_pgdata())
    .spawn().expect("errr");
  child.wait().expect("wait err ");
}
  
  
pub fn stop_cmd() {
  let mut child = Command::new(get_ctl())
    .arg("stop")
    .arg("-D")
    .arg(get_pgdata())
    .spawn().expect("errr");
  child.wait().expect("wait err ");
}
  
  
pub fn status_cmd() -> CtlStatusResponse {

  let no_server_running = String::from("pg_ctl: no server running\n");

  if let Ok(cmd) = Command::new(get_ctl())
    .arg("status")
    .arg("-D")
    .arg(get_pgdata())
    .stdout(Stdio::piped())
    .spawn() {
    
    let mut result = String::new();
    
    match cmd.stdout.unwrap().read_to_string(&mut result) {
        Err(why) => panic!("couldn't read status stdout: {}", why),
        Ok(_) => print!(""),
    };

    if result.eq(&no_server_running) {
      return CtlStatusResponse::NoServerRunning
    } else if result.contains("pg_ctl: server is running") {
      return CtlStatusResponse::ServerRunning
    }
  }
  
  CtlStatusResponse::NoResponse
}