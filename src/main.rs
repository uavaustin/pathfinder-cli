use clap;

fn main() {
    // Load subcommands, options, etc. from YAML file
    let yaml = clap::load_yaml!("cli.yaml");
    // Get subcommands, options, etc. used in the command line
    let matches = clap::App::from_yaml(yaml).get_matches();
    if matches.subcommand_name().is_none() {
        println!("Hello world!");
    }
}

#[cfg(test)]
mod tests {
    use pathfinder::*;

    #[test]
    fn make_flyzone() {
        let _flyzone = vec![vec![
            Location::from_degrees(30.32469, -97.60466, 0f32),
            Location::from_degrees(30.32437, -97.60367, 0f32),
            Location::from_degrees(30.32356, -97.60333, 0f32)
        ]];
    }
}
