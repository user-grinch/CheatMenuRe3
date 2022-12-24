use imgui::*;

pub fn calc_size(ui: &Ui, count: u32, spacing: bool) -> [f32; 2] {
    let mut spacing_ = spacing;
    if count == 1 {
        spacing_ = false;
    }

    // manually tested values
    let mut factor = 7.0 / 2.0; // ItemSpacing.x
    let x;

    if count == 3 {
        factor = 7.0 / 1.403; // ItemSpacing.x
    }

    if spacing_ {
        x = ui.window_content_region_width() / count as f32 - factor;
    } else {
        x = ui.window_content_region_width() / count as f32;
    }

    return [x, ui.frame_height() * 1.3];
}