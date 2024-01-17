use core::time::Duration;

use std::io;
use std::process::ExitCode;
use std::time::SystemTime;

use row2pgcopy::header::HeaderWriter;
use row2pgcopy::item::PgNumArray;
use row2pgcopy::time::systemtime::Timestampz;
use row2pgcopy::uuid::Uuid;

#[derive(serde::Serialize)]
struct Row {
    id: Uuid,
    i2: i16,
    f4: f32,
    tx: String,
    a2: PgNumArray<i16>,
    a8: PgNumArray<i64>,
    ad: PgNumArray<f64>,
    tz: Timestampz,
}

fn sub<W>(mut wtr: W) -> Result<(), String>
where
    W: io::Write,
{
    let hwtr = row2pgcopy::header::header_writer_default_new(); // HeaderWriter;
    hwtr.write_header(wtr.by_ref())
        .map_err(|e| format!("Unable to write a pgcopy header: {e}"))?;

    let rows: Vec<Row> = vec![
        Row {
            id: Uuid(0xcafef00d_dead_beaf_face_864299792458),
            i2: 42,
            f4: 1.01325,
            tx: "fuji".into(),
            a2: PgNumArray(vec![3776, 599]),
            a8: PgNumArray(vec![333, 634]),
            ad: PgNumArray(vec![3.776, 0.599]),
            tz: Timestampz::from(
                SystemTime::UNIX_EPOCH
                    .checked_add(Duration::from_millis(0))
                    .unwrap(),
            ),
        },
        Row {
            id: Uuid(0xdafef00d_dead_beaf_face_864299792458),
            i2: 43,
            f4: 2.01325,
            tx: "takao".into(),
            a2: PgNumArray(vec![4776, 699]),
            a8: PgNumArray(vec![433, 734]),
            ad: PgNumArray(vec![4.776, 0.699]),
            tz: Timestampz::from(
                SystemTime::UNIX_EPOCH
                    .checked_add(Duration::from_millis(1))
                    .unwrap(),
            ),
        },
    ];

    {
        let mut w = wtr.by_ref();

        for row in rows {
            let col_cnt: i16 = 8;
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
