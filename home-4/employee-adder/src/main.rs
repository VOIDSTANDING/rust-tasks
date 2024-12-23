fn main() {
    let mut company: std::collections::HashMap<String, Vec<String>> = std::collections::HashMap::new();

    loop {
        println!("\n> add <name> to <department>");
        println!("> ls <department>");
        println!("> ls a");
        println!("> exit\n");
        println!("Your choice?");

        let mut command = String::new();
        std::io::stdin().read_line(&mut command).unwrap();
        let command = command.trim();

        match command.split_whitespace().collect::<Vec<&str>>().as_slice() {

            ["add", name, "to", department] => {
                
                if department.contains("a") {
                    println!("'a' is forbidden.");
                }

                else {
                    company.entry(department.to_string()).or_default().push(name.to_string());
                }
            }

            ["ls", "a"] => {
                let mut all_departments: Vec<_> = company.keys().collect();
                all_departments.sort();
                for department in all_departments {
                    println!("{}: {:?}", department, company[department]);
                }
            }

            ["ls", department] => {
                if let Some(staff) = company.get(*department) {
                    println!("{:?}", staff);
                } 
                
                else {
                    println!("No such department.");
                }
            }

            ["exit"] => break,

            _ => println!("Bad command."),
        }
    }
}
