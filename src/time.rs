use anyhow::{bail, Result};
use chrono::Duration;
use pest::Parser;

/// Parser for an approximate superset of [systemd.time](https://www.freedesktop.org/software/systemd/man/systemd.time.html#:~:text=Internally%2C%20systemd%20generally%20operates%20with,usually%20seconds%20(see%20above)).
/// Exceptions, also contains ns. Year is 365 days not 365.25, and month is 30 days not 30.44.
#[derive(pest_derive::Parser)]
#[grammar = "time/time.pest"]
pub struct SystemDStyleTimeParser;

/// Convert a string to a duration. This wraps the pest parser and does some extra validation.
pub fn parse_time_span(string: &str) -> Result<Duration> {
    let time_span = SystemDStyleTimeParser::parse(Rule::time_span, string)?
        .next()
        .unwrap();

    let end = time_span.as_span().end_pos().pos();
    if end != string.len() {
        let matched = &string[..end];
        let unmatched = &string[end..];
        eprintln!(
            "Invalid time span: \"{string}\"\n\n\
                I was able to parse \"{matched}\" \
                but I'm not sure what to do with the rest.\n\
                Let me try \"{unmatched}\" on it's own and see if I can give you a more helpful error message.\n"
        );
        parse_time_span(unmatched)?;
        bail!("Parse failed.");
    }

    let span_elements = time_span.into_inner();

    let mut duration = Duration::zero();

    for element in span_elements {
        let mut element_pairs = element.into_inner();
        let quantity = element_pairs
            .next()
            .unwrap()
            .as_str()
            .trim()
            .parse::<i64>()?;
        let unit = element_pairs.next().unwrap().into_inner().next().unwrap();

        let element_duration = match unit.as_rule() {
            Rule::nanosecond => Duration::nanoseconds(quantity),
            Rule::microsecond => Duration::microseconds(quantity),
            Rule::millisecond => Duration::milliseconds(quantity),
            Rule::second => Duration::seconds(quantity),
            Rule::minute => Duration::minutes(quantity),
            Rule::hour => Duration::hours(quantity),
            Rule::day => Duration::days(quantity),
            Rule::week => Duration::weeks(quantity),
            // TODO: the following both round down, which I am okay with because input is integer
            // anyways, so users would just use smalller units, but eventually something worth
            // fixing to comply with systemd.time expectations.
            Rule::month => Duration::days(quantity * 30),
            Rule::year => Duration::days(quantity * 365),
            Rule::time_span => todo!(),
            Rule::span_element => todo!(),
            Rule::quantity => todo!(),
            Rule::unit => todo!(),
            Rule::WHITESPACE => todo!(),
        };
        duration = duration + element_duration;
    }
    Ok(duration)
}

/// For unrolling duration into a human readable display
#[allow(
    non_snake_case,
    clippy::module_name_repetitions,
    clippy::missing_docs_in_private_items
)]
pub struct FlatTime {
    y: i64,
    M: i64,
    w: i64,
    d: i64,
    h: i64,
    m: i64,
    s: i64,
    ms: i64,
    us: i64,
    ns: i64,
}

