#![allow(dead_code)]
use crate::module::memory::get_symbol_addr;

pub(crate) struct CHud {

}

impl CHud {
    pub fn get_rid_of_all_hud_messages() {
        let addr = get_symbol_addr("?GetRidOfAllHudMessages@CHud@@SAXXZ");
        unsafe {
            type Func = extern "cdecl" fn();
            let func: Func = std::mem::transmute(addr); 
            func()
        }
    }

	pub fn set_help_message(message: &str, quick: bool) {
        let addr = get_symbol_addr("?SetHelpMessage@CHud@@SAXPEAG_N@Z");
        unsafe {
            type Func = extern "cdecl" fn(&[u8], bool);
            let str: String = String::from(message) + "\0";
            let func: Func = std::mem::transmute(addr); 
            func(str.as_bytes(), quick);
        }
    }
	pub fn set_message(message: &str) {
        let addr = get_symbol_addr("?SetMessage@CHud@@SAXPEAG@Z");
        unsafe {
            type Func = extern "cdecl" fn(&[u8]);
            let str: String = String::from(message) + "\0";
            let func: Func = std::mem::transmute(addr); 
            func(str.as_bytes());
        }
    }
	pub fn set_big_message(message: &str, style: u16) {
        let addr = get_symbol_addr("?SetBigMessage@CHud@@SAXPEAGG@Z");
        unsafe {
            type Func = extern "cdecl" fn(&[u8], u16);
            let str: String = String::from(message) + "\0";
            let func: Func = std::mem::transmute(addr); 
            func(str.as_bytes(), style);
        }
    }
}