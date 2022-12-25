use imgui::*;
use crate::cheatmenu::PageInfo;
use crate::module::widgets;

pub const DISCORD_INVITE: &str =  "https://discord.gg/ZzW7kmf";
pub const GITHUB_LINK: &str = "https://github.com/user-grinch/Cheat-Menu";
pub const PATREON_LINK: &str = "https://www.patreon.com/grinch_";

// Returns pageinfo 
pub fn get() -> &'static PageInfo {
    static INFO: PageInfo = PageInfo {
        name: "Welcome",
        func: draw,
    };

    &INFO
}

fn draw(ui : &Ui) {
    widgets::text_centered(ui, "Welcome to CheatMenu");
    widgets::text_centered(ui, "Author: Grinch_");
    ui.new_line();
    ui.text_wrapped("Please ensure you have the latest version from GitHub.");

    let sz = widgets::calc_size(ui, 3, true);
    if ui.button_with_size("Discord server", sz) {
        open::that(DISCORD_INVITE).unwrap();
    }
    ui.same_line();
    if ui.button_with_size("GitHub", sz) {
        open::that(GITHUB_LINK).unwrap();
    }
    ui.same_line();
    if ui.button_with_size("Patreon", sz) {
        open::that(PATREON_LINK).unwrap();
    }
    
    ui.new_line();
    ui.text_wrapped("Be sure to inform me about bugs or suggestions & join the discord server for beta updates");
    ui.dummy([0.0, 20.0]);
    widgets::text_centered(ui, "If you like my work, consider donating :)");
    widgets::text_centered(ui, "Copyright Grinch_ 2022-2023. All rights reserved.");
}



