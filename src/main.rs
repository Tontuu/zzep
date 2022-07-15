use ncurses::*;
use std::thread::sleep;
use std::time::{Duration, Instant};
use rand::Rng;

const SETUP_TEXT: &str = "Reaction-Time Game";
const KEYS_TEXT: &str = "Press any key to start";
const READY_TEXT: &str = "Get Ready";
const WIN_TEXT: &str = "You won!";
const LOSE_TEXT: &str = "You lost";

#[derive(PartialEq)]
enum GameState {
    Setup,
    Play,
    End
}

#[derive(PartialEq)]
enum Result {
    Win,
    Lose
}

#[derive(Default, Copy, Clone)]
struct Vec2 {
    x: i32,
    y: i32,
}



#[derive(Default, Copy, Clone)]
struct Ui {
    pos: Vec2
}

impl Ui {
    fn reset_pos(&mut self) {
	self.pos = Vec2 { x: 0, y: 0};
    }

    fn set_pos(&mut self, pos: Vec2) {
	self.pos = pos;
    }

    fn center_pos(&mut self) {
	self.pos.x = COLS()/2-1;
	self.pos.y = LINES()/2-1;
    }

    fn offset(&mut self, x: i32, y: i32) {
	self.pos.x += -x;
	self.pos.y += y;
    }

    fn label(&mut self, text: &str) {
	mv(self.pos.y, self.pos.x);
	addstr(text);
	self.center_pos();
    }
}

fn setup(ui: &mut Ui) {
    clear();

    attron(A_STANDOUT());
    ui.offset(SETUP_TEXT.len() as i32 / 2, 0);
    ui.label(SETUP_TEXT);
    attroff(A_STANDOUT());

    ui.offset(KEYS_TEXT.len() as i32 / 2, 2);
    ui.label(KEYS_TEXT);
}

fn init_game(ui: &mut Ui) -> (Result, String) {
    noecho();

    let rand_row = rand::thread_rng().gen_range(0..LINES()-1);
    let rand_col = rand::thread_rng().gen_range(0..COLS()-1);
    let rand_ch = rand::thread_rng().gen_range(65..90) as u8 as char;
    let cooldown = rand::thread_rng().gen_range(1..5);

    clear();
    ui.offset(READY_TEXT.len() as i32 / 2, 0);
    ui.label(READY_TEXT);

    refresh();
    sleep(Duration::from_secs(2));
    for i in (1..4).rev() {
	clear();

	ui.center_pos();
	ui.label(&i.to_string());

	refresh();

	sleep(Duration::from_secs(1));
    }
    clear();
    refresh();

    sleep(Duration::from_secs(cooldown));

    
    ui.reset_pos();
    attron(A_BLINK());
    ui.offset(-rand_col, rand_row);
    ui.label(&rand_ch.to_string());
    attroff(A_BLINK());
    refresh();

    let now = Instant::now();

    // Throws away any typeahead that has been typed by the user at waiting time.
    flushinp();

    let key = getch() as u8 as char;


    let mut result = Result::Lose;
    if key == rand_ch { result = Result::Win; }

    let time_spent = format!("{:.8}ms", now.elapsed().as_secs_f32().to_string());


    (result, time_spent)
}

fn main() {
    initscr();
    curs_set(CURSOR_VISIBILITY::CURSOR_INVISIBLE);

    let mut state = GameState::Setup;
    let mut ui = Ui::default();
    ui.center_pos();

    while state != GameState::End {
	if state == GameState::Setup {
	    setup(&mut ui);
	}

	let key = getch();

	match key as u8 as char {
	    'q'  => state = GameState::End,
	     _   => state = GameState::Play
	}

	if state == GameState::Play {
	    let (result, time) = init_game(&mut ui);

	    attron(A_BOLD());
	    if result == Result::Win {
		ui.offset(WIN_TEXT.len() as i32 / 2, 0);
		ui.label(WIN_TEXT);
	    } else {
		ui.offset(LOSE_TEXT.len() as i32 / 2, 0);
		ui.label(LOSE_TEXT);
	    } 
	    attroff(A_BOLD());

	    ui.offset(time.len() as i32 / 2, 1);
	    ui.label(&time)
	}

	refresh();
    }
    endwin();
}

