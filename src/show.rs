use piechart::*;
use super::portfolio::Portfolio;

const COLORS : [u8; 8] = [ 213, 226, 160, 134, 123, 172, 231, 207 ];
const SYMBOLS : [char; 8] = ['▪', '•', '▴', '*', '♠', '⚬', '‣', '♥'];

pub fn show_portfolio(portfolio: &Portfolio) {
    let data = portfolio.data()
        .zip(COLORS.iter())
        .zip(SYMBOLS.iter())
        .map(|(((category, amount), color), symbol)| Data { label: category.to_string(), value: **amount, color: Some(Style::new().fg(Color::Fixed(*color))), fill: *symbol })
        .collect::<Vec<Data>>();

    Chart::new()
        .radius(9)
        .aspect_ratio(4)
        .legend(true)
        .total(true)
        .draw(&data);
}
