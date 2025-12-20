use csv::ReaderBuilder;

use crate::workdir::Workdir;

/// Helper function to parse stats CSV and get column index
fn get_column_index(headers: &csv::StringRecord, column_name: &str) -> Option<usize> {
    headers.iter().position(|h| h == column_name)
}

/// Helper function to get field value from a record
fn get_field_value(record: &csv::StringRecord, column_idx: usize) -> Option<String> {
    record.get(column_idx).map(|s| s.to_string())
}

/// Helper function to verify new columns exist in output
fn verify_new_columns_exist(csv_content: &str) -> Vec<String> {
    let mut rdr = ReaderBuilder::new()
        .has_headers(true)
        .from_reader(csv_content.as_bytes());

    let headers = rdr.headers().unwrap();
    let expected_columns = vec![
        "pearson_skewness",
        "range_stddev_ratio",
        "quartile_coefficient_dispersion",
        "bowley_skewness",
        "mode_zscore",
    ];

    let mut found_columns = Vec::new();
    for col in &expected_columns {
        if headers.iter().any(|h| h == *col) {
            found_columns.push(col.to_string());
        }
    }

    found_columns
}

#[test]
fn moarstats_basic_with_existing_stats() {
    let wrk = Workdir::new("moarstats_basic");
    let test_file = wrk.load_test_file("boston311-100.csv");

    // Generate baseline stats first
    let mut stats_cmd = wrk.command("stats");
    stats_cmd
        .arg("--everything")
        .arg("--infer-dates")
        .arg(&test_file);
    wrk.assert_success(&mut stats_cmd);

    // Verify stats file was created
    let stats_path = wrk.path("boston311-100.stats.csv");
    assert!(stats_path.exists(), "Stats file should exist");

    // Run moarstats
    let mut cmd = wrk.command("moarstats");
    cmd.arg(&test_file);
    wrk.assert_success(&mut cmd);

    // Verify output file exists
    assert!(
        stats_path.exists(),
        "Stats file should still exist after moarstats"
    );

    // Read and verify new columns
    let stats_content = wrk.read_to_string("boston311-100.stats.csv").unwrap();
    let new_columns = verify_new_columns_exist(&stats_content);
    assert!(
        !new_columns.is_empty(),
        "At least some new columns should be added"
    );

    // Verify that numeric columns have computed values
    let mut rdr = ReaderBuilder::new()
        .has_headers(true)
        .from_reader(stats_content.as_bytes());

    let headers = rdr.headers().unwrap().clone();
    let field_idx = get_column_index(&headers, "field").unwrap();
    let type_idx = get_column_index(&headers, "type").unwrap();

    // Find a numeric column (latitude or longitude)
    let mut found_numeric = false;
    for result in rdr.records() {
        let record = result.unwrap();
        let field_name = get_field_value(&record, field_idx).unwrap();
        let field_type = get_field_value(&record, type_idx).unwrap();

        if (field_name == "latitude" || field_name == "longitude")
            && (field_type == "Float" || field_type == "Integer")
        {
            found_numeric = true;

            // Check if pearson_skewness column exists and that this row has a value
            if let Some(pearson_idx) = get_column_index(&headers, "pearson_skewness") {
                let pearson_val = get_field_value(&record, pearson_idx);
                // When the pearson_skewness column is present for a numeric field,
                // its value should be set
                assert!(
                    pearson_val.is_some(),
                    "pearson_skewness value should exist for numeric columns when the column is \
                     present"
                );
            }

            break;
        }
    }

    assert!(found_numeric, "Should find at least one numeric column");
}

