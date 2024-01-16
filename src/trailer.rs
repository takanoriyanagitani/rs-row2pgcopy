use std::io;

pub fn write_trailer<W>(mut wtr: W) -> Result<(), io::Error>
where
    W: io::Write,
{
    wtr.write_all(&[0xff, 0xff])?;
    Ok(())
}
