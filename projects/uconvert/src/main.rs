#![allow(clippy::needless_question_mark)]
mod convert;
mod error;
mod measure;
mod unit;

use anyhow::Result;
use convert::ask_user;
use convert::ConvertRequest;
use error::AppError;

fn main() -> Result<()> {
    let args: Vec<String> = std::env::args().skip(1).collect();
    let request_txt = match args.is_empty() {
        true => ask_user("Convert"),
        false => args.join(" "),
    };
    let request = ConvertRequest::from_text(&request_txt)?;
    let new_value = request.convert()?;
    println!("{} -> {}", request.input, new_value);

    Ok(())
}
