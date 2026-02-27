use gset::Getset;
use iced::Point;

/// Состояние позиции Солнечной системы
#[derive(Default, Getset)]
pub struct SolarSystemPositionState {
    /// Позиция Солнечной системы
    #[getset(get_copy, vis = "pub")]
    center_position: Point,
    /// Последняя позиция курсора
    last_cursor_position: Option<Point>,
    #[getset(get, vis = "pub")]
    pinch: CursorPinch,
}

impl SolarSystemPositionState {
    pub fn set_pinch(&mut self, cursor_pinch: CursorPinch) {
        self.pinch = cursor_pinch;
    }

    pub fn set_cursor_position(
        &mut self,
        cursor_position: Point,
    ) {
        self.last_cursor_position = Some(cursor_position);
    }

    pub fn clear_cursor_position(&mut self) {
        self.last_cursor_position = None;
    }

    pub fn move_center_position(
        &mut self,
        cursor_position: Point,
    ) {
        if let Some(last_position) =
            self.last_cursor_position
        {
            self.center_position = Point::new(
                self.center_position.x + cursor_position.x
                    - last_position.x,
                self.center_position.y + cursor_position.y
                    - last_position.y,
            );
        }

        self.last_cursor_position = Some(cursor_position);
    }

    pub fn center_system_position(&mut self) {
        self.center_position = Point::ORIGIN;
    }
}

impl SolarSystemPositionState {
    pub fn reload(&mut self) {
        self.center_position = Point::ORIGIN;
        self.pinch = CursorPinch::NotClamped;
        self.last_cursor_position = None;
    }
}

/// Состояние нажатия на левую кнопку мыши
#[derive(Default)]
pub enum CursorPinch {
    Clamped,
    #[default]
    NotClamped,
}