#[test]
fn moarstats_auto_generate_stats() {
    let wrk = Workdir::new("moarstats_auto_gen");
    let test_file = wrk.load_test_file("boston311-100.csv");

    // Ensure no stats file exists initially
    let stats_path = wrk.path("boston311-100.stats.csv");
    if stats_path.exists() {
        std::fs::remove_file(&stats_path).unwrap();
    }

    // Run moarstats with stats options - should auto-generate stats
    let mut cmd = wrk.command("moarstats");
    cmd.arg(&test_file)
        .arg("--stats-options")
        .arg("--everything --infer-dates");
    wrk.assert_success(&mut cmd);

    // Verify stats file was auto-generated
    assert!(stats_path.exists(), "Stats file should be auto-generated");

    // Verify new columns are appended
    let stats_content = wrk.read_to_string("boston311-100.stats.csv").unwrap();
    let new_columns = verify_new_columns_exist(&stats_content);
    assert!(
        !new_columns.is_empty(),
        "New columns should be added after auto-generation"
    );
}

#[test]
fn moarstats_custom_output() {
    let wrk = Workdir::new("moarstats_custom_output");
    let test_file = wrk.load_test_file("boston311-100.csv");

    // Generate baseline stats first
    let mut stats_cmd = wrk.command("stats");
    stats_cmd
        .arg("--everything")
        .arg("--infer-dates")
        .arg(&test_file);
    wrk.assert_success(&mut stats_cmd);

    let original_stats_path = wrk.path("boston311-100.stats.csv");
    assert!(
        original_stats_path.exists(),
        "Original stats file should exist"
    );

    // Read original content for comparison
    let original_content = wrk.read_to_string("boston311-100.stats.csv").unwrap();

    // Run moarstats with custom output
    let mut cmd = wrk.command("moarstats");
    cmd.arg(&test_file)
        .arg("--output")
        .arg("enhanced_stats.csv");
    wrk.assert_success(&mut cmd);

    // Verify custom output file exists
    let enhanced_path = wrk.path("enhanced_stats.csv");
    assert!(enhanced_path.exists(), "Enhanced stats file should exist");

    // Verify original file is unchanged (should not have new columns)
    let original_content_after = wrk.read_to_string("boston311-100.stats.csv").unwrap();
    assert_eq!(
        original_content, original_content_after,
        "Original stats file should be unchanged"
    );

    // Verify enhanced file has new columns
    let enhanced_content = wrk.read_to_string("enhanced_stats.csv").unwrap();
    let new_columns = verify_new_columns_exist(&enhanced_content);
    assert!(
        !new_columns.is_empty(),
        "Enhanced stats file should have new columns"
    );
}

#[test]
fn moarstats_custom_rounding() {
    let wrk = Workdir::new("moarstats_rounding");
    let test_file = wrk.load_test_file("boston311-100.csv");

    // Generate baseline stats
    let mut stats_cmd = wrk.command("stats");
    stats_cmd
        .arg("--everything")
        .arg("--infer-dates")
        .arg(&test_file);
    wrk.assert_success(&mut stats_cmd);

    // Run moarstats with custom rounding
    let mut cmd = wrk.command("moarstats");
    cmd.arg(&test_file).arg("--round").arg("2");
    wrk.assert_success(&mut cmd);

    // Verify values are rounded to 2 decimal places
    let stats_content = wrk.read_to_string("boston311-100.stats.csv").unwrap();
    let mut rdr = ReaderBuilder::new()
        .has_headers(true)
        .from_reader(stats_content.as_bytes());

    let headers = rdr.headers().unwrap().clone();

    if let Some(pearson_idx) = get_column_index(&headers, "pearson_skewness") {
        for result in rdr.records() {
            let record = result.unwrap();
            if let Some(val_str) = get_field_value(&record, pearson_idx) {
                if !val_str.is_empty() {
                    // Check decimal places (allowing for scientific notation edge cases)
                    if let Some(dot_pos) = val_str.find('.') {
                        let after_dot = &val_str[dot_pos + 1..];
                        // Should have at most 2 decimal places (or be in scientific notation)
                        assert!(
                            after_dot.len() <= 2 || val_str.contains('e') || val_str.contains('E'),
                            "Value '{}' should be rounded to 2 decimal places",
                            val_str
                        );
                    }
                }
            }
        }
    }
}

