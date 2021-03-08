fn main() {
    println!("Hello, world!");
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
