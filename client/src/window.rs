use bevy::prelude::*;
use bevy::window::WindowResolution;

const WINDOW_WIDTH: f32 = 1200.0;
const WINDOW_HEIGHT: f32 = 1200.0;
const WINDOW_TITLE: &str = "Grid Guru - Territory Capture";
const BACKGROUND_COLOR: Color = Color::srgb(0.15, 0.15, 0.15);

pub(super) fn plugin(app: &mut App) {
    let primary_window = Window {
        title: WINDOW_TITLE.into(),
        resolution: WindowResolution::new(WINDOW_WIDTH, WINDOW_HEIGHT)
            .with_scale_factor_override(1.0),
        resizable: true,
        position: WindowPosition::Centered(MonitorSelection::Primary),
        present_mode: bevy::window::PresentMode::AutoVsync,
        fit_canvas_to_parent: true,
        prevent_default_event_handling: false,
        canvas: Some("#bevy".to_owned()),
        focused: true,
        ..default()
    };

    app.insert_resource(ClearColor(BACKGROUND_COLOR))
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(primary_window),
            exit_condition: bevy::window::ExitCondition::OnAllClosed,
            close_when_requested: true,
            ..default()
        }))
        .add_systems(Update, maintain_aspect_ratio);
}

pub fn maintain_aspect_ratio(mut windows: Query<&mut Window>) {
    if let Ok(mut window) = windows.get_single_mut() {
        let size = window.width().min(window.height());
        window.resolution.set(size, size);
    }
}
