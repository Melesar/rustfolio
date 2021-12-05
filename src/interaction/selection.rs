use std::{
    fmt::Display,
    io::{Write, Stdout}
};

use super::draw_promt;

use crossterm::{
    cursor::{self, SavePosition, RestorePosition, MoveToNextLine, MoveTo, MoveToPreviousLine, MoveRight},
    terminal::{enable_raw_mode, disable_raw_mode, Clear, ClearType},
    style::{SetAttribute, Attribute, SetForegroundColor, ResetColor, Color, Print},
    event::{read, Event, KeyCode},
    queue, execute
};

const MAX_VISIBLE_OPTIONS : usize  = 5; 

pub fn select_one<I, T, F>(label: &str, iter: I, tranform: F) -> T
    where I : Iterator<Item=T>,
          F : FnMut(&T) -> String,
{
    let mut stdout = std::io::stdout();
    
    let mut all_options : Vec<T> = iter.collect();
    let mut transformed_options : Vec<String> = all_options.iter().map(tranform).collect();
    let mut current_filter = String::new();
    let mut current_options : Vec<(usize, &String)> = transformed_options.iter().enumerate().collect();
    let mut current_selection = Some(0_usize);

    draw_promt(&mut stdout, label, &None::<f32>);

    let initial_cursor_position = cursor::position().unwrap_or_default();
    let options_to_draw = std::cmp::min(MAX_VISIBLE_OPTIONS, transformed_options.len());
    for (idx, option) in transformed_options.iter().take(options_to_draw).enumerate() {
        let prefix = current_selection.map_or(" ", |s| if s == idx { ">" } else { " " });
        print!("\n{}{}", prefix, option);
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
                    current_selection = update_selection(&current_options, current_selection);
                    draw_options(&mut stdout, &current_selection, &current_options);
                },
                KeyCode::Backspace => {
                    current_filter.pop();
                    execute!(stdout, move_to_start, Clear(ClearType::UntilNewLine), Print(&current_filter), SavePosition).unwrap();
                    current_options = apply_filter(&current_filter, &transformed_options);
                    current_selection = update_selection(&current_options, current_selection);
                    draw_options(&mut stdout, &current_selection, &current_options);
                },
                KeyCode::Down => {
                    current_selection = current_selection.and_then(|v| Some(std::cmp::min(v + 1, current_options.len() - 1)));
                    draw_options(&mut stdout, &current_selection, &current_options);
                },
                KeyCode::Up => {
                    current_selection = current_selection.and_then(|v| v.checked_sub(1).or(Some(0)));
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

    all_options.remove(selected_option)
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

fn apply_filter<'a>(filter: &str, all_options: &'a[String]) -> Vec<(usize, &'a String)> {
    all_options.iter()
        .enumerate()
        .filter(|(_, s)| s.contains(filter))
        .collect()
}

fn update_selection<U: Display>(current_options: &[(usize, &U)], current_selection: Option<usize>) -> Option<usize> {
    if current_options.len() == 0 {
        None
    } else {
        Some(current_selection.map_or(0, |selection| std::cmp::min(selection, current_options.len() - 1)))
    }
}