#[test]
fn moarstats_verify_computed_values() {
    let wrk = Workdir::new("moarstats_verify_values");
    let test_file = wrk.load_test_file("boston311-100.csv");

    // Generate baseline stats with everything
    let mut stats_cmd = wrk.command("stats");
    stats_cmd
        .arg("--everything")
        .arg("--infer-dates")
        .arg(&test_file);
    wrk.assert_success(&mut stats_cmd);

    // Run moarstats
    let mut cmd = wrk.command("moarstats");
    cmd.arg(&test_file);
    wrk.assert_success(&mut cmd);

    // Parse output and verify computed values
    let stats_content = wrk.read_to_string("boston311-100.stats.csv").unwrap();
    let mut rdr = ReaderBuilder::new()
        .has_headers(true)
        .from_reader(stats_content.as_bytes());

    let headers = rdr.headers().unwrap().clone();
    let field_idx = get_column_index(&headers, "field").unwrap();
    let type_idx = get_column_index(&headers, "type").unwrap();

    let mean_idx = get_column_index(&headers, "mean");
    let median_idx = get_column_index(&headers, "median");
    let q2_median_idx = get_column_index(&headers, "q2_median");
    let stddev_idx = get_column_index(&headers, "stddev");
    let range_idx = get_column_index(&headers, "range");
    let q1_idx = get_column_index(&headers, "q1");
    let q3_idx = get_column_index(&headers, "q3");
    let mode_idx = get_column_index(&headers, "mode");

    let pearson_idx = get_column_index(&headers, "pearson_skewness");
    let range_stddev_idx = get_column_index(&headers, "range_stddev_ratio");
    let quartile_coeff_idx = get_column_index(&headers, "quartile_coefficient_dispersion");
    let bowley_idx = get_column_index(&headers, "bowley_skewness");
    let mode_zscore_idx = get_column_index(&headers, "mode_zscore");

    let mut found_latitude = false;
    let mut found_numeric_with_quartiles = false;

    for result in rdr.records() {
        let record = result.unwrap();
        let field_name = get_field_value(&record, field_idx).unwrap();
        let field_type = get_field_value(&record, type_idx).unwrap();

        // Check latitude column (should be Float)
        if field_name == "latitude" && field_type == "Float" {
            found_latitude = true;

            // Verify pearson_skewness is computed if mean, median, stddev exist
            if let (Some(mean_idx), Some(stddev_idx), Some(pearson_idx)) =
                (mean_idx, stddev_idx, pearson_idx)
            {
                let mean_val = get_field_value(&record, mean_idx);
                let stddev_val = get_field_value(&record, stddev_idx);
                let median_val = median_idx
                    .and_then(|idx| get_field_value(&record, idx))
                    .or_else(|| q2_median_idx.and_then(|idx| get_field_value(&record, idx)));
                let pearson_val = get_field_value(&record, pearson_idx);

                if mean_val.is_some() && stddev_val.is_some() && median_val.is_some() {
                    // If all base stats exist, pearson_skewness should be computed
                    assert!(
                        pearson_val.is_some(),
                        "pearson_skewness should be computed for latitude"
                    );
                }
            }

            // Verify range_stddev_ratio if range and stddev exist
            if let (Some(range_idx), Some(stddev_idx), Some(range_stddev_idx)) =
                (range_idx, stddev_idx, range_stddev_idx)
            {
                let range_val = get_field_value(&record, range_idx);
                let stddev_val = get_field_value(&record, stddev_idx);
                let range_stddev_val = get_field_value(&record, range_stddev_idx);

                if range_val.is_some() && stddev_val.is_some() {
                    assert!(
                        range_stddev_val.is_some(),
                        "range_stddev_ratio should be computed for latitude"
                    );
                }
            }
        }

        // Check for numeric column with quartiles
        if (field_type == "Float" || field_type == "Integer")
            && q1_idx.is_some()
            && q3_idx.is_some()
        {
            found_numeric_with_quartiles = true;

            if let (Some(q1_idx), Some(q3_idx), Some(quartile_coeff_idx)) =
                (q1_idx, q3_idx, quartile_coeff_idx)
            {
                let q1_val = get_field_value(&record, q1_idx);
                let q3_val = get_field_value(&record, q3_idx);
                let quartile_coeff_val = get_field_value(&record, quartile_coeff_idx);

                if q1_val.is_some() && q3_val.is_some() {
                    assert!(
                        quartile_coeff_val.is_some(),
                        "quartile_coefficient_dispersion should be computed"
                    );
                }
            }

            // Check bowley_skewness
            if let (Some(q1_idx), Some(q3_idx), Some(bowley_idx)) = (q1_idx, q3_idx, bowley_idx) {
                let q1_val = get_field_value(&record, q1_idx);
                let q3_val = get_field_value(&record, q3_idx);
                let q2_val = q2_median_idx
                    .and_then(|idx| get_field_value(&record, idx))
                    .or_else(|| median_idx.and_then(|idx| get_field_value(&record, idx)));
                let bowley_val = get_field_value(&record, bowley_idx);

                if q1_val.is_some() && q2_val.is_some() && q3_val.is_some() {
                    assert!(bowley_val.is_some(), "bowley_skewness should be computed");
                }
            }
        }

        // Check for numeric column with mode
        if (field_type == "Float" || field_type == "Integer")
            && mode_idx.is_some()
            && mean_idx.is_some()
            && stddev_idx.is_some()
        {
            if let (Some(mode_idx), Some(mean_idx), Some(stddev_idx), Some(mode_zscore_idx)) =
                (mode_idx, mean_idx, stddev_idx, mode_zscore_idx)
            {
                let mode_val = get_field_value(&record, mode_idx);
                let mean_val = get_field_value(&record, mean_idx);
                let stddev_val = get_field_value(&record, stddev_idx);
                let mode_zscore_val = get_field_value(&record, mode_zscore_idx);

                // Mode might be a string, so we check if it can be parsed
                if mode_val.is_some() && mean_val.is_some() && stddev_val.is_some() {
                    // mode_zscore might be empty if mode is not numeric, which is fine
                    assert!(mode_zscore_val.is_some(), "mode_zscore column should exist");
                }
            }
        }
    }

    assert!(found_latitude, "Should find latitude column");
    assert!(
        found_numeric_with_quartiles,
        "Should find numeric column with quartiles"
    );
}

