use crate::schema::xml::{Background, Image, StartTime};
use anyhow::Result;
use std::io::Write;

pub struct GnomeXMLBackgroundSerializer<'a, T: Write> {
    writer: &'a mut T,
}

impl<'a, T> GnomeXMLBackgroundSerializer<'a, T>
where
    T: Write,
{
    pub fn new(writer: &'a mut T) -> Self {
        Self { writer }
    }

    pub fn serialize(&mut self, background: &Background) -> Result<()> {
        // By definition we can only find one starttime
        let StartTime {
            year,
            month,
            day,
            hour,
            minute,
            second,
        } = background.starttime;
        writeln!(self.writer, "<background>")?;
        writeln!(self.writer, "\t<starttime>")?;
        writeln!(self.writer, "\t\t<year>{}</year>", year)?;
        writeln!(self.writer, "\t\t<month>{}</month>", month)?;
        writeln!(self.writer, "\t\t<day>{}</day>", day)?;
        writeln!(self.writer, "\t\t<hour>{}</hour>", hour)?;
        writeln!(self.writer, "\t\t<minute>{}</minute>", minute)?;
        writeln!(self.writer, "\t\t<second>{}</second>", second)?;
        writeln!(self.writer, "\t</starttime>")?;

        for entry in background.images.iter() {
            match entry {
                Image::Static { duration, file, .. } => {
                    writeln!(self.writer, "\t<static>")?;
                    writeln!(self.writer, "\t\t<duration>{}</duration>", duration)?;
                    writeln!(self.writer, "\t\t<file>{}</file>", file)?;
                    writeln!(self.writer, "\t</static>")?;
                }
                Image::Transition {
                    kind,
                    duration,
                    from,
                    to,
                    ..
                } => {
                    writeln!(self.writer, "\t<transition type=\"{}\">", kind)?;
                    writeln!(self.writer, "\t\t<duration>{}</duration>", duration)?;
                    writeln!(self.writer, "\t\t<from>{}</from>", from)?;
                    writeln!(self.writer, "\t\t<to>{}</to>", to)?;
                    writeln!(self.writer, "\t</transition>")?;
                }
            }
        }
        write!(self.writer, "</background>")?;
        self.writer.flush()?;
        Ok(())
    }
}
