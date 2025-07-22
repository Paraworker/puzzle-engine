use crate::states::playing::session::{GameSession, PlayState};
use bevy::{input::mouse::MouseWheel, prelude::*};

#[derive(Component)]
pub struct PlayingCamera {
    /// Distance from focus point
    radius: f32,
    /// Horizontal rotation angle (in radians)
    azimuth: f32,
    /// Vertical rotation angle (in radians)
    elevation: f32,
}

impl PlayingCamera {
    pub const FOCUS: Vec3 = Vec3::ZERO;

    pub fn new() -> Self {
        Self {
            radius: 10.0,
            azimuth: 0.0,
            elevation: std::f32::consts::FRAC_PI_6, // 30°,
        }
    }

    pub fn transform(&self) -> Transform {
        let x = self.radius * self.elevation.cos() * self.azimuth.sin();
        let y = self.radius * self.elevation.sin();
        let z = self.radius * self.elevation.cos() * self.azimuth.cos();

        Transform::from_translation(Vec3::new(x, y, z)).looking_at(Self::FOCUS, Vec3::Y)
    }

    pub fn zoom(&mut self, delta: f32) {
        const ZOOM_SPEED: f32 = 0.2;
        const MIN_DISTANCE: f32 = 5.0;
        const MAX_DISTANCE: f32 = 40.0;

        self.radius -= delta * ZOOM_SPEED;
        self.radius = self.radius.clamp(MIN_DISTANCE, MAX_DISTANCE);
    }

    pub fn drag(&mut self, delta_x: f32, delta_y: f32) {
        self.azimuth -= delta_x * 0.01;
        self.elevation += delta_y * 0.01;
        self.elevation = self
            .elevation
            .clamp(0.1, std::f32::consts::FRAC_PI_2 - 0.05);
    }
}

/// A system that zooms the camera when the mouse wheel is scrolled.
pub fn zoom(
    mut scroll_evr: EventReader<MouseWheel>,
    mut query: Query<(&mut Transform, &mut PlayingCamera)>,
    session: Res<GameSession>,
) {
    if let PlayState::Navigating = session.state {
        for ev in scroll_evr.read() {
            for (mut transform, mut camera) in &mut query {
                camera.zoom(ev.y);

                // Update transform
                *transform = camera.transform();
            }
        }
    }
}

/// A system that orbits the camera around the focus point when dragged.
pub fn orbit(
    mut drag_events: EventReader<Pointer<Drag>>,
    mut camera_query: Query<(&mut Transform, &mut PlayingCamera)>,
    session: Res<GameSession>,
) {
    if let PlayState::Navigating = session.state {
        for drag in drag_events.read() {
            for (mut transform, mut cam) in camera_query.iter_mut() {
                cam.drag(drag.delta.x, drag.delta.y);

                // Update transform
                *transform = cam.transform();
            }
        }
    }
}
