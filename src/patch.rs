use winapi::shared::minwindef::{DWORD, LPVOID};
use winapi::um::memoryapi::VirtualProtect;
use winapi::um::winnt::PAGE_EXECUTE_READWRITE;

use crate::BASE;

// const CALL_OPCODE: u8 = 0xE8;
// const JMP_OPCODE: u8 = 0xE9;

const GET_POSSIBLE_LOGGER: usize = 0xcff860;
const CALL_LOGGER: usize = 0xced990; // memory offset and imagebase offset be different :zaqLost:

pub fn calc(address: usize) -> usize {
	unsafe { return address + BASE }
}

pub fn raw(address: usize) -> usize {
	unsafe { return address - BASE }
}

pub fn logger(level: i32, message: &str) { // this doesn't work. causes a crash. cant be bothered to fix
	unsafe {
		let possibly_get_logger = *(&(GET_POSSIBLE_LOGGER + BASE) as *const usize as *const extern "thiscall" fn() -> *const usize);
		let our_logger = possibly_get_logger();

		// let test = our_logger;

		info!("worked?");

		let call_logger = *(&(CALL_LOGGER + BASE) as *const usize as *const extern "thiscall" fn(*const extern "thiscall" fn(), i32, &str));
		call_logger(our_logger as *const extern "thiscall" fn(), level, message);

		info! ("finished");
	}
}

pub fn call_patch(src: usize, dst: LPVOID) {
	unsafe {
		let rel_distance: DWORD = dst as DWORD - (src + BASE) as DWORD - 4;
		patch(src, rel_distance);
	}
}

// pub fn detour(old: usize, new: LPVOID) {
// 	unsafe {
// 		let old = (old + BASE) as LPVOID;
// 		let jmp_distance: DWORD = new as DWORD - old as DWORD - 5;
// 		let mut old_protect: DWORD = PAGE_EXECUTE_READWRITE;
// 		VirtualProtect(old, 5, PAGE_EXECUTE_READWRITE, &mut old_protect);
// 		*(old as *mut BYTE) = JMP_OPCODE;
// 		*(((old as usize)+1) as *mut DWORD) = jmp_distance;
// 		VirtualProtect(old, 5, old_protect, &mut old_protect);
// 	}
// }

pub fn patch<T>(dst: usize, new: T) {
	unsafe {
		let dst = (dst + BASE) as LPVOID;
		let mut old_protect: DWORD = PAGE_EXECUTE_READWRITE;
		VirtualProtect(dst, std::mem::size_of::<T>(), PAGE_EXECUTE_READWRITE, &mut old_protect);
		*(dst as *mut T) = new;
		VirtualProtect(dst, std::mem::size_of::<T>(), old_protect, &mut old_protect);
	}
}
