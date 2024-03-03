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

    let mut array_person = [
        person_factory("Pablo".to_string(), Gender::MALE ),
        person_factory("Sinthia".to_string(), Gender::FEMALE ),
        person_factory("Antonella".to_string(), Gender::FEMALE )
,   ];

    let n = 3;

    let mut  i=0;
    while i<n {
        println!("{:#?}", array_person[i]);
        i=i+1
    }
}
