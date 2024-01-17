use std::io;
use std::process::ExitCode;

use row2pgcopy::blob::PgBlob;
use row2pgcopy::header::HeaderWriter;

#[derive(serde::Serialize)]
struct Row {
    dat: PgBlob,
}

fn sub<W>(mut wtr: W) -> Result<(), String>
where
    W: io::Write,
{
    let hwtr = row2pgcopy::header::header_writer_default_new(); // HeaderWriter;
    hwtr.write_header(wtr.by_ref())
        .map_err(|e| format!("Unable to write a pgcopy header: {e}"))?;

    let rows = vec![
        Row {
            dat: PgBlob::from("3776"),
        },
        Row {
            dat: PgBlob::from("fuji"),
        },
    ];
    {
        let mut w = wtr.by_ref();

        for row in rows {
            let col_cnt: i16 = 1;
            row2pgcopy::item::write_col_cnt(&mut w, col_cnt)
                .map_err(|e| format!("unable to write a col count: {e}"))?;
            row2pgcopy::item::to_writer(&mut w, &row)
                .map_err(|e| format!("unable to serialize a row: {e}"))?;
        }
    }

    row2pgcopy::trailer::write_trailer(wtr.by_ref())
        .map_err(|e| format!("unable to write a pgcopy trailer: {e}"))?;
    wtr.flush().map_err(|e| format!("unable to flush: {e}"))?;
    Ok(())
}

fn main() -> ExitCode {
    let o = io::stdout();
    let ol = o.lock();
    match sub(ol) {
        Ok(_) => ExitCode::SUCCESS,
        Err(e) => {
            eprintln!("{e}");
            ExitCode::FAILURE
        }
    }
}
