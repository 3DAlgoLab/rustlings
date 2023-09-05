#[derive(Debug)]
#[allow(dead_code)]
struct User {
    username: String,
    email: String,
    sign_in_count: u64,
    active: bool,
}

#[allow(dead_code)]
impl User {
    fn incremet_sining_count(&mut self) {
        self.sign_in_count += 1;
    }

    fn change_email(&mut self, new_email: &str) {
        self.email = String::from(new_email)
    }

    fn change_username(&mut self, new_username: &str) {
        self.username = String::from(new_username)
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test1() {
        dbg!("Start!");
        let mut user1 = User {
            username: String::from("Jaeyoon"),
            email: String::from("diem389@gmail.com"),
            active: true,
            sign_in_count: 1,
        };

        user1.username = "anotherusername".to_string();
        dbg!(&user1);

        // change_username(&mut user1, "jay");
        // dbg!(&user1);
        user1.change_username("Jaeyoon Jeong");
        dbg!(&user1);

        user1.change_email("zzblade@naver.com");
        dbg!(&user1);

        user1.incremet_sining_count();
        dbg!(&user1);
    }
}
