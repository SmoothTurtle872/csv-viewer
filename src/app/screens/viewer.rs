use rand::random_range;
use std::{thread, time::Duration};

use iced::{
    Fill, Task,
    widget::{button, column, container, row, text},
};

use rfd::AsyncMessageDialog;

pub struct Screen {
    pub file: crate::file::File,
    pub selling: bool,
}

impl super::Page for Screen {
    type Message = Message;
    type AppMessage = super::super::Message;
    fn update(&mut self, message: Self::Message) -> Task<Self::AppMessage> {
        match message {
            Self::Message::SellAsNFT => {
                self.selling = true;
                Task::perform(sell_as_nft(), Self::AppMessage::SoldAsNFT)
            }
        }
    }

    fn view(&self, _ctx: &crate::app::App) -> iced::Element<'_, Self::Message> {
        let mut rows = column![];
        for data_row in self.file.get_data() {
            let mut display_row = row![];
            for data in data_row {
                display_row = display_row.push(
                    container(text!("{data}"))
                        .style(|theme| container::bordered_box(theme))
                        .width(Fill)
                        .height(Fill)
                        .center(Fill),
                );
            }
            rows = rows.push(display_row);
        }
        column![
            container(rows.width(Fill).height(Fill))
                .width(Fill)
                .style(|theme| container::bordered_box(theme)),
            container(if !self.selling {
                button("Sell as NFT (REAL)").on_press(Message::SellAsNFT)
            } else {
                button("Selling as NFT...")
            })
            .width(Fill)
            .center_x(Fill)
        ]
        .spacing(10)
        .padding(10)
        .into()
    }
}

#[derive(Debug, Clone)]
pub enum Message {
    SellAsNFT,
}

async fn sell_as_nft() -> () {
    let mut amount = random_range(25..9999);
    loop {
        let choice = AsyncMessageDialog::new()
            .set_buttons(rfd::MessageButtons::YesNo)
            .set_description(format!(
                "We will now sell this as an NFT for ${amount}, is that the right amount?"
            ))
            .set_title("Sell CSV as NFT")
            .set_level(rfd::MessageLevel::Info)
            .show()
            .await;
        match choice {
            rfd::MessageDialogResult::Yes => break,
            _ => amount = random_range(25..9999),
        }
    }

    thread::sleep(Duration::from_secs(5));

    _ = AsyncMessageDialog::new()
        .set_buttons(rfd::MessageButtons::Ok)
        .set_description(format!("Successfully sold NFT for ${amount}!"))
        .set_title("NFT Sold!")
        .set_level(rfd::MessageLevel::Info)
        .show()
        .await;
}
