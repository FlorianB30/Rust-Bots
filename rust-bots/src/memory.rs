use bevy::prelude::Resource;
use crate::resources::ResourceType;

#[derive(Resource, Default)]
pub struct Memory {
    pub discoveries: Vec<(usize, usize, ResourceType)>,
    pub collected: usize,
}
