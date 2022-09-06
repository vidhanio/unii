use clap::Parser;
use unii::course::Code;

#[derive(clap::Parser)]
struct Arguments {
    #[clap(subcommand)]
    subcommand: Subcommand,
}

#[derive(clap::Subcommand)]
enum Subcommand {
    #[clap(subcommand)]
    Course(CourseSubcommand),
}

#[derive(clap::Subcommand)]
enum CourseSubcommand {
    Create {
        code: Code,
        #[clap(short, long, value_parser)]
        name: Option<String>,
        #[clap(short, long, value_parser)]
        description: Option<String>,
    },
    Delete {
        code: String,
    },
    List,
}

fn main() {
    let args = Arguments::parse();
    match args.subcommand {
        Subcommand::Course(subcommand) => match subcommand {
            CourseSubcommand::Create {
                code,
                name,
                description,
            } => {
                println!("Creating course with code {}", code);
                if let Some(name) = name {
                    println!("Course name: {}", name);
                }
                if let Some(description) = description {
                    println!("Course description: {}", description);
                }
            }
            CourseSubcommand::Delete { code } => {
                println!("Deleting course with code {}", code);
            }
            CourseSubcommand::List => {
                println!("Listing courses");
            }
        },
    }
}
