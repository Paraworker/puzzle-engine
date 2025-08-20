use rfd::FileDialog;
use rulery::{CheckedGameRules, UncheckedGameRules, expr::boolean::BoolExpr};
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

                // Load and check rules
                let checked =
                    match UncheckedGameRules::load(&path).and_then(|unchecked| unchecked.check()) {
                        Ok(rules) => rules,
                        Err(err) => {
                            Self::show_dialog("Open Rule Failed".into(), err.to_shared_string());
                            return;
                        }
                    };

                let ui = ui.upgrade().unwrap();

                // Set ui data
                if let Err(str) = Self::set_ui_from_rules(&ui, &checked) {
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
                let rules = CheckedGameRules::default();
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

    fn collect_rules_from_ui(ui: AppWindow) -> Result<CheckedGameRules, String> {
        let mut unchecked = UncheckedGameRules::default();

        // Name
        unchecked.set_name(ui.get_rules_name().to_string());

        // Board
        let Ok(rows) = ui.get_board_rows().parse() else {
            return Err("invalid board rows".to_string());
        };

        let Ok(cols) = ui.get_board_cols().parse() else {
            return Err("invalid board columns".to_string());
        };

        unchecked.set_board_rows(rows);
        unchecked.set_board_cols(cols);

        // Pieces
        unchecked
            .set_pieces_from_ron_str(&ui.get_pieces())
            .map_err(|err| err.to_string())?;

        // Players
        unchecked
            .set_players_from_ron_str(&ui.get_players())
            .map_err(|err| err.to_string())?;

        // Initial Layout
        unchecked
            .set_initial_layout_from_ron_str(&ui.get_initial_layout())
            .map_err(|err| err.to_string())?;

        // Game over condition
        let cond = BoolExpr::from_ron_str(&ui.get_game_over_condition())
            .map_err(|err| format!("Game Over Condition: {}", err))?;

        unchecked.set_game_over_condition(cond);

        unchecked.check().map_err(|err| err.to_string())
    }

    fn set_ui_from_rules(ui: &AppWindow, rules: &CheckedGameRules) -> Result<(), String> {
        let pieces = rules.pieces_to_ron_str().map_err(|err| err.to_string())?;

        let players = rules.players_to_ron_str().map_err(|err| err.to_string())?;

        let initial_layout = rules
            .initial_layout_to_ron_str()
            .map_err(|err| err.to_string())?;

        let game_over_condition = rules
            .game_over_condition_to_ron_str()
            .map_err(|err| err.to_string())?;

        ui.set_rules_name((rules.name()).into());
        ui.set_board_rows(rules.board_rows().to_string().into());
        ui.set_board_cols(rules.board_cols().to_string().into());
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
