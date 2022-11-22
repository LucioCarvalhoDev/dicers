pub mod dice {
    use rand::prelude::*;

    pub fn roll(definition: &str) -> Vec<i32> {
        let mut split: Vec<&str> = definition.split("d").collect();

        if split[0] == "" {
            split[0] = "1";
        }

        let mut results: Vec<i32> = Vec::new();
        while results.len() < split[0].to_owned().parse::<i32>().unwrap() as usize {
            results.push(
                rand::thread_rng().gen_range(1..=split[1].to_owned().parse::<i32>().unwrap()),
            );
        }
        results
    }

}
