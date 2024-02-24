use bevy::prelude::*;
use bevy_prototype_lyon::prelude::*;

use super::{
    chip::{ChipInputPin, ChipOutputPin},
    cursor::{Cursor, CursorState},
    io_pin::{BoardBinaryInputPin, BoardBinaryOutputPin},
};

#[derive(Component)]
pub struct Wire {
    pub src_pin: Option<Entity>,
    pub dest_pin: Option<Entity>,
}

/**
 * Updates the wires location to always stay connected to its source and destination pins.
 * If the source or destination pin was deleted or the wire is just not connected this also deletes the wire entirely.
 */
//TODO: Optimisation potential with only updating necessary wires.
#[allow(clippy::type_complexity)]
pub fn update_wires(
    mut q_wires: Query<(&mut Wire, &mut Path, &GlobalTransform, Entity)>,
    q_dest_pins: Query<&GlobalTransform, Or<(With<ChipInputPin>, With<BoardBinaryOutputPin>)>>,
    q_src_pins: Query<&GlobalTransform, Or<(With<ChipOutputPin>, With<BoardBinaryInputPin>)>>,
    cursor: ResMut<Cursor>,
    mut commands: Commands,
) {
    for (wire, mut wire_path, _, wire_entity) in q_wires.iter_mut() {
        if let Some(cursor_world_pos) = cursor.world_pos {
            let Some(wire_src_pin_entity) = wire.src_pin else {
                commands.entity(wire_entity).despawn();
                return;
            };

            if let Some(wire_dest_pin_entity) = wire.dest_pin {
                if let (Ok(wire_src_pin_transform), Ok(wire_dest_pin_transform)) = (
                    q_src_pins.get(wire_src_pin_entity),
                    q_dest_pins.get(wire_dest_pin_entity),
                ) {
                    let new_wire = shapes::Line(
                        wire_src_pin_transform.translation().truncate(),
                        wire_dest_pin_transform.translation().truncate(),
                    );

                    *wire_path = ShapePath::build_as(&new_wire);
                } else {
                    commands.entity(wire_entity).despawn();
                    return;
                }
            } else if let CursorState::DraggingWire(dragged_wire) = cursor.state {
                if dragged_wire.eq(&wire_entity) {
                    if let Ok(wire_src_pin_transform) = q_src_pins.get(wire_src_pin_entity) {
                        let new_wire = shapes::Line(
                            wire_src_pin_transform.translation().truncate(),
                            cursor_world_pos,
                        );

                        *wire_path = ShapePath::build_as(&new_wire);
                    }
                } else {
                    commands.entity(wire_entity).despawn();
                }
            } else {
                commands.entity(wire_entity).despawn();
            }
        }
    }
}
