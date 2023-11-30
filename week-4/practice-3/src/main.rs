fn main() {
    let name1 = "Kiibati";
    println!("My name is {}",name1);

    //find and replace
    let name2 = name1.replace("Kiibati","kiibz");
    println!("You can also call me {}", name2);
    let faculty = "Faculty of Science and Technology";

    //find and replace
    let school = faculty.replace("Faculty", "School");
    println!("I am a student of the {}",school);

}