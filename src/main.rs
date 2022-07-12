use ncurses::*;
use std::thread::sleep;
use std::time::{Duration, Instant};
use rand::Rng;

const SETUP_TEXT: &str = "Reaction-Time Game";
const READY_TEXT: &str = "Get Ready!";
const WIN_TEXT: &str = "[ You won ]";
const LOSE_TEXT: &str = " [ You lost ]";

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
       fn set_pos(&mut self, pos: Vec2) {
	self.pos = pos;
    }

    fn center_pos(&mut self) {
	self.pos.x = self.pos.x / 2;
	self.pos.y = self.pos.y / 2;
    }

    fn offset(&mut self, x: i32, y: i32) {
	self.pos.x += x;
	self.pos.y += y;
    }

    fn label(&self, text: &str) {
	mv(self.pos.y, self.pos.x);
	addstr(text);
    }
}

fn setup(mut ui: Ui) {
    clear();
    ui.offset(-(SETUP_TEXT.len() as i32 / 2), 0);
    ui.label(SETUP_TEXT);
}

fn init_game(mut ui: Ui) -> (Result, String) {
    noecho();
    let rand_row = rand::thread_rng().gen_range(0..LINES()-1);
    let rand_col = rand::thread_rng().gen_range(0..COLS()-1);
    let rand_ch = rand::thread_rng().gen_range(65..90) as u8 as char;
    let cooldown = rand::thread_rng().gen_range(1..5);

    clear();
    ui.offset(-(READY_TEXT.len() as i32 / 2), 0);
    ui.label(READY_TEXT);
    refresh();
    sleep(Duration::from_secs(2));
    for i in (1..4).rev() {
	clear();
	ui.set_pos(Vec2{ x:COLS()-1, y:LINES()-1 });
	ui.center_pos();
	ui.label(&i.to_string());
	refresh();
	sleep(Duration::from_secs(1));
    }
    clear();
    refresh();

    sleep(Duration::from_secs(cooldown));

    mv(rand_row, rand_col);
    addstr(&rand_ch.to_string());
    refresh();

    let now = Instant::now();

    // Throws away any typeahead that has been typed by the user at waiting time.
    flushinp();

    let key = getch() as u8 as char;

    let mut result = Result::Lose;
    if key == rand_ch { result = Result::Win; }

    let time_spent = format!(" {:.6}ms", now.elapsed().as_secs_f32().to_string());

    (result, time_spent)
}

fn main() {
    initscr();
    curs_set(CURSOR_VISIBILITY::CURSOR_INVISIBLE);

    let mut state = GameState::Setup;
    let mut ui = Ui::default();

    let pos = Vec2 { x: COLS()-1, y: LINES()-1};
    ui.set_pos(pos);
    ui.center_pos();

    while state != GameState::End {
	if state == GameState::Setup {
	    setup(ui);
	}

	let key = getch();

	match key as u8 as char {
	    'q'  => state = GameState::End,
	    ' ' => state = GameState::Play,
	    '\n' => state = GameState::Play,
	     _   => state = GameState::Setup
	}

	if state == GameState::Play {
	    let (result, time) = init_game(ui);
	    if result == Result::Win {
		ui.offset(-(WIN_TEXT.len() as i32 / 2), 0);
		ui.label(WIN_TEXT);
		ui.offset(WIN_TEXT.len() as i32 / 2, 0);
	    } else {
		ui.offset(-(LOSE_TEXT.len() as i32 / 2), 0);
		ui.label(LOSE_TEXT);
		ui.offset(LOSE_TEXT.len() as i32 / 2, 0);
	    } 
	    mv((LINES()+2)/2, ((COLS()-1)/2) - time.len() as i32 / 2);
	    addstr(&time);
	}

	refresh();
    }
    endwin();
}

