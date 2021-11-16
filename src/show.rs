use piechart::*;
use super::portfolio::Portfolio;

pub fn show_portfolio(portfolio: &Portfolio) {
    let data = portfolio.data()
        .map(|c| Data { label: c.0.to_string(), value: c.1.0, color: None, fill: '*' })
        .collect::<Vec<Data>>();

    Chart::new()
        .radius(9)
        .aspect_ratio(3)
        .legend(true)
        .draw(&data);
}
