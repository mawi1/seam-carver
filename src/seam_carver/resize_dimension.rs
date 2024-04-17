use std::str::FromStr;

use anyhow::{bail, Context};
use regex::Regex;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ResizeDimension {
    Width(u32),
    Height(u32),
    WidthHeight(u32, u32),
}

impl FromStr for ResizeDimension {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let re = Regex::new(
            r"^((?<Width>\d+)x(?<Height>\d+))|((?<OnlyWidth>\d+)x)|(x(?<OnlyHeight>\d+))$",
        )
        .unwrap();

        const PARSE_WIDTH_ERR: &'static str = "could not parse width";
        const PARSE_HEIGHT_ERR: &'static str = "could not parse width";

        if let Some(caps) = re.captures(s) {
            if let Some(m) = caps.name("OnlyWidth") {
                let w = m.as_str().parse().context(PARSE_WIDTH_ERR)?;
                return Ok(ResizeDimension::Width(w));
            } else if let Some(m) = caps.name("OnlyHeight") {
                let h = m.as_str().parse().context(PARSE_HEIGHT_ERR)?;
                return Ok(ResizeDimension::Height(h));
            } else {
                let w = (&caps["Width"]).parse::<u32>().context(PARSE_WIDTH_ERR)?;
                let h = (&caps["Height"]).parse::<u32>().context(PARSE_HEIGHT_ERR)?;
                return Ok(ResizeDimension::WidthHeight(w, h));
            }
        } else {
            bail!("invalid dimensions")
        };
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_rd_with_height() {
        let rd = ResizeDimension::from_str("20x22").unwrap();
        assert_eq!(rd, ResizeDimension::WidthHeight(20, 22));
    }

    #[test]
    fn test_rd_with() {
        let rd = ResizeDimension::from_str("20x").unwrap();
        assert_eq!(rd, ResizeDimension::Width(20));
    }

    #[test]
    fn test_rd_height() {
        let rd = ResizeDimension::from_str("x100").unwrap();
        assert_eq!(rd, ResizeDimension::Height(100));
    }

    #[test]
    fn test_rd_invalid() {
        let r = ResizeDimension::from_str("Foo").unwrap_err();
        assert_eq!(r.to_string(), "invalid dimensions");
    }
}
