extern crate pancurses;
extern crate rand;
use pancurses::*;
use rand::Rng;

const DELAYSIZE: i32 = 200;

const COLOR_TABLE: [i16; 8] = [
    COLOR_RED,
    COLOR_BLUE,
    COLOR_GREEN,
    COLOR_CYAN,
    COLOR_RED,
    COLOR_MAGENTA,
    COLOR_YELLOW,
    COLOR_WHITE,
];

struct Flake {
    x: i32,
    y: i32,
    delta_x: i32,
    delta_y: i32
}

fn main() {
    let window = initscr();
    window.nodelay(true);
    noecho();

    if has_colors() {
        start_color();
    }

    for (i, color) in COLOR_TABLE.iter().enumerate() {
        init_pair(i as i16, *color, COLOR_BLACK);
    }

    let mut rng = rand::thread_rng();

    let max_x = window.get_max_x();
    let max_y = window.get_max_y();

    while window.getch().is_none() {
        let rand_x = rand::thread_rng().gen_range(0..max_x);
        let rand_y = rand::thread_rng().gen_range(0..max_y);
        window.mvaddstr(rand_y, rand_x, "Test");
        // window.erase();
        // myrefresh(&window);
    }

    endwin();
}

fn myrefresh(window: &Window) {
    napms(DELAYSIZE);
    window.mv(window.get_max_y() - 1, window.get_max_x() - 1);
    window.refresh();
}

fn get_color<T: Rng>(rng: &mut T, window: &Window) {
    let bold = if rng.gen::<bool>() { A_BOLD } else { A_NORMAL } as chtype;
    window.attrset(COLOR_PAIR(rng.gen::<chtype>() % 8) | bold);
}
