use crate::state::constants::physics::{LOWER_BOUND_X, LOWER_BOUND_Y, UPPER_BOUND_X, UPPER_BOUND_Y};
use crate::state::structs::{Direction, Food, GameState, Vector2D};
use rodio::Sink;
use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;
use rand::Rng;

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
        let player_width = game_state.sprites.player[0].width as f32;

        // Check if the player is out of vertical bounds
        if game_state.player.ledger[0].y + player_width <= LOWER_BOUND_Y {
            game_state.player.ledger[0].y = (game_state.player.ledger[0].y + player_width).abs();
        } else if game_state.player.ledger[0].y + player_width >= UPPER_BOUND_Y {
            game_state.player.ledger[0].y = (game_state.player.ledger[0].y - player_width).abs(); // Adjust y to fit within bounds

        }
    }
}

pub struct HorizontalBounds;

impl CoreLogic for HorizontalBounds {
    fn execute(&self, game_state: &mut GameState, sink: &mut Sink) {
        let player_width = game_state.sprites.player[0].width as f32;

        // Check if the player is out of horizontal bounds
        if game_state.player.ledger[0].x + player_width <= LOWER_BOUND_X {
            game_state.player.ledger[0].x = (game_state.player.ledger[0].x + player_width).abs(); // Adjust x to fit within bounds

        } else if game_state.player.ledger[0].x + player_width >= UPPER_BOUND_X {
            game_state.player.ledger[0].x = (game_state.player.ledger[0].x - player_width).abs(); // Adjust x to fit within bounds

        }
    }
}

pub struct CheckGameOver;

impl CoreLogic for CheckGameOver {
    fn execute(&self, game_state: &mut GameState, sink: &mut Sink) {}
}


pub struct ModifyPosition;

impl CoreLogic for ModifyPosition {
    fn execute(&self, game_state: &mut GameState, sink: &mut Sink) {
        match game_state.player.direction {
            Direction::Left => {

                game_state.player.ledger[0].x -= 1.0; // Move the head of the snake left

                // let new_x = game_state.player.ledger.last().unwrap().x - 1.0;
                // game_state.player.ledger.push(Vector2D { x: new_x, y: game_state.player.ledger[0].y });
                // game_state.player.ledger.sort_by(|a, b| a.x.partial_cmp(&b.x).unwrap());

            }
            Direction::Right => {

                game_state.player.ledger[0].x += 1.0; // Move the head of the snake right

                // let new_x = game_state.player.ledger.last().unwrap().x + 1.0;
                // game_state.player.ledger.push(Vector2D { x: new_x, y: game_state.player.ledger[0].y });
                // game_state.player.ledger.sort_by(|a, b| a.x.partial_cmp(&b.x).unwrap());

            }
            Direction::Up => {
                game_state.player.ledger[0].y -= 1.0; // Move the head of the snake up
                // let new_y = game_state.player.ledger.last().unwrap().y - 1.0;
                // game_state.player.ledger.push(Vector2D { x: game_state.player.ledger[0].x, y: new_y });
                // game_state.player.ledger.sort_by(|a, b| a.y.partial_cmp(&b.y).unwrap());
            }
            Direction::Down => {
                game_state.player.ledger[0].y += 1.0; // Move the head of the snake down
                // let new_y = game_state.player.ledger.last().unwrap().y + 1.0;
                // game_state.player.ledger.push(Vector2D { x: game_state.player.ledger[0].x, y: new_y });
                // game_state.player.ledger.sort_by(|a, b| a.y.partial_cmp(&b.y).unwrap());

            }
        }
    }
}

pub struct SpawnFood;

impl CoreLogic for SpawnFood {
    fn execute(&self, game_state: &mut GameState, sink: &mut Sink) {
        if game_state.food.is_active {
            return; // Food is already active, no need to spawn new food
        }

        game_state.food = Food {
            position: Vector2D {
                x: rand::rng().random_range(LOWER_BOUND_X..UPPER_BOUND_X),
                y: rand::rng().random_range(LOWER_BOUND_Y..UPPER_BOUND_Y),
            },
            is_active: true,
        };
    }
}



pub fn initialize_core_logic_map() -> HashMap<String, Rc<RefCell<dyn CoreLogic>>> {
    let mut logic_map: HashMap<String, Rc<RefCell<dyn CoreLogic>>> = HashMap::new();

    logic_map.insert("VerticalBounds".to_string(), Rc::new(RefCell::new(VerticalBounds)));
    logic_map.insert("HorizontalBounds".to_string(), Rc::new(RefCell::new(HorizontalBounds)));
    // logic_map.insert("CheckGameOver".to_string(), Rc::new(RefCell::new(CheckGameOver)));
    logic_map.insert("ModifyPosition".to_string(), Rc::new(RefCell::new(ModifyPosition)));
    logic_map.insert("SpawnFood".to_string(), Rc::new(RefCell::new(SpawnFood)));

    logic_map
}
