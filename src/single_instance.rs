use widestring::U16CString;
use windows::{
    core::PCWSTR,
    Win32::{
        Foundation::*,
        System::Threading::{CreateMutexExW, CREATE_MUTEX_INITIAL_OWNER},
    },
};

use crate::SINGLE_INSTANCE_MUTEX;

pub fn check_single_instance() -> bool {
    // run only single instance
    unsafe {
        let name = U16CString::from_str(SINGLE_INSTANCE_MUTEX).unwrap();

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
            return false;
        }

        if GetLastError() == ERROR_ALREADY_EXISTS {
            eprintln!("another instance is already running.");
            return false;
        }
    }

    println!("running the app...");
    return true;
}
