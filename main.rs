#[derive(Debug)]
enum ErrorType {
    NotAvailable(String,),
    PostionError{x:u32, y:u32, z:u32} ,
    UnknownError,
}

fn main() {

    let error_list = vec![
        ErrorType::UnknownError,
        ErrorType::PostionError{x:15, y:48, z:89},
        ErrorType::UnknownError,
        ErrorType::NotAvailable(String::from("url not available")),
        ErrorType::UnknownError,
        ErrorType::NotAvailable(String::from("resource already on use"))  
    ];

    for error in &error_list{
        match error {
            ErrorType::NotAvailable(data) => println!("{}",data),
            ErrorType::PostionError{x,y,z} => println!("Error occured at position x={} y={} z={} ",x,y,z),
            ErrorType::UnknownError => println!("Unknown Error")
        }
    }
}
