#![allow(dead_code)]
use crate::modules::memory::get_symbol_addr;
use widestring::U16CString;
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
            type Func = extern "cdecl" fn(*const u16, bool);
            let v: Vec<u16> = message.encode_utf16().collect();
            let msg = U16CString::from_vec_unchecked(v);
            let func: Func = std::mem::transmute(addr); 
            func(msg.as_ptr(), quick);
        }
    }
	pub fn set_message(message: &str) {
        let addr = get_symbol_addr("?SetMessage@CHud@@SAXPEAG@Z");
        unsafe {
            type Func = extern "cdecl" fn(*const u16);
            let v: Vec<u16> = message.encode_utf16().collect();
            let msg = U16CString::from_vec_unchecked(v);
            let func: Func = std::mem::transmute(addr); 
            func(msg.as_ptr());
        }
    }
	pub fn set_big_message(message: &str, style: u16) {
        let addr = get_symbol_addr("?SetBigMessage@CHud@@SAXPEAGG@Z");
        unsafe {
            type Func = extern "cdecl" fn(*const u16, u16);
            let v: Vec<u16> = message.encode_utf16().collect();
            let msg = U16CString::from_vec_unchecked(v);
            let func: Func = std::mem::transmute(addr); 
            func(msg.as_ptr(), style);
        }
    }
}