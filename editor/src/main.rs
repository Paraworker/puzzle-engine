use crate::app::App;

mod app;

fn main() -> Result<(), slint::PlatformError> {
    App::new().run()
}
