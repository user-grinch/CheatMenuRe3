use imgui::*;
use crate::cheatmenu::PageInfo;
use crate::sdk::playerinfo::CPlayerInfo;

// Returns pageinfo 
pub fn get() -> &'static PageInfo {
    static INFO: PageInfo = PageInfo {
        name: "Player",
        func: draw,
    };

    &INFO
}

fn draw(ui : &Ui) {
    let tab_bar = ui.tab_bar("PLAYER_TAB");
    if tab_bar.is_some() {
        let pinfo = CPlayerInfo::get_mut();
        let tab_item = ui.tab_item("Checkbox");
        if tab_item.is_some() {
            ui.spacing();
            ui.columns(2, "", false);

            ui.checkbox("Fast reload", &mut pinfo.m_bFastReload);
            ui.checkbox("Infinite sprint", &mut pinfo.m_bInfiniteSprint);
            ui.checkbox("No jail fee", &mut pinfo.m_bGetOutOfJailFree);

            ui.next_column();

            ui.checkbox("No hospital fee", &mut pinfo.m_bGetOutOfHospitalFree);
            ui.checkbox("Remote mode", &mut pinfo.m_bInRemoteMode);

            ui.columns(1, "", false);
            tab_item.unwrap().end();
        }

        let tab_item = ui.tab_item("Menu");
        if tab_item.is_some() {
            ui.spacing();
            
            if ui.collapsing_header("Money", TreeNodeFlags::FRAMED) {
                ui.spacing();
                ui.input_int("Money", &mut pinfo.m_nMoney)
                    .chars_decimal(true)
                    .build();
                ui.spacing();
                ui.separator();
            }

            if ui.collapsing_header("Packages", TreeNodeFlags::FRAMED) {
                ui.spacing();
                ui.input_int("Collected packages", &mut pinfo.m_nCollectedPackages)
                    .build();
                ui.input_int("Total packages", &mut pinfo.m_nTotalPackages)
                    .build();
                ui.spacing();
                ui.separator();
            }
            
            tab_item.unwrap().end();
        }
        tab_bar.unwrap().end();
    }
}

