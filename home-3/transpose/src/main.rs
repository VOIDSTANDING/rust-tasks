fn main() {
    let args: Vec<String> = std::env::args().collect();

    if args.len() != 2 {
        println!("USAGE: {} <path_to_file>", args[0]);
        return;
    }

    // let progname = &args[0];
    // let filename = &args[1];

    let content = std::fs::read_to_string(&args[1]).expect("Failed to read the file.");

    let lines: Vec<&str> = content.trim().split('\n').collect();

    let mut rows: Vec<Vec<&str>> = Vec::new();

    for i in lines {
        let row: Vec<&str> = i.split_whitespace().collect();
        println!("{:?}", &row);
        rows.push(row);
    }

    println!("{:?}", &rows);
    
    let mut transposed: Vec<Vec<&str>> = vec![Vec::new(); rows[0].len()];
    
    for i in 0..rows.len() {
        for j in 0..rows[0].len() {
            transposed[j].push(rows[i][j]);
        }
    }

    println!("{:?}", &transposed);
    
    for r in transposed {
        println!("{}", r.join(" "));
    }
}
