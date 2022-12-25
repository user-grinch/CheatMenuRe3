use imgui::*;
use crate::cheatmenu::PageInfo;

// Returns pageinfo 
pub fn get() -> &'static PageInfo {
    static INFO: PageInfo = PageInfo {
        name: "Teleport",
        func: draw,
    };

    &INFO
}

fn draw(ui : &Ui) {
    ui.text("THIS IS A TELEPORT TEST");
}
