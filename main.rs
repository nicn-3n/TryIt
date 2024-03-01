struct Person {
    id:u32,
    name:String,
    gender:String
} 

fn main() {

    let person_1 = Person {
        id:1,
        name:"Paul Abbas".to_string(),
        gender:"xmale".to_string()
    };

    let gender_message = if person_1.gender.eq("male") {
        "I am a boy"
    } else if person_1.gender.eq("female") {
        "I am a girl"
    } else {
        "gender:unknown"
    };


    println!("Hello world I am {}, {}!", person_1.name, gender_message);
}
