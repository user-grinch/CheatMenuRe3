use hudhook::hooks::ImguiRenderLoop;
use hudhook::hooks::dx9::ImguiDx9Hooks;
use hudhook::reexports::*;
use hudhook::*;

mod modmenu;
mod utils;
use modmenu::CheatMenu;

#[no_mangle]
pub unsafe extern "stdcall" fn DllMain( hmodule: HINSTANCE, reason: u32, _: *mut std::ffi::c_void ){
    if reason == DLL_PROCESS_ATTACH {
        hudhook::lifecycle::global_state::set_module(hmodule);

        std::thread::spawn(move || {
            let hooks: Box<dyn hooks::Hooks> = 
            CheatMenu::new().into_hook::<ImguiDx9Hooks>();

            hooks.hook();
            hudhook::lifecycle::global_state::set_hooks(hooks);
        });
    }
}