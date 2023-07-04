use crate::terminal::running_color;
use crate::{
    alert::alert,
    format::fmt_time,
    input::Command,
    terminal::TerminalHandler,
};
use crate::prelude::*;
use porsmo::counter::Counter;
use porsmo::counter::DoubleEndedDuration;
use std::time::Duration;

pub struct TimerUI {
    counter: Counter,
    initial: Duration,
    quit: bool,
}

impl TimerUI {
    pub fn new(initial: Duration) -> Self {
        Self {
            counter: Counter::default().start(),
            initial,
            quit: false,
        }
    }

    pub fn ended(&self) -> bool {
        self.quit
    }

    pub fn excess_time_left(&self) -> DoubleEndedDuration {
        self.counter.checked_time_left(self.initial)
    }

    pub fn quit(self) -> Self {
        Self { counter: self.counter.stop(), quit: true, ..self }
    }

    pub fn handle_command(mut self, command: Command) -> Self {
        self.counter = match command {
            Command::Quit => return self.quit(),
            Command::Pause => self.counter.stop(),
            Command::Resume => self.counter.start(),
            Command::Toggle | Command::Enter => self.counter.toggle(),
            _ => self.counter,
        };
        self
    }

    pub fn show(&self, terminal: &mut TerminalHandler) -> Result<()> {
        match self.excess_time_left() {
            DoubleEndedDuration::Positive(elapsed) => {
                terminal
                    .clear()?
                    .info("Timer")?
                    .set_foreground_color(running_color(self.counter.started()))?
                    .print(fmt_time(elapsed))?
                    .info("[Q]: quit, [Space]: pause/resume")?
                    .flush()?;
            }
            DoubleEndedDuration::Negative(elapsed) => {
                terminal
                    .clear()?
                    .info("Timer Has Ended")?
                    .set_foreground_color(running_color(self.counter.started()))?
                    .print(format_args!("+{}", fmt_time(elapsed)))?
                    .info("[Q]: quit, [Space]: pause/resume")?
                    .flush()?;
            }
        }
        Ok(())
    }
}

