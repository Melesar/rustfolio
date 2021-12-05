use std::io::{Write, Stdout};
use std::fmt::Display;
use chrono::Local;

const MAX_VISIBLE_OPTIONS : usize  = 5; 

use crossterm::{
    cursor::{self, SavePosition, RestorePosition, MoveToNextLine, MoveTo, MoveToPreviousLine, MoveRight},
    terminal::{enable_raw_mode, disable_raw_mode, Clear, ClearType},
    style::{SetAttribute, Attribute, SetForegroundColor, ResetColor, Color, Print},
    event::{read, Event, KeyCode},
    queue, execute
};

use crate::currency::Currency;

pub fn select_one<I, T, F>(label: &str, iter: I, tranform: F) -> T
    where I : Iterator<Item=T>,
          F : FnMut(&T) -> String,
{
    let mut stdout = std::io::stdout();
    
    let mut all_options : Vec<T> = iter.collect();
    let mut transformed_options : Vec<String> = all_options.iter().map(tranform).collect();
    let mut current_filter = String::new();
    let mut current_options : Vec<(usize, &String)> = transformed_options.iter().enumerate().collect();
    let mut current_selection = None;

    draw_promt(&mut stdout, label, &None::<f32>);

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

    all_options.remove(selected_option)
}

pub fn ask_for_input<F, T>(label: &str, validation: F, default_value: Option<T>) -> T 
    where F : Fn(&String) -> Result<T, String>,
          T : Display
{
    ask_for_input_impl(label, validation, default_value, false).unwrap()
}

pub fn confirmation(label: &str, default_positive: bool) -> bool {
    let mut stdout = std::io::stdout();
    
    draw_promt(&mut stdout, &format!("{} [{yes}/{no}]", label, 
                                     yes = if default_positive { 'Y' } else { 'y' },
                                     no = if !default_positive { 'N' } else { 'n' } ), &None::<f32>);

    let stdin = std::io::stdin();
    let mut input = String::new();
    stdin.read_line(&mut input).unwrap_or_default();

    let input = input.trim();
    if default_positive && input.eq("n") {
        false
    } else if !default_positive && !input.eq("y") {
        false
    } else {
        true
    }
}

pub fn populate_new_portfolio(portfolio: &mut super::portfolio::Portfolio) {
    let mut stdout = std::io::stdout();

    queue!(stdout, SetAttribute(Attribute::Bold), Print("Create categories for your new portfolio"), SetAttribute(Attribute::Reset)).unwrap();
    queue!(stdout, SetForegroundColor(Color::Cyan), Print(" [press Esc to finish]\n"), ResetColor).unwrap_or_default();
    stdout.flush().unwrap();

    let date = Local::now();
    let mut data = vec![];
    loop {
        let category = ask_for_input_impl("Category name", |s| Ok(String::from(s)), None, true);
        if category.is_err() { break; }

        let amount = ask_for_input_impl("Amount", super::add_interactively::validate_amount, None, true);
        if amount.is_err() { break; }

        portfolio.add_category(category.unwrap());
        data.push(Currency(amount.unwrap()));
    }

    portfolio.set_data_for_date(date, data);
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

fn draw_promt<T: Display>(stdout: &mut Stdout, label: &str, default_value: &Option<T>) {

    queue!(stdout, SetAttribute(Attribute::Bold), SetForegroundColor(Color::Green), Print("? ".to_string()), ResetColor).unwrap();
    queue!(stdout, SetAttribute(Attribute::Bold), Print(format!("{}: ", label)), SetAttribute(Attribute::Reset)).unwrap();
    if let Some(default) = default_value {
        queue!(stdout, SetForegroundColor(Color::DarkGrey), Print(format!("[{}] ", default)), ResetColor).unwrap_or_default();
    }

    stdout.flush().unwrap();
}

fn ask_for_input_impl<F, T>(label: &str, validation: F, default_value: Option<T>, esc_interrupts: bool) -> Result<T, String>
    where F : Fn(&String) -> Result<T, String>,
          T : Display
{
    let mut stdout = std::io::stdout();

    draw_promt(&mut stdout, label, &default_value);
    execute!(stdout, SavePosition).unwrap_or_default();

    enable_raw_mode().unwrap_or_default();

    let mut input = String::new();
    let mut result = default_value.ok_or(String::new()).or(validation(&input));
    loop {
        match read().unwrap() {
            Event::Key(k) => match k.code {
                KeyCode::Char(c) => {
                    input.push(c);
                    result = validation(&input);
                    execute!(stdout, RestorePosition, Clear(ClearType::UntilNewLine), Print(&input)).unwrap_or_default();
                },
                KeyCode::Backspace => {
                    input.pop();
                    result = validation(&input);
                    execute!(stdout, RestorePosition, Clear(ClearType::UntilNewLine), Print(&input)).unwrap_or_default();
                },
                KeyCode::Enter => {
                    match result.as_ref() {
                        Ok(_) => break,
                        Err(e) => { input.clear(); display_error(&mut stdout, e) }
                    }
                },
                KeyCode::Esc => if esc_interrupts { result = Err(String::new()); break; },
                _ => (),
            },
            _ => (),
        }
    }

    disable_raw_mode().unwrap_or_default();

    if let Ok(r) = result.as_ref() {
        execute!(stdout, RestorePosition, SetForegroundColor(Color::DarkCyan), Print(r), Print('\n'), ResetColor).unwrap_or_default();
    }

    result
}

fn display_error(stdout: &mut Stdout, error_msg: &str) {
    queue!(stdout, RestorePosition, Clear(ClearType::UntilNewLine)).unwrap_or_default();
    queue!(stdout, Print(" "), SetForegroundColor(Color::Red), Print(format!("[{}]", error_msg)), ResetColor).unwrap_or_default();
    queue!(stdout, RestorePosition).unwrap_or_default();

    stdout.flush().unwrap_or_default();
}
