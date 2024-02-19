use std::path::PathBuf;

use clap::{Parser, Subcommand, ValueEnum};
use psl::builtin_connectors::{BUILTIN_CONNECTORS, MYSQL, POSTGRES, SQLITE};

#[derive(ValueEnum, Debug, Clone)]
#[clap(rename_all = "kebab_case")]
enum SerializationFormat {
    Bytes,
    Json,
}

#[derive(Parser)]
#[command()]
struct Cli {
    #[command(subcommand)]
    command: Option<Commands>,
}

#[derive(Subcommand)]
enum Commands {
    // Given a textual `.prisma` file, read it, validate it, print its `ValidatedSchema`, and write it to a `.bin` file.
    Serialize {
        /// Path to the input `.prisma` file.
        #[arg(short, long)]
        input: PathBuf,

        /// The format to use during serialization.
        #[arg(short, long)]
        format: SerializationFormat,

        /// Path to the output serialized file.
        #[arg(short, long)]
        output: PathBuf,
    },
    // Given a binary `.bin` file or a textual `.json` file, read it as a `ValidatedSchema`, and print it.
    Deserialize {
        /// Path to the input serialized file.
        #[arg(short, long)]
        input: PathBuf,

        /// The format to use during deserialization.
        #[arg(short, long)]
        format: SerializationFormat,
    },
}

fn main() -> std::io::Result<()> {
    let cli = Cli::parse();

    // You can check for the existence of subcommands, and if found use their
    // matches just as you would the top level cmd
    match cli.command {
        Some(Commands::Serialize { input, output, format }) => {
            println!("Serializing {:?}...", &input.as_os_str());

            let schema_as_text = std::fs::read_to_string(input)?;

            match format {
                SerializationFormat::Bytes => {
                    let schema_as_bytes = psl::serialize_to_bytes(schema_as_text.into(), BUILTIN_CONNECTORS)
                        .map_err(|e| std::io::Error::new(std::io::ErrorKind::Other, e))?;

                    println!("Writing to {:?}", output);
                    std::fs::write(output, schema_as_bytes)?;
                }
                SerializationFormat::Json => {
                    let schema_as_json = psl::serialize_to_json(schema_as_text.into(), BUILTIN_CONNECTORS)
                        .map_err(|e| std::io::Error::new(std::io::ErrorKind::Other, e))?;

                    println!("Writing to {:?}", output);
                    std::fs::write(output, schema_as_json)?;
                }
            };

            Ok(())
        }
        Some(Commands::Deserialize { input, format }) => {
            println!("Deserializing {:?}...", &input.as_os_str());

            let connector_registry: psl::ValidatedConnectorRegistry<'_> =
                &[POSTGRES, MYSQL, SQLITE].map(|c| c.as_validated_connector());

            let schema_qe = match format {
                SerializationFormat::Bytes => {
                    let schema_as_binary = std::fs::read(input)?;
                    psl::deserialize_from_bytes(schema_as_binary.as_slice(), &connector_registry)
                        .map_err(|e| std::io::Error::new(std::io::ErrorKind::Other, e))
                }
                SerializationFormat::Json => {
                    let schema_as_json = std::fs::read_to_string(input)?;
                    psl::deserialize_from_json(schema_as_json.as_str(), &connector_registry)
                        .map_err(|e| std::io::Error::new(std::io::ErrorKind::Other, e))
                }
            }?;

            println!("connector.provider_name(): {}", &schema_qe.connector.provider_name());

            Ok(())
        }
        None => {
            print!("No subcommand provided. Exiting.");
            Ok(())
        }
    }
}
