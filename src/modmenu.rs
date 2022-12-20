use hudhook::{hooks::{ImguiRenderLoop, ImguiRenderLoopFlags}};
use imgui::{Condition, Window};
use toy_arms::VirtualKeyCode;

use crate::utils::{get_symbol_addr, write_mem};
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
        if (self.shown) {
            Window::new("Cheat Menu by Grinch_")
                .size([350.0, 650.0], Condition::Once)
                .collapsible(false)
                .opened(&mut self.shown)
                .build(ui, || {
                    if ui.button("CHANGE MOENY") {
                        let addr = get_symbol_addr("?Players@CWorld@@2PAVCPlayerInfo@@A");
                        write_mem::<u32>(addr + 0xD0, 69);
                }
            });
        }

        if toy_arms::detect_keydown!(VirtualKeyCode::VK_LCONTROL) 
        && toy_arms::detect_keypress(VirtualKeyCode::VK_M) {
            self.shown = !self.shown;
        }
    }
}
