use anyhow::{Context, Result};
use permutohedron::LexicalPermutation;
use std::fs;
use std::path::Path;
use std::process::Command;
use uuid::Uuid;

fn run_permutation(path: &Path, ordering: &Vec<&str>) -> Result<()> {
    let original_file_name = path.file_name().unwrap().to_str().unwrap();

    let temp_files_dir = "../tempFiles";

    // 1. Copy the file to a random path so the output files don't collide
    let rand_name = format!("{}.rs", Uuid::new_v4());
    let rand_path = Path::new(&rand_name);
    fs::copy(path, rand_path).with_context(|| format!("Failed to copy file from {:?}", path))?;

    let output_file = format!(
        "{}/{}_formatted_{}_0.rs",
        temp_files_dir,
        rand_name.split('.').next().unwrap(),
        ordering.last().unwrap()
    );

    let cmd: String = format!(
        "(cd ../ && cargo run -- --visitors {} vpermute/{})",
        ordering.join(","),
        rand_path.display()
    );

    let status = Command::new("sh")
        .arg("-c")
        .arg(cmd.clone())
        .status()
        .with_context(|| format!("Failed to execute command: {}", cmd))?;

    if !status.success() {
        anyhow::bail!("Failed to run command for permutation {:?}", ordering);
    }

    let experiment_dir = format!("../experiment/{}", original_file_name);
    fs::create_dir_all(&experiment_dir)
        .with_context(|| format!("Failed to create experiment directory: {}", &experiment_dir))?;

    let output_path = format!("{}/{}.rs", experiment_dir, ordering.join(","));

    fs::copy(&output_file, &output_path)
        .with_context(|| format!("Failed to copy output file to {}", output_path))?;

    fs::remove_file(rand_path)
        .with_context(|| format!("Failed to delete uuid file {}", rand_name))?;

    Ok(())
}

fn main() {
    //parameters
    let mut visitors: Vec<&str> = vec!["FunctionInlineVisitor", "QuantifierVisitor"];
    let file_path: &str = "../examples/recur.rs";

    let path = Path::new(file_path);

    loop {
        run_permutation(path, &visitors).unwrap();
        if !visitors.next_permutation() {
            break;
        }
    }
    println!("done!");
}
