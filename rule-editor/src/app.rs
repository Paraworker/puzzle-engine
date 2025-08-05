use rfd::FileDialog;
use rule_engine::{
    GameRules, board::BoardRuleSet, expr::boolean::BoolExpr, initial_layout::InitialLayout,
    piece::PieceRuleSet, player::PlayerRuleSet,
};
use slint::{SharedString, ToSharedString};

slint::include_modules!();

pub struct App {
    ui: AppWindow,
}

impl App {
    pub fn new() -> Self {
        let ui = AppWindow::new().unwrap();

        ui.on_open_rules_clicked({
            let ui = ui.as_weak();
            move || {
                // Open file dialog
                let Some(path) = FileDialog::new()
                    .add_filter("RON file", &["ron"])
                    .pick_file()
                else {
                    // Cancelled
                    return;
                };

                // Load rules
                let rules = match GameRules::load(&path) {
                    Ok(rules) => rules,
                    Err(err) => {
                        Self::show_dialog("Load Failed".into(), err.to_shared_string());
                        return;
                    }
                };

                let ui = ui.upgrade().unwrap();

                // Set ui data
                if let Err(str) = Self::set_ui_from_rules(&ui, &rules) {
                    Self::show_dialog("Open Rule Failed".into(), str.into());
                    return;
                }

                // Show editor ui
                ui.set_show_launcher(false);
            }
        });

        ui.on_create_rules_clicked({
            let ui = ui.as_weak();
            move || {
                // Create a default rules
                let rules = GameRules::default();
                let ui = ui.upgrade().unwrap();

                // Set ui data
                if let Err(str) = Self::set_ui_from_rules(&ui, &rules) {
                    Self::show_dialog("Create Rule Failed".into(), str.into());
                    return;
                }

                // Show editor ui
                ui.set_show_launcher(false);
            }
        });

        ui.on_save_as_activated({
            let ui = ui.as_weak();
            move || {
                // Collect rules from ui
                let rules = match Self::collect_rules_from_ui(ui.upgrade().unwrap()) {
                    Ok(rules) => rules,
                    Err(err) => {
                        Self::show_dialog("Invalid Rules".into(), err.to_shared_string());
                        return;
                    }
                };

                // Open file dialog
                let Some(path) = FileDialog::new()
                    .add_filter("RON file", &["ron"])
                    .set_file_name("untitled.ron")
                    .save_file()
                else {
                    // Cancelled
                    return;
                };

                // Save rules
                if let Err(err) = rules.save(&path) {
                    Self::show_dialog("Save Failed".into(), err.to_shared_string());
                }
            }
        });

        Self { ui }
    }

    pub fn run(&self) -> Result<(), slint::PlatformError> {
        self.ui.run()
    }

    fn collect_rules_from_ui(ui: AppWindow) -> Result<GameRules, String> {
        // Name
        let name = ui.get_rules_name().to_string();
        if name.is_empty() {
            return Err("Empty rules name".to_string());
        }

        // Board
        let board = {
            let Ok(rows) = ui.get_board_rows().parse() else {
                return Err("invalid board rows".to_string());
            };

            let Ok(cols) = ui.get_board_cols().parse() else {
                return Err("invalid board columns".to_string());
            };

            match BoardRuleSet::new(rows, cols) {
                Ok(board) => board,
                Err(err) => {
                    return Err(err.to_string());
                }
            }
        };

        // Pieces
        let pieces = {
            let str = ui.get_pieces();
            if str.is_empty() {
                return Err("Pieces is empty".into());
            }

            match PieceRuleSet::from_ron_str(&str) {
                Ok(cond) => cond,
                Err(err) => return Err(format!("Pieces: {}", err)),
            }
        };

        // Players
        let players = {
            let str = ui.get_players();
            if str.is_empty() {
                return Err("Players is empty".into());
            }

            match PlayerRuleSet::from_ron_str(&str) {
                Ok(cond) => cond,
                Err(err) => return Err(format!("Players: {}", err)),
            }
        };

        // Initial Layout
        let initial_layout = {
            let str = ui.get_initial_layout();
            if str.is_empty() {
                return Err("Initial Layout is empty".into());
            }

            match InitialLayout::from_ron_str(&str) {
                Ok(cond) => cond,
                Err(err) => return Err(format!("Initial Layout: {}", err)),
            }
        };

        // Game over condition
        let game_over_condition = {
            let str = ui.get_game_over_condition();
            if str.is_empty() {
                return Err("Game over condition is empty".into());
            }

            match BoolExpr::from_ron_str(&str) {
                Ok(cond) => cond,
                Err(err) => return Err(format!("Game Over Condition: {}", err)),
            }
        };

        Ok(GameRules {
            name,
            board,
            pieces,
            players,
            initial_layout,
            game_over_condition,
        })
    }

    fn set_ui_from_rules(ui: &AppWindow, rules: &GameRules) -> Result<(), String> {
        let pieces = match rules.pieces.to_ron_str() {
            Ok(cond) => cond,
            Err(err) => return Err(err.to_string()),
        };

        let players = match rules.players.to_ron_str() {
            Ok(cond) => cond,
            Err(err) => return Err(err.to_string()),
        };

        let initial_layout = match rules.initial_layout.to_ron_str() {
            Ok(cond) => cond,
            Err(err) => return Err(err.to_string()),
        };

        let game_over_condition = match rules.game_over_condition.to_ron_str() {
            Ok(cond) => cond,
            Err(err) => return Err(err.to_string()),
        };

        ui.set_rules_name((&rules.name).into());
        ui.set_board_rows(rules.board.rows().to_string().into());
        ui.set_board_cols(rules.board.cols().to_string().into());
        ui.set_pieces(pieces.into());
        ui.set_players(players.into());
        ui.set_initial_layout(initial_layout.into());
        ui.set_game_over_condition(game_over_condition.into());

        Ok(())
    }

    fn show_dialog(topic: SharedString, msg: SharedString) {
        let dialog = Dialog::new().unwrap();

        dialog.set_topic(topic);
        dialog.set_msg(msg);
        dialog.on_ok_clicked({
            let dialog = dialog.as_weak();
            move || {
                dialog.upgrade().unwrap().hide().unwrap();
            }
        });

        dialog.show().unwrap();
    }
}
