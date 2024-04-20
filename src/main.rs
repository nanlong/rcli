use anyhow::Result;
use clap::Parser;
use rcli::*;

#[tokio::main]
async fn main() -> Result<()> {
    let opts = Opts::parse();

    match opts.cmd {
        SubCommand::Csv(csv_opts) => {
            process_csv(
                &csv_opts.input,
                &csv_opts.output,
                csv_opts.format,
                csv_opts.delimiter,
                csv_opts.no_header,
            )?;
        }
        SubCommand::Genpass(genpass_opts) => {
            let password = process_genpass(
                genpass_opts.length,
                genpass_opts.no_upper,
                genpass_opts.no_lower,
                genpass_opts.no_num,
                genpass_opts.no_symbol,
            )?;

            println!("{}", password);
            let estimate = zxcvbn::zxcvbn(&password, &[])?;
            eprintln!("Estimated strength: {}", estimate.score());
        }
        SubCommand::Base64(subcommand) => match subcommand {
            Base64Subcommand::Encode(opts) => {
                let mut reader = get_reader(&opts.input)?;
                let encoded = process_base64(&mut reader, &opts.format, Base64Action::Encode)?;
                println!("{}", encoded);
            }
            Base64Subcommand::Decode(opts) => {
                let mut reader = get_reader(&opts.input)?;
                let decoded = process_base64(&mut reader, &opts.format, Base64Action::Decode)?;
                println!("{}", decoded);
            }
        },
        SubCommand::Text(subcommand) => match subcommand {
            TextSubcommand::Sign(opts) => {
                let mut reader = get_reader(&opts.input)?;
                let key = get_content(&opts.key)?;
                let signed = process_sign(&mut reader, &key, opts.format)?;
                println!("{}", signed);
            }
            TextSubcommand::Verify(opts) => {
                let mut reader = get_reader(&opts.input)?;
                let key = get_content(&opts.key)?;
                let sig = get_content(&opts.sig)?;
                let verified = process_verify(&mut reader, &key, &sig, opts.format)?;
                println!("{}", verified);
            }
            TextSubcommand::Generate(opts) => {
                let key = process_generate(opts.format);
                output_contents(&opts.output, &key);
            }
            TextSubcommand::Encrypt(opts) => {
                let mut reader = get_reader(&opts.input)?;
                let key = get_content(&opts.key)?;
                let encrypted = process_encrypt(&mut reader, &key)?;
                output_contents(&opts.output, &encrypted);
            }
            TextSubcommand::Decrypt(opts) => {
                let mut reader = get_reader(&opts.input)?;
                let key = get_content(&opts.key)?;
                let decrypted = process_decrypt(&mut reader, &key)?;
                output_contents(&opts.output, &decrypted);
            }
        },
        SubCommand::Http(subcommand) => match subcommand {
            HttpSubcommand::Server(opts) => {
                process_server(opts.dir, opts.port).await?;
            }
        },
        SubCommand::JWT(subcommand) => match subcommand {
            JwtSubcommand::Sign(opts) => {
                match process_jwt_sign(&opts.key, &opts.aud, &opts.exp, &opts.iss, &opts.sub) {
                    Ok(token) => println!("Sign JWT: \n{}", token),
                    Err(e) => eprintln!("Error: {}", e),
                }
            }
            JwtSubcommand::Verify(opts) => {
                let verified = process_jwt_verify(&opts.key, &opts.token, &opts.aud);
                println!("Verify JWT: {}", verified.is_ok());
            }
        },
    }

    Ok(())
}
