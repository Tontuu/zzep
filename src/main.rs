/*Reaction-Time Tester.
Write a program that measures how quickly a user
can respond. The program waits for a random
interval of time then prints a single digit on the
screen. The user has to type that digit as quickly as
possible. The program records how long it takes the
user to respond. Your program should perform 10
such tests and report the minimum, maximum and
average response time. (Hint, read the manual
page for gettimeofday).*/

use ncurses::*;
use std::thread::sleep;
use std::time::{Duration, Instant};
use rand::Rng;

const READ_TEXT: &str = "Get Ready!";
const TITLE_TEXT: &str = "Reaction-Time Game";
const WIN_TEXT: &str = "[ You won ]";
const LOSE_TEXT: &str = "[ You lost ]";

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

fn setup() {
    clear();
    mv((LINES()-1)/2, ((COLS()-1)/2) - TITLE_TEXT.len() as i32 /2);
    addstr(TITLE_TEXT);
}

fn init_game() -> (Result, String) {
    flushinp();

    let rand_row = rand::thread_rng().gen_range(0..LINES()-1);
    let rand_col = rand::thread_rng().gen_range(0..COLS()-1);
    let rand_ch = rand::thread_rng().gen_range(65..90) as u8 as char;
    let cooldown = rand::thread_rng().gen_range(1..5);

    clear();
    mv((LINES()-1)/2, ((COLS()-1)/2)-READ_TEXT.len() as i32 /2);
    addstr(READ_TEXT);
    refresh();
    sleep(Duration::from_secs(2));
    for i in (1..4).rev() {
	clear();
	mv((LINES()-1)/2, (COLS()-1)/2);
	addstr(&i.to_string());
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

    let key = getch() as u8 as char;

    let mut result = Result::Lose;
    if key == rand_ch { result = Result::Win; }

    let time_spent = format!(" {:.6}ms", now.elapsed().as_secs_f32().to_string());

    (result, time_spent)
}

fn main() {

    initscr();
    noecho();
    curs_set(CURSOR_VISIBILITY::CURSOR_INVISIBLE);

    let mut state = GameState::Setup;


    while state != GameState::End {
	if state == GameState::Setup {
	    setup();
	}

	let key = getch();

	match key as u8 as char {
	    'q'  => state = GameState::End,
	    ' ' => state = GameState::Play,
	    '\n' => state = GameState::Play,
	     _   => state = GameState::Setup
	}

	if state == GameState::Play {
	    let (result, time) = init_game();
	    if result == Result::Win {
		mv((LINES()-1)/2, ((COLS()-1)/2) - WIN_TEXT.len() as i32 /2);
		addstr(WIN_TEXT);
	    } else {
		mv((LINES()-1)/2, ((COLS()-1)/2) - LOSE_TEXT.len() as i32 /2);
		addstr(LOSE_TEXT);
	    } 
	    mv((LINES()+2)/2, ((COLS()-1)/2) - time.len() as i32 / 2);
	    addstr(&time);
	}

	refresh();
    }
    endwin();
}
