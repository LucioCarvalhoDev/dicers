pub mod funcs {
    pub fn sum(rolls: &Vec<i32>) -> i32 {
        let mut res = 0;
        let mut i = 0;

        while i < rolls.len() {
            res += rolls[i];
            i += 1;
        }
        res
    }

    pub fn max(rolls: &Vec<i32>) -> i32 {
        let mut res = 0;
        let mut i = 0;

        while i < rolls.len() {
            if rolls[i] > res {
                res = rolls[i];
            }
            i += 1;
        }

        res
    }

    pub fn min(rolls: &Vec<i32>) -> i32 {
        let mut res = i32::MAX;
        let mut i = 0;
        while i < rolls.len() {
            if rolls[i] < res {
                res = rolls[i];
            }
            i += 1;
        }

        res
    }

    pub fn med(rolls: &Vec<i32>) -> f32 {
        let mut res = 0f32;
        let mut i = 0;

        while i < rolls.len() {
            res += rolls[i] as f32;
            i += 1;
        }
        res / (rolls.len() as f32)
    }
}
