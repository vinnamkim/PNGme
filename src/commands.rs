use clap::{arg, Command};

const ABOUT: &str = r#"
 Your program will have four commands:

1. Encode a message into a PNG file
2. Decode a message stored in a PNG file
3. Remove a message from a PNG file
4. Print a list of PNG chunks that can be searched for messages
"#;

pub(crate) fn cli() -> Command {
    Command::new("pngme")
        .about(ABOUT)
        .subcommand_required(true)
        .arg_required_else_help(true)
        .allow_external_subcommands(true)
        .subcommand(
            Command::new("encode")
                .about("Encode a message into a PNG file")
                .arg(arg!(<REMOTE> "The remote to clone"))
                .arg_required_else_help(true),
        )
        // .subcommand(
        //     Command::new("decode")
        //         .about("Compare two commits")
        //         .arg(arg!(base: [COMMIT]))
        //         .arg(arg!(head: [COMMIT]))
        //         .arg(arg!(path: [PATH]).last(true))
        //         .arg(
        //             arg!(--color <WHEN>)
        //                 .value_parser(["always", "auto", "never"])
        //                 .num_args(0..=1)
        //                 .require_equals(true)
        //                 .default_value("auto")
        //                 .default_missing_value("always"),
        //         ),
        // )
        // .subcommand(
        //     Command::new("remove")
        //         .about("pushes things")
        //         .arg(arg!(<REMOTE> "The remote to target"))
        //         .arg_required_else_help(true),
        // )
        // .subcommand(
        //     Command::new("print")
        //         .about("adds things")
        //         .arg_required_else_help(true)
        //         // .arg(arg!(<PATH> ... "Stuff to add").value_parser(clap::value_parser!(PathBuf))),
        // )
        // .subcommand(
        //     Command::new("stash")
        //         .args_conflicts_with_subcommands(true)
        //         // .args(push_args())
        //         // .subcommand(Command::new("push").args(push_args()))
        //         // .subcommand(Command::new("pop").arg(arg!([STASH])))
        //         // .subcommand(Command::new("apply").arg(arg!([STASH]))),
        // )
}
