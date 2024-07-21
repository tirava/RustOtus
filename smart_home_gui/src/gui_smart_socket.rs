use crate::prelude::{SmartDevice, SmartSocket};
use iced::widget::{button, column, container, row, scrollable, text, text_input};
use iced::{alignment, Alignment, Application, Color, Command, Element, Length, Theme};
use once_cell::sync::Lazy;

pub mod prelude {
    pub use crate::gui_smart_socket::SmartSocketGUI;
}

const SOCKET_ADDR: &str = "127.0.0.1:54321";
static MESSAGE_LOG: Lazy<scrollable::Id> = Lazy::new(scrollable::Id::unique);

pub struct SmartSocketGUI {
    messages: Vec<String>,
    address: String,
    state: State,
}

enum State {
    Disconnected,
    Connected,
}

#[derive(Debug, Clone)]
pub enum Message {
    AddressChanged(String),
    CommandSendInfo,
    CommandReceivedInfoOk(String),
    CommandReceivedInfoError(String),
}

impl Application for SmartSocketGUI {
    type Executor = iced::executor::Default;
    type Message = Message;
    type Theme = Theme;
    type Flags = ();

    fn new(_flags: ()) -> (Self, Command<Message>) {
        (
            SmartSocketGUI {
                address: SOCKET_ADDR.to_string(),
                messages: Vec::new(),
                state: State::Disconnected,
            },
            Command::none(),
        )
    }

    fn title(&self) -> String {
        String::from("Smart Socket GUI")
    }

    fn update(&mut self, message: Message) -> Command<Message> {
        match message {
            Message::AddressChanged(value) => {
                self.address = value;
                self.state = State::Disconnected;
            }
            Message::CommandSendInfo => {
                return Command::perform(
                    // todo - use self.address
                    SmartSocket::send_command("127.0.0.1:54321", "info"),
                    |result| match result {
                        Ok(result) => Message::CommandReceivedInfoOk(result),
                        Err(e) => {
                            let err = e.to_string();
                            Message::CommandReceivedInfoError(err)
                        }
                    },
                );
            }
            Message::CommandReceivedInfoOk(result) => {
                // todo delete println -> logs
                println!("GUI: SmartSocket command 'info' - '{}'", result);
                self.state = State::Connected;
                self.messages.push(result);
            }
            Message::CommandReceivedInfoError(error) => {
                // todo delete println -> logs
                println!("GUI: SmartSocket command 'info' - error: {}", error);
                self.messages.push(error);
            }
        }

        Command::none()
    }

    fn view(&self) -> Element<Message> {
        let message_log: Element<_> = if self.messages.is_empty() {
            container(text("Smart Socket info logs...").style(Color::from_rgb8(0x88, 0x88, 0x88)))
                .width(Length::Fill)
                .height(Length::Fill)
                .center_x()
                .center_y()
                .into()
        } else {
            scrollable(
                column(self.messages.iter().cloned().map(text).map(Element::from)).spacing(10),
            )
            .id(MESSAGE_LOG.clone())
            .height(Length::Fill)
            .into()
        };

        let new_message_input = {
            let input = text_input("address:port", self.address.as_str())
                .on_input(Message::AddressChanged)
                .on_submit(Message::CommandSendInfo)
                .padding(10);

            let button = button(
                text("Connect")
                    .height(40)
                    .vertical_alignment(alignment::Vertical::Center),
            )
            .padding([0, 20])
            .style(iced::theme::Button::Primary)
            .on_press(Message::CommandSendInfo);

            // if matches!(self.state, State::Connected(_)) {
            //     if let Some(message) = echo::Message::new(&self.new_message) {
            //         input = input.on_submit(Message::Send(message.clone()));
            //         button = button.on_press(Message::Send(message));
            //     }
            // }

            row![input, button]
                .spacing(10)
                .align_items(Alignment::Center)
        };

        column![new_message_input, message_log]
            .height(Length::Fill)
            .padding(20)
            .spacing(10)
            .into()
    }
}
