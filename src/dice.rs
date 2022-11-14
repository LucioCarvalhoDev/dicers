pub mod dice {
    use rand::prelude::*;

    pub struct Dice {
        pub def: String,
        pub faces: i32,
        pub quantity: i32,
    }

    impl Dice {
        pub fn new(definition: &str) -> Dice {
            let mut split: Vec<&str> = definition.split("d").collect();

            if split[0] == "" {
                split[0] = "1";
            }
            Dice {
                def: definition.to_owned(),
                faces: split[1].to_owned().parse::<i32>().unwrap(),
                quantity: split[0].to_owned().parse::<i32>().unwrap(),
            }
        }

        pub fn roll(&self) -> Vec<i32> {
            let mut results: Vec<i32> = Vec::new();
            while results.len() < self.quantity as usize {
                results.push(rand::thread_rng().gen_range(1..=self.faces));
            }
            results
        }
    }
}
