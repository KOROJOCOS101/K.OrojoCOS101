use std::io;

fn main() {
    
    let public_servant = vec!["Office Administrator", "Academic", "Lawyer", "Teacher"];
    let aps1_2 = vec!["Intern", "Paralegal", "Placement"];
    let aps3_5 = vec!["Administrator", "Research Assistant", "Junior Associate", "Classroom Teacher"];
    let aps5_8 = vec!["Senior Administrator", "PHD Candidate", "Associate", "Senior Teacher"];
    let el8_10 = vec!["Office Manager", "Post-Doc Researcher", "Senior Associate 1-2", "Leading Teacher"];
    let el10_13 = vec!["Director", "Senior Lecturer", "Senior Associate 3-4", "Deputy Principal"];
    let ses = vec!["CEO", "Dean", "Partner", "Principal"];

    loop {
        
        let mut name = String::new();
        println!("Enter name:");
        io::stdin().read_line(&mut name).expect("Failed to read line");

        let mut title = String::new();
        println!("Enter job title:");
        io::stdin().read_line(&mut title).expect("Failed to read line");

        let mut exp = String::new();
        println!("Enter years of experience:");
        io::stdin().read_line(&mut exp).expect("Failed to read line");

        let experience: u32 = match exp.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Invalid");
                0
            }
        };

        let position = match title.as_str() {
        
            title if public_servant.contains(&title) => true,
        
            title if aps1_2.contains(&title) && experience >= 1 && experience <= 2 => true,
        
            title if aps3_5.contains(&title) && experience >= 3 && experience <= 5 => true,
        
            title if aps5_8.contains(&title) && experience >= 5 && experience <= 8 => true,

            title if el8_10.contains(&title) && experience >= 8 && experience <= 10 => true,
            
            title if el10_13.contains(&title) && experience >= 10 && experience <= 13 => true,
            
            title if ses.contains(&title) => true,
            
            _ => false,
        };
        


        if position {
            println!(
                "{} is a valid staff position for {} with {} years of experience.",
                title, name, experience
            );
        } else {
            println!("Invalid staff position or experience for {}.", name);
        }

        println!("Do you want to enter another staff member? (yes/no)");
        let mut answer = String::new();
        io::stdin().read_line(&mut answer).expect("Failed to read line");

        if answer.trim() != "yes" {
            break;
        }
    }
}