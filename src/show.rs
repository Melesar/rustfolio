use std::io::Write;

use cli_table::{Table, Cell, print_stdout};
use crossterm::{style::{SetAttribute, Attribute, Print}, queue};
use piechart::*;
use super::portfolio::Portfolio;

const COLORS : [u8; 8] = [ 213, 226, 160, 134, 123, 172, 231, 207 ];
const SYMBOLS : [char; 8] = ['▪', '•', '▴', '*', '♠', '⚬', '‣', '♥'];

pub fn show_as_chart(portfolio: &Portfolio) -> Result<(), String>{
    if let Some(data_iter) = portfolio.data() {
        let data = data_iter
            .zip(COLORS.iter())
            .zip(SYMBOLS.iter())
            .map(|(((category, amount), color), symbol)| Data { label: category.to_string(), value: **amount, color: Some(Style::new().fg(Color::Fixed(*color))), fill: *symbol })
            .collect::<Vec<Data>>();

        let radius = 9_u16;
        let aspect = 4_u16;
        let result = Chart::new()
            .radius(radius)
            .aspect_ratio(aspect)
            .legend(true)
            .draw_into(std::io::stdout(), &data)
            .map_err(|e| format!("Failed to draw piechart: {}", e));

        if result.is_ok() {
            let mut stdout = std::io::stdout();
            let total_value = data.iter().map(|d| d.value).sum::<f32>();
            for _ in 0..(radius * aspect + 4) {
                queue!(stdout, Print(" ")).unwrap_or_default();
            }
            queue!(stdout, SetAttribute(Attribute::Bold), Print("Total: "), SetAttribute(Attribute::Reset)).unwrap_or_default();
            queue!(stdout, Print(total_value), Print("\n")).unwrap_or_default();

            stdout.flush().unwrap_or_default();
        }

        result
    } else {
        Err(String::from("No data was found in the current portfolio"))
    }
}

pub fn show_as_table(portfolio: &Portfolio) -> Result<(), String> {
    let table = portfolio.values().map(|(date, values)| {
        let mut cells = vec![date.date().format("%Y-%m-%d").cell()];
        cells.extend(values.into_iter().map(|v| (*v).cell()));
        cells.push(values.into_iter().map(|c| c.0).sum::<f32>().cell());
        cells
    })
    .table()
        .title(vec!["Date"].into_iter().chain(portfolio.categories()).chain(vec!["Total"]));

    print_stdout(table).map_err(|_| String::from("Failed to draw table"))
}
