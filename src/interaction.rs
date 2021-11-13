use std::io::Write;

const MAX_VISIBLE_OPTIONS : usize  = 4; 

use crossterm::{
    cursor::{SavePosition, RestorePosition},
    terminal::{enable_raw_mode, disable_raw_mode},
    style::{SetAttribute, Attribute, SetForegroundColor, ResetColor, Color, Print},
    event::{read, Event, KeyCode},
    queue, execute
};

pub fn select_one<I, T, S>(label: S, iter: T) -> Option<I> 
    where T : Iterator<Item=I>,
          S : Into<String>
{
    enable_raw_mode().unwrap();

    let mut stdout = std::io::stdout();

    queue!(stdout, SetAttribute(Attribute::Bold), SetForegroundColor(Color::Green), Print("? ".to_string()), ResetColor).unwrap();
    queue!(stdout, SetAttribute(Attribute::Bold), Print(format!("{}: ", label.into())), SetAttribute(Attribute::Reset)).unwrap();
    queue!(stdout, SavePosition).unwrap();

    stdout.flush().unwrap();
    
    let mut all_options : Vec<I> = iter.collect();
    let mut current_filter = String::new();
    let mut current_options = all_options.iter().collect();
    let mut current_selection = None;

    while match read().unwrap() {
        Event::Key(k) => match k.code {
            KeyCode::Char(c) => {
                current_filter.push(c);
                execute!(stdout, Print(c)).unwrap();
                current_options = apply_filter(&current_filter, &all_options);
                true
            },
            KeyCode::Backspace => {
                current_filter.pop();
                execute!(stdout, RestorePosition, Print(&current_filter)).unwrap();
                current_options = apply_filter(&current_filter, &all_options);
                true
            },
            KeyCode::Down => {
                current_selection = current_selection.map_or(Some(0_usize), |v| Some(std::cmp::min(v + 1, current_options.len() - 1)));
                true
            },
            KeyCode::Up => {
                current_selection = current_selection.and_then(|v| if v == 0 { None } else { Some(std::cmp::max(v - 1, 0)) });
                true
            },
            KeyCode::Enter => {
                true
            }
            _ => true,
        },
        _ => true,
    }

    { disable_raw_mode().unwrap(); }

    Some(all_options.remove(0))
}

pub fn ask_for_input(label: &str) -> String {
    String::new()
}

pub fn confirmation(label: &str) -> bool {
    false
}

fn apply_filter<'a, I>(filter: &str, all_options: &'a Vec<I>) -> Vec<&'a I> {
    all_options.iter().collect()
}
