use bevy::input::mouse::MouseButton;
use bevy::prelude::*;

use super::constants::{XMUL, YMUL};
use super::tile::Tile;

// Import the ClaimTilePosition resource from the networking module
use crate::plugins::networking::starknet_call::ClaimTilePosition;

pub struct HighlightPlugin;
impl Plugin for HighlightPlugin {
    fn build(&self, app: &mut App) {
        app.register_type::<Highlightable>()
            .register_type::<HighlightState>()
            .init_resource::<HighlightState>()
            .add_systems(Update, toggle_highlight_system)
            .add_systems(Update, hover_highlight_system)
            .add_systems(Update, update_claim_tile_position);
    }
}

// Component to mark an entity as highlightable
#[derive(Component, Debug, Reflect, Default)]
pub struct Highlightable {
    pub is_highlighted: bool,
    pub is_hovered: bool,
}

// Resource to track highlight state
#[derive(Resource, Debug, Default, Reflect)]
pub struct HighlightState {
    pub highlighted_entity: Option<Entity>,
    pub hovered_entity: Option<Entity>,
}

// System to update the ClaimTilePosition resource based on the highlighted tile
fn update_claim_tile_position(
    highlight_state: Res<HighlightState>,
    highlightables: Query<&Tile>,
    mut claim_tile_position: ResMut<ClaimTilePosition>,
) {
    // Only update if we have a highlighted entity
    if let Some(highlighted_entity) = highlight_state.highlighted_entity {
        // Get the tile component to access grid coordinates
        if let Ok(tile) = highlightables.get(highlighted_entity) {
            // Convert grid coordinates to static strings that can be used by the Starknet call
            // We need to convert the i32 grid coordinates to static strings
            let x_str = match tile.grid_x {
                0 => "0",
                1 => "1",
                2 => "2",
                3 => "3",
                4 => "4",
                5 => "5",
                6 => "6",
                7 => "7",
                _ => "0", // Default to 0 for any other value
            };

            let y_str = match tile.grid_y {
                0 => "0",
                1 => "1",
                2 => "2",
                3 => "3",
                4 => "4",
                5 => "5",
                6 => "6",
                7 => "7",
                _ => "0", // Default to 0 for any other value
            };

            // Update the ClaimTilePosition resource
            claim_tile_position.x = x_str;
            claim_tile_position.y = y_str;

            // Log the update for debugging
            info!("Updated claim tile position to ({}, {})", x_str, y_str);
        }
    }
}

