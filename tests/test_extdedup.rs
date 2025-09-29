use std::env::temp_dir;

use newline_converter::dos2unix;

use crate::workdir::Workdir;

#[test]
fn extdedup_linemode() {
    let wrk = Workdir::new("extdedup_linemode").flexible(true);
    wrk.clear_contents().unwrap();

    let test_file = wrk.load_test_file("boston311-100-20dupes-random.csv");

    let mut cmd = wrk.command("extdedup");
    cmd.arg(test_file).arg("boston311-100-extdeduped.csv");
    wrk.output(&mut cmd);

    // load deduped output
    let deduped_output: String = wrk.from_str(&wrk.path("boston311-100-extdeduped.csv"));

    let expected_csv = wrk.load_test_resource("boston311-100-deduped.csv");
    wrk.create_from_string("boston311-100-deduped.csv", &expected_csv);

    assert_eq!(dos2unix(&deduped_output), dos2unix(&expected_csv));
}

#[test]
fn extdedup_linemode_dupesoutput() {
    let wrk = Workdir::new("extdedup-dupes-output").flexible(true);
    wrk.clear_contents().unwrap();

    let test_file = wrk.load_test_file("boston311-100-20dupes-random.csv");

    let mut cmd = wrk.command("extdedup");
    cmd.arg(test_file)
        .arg("boston311-100-extdeduped.csv")
        .args([
            "--dupes-output",
            "boston311-100-extdededuped-dupeoutput.txt",
        ]);
    wrk.output(&mut cmd);

    // load deduped output
    let deduped_output: String = wrk.from_str(&wrk.path("boston311-100-extdeduped.csv"));

    let expected_csv = wrk.load_test_resource("boston311-100-deduped.csv");
    wrk.create_from_string("boston311-100-deduped.csv", &expected_csv);

    assert_eq!(dos2unix(&deduped_output), dos2unix(&expected_csv));

    // load dupe-output txt
    let dupes_output: String = wrk.from_str(&wrk.path("boston311-100-extdededuped-dupeoutput.txt"));

    let expected_output = wrk.load_test_resource("boston311-extdedup-dupeoutput.txt");
    wrk.create_from_string("boston311-extdedup-dupeoutput.txt", &expected_output);

    assert_eq!(dos2unix(&dupes_output), dos2unix(&expected_output));
}

#[test]
fn extdedupe_csvmode() {
    let wrk = Workdir::new("extdedup-csvmode").flexible(true);
    wrk.clear_contents().unwrap();

    let test_file = wrk.load_test_file("boston311-100-20dupes-random.csv");

    let mut cmd = wrk.command("extdedup");
    cmd.arg(test_file)
        .arg("boston311-100-extdeduped.csv")
        .args(["--select", "case_enquiry_id,open_dt,target_dt"]);
    wrk.output(&mut cmd);

    // load deduped output
    let deduped_output: String = wrk.from_str(&wrk.path("boston311-100-extdeduped.csv"));

    let expected_csv = wrk.load_test_resource("boston311-100-deduped.csv");
    wrk.create_from_string("boston311-100-deduped.csv", &expected_csv);

    assert_eq!(dos2unix(&deduped_output), dos2unix(&expected_csv));

    // Check that the correct number of rows were deduplicated
    let output = wrk.output(&mut cmd);

    // 20 duplicates should be removed
    assert!(String::from_utf8_lossy(&output.stderr).contains("20\n"));
}

#[test]
fn extdedupe_csvmode_dupesoutput() {
    let wrk = Workdir::new("extdedup-csvmode-dupesoutput").flexible(true);
    wrk.clear_contents().unwrap();

    let test_file = wrk.load_test_file("boston311-100-20dupes-random.csv");

    let mut cmd = wrk.command("extdedup");
    cmd.arg(test_file)
        .arg("boston311-100-extdeduped.csv")
        .args([
            "--select",
            "case_enquiry_id,open_dt,target_dt",
            "--dupes-output",
            "boston311-100-extdededuped-dupeoutput.csv",
        ]);
    wrk.output(&mut cmd);

    // load deduped output
    let deduped_output: String = wrk.from_str(&wrk.path("boston311-100-extdeduped.csv"));

    let expected_csv = wrk.load_test_resource("boston311-100-deduped.csv");
    wrk.create_from_string("boston311-100-deduped.csv", &expected_csv);

    assert_eq!(dos2unix(&deduped_output), dos2unix(&expected_csv));

    // load dupe-output txt
    let dupes_output: String = wrk.from_str(&wrk.path("boston311-100-extdededuped-dupeoutput.csv"));

    let expected_output = wrk.load_test_resource("boston311-extdedup-dupeoutput.csv");
    wrk.create_from_string("boston311-extdedup-dupeoutput.csv", &expected_output);

    assert_eq!(dos2unix(&dupes_output), dos2unix(&expected_output));

    // Check that the correct number of rows were deduplicated
    let output = wrk.output(&mut cmd);
    // 20 duplicates should be removed
    assert!(String::from_utf8_lossy(&output.stderr).contains("20\n"));
}

