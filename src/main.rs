// Prevents additional console window on Windows in release
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use netflix_skip::run_main;

fn main() {
    run_main();
}
