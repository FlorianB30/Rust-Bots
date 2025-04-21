use bevy::prelude::*;
use bevy::prelude::{Style, Val, PositionType, JustifyContent, AlignItems, UiRect, TextStyle};

use crate::station::Station;

#[derive(Component)]
struct UiRoot;

#[derive(Component)]
struct InventoryText;

#[derive(Component)]
struct MemoryText;

pub struct UiPlugin;

impl Plugin for UiPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, setup_ui)
           .add_systems(Update, update_ui);
    }
}

fn setup_ui(mut commands: Commands, asset_server: Res<AssetServer>) {
    let font = asset_server.load("fonts/FiraSans-Bold.ttf");

    commands.spawn((
        NodeBundle {
            style: Style {
                width: Val::Percent(100.0),
                height: Val::Px(100.0),
                position_type: PositionType::Absolute,
                justify_content: JustifyContent::SpaceBetween,
                align_items: AlignItems::FlexStart,
                ..default()
            },
            background_color: Color::NONE.into(),
            ..default()
        },
        UiRoot,
    )).with_children(|parent| {
        parent.spawn((
            TextBundle::from_section(
                "Inventory: 0",
                TextStyle {
                    font: font.clone(),
                    font_size: 25.0,
                    color: Color::WHITE,
                },
            ).with_style(Style {
                margin: UiRect::all(Val::Px(10.0)),
                ..default()
            }),
            InventoryText,
        ));
        parent.spawn((
            TextBundle::from_section(
                "Memory: 0 resources",
                TextStyle {
                    font: font.clone(),
                    font_size: 25.0,
                    color: Color::GRAY,
                },
            ).with_style(Style {
                margin: UiRect::all(Val::Px(10.0)),
                ..default()
            }),
            MemoryText,
        ));
    });
}

fn update_ui(
    station: Res<Station>,
    mut query_inventory: Query<&mut Text, With<InventoryText>>,
    mut query_memory: Query<&mut Text, (With<MemoryText>, Without<InventoryText>)>,
) {
    if let Ok(mut text) = query_inventory.get_single_mut() {
        let inventory = station.inventory.lock().unwrap();
        text.sections[0].value = format!("Inventory: {}", *inventory);
    }

    if let Ok(mut text) = query_memory.get_single_mut() {
        let memory = station.memory.lock().unwrap();
        text.sections[0].value = format!("Memory: {} resources", memory.len());
    }
}
