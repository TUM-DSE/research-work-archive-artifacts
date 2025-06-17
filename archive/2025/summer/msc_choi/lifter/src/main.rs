use elf::abi::STT_FUNC;
use elf::endian::AnyEndian;
use elf::ElfBytes;
use lifter::arm64::AArch64Lifter;
use lifter::Lifter;
use serde::Deserialize;
use std::fs;
use std::fs::{File, OpenOptions};
use std::io::Write;
use std::path::{Path, PathBuf};
use std::sync::{Arc, Mutex};
use std::time::{Duration, Instant};
use rayon::prelude::*;

fn main() {
    // benchmark_instructions();
    benchmark_sightglass();
}

fn benchmark_instructions() {
    println!("Running instruction benchmarks");

    let base_dir = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    let subdir_path = base_dir.join("tests/lifter/insts/tests/");
    let output_csv_path = base_dir.join("results/benchmark_instructions.csv");

    let results_dir = base_dir.join("results");
    fs::create_dir_all(&results_dir).expect("Failed to create results directory");

    let mut csv_file = File::create(&output_csv_path).expect("Failed to create CSV file");

    writeln!(csv_file, "opcode,lifting_time,instruction_count,block_count").expect("Failed to write header");

    for entry in fs::read_dir(subdir_path).expect("Failed to read directory") {
        let entry = entry.expect("Failed to read entry");
        let path = entry.path();

        if path.is_file() {
            println!("\tProcessing {:?}", path);

            match read_yaml_file(&path, 5) {
                Ok((opcode, time, instrs, blocks)) => {
                    writeln!(csv_file, "{},{},{},{}", opcode, time, instrs, blocks).expect("Failed to write row");
                }
                Err(e) => panic!("\t\tError: {:?}", e),
            }
        }
    }

    println!("\nResults written to {:?}", output_csv_path);
}

#[derive(Deserialize)]
struct TestFile {
    tests: Vec<TestSpec>,
}

#[derive(Deserialize, Clone)]
struct TestSpec {
    name: String,
    bytes: Vec<u8>,
    proofs: Option<Vec<u8>>,
}

fn read_yaml_file(path: impl AsRef<Path>, runs: usize) -> Result<(String, f64, f64, f64), Box<dyn std::error::Error>> {
    let yaml_str = fs::read_to_string(&path).unwrap();
    let test_file: TestFile = serde_yaml::from_str(&yaml_str).unwrap();
    let file_stem = path.as_ref().file_stem().unwrap().to_str().unwrap();

    let mut execution_times = Vec::new();
    let mut inst_counts = Vec::new();
    let mut block_counts = Vec::new();

    for _ in 0..runs {
        for test in test_file.tests.clone() {
            let test_name = &test.name;
            let base_dir = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
            let file = base_dir.join(format!("tests/lifter/insts/tests/{}.yaml", file_stem));

            let yaml_str = fs::read_to_string(&file).expect("Cannot read YAML file");
            let mut test_file: TestFile = serde_yaml::from_str(&yaml_str).expect("Invalid YAML");

            let test = test_file
                .tests
                .iter_mut()
                .find(|t| t.name == *test_name)
                .unwrap_or_else(|| panic!("Test '{}' not found in '{:?}'", test_name, file));

            let bytes = &test.bytes;
            let proofs = test.proofs.as_ref().map(|bytes| &bytes[..]);

            let lifter = AArch64Lifter;
            let start = Instant::now();
            let code_region = lifter.lift(bytes, proofs.unwrap_or(&[]), false).unwrap();
            let duration = start.elapsed();

            let _result = code_region.display().to_string();
            let block_count = code_region.blocks().count();
            let inst_count = code_region.blocks().fold(0, |acc, b| acc + b.inst_count());

            execution_times.push(duration);
            inst_counts.push(inst_count);
            block_counts.push(block_count);
        }
    }

    let total_duration = execution_times.iter().sum::<Duration>();
    let average_duration = total_duration.as_nanos() as f64 / execution_times.len() as f64;
    let average_insts = inst_counts.iter().sum::<usize>() as f64 / inst_counts.len() as f64;
    let average_blocks = block_counts.iter().sum::<usize>() as f64 / block_counts.len() as f64;

    println!(
        "\t\tExecution time (ns): {:.2?}, Blocks: {:.2?}, Instructions: {:.2?}",
        average_duration, average_blocks, average_insts
    );

    Ok((file_stem.to_string(), average_duration, average_insts, average_blocks))
}

fn benchmark_sightglass() {
    println!("Running sightglass benchmarks");

    let base_dir = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    let subdir_path = base_dir.join("tests/bin");
    let results_dir = base_dir.join("results");
    fs::create_dir_all(&results_dir).expect("Failed to create results directory");

    let output_csv_path = base_dir.join("results/benchmark_sightglass.csv");
    let file = OpenOptions::new()
        .write(true)
        .truncate(true)
        .create(true)
        .open(&output_csv_path)
        .expect("Failed to open results file");

    let writer = Arc::new(Mutex::new(file));
    let entries: Vec<_> = fs::read_dir(&subdir_path)
        .expect("Failed to read directory")
        .filter_map(Result::ok)
        .map(|e| e.path())
        .filter(|p| p.is_file())
        .collect();
    // let entries: Vec<PathBuf> = vec![subdir_path.join("shootout-keccak.cwasm")];

    writeln!(writer.lock().unwrap(), "filename,lifting_time_ns").unwrap();

    entries.par_iter().for_each(|path| {
        let duration = read_elf_file(path).unwrap();
        writeln!(
            writer.lock().unwrap(),
            "{},{}",
            path.file_name().unwrap().to_string_lossy(),
            duration.as_nanos()
        ).unwrap();
    });

    println!("\nResults written to {:?}", output_csv_path);
}

fn read_elf_file(
    path: &Path,
) -> Result<Duration, Box<dyn std::error::Error + Send + Sync>> {
    let file_data = fs::read(path)?;
    let slice = file_data.as_slice();
    let file = ElfBytes::<AnyEndian>::minimal_parse(slice)?;

    let text_shdr = file
        .section_header_by_name(".text")?
        .ok_or("Missing .text section")?;
    let (bytes, _) = file.section_data(&text_shdr)?;
    let (string_table, _) = file.symbol_table()?.ok_or("Missing symbol table")?;
    let section_addr = text_shdr.sh_addr;

    let mut total_duration = Duration::ZERO;

    for s in string_table.iter().filter(|s| s.st_symtype() == STT_FUNC) {
        let offset: usize = s
            .st_value
            .checked_sub(section_addr)
            .ok_or("Symbol address before section start")?
            .try_into()
            .map_err(|_| "Offset too large")?;
        let size: usize = s.st_size.try_into().map_err(|_| "Symbol size too large")?;
        if offset % 4 != 0 || size % 4 != 0 {
            continue;
        }
        let end = offset.checked_add(size).ok_or("Symbol range overflow")?;
        if end > bytes.len() {
            continue;
        }

        let lifter = AArch64Lifter;
        let start = Instant::now();
        let panic = std::panic::catch_unwind(|| match lifter.lift(&bytes[offset..end], &[], false) {
            Ok(_) => Ok(()),
            Err(e) => Err(e),
        });

        if panic.is_err() {
            continue;
        }

        total_duration += start.elapsed();
    }

    Ok(total_duration)
}
