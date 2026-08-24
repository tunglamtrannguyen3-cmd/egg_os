use super::font::draw_string;
use super::framebuffer::Framebuffer;

/// Renders a low-battery dialog box centered on screen
pub fn show_low_battery_dialog(fb: &mut Framebuffer, screen_width: usize, screen_height: usize) {
    let dialog_w = 320;
    let dialog_h = 100;
    let start_x = screen_width.saturating_sub(dialog_w) / 2;
    let start_y = screen_height.saturating_sub(dialog_h) / 2;

    unsafe {
        // Render dark container background (0x002A1215)
        fb.draw_rect(start_x, start_y, dialog_w, dialog_h, 0x002A_1215);

        // Render red top warning border line (0x00F87171)
        fb.draw_rect(start_x, start_y, dialog_w, 3, 0x00F8_7171);
    }

    // Render warning title & instruction body
    draw_string(fb, start_x + 16, start_y + 20, "LOW BATTERY WARNING", 0x00F8_7171);
    draw_string(fb, start_x + 16, start_y + 48, "Please connect charger...", 0x00F8_FAFC);
}
