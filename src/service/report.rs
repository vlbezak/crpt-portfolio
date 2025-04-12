use colored::*;

pub fn readable_mkt_cap(mkt_cap: f64) -> String {
    if mkt_cap < 1_000.0 {
        format!("{:.3} K", mkt_cap / 1_000.0)
    } else if mkt_cap < 1_000_000.0 {
        format!("{:.3} M", mkt_cap / 1_000_000.0)
    } else if mkt_cap < 1_000_000_000.0 {
        format!("{:.3} B", mkt_cap / 1_000_000_000.0)
    } else {
        format!("{:.3} T", mkt_cap / 1_000_000_000_000.0)
    }
}

pub fn colored_change_percent(value: f64) -> ColoredString {
    if value > 0.0 {
        format!("{:>7.2}%", value).green()
    } else {
        format!("{:>7.2}%", value).red()
    }
}