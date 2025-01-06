use clap::Parser;
use clap::Subcommand;

use crate::container::Container;

#[derive(Parser)]
#[command(version, about, long_about = None)]
#[command(propagate_version = true)]
struct Cli {
    format: Option<String>,
    width: Option<usize>,
    height: Option<usize>,

    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    Opt { image: String },
}

pub struct Console {
    pub container: Container,
}

impl Console {
    pub fn execute(&self) -> Result<(), std::io::Error> {
        let cli = Cli::parse();

        match cli.command {
            Commands::Opt { image } => self.handle_opt(image, cli.format, cli.width, cli.height),
            _ => unreachable!(),
        }

        Ok(())
    }

    fn handle_opt(
        &self,
        file_path: String,
        format: Option<String>,
        width: Option<usize>,
        height: Option<usize>,
    ) {
        todo!();
        // let param = OptImgParam {image, specification};

        // self.container.image_service.opt_img(param);
    }
}
