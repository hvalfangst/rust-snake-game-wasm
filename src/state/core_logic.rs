use crate::state::constants::physics::{ACCELERATION, LOWER_BOUND_X, LOWER_BOUND_Y, MAX_VELOCITY, UPPER_BOUND_X, UPPER_BOUND_Y};
use crate::state::structs::{Direction, GameState};
use rodio::Sink;
use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;

pub fn execute_core_logic(game_state: &mut GameState, core_logic_operations: &HashMap<String, Rc<RefCell<dyn CoreLogic>>>, sink: &mut Sink) {
    for (_, core_logic_operation) in core_logic_operations.iter() {
        core_logic_operation.borrow().execute(game_state, sink);
    }
}

pub trait CoreLogic {
    fn execute(&self, game_state: &mut GameState, sink: &mut Sink);
}

pub struct VerticalBounds;

impl CoreLogic for VerticalBounds {
    fn execute(&self, game_state: &mut GameState, sink: &mut Sink) {
        // Check if the player is out of vertical bounds
        if game_state.player.y + 16.0 <= LOWER_BOUND_Y {
            println!(
                "Player reached upmost y. Resetting y to LOWER_BOUND. Current y: {}, vy: {}",
                game_state.player.y, game_state.player.vy
            );
            game_state.player.y =  (game_state.player.y + 16.0).abs();
            game_state.player.vy = 0.0; // Stop vertical movement
        } else if game_state.player.y + 16.0 >= UPPER_BOUND_Y {
            println!(
                "Player reached downmost y. Resetting y to UPPER_BOUND. Current y: {}, vy: {}",
                game_state.player.y, game_state.player.vy
            );
            game_state.player.y = (game_state.player.y - 16.0).abs(); // Adjust y to fit within bounds
            game_state.player.vy = 0.0; // Stop vertical movement
        }
    }
}

pub struct HorizontalBounds;

impl CoreLogic for HorizontalBounds {
    fn execute(&self, game_state: &mut GameState, sink: &mut Sink) {
        // Check if the player is out of horizontal bounds
        if game_state.player.x + 16.0 <= LOWER_BOUND_X {
            println!(
                "Player reached leftmost x. Resetting x to LOWER_BOUND. Current x: {}, vx: {}",
                game_state.player.x, game_state.player.vx
            );
            game_state.player.x = (game_state.player.x + 16.0).abs(); // Adjust x to fit within bounds
            println!(
                "Adjusted x to fit within bounds. New x: {}, vx: {}",
                game_state.player.x, game_state.player.vx
            );
            game_state.player.vx = 0.0; // Stop horizontal movement
        } else if game_state.player.x + 16.0 >= UPPER_BOUND_X {
            println!(
                "Player reached rightmost x. Resetting x to UPPER_BOUND. Current x: {}, vx: {}",
                game_state.player.x, game_state.player.vx
            );
            game_state.player.x = (game_state.player.x - 16.0).abs(); // Adjust x to fit within bounds
            game_state.player.vx = 0.0; // Stop horizontal movement
        }
    }
}

pub struct CheckGameOver;

impl CoreLogic for CheckGameOver {
    fn execute(&self, game_state: &mut GameState, sink: &mut Sink) {
    }
}

pub fn increase_x_velocity(game_state: &mut GameState) {
    game_state.player.vx += ACCELERATION;

    if game_state.player.vx > MAX_VELOCITY {
        game_state.player.vx = MAX_VELOCITY;
    } else {
        game_state.player.vx *= 0.98;
        if game_state.player.vx > MAX_VELOCITY {
            game_state.player.vx = MAX_VELOCITY;
        }
    }
}

pub fn decrease_x_velocity(game_state: &mut GameState) {
    game_state.player.vx *= 0.95;
    if game_state.player.vx.abs() < 0.1 {
        game_state.player.vx = 0.0;
    }
}

pub fn increase_y_velocity(game_state: &mut GameState) {
    game_state.player.vy += ACCELERATION;

    if game_state.player.vy > MAX_VELOCITY {
        game_state.player.vy = MAX_VELOCITY;
    } else {
        game_state.player.vy *= 0.98;
        if game_state.player.vy > MAX_VELOCITY {
            game_state.player.vy = MAX_VELOCITY;
        }
    }
}

pub fn decrease_y_velocity(game_state: &mut GameState) {
    game_state.player.vy *= 0.95;
    if game_state.player.vy.abs() < 0.1 {
        game_state.player.vy = 0.0;
    }
}

pub struct ModifyPosition;

impl CoreLogic for ModifyPosition {
    fn execute(&self, game_state: &mut GameState, sink: &mut Sink) {
        match game_state.player.direction {
            Direction::Left => {
                // println!("Moving left: x = {}, vx = {}", game_state.player.x, game_state.player.vx);
                game_state.player.x -= game_state.player.vx;
            }
            Direction::Right => {
                // println!("Moving right: x = {}, vx = {}", game_state.player.x, game_state.player.vx);
                game_state.player.x += game_state.player.vx;
            }
            Direction::Up => {
                // println!("Moving up: y = {}, vy = {}", game_state.player.y, game_state.player.vy);
                game_state.player.y -= game_state.player.vy;
            }
            Direction::Down => {
                // println!("Moving down: y = {}, vy = {}", game_state.player.y, game_state.player.vy);
                game_state.player.y += game_state.player.vy;
            }
        }
    }
}

pub fn initialize_core_logic_map() -> HashMap<String, Rc<RefCell<dyn CoreLogic>>> {
    let mut logic_map: HashMap<String, Rc<RefCell<dyn CoreLogic>>> = HashMap::new();

    logic_map.insert("VerticalBounds".to_string(), Rc::new(RefCell::new(VerticalBounds)));
    logic_map.insert("HorizontalBounds".to_string(), Rc::new(RefCell::new(HorizontalBounds)));
    logic_map.insert("CheckGameOver".to_string(), Rc::new(RefCell::new(CheckGameOver)));
    logic_map.insert("ModifyPosition".to_string(), Rc::new(RefCell::new(ModifyPosition)));

    logic_map
}
