use permutohedron::LexicalPermutation;
use std::fs;
use std::process::Command;

fn main() {
    let mut visitors: Vec<&str> = vec![
        "FunctionInlineVisitor",
        "QuantifierVisitor",
    ];

    loop {
        let file: &str = "examples/recur.rs";
        let output_file = format!(
            "../tempFiles/sample_formatted_{}_0.rs",
            visitors.last().unwrap()
        );
        let cmd: String = format!(
            "(cd ../ && cargo run -- --visitors {} {})",
            visitors.join(","),
            file
        );
        let temp_files_dir = "../tempFiles";

        if fs::metadata(temp_files_dir).is_ok() {
            fs::remove_dir_all(temp_files_dir).unwrap();
        }

	fs::create_dir(temp_files_dir).unwrap();
	

        let status = Command::new("sh")
            .arg("-c")
            .arg(cmd)
            .status()
            .expect("Failed to execute command");

        if !status.success() {
            eprintln!("Error running command for permutation: {:?}", visitors);
            continue;
        }

        let experiment_dir = format!("../experiment/{}", visitors.join(", "));
        fs::create_dir_all(&experiment_dir).unwrap();

        let files = fs::read_dir(temp_files_dir)
            .unwrap()
            .filter_map(|entry| entry.ok())
            .collect::<Vec<_>>();

        for (_, file) in files.iter().enumerate() {
            let file_name = file.file_name().to_str().unwrap().to_string();
            fs::rename(file.path(), format!("{}/{}", experiment_dir, file_name)).unwrap();
        }

        if !visitors.next_permutation() {
            break;
        }
    }
    println!("done!");
}
