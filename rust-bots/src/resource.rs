// #[derive(Debug, Hash, Eq, PartialEq, Clone)]
// pub enum Resource {
//     Energy,
//     Minerals,
//     ScientificInterest,
// }

// #[derive(Debug, Clone)]
// pub struct ResourceUnit {
//     pub resource_type: Resource,
//     pub quantity: u32,
// }

// impl ResourceUnit {
//     pub fn new(resource_type: Resource, quantity: u32) -> Self {
//         ResourceUnit { resource_type, quantity }
//     }
// }
// src/resource.rs

#[derive(Debug, Clone, Copy)]
pub enum Resource {
    Energy,
    Minerals,
    ScientificData,
}

impl Resource {
    pub fn to_string(&self) -> &'static str {
        match self {
            Resource::Energy => "Energy",
            Resource::Minerals => "Minerals",
            Resource::ScientificData => "Scientific Data",
        }
    }
}
