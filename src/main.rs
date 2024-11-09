use serde_json;
use std::error::Error;
use std::path::PathBuf;
use structopt::StructOpt;
mod core;
#[derive(StructOpt, Debug)]
struct Opt {
    /// old file path
    #[structopt(parse(from_os_str))]
    old_file_path: PathBuf,
    /// new file path
    #[structopt(parse(from_os_str))]
    new_file_path: PathBuf,
    /// with data
    #[structopt(short="d", long)]
    with_data: bool,
    /// header row —— not implemented
    #[structopt(short="h", long)]
    header_row: Option<usize>
}
fn main() -> Result<(), Box<dyn Error>> {
    let opt = Opt::from_args();
    let res = core::diff_xlsx(
        opt.old_file_path,
        opt.new_file_path,
        opt.with_data);
    match res {
        Ok(s) => println!("{}", serde_json::to_string_pretty(&s)?),
        Err(err) => println!("{:?}", err),
    }
    return Ok(());
}
