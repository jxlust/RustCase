#[derive(Debug)]
// enum PersonId {
//     Passport,
//     IndentityCard,
// }
enum PersonId {
    Passport(u32),
    IndentityCard(u32, u32, u32),
}
#[derive(Debug)]
struct Person {
    name: String, //fields
    last_name: String,
    age: u32,
    id: PersonId,
}
impl Person {
    fn new() -> Person {
        Person {
            name: "default".to_string(),
            last_name: "default".to_string(),
            age: 10,
            id: PersonId::IndentityCard(10220, 2211, 31333),
        }
    }
    fn from(name: String, last_name: String, age: u32, id: PersonId) -> Person {
        Person {
            name,
            last_name,
            age,
            id,
        }
    }
    fn display_info(&self) {
        println!(
            "{} {} {} {:?}",
            self.name, self.last_name, self.age, self.id
        );
    }
    fn change_age(&mut self, age: u32) {
        self.age = age;
    }
}

fn main() {
    let mut person = Person::new();
    let person_2 = Person::from(
        String::from("ping"),
        String::from("liu"),
        20,
        PersonId::Passport(10000),
    );
    person.display_info();
    person.change_age(30);

    // println!("{:?}", person);
    // println!("{:?}", person_2.id);

    check_person_id(person.id); //It not match passport! / 10220 - 2211 - 31333 / 10220
    check_person_id(person_2.id); // It is match passport! 10000 / 10000 / 10000
}

fn check_person_id(id: PersonId) {
    if let PersonId::Passport(num) = id {
        println!("It is match passport! {}", num);
    } else {
        println!("It not match passport!");
    }
    let result = match id {
        PersonId::Passport(x) => {
            println!("{}", x);
            x
        }
        PersonId::IndentityCard(x, y, z) => {
            println!("{} - {} - {}", x, y, z);
            x
        }
    };
    println!("{}", result);
}
