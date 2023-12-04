use thiserror::Error;

#[derive(Error, Debug, PartialEq, Eq)]
pub enum ColorParserError {
    #[error("Invalid color count record: {0}")]
    InvalidColorCount(String),

    #[error("Invalid color: {0}")]
    InvalidColor(String),

    #[error("Invalid count: {0}")]
    InvalidCount(String),
}

#[derive(Debug, PartialEq, Eq)]
pub enum Color {
    Red,
    Green,
    Blue,
}

#[derive(Debug, PartialEq, Eq)]
pub struct ColorCount {
    pub red: Option<u16>,
    pub green: Option<u16>,
    pub blue: Option<u16>,

    pub order: Vec<Color>,
}

impl ColorCount {
    pub fn new(
        red: Option<u16>,
        green: Option<u16>,
        blue: Option<u16>,
        order: Option<Vec<Color>>,
    ) -> Self {
        Self {
            red,
            green,
            blue,
            order: order.unwrap_or_else(|| vec![]),
        }
    }

    pub fn default() -> Self {
        Self {
            red: None,
            green: None,
            blue: None,
            order: vec![],
        }
    }

    pub fn parse(color_counts_str: &str) -> Result<Self, ColorParserError> {
        parse_color_counts(color_counts_str)
    }

    pub fn all_some(&self) -> bool {
        self.red.is_some() && self.green.is_some() && self.blue.is_some()
    }

    pub fn get(&self, color: &Color) -> Option<u16> {
        match color {
            Color::Red => self.red,
            Color::Green => self.green,
            Color::Blue => self.blue,
        }
    }

    pub fn set(&mut self, color: &Color, count: Option<u16>) {
        match color {
            Color::Red => self.red = count,
            Color::Green => self.green = count,
            Color::Blue => self.blue = count,
        }
    }

    pub fn gt(&self, other: &Self, color: &Color) -> bool {
        self.get(color).unwrap_or(0) > other.get(color).unwrap_or(0)
    }
}

/// Parses colors into a color count
fn parse_color_counts(color_counts_str: &str) -> Result<ColorCount, ColorParserError> {
    let mut color_records = color_counts_str.split(",");
    let mut result = ColorCount::default();

    color_records.try_for_each(|record| match parse_color_count(record) {
        Ok((color, count)) => {
            match color {
                "red" => {
                    result.red = Some(count);
                    result.order.push(Color::Red);
                }
                "green" => {
                    result.green = Some(count);
                    result.order.push(Color::Green)
                }
                "blue" => {
                    result.blue = Some(count);
                    result.order.push(Color::Blue)
                }
                _ => return Err(ColorParserError::InvalidColor(color.to_string())),
            }
            Ok(())
        }
        Err(e) => Err(e),
    })?;
    Ok(result)
}

/// Parses a color count record into a color and count
/// The record should be in the format: <count> <color>
/// For example: 1 red
fn parse_color_count(record: &str) -> Result<(&str, u16), ColorParserError> {
    let mut parts = record.trim().split(" ");
    let count_str = parts.next();
    let color_str = parts.next();

    // Extract the count and color parts
    if let (Some(count), Some(color)) = (count_str, color_str) {
        // Parse the count
        let count = count.parse::<u16>().or_else(|_| {
            Err(ColorParserError::InvalidCount(
                count_str
                    .unwrap_or_else(|| "Value not provided")
                    .trim()
                    .to_string(),
            ))
        })?;
        Ok((color, count))
    } else {
        Err(ColorParserError::InvalidColorCount(
            record.trim().to_string(),
        ))
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn parses_a_valid_colors_string() {
        let colors_str = "1 red, 2 green, 3 blue";
        let color_count = parse_color_counts(colors_str).unwrap();
        assert_eq!(
            color_count,
            ColorCount::new(
                Some(1),
                Some(2),
                Some(3),
                Some(vec![Color::Red, Color::Green, Color::Blue])
            )
        );
    }

    #[test]
    fn fails_on_invalid_colors_string() {
        let colors_str = "1 red, 2 green, 3 yellow";
        let result = parse_color_counts(colors_str);
        assert!(result.is_err());
        assert_eq!(
            result.unwrap_err(),
            ColorParserError::InvalidColor("yellow".to_string())
        );

        let colors_str = "1 red, 2 green, 3";
        let result = parse_color_counts(colors_str);
        assert!(result.is_err());
        assert_eq!(
            result.unwrap_err(),
            ColorParserError::InvalidColorCount("3".to_string())
        );
    }

    #[test]
    fn test_parse_color_count() {
        let record = "1 red";
        let (color, count) = parse_color_count(record).unwrap();
        assert_eq!(color, "red");
        assert_eq!(count, 1);

        let record = "2 green";
        let (color, count) = parse_color_count(record).unwrap();
        assert_eq!(color, "green");
        assert_eq!(count, 2);

        let record = "3 blue";
        let (color, count) = parse_color_count(record).unwrap();
        assert_eq!(color, "blue");
        assert_eq!(count, 3);

        let record = "five red";
        let result = parse_color_count(record);
        assert!(result.is_err());
        assert_eq!(
            result.unwrap_err(),
            ColorParserError::InvalidCount("five".to_string())
        );
    }
}
