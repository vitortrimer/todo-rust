use std::cmp::min;
use ncurses::*;
use ncurses::CURSOR_VISIBILITY::CURSOR_INVISIBLE;

const REGULAR_PAIR: i16 = 0;
const HIGHLIGHT_PAIR: i16 = 1;

fn main() {
    initscr();
    noecho();
    curs_set(CURSOR_INVISIBLE);

    start_color();
    init_pair(REGULAR_PAIR, COLOR_WHITE, COLOR_BLACK);
    init_pair(HIGHLIGHT_PAIR, COLOR_BLACK, COLOR_WHITE);
    
    let mut quit = false;
    let mut todos= vec!["Wake up", "Study", "Go out"];
    let mut todo_curr: usize = 1;

    while !quit {
        for (row, todo) in todos.iter().enumerate() {
            let pair = {
                if todo_curr == row {
                    HIGHLIGHT_PAIR
                } else {
                    REGULAR_PAIR
                }
            };

            attron(COLOR_PAIR(pair));
            mv(row as i32, 0);
            addstr(*todo);
            attroff(COLOR_PAIR(pair));
        }

        refresh();

        let key = getch();
        match key as u8 as char {
            'q' => quit = true,
            'k' => if todo_curr > 0 {
                todo_curr -= 1;
            },
            'j' => todo_curr = min(todo_curr + 1, todos.len() - 1),
            _ => {},
        }
    }

    endwin();
}