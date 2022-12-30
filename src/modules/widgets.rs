use imgui::*;

pub fn calc_size(ui: &Ui, count: u32, spacing: bool) -> [f32; 2] {
    let mut spacing_ = spacing;
    if count == 1 {
        spacing_ = false;
    }

    // manually tested values
    let mut factor = 4.0 / 2.0; // ItemSpacing.x
    let x;

    if count == 3 {
        factor = 4.0 / 1.403; // ItemSpacing.x
    }

    if spacing_ {
        x = ui.window_content_region_width() / count as f32 - factor;
    } else {
        x = ui.window_content_region_width() / count as f32;
    }

    return [x, ui.frame_height() * 1.3];
}

pub fn text_centered(ui: &Ui, text : &str)
{
    let sz = ui.calc_text_size(text);
    ui.new_line();
    let width = ui.window_content_region_width() - sz[0];
    ui.same_line_with_spacing(width/2.0, 0.0);
    ui.text(text);
}