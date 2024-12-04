use getset::{CopyGetters, Getters};
use iced::Point;

#[derive(Default, Getters, CopyGetters)]
pub struct SolarSystemPositionState {
    #[getset(get_copy = "pub")]
    center_position: Point,
    last_cursor_position: Option<Point>,
    #[getset(get = "pub")]
    pinch: CursorPinch,
}

impl SolarSystemPositionState {
    pub fn set_pinch(&mut self, cursor_pinch: CursorPinch) {
        self.pinch = cursor_pinch;
    }

    pub fn set_cursor_position(&mut self, cursor_position: Point) {
        self.last_cursor_position = Some(cursor_position);
    }

    pub fn clear_cursor_position(&mut self) {
        self.last_cursor_position = None;
    }

    pub fn move_center_position(&mut self, cursor_position: Point) {
        let last_cursor_position = self.last_cursor_position.unwrap();

        self.center_position = Point::new(
            self.center_position.x + cursor_position.x - last_cursor_position.x,
            self.center_position.y + cursor_position.y - last_cursor_position.y,
        );

        self.last_cursor_position = Some(cursor_position);
    }

    pub fn center_system_position(&mut self) {
        self.center_position = Point::ORIGIN;
    }
}

#[derive(Default)]
pub enum CursorPinch {
    Clamped,
    #[default]
    NotClamped,
}