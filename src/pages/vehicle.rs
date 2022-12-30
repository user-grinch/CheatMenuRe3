use imgui::*;
use crate::cheatmenu::PageInfo;
use crate::sdk::playerinfo::CPlayerInfo;

// Returns pageinfo 
pub fn get() -> &'static PageInfo {
    static INFO: PageInfo = PageInfo {
        name: "Vehicle",
        func: draw,
    };

    &INFO
}

fn draw(ui : &Ui) {
    let tab_bar = ui.tab_bar("PLAYER_TAB");
    if tab_bar.is_some() {
        let tab_item = ui.tab_item("Checkbox");
        if tab_item.is_some() {
            ui.spacing();
            
            tab_item.unwrap().end();
        }

        let tab_item = ui.tab_item("Menu");
        if tab_item.is_some() {
            ui.spacing();
            
            tab_item.unwrap().end();
        }
        tab_bar.unwrap().end();
    }
}

