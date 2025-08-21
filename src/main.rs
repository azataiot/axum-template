fn main() {
    #[derive(Debug)]
    struct User {
        id: String,
        name: String,
        gender: Gender,
    }

    #[derive(Debug)]
    enum Gender {
        MALE,
        FEMALE,
    }

    let user = User {
        id: String::from("1"),
        name: "John".to_string(),
        gender: Gender::MALE,
    };

    println!("{:#?}", user)
}
