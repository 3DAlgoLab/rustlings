#[allow(dead_code, unused_variables)]

struct Person<'p> {
    name: &'p str,
    points: &'p f32,
}

#[cfg(test)]
mod test {
    use super::*;

    fn example_0() {
        let r: &i32;
        // {
        let x: i32 = 5;
        r = &x;
        // }

        dbg!(r);
    }

    fn example_1() {
        let a = 10;
        let b = 20;
        dbg!(largest(&a, &b)); //Expected value = 20
    }

    // fn largest<'a>(compare_1: &'a i32, compare_2: &'a i32) -> &'a i32 {
    //     if compare_1 > compare_2 {
    //         compare_1
    //     } else {
    //         compare_2
    //     }
    // }

    fn largest<'a, T: PartialOrd>(compare_1: &'a T, compare_2: &'a T) -> &'a T {
        if compare_1 > compare_2 {
            compare_1
        } else {
            compare_2
        }
    }

    #[test]
    fn test1() {
        example_0();
        example_1();
    }

    #[test]
    fn test2() {
        let alice = Person {
            name: "Jaeyoon",
            points: &100.0,
        };

        let bob = Person {
            name: "bob",
            points: &40.0,
        };

        let highest = largest(alice.points, bob.points);
        dbg!(highest);
    }
}
