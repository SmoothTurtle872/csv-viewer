use iced::{
    Length, Task,
    widget::{button, column, container, text},
};
use iced_aw::{MenuBar, menu, menu_items};

mod screens;
use screens::{Page, Screen};

use crate::file::FileError;

pub struct App {
    screen: screens::Screen,
}

impl App {
    pub fn view(&self) -> iced::Element<'_, Message, iced::Theme, iced::Renderer> {
        column![
            self.header(),
            match &self.screen {
                Screen::Homepage(page) => page.view(&self).map(Message::HomepageMessage),
                Screen::Viewer(page) => page.view(&self).map(Message::ViewerMessage),
            }
        ]
        .into()
    }

    fn header(&self) -> iced::Element<'_, Message, iced::Theme, iced::Renderer> {
        let file_items = menu_items!((button("open").on_press(Message::FileButtonPressed)));
        let file_menu = menu::Menu::new(file_items)
            .width(Length::Shrink)
            .spacing(10);
        let preferences_items = menu_items!((button("theme")));
        let preferences_menu = menu::Menu::new(preferences_items)
            .width(Length::Shrink)
            .spacing(10);
        let header = MenuBar::new(menu_items!(
            (container(text("File")), file_menu),
            (container(text("Preferences")), preferences_menu)
        ))
        .spacing(10)
        .padding(10)
        .draw_path(menu::DrawPath::Backdrop);
        header.into()
    }

    pub fn update(&mut self, message: Message) -> Task<Message> {
        match message {
            Message::FileButtonPressed => {
                if let Screen::Homepage(screen) = &mut self.screen {
                    screen.opening = true;
                }
                Task::perform(super::file::open_file(), Message::FileOpened)
            }
            Message::HomepageMessage(message) => {
                if let Screen::Homepage(screen) = &mut self.screen {
                    screen.update(message)
                } else {
                    Task::none()
                }
            }

            Message::ViewerMessage(message) => {
                if let Screen::Viewer(screen) = &mut self.screen {
                    screen.update(message)
                } else {
                    Task::none()
                }
            }
            Message::FileOpened(file) => match file {
                Ok(file) => {
                    self.screen = Screen::Viewer(screens::ViewScreen {
                        file,
                        selling: false,
                    });
                    Task::none()
                }
                Err(err) => Task::perform(display_error(err), Message::AcceptedError),
            },
            Message::AcceptedError(_) => {
                if let Screen::Homepage(screen) = &mut self.screen {
                    screen.opening = false;
                }
                Task::none()
            }
            Message::SoldAsNFT(_) => {
                self.screen = Screen::Homepage(screens::HomepageScreen { opening: false });
                Task::none()
            }
        }
    }

    pub fn new() -> Self {
        Self {
            screen: Screen::default(),
        }
    }
}

#[derive(Debug, Clone)]
pub enum Message {
    FileButtonPressed,
    FileOpened(Result<super::file::File, super::file::FileError>),
    HomepageMessage(screens::HomepageMessage),
    ViewerMessage(screens::ViewerMessage),
    AcceptedError(()),
    SoldAsNFT(()),
}

async fn display_error(err: FileError) -> () {
    _ = rfd::AsyncMessageDialog::new()
        .set_description(format!(
            "An error occured while opening the specified files:\n{err}"
        ))
        .set_title("Error")
        .set_level(rfd::MessageLevel::Warning)
        .set_buttons(rfd::MessageButtons::Ok)
        .show()
        .await;
}
