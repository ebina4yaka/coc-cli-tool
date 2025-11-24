use rand::Rng;

pub struct Dice {
    num: u8,
    max: u8,
    roll: Vec<u8>,
}

impl Dice {
    pub fn new(num: u8, max: u8) -> Self {
        Dice {
            num,
            max,
            roll: Vec::new(),
        }
    }

    pub fn roll(&mut self) {
        let mut rng = rand::rng();
        self.roll_with(|max| rng.random_range(1u8..=max));
    }

    pub fn roll_with<F>(&mut self, mut generator: F)
    where
        F: FnMut(u8) -> u8,
    {
        self.roll = (0..self.num)
            .map(|_| generator(self.max).clamp(1u8, self.max))
            .collect::<Vec<u8>>();
    }

    pub fn display_rolls(&self) -> String {
        let rolls: Vec<String> = self.roll.iter().map(|r| r.to_string()).collect();
        rolls.join("+")
    }

    pub fn result(&self) -> i32 {
        self.roll.iter().map(|&value| i32::from(value)).sum()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn dice_roll_with_clamps_out_of_range_values() {
        let mut dice = Dice::new(2, 6);
        let mut values = vec![8u8, 0u8].into_iter();
        dice.roll_with(|max| values.next().unwrap_or(max));
        assert_eq!(dice.display_rolls(), "6+1");
        assert_eq!(dice.result(), 7);
    }
}