#[test]
fn extdedupe_csvmode_neighborhood() {
    let wrk = Workdir::new("extdedup-csvmode-neighborhood").flexible(true);
    wrk.clear_contents().unwrap();

    let test_file = wrk.load_test_file("boston311-100-20dupes-random.csv");

    let mut cmd = wrk.command("extdedup");
    cmd.arg(test_file)
        .arg("boston311-100-extdeduped.csv")
        .args(["--select", "neighborhood"]);
    wrk.output(&mut cmd);

    // load deduped output
    let deduped_output: String = wrk.from_str(&wrk.path("boston311-100-extdeduped.csv"));

    let expected_csv = wrk.load_test_resource("boston311-extdedup-neighborhood.csv");
    wrk.create_from_string("boston311-extdedup-neighborhood.csv", &expected_csv);

    assert_eq!(dos2unix(&deduped_output), dos2unix(&expected_csv));

    // Check that the correct number of rows were deduplicated
    let output = wrk.output(&mut cmd);

    // 81 duplicates should be removed
    assert!(String::from_utf8_lossy(&output.stderr).contains("81\n"));
}

#[test]
fn extdedup_large_memory_test() {
    let wrk = Workdir::new("extdedup_large_memory").flexible(true);
    wrk.clear_contents().unwrap();

    // Generate a large CSV file with many duplicates
    // This test creates a file that should exceed typical memory limits
    // when processed with a very low memory limit
    let large_csv_path = generate_large_csv_with_duplicates(10_000_000);

    // Copy the generated file to the workdir
    use std::fs;
    fs::copy(&large_csv_path, wrk.path("large_test.csv")).expect("Failed to copy large CSV");

    // Clean up the temp file
    fs::remove_file(&large_csv_path).expect("Failed to remove temp file");

    // Test with very low memory limit to force disk usage
    // Use 1% of system memory - this should force disk usage
    // since hash table for 10M unique entries needs ~1GB
    let mut cmd = wrk.command("extdedup");
    cmd.arg("large_test.csv")
        .arg("large_test_deduped.csv")
        .args(["--memory-limit", "1"]); // 1% of system memory
    let output = wrk.output(&mut cmd);

    // Verify the command completed successfully
    assert!(output.status.success());

    // Load and verify the deduped output
    let deduped_output: String = wrk.from_str(&wrk.path("large_test_deduped.csv"));
    let lines: Vec<&str> = deduped_output.lines().collect();

    // Should have header + 5,000,000 unique rows (since we generated 50% duplicates)
    assert_eq!(lines.len(), 5000001); // 1 header + 5,000,000 unique rows

    // Verify that duplicates were actually removed
    let stderr_output = String::from_utf8_lossy(&output.stderr);
    assert!(stderr_output.contains("5000000")); // Should report 5,000,000 duplicates removed

    // Verify the output contains the expected unique rows
    assert!(deduped_output.contains("row_0"));
    assert!(deduped_output.contains("row_4999999"));
    // Should not contain any duplicate markers
    assert!(!deduped_output.contains("duplicate"));
}

fn generate_large_csv_with_duplicates(total_rows: usize) -> String {
    use std::{
        fs::File,
        io::{BufWriter, Write},
    };

    let temp_path = temp_dir()
        .join(format!("qsv_test_large_{}.csv", std::process::id()))
        .to_string_lossy()
        .into_owned();
    let file = File::create(&temp_path).expect("Failed to create temp file");
    let mut writer = BufWriter::with_capacity(64 * 1024, file); // 64KB buffer

    // Write header
    writer
        .write_all(b"id,name,value,category\n")
        .expect("Failed to write header");

    let unique_rows = total_rows / 2; // 50% unique, 50% duplicates
    let duplicate_rows = total_rows - unique_rows;

    // Generate unique rows
    for i in 0..unique_rows {
        let line = format!("{},\"row_{}\",{},category_{}\n", i, i, i * 10, i % 10);
        writer
            .write_all(line.as_bytes())
            .expect("Failed to write unique row");
    }

    // Generate duplicate rows (repeat some of the unique rows)
    for i in 0..duplicate_rows {
        let original_index = i % unique_rows;
        let line = format!(
            "{},\"row_{}\",{},category_{}\n",
            original_index,
            original_index,
            original_index * 10,
            original_index % 10
        );
        writer
            .write_all(line.as_bytes())
            .expect("Failed to write duplicate row");
    }

    writer.flush().expect("Failed to flush writer");
    temp_path
}
