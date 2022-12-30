use imgui::*;
use crate::{cheatmenu::PageInfo, modules::memory::{get_symbol_addr, get_mut_ref}};

// Returns pageinfo 
pub fn get() -> &'static PageInfo {
    static INFO: PageInfo = PageInfo {
        name: "Game",
        func: draw,
    };

    &INFO
}

lazy_static! {
    static ref FAST_TIME: u64 =  get_symbol_addr("?gbFastTime@@3_NA");
    static ref GAME_HOUR: u64 =  get_symbol_addr("?ms_nGameClockHours@CClock@@2EA");
    static ref GAME_MINUTE: u64 =  get_symbol_addr("?ms_nGameClockMinutes@CClock@@2EA");
}

fn draw(ui : &Ui) {

    let tab_bar = ui.tab_bar("GAME_TAB");
    if tab_bar.is_some() {

        let tab_item = ui.tab_item("Checkbox");
        if tab_item.is_some() {
            ui.spacing();
            ui.checkbox("Fast time", get_mut_ref(*FAST_TIME));
            tab_item.unwrap().end();
        }
        let tab_item = ui.tab_item("Menu");
        if tab_item.is_some() {
            ui.spacing();

            if ui.collapsing_header("Time", TreeNodeFlags::FRAMED) {
                
                let mut hour: i32 = (*(get_mut_ref(*GAME_HOUR) as &mut i8)).try_into().unwrap();
                if ui.input_int("Game hour", &mut hour).build() {
                    *get_mut_ref(*GAME_HOUR) = hour;
                }

                let mut min: i32 = (*(get_mut_ref(*GAME_MINUTE) as &mut i8)).try_into().unwrap();
                if ui.input_int("Game minute", &mut min).build() {
                    *get_mut_ref(*GAME_HOUR) = min;
                }
                ui.spacing();
                ui.separator();
            }
            tab_item.unwrap().end();
        }
        tab_bar.unwrap().end();
    }
}

