use super::{
    bounding_box::BoundingBox, render_settings::CircuitBoardRenderingSettings,
    signal_state::SignalState,
};
use bevy::prelude::*;
use bevy_prototype_lyon::{draw::Fill, entity::ShapeBundle, prelude::GeometryBuilder, shapes};
use std::ops::{Deref, DerefMut};

#[derive(Reflect)]
pub enum PinType {
    Input,
    Output,
}

#[derive(Reflect)]
pub struct PinModel {
    pub signal_state: SignalState,
    pub pin_type: PinType,
    pub label: String,
}

#[derive(Component, Reflect)]
#[reflect(Component)]
pub struct PinModelCollection(pub Vec<PinModel>);

impl Deref for PinModelCollection {
    type Target = Vec<PinModel>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for PinModelCollection {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

#[derive(Component)]
pub struct PinCollection;

#[derive(Component)]
pub struct PinView {
    pub pin_index: usize,
}

impl PinView {
    pub fn new(index: usize) -> Self {
        Self { pin_index: index }
    }
}

#[derive(Bundle)]
pub struct PinViewBundle {
    pin_view: PinView,
    shape_bundle: ShapeBundle,
    fill: Fill,
    bounding_box: BoundingBox,
}

impl PinViewBundle {
    pub fn new(
        render_settings: &CircuitBoardRenderingSettings,
        pin_index: usize,
        radius: f32,
        translation: Vec3,
    ) -> Self {
        Self {
            pin_view: PinView::new(pin_index),
            shape_bundle: ShapeBundle {
                path: GeometryBuilder::build_as(&shapes::Circle {
                    radius,
                    ..default()
                }),
                spatial: SpatialBundle {
                    transform: Transform::from_translation(translation),
                    ..default()
                },
                ..default()
            },
            fill: Fill::color(render_settings.pin_color),
            bounding_box: BoundingBox::circle_new(radius, false),
        }
    }
}
