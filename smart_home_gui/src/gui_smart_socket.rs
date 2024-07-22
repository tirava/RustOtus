use crate::prelude::SmartHouseError;
use iced::border::Radius;
use iced::theme::{Button, Container, Scrollable};
use iced::widget::scrollable::{Scrollbar, Scroller};
use iced::widget::{button, column, container, row, scrollable, text, text_input};
use iced::{alignment, Alignment, Application, Border, Color, Command, Element, Length, Theme};
use once_cell::sync::Lazy;
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::net::TcpStream;

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
    CommandReceivedInfo(String, String),
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
                return Command::perform(send_command(self.address.clone(), "info"), |result| {
                    match result {
                        Ok(result) => Message::CommandReceivedInfo(result, "".to_string()),
                        Err(e) => Message::CommandReceivedInfo("".to_string(), e.to_string()),
                    }
                });
            }
            Message::CommandReceivedInfo(result, error) => {
                let date_time = chrono::Local::now().format("%Y-%m-%d %H:%M:%S").to_string();
                let result = match error.is_empty() {
                    true => {
                        self.state = State::Connected;
                        format!("{date_time}: SmartSocket command 'info' result: '{result}'")
                    }
                    false => {
                        self.state = State::Disconnected;
                        format!("{date_time}: SmartSocket command 'info' result: '{error}'")
                    }
                };
                self.messages.push(result);
                return scrollable::snap_to(MESSAGE_LOG.clone(), scrollable::RelativeOffset::END);
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
                .style(Container::Custom(Box::new(StyleContainer)))
                .into()
        } else {
            scrollable(
                column(
                    self.messages
                        .iter()
                        .cloned()
                        .map(|s| text(s).size(12))
                        .map(Element::from),
                )
                .padding(10)
                .spacing(10),
            )
            .id(MESSAGE_LOG.clone())
            .height(Length::Fill)
            .style(Scrollable::Custom(Box::new(StyleScrollable)))
            .into()
        };

        let connect_info = {
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
            .style(Button::Primary)
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

        column![connect_info, message_log]
            .height(Length::Fill)
            .padding(20)
            .spacing(10)
            .into()
    }
}

async fn send_command(addr: String, command: &str) -> Result<String, SmartHouseError> {
    println!(
        "SMART_DEVICE: connecting to address '{}' with command '{}'...",
        addr, command
    );

    match TcpStream::connect(addr).await {
        Ok(mut stream) => {
            let command = format!("{}\n", command);
            stream.write_all(command.as_bytes()).await?;

            let mut data = String::new();
            match stream.read_to_string(&mut data).await {
                Ok(_) => Ok(data),
                Err(err) => Err(SmartHouseError::from(err)),
            }
        }
        Err(err) => Err(SmartHouseError::from(err)),
    }
}

struct StyleContainer;

impl container::StyleSheet for StyleContainer {
    type Style = Theme;

    fn appearance(&self, _style: &Self::Style) -> container::Appearance {
        container::Appearance {
            text_color: None,
            background: None,
            border: Border {
                color: Color::from_rgb8(0xd8, 0xd8, 0xd8),
                width: 1.0,
                radius: Radius::from(0),
            },
            shadow: Default::default(),
        }
    }
}

struct StyleScrollable;

impl scrollable::StyleSheet for StyleScrollable {
    type Style = Theme;

    fn active(&self, _style: &Self::Style) -> scrollable::Appearance {
        scrollable::Appearance {
            container: container::Appearance {
                text_color: None,
                background: None,
                border: Border {
                    color: Color::from_rgb8(0xd8, 0xd8, 0xd8),
                    width: 1.0,
                    radius: Radius::from(0),
                },
                shadow: Default::default(),
            },
            scrollbar: Scrollbar {
                background: Default::default(),
                border: Default::default(),
                scroller: Scroller {
                    color: Default::default(),
                    border: Default::default(),
                },
            },
            gap: None,
        }
    }

    fn hovered(
        &self,
        _style: &Self::Style,
        _is_mouse_over_scrollbar: bool,
    ) -> scrollable::Appearance {
        self.active(_style)
    }
}
