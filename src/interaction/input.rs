use std::{
    fmt::Display,
    io::{Stdout, Write}
};

use super::draw_promt;

use crossterm::{
    cursor::RestorePosition,
    terminal::{enable_raw_mode, disable_raw_mode, Clear, ClearType},
    style::{SetForegroundColor, ResetColor, Color, Print},
    event::{read, Event, KeyCode},
    queue, execute
};

pub struct Input<F, T>
    where F: Fn(&String) -> Result<T, String>,
          T: Display + Clone
{
    label: String,
    validation: F,
    default_value: Option<T>,
    esc_interrupts: bool
}

impl<F, T> Input<F, T> 
    where F: Fn(&String) -> Result<T, String>,
          T: Display + Clone
{
    pub fn new<S: Into<String>>(label: S, validation: F) -> Self {
        Input {
            label: label.into(),
            validation,
            default_value: None::<T>,
            esc_interrupts: false
        }
    }

    pub fn default_value(mut self, default_value: Option<T>) -> Self {
        self.default_value = default_value;
        self
    }

    pub fn esc_interrupts(mut self, esc_interrupts: bool) -> Self {
        self.esc_interrupts = esc_interrupts;
        self
    }

    pub fn ask_for_input(&self) -> Result<T, String> {
        let mut stdout = std::io::stdout();

        draw_promt(&mut stdout, &self.label, &self.default_value);

        enable_raw_mode().unwrap_or_default();

        let mut input = String::new();
        let mut result = self.default_value.clone().ok_or(String::new()).or((self.validation)(&input));
        loop {
            match read().unwrap() {
                Event::Key(k) => match k.code {
                    KeyCode::Char(c) => {
                        input.push(c);
                        result = (self.validation)(&input);
                        execute!(stdout, RestorePosition, Clear(ClearType::UntilNewLine), Print(&input)).unwrap_or_default();
                    },
                    KeyCode::Backspace => {
                        input.pop();
                        result = (self.validation)(&input);
                        execute!(stdout, RestorePosition, Clear(ClearType::UntilNewLine), Print(&input)).unwrap_or_default();
                    },
                    KeyCode::Enter => {
                        match result.as_ref() {
                            Ok(_) => break,
                            Err(e) => { input.clear(); display_error(&mut stdout, e) }
                        }
                    },
                    KeyCode::Esc => if self.esc_interrupts { result = Err(String::new()); break; },
                    _ => (),
                },
                _ => (),
            }
        }

        disable_raw_mode().unwrap_or_default();

        if let Ok(r) = result.as_ref() {
            execute!(stdout, RestorePosition, Clear(ClearType::UntilNewLine), SetForegroundColor(Color::DarkCyan), Print(r), Print('\n'), ResetColor).unwrap_or_default();
        }

        result
    }
}

fn display_error(stdout: &mut Stdout, error_msg: &str) {
    queue!(stdout, RestorePosition, Clear(ClearType::UntilNewLine)).unwrap_or_default();
    queue!(stdout, Print(" "), SetForegroundColor(Color::Red), Print(format!("[{}]", error_msg)), ResetColor).unwrap_or_default();
    queue!(stdout, RestorePosition).unwrap_or_default();

    stdout.flush().unwrap_or_default();
}
