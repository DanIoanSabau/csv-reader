extern crate csv;

fn read_csv_file(path: &str) -> Result<Vec<csv::StringRecord>, Box<dyn std::error::Error>> {
    let mut csv_reader = csv::Reader::from_path(path)?;
    let mut records = Vec::new();
    for result in csv_reader.records() {
        records.push(result?);
    }
    Ok(records)
}

fn main() {
    let program_arguments = std::env::args().collect::<Vec<String>>();
    if 2 != program_arguments.len() {
        eprintln!("Usage: {} <csv-filename>", &program_arguments[0]);
        return;
    }


    let input_file_name = &program_arguments[1];
    match read_csv_file(&input_file_name) {
        Ok(file_lines) => {
            println!("File content:");
            for line in file_lines.iter() {
                println!("{:?}", line);
            }
        }
        Err(error) => {
            eprintln!("Error occurred while reading \"{}\". Program exited with error: {}", input_file_name, error);
        }
    }
}
