trait Attacker {
    fn choose_style(&self) -> String;
    // fn choose_weapon(&self) -> String;
}

#[derive(Debug)]
enum Character {
    Warrior,
    Archer,
    Wizard,
}

impl Attacker for Character {
    fn choose_style(&self) -> String {
        match self {
            Character::Warrior => "wing chun".to_string(),
            Character::Archer => "king fu".to_string(),
            Character::Wizard => "thai chi".to_string(),
        }
    }

    // fn choose_weapon(&self) -> String {}
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_traits() {
        dbg!("Start!");
        let my_character = Character::Archer;
        let chosen_style = my_character.choose_style();
        dbg!(chosen_style);
    }

    
}
