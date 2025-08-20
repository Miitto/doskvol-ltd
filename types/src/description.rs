#[derive(Debug, Clone, PartialEq, Eq, Hash, serde::Serialize, serde::Deserialize)]
pub struct Description<T: std::fmt::Display>(T);

pub enum Segment {
    Text(String),
    Italic(String),
    Bold(String),
}

impl<T: std::fmt::Display> Description<T> {
    pub const fn new(value: T) -> Self {
        Self(value)
    }

    pub fn to_segments(&self) -> Vec<Segment> {
        let string = self.to_string();
        let mut segments = vec![];

        let mut chars = string.chars().peekable();

        let mut start = String::new();
        while chars.peek().is_some_and(|&char| char != '*') {
            start.push(chars.next().unwrap());
        }

        if !start.is_empty() {
            segments.push(Segment::Text(start));
        }

        while let Some(char) = chars.next() {
            if char == '*' {
                let mut aster_count = 1;
                let mut text = String::new();

                // Check for **
                if let Some(&next_char) = chars.peek()
                    && next_char == '*'
                {
                    chars.next(); // consume second '*'
                    aster_count = 2;
                }

                while let Some(next_char) = chars.next() {
                    if next_char == '*' {
                        // If we encounter another '*', we stop reading
                        if aster_count == 2 {
                            if let Some(&next_next_char) = chars.peek()
                                && next_next_char == '*'
                            {
                                chars.next(); // consume second '*'
                                segments.push(Segment::Bold(text));
                            } else {
                                if let Some(last) = segments.last_mut() {
                                    match last {
                                        Segment::Text(txt) => {
                                            txt.push('*');
                                        }
                                        Segment::Italic(txt) => {
                                            txt.push('*');
                                        }
                                        Segment::Bold(txt) => {
                                            txt.push('*');
                                        }
                                    }
                                }
                                segments.push(Segment::Italic(text));
                            }
                        } else {
                            segments.push(Segment::Italic(text));
                        }
                        break;
                    } else {
                        text.push(next_char);
                    }
                }
            } else {
                let mut text = String::from(char);
                while chars.peek().is_some_and(|&char| char != '*') {
                    text.push(chars.next().unwrap());
                }
                segments.push(Segment::Text(text));
            }
        }

        segments
    }
}

impl<'a> From<&'a str> for Description<&'a str> {
    fn from(value: &'a str) -> Self {
        Self(value)
    }
}

impl From<&str> for Description<String> {
    fn from(value: &str) -> Self {
        Self(value.to_string())
    }
}

impl<T: std::fmt::Display> std::fmt::Display for Description<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}
