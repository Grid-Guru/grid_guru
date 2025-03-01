use bevy::{input::mouse::MouseWheel, prelude::*};

pub struct RMousePlugin;
impl Plugin for RMousePlugin {
    fn build(&self, app: &mut App) {
        app.register_type::<MouseZoom>()
            .insert_resource(MouseZoom::default())
            .add_systems(Update, handle_mouse_wheel)
            .add_systems(Update, handle_zoom_reset);
    }
}

#[derive(Resource, Debug, Reflect, Default)]
pub struct MouseZoom {
    pub zoom_level: f32,
    pub min_zoom: f32,
    pub max_zoom: f32,
    pub zoom_speed: f32,
}

impl MouseZoom {
    pub fn new(min_zoom: f32, max_zoom: f32, zoom_speed: f32) -> Self {
        Self {
            zoom_level: 1.0,
            min_zoom,
            max_zoom,
            zoom_speed,
        }
    }

    // Reset zoom level to default (1.0)
    pub fn reset(&mut self) {
        self.zoom_level = 1.0;
    }
}

// System to handle zoom reset when Z key is pressed
fn handle_zoom_reset(
    keyboard_input: Res<ButtonInput<KeyCode>>,
    mut mouse_zoom: ResMut<MouseZoom>,
    mut query: Query<&mut Projection>,
) {
    if keyboard_input.just_pressed(KeyCode::KeyZ) {
        // Reset zoom level to default
        mouse_zoom.reset();

        // Apply reset zoom to all orthographic cameras
        for mut projection in query.iter_mut() {
            if let Projection::Orthographic(ref mut ortho) = *projection {
                // Use the isometric default scale
                ortho.scale = 0.01;
                info!(
                    "Zoom reset to default isometric view. Camera scale: {}",
                    ortho.scale
                );
            }
        }
    }
}

fn handle_mouse_wheel(
    mut mouse_wheel_events: EventReader<MouseWheel>,
    mut mouse_zoom: ResMut<MouseZoom>,
    mut query: Query<&mut Projection>,
) {
    for event in mouse_wheel_events.read() {
        // Use a very small fixed delta to make zooming more controlled
        let zoom_delta = event.y.signum() * 0.1;

        // Update zoom level with the fixed small delta
        mouse_zoom.zoom_level -= zoom_delta;

        // Ensure zoom level stays within safe bounds
        mouse_zoom.zoom_level = mouse_zoom.zoom_level.clamp(0.5, 2.0);

        // Apply zoom to all orthographic projections
        for mut projection in query.iter_mut() {
            if let Projection::Orthographic(ref mut ortho) = *projection {
                // Set the scale based on zoom level, using the isometric base scale
                ortho.scale = 0.01 * (1.0 / mouse_zoom.zoom_level);

                // Log the current zoom level and scale
                info!(
                    "Zoom level: {}, Isometric camera scale: {}",
                    mouse_zoom.zoom_level, ortho.scale
                );
            }
        }
    }
}
