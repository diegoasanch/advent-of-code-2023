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

type ColorCount = u16;

#[derive(Debug, PartialEq, Eq)]
pub struct ColorSet {
    pub red: Option<ColorCount>,
    pub green: Option<ColorCount>,
    pub blue: Option<ColorCount>,
}

impl ColorSet {
    pub fn new(
        red: Option<ColorCount>,
        green: Option<ColorCount>,
        blue: Option<ColorCount>,
    ) -> Self {
        Self { red, green, blue }
    }

    pub fn default() -> Self {
        Self {
            red: None,
            green: None,
            blue: None,
        }
    }

    pub fn parse(color_counts_str: &str) -> Result<Self, ColorParserError> {
        let mut color_records = color_counts_str.split(",");
        let mut result = ColorSet::default();

        color_records.try_for_each(|record| match parse_color_count(record) {
            Ok((color, count)) => {
                match color {
                    "red" => result.red = Some(count),

                    "green" => result.green = Some(count),
                    "blue" => result.blue = Some(count),
                    _ => return Err(ColorParserError::InvalidColor(color.to_string())),
                }
                Ok(())
            }
            Err(e) => Err(e),
        })?;
        Ok(result)
    }

    pub fn all_some(&self) -> bool {
        self.red.is_some() && self.green.is_some() && self.blue.is_some()
    }

    pub fn get(&self, color: &Color) -> Option<ColorCount> {
        match color {
            Color::Red => self.red,
            Color::Green => self.green,
            Color::Blue => self.blue,
        }
    }

    pub fn set(&mut self, color: &Color, count: Option<ColorCount>) {
        match color {
            Color::Red => self.red = count,
            Color::Green => self.green = count,
            Color::Blue => self.blue = count,
        }
    }

    pub fn gt_color(&self, other: &Self, color: &Color) -> bool {
        self.get(color).unwrap_or(0) > other.get(color).unwrap_or(0)
    }

    /// Determines if this color count is greater than the other color count
    /// for all colors
    pub fn gt(&self, other: &Self) -> bool {
        self.gt_color(other, &Color::Red)
            && self.gt_color(other, &Color::Green)
            && self.gt_color(other, &Color::Blue)
    }

    /// Count of each color multiplied together
    pub fn power(&self) -> u32 {
        self.red.unwrap_or(0) as u32
            * self.green.unwrap_or(0) as u32
            * self.blue.unwrap_or(0) as u32
    }
}

/// Parses a color count record into a color and count
/// The record should be in the format: <count> <color>
/// For example: 1 red
fn parse_color_count(record: &str) -> Result<(&str, ColorCount), ColorParserError> {
    let mut parts = record.trim().split(" ");
    let count_str = parts.next();
    let color_str = parts.next();

    // Extract the count and color parts
    if let (Some(count), Some(color)) = (count_str, color_str) {
        // Parse the count
        let count = count.parse::<ColorCount>().or_else(|_| {
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
        let color_count = ColorSet::parse(colors_str).unwrap();
        assert_eq!(color_count, ColorSet::new(Some(1), Some(2), Some(3),));
    }

    #[test]
    fn fails_on_invalid_colors_string() {
        let colors_str = "1 red, 2 green, 3 yellow";
        let result = ColorSet::parse(colors_str);
        assert!(result.is_err());
        assert_eq!(
            result.unwrap_err(),
            ColorParserError::InvalidColor("yellow".to_string())
        );

        let colors_str = "1 red, 2 green, 3";
        let result = ColorSet::parse(colors_str);
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
