use std::error::Error;

use actix_multipart::Multipart;
use chrono::NaiveDate;
use encoding_rs::ISO_8859_10;
use encoding_rs_io::DecodeReaderBytesBuilder;
use futures_util::stream::StreamExt as _;

#[derive(Debug)]
struct Transaction {
    date: NaiveDate,
    recipient: String,
    statement_type: String,
    description: String,
    amount_cents: i64,
}

pub async fn read_statement(mut payload: Multipart) -> Result<bool, Box<dyn Error>> {
    // iterate over multipart stream
    while let Some(item) = payload.next().await {
        let mut field = item?;

        // Field in turn is stream of *Bytes* object
        while let Some(chunk) = field.next().await {
            let data: Vec<u8> = chunk?.iter().cloned().collect();

            // transcode from ISO-8859-10 to UTF-8
            let transcoded_reader = DecodeReaderBytesBuilder::new()
                .encoding(Some(ISO_8859_10))
                .build(data.as_slice());

            let mut rdr = csv::ReaderBuilder::new()
                .delimiter(b';')
                .from_reader(transcoded_reader);

            for result in rdr.records() {
                let result = result?;

                let date = NaiveDate::parse_from_str(&result[0], "%d.%m.%Y")?;
                let recipient = result[1].trim().to_string();
                let statement_type = result[2].trim().to_string();
                let description = result[3].trim().to_string();
                let amount_cents: i64 =
                    (result[4].trim().replace(',', ".").parse::<f64>()? * 100.0) as i64;

                let transaction = Transaction {
                    date,
                    recipient,
                    statement_type,
                    description,
                    amount_cents,
                };

                // TODO: push statement into database.
            }
        }
    }

    Ok(true)
}
