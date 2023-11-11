use std::time::Duration;

use crate::terminal::running_color;
use crate::{format::format_duration, input::Command, terminal::TerminalHandler};
use crate::{prelude::*, CounterUIState};
use porsmo::counter::Counter as Stopwatch;

#[derive(Debug)]
pub struct StopwatchState {
    pub counter: Stopwatch,
}

impl StopwatchState {
    pub fn new(start_time: Duration) -> Self {
        let counter = Stopwatch::from(start_time).start();
        Self { counter }
    }
}

impl CounterUIState for StopwatchState {
    fn show(&self, terminal: &mut TerminalHandler) -> Result<()> {
        terminal
            .clear()?
            .info("Stopwatch")?
            .set_foreground_color(running_color(self.counter.started()))?
            .print(format_duration(&self.counter.elapsed()))?
            .info("[Q]: quit, [Space]: pause/resume")?
            .flush()
    }

    fn handle_command(self, cmd: Command) -> Option<Self> {
        let Self { counter } = self;
        match cmd {
            Command::Quit => None,
            Command::Pause => Some(counter.stop()),
            Command::Resume => Some(counter.start()),
            Command::Toggle | Command::Enter => Some(counter.toggle()),
            _ => Some(counter),
        }
        .map(|counter| Self { counter })
    }
}
