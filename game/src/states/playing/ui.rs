use crate::states::{
    AppState,
    playing::{
        phases::{GamePhase, placing::start_place_piece},
        session::GameSession,
    },
};
use bevy::prelude::*;
use bevy_egui::{
    EguiContexts,
    egui::{self, Stroke},
};

#[derive(Resource)]
pub struct TopPanelText(pub String);

pub fn top_panel(mut egui: EguiContexts, text: Res<TopPanelText>) {
    egui::TopBottomPanel::top("top_panel")
        .frame(
            egui::Frame::NONE
                .fill(egui::Color32::from_rgba_premultiplied(30, 30, 30, 192))
                .stroke(Stroke::NONE)
                .inner_margin(egui::Margin::symmetric(0, 10)),
        )
        .show(egui.ctx_mut().unwrap(), |ui_ctx| {
            ui_ctx.with_layout(
                egui::Layout::centered_and_justified(egui::Direction::LeftToRight),
                |ui_ctx| ui_ctx.label(egui::RichText::new(&text.0).strong().size(24.0)),
            );
        });
}

pub fn bottom_panel(
    mut commands: Commands,
    mut egui: EguiContexts,
    mut session: ResMut<GameSession>,
    next_state: Res<NextState<AppState>>,
    current_phase: Res<State<GamePhase>>,
    mut next_phase: ResMut<NextState<GamePhase>>,
) {
    if let NextState::Pending(_) = *next_state {
        return;
    }

    let session = session.as_mut();

    egui::TopBottomPanel::bottom("bottom_panel")
        .frame(
            egui::Frame::NONE
                .fill(egui::Color32::from_rgb(30, 30, 30))
                .inner_margin(egui::Margin::symmetric(10, 8)),
        )
        .show(egui.ctx_mut().unwrap(), |ui| {
            let (piece_color, player) = session.players.get_by_index(session.turn.current_player());

            egui::ScrollArea::horizontal().show(ui, |ui| {
                // Row 1: In Stock
                ui.horizontal(|ui| {
                    ui.label(egui::RichText::new("In Stock").size(18.0).monospace())
                        .on_hover_text("Number of your pieces available for placement");

                    ui.separator();

                    for (model, piece) in player.pieces() {
                        let button = egui::Button::new(
                            egui::RichText::new(format!("{} × {}", model, piece.stock()))
                                .size(18.0)
                                .strong(),
                        );

                        // Enable the button if the session is in selecting state and the count is not depleted
                        let enabled = matches!(current_phase.get(), GamePhase::Selecting)
                            && !piece.stock().is_depleted();

                        if ui.add_enabled(enabled, button).clicked() {
                            // Avoid duplicate transition
                            if let NextState::Unchanged = *next_phase {
                                // Enter placing state
                                start_place_piece(
                                    &mut commands,
                                    &mut next_phase,
                                    model,
                                    piece_color,
                                );
                            }
                        }
                    }
                });

                ui.separator();

                // Row 2: Captured
                ui.horizontal(|ui| {
                    ui.label(egui::RichText::new("Captured").size(18.0).monospace())
                        .on_hover_text("Number of your pieces that have been captured");

                    ui.separator();

                    for (model, piece) in player.pieces() {
                        let button = egui::Button::new(
                            egui::RichText::new(format!("{} × {}", model, piece.captured()))
                                .size(18.0),
                        );

                        ui.add_enabled(false, button);
                    }
                });
            });
        });
}
