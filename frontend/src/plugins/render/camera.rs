use bevy::{
    color::palettes::css::{BLACK, BLUE, WHITE},
    prelude::*,
};

pub struct RCameraPlugin;
impl Plugin for RCameraPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, setup_camera);
        app.insert_resource(ClearColor(BLACK.into()));
    }
}

fn setup_camera(mut commands: Commands) {
    //     // light
    let point_light = PointLight {
        shadows_enabled: false,
        ..default()
    };

    let mut directional_light = DirectionalLight::default();
    directional_light.shadows_enabled = true;
    directional_light.illuminance = 1000.0;

    commands.spawn((Transform::from_xyz(4.0, 2.0, 4.0), directional_light));
    //     // camera

    let mut projection = OrthographicProjection::default_3d();
    projection.scale = 0.008;

    commands.spawn((
        Camera3d::default(),
        Projection::Orthographic(projection),
        Transform::from_xyz(-5.0, -5.0, 12.0)
            .looking_at(Vec3::ZERO.with_x(4.2).with_y(4.2), Vec3::Z),
    ));
}
