use bevy_ecs::component::TableStorage;
use bevy_ecs::prelude::Component;
use bevy_ecs::world::EntityMut;
use enum_dispatch::enum_dispatch;
use serde::de::DeserializeOwned;
use serde::Serialize;

use crate::serialize::writer::WriteBuffer;
use crate::BitSerializable;

// client writes an Enum containing all their message type
// each message must derive message

// that big enum will implement MessageProtocol via a proc macro
// TODO: remove the extra  Serialize + DeserializeOwned + Clone  bounds
pub trait ComponentProtocol:
    BitSerializable + Serialize + DeserializeOwned + ComponentBehaviour
{
    // fn insert(self, entity: &mut EntityMut);
}

#[enum_delegate::register]
pub trait ComponentBehaviour {
    /// Insert the component for an entity
    fn insert(self, entity: &mut EntityMut);
}

impl<T: Component> ComponentBehaviour for T {
    fn insert(self, entity: &mut EntityMut) {
        entity.insert(self);
    }
}

pub trait ComponentProtocolKind: BitSerializable + Serialize + DeserializeOwned {}
