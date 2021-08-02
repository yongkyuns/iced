use std::{cell::RefCell, rc::Rc};

use iced::{
    button, Align, Button, Column, Element, Row, Sandbox, Settings, Text,
};

pub fn main() -> iced::Result {
    ManyCounters::run(Settings::default())
}

#[derive(Debug)]
pub struct ValueHolder {
    val: i32,
}

impl ValueHolder {
    fn increment(&mut self) {
        self.val += 1;
    }
    fn decrement(&mut self) {
        self.val -= 1;
    }
}

impl Default for ValueHolder {
    fn default() -> Self {
        ValueHolder { val: 1 }
    }
}

// #[derive(Default)]
struct ManyCounters {
    counters: Vec<Counter>,
    data: Rc<RefCell<ValueHolder>>,
}

impl Sandbox for ManyCounters {
    type Message = Messages;

    fn new() -> Self {
        let mut v = Vec::new();
        let data = Rc::new(RefCell::new(ValueHolder::default()));
        (0..3).for_each(|i| v.push(Counter::new(Rc::clone(&data))));
        Self { counters: v, data }
    }

    fn title(&self) -> String {
        String::from("Counter - Iced")
    }

    fn update(&mut self, message: Messages) {
        match message {
            Messages::Counter(id, msg) => {
                if let Some(counter) = self.counters.get_mut(id) {
                    counter.update(msg);
                }
            }
        }
    }

    fn view(&mut self) -> Element<Messages> {
        let row = self.counters.iter_mut().enumerate().fold(
            Row::new().spacing(20),
            |row, (id, counter)| {
                let element: Element<Message> = counter.view().into();
                row.push(
                    element.map(move |message| Messages::Counter(id, message)),
                )
            },
        );
        row.into()
    }
}

#[derive(Debug, Clone, Copy)]
pub enum Messages {
    Counter(usize, Message),
}

#[derive(Default)]
struct Counter {
    value: i32,
    increment_button: button::State,
    decrement_button: button::State,
    data: Rc<RefCell<ValueHolder>>,
}

#[derive(Debug, Clone, Copy)]
pub enum Message {
    IncrementPressed,
    DecrementPressed,
}

impl Counter {
    fn new(data: Rc<RefCell<ValueHolder>>) -> Self {
        Self {
            data: Rc::clone(&data),
            ..Default::default()
        }
    }
}

impl Sandbox for Counter {
    type Message = Message;

    fn new() -> Self {
        Self::default()
    }

    fn title(&self) -> String {
        String::from("Counter - Iced")
    }

    fn update(&mut self, message: Message) {
        match message {
            Message::IncrementPressed => {
                let mut data = self.data.borrow_mut();
                self.value += data.val;
                data.increment();
            }
            Message::DecrementPressed => {
                let mut data = self.data.borrow_mut();
                self.value -= data.val;
                data.decrement();
            }
        }
    }

    fn view(&mut self) -> Element<Message> {
        Column::new()
            .padding(20)
            .align_items(Align::Center)
            .push(
                Button::new(&mut self.increment_button, Text::new("Increment"))
                    .on_press(Message::IncrementPressed),
            )
            .push(Text::new(self.value.to_string()).size(50))
            .push(
                Button::new(&mut self.decrement_button, Text::new("Decrement"))
                    .on_press(Message::DecrementPressed),
            )
            .into()
    }
}