impl IntoIterator for &FlatTime {
    type Item = (i64, &'static str);
    type IntoIter = Box<dyn Iterator<Item = Self::Item>>;

    fn into_iter(
        self,
    ) -> std::boxed::Box<(dyn std::iter::Iterator<Item = (i64, &'static str)> + 'static)> {
        let values = [
            self.y, self.M, self.w, self.d, self.h, self.m, self.s, self.ms, self.us, self.ns,
        ];
        Box::new(values.into_iter().zip(UNITS))
    }
}

impl FlatTime {
    /// Format the time as a string, but only include the first non-zero unit.
    pub fn format_abbreviated(&self) -> String {
        for (value, unit) in self {
            if value == 0 {
                continue;
            }
            return format!("{value}{unit}");
        }
        "0s".to_string()
    }

    /// Format the time as a string, including all units.
    pub fn format(&self) -> String {
        let mut result = String::new();
        let mut first = true;
        for (value, unit) in self {
            if value == 0 {
                continue;
            }
            if !first {
                result.push(' ');
            }
            first = false;
            result.push_str(&format!("{value}{unit}"));
        }
        if result.is_empty() {
            result.push_str("0s");
        }
        result
    }
}

#[allow(non_snake_case, clippy::many_single_char_names)]
impl From<Duration> for FlatTime {
    fn from(mut value: Duration) -> Self {
        let mut days = value.num_days();
        let y = days / 365;
        days %= 365;
        let M = days / 30;
        value = value - Duration::days(y * 365 + M * 30);
        let w = value.num_weeks();
        value = value - Duration::weeks(w);
        let d = value.num_days();
        value = value - Duration::days(d);
        let h = value.num_hours();
        value = value - Duration::hours(h);
        let m = value.num_minutes();
        value = value - Duration::minutes(m);
        let s = value.num_seconds();
        value = value - Duration::seconds(s);
        let ms = value.num_milliseconds();
        value = value - Duration::milliseconds(ms);
        let us = value.num_microseconds().expect("value too extreme");
        value = value - Duration::microseconds(us);
        let ns = value.num_nanoseconds().expect("value too extreme");
        Self {
            y,
            M,
            w,
            d,
            h,
            m,
            s,
            ms,
            us,
            ns,
        }
    }
}

const UNITS: [&str; 10] = ["y", "M", "w", "d", "h", "m", "s", "ms", "us", "ns"];
const _UNITS_LONG: [&str; 10] = [
    "nanosecond",
    "microsecond",
    "millisecond",
    "second",
    "minute",
    "hour",
    "day",
    "week",
    "month",
    "year",
];

#[cfg(test)]
mod tests {
    use super::{parse_time_span, Rule, SystemDStyleTimeParser};
    use chrono::Duration;
    use pest::Parser;

    // Testing valid time span expressions
    #[test]
    fn test_parse_valid_strings() {
        let valid_time_spans = [
            "5min 30s",
            "2h 30min",
            "10d 5h 30m",
            "1y 6M 3w 5d 4h 20m 30s",
        ];

        let answers = [
            Duration::minutes(5) + Duration::seconds(30),
            Duration::hours(2) + Duration::minutes(30),
            Duration::days(10) + Duration::hours(5) + Duration::minutes(30),
            Duration::days(365)
                + Duration::days(30 * 6)
                + Duration::weeks(3)
                + Duration::days(5)
                + Duration::hours(4)
                + Duration::minutes(20)
                + Duration::seconds(30),
        ];

        for (time_span, answer) in valid_time_spans.iter().zip(answers) {
            let result = SystemDStyleTimeParser::parse(Rule::time_span, time_span);
            assert!(result.is_ok());
            println!("{}", parse_time_span(time_span).unwrap());
            assert_eq!(parse_time_span(time_span).unwrap(), answer);
        }
    }

    #[test]
    fn test_parse_invalid_strings() {
        let invalid_time_spans = vec!["2hx  30min", "10dd 5h 30m", "watermelon"];

        for time_span in invalid_time_spans {
            assert!(parse_time_span(time_span).is_err());
        }
    }

    #[test]
    fn test_parse_empty() {
        assert!(parse_time_span("").is_err());
    }

    #[test]
    fn test_edge_cases() {
        assert!(parse_time_span("1s").is_ok_and(|d| d == Duration::seconds(1)));
        // this isn't actually an edge case for our implementation, but it's a good test
        let hard = parse_time_span("1y 11M 4w 6d 23h 59m 59s");
        let hard_answer = Duration::days(365)
            + Duration::days(30 * 11)
            + Duration::weeks(4)
            + Duration::days(6)
            + Duration::hours(23)
            + Duration::minutes(59)
            + Duration::seconds(59);

        assert!(hard.is_ok_and(|d| d == hard_answer));
    }

    #[test]
    fn test_display() {
        use crate::time::FlatTime;
        use chrono::Duration;

        let time = FlatTime::from(
            Duration::days(365)
                + Duration::days(30 * 11)
                + Duration::weeks(3)
                + Duration::days(6)
                + Duration::hours(23)
                + Duration::minutes(59)
                + Duration::seconds(59),
        );

        assert_eq!(time.format(), "1y 11M 3w 6d 23h 59m 59s");
        assert_eq!(time.format_abbreviated(), "1y");
    }
    #[test]
    fn max_time() {
        let time = crate::time::FlatTime::from(chrono::Duration::milliseconds(i64::MAX));
        assert_eq!(
            time.format(),
            "292471208y 8M 1w 7h 12m 55s 807ms",
            "Unchecked past num of years... simply a regression test"
        );
    }
}
