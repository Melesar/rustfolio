use std::io::{Write, Stdout};
use std::fmt::Display;

const MAX_VISIBLE_OPTIONS : usize  = 4; 

use crossterm::{
    cursor::{self, SavePosition, RestorePosition, MoveToNextLine, MoveTo, MoveToPreviousLine, MoveRight},
    terminal::{enable_raw_mode, disable_raw_mode, Clear, ClearType},
    style::{SetAttribute, Attribute, SetForegroundColor, ResetColor, Color, Print},
    event::{read, Event, KeyCode},
    queue, execute
};

pub fn select_one<I, T, F>(label: &str, iter: I, tranform: F) -> Option<T> 
    where I : Iterator<Item=T>,
          F : FnMut(&T) -> String,
{
    let mut stdout = std::io::stdout();
    
    let mut all_options : Vec<T> = iter.collect();
    let mut transformed_options : Vec<String> = all_options.iter().map(tranform).collect();
    let mut current_filter = String::new();
    let mut current_options : Vec<(usize, &String)> = transformed_options.iter().enumerate().collect();
    let mut current_selection = None;

    queue!(stdout, SetAttribute(Attribute::Bold), SetForegroundColor(Color::Green), Print("? ".to_string()), ResetColor).unwrap();
    queue!(stdout, SetAttribute(Attribute::Bold), Print(format!("{}: ", label)), SetAttribute(Attribute::Reset)).unwrap();

    stdout.flush().unwrap();

    let initial_cursor_position = cursor::position().unwrap_or_default();
    let options_to_draw = std::cmp::min(MAX_VISIBLE_OPTIONS, transformed_options.len());
    for option in transformed_options.iter().take(options_to_draw) {
        print!("\n  {}", option);
    }

    execute!(stdout, MoveToPreviousLine(options_to_draw as u16), MoveRight(initial_cursor_position.0), SavePosition).unwrap_or_default();

    let initial_cursor_position = cursor::position().unwrap_or_default();
    let move_to_start = MoveTo(initial_cursor_position.0, initial_cursor_position.1);

    enable_raw_mode().unwrap_or_default();

    let selected_option : usize;
    loop {
        match read().unwrap() {
            Event::Key(k) => match k.code {
                KeyCode::Char(c) => {
                    current_filter.push(c);
                    execute!(stdout, Print(c), SavePosition).unwrap();
                    current_options = apply_filter(&current_filter, &transformed_options);
                    draw_options(&mut stdout, &current_selection, &current_options);
                },
                KeyCode::Backspace => {
                    current_filter.pop();
                    execute!(stdout, move_to_start, Clear(ClearType::UntilNewLine), Print(&current_filter), SavePosition).unwrap();
                    current_options = apply_filter(&current_filter, &transformed_options);
                    draw_options(&mut stdout, &current_selection, &current_options);
                },
                KeyCode::Down => {
                    current_selection = current_selection.map_or(Some(0_usize), |v| Some(std::cmp::min(v + 1, current_options.len() - 1)));
                    draw_options(&mut stdout, &current_selection, &current_options);
                },
                KeyCode::Up => {
                    current_selection = current_selection.and_then(|v| if v == 0 { None } else { Some(std::cmp::max(v - 1, 0)) });
                    draw_options(&mut stdout, &current_selection, &current_options);
                },
                KeyCode::Enter => {
                    if let Some(s) = current_selection {
                        selected_option = current_options[s].0;
                        break;
                    }
                }
                _ => (),
            },
            _ => (),
        }
    }

    queue!(stdout, RestorePosition).unwrap_or_default();
    for _ in 0..current_options.len() {
        queue!(stdout, MoveToNextLine(1), Clear(ClearType::CurrentLine)).unwrap_or_default();
    }

    let selection = transformed_options.remove(selected_option);
    queue!(stdout, move_to_start, Clear(ClearType::UntilNewLine)).unwrap_or_default();
    queue!(stdout, SetForegroundColor(Color::DarkCyan), Print(selection), ResetColor).unwrap_or_default();
    queue!(stdout, MoveToNextLine(1)).unwrap_or_default();
    stdout.flush().unwrap_or_default();

    disable_raw_mode().unwrap_or_default();

    Some(all_options.remove(selected_option))
}

pub fn ask_for_input(label: &str) -> String {
    String::new()
}

pub fn confirmation(label: &str) -> bool {
    false
}

fn apply_filter<'a>(filter: &str, all_options: &'a[String]) -> Vec<(usize, &'a String)> {
    all_options.iter().filter(|s| s.contains(filter)).enumerate().collect()
}

//TODO handle large number of options
fn draw_options<U: Display>(stdout: &mut Stdout, selection: &Option<usize>, current_options: &[(usize, &U)]) {
    queue!(stdout, RestorePosition, MoveToNextLine(1)).unwrap_or_default();

    for (selection_index, (_, option)) in current_options.iter().enumerate() {
        let is_selected = selection.filter(|s| *s == selection_index).is_some();
        let prefix = if is_selected { "> " } else { "  " };
        queue!(stdout, Clear(ClearType::CurrentLine)).unwrap_or_default(); 
        if is_selected { 
            queue!(stdout, SetAttribute(Attribute::Bold)).unwrap_or_default();
        }
        queue!(stdout, Print(prefix), Print(option), SetAttribute(Attribute::Reset), MoveToNextLine(1)).unwrap_or_default();
    }

    for _ in current_options.len()..MAX_VISIBLE_OPTIONS {
        queue!(stdout, Clear(ClearType::CurrentLine), MoveToNextLine(1)).unwrap_or_default();
    }

    queue!(stdout, RestorePosition).unwrap_or_default();

    stdout.flush().unwrap_or_default();
}