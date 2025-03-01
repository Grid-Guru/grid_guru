use crate::plugins::render::mouse::MouseZoom;
use bevy::prelude::*;

pub struct RCameraPlugin;
impl Plugin for RCameraPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(ClearColor(Color::srgb_u8(30, 30, 30)));
        // Using simple values for zoom
        app.insert_resource(MouseZoom::new(0.5, 2.0, 0.01));
        app.add_systems(Startup, setup_camera);
    }
}

fn setup_camera(mut commands: Commands) {
    // lights
    let mut directional_light = DirectionalLight::default();
    directional_light.shadows_enabled = true;
    directional_light.illuminance = 2000.0;
    // Adjust light position for isometric view
    commands.spawn((Transform::from_xyz(4.0, 4.0, 8.0), directional_light));

    let point_light = PointLight {
        shadows_enabled: false,
        intensity: 2000.0,
        ..default()
    };
    let light_location = Transform::from_xyz(4.2, 4.2, 6.0);
    commands.spawn((
        light_location.looking_at(Vec3::new(4.2, 4.2, 0.0), Vec3::Z),
        point_light,
    ));

    // camera
    let mut projection = OrthographicProjection::default_3d();
    // Adjust scale for isometric view
    projection.scale = 0.01;

    // Set up isometric camera position
    // For true isometric, we need a specific angle (approximately 35.264 degrees from horizontal)
    // and positioned equidistant from all axes
    // let camera_location = Transform::from_xyz(8.0, 8.0, 8.0);
    let camera_location = Transform::from_xyz(-5.0, -5.0, 12.0);

    commands.spawn((
        Camera3d::default(),
        Projection::Orthographic(projection),
        // Look at the center of the grid for isometric view
        camera_location.looking_at(Vec3::new(4.2, 4.2, 0.0), Vec3::Z),
    ));
}
