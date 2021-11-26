use winapi::shared::minwindef::{BYTE, DWORD, LPVOID};
use std::{ffi::c_void, ops::Index, usize, mem, ptr};

use log::info;

use lu_packets::world::client::LoadStaticZone;
use crate::BASE;
use crate::patch::*;
// use crate::zone::get_zone_name;

// Client login response packet
const CLIENT_LOGIN_RESPONSE: usize = 0x7339d2 + 1; // this is where we overwrite the below location
const ORIG_LOGIN_RESPONSE: usize = 0x732f90; // this is the location of the original function

const CLIENT_LOAD_ZONE: usize = 0x733a1d + 1;
const ORIG_LOAD_ZONE: usize = 0x72f170;

const CLIENT_TRANSFER_WORLD: usize = 0x733acc + 1;  // 00b33910
const ORIG_TRANSFER_WORLD: usize = 0x732c60;
// const ORIG_TRANSFER_WORLD: usize = 0x732c60;

// Client game message packet
const CLIENT_GAME_MSG: usize = 0x732a9a + 1;
const ORIG_GAME_MSG: usize = 0x72d570;

// PACKET IS ALWAYS USIZE

// goto 016339D2 for where we want to be
// CPU Disasm 014639D2  |.  E8 79906577   CALL mod::lurpc::login_response          ; \mod.mod::lurpc::login_response

// CPU Disasm 012639D2  |.  E8 B9F5FFFF   CALL 01262F90                            ; \legouniverse.01262F90
// 1262F90 we call the original which is this

// https://lcdruniverse.org/lu_packets/lu_packets/world/client/struct.LoadStaticZone.html


unsafe extern "thiscall" fn load_static_zone(this: usize, packet: i32, length: u32, param_3: i32) {
	let original_func = *(&(ORIG_LOAD_ZONE + BASE) as *const usize as *const extern "thiscall" fn(usize, i32, u32, i32));
	original_func(this, packet, length, param_3);
	info!("Load Static Zone received.");

	let test = Box::from_raw(packet as *mut LoadStaticZone);

	info!("packet: {:?}", test);
	// info!("packet: {:?}", packet);
	info!("length: {}", length);
	info!("param_3: {}", param_3);

	info!("Load Static Zone processed.");
	return;
}

// Until all the locations are known, this will have to do
pub fn patch_client() {
	call_patch(CLIENT_LOAD_ZONE, load_static_zone as LPVOID);
	// call_patch(CLIENT_LOGIN_RESPONSE, login_response as LPVOID);
	// call_patch(CLIENT_GAME_MSG, game_messages_general as LPVOID);
}
