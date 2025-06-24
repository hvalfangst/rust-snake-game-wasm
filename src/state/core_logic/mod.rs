pub mod movement;
pub mod bounds;
pub mod food;
pub mod game_state;
pub mod game_over;
pub mod collision;
mod snake;
mod background;

use crate::state::structs::GameState;
use rodio::Sink;
use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;
use log::log;

pub trait CoreLogic {
    fn execute(&self, game_state: &mut GameState, sink: &mut Sink);
}

pub fn execute_core_logic(game_state: &mut GameState, core_logic_operations: &HashMap<String, Rc<RefCell<dyn CoreLogic>>>, sink: &mut Sink) {
    for (_, core_logic_operation) in core_logic_operations.iter() {
        core_logic_operation.borrow().execute(game_state, sink);
    }
}

pub fn initialize_core_logic_map() -> HashMap<String, Rc<RefCell<dyn CoreLogic>>> {
    let mut logic_map: HashMap<String, Rc<RefCell<dyn CoreLogic>>> = HashMap::new();

    logic_map.insert("CheckGameOver".to_string(), Rc::new(RefCell::new(game_over::CheckGameOver)));

    // Game state updates
    logic_map.insert("UpdateDeltaTime".to_string(), Rc::new(RefCell::new(game_state::UpdateDeltaTime)));

    // Movement
    logic_map.insert("ModifyCoordinatesOfBodyParts".to_string(), Rc::new(RefCell::new(movement::ModifyCoordinatesOfBodyParts)));

    // Bounds checking
    logic_map.insert("VerticalBounds".to_string(), Rc::new(RefCell::new(bounds::VerticalBounds)));
    logic_map.insert("HorizontalBounds".to_string(), Rc::new(RefCell::new(bounds::HorizontalBounds)));

    // Food system
    logic_map.insert("SpawnFood".to_string(), Rc::new(RefCell::new(food::SpawnFood)));
    logic_map.insert("CheckIfFoodWasEaten".to_string(), Rc::new(RefCell::new(food::CheckIfFoodWasEaten)));
    logic_map.insert("AlternateBetweenFoodSpriteFrames".to_string(), Rc::new(RefCell::new(food::AlternateBetweenFoodSpriteFrames)));

    // Collision detection
    logic_map.insert("CheckSelfCollision".to_string(), Rc::new(RefCell::new(collision::CheckSelfCollision)));

    // Snake sprite logic
    logic_map.insert("AlternateBodySpriteFrameIndex".to_string(), Rc::new(RefCell::new(snake::AlternateBodySpriteFrameIndex)));
    logic_map.insert("AlternateHeadSpriteFrameIndex".to_string(), Rc::new(RefCell::new(snake::AlternateHeadSpriteFrameIndex)));

    // Background sprite frame
    logic_map.insert("AlternateBackgroundSpriteFrame".to_string(), Rc::new(RefCell::new(background::AlternateBackgroundSpriteFrame)));

    logic_map
}