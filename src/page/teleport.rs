use imgui::*;
use crate::cheatmenu::PageInfo;
use crate::module::widgets;
use crate::sdk::playerinfo::CPlayerInfo;
use crate::sdk::hud::CHud;

// Returns pageinfo 
pub fn get() -> &'static PageInfo {
    static INFO: PageInfo = PageInfo {
        name: "Teleport",
        func: draw,
    };

    &INFO
}

fn draw(ui : &Ui) {
    unsafe {
        static mut BUF: String = String::new();

        ui.input_text("Coordinates", &mut BUF)
            .build();

        ui.spacing();

        let sz = widgets::calc_size(ui, 2, true);
        if ui.button_with_size("Clear", sz) {
            BUF = "".to_string();
        }
        ui.same_line();
        if ui.button_with_size("Teleport", sz) {
            let mut coords : Vec<&str> = BUF.split(", ").collect();

            if coords.len() == 0 {
                coords = BUF.split(",").collect();
            }

            let (temp_x, temp_y, temp_z) = (coords.get(0), coords.get(1), coords.get(2));

            if temp_x.is_some() && temp_y.is_some() && temp_z.is_some() {
                let (temp_x, temp_y, temp_z) = (temp_x.unwrap().parse::<f32>(), temp_y.unwrap().parse::<f32>(), temp_z.unwrap().parse::<f32>()); 

                if temp_x.is_ok() && temp_y.is_ok() && temp_z.is_ok() {
                    let pinfo = CPlayerInfo::get_mut();
                    let pos = pinfo.get_pos();
                    (*pos).x = temp_x.unwrap();
                    (*pos).y = temp_y.unwrap();
                    (*pos).z = temp_z.unwrap();
                } else {
                    CHud::set_help_message("Invalid coordinages", false);
                }
            } else {
                CHud::set_help_message("Invalid coordinages", false);
            }
            
        }
    }
}

