// Prevents additional console window on Windows in release
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use widestring::U16CString;
use windows::{
    core::PCWSTR,
    Win32::{
        Foundation::*,
        System::Threading::{CreateMutexExW, CREATE_MUTEX_INITIAL_OWNER},
    },
};

use netflix_skip::run_main;

fn main() {
    // run only single instance
    unsafe {
        let name = U16CString::from_str("Global\\MyRustAppSingleton").unwrap();

        let handle = CreateMutexExW(
            Some(std::ptr::null()),
            PCWSTR(name.as_ptr()),
            CREATE_MUTEX_INITIAL_OWNER,
            0,
        )
        .unwrap_or_else(|e| {
            eprintln!("failed to create mutex handle: {:?}", e);
            std::process::exit(1);
        });

        if handle.is_invalid() {
            eprintln!("failed to create mutex : invalid handle");
            return;
        }

        if GetLastError() == ERROR_ALREADY_EXISTS {
            eprintln!("another instance is already running.");
            return;
        }
    }

    println!("running the app...");
    run_main();
}
