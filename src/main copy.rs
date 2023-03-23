extern crate gtk;
extern crate gdk;
extern crate pango;
extern crate pangocairo;

use gtk::prelude::*;
use gtk::{DrawingArea, Window, WindowType};
use std::rc::Rc;
use std::cell::RefCell;
use glib::{timeout_add, timeout_add_local};
use std::time::Duration;
use std::collections::HashSet;
use gdk::keys::constants::{Up, Down};


const WIDTH: i32 = 640;
const HEIGHT: i32 = 480;
const PADDLE_WIDTH: i32 = 20;
const PADDLE_HEIGHT: i32 = 80;
const BALL_RADIUS: i32 = 10;
const BALL_SPEED: i32 = 5;

struct GameState {
    paddle1_y: i32,
    paddle2_y: i32,
    ball_x: i32,
    ball_y: i32,
    ball_dx: i32,
    ball_dy: i32,
    score1: i32,
    score2: i32,
}

impl GameState {
    fn new() -> GameState {
        GameState {
            paddle1_y: HEIGHT / 2 - PADDLE_HEIGHT / 2,
            paddle2_y: HEIGHT / 2 - PADDLE_HEIGHT / 2,
            ball_x: WIDTH / 2 - BALL_RADIUS / 2,
            ball_y: HEIGHT / 2 - BALL_RADIUS / 2,
            ball_dx: BALL_SPEED,
            ball_dy: BALL_SPEED,
            score1: 0,
            score2: 0,
        }
    }

    fn update(&mut self) {
        self.ball_x += self.ball_dx;
        self.ball_y += self.ball_dy;

        if self.ball_y <= 0 || self.ball_y >= HEIGHT - BALL_RADIUS {
            self.ball_dy = -self.ball_dy;
        }

        if self.ball_x <= PADDLE_WIDTH && self.ball_y >= self.paddle1_y && self.ball_y <= self.paddle1_y + PADDLE_HEIGHT {
            self.ball_dx = -self.ball_dx;
        }

        if self.ball_x >= WIDTH - PADDLE_WIDTH - BALL_RADIUS && self.ball_y >= self.paddle2_y && self.ball_y <= self.paddle2_y + PADDLE_HEIGHT {
            self.ball_dx = -self.ball_dx;
        }

        if self.ball_x <= 0 {
            self.score2 += 1;
            self.reset();
        }

        if self.ball_x >= WIDTH - BALL_RADIUS {
            self.score1 += 1;
            self.reset();
        }
    }

    fn reset(&mut self) {
        self.ball_x = WIDTH / 2 - BALL_RADIUS / 2;
        self.ball_y = HEIGHT / 2 - BALL_RADIUS / 2;
        self.ball_dx = -self.ball_dx;
        self.ball_dy = BALL_SPEED;
    }
}

fn main() {
    if gtk::init().is_err() {
        println!("Failed to initialize GTK.");
        return;
    }

    let window = Window::new(WindowType::Toplevel);
    window.set_title("Pong");
    window.set_default_size(WIDTH, HEIGHT);

    let drawing_area = DrawingArea::new();
    drawing_area.set_size_request(WIDTH, HEIGHT);

    let state = Rc::new(RefCell::new(GameState::new()));
    let state_draw = state.clone();
    drawing_area.connect_draw(move |_, cr| {
        
        let state = state_draw.borrow();
        // Clear the drawing area
        cr.set_source_rgb(0.0, 0.0, 0.0);
        cr.rectangle(0.0, 0.0, WIDTH as f64, HEIGHT as f64);
        cr.fill();

        // Draw the paddles
        cr.rectangle(0.0, state.paddle1_y as f64, PADDLE_WIDTH as f64, PADDLE_HEIGHT as f64);
        cr.rectangle(WIDTH as f64 - PADDLE_WIDTH as f64, state.paddle2_y as f64, PADDLE_WIDTH as f64, PADDLE_HEIGHT as f64);
        cr.set_source_rgb(1.0, 1.0, 1.0);
        cr.fill();
    
        // Draw the ball
        cr.arc(state.ball_x as f64 + BALL_RADIUS as f64 / 2.0, state.ball_y as f64 + BALL_RADIUS as f64 / 2.0, BALL_RADIUS as f64 / 2.0, 0.0, 2.0 * std::f64::consts::PI);
        cr.set_source_rgb(1.0, 1.0, 1.0);
        cr.fill();
    
        // Draw the score
        let score_text = format!("{} - {}", state.score1, state.score2);
        let layout = pangocairo::functions::create_layout(cr);
        let font_desc = pango::FontDescription::from_string("sans bold 48");
        layout.set_font_description(Some(&font_desc));
        layout.set_text(&score_text);
        let (score_width, score_height) = layout.pixel_size();
        let score_x = (WIDTH - score_width) / 2;
        let score_y = (HEIGHT - score_height) / 2;
        cr.move_to(score_x as f64, score_y as f64);
        pangocairo::functions::show_layout(cr, &layout);
    
        Inhibit(false)
    });
    
    let state_key_press = state.clone();
    window.connect_key_press_event(move |_, key| {
        let mut state = state_key_press.borrow_mut();
        match key.keyval() {
            gdk::keys::constants::Down => {
                state.paddle2_y += 10;
                if state.paddle2_y + PADDLE_HEIGHT >= HEIGHT {
                    state.paddle2_y = HEIGHT - PADDLE_HEIGHT;
                }
            },
            gdk::keys::constants::Up => {
                state.paddle2_y -= 10;
                if state.paddle2_y <= 0 {
                    state.paddle2_y = 0;
                }
            },
            gdk::keys::constants::s => {
                state.paddle1_y += 10;
                if state.paddle1_y + PADDLE_HEIGHT >= HEIGHT {
                    state.paddle1_y = HEIGHT - PADDLE_HEIGHT;
                }
            },
            gdk::keys::constants::w => {
                state.paddle1_y -= 10;
                if state.paddle1_y <= 0 {
                    state.paddle1_y = 0;
                }
            },
            _ => (),
        }
        Inhibit(false)
    });

    window.add(&drawing_area);
    window.show_all();

    window.connect_delete_event(|_, _| {
        gtk::main_quit();
        Inhibit(false)
    });
    
    let state_timeout = state.clone();
    glib::timeout_add_local(Duration::from_millis(16), move || {
        state_timeout.borrow_mut().update();
        drawing_area.queue_draw();
        Continue(true)
});

    gtk::main();
}
