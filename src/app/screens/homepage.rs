use iced::{
    Fill, Font, Task,
    font::{Family, Stretch, Style, Weight},
    widget::{Button, column, container, text},
};

#[derive(Debug, Default)]
pub struct Screen {
    pub opening: bool,
}

impl super::Page for Screen {
    type Message = Message;
    type AppMessage = super::super::Message;
    fn update(&mut self, message: Self::Message) -> Task<Self::AppMessage> {
        match message {
            Message::OpenRepo => {
                _ = open::that(env!("CARGO_PKG_REPOSITORY"));
                Task::none()
            }
            Message::OpenFile => {
                self.opening = true;
                Task::perform(crate::file::open_file(), Self::AppMessage::FileOpened)
            }
        }
    }

    fn view(&self, _ctx: &crate::app::App) -> iced::Element<'_, Self::Message> {
        container(
            column![
                container(
                    column![
                        text!("Welcome to CSV Viewer")
                            .width(Fill)
                            .center()
                            .font(Font {
                                family: Family::Monospace,
                                weight: Weight::ExtraBold,
                                stretch: Stretch::Expanded,
                                style: Style::Normal
                            })
                            .size(30),
                        container(Button::new(text!("Repository")).on_press(Message::OpenRepo))
                            .center_x(Fill)
                    ]
                    .spacing(10)
                ),
                container(if !self.opening {
                    Button::new(text!("Open File")).on_press(Message::OpenFile)
                } else {
                    Button::new(text!("Open File"))
                })
                .center(Fill)
                .style(|theme| { container::bordered_box(&theme) })
            ]
            .spacing(10),
        )
        .style(|theme| container::bordered_box(&theme))
        .padding(10)
        .into()
    }
}

#[derive(Debug, Clone)]
pub enum Message {
    OpenRepo,
    OpenFile,
}
