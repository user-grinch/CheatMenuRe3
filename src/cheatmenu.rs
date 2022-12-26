use hudhook::{hooks::{ImguiRenderLoop, ImguiRenderLoopFlags}};
use imgui::{Condition, Window, Ui, ColorStackToken};
use toy_arms::VirtualKeyCode;
use simplelog::*;
use std::{fs::File};
use log::info;
use chrono::Datelike;
use crate::{page::*, sdk::playerinfo::CPlayerInfo};

use crate::module::widgets::{calc_size};
use crate::module::memory::{get_symbol_addr, write_mem, read_mem};
const VERSION_TITLE :&str = "Cheat Menu v0.1-alpha";
const LOG_NAME :&str = "CheatMenuRe3.log";

pub struct PageInfo {
    pub name: &'static str,
    pub func: fn(ui: &Ui),
}

pub struct CheatMenu
{
    visible : bool,
    page_list : [&'static PageInfo; 3],
    page_selected : &'static PageInfo,
}

impl CheatMenu {
    pub fn new() -> Self {

        // Setup the log
        WriteLogger::init(LevelFilter::Info, Config::default(), File::create(LOG_NAME).unwrap()).unwrap_or_default();
        info!("{}", VERSION_TITLE);
        info!("Copyright (c) 2022-2023, Grinch_");
        info!("Join discord https://discord.gg/ZzW7kmf");
        let date = chrono::Utc::now().date_naive();
        info!("Date: {}-{}-{}", date.day(), date.month(), date.year());

        return CheatMenu { 
            visible: false, 
            page_list: [teleport::get(), player::get(), menu::get()],
            page_selected : welcome::get(),
        }
    }

    /*
        Creates the top level navigation pages for the cheat menu
        Stacked buttons on the top 
    */
    pub fn create_pages(&mut self, ui: &Ui) {
        let spacing = ui.push_style_var(imgui::StyleVar::ItemSpacing([0.0, 0.0]));
        let rounding = ui.push_style_var(imgui::StyleVar::FrameRounding(0.0));

        let mut i :u32 = 0;
        for page in &mut self.page_list {

            let colors : ColorStackToken;

            if page.name == self.page_selected.name {
                colors = ui.push_style_color(imgui::StyleColor::Button, ui.style_color(imgui::StyleColor::ButtonActive));
            } else {
                colors  = ui.push_style_color(imgui::StyleColor::Button, ui.style_color(imgui::StyleColor::Button));
            }

            let sz = calc_size(ui, 3, false);
            if ui.button_with_size(page.name, sz) {
                self.page_selected = &page;
            }

            if page.name == self.page_selected.name {
                colors.pop();
            }

            if (i % 3) != 2 {
                ui.same_line();
            }
            i += 1;
        }

        rounding.pop();
        spacing.pop();
        ui.dummy([0.0, 10.0]);
        (self.page_selected.func)(ui);
    }

    /*
        Patches the game mouse
        Makes it visible, movable
    */
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
       CPlayerInfo::make_player_safe(self.visible);

        // // call CPads::Update()
        let addr = get_symbol_addr("?UpdatePads@CPad@@SAXXZ");
        unsafe {
            type CPadsUpdate = extern "cdecl" fn();
            let func: CPadsUpdate = std::mem::transmute(addr); 
            func();
        }

        // // Reset cursor positions
        let addr = get_symbol_addr("?NewMouseControllerState@CPad@@2VCMouseControllerState@@A");
        write_mem::<f32>(addr+0x8, 0.0, true); // NewMouseControllerState.x
        write_mem::<f32>(addr+0xC, 0.0, true); // NewMouseControllerState.y
    }
}

