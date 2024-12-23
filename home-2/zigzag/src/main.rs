fn main() {
    let args: Vec<String> = std::env::args().collect();

    if args.len() != 3 {
        println!("USAGE: {} <string> <num_rows>", args[0]);
        return;
    }

    let input_string = &args[1];
    let num_rows: usize = args[2].parse().expect("Bad argument");

    let result = convert(input_string, num_rows);

    println!("{}", result);
}


fn convert(s: &str, num_rows: usize) -> String {
    if num_rows == 1 || num_rows >= s.len() {
        return s.to_string();
    }

    let mut rows = vec![String::new(); num_rows];
    let mut current_row = 0;
    let mut step = 1;

    for char in s.chars() {
        rows[current_row].push(char);

        if current_row == 0 {
            step = 1;
        } 
        else if current_row == num_rows - 1 {
            step = -1;
        }
        current_row = (current_row as isize + step) as usize;
    }

    rows.concat()
}
