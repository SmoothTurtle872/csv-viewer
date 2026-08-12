use iced::Task;
pub trait Page {
    type Message;
    type AppMessage;

    fn view(&self, ctx: &super::App) -> iced::Element<'_, Self::Message>;

    fn update(&mut self, message: Self::Message) -> Task<Self::AppMessage>;
}

pub enum Screen {
    Homepage(homepage::Screen),
    Viewer(viewer::Screen),
}

impl Default for Screen {
    fn default() -> Self {
        Self::Homepage(homepage::Screen::default())
    }
}

mod homepage;
mod viewer;

pub type HomepageMessage = homepage::Message;
pub type ViewerMessage = viewer::Message;

pub type ViewScreen = viewer::Screen;
pub type HomepageScreen = homepage::Screen;
