// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use serde::Serialize;
use tauri::api::process::{Command, Output};

// Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}

fn interprete_py_string(source: &str) -> Output {
    let output = Command::new_sidecar("rustpython")
        .expect("failed to create `rustpython` binary command")
        .args(["-c", source])
        .output()
        .expect("failed to run `rustpython`");

    output
}

#[derive(Debug, Serialize)]
struct InterpretResult {
    stdout: String,
    stderr: String,
    exit_code: i32,
}

#[tauri::command]
fn run_code(source: &str) -> InterpretResult {
    let out = interprete_py_string(source);
    // let stdres;
    let exit_code = out.status.code();
    // match exit_code {
    //     Some(0) => {
    //         stdres = out.stdout;
    //     }
    //     Some(_code) => {
    //         stdres = out.stderr;
    //     }
    //     None => {
    //         stdres = out.stderr;
    //     }
    // };
    InterpretResult {
        stdout: out.stdout,
        stderr: out.stderr,
        exit_code: exit_code.unwrap_or(-1),
    }
}

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![greet, run_code])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
