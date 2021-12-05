mod selection;
mod input;

use std::io::{Write, Stdout};
use std::fmt::Display;
use chrono::Local;

const MAX_VISIBLE_OPTIONS : usize  = 5; 

use crossterm::{
    cursor::SavePosition,
    style::{SetAttribute, Attribute, SetForegroundColor, ResetColor, Color, Print},
    queue
};

use crate::currency::Currency;

pub use input::Input;
pub use selection::select_one;

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
    let category_input = Input::new("Category name", |s| Ok(String::from(s)))
        .esc_interrupts(true);
    let amount_input = Input::new("Amount", super::add_interactively::validate_amount)
        .esc_interrupts(true);
    loop {
        let category = category_input.ask_for_input();
        if category.is_err() { break; }

        let amount = amount_input.ask_for_input();
        if amount.is_err() { break; }

        portfolio.add_category(category.unwrap());
        data.push(Currency(amount.unwrap()));
    }

    portfolio.set_data_for_date(date, data);
}

fn draw_promt<T: Display>(stdout: &mut Stdout, label: &str, default_value: &Option<T>) {

    queue!(stdout, SetAttribute(Attribute::Bold), SetForegroundColor(Color::Green), Print("? ".to_string()), ResetColor).unwrap();
    queue!(stdout, SetAttribute(Attribute::Bold), Print(format!("{}: ", label)), SetAttribute(Attribute::Reset)).unwrap();
    queue!(stdout, SavePosition).unwrap_or_default();
    if let Some(default) = default_value {
        queue!(stdout, SetForegroundColor(Color::DarkGrey), Print(format!("[{}] ", default)), ResetColor).unwrap_or_default();
    }

    stdout.flush().unwrap();
}

