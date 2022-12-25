use imgui::*;
use crate::cheatmenu::PageInfo;
use crate::module::widgets;
use crate::page::welcome::{DISCORD_INVITE, GITHUB_LINK, PATREON_LINK};

// Returns pageinfo 
pub fn get() -> &'static PageInfo {
    static INFO: PageInfo = PageInfo {
        name: "Menu",
        func: draw,
    };

    &INFO
}

fn draw(ui : &Ui) {

    let tab_bar = ui.tab_bar("MENU_TAB");
    if tab_bar.is_some() {

        let tab_item = ui.tab_item("Config");
        if tab_item.is_some() {
            tab_item.unwrap().end();
        }

        let tab_item = ui.tab_item("About");
        if tab_item.is_some() {
            ui.spacing();

            let sz = widgets::calc_size(ui, 2, true);
            if ui.button_with_size("Check update", sz) {
                open::that(GITHUB_LINK).unwrap();
            }
            ui.same_line();
            if ui.button_with_size("Discord server", sz) {
                open::that(DISCORD_INVITE).unwrap();
            }

            if ui.button_with_size("GitHub", sz) {
                open::that(GITHUB_LINK).unwrap();
            }
            ui.same_line();
            if ui.button_with_size("Patreon", sz) {
                open::that(PATREON_LINK).unwrap();
            }

            ui.spacing();

            ui.columns(2, "", false);
            ui.text("Author: Grinch_");
            ui.next_column();
            ui.text("Version: 0.1-alpha");
            ui.columns(1, "", false);

            ui.new_line();
            ui.text_wrapped("Be sure to inform me about bugs or suggestions & join the discord server for beta updates");
            ui.dummy([0.0, 20.0]);
            widgets::text_centered(ui, "If you like my work, consider donating :)");
            widgets::text_centered(ui, "Copyright Grinch_ 2022-2023. All rights reserved.");
            tab_item.unwrap().end();
        }
        tab_bar.unwrap().end();
    }
}

