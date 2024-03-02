#[derive(Debug)]
enum Gender {
    FEMALE,
    MALE
}

#[derive(Debug)]
struct Person {
    id:u32,
    name:String,
    gender:Gender
} 

fn person_factory(name:String, gender:Gender) -> Person {
    return Person {
        id:12,
        name:name,
        gender:gender
    };
}

fn main() {

    let mut list_person = Vec::new();

    list_person.push(person_factory("Pablo".to_string(), Gender::MALE ));
    list_person.push(person_factory("Sinthia".to_string(), Gender::FEMALE ));


    for person in &list_person{ 
        println!("{:#?}", person)
    }
}
