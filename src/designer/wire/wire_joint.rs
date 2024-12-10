use bevy::{ecs::reflect, prelude::*};

use crate::{
    designer::{
        cursor::{Cursor, CursorState},
        model::Model,
        position::Position,
        signal::{Signal, SignalState},
    },
    get_cursor,
};

use super::{WireNode, WireNodes};

#[derive(Component, Reflect)]
#[reflect(Component)]
pub struct WireJointModel;

#[derive(Bundle)]
pub struct WireJointModelBundle {
    pub wire_joint: WireJointModel,
    pub signal_state: SignalState,
    pub model: Model,
}

impl WireJointModelBundle {
    pub fn new(position: Position) -> Self {
        Self {
            wire_joint: WireJointModel,
            signal_state: SignalState::new(Signal::Low),
            model: Model::from_position(position),
        }
    }
}

pub fn create_wire_joint(
    input: Res<ButtonInput<MouseButton>>,
    q_cursor: Query<(&Cursor, &Transform)>,
    mut q_wires: Query<&mut WireNodes>,
    mut commands: Commands,
) {
    let (cursor, cursor_transform) = get_cursor!(q_cursor);

    if !input.just_pressed(MouseButton::Left) {
        return;
    }

    if let CursorState::DraggingWire(wire_entity) = cursor.state {
        if let Ok(mut wire) = q_wires.get_mut(wire_entity) {
            let position = Position::from_translation(cursor_transform.translation);
            let wire_joint = WireJointModelBundle::new(position);
            let uuid = wire_joint.model.id.0;
            commands.spawn(wire_joint);
            wire.0.push(WireNode::Joint(uuid));
        }
    }
}
