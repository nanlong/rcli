use clap::Parser;
use rcli::*;

fn main() {
    let opts = Opts::parse();

    match opts.cmd {
        SubCommand::Csv(csv_opts) => {
            process_csv(
                &csv_opts.input,
                &csv_opts.output,
                csv_opts.format,
                csv_opts.delimiter,
                csv_opts.no_header,
            )
            .unwrap();
        }
    }
}
