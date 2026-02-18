use egui::{PointerButton, Response};

pub fn primary_click_position(response: &Response) -> Option<egui::Pos2> {
    if response.clicked_by(PointerButton::Primary) {
        response.interact_pointer_pos()
    } else {
        None
    }
}
