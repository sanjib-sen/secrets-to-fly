use std::env::args;
use std::fs::{metadata, read_to_string};
use std::process::{exit, Command, Stdio};
const WELL_KNOWN_ENV_FILES: [&str; 15] = [
    ".env",
    ".env.local",
    ".env.development",
    ".env.production",
    ".env.test",
    ".env.staging",
    ".env.ci",
    ".env.preview",
    ".env.development.local",
    ".env.production.local",
    ".env.test.local",
    ".env.staging.local",
    ".env.ci.local",
    ".env.dev",
    ".env.prod",
];

fn main() {
    let file = match args().nth(1) {
        Some(file) => file,
        None => {
            println!("No .env file name provided, Looking for common .env files");
            check_well_known_env_files()
        }
    };
    get_envs_from_file(file.as_str())
}

fn check_well_known_env_files() -> String {
    for file_name in WELL_KNOWN_ENV_FILES.iter() {
        if metadata(file_name).is_ok() {
            return file_name.to_string();
        }
    }
    println!("No well known .env file found, try with \"secrets-to-fly <.env-name>\"");
    exit(1)
}

fn get_envs_from_file(file: &str) {
    let mut envs: Vec<String> = Vec::new();
    let content = match read_to_string(file) {
        Ok(content) => content,
        Err(_) => return,
    };
    for line in content.lines() {
        if line.starts_with('#') {
            continue;
        }
        let env = line.split('=').collect::<Vec<&str>>();
        if env.len() != 2 {
            continue;
        }
        let key: String = env[0].replace("\"", "").replace("\'", "").replace(" ", "");
        let value: String = env[1].replace("\"", "").replace("\'", "").replace(" ", "");
        envs.push(format!("{}={}", key, value));
    }
    println!("Setting secrets for fly.io... Please wait...");
    let mut command = Command::new("fly")
        .arg("secrets")
        .arg("set")
        .arg(envs.join(" "))
        .arg("-a")
        .arg("comcord")
        .stdout(Stdio::inherit())
        .stderr(Stdio::inherit())
        .spawn()
        .unwrap();
    command.wait().unwrap();
}
