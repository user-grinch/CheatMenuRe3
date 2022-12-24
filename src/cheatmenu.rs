use hudhook::{hooks::{ImguiRenderLoop, ImguiRenderLoopFlags}};
use imgui::{Condition, Window, Ui, ColorStackToken};
use toy_arms::VirtualKeyCode;
use simple_log::*;

use crate::{sdk::playerinfo::{CPlayerInfo}, memory::get_struct_from_symbol, widgets};
use crate::widgets::{calc_size};
use crate::memory::{get_symbol_addr, write_mem, read_mem};

const VERSION_NUMBER :f32 = 0.1;
const LOG_NAME :&str = "CheatMenuRe3.log";

pub struct CheatMenu
{
    visible : bool,
    headers : [String; 3],
    header_selected : u32,
}

impl CheatMenu {
    pub fn new() -> Self {
        std::fs::remove_file(LOG_NAME)
            .expect("Log doesn't exist");

        let config = LogConfigBuilder::builder()
            .path(LOG_NAME)
            .size(1 * 100)
            .roll_count(10)
            .time_format("%H:%M:%S")
            .level("info")
            .output_file()
            .build();

        simple_log::new(config).unwrap();
        info!("Cheat Menu v{}-alpha", VERSION_NUMBER);
        info!("Copyright (c) 2022-2023, Grinch_");
        info!("Join the discord server https://discord.gg/ZzW7kmf");

        return CheatMenu { 
                visible: false, 
                headers: [
                    "Teleport".to_string(), "Player".to_string(), "Menu".to_string()
                ],
                header_selected : 0,
            }
    }

    /*
        Creates the top level navigation pages for the cheat menu
        Stacked buttons on the top 
    */
    pub fn create_pages(&mut self, ui: &Ui) {
        ui.spacing();
        let spacing = ui.push_style_var(imgui::StyleVar::ItemSpacing([0.0, 0.0]));
        let rounding = ui.push_style_var(imgui::StyleVar::FrameRounding(0.0));

        for i in 0..self.headers.len() {

            let colors : ColorStackToken;

            if i == self.header_selected as usize {
                colors = ui.push_style_color(imgui::StyleColor::Button, ui.style_color(imgui::StyleColor::ButtonActive));
            } else {
                colors  = ui.push_style_color(imgui::StyleColor::Button, ui.style_color(imgui::StyleColor::Button));
            }

            if ui.button_with_size(self.headers[i].as_str(), calc_size(ui, 3, false)) {
                self.header_selected = i as u32;
            }

            if i == self.header_selected as usize {
                colors.pop();
            }

            if (i % 3) != 2 {
                ui.same_line();
            }
        }

        rounding.pop();
        spacing.pop();
    }

    pub fn process_mouse(&self) {
        // NOP psMouseSetPos
        let addr = get_symbol_addr("psMouseSetPos");
        unsafe {
            static mut DEF_BYTE1 : u8 = 0;
            if self.visible {
                DEF_BYTE1 = read_mem::<u8>(addr, true);
                write_mem::<u8>(addr, 0xC3, true);
            } else {
                write_mem::<u8>(addr, DEF_BYTE1, true);
            }
        }

        // Disable controls
        let addr = get_symbol_addr("?MakePlayerSafe@CPlayerInfo@@QEAAX_N@Z");
        unsafe {
            type MakePlayerSafe = extern "fastcall" fn(u64, bool);
            let func: MakePlayerSafe = std::mem::transmute(addr); 
            func(get_symbol_addr("?Players@CWorld@@2PAVCPlayerInfo@@A"), self.visible);
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

        style.window_padding = [4.0, 4.0];
        style.window_rounding = 5.0;
        style.frame_padding = [4.0, 4.0];
        style.frame_rounding = 5.0;
        style.popup_rounding = 5.0;
        style.item_spacing = [4.0, 4.0];
        style.item_inner_spacing = [4.0, 4.0];
        style.indent_spacing = 25.0;
        style.scrollbar_size = 12.0;
        style.scrollbar_rounding = 10.0;
        style.grab_min_size = 5.0;
        style.grab_rounding = 3.0;
        style.window_title_align = [0.5, 0.5];
    }

    fn render(&mut self, ui: &mut imgui::Ui, _: &ImguiRenderLoopFlags) {

        if self.visible {
            Window::new("Cheat Menu by Grinch_")
                .size([350.0, 650.0], Condition::Once)
                .collapsible(false)
                // .opened(&mut (self.shown))
                .build(ui, || {
                    self.create_pages(ui);
                }
            );
        } else {
            ui.set_mouse_cursor(None);
        }
        
        unsafe {
            static mut MOUSE_SHOWN : bool = false;
            if self.visible != MOUSE_SHOWN {
                self.process_mouse();
                MOUSE_SHOWN = self.visible;
            }
        }

        if toy_arms::detect_keydown!(VirtualKeyCode::VK_LCONTROL) 
        && toy_arms::detect_keypress(VirtualKeyCode::VK_TAB) {
            self.visible = !self.visible;
        }
    }
}
