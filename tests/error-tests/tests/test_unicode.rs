// Ensures that unicode characters are handled in the JSON output.

fn main() {
    let foo = "❤";
//      ^^^WARN unused variable
//      ^^^NOTE(>=1.17.0) #[warn(unused_variables)]
//      ^^^NOTE(>=1.21.0,<1.22.0) to disable this warning
//      ^^^NOTE(>=1.22.0,<1.25.0-beta) to avoid this warning
//      ^^^HELP(>=1.25.0-beta,<1.44.0-beta) consider
//      ^^^HELP(>=1.44.0-beta) if this is intentional
//      ^^^HELP(>=1.25.0-beta) /Accept Replacement:.*_foo/
}