// System to handle hover highlights via raycasting
fn hover_highlight_system(
    mut commands: Commands,
    windows: Query<&Window>,
    cameras: Query<(&Camera, &GlobalTransform)>,
    mut highlightables: Query<(Entity, &GlobalTransform, &mut Highlightable, &Tile)>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut highlight_state: ResMut<HighlightState>,
    highlight_query: Query<(Entity, &HighlightMarker)>,
) {
    // Get the primary window
    let window = windows.single();

    // Get the cursor position
    let Some(cursor_position) = window.cursor_position() else {
        return;
    };

    // Get the camera
    let (camera, camera_transform) = cameras.single();

    // Cast a ray from the cursor position
    let Ok(ray) = camera.viewport_to_world(camera_transform, cursor_position) else {
        return;
    };

    // Find the closest highlightable entity that intersects with the ray
    let mut closest_entity = None;
    let mut closest_distance = f32::MAX;

    for (entity, transform, _, tile) in highlightables.iter() {
        // Create a simple bounding box for the entity
        let position = transform.translation();
        // Apply the Y offset but use FULL tile size for detection to make hovering easier
        let adjusted_position = Vec3::new(position.x, position.y + 0.45, position.z);
        // Use full tile size for detection area, making it easier to hover
        let size = Vec3::new(XMUL, YMUL, 0.2); // Full tile size for detection

        // Check if the ray intersects with the entity's bounding box
        let t_min = (adjusted_position - size / 2.0 - ray.origin) / *ray.direction;
        let t_max = (adjusted_position + size / 2.0 - ray.origin) / *ray.direction;

        let t1 = t_min.min(t_max);
        let t2 = t_min.max(t_max);

        let t_near = t1.x.max(t1.y).max(t1.z);
        let t_far = t2.x.min(t2.y).min(t2.z);

        if t_near <= t_far && t_far > 0.0 && t_near < closest_distance {
            closest_distance = t_near;
            closest_entity = Some((entity, tile.grid_x, tile.grid_y));
        }
    }

    // Clear hover state for previously hovered entity if it's different
    if let Some(prev_hovered) = highlight_state.hovered_entity {
        if closest_entity.is_none() || closest_entity.as_ref().unwrap().0 != prev_hovered {
            // Remove hover state from previous entity
            if let Ok((_, _, mut highlightable, _)) = highlightables.get_mut(prev_hovered) {
                highlightable.is_hovered = false;
            }

            // Remove any hover highlights
            for (highlight_entity, marker) in highlight_query.iter() {
                if marker.parent_entity == prev_hovered && marker.is_hover_highlight {
                    commands.entity(highlight_entity).despawn_recursive();
                }
            }

            // Clear the hovered entity in the state
            highlight_state.hovered_entity = None;
        }
    }

    // If we found an entity, add hover highlight (unless it's already clicked/highlighted)
    if let Some((entity, grid_x, grid_y)) = closest_entity {
        // Get mutable reference to the Highlightable component
        if let Ok((_, _, mut highlightable, _)) = highlightables.get_mut(entity) {
            // Skip if this entity is already highlighted by click
            if highlightable.is_highlighted {
                return;
            }

            // Skip if this entity is already hovered
            if highlightable.is_hovered {
                return;
            }

            // Set hover state
            highlightable.is_hovered = true;
            highlight_state.hovered_entity = Some(entity);

            // Create a hover highlight effect (slightly different from click highlight)
            let highlight_mesh = meshes.add(Cuboid::new(XMUL * 0.5, YMUL * 0.5, 0.05));
            let highlight_material = materials.add(StandardMaterial {
                // Light blue color with transparency for hover
                base_color: Color::rgba(0.7, 0.7, 1.0, 0.5),
                // Light blue glow effect
                emissive: LinearRgba::new(0.5, 0.5, 1.0, 0.5),
                alpha_mode: AlphaMode::Blend,
                ..default()
            });

            // Get the exact position of the tile to ensure the highlight is positioned correctly
            if let Ok((_, transform, _, _)) = highlightables.get(entity) {
                let tile_position = transform.translation();

                // Spawn the highlight as a separate entity
                commands.spawn((
                    // Individual components for 3D rendering
                    Mesh3d(highlight_mesh),
                    MeshMaterial3d(highlight_material),
                    // Use the exact tile position but with higher Z
                    Transform::from_xyz(
                        tile_position.x,
                        tile_position.y + 0.45,
                        tile_position.z + 0.5,
                    ),
                    GlobalTransform::default(),
                    Visibility::default(),
                    InheritedVisibility::default(),
                    ViewVisibility::default(),
                    HighlightMarker {
                        parent_entity: entity,
                        is_hover_highlight: true,
                    },
                    Name::new(format!("Hover Highlight for Tile ({}, {})", grid_x, grid_y)),
                ));
            }
        }
    }
}

