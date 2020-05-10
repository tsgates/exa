use datetime::TimeZone;
use ansi_term::Style;
use ansi_term::Colour::{Red, Cyan, Yellow, Green, Purple, Blue};

use crate::output::cell::TextCell;
use crate::output::time::TimeFormat;


pub trait Render {
    fn render(self, style: Style,
                        tz: &Option<TimeZone>,
                        format: &TimeFormat) -> TextCell;
}

impl Render for std::time::Duration {
    fn render(self, style: Style,
                        tz: &Option<TimeZone>,
                        format: &TimeFormat) -> TextCell {

        if let Some(ref tz) = *tz {
            let datestamp = format.format_zoned(self, tz);
            let mut style = style;

            // XXX. intercept the rendering in an ad-hoc way
            if let TimeFormat::Human(_) = format {
                style = Style::new().fg(
                    if datestamp.ends_with("s") {
                        Red
                    } else if datestamp.ends_with("m") {
                        Cyan
                    } else if datestamp.ends_with("h") {
                        Yellow
                    } else if datestamp.ends_with("d") {
                        Green
                    } else if datestamp.ends_with("M") {
                        Purple
                    } else {
                        Blue
                    }
                );
            }

            TextCell::paint(style, datestamp)
        }
        else {
            let datestamp = format.format_local(self);
            TextCell::paint(style, datestamp)
        }
    }
}
