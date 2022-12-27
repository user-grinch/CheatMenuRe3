#![allow(dead_code)]
use pdb::FallibleIterator;
use toy_arms::internal::Module;
use std::ffi::c_void;
use windows::Win32::System::Memory::{VirtualProtect, PAGE_PROTECTION_FLAGS, PAGE_EXECUTE_READWRITE};

const EXE_NAME : &str = "re3.exe";
const PDB_NAME : &str = "re3.pdb";

pub fn get_symbol_offset(sym_name : &str) -> u64 {
    let file = std::fs::File::open(PDB_NAME).ok().unwrap();
    let mut pdb = pdb::PDB::open(file).unwrap();

    let symbol_table = pdb.global_symbols().unwrap();
    let address_map = pdb.address_map().unwrap();
    
    let mut symbols = symbol_table.iter();
    while let Some(symbol) = symbols.next().unwrap() {
        match symbol.parse() {
            Ok(pdb::SymbolData::Public(data)) => {
                let rva = data.offset.to_rva(&address_map).unwrap_or_default();
                if data.name.to_string() == sym_name.to_string() {
                    let str = rva.to_string();
                    let without_prefix = str.as_str().trim_start_matches("0x");
                    let addr = u64::from_str_radix(without_prefix, 16).unwrap();
                    return addr;
                }
            }
            _ => {}
        }
    }
    return 0;
}

pub fn get_symbol_addr(sym_name : &str) -> u64 {
    let offset = get_symbol_offset(sym_name);
    let module = Module::from_module_name(EXE_NAME).unwrap();
    return module.module_base_address as u64 + offset;
}

pub fn get_struct_from_symbol<T>(sym_name : &str) -> &mut T {
    unsafe {
        let base_addr = get_symbol_addr(sym_name);
        let strct : &mut T = std::mem::transmute(base_addr);
        return strct;
    }
}


pub fn read_mem<T>(address : u64, virtual_protect : bool) -> T  {
    let _ptr: *mut T = address as *mut T;
    let mut protect = PAGE_PROTECTION_FLAGS(0);

    unsafe {
        if virtual_protect {
            VirtualProtect(address as *const c_void, 1, PAGE_EXECUTE_READWRITE, &mut protect);
        }
    
        let val : T = std::ptr::read(_ptr);
    
        if virtual_protect {
            VirtualProtect(address as *const c_void, 1, protect, &mut protect);
        }
        return val;
    }
}

pub fn write_mem<T>(address : u64, val : T, virtual_protect : bool)  {
    let _ptr: *mut T = address as *mut T;
    let mut protect = PAGE_PROTECTION_FLAGS(0);

    unsafe {
        if virtual_protect {
            VirtualProtect(address as *const c_void, 1, PAGE_EXECUTE_READWRITE, &mut protect);
        }
    
        std::ptr::write(_ptr, val);
    
        if virtual_protect {
            VirtualProtect(address as *const c_void, 1, protect, &mut protect);
        }
    }
}
