//! Functions for pgcopy trailer(-1 as i16)

use std::io;

/// Writes trailer bytes(-1 as i16)
pub fn write_trailer<W>(mut wtr: W) -> Result<(), io::Error>
where
    W: io::Write,
{
    wtr.write_all(&[0xff, 0xff])?;
    Ok(())
}
