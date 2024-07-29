use rust_xlsxwriter::{Workbook, XlsxError};
use std::env;

fn main() -> Result<(), XlsxError> {
    let args: Vec<String> = env::args().collect();
    let mut book = Workbook::new();
    let sheet = book.add_worksheet();
    for arg in args.into_iter().skip(0) {
        let params: Vec<&str> = arg.split_whitespace().collect();
        for (i, p) in params.into_iter().enumerate() {
            sheet.write_string(i as u32, 0, p).expect("cannot write");
        }
    }

    book.save("./temp/testbook.xlsx").expect("cannot save");

    Ok(())
}
