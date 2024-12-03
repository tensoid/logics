use bevy::{
    ecs::query::{QueryData, QueryFilter, WorldQuery},
    prelude::*,
};
use uuid::Uuid;

use crate::{
    designer::{
        cursor::{Cursor, CursorState},
        model::{Model, ModelId},
        position::Position,
        signal_state::{Signal, SignalState},
    },
    find_model_by_uuid, get_cursor,
};

use super::{WireNode, WireNodes};

#[derive(Component)]
pub struct WireJointModel;

#[derive(Bundle)]
pub struct WireJointModelBundle {
    wire_joint: WireJointModel,
    signal_state: SignalState,
    model: Model,
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
