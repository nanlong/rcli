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
        SubCommand::Base64(subcommand) => match subcommand {
            Base64Subcommand::Encode(opts) => {
                let mut reader = get_reader(&opts.input).unwrap();
                let encoded =
                    process_base64(&mut reader, &opts.format, Base64Action::Encode).unwrap();
                println!("{}", encoded);
            }
            Base64Subcommand::Decode(opts) => {
                let mut reader = get_reader(&opts.input).unwrap();
                let decoded =
                    process_base64(&mut reader, &opts.format, Base64Action::Decode).unwrap();
                println!("{}", decoded);
            }
        },
        SubCommand::Text(subcommand) => match subcommand {
            TextSubcommand::Sign(opts) => {
                let mut reader = get_reader(&opts.input).unwrap();
                let key = get_content(&opts.key).unwrap();
                let signed = process_sign(&mut reader, &key, opts.format).unwrap();
                println!("{}", signed);
            }
            TextSubcommand::Verify(opts) => {
                let mut reader = get_reader(&opts.input).unwrap();
                let key = get_content(&opts.key).unwrap();
                let sig = get_content(&opts.sig).unwrap();
                let verified = process_verify(&mut reader, &key, &sig, opts.format).unwrap();
                println!("{}", verified);
            }
            TextSubcommand::Generate(opts) => {
                let key = process_generate(opts.format);
                output_contents(&opts.output, &key);
            }
        },
    }
}
