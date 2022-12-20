use pdb::FallibleIterator;
use toy_arms::internal::Module;
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

pub fn write_mem<T>(address : u64, val : T) {
    let _ptr: *mut T = address as *mut T;
    unsafe {
        std::ptr::write(_ptr, val);
    }
}