use structopt::StructOpt;

#[derive(Debug, StructOpt)]
#[structopt(name = "oxy2fa", about = "oxy2fa is a two-factor authentication agent.")]
struct Cli {

}

#[derive(StructOpt)]
enum Command {
    Keys {

    },
    Add {

    }
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = Cli::from_args();
    println!("{:?}", args);
    Ok(())
}

#[cfg(test)]
mod test_oxy2fa {

    #[test]
    fn something() {

    }
}