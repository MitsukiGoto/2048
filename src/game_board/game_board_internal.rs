use super::window_ext::WindowExt;
use super::GameBoard;
use pancurses::{
    ACS_BTEE, ACS_HLINE, ACS_LLCORNER, ACS_LRCORNER, ACS_LTEE, ACS_PLUS, ACS_RTEE, ACS_TTEE,
    ACS_ULCORNER, ACS_URCORNER, ACS_VLINE,
};

impl GameBoard {
    pub(crate) fn reflect_game_board_state(&self) {
        let state = &self.game_board_state;
        let margin_y = |i: i32| -> i32 {
            if i != 1 {
                (i / 2) + 1
            } else {
                1
            }
        };
        for i in 0..self.to_be_rendered {
            for j in 0..self.to_be_rendered {
                let n = state.get_state()[i as usize][j as usize];
                self.window.mvaddstr(
                    self.y + margin_y(self.height) + j * (self.height + 1),
                    self.x + margin_y(self.width) + i * (self.width + 1),
                    n.to_string(),
                );
            }
        }
    }

    pub(crate) fn render_background_grid(&self) {
        for i in 0..=self.to_be_rendered {
            self.window.mvvline(
                0,
                (self.width + 1) * i,
                ACS_VLINE(),
                self.vertical_side_length,
            );
            self.window.mvhline(
                (self.height + 1) * i,
                0,
                ACS_HLINE(),
                self.horizontal_side_length,
            );
        }
        for i in 1..=3 {
            self.window
                .mvaddch(self.y + (self.height + 1) * i, self.x, ACS_LTEE());
            self.window.mvaddch(
                self.y + (self.height + 1) * i,
                self.x + self.horizontal_side_length,
                ACS_RTEE(),
            );
            self.window
                .mvaddch(self.y, self.x + (self.width + 1) * i, ACS_TTEE());
            self.window.mvaddch(
                self.y + self.vertical_side_length,
                self.x + (self.width + 1) * i,
                ACS_BTEE(),
            );
        }
        for i in 1..=3 {
            for j in 1..=3 {
                self.window.mvaddch(
                    self.y + (self.height + 1) * i,
                    self.x + (self.width + 1) * j,
                    ACS_PLUS(),
                );
            }
        }
        self.window.mvaddch(self.y, self.x, ACS_ULCORNER());
        self.window
            .mvaddch(self.y + self.vertical_side_length, self.x, ACS_LLCORNER());
        self.window
            .mvaddch(self.y, self.x + self.horizontal_side_length, ACS_URCORNER());
        self.window.mvaddch(
            self.y + self.vertical_side_length,
            self.horizontal_side_length,
            ACS_LRCORNER(),
        );
    }
}