use anyhow::{Context, Result};
use clap::Parser;
use itertools::Itertools; // Import itertools for permutations
use rayon::prelude::*;
use std::fs;
use std::path::Path;
use std::process::Command;
use uuid::Uuid;

/// Command-line arguments structure
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// File path to process
    #[arg(short, long)]
    file: String,

    /// Comma-separated list of visitors
    #[arg(short, long)]
    visitors: String,

    /// Experiment name
    #[arg(short = 'n', long)]
    experiment_name: String,

    /// Enable debug printing
    #[arg(long)]
    debug: bool,
}

fn run_permutation(
    path: &Path,
    ordering: &Vec<&str>,
    experiment_name: &str,
    debug: bool,
) -> Result<()> {
    if debug {
        println!(
            "DEBUG: Starting permutation for {:?} in experiment '{}'",
            ordering, experiment_name
        );
    }

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

    let experiment_dir = format!("../permutation_experiments/{}/{}", experiment_name, original_file_name);
    fs::create_dir_all(&experiment_dir)
        .with_context(|| format!("Failed to create experiment directory: {}", &experiment_dir))?;

    let output_path = format!("{}/{}.rs", experiment_dir, ordering.join(","));
    fs::copy(&output_file, &output_path)
        .with_context(|| format!("Failed to copy output file to {}", output_path))?;

    fs::remove_file(rand_path)
        .with_context(|| format!("Failed to delete uuid file {}", rand_name))?;

    if debug {
        println!(
            "DEBUG: Successfully processed permutation {:?} for experiment '{}'",
            ordering, experiment_name
        );
    }
    Ok(())
}

fn main() {
    // Parse command-line arguments
    let args = Args::parse();

    // Extract file path, visitors, and experiment name
    let file_path = args.file;
    let visitors: Vec<&str> = args.visitors.split(',').collect();
    let experiment_name = args.experiment_name;
    let debug = args.debug; // Get the debug flag

    let path = Path::new(&file_path);

    // Generate permutations using itertools
    let perms: Vec<Vec<&str>> = visitors.clone().into_iter().permutations(visitors.len()).collect();

    if debug {
        println!(
            "DEBUG: Total permutations generated for experiment '{}': {}",
            experiment_name,
            perms.len()
        );
        for perm in &perms {
            println!("DEBUG: Visitors for this permutation: {:?}", perm);
        }
    }

    // Run permutations in parallel
    perms.par_iter().for_each(|perm| {
        if let Err(e) = run_permutation(path, perm, &experiment_name, debug) {
            eprintln!("Error with permutation {:?} for experiment '{}': {}", perm, experiment_name, e);
        }
    });

    println!("Experiment '{}' done!", experiment_name);
}
