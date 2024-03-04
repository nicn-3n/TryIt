fn main() {

    let mut list_of_element : Vec<u32> = Vec::new(); 
    println!("Empty list :{:#?}",list_of_element);
    list_of_element.push(4);
    list_of_element.push(74);
    list_of_element.push(47);
    println!("List :{:?}",list_of_element);
    let x = list_of_element.pop();
    println!("Removed element {:?} ",x);
    println!("Remaining List :{:?}",list_of_element);

}