impl ImguiRenderLoop for CheatMenu {
    fn initialize(&mut self, _ctx: &mut imgui::Context) {
        let io = _ctx.io_mut();
        io.mouse_draw_cursor = true;

        _ctx.fonts().add_font(&[imgui::FontSource::TtfData {
            data: include_bytes!("C:/Windows/Fonts/trebucbd.ttf"),
            size_pixels: 16.0,
            config: None,
        }]);

        let style = _ctx.style_mut();
        style.tab_border_size = 0.0;
        style.child_border_size = 0.0;
        style.frame_border_size = 0.0;
        style.popup_border_size = 0.0;
        style.window_border_size = 0.0;

        style.window_padding = [5.0, 5.0];
        style.window_rounding = 5.0;
        style.frame_padding = [5.0, 5.0];
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

        style[imgui::StyleColor::Text] = [0.80, 0.80, 0.83, 1.00];
        style[imgui::StyleColor::Text] = [0.80, 0.80, 0.83, 1.00];
        style[imgui::StyleColor::TextDisabled] = [0.35, 0.33, 0.3, 1.00];
        style[imgui::StyleColor::WindowBg] = [0.06, 0.05, 0.06, 0.95];
        style[imgui::StyleColor::ChildBg] = [0.0, 0.0, 0.0, 0.0];
        style[imgui::StyleColor::PopupBg] = [0.06, 0.05, 0.06, 0.95];
        style[imgui::StyleColor::Border] = [0.12, 0.12, 0.12, 1.0];
        style[imgui::StyleColor::BorderShadow] = [0.20, 0.20, 0.20, 1.00];
        style[imgui::StyleColor::FrameBg] = [0.15, 0.15, 0.15, 0.95];
        style[imgui::StyleColor::FrameBgHovered] = [0.25, 0.25, 0.25, 1.00];
        style[imgui::StyleColor::FrameBgActive] = [0.20, 0.20, 0.20, 1.00];
        style[imgui::StyleColor::TitleBg] = [0.12, 0.12, 0.12, 0.94];
        style[imgui::StyleColor::TitleBgCollapsed] = [1.00, 0.98, 0.95, 0.75];
        style[imgui::StyleColor::TitleBgActive] = [0.07, 0.07, 0.09, 1.00];
        style[imgui::StyleColor::MenuBarBg] = [0.15, 0.15, 0.15, 0.95];
        style[imgui::StyleColor::ScrollbarBg] = [0.15, 0.15, 0.15, 0.95];
        style[imgui::StyleColor::ScrollbarGrab] = [0.5, 0.5, 0.5, 0.3];
        style[imgui::StyleColor::ScrollbarGrabHovered] = [0.7, 0.7, 0.7, 0.3];
        style[imgui::StyleColor::ScrollbarGrabActive] = [0.9, 0.9, 0.9, 0.3];
        style[imgui::StyleColor::CheckMark] = [0.80, 0.80, 0.83, 0.31];
        style[imgui::StyleColor::SliderGrab] = [0.80, 0.80, 0.83, 0.31];
        style[imgui::StyleColor::SliderGrabActive] = [0.80, 0.80, 0.83, 0.31];
        style[imgui::StyleColor::Separator] = [0.15, 0.15, 0.15, 0.95];
        style[imgui::StyleColor::Button] = [0.15, 0.15, 0.15, 0.95];
        style[imgui::StyleColor::ButtonHovered] = [0.25, 0.25, 0.25, 1.00];
        style[imgui::StyleColor::ButtonActive] = [0.20, 0.20, 0.20, 1.00];
        style[imgui::StyleColor::Tab] = [0.15, 0.15, 0.15, 0.95];
        style[imgui::StyleColor::TabHovered] = [0.25, 0.25, 0.25, 1.00];
        style[imgui::StyleColor::TabActive] = [0.20, 0.20, 0.20, 1.00];
        style[imgui::StyleColor::Header] = [0.0, 0.0, 0.0, 0.0];
        style[imgui::StyleColor::HeaderHovered] = [0.25, 0.25, 0.25, 1.00];
        style[imgui::StyleColor::HeaderActive] = [0.06, 0.05, 0.07, 1.00];
        style[imgui::StyleColor::ResizeGrip] = [0.12, 0.12, 0.12, 0.00];
        style[imgui::StyleColor::ResizeGripHovered] = [0.25, 0.25, 0.25, 1.00];
        style[imgui::StyleColor::ResizeGripActive] = [0.20, 0.20, 0.20, 1.00];
        style[imgui::StyleColor::PlotLines] = [0.40, 0.39, 0.38, 0.63];
        style[imgui::StyleColor::PlotLinesHovered] = [0.25, 1.00, 0.00, 1.00];
        style[imgui::StyleColor::PlotHistogram] = [0.40, 0.39, 0.38, 0.63];
        style[imgui::StyleColor::PlotHistogramHovered] = [0.25, 1.00, 0.00, 1.00];
        style[imgui::StyleColor::TextSelectedBg] = [0.06, 0.05, 0.06, 0.95];
        style[imgui::StyleColor::ModalWindowDimBg] = [0.20, 0.20, 0.20, 0.6];
    }

    fn render(&mut self, ui: &mut imgui::Ui, _: &ImguiRenderLoopFlags) {

        if self.visible {
            Window::new(VERSION_TITLE)
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