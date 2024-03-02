enum Gender {
    FEMALE,
    MALE
}

struct Person {
    id:u32,
    name:String,
    gender:Gender
} 

fn main() {

    let person_1 = Person {
        id:1,
        name:"Paul Abbas".to_string(),
        gender: Gender::FEMALE
    };

    let gender_message = match person_1.gender {
        Gender::MALE => "I am a boy",
        Gender::FEMALE => "I am a girl"
    };


    println!("Hello world I am {}, {}!", person_1.name, gender_message);
}
