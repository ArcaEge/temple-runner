use bevy::prelude::*;
use leafwing_input_manager::prelude::*;

pub struct InputPlugin;

impl Plugin for InputPlugin {
    fn build(&self, _app: &mut App) {
        // Unused for now
    }
}

#[derive(Actionlike, PartialEq, Eq, Hash, Clone, Copy, Debug, Reflect)]
pub enum PlayerAction {
    #[actionlike(Axis)]
    Move,
    Jump,
}

impl PlayerAction {
    pub fn default_input_map() -> InputMap<Self> {
        let mut input_map = InputMap::default();

        // Default gamepad input bindings
        input_map.insert_axis(Self::Move, GamepadAxis::LeftStickX);
        input_map.insert(Self::Jump, GamepadButton::South);
        // input_map.insert(Self::UseItem, GamepadButton::RightTrigger2);

        // Default kbm input bindings
        input_map.insert_axis(Self::Move, VirtualAxis::ad());
        input_map.insert(Self::Jump, KeyCode::Space);
        // input_map.insert(Self::UseItem, MouseButton::Left);

        input_map
    }
}
