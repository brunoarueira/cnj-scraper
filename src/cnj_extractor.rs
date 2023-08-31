use time::OffsetDateTime;
use itertools::Itertools;
use regex::Regex;

const REGEX_CNJ: &str = r"\d{7}-\d{2}\.\d{4}\.\d\.\d{2}\.\d{4}";
const REGEX: &str = r"(\d\s*\d?\s*\d?\s*\d?\s*\d?\s*\d?\s*\d?\s*[- .]?\d\s*\d\s*\.?\d\s*\d\s*\d\s*\d\s*\.?\d\s*\.?\d\s*\d\s*\.?\d\s*\d\s*\d\s*\d)";
const REGEX_NUMBER: &str = r"\d*";
const YEAR_OF_FIRST_LAWSUIT: i32 = 1895; // Year of the first lawsuit in Brazil

pub fn extract(input: &str) -> Vec<String> {
    let last_valid_year: i32 = OffsetDateTime::now_utc().year() + 2;
    let regex = Regex::new(&format!("{}|({})", REGEX, REGEX_CNJ)).unwrap();
    let regex_number = Regex::new(REGEX_NUMBER).unwrap();

    regex
        .captures_iter(&input)
        .map(|regex_match| regex_match.get(0).unwrap().as_str())
        .map(|value| {
            let cnj_without_segment_chars = format!(
                "{:0>20}",
                regex_number
                    .find_iter(value)
                    .map(|v| v.as_str())
                    .collect::<Vec<&str>>()
                    .join("")
            );

            let year = &cnj_without_segment_chars[9..13];
            let parsed_year = year.parse::<i32>().unwrap();

            if parsed_year > last_valid_year || parsed_year < YEAR_OF_FIRST_LAWSUIT {
                return String::from("");
            }

            let new_value = format!(
                "{}-{}.{}.{}.{}.{}",
                &cnj_without_segment_chars[0..7],
                &cnj_without_segment_chars[7..9],
                year,
                &cnj_without_segment_chars[13..14],
                &cnj_without_segment_chars[14..16],
                &cnj_without_segment_chars[16..]
            );

            String::from(new_value)
        })
        .filter(|value| value != "")
        .into_iter()
        .unique()
        .collect()
}