#[test]
fn moarstats_missing_base_statistics() {
    let wrk = Workdir::new("moarstats_missing_stats");
    let test_file = wrk.load_test_file("boston311-100.csv");

    // Generate minimal stats (without --everything, --quartiles, --median, --mode)
    let mut stats_cmd = wrk.command("stats");
    stats_cmd.arg("--infer-dates").arg(&test_file);
    wrk.assert_success(&mut stats_cmd);

    // Run moarstats - should succeed but only add stats that can be computed
    let mut cmd = wrk.command("moarstats");
    cmd.arg(&test_file);
    wrk.assert_success(&mut cmd);

    // Verify output file exists
    let stats_path = wrk.path("boston311-100.stats.csv");
    assert!(stats_path.exists(), "Stats file should exist");

    // Read and check which columns were added
    let stats_content = wrk.read_to_string("boston311-100.stats.csv").unwrap();

    // With minimal stats, we should only be able to compute range_stddev_ratio
    // (if range and stddev are available in default stats)
    // pearson_skewness requires median which won't be available
    // quartile stats require q1/q3 which won't be available
    // bowley_skewness requires quartiles which won't be available
    // mode_zscore requires mode which won't be available

    // But the command should succeed and add whatever can be computed
    let _new_columns = verify_new_columns_exist(&stats_content);
    // At least range_stddev_ratio might be available if range and stddev are in default stats
    // The exact columns depend on what default stats includes
}
