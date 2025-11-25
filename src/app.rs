use crate::controller::{AppEvents, Controller};
use ratatui::DefaultTerminal;
use ratatui::Frame;
use ratatui::prelude::Backend;
use std::io;

pub struct App {
    controller: Controller,
    exit: bool,
}

impl App {
    pub fn new() -> App {
        App {
            exit: false,
            controller: Controller::new(),
        }
    }

    pub fn run(&mut self, terminal: &mut DefaultTerminal) -> io::Result<()> {
        while !self.exit {
            terminal.draw(|frame: &mut Frame<'_>| self.controller.draw(frame))?;
            match self.controller.handle_events() {
                Err(e) => return Err(e),
                Ok(event) => match event {
                    AppEvents::None => {}
                    AppEvents::Exit => self.exit = true,
                    _ => panic!(),
                },
            }
        }
        Ok(())
    }
}
