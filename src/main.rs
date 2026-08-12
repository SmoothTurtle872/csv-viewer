use iced::application;

use csv_viewer::app::App;

fn main() -> iced::Result {
    application(App::new, App::update, App::view).run()
}
