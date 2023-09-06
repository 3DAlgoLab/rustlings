#[allow(unused)]
use std::collections::{HashMap, HashSet};

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn tests_hashmap() {
        dbg!("Module collections Test...");

        let person_1 = "alice";
        let person_2 = "bob";

        // hashmap
        let mut results_hm: HashMap<&str, u32> = HashMap::new();
        results_hm.insert(person_1, 55);
        results_hm.insert(person_2, 51);

        let test_result = results_hm.get(person_1);
        dbg!(test_result);
        dbg!(test_result.unwrap());
        dbg!(&results_hm);

        if results_hm.contains_key("alice") {
            dbg!("Alice is present!");
        }
    }

    #[test]
    fn test_hashset() {
        let mut names_hs: HashSet<&str> = HashSet::new();
        names_hs.insert("Alice");
        names_hs.insert("Bob");
        names_hs.insert("Jaeyoon");

        if names_hs.contains("Bob") {
            dbg!("Bos is here");
        }
    }
}
