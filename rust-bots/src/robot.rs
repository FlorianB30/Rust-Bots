// #[derive(Debug, Clone, Hash, Eq, PartialEq)]
// pub struct Robot {
//     pub id: u32,
//     pub collected_resources: Vec<ResourceUnit>, // Ressources collectées par le robot
// }

// impl Robot {
//     pub fn new() -> Self {
//         static NEXT_ID: u32 = 0; // Un identifiant unique pour chaque robot
//         Robot {
//             id: NEXT_ID,
//             collected_resources: Vec::new(),
//         }
//     }

//     pub fn collect_resource(&mut self, resource: ResourceUnit) {
//         self.collected_resources.push(resource);
//     }
// }
// src/robot.rs

use crate::resource::Resource;
use crate::station::Station;

#[derive(Debug)]
pub struct Robot {
    pub id: u32,              // Identifiant unique du robot
    pub position: (usize, usize), // Position du robot sur la carte
    pub resources_collected: Vec<Resource>, // Liste des ressources collectées par le robot
}

impl Robot {
    // Crée un robot avec un identifiant et une position initiale
    pub fn new(id: u32, position: (usize, usize)) -> Self {
        Robot {
            id,
            position,
            resources_collected: Vec::new(),
        }
    }

    // Méthode pour explorer une position et collecter des ressources
    pub fn explore(&mut self, resource: Option<Resource>, station: &mut Station) {
        if let Some(r) = resource {
            self.resources_collected.push(r);
            station.collect_resources(&r);
        }
    }

    // Méthode pour rapporter les données scientifiques à la station
    pub fn report_data(&self, station: &mut Station) {
        for resource in &self.resources_collected {
            if let Resource::ScientificData = resource {
                station.store_scientific_data("Discovered new scientific data".to_string());
            }
        }
    }

    // Méthode pour retourner à la station
    pub fn return_to_station(&self, station: &mut Station) {
        for resource in &self.resources_collected {
            station.collect_resources(resource);
        }
    }
}
