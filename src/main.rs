#![feature(generic_const_exprs)]
#![feature(inline_const_pat)]
#![feature(adt_const_params)]

use std::time::Duration;

pub use field::Field;
use iced::{
    canvas::{self, Cache, Canvas, Cursor, Fill, Frame, Geometry, Path, Stroke},
    executor, Application, Color, Column, Command, Container, Element, Length, Point, Rectangle,
    Sandbox, Settings, Size, Subscription,
};

mod field;

pub fn main() -> iced::Result {
    std::thread::sleep(Duration::from_secs(5));
    Counter::run(Settings::default())
}

struct Counter {
    fields: Box<(Field<64, 64>, Field<64, 64>)>,
}

#[derive(Debug, Clone, Copy)]
enum Message {
    Test,
}

impl Application for Counter {
    type Message = Message;
    type Executor = executor::Default;
    type Flags = ();

    fn new(_flags: ()) -> (Self, Command<Message>) {
        (
            Self {
                fields: Default::default(),
            },
            Command::none(),
        )
    }

    fn title(&self) -> String {
        String::from("GOL - Iced")
    }

    fn update(&mut self, message: Message) -> Command<Message> {
        self.fields.0.turn(&mut self.fields.1);
        core::mem::swap(&mut self.fields.0, &mut self.fields.1);
        Command::none()
    }

    fn subscription(&self) -> Subscription<Message> {
        iced::time::every(std::time::Duration::from_millis(32)).map(|_| Message::Test)
    }

    fn view(&mut self) -> Element<Message> {
        let canvas = Canvas::new(self)
            .width(Length::Units(1024))
            .height(Length::Units(1024));

        Container::new(canvas)
            .width(Length::Fill)
            .height(Length::Fill)
            .padding(20)
            .into()
    }
}

impl<Message> canvas::Program<Message> for Counter {
    fn draw(&self, bounds: Rectangle, _cursor: Cursor) -> Vec<Geometry> {
        let mut frame = Frame::new(bounds.size());

        for (i, v) in self.fields.0.data.iter().enumerate() {
            frame.fill_rectangle(
                Point {
                    x: ((i / 64) * 4) as f32,
                    y: ((i % 64) * 4) as f32,
                },
                Size {
                    width: 4.0,
                    height: 4.0,
                },
                if *v { Color::BLACK } else { Color::WHITE },
            )
        }

        vec![frame.into_geometry()]
    }
}
