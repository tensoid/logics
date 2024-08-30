use super::{
    bounding_box::BoundingBox, render_settings::CircuitBoardRenderingSettings,
    signal_state::SignalState,
};
use bevy::prelude::*;
use bevy_prototype_lyon::{draw::Fill, entity::ShapeBundle, prelude::GeometryBuilder, shapes};
use std::ops::{Deref, DerefMut};
use uuid::Uuid;

#[derive(Reflect, PartialEq, Clone)]
pub enum PinType {
    Input,
    Output,
}

#[derive(Reflect, Clone)]
pub struct PinModel {
    pub signal_state: SignalState,
    pub pin_type: PinType,
    pub label: String,
    pub uuid: Uuid,
}

impl PinModel {
    /// Creates a new PinModel with [`PinType::Input`].
    /// This does not create a [`Uuid`] and instead sets the uuid field to all zeros.
    pub fn new_input(label: String) -> Self {
        Self {
            label,
            pin_type: PinType::Input,
            signal_state: SignalState::Low,
            uuid: Uuid::nil(),
        }
    }

    /// Creates a new PinModel with [`PinType::Output`].
    /// This does not create a [`Uuid`] and instead sets the uuid field to all zeros.
    pub fn new_output(label: String) -> Self {
        Self {
            label,
            pin_type: PinType::Output,
            signal_state: SignalState::Low,
            uuid: Uuid::nil(),
        }
    }
}

#[derive(Component, Reflect, Clone)]
#[reflect(Component)]
pub struct PinModelCollection(pub Vec<PinModel>);

impl PinModelCollection {
    pub fn get_model(&self, uuid: Uuid) -> Option<&PinModel> {
        self.iter().find(|m| m.uuid.eq(&uuid))
    }

    pub fn get_model_mut(&mut self, uuid: Uuid) -> Option<&mut PinModel> {
        self.iter_mut().find(|m| m.uuid.eq(&uuid))
    }

    pub fn iter_inputs(&self) -> impl Iterator<Item = &PinModel> {
        self.iter().filter(|m| m.pin_type.eq(&PinType::Input))
    }

    pub fn iter_inputs_mut(&mut self) -> impl Iterator<Item = &mut PinModel> {
        self.iter_mut().filter(|m| m.pin_type.eq(&PinType::Input))
    }

    pub fn iter_outputs(&self) -> impl Iterator<Item = &PinModel> {
        self.iter().filter(|m| m.pin_type.eq(&PinType::Output))
    }

    pub fn iter_outputs_mut(&mut self) -> impl Iterator<Item = &mut PinModel> {
        self.iter_mut().filter(|m| m.pin_type.eq(&PinType::Input))
    }

    pub fn num_inputs(&self) -> usize {
        self.iter_inputs().count()
    }

    pub fn num_outputs(&self) -> usize {
        self.iter_outputs().count()
    }

    /// Creates new uuids for all pins to avoid spawning two builtin chips with equal pin uuids.
    pub fn randomize_pin_uuids(&mut self) {
        self.iter_mut().for_each(|m| m.uuid = Uuid::new_v4());
    }
}

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

#[derive(Bundle)]
pub struct PinCollectionBundle {
    pin_collection: PinCollection,
    spatial_bundle: SpatialBundle,
}

impl PinCollectionBundle {
    pub fn new() -> Self {
        Self {
            pin_collection: PinCollection,
            spatial_bundle: SpatialBundle::default(),
        }
    }
}

#[derive(Component)]
pub struct PinView {
    pub uuid: Uuid,
}

impl PinView {
    pub fn new(uuid: Uuid) -> Self {
        Self { uuid }
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
        uuid: Uuid,
        radius: f32,
        translation: Vec3,
    ) -> Self {
        Self {
            pin_view: PinView::new(uuid),
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
