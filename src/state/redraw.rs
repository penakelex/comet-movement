use tap::Tap;

/// Состояние повторной отрисовки
pub struct RedrawState {
    ticks_until_redraw: u32,
    ticks_between_redraws: u32,
}

impl RedrawState {
    pub fn new(ticks_between_redraws: u32) -> Self {
        Self {
            ticks_until_redraw: 0,
            ticks_between_redraws,
        }
    }
}

impl RedrawState {
    /// Проверка, нужно ли отрисовывать на текущем тике
    pub fn redraw_on_tick(&mut self) -> bool {
        (self.ticks_until_redraw == 0).tap(|redraw| {
            if *redraw {
                self.ticks_until_redraw = self.ticks_between_redraws;
            } else {
                self.ticks_until_redraw -= 1;
            }
        })
    }
}

impl RedrawState {
    pub fn reload(&mut self) {
        self.ticks_until_redraw = 0;
    }
}