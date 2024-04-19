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
        SubCommand::Genpass(genpass_opts) => {
            let password = process_genpass(
                genpass_opts.length,
                genpass_opts.no_upper,
                genpass_opts.no_lower,
                genpass_opts.no_num,
                genpass_opts.no_symbol,
            )
            .unwrap();

            println!("{}", password);
            let estimate = zxcvbn::zxcvbn(&password, &[]).unwrap();
            eprintln!("Estimated strength: {}", estimate.score());
        }
    }
}
