#![feature(abi_thiscall)]
mod discord;
mod patch;
mod lurpc;
mod zone;

#[macro_use] extern crate log;
#[macro_use] extern crate lazy_static;
extern crate simplelog;
extern crate roxmltree; // i need xml stuff

// use rustcord::Rustcord;
use simplelog::*;
use std::fs::File;

use discord::run_discord_rpc;
use lurpc::patch_client;

use winapi::{
	shared::minwindef::{BOOL, DWORD, HINSTANCE, LPVOID, TRUE},
	um::libloaderapi::GetModuleHandleA
};

static mut BASE: usize = 0;

#[no_mangle]
pub extern "system" fn DllMain(
	_dll_module: HINSTANCE,
	call_reason: DWORD,
	_reserved: LPVOID)
	-> BOOL {
	const DLL_PROCESS_ATTACH: DWORD = 1;

	match call_reason {
		DLL_PROCESS_ATTACH => init(),
		_ => TRUE
	}
}

fn init() -> BOOL {
	let _ = WriteLogger::init(LevelFilter::Info, Config::default(), File::create("lurpc.log").unwrap());
	info!("Starting LURPC...");
	unsafe { BASE = GetModuleHandleA(std::ptr::null()) as usize; }
	run_discord_rpc();
	patch_client();
	TRUE
}

