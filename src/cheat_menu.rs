use hudhook::{hooks::{ImguiRenderLoop, ImguiRenderLoopFlags}};
use imgui::{Condition, Window};
use toy_arms::VirtualKeyCode;

use crate::game_mem::{get_symbol_addr, write_mem, read_mem};
pub struct CheatMenu
{
    shown : bool,
}

impl CheatMenu {
    pub fn new() -> Self {
        #[cfg(debug_assertions)]
        {
            hudhook::utils::alloc_console();
            println!("Initializing...");
        }
        CheatMenu { shown: false }
    }

    pub fn process_mouse(&self) {
        // NOP psMouseSetPos
        let addr = get_symbol_addr("psMouseSetPos");
        unsafe {
            static mut DEF_BYTE1 : u8 = 0;
            if self.shown {
                DEF_BYTE1 = read_mem::<u8>(addr, true);
                write_mem::<u8>(addr, 0xC3, true);
            } else {
                write_mem::<u8>(addr, DEF_BYTE1, true);
            }
        }

        // CPad::GetPad(0)->DisablePlayerControls
        let addr = get_symbol_addr("?MakePlayerSafe@CPlayerInfo@@QEAAX_N@Z");
        unsafe {
            type MakePlayerSafe = extern "fastcall" fn(u64, bool);
            let func: MakePlayerSafe = std::mem::transmute(addr); 
            func(get_symbol_addr("?Players@CWorld@@2PAVCPlayerInfo@@A"), self.shown);
        }

        // call CPads::Update()
        let addr = get_symbol_addr("?UpdatePads@CPad@@SAXXZ");
        unsafe {
            type CPadsUpdate = extern "cdecl" fn();
            let func: CPadsUpdate = std::mem::transmute(addr); 
            func();
        }

        // Reset cursor positions
        let addr = get_symbol_addr("?NewMouseControllerState@CPad@@2VCMouseControllerState@@A");
        write_mem::<f32>(addr+0x8, 0.0, true); // NewMouseControllerState.x
        write_mem::<f32>(addr+0xC, 0.0, true); // NewMouseControllerState.y
    }
}

impl ImguiRenderLoop for CheatMenu {
    fn initialize(&mut self, _ctx: &mut imgui::Context) {
        let io = _ctx.io_mut();
        io.mouse_draw_cursor = true;
        
        let style = _ctx.style_mut();
        style.tab_border_size = 0.0;
        style.child_border_size = 0.0;
        style.frame_border_size = 0.0;
        style.popup_border_size = 0.0;
        style.window_border_size = 0.0;

        style.window_padding = [8.0, 8.0];
        style.window_rounding = 5.0;
        style.frame_padding = [8.0, 8.0];
        style.frame_rounding = 5.0;
        style.popup_rounding = 5.0;
        style.item_spacing = [7.0, 7.0];
        style.item_inner_spacing = [7.0, 7.0];
        style.indent_spacing = 25.0;
        style.scrollbar_size = 12.0;
        style.scrollbar_rounding = 10.0;
        style.grab_min_size = 5.0;
        style.grab_rounding = 3.0;
        style.window_title_align = [0.5, 0.5];
    }

    fn render(&mut self, ui: &mut imgui::Ui, _: &ImguiRenderLoopFlags) {

        if self.shown {
            Window::new("Cheat Menu by Grinch_")
                .size([350.0, 650.0], Condition::Once)
                .collapsible(false)
                .opened(&mut self.shown)
                .build(ui, || {
                    if ui.button("CHANGE MOENY") {
                        let addr = get_symbol_addr("?Players@CWorld@@2PAVCPlayerInfo@@A");
                        write_mem::<u32>(addr + 0xD0, 69, false);
                    }
                }
            );
        } else {
            ui.set_mouse_cursor(None);
        }
        
        unsafe {
            static mut MOUSE_SHOWN : bool = false;
            if self.shown != MOUSE_SHOWN {
                self.process_mouse();
                MOUSE_SHOWN = self.shown;
            }
        }

        if toy_arms::detect_keydown!(VirtualKeyCode::VK_LCONTROL) 
        && toy_arms::detect_keypress(VirtualKeyCode::VK_TAB) {
            self.shown = !self.shown;
        }
    }
}
