#[derive(Debug, Clone)]
pub enum WidgetSize {
    Small,
    Medium,
    Large,
    ExtraLarge,

    Invalid(String),
}

mod str {
    use super::WidgetSize;
    use super::WidgetSize::*;

    impl ToString for WidgetSize {
        fn to_string(&self) -> String {
            match self {
                Small => String::from("Small"),
                Medium => String::from("Medium"),
                Large => String::from("Large"),
                ExtraLarge => String::from("ExtraLarge"),
                Invalid(x) => format!("[!!INVALID: '{}']", x),
            }
        }
    }

    impl std::str::FromStr for WidgetSize {
        type Err = WidgetSize;

        fn from_str(s: &str) -> Result<Self, Self::Err> {
            match s {
                "Small" => Ok(Small),
                "Medium" => Ok(Medium),
                "Large" => Ok(Large),
                "ExtraLarge" => Ok(ExtraLarge),
                x => Err(Invalid(x.to_string())),
            }
        }
    }
}
