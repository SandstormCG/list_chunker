use std::{
    env,
    fs::File,
    io::{prelude::*, BufReader},
    path::{Path, PathBuf},
    fs
};

fn usage() {
    println!("List chunker");
    println!("list_chunker add add_file values_file");
    println!("add_file: file with one value per line to be added into values_file");
    println!("values_file: file with unique values");
}



fn lines_from_file(filename: impl AsRef<Path>) -> Vec<String> {
    let file = File::open(filename).expect("no such file");
    let buf = BufReader::new(file);
    buf.lines()
        .map(|l| l.expect("Could not parse line"))
        .collect()
}

fn save_to_file(file_path: &PathBuf, values: &Vec<String>) {
    let mut file = File::create(file_path).expect("no such file");
    file.write_all(values.join("\n").as_bytes())
                        .expect("Error while writing to file");
}

fn save_chunks(values: &Vec<String>){
    let mut chunk_path = PathBuf::new();
    chunk_path.push(&"chunks");
    fs::create_dir(chunk_path.clone()).expect("Error creating chunks directory");
    for (i, chunk) in values.chunks(20).enumerate() {
        let mut chunk_path_item = chunk_path.clone();
        chunk_path_item.push(i.to_string());
        save_to_file(&chunk_path_item, &chunk.to_vec());
        //println!("{:?}", chunk);
    }
}

fn add_unique_lines (args: Vec<String>) {
    let add_file = PathBuf::from(&args[2]);
    let values_file = PathBuf::from(&args[3]);

    let add_lines = lines_from_file(add_file);
    let mut value_lines = lines_from_file(values_file.clone());

    for add_line in add_lines.iter() {
        if !value_lines.contains(add_line){
            value_lines.push(add_line.clone());
        }
    }


    value_lines.sort();
    save_to_file(&values_file,&value_lines);

    save_chunks(&value_lines)


    //println!("{:?}", value_lines);

    //exec(args[3].clone(), args[4].clone());

}

fn main() {
    let args: Vec<String> = env::args().collect();

    if args[1].contains("add") {
        add_unique_lines(args);
    } else if args[1].contains("-h") {
        usage();
    } else {
        println!("Invalid arguments");
        usage();
    }
}

