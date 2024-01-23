use std::io;

/// Writes a PGCOPY header(e.g, PGCOPY\n\xff\d\a\0....)
pub trait HeaderWriter {
    fn write_header<W>(&self, wtr: W) -> Result<(), io::Error>
    where
        W: io::Write;
}

struct HeaderWriterDefault {}

impl HeaderWriter for HeaderWriterDefault {
    fn write_header<W>(&self, mut wtr: W) -> Result<(), io::Error>
    where
        W: io::Write,
    {
        write!(wtr, "PGCOPY")?;
        wtr.write_all(&[0x0a, 0xff, 0x0d, 0x0a, 0x00])?;
        wtr.write_all(&[0, 0, 0, 0])?;
        wtr.write_all(&[0, 0, 0, 0])?;
        Ok(())
    }
}

/// Creates a default [`HeaderWriter`]
pub fn header_writer_default_new() -> impl HeaderWriter {
    HeaderWriterDefault {}
}
