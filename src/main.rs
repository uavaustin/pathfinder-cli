use std::io::Read;

fn main() -> std::io::Result<()> {
    let mut args: Vec<String> = std::env::args().collect();
    // Take executable path from args (read docs on std::env::args() for possible issues)
    // TODO: Replace this with string literal in '--help' messages
    let exec_path = args.drain(0..=0).last().expect("First item of arguments not found.");

    // No arguments passed
    if args.is_empty() {
        println!("No default is currently implemented. Use some argument.\n\
                 For more information try --help");
        return Ok(());
    }

    // Used to determine which task is being done: (vehicle avoidance, pathfinder)
    let mut task = (false, false);

    // Process each argument
    // TODO: Consider having argument priority and cancelling (ex. '-h' causing the process not to run)
    // TODO: Have a cleaner method of sorting and printing these messages (maybe clap?)
    for a in args {
        match a.as_str() {
            // Vehicle avoidance task
            "avoid" | "a" => {
                task.0 = true;
            },
            // Pathfinder task
            "pathfinder" | "p" => {
                task.1 = true;
            },
            // Help
            "-h" | "--help" => {
                // Check if the task is vehicle avoidance
                // TODO: Explain what '<DATA>' means (and decide what kind of data should be given)
                if task.0 {
                    println!(
                        "USAGE:\
                        \n    {ep} avoid [OPTIONS]\
                        \n\nOPTIONS:\
                        \n    -h, --help          Prints help information", ep=exec_path);
                }
                // Check if the task is pathfinder
                else if task.1 {
                    println!(
                        "USAGE:\
                        \n    {ep} pathfinder [OPTIONS]\
                        \n\nOPTIONS:\
                        \n    -f, --full <DATA>   Run full pathfinder\
                        \n    -h, --help          Prints help information", ep=exec_path);
                }
                // If no task was given
                else {
                    println!(
                        "USAGE:\
                        \n    {ep} [+task] [OPTIONS]\
                        \n\nOPTIONS:\
                        \n    -h, --help          Prints help information\
                        \n\nTASKS:\
                        \n    pathfinder, p       Runs pathfinder -- Not yet implemented!\
                        \n    avoid, a            Runs vehicle avoidance -- Not yet implemented!", ep=exec_path);
                }
            },
            // Unexpected argument
            // TODO: Add colors (run 'cargo build -1' as reference)
            _ => println!(
                "error: Found argument {arg} which wasn't expected, or isn't valid in this context\n\n\
                USAGE:\
                \n    {ep} [+task] [OPTIONS]\n\n\
                For more information try --help", arg=a, ep=exec_path),
        }
    }

    Ok(())
}