// System to handle toggling highlights via raycasting
fn toggle_highlight_system(
    mut commands: Commands,
    windows: Query<&Window>,
    mouse_button_input: Res<ButtonInput<MouseButton>>,
    cameras: Query<(&Camera, &GlobalTransform)>,
    mut highlightables: Query<(Entity, &GlobalTransform, &mut Highlightable, &Tile)>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut highlight_state: ResMut<HighlightState>,
    highlight_query: Query<(Entity, &HighlightMarker)>,
) {
    // Only process clicks
    if !mouse_button_input.just_pressed(MouseButton::Left) {
        return;
    }

    // Get the primary window
    let window = windows.single();

    // Get the cursor position
    let Some(cursor_position) = window.cursor_position() else {
        return;
    };

    // Get the camera
    let (camera, camera_transform) = cameras.single();

    // Cast a ray from the cursor position
    let Ok(ray) = camera.viewport_to_world(camera_transform, cursor_position) else {
        return;
    };

    // Find the closest highlightable entity that intersects with the ray
    let mut closest_entity = None;
    let mut closest_distance = f32::MAX;

    for (entity, transform, _, tile) in highlightables.iter() {
        // Create a simple bounding box for the entity
        let position = transform.translation();
        // Apply the Y offset but use FULL tile size for detection to make clicking easier
        let adjusted_position = Vec3::new(position.x, position.y + 0.45, position.z);
        // Use full tile size for detection area, making it easier to click
        let size = Vec3::new(XMUL, YMUL, 0.2); // Full tile size for detection

        // Check if the ray intersects with the entity's bounding box
        let t_min = (adjusted_position - size / 2.0 - ray.origin) / *ray.direction;
        let t_max = (adjusted_position + size / 2.0 - ray.origin) / *ray.direction;

        let t1 = t_min.min(t_max);
        let t2 = t_min.max(t_max);

        let t_near = t1.x.max(t1.y).max(t1.z);
        let t_far = t2.x.min(t2.y).min(t2.z);

        if t_near <= t_far && t_far > 0.0 && t_near < closest_distance {
            closest_distance = t_near;
            closest_entity = Some((entity, tile.grid_x, tile.grid_y));
        }
    }

    // First, unhighlight any currently highlighted entity
    if let Some(current_highlighted) = highlight_state.highlighted_entity {
        // Set the highlight state to false
        if let Ok((_, _, mut highlightable, _)) = highlightables.get_mut(current_highlighted) {
            highlightable.is_highlighted = false;
        }

        // Remove the highlight entity
        for (highlight_entity, marker) in highlight_query.iter() {
            if marker.parent_entity == current_highlighted && !marker.is_hover_highlight {
                commands.entity(highlight_entity).despawn_recursive();
            }
        }

        // Clear the highlighted entity in the state
        highlight_state.highlighted_entity = None;
    }

    // If we found an entity, handle its highlight
    if let Some((entity, grid_x, grid_y)) = closest_entity {
        // Get mutable reference to the Highlightable component
        if let Ok((_, _, mut highlightable, _)) = highlightables.get_mut(entity) {
            // If clicking on the already highlighted tile, just unhighlight it (already done above)
            if highlightable.is_highlighted {
                // Just log the unhighlight
                info!(
                    "Unhighlighted tile at grid position ({}, {})",
                    grid_x, grid_y
                );
                return;
            }

            // Otherwise, highlight the new tile
            highlightable.is_highlighted = true;
            highlight_state.highlighted_entity = Some(entity);

            // Create a highlight effect
            let highlight_mesh = meshes.add(Cuboid::new(XMUL * 0.5, YMUL * 0.5, 0.05));
            let highlight_material = materials.add(StandardMaterial {
                // Change color to white with some transparency
                base_color: Color::rgba(1.0, 1.0, 1.0, 0.7),
                // White glow effect
                emissive: LinearRgba::new(1.0, 1.0, 1.0, 1.0),
                alpha_mode: AlphaMode::Blend,
                ..default()
            });

            // Get the exact position of the tile to ensure the highlight is positioned correctly
            if let Ok((_, transform, _, _)) = highlightables.get(entity) {
                let tile_position = transform.translation();

                // Spawn the highlight as a separate entity
                commands.spawn((
                    // Individual components for 3D rendering
                    Mesh3d(highlight_mesh),
                    MeshMaterial3d(highlight_material),
                    // Use the exact tile position but with higher Z
                    Transform::from_xyz(
                        tile_position.x,
                        tile_position.y + 0.45,
                        tile_position.z + 0.5,
                    ),
                    GlobalTransform::default(),
                    Visibility::default(),
                    InheritedVisibility::default(),
                    ViewVisibility::default(),
                    HighlightMarker {
                        parent_entity: entity,
                        is_hover_highlight: false,
                    },
                    Name::new(format!("Highlight for Tile ({}, {})", grid_x, grid_y)),
                ));

                info!("Highlighted tile at grid position ({}, {})", grid_x, grid_y);
            }
        }
    }
}

// Marker component for highlight entities with reference to parent
#[derive(Component)]
pub struct HighlightMarker {
    pub parent_entity: Entity,
    pub is_hover_highlight: bool,
}

// System to add Highlightable component to tiles
pub fn make_tiles_highlightable(
    mut commands: Commands,
    tiles: Query<Entity, (With<Tile>, Without<Highlightable>)>,
) {
    for entity in tiles.iter() {
        commands.entity(entity).insert(Highlightable::default());
    }
}
