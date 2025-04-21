use crate::resources::ResourceType;
use bevy::prelude::Resource;

#[derive(Resource, Default)]
pub struct Memory {
    pub discovered: Vec<(usize, usize, ResourceType)>,
}

impl Memory {
    pub fn add(&mut self, x: usize, y: usize, resource: ResourceType) {
        if !self.discovered.contains(&(x, y, resource)) {
            self.discovered.push((x, y, resource));
        }
    }

    pub fn clear(&mut self) {
        self.discovered.clear();
    }

    pub fn len(&self) -> usize {
        self.discovered.len()
    }
}
