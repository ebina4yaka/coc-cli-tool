// Call of Cthulhu 6th Edition Status Management
#[derive(Copy, Clone)]
pub enum StatusType {
    Str,
    Con,
    Pow,
    Dex,
    App,
    Siz,
    Int,
    Edu,
}

const STATUS_TYPES: [StatusType; 8] = [
    StatusType::Str,
    StatusType::Con,
    StatusType::Pow,
    StatusType::Dex,
    StatusType::App,
    StatusType::Siz,
    StatusType::Int,
    StatusType::Edu,
];

impl StatusType {
    fn iterator() -> impl Iterator<Item = StatusType> {
        STATUS_TYPES.iter().copied()
    }

    fn to_string(&self) -> &str {
        match self {
            StatusType::Str => "STR",
            StatusType::Con => "CON",
            StatusType::Pow => "POW",
            StatusType::Dex => "DEX",
            StatusType::App => "APP",
            StatusType::Siz => "SIZ",
            StatusType::Int => "INT",
            StatusType::Edu => "EDU",
        }
    }

    fn values() -> impl Iterator<Item = StatusType> {
        StatusType::iterator()
    }

    fn get_dice(&self) -> Dice {
        match self {
            StatusType::Str
            | StatusType::Con
            | StatusType::Pow
            | StatusType::Dex
            | StatusType::App => Dice::new(3, 6),
            StatusType::Siz | StatusType::Int => Dice::new(2, 6),
            StatusType::Edu => Dice::new(3, 6),
        }
    }

    fn roll_special(&self) -> i32 {
        match self {
            StatusType::Siz | StatusType::Int => 6,
            StatusType::Edu => 3,
            _ => 0,
        }
    }
}

pub struct Sheet {
    pub statuses: Vec<Status>,
}

impl Sheet {
    pub fn new() -> Self {
        let mut statuses = Vec::new();
        for status_type in StatusType::values() {
            let mut status = Status::new(status_type);
            status.roll_status();
            statuses.push(status);
        }
        Sheet { statuses }
    }

    pub fn print_statuses(&self) {
        for status in &self.statuses {
            println!("{}", status.to_string());
        }
    }
}

pub struct Status {
    status_type: StatusType,
    dice: Dice,
    value: i32,
}

impl Status {
    fn new(status_type: StatusType) -> Self {
        Status::new_with_dice(status_type, status_type.get_dice())
    }

    fn new_with_dice(status_type: StatusType, dice: Dice) -> Self {
        Status {
            status_type,
            dice,
            value: 0,
        }
    }

    pub fn roll_status(&mut self) {
        self.dice.roll();
        self.finish_roll();
    }

    fn finish_roll(&mut self) {
        self.value = self.dice.result() + self.status_type.roll_special();
    }

    #[cfg(test)]
    fn roll_status_with<F>(&mut self, generator: F)
    where
        F: FnMut(u8) -> u8,
    {
        self.dice.roll_with(generator);
        self.finish_roll();
    }

    fn dice_value_string(&self) -> String {
        if self.value >= 10 {
            format!("{}", self.value)
        } else {
            format!(" {}", self.value)
        }
    }

    pub fn to_string(&self) -> String {
        let bonus = self.status_type.roll_special();
        if bonus > 0 {
            return format!(
                "{}: {} ({}+{})",
                self.status_type.to_string(),
                self.dice_value_string(),
                self.dice.display_rolls(),
                bonus,
            );
        }
        format!(
            "{}: {} ({})",
            self.status_type.to_string(),
            self.dice_value_string(),
            self.dice.display_rolls(),
        )
    }
}

// Dice struct and its methods

extern crate rand;
use rand::Rng;

pub struct Dice {
    num: u8,
    max: u8,
    roll: Vec<u8>,
}

impl Dice {
    fn new(num: u8, max: u8) -> Self {
        Dice {
            num,
            max,
            roll: Vec::new(),
        }
    }

    fn roll(&mut self) {
        let mut rng = rand::rng();
        self.roll_with(|max| rng.random_range(1u8..=max));
    }

    fn roll_with<F>(&mut self, mut generator: F)
    where
        F: FnMut(u8) -> u8,
    {
        self.roll = (0..self.num)
            .map(|_| generator(self.max).clamp(1u8, self.max))
            .collect::<Vec<u8>>();
    }

    fn display_rolls(&self) -> String {
        let rolls: Vec<String> = self.roll.iter().map(|r| r.to_string()).collect();
        rolls.join("+")
    }

    fn result(&self) -> i32 {
        self.roll.iter().map(|&value| i32::from(value)).sum()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn int_status_applies_bonus() {
        let mut status = Status::new(StatusType::Int);
        let mut rolls = vec![4u8, 5u8].into_iter();
        status.roll_status_with(|_| rolls.next().expect("enough rolls"));
        assert_eq!(status.value, 15);
        assert!(status.to_string().contains("(4+5+6)"));
    }

    #[test]
    fn str_status_displays_padded_value() {
        let mut status = Status::new(StatusType::Str);
        let mut rolls = vec![2u8, 2u8, 3u8].into_iter();
        status.roll_status_with(|_| rolls.next().expect("enough rolls"));
        assert_eq!(status.value, 7);
        assert!(status.to_string().contains("STR:  7 (2+2+3)"));
    }

    #[test]
    fn dice_roll_with_clamps_out_of_range_values() {
        let mut dice = Dice::new(2, 6);
        let mut values = vec![8u8, 0u8].into_iter();
        dice.roll_with(|max| values.next().unwrap_or(max));
        assert_eq!(dice.display_rolls(), "6+1");
        assert_eq!(dice.result(), 7);
    }
}
