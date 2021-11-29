mod homepage;
mod pak_cli;
mod project;
fn main() {
    let options = eframe::NativeOptions {
        drag_and_drop_support: true,
        ..Default::default()
    };
    eframe::run_native(Box::new(homepage::WelcomeScreen::default()), options);
}