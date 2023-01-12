mod settings;
use settings::Settings;

fn main() {
    let mut settings = Settings::new();

    settings.load();
    settings.debug();
}
