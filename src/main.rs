use std::io;

use clap::{App, Arg};
use itertools::{enumerate, process_results, Itertools};
use log::LevelFilter;
use simple_logger::SimpleLogger;
use tokio;
use tokio_postgres::NoTls;

mod array;
mod raw;
mod serialize_for_insert;

use raw::Raw;
use serialize_for_insert::SerializeForInsert;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    SimpleLogger::new()
        .with_level(LevelFilter::Debug)
        .init()
        .unwrap();

    let app = App::new("pg_gensql")
        .version("0.1")
        .author("Alex Chamberlain <alex@alexchamberlain.co.uk>")
        .about("Generate INSERT SQL from a SELECT statement.")
        .arg(
            Arg::new("DB_URL")
                .about("URL to connect to")
                .required(true)
                .index(1),
        )
        .arg(
            Arg::new("SQL")
                .about("SQL statement to run")
                .required(true)
                .index(2),
        );

    let matches = app.get_matches();

    let res: () = run_gensql(
        &mut io::stdout(),
        matches.value_of("DB_URL").unwrap(),
        matches.value_of("SQL").unwrap(),
    )
    .await?;

    return Ok(res);
}

async fn run_gensql<W: io::Write>(
    writer: &mut W,
    db_url: &str,
    sql: &str,
) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    let (client, connection) = tokio_postgres::connect(db_url, NoTls).await?;

    tokio::spawn(async move {
        if let Err(e) = connection.await {
            eprintln!("connection error: {}", e);
        }
    });

    let statement = client.prepare(sql).await?;

    let rows = client.query(&statement, &[]).await?;

    write!(
        writer,
        "INSERT INTO foo({}) VALUES\n",
        statement.columns().iter().map(|c| c.name()).join(",")
    )?;

    for row in rows {
        write!(
            writer,
            "({})",
            process_results(
                enumerate(statement.columns()).map(|(i, col)| {
                    let value: Option<Raw> = row.get(i);
                    SerializeForInsert::serialize(col.type_(), &value)
                }),
                |mut iter| iter.join(",")
            )?
        )?;
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use std::io::Cursor;

    use super::*;

    #[tokio::test]
    async fn test_run_gensql() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
        let mut c = Cursor::new(Vec::new());

        run_gensql(
            &mut c,
            "host=localhost password=example user=postgres port=5433",
            "SELECT * FROM test_scalar_not_null",
        )
        .await?;

        let v = c.into_inner();
        let s = std::str::from_utf8(&v)?;

        assert_eq!(s, "INSERT INTO foo(id,a_text,a_uuid,a_bool,a_char,a_int2,a_int4,a_int8,a_float4,a_float8,a_timestamptz,a_date,a_jsonb) VALUES
(1,'Hello, World!','3c8bc504-5281-471b-bd3d-0aa82da7c6c1',true,'a',42,65537,4294967297,3.142,3.142,'2020-01-01T01:30:00+00:00','2021-01-01','{\"hello\":\"world\"}'::jsonb)");

        Ok(())
    }
}
