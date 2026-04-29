use crate::workdir::Workdir;

#[test]
fn edit_by_col_name() {
    let wrk = Workdir::new("edit_by_col_name");
    wrk.create(
        "data.csv",
        vec![svec!["letter", "number"], svec!["a", "1"], svec!["b", "2"]],
    );

    let mut cmd = wrk.command("edit");
    cmd.arg("data.csv");
    cmd.arg("number");
    cmd.arg("0");
    cmd.arg("3");

    let got: String = wrk.stdout(&mut cmd);
    let expected = "letter,number
a,3
b,2"
    .to_string();
    assert_eq!(got, expected);
}

#[test]
fn edit_by_col_index() {
    let wrk = Workdir::new("edit_by_col_index");
    wrk.create(
        "data.csv",
        vec![svec!["letter", "number"], svec!["a", "1"], svec!["b", "2"]],
    );

    let mut cmd = wrk.command("edit");
    cmd.arg("data.csv");
    cmd.arg("1");
    cmd.arg("0");
    cmd.arg("3");

    let got: String = wrk.stdout(&mut cmd);
    let expected = "letter,number
a,3
b,2"
    .to_string();
    assert_eq!(got, expected);
}

#[test]
fn edit_first_header() {
    let wrk = Workdir::new("edit_first_header");
    wrk.create_from_string(
        "data.csv",
        "letter,number
a,1
b,2",
    );

    let mut cmd = wrk.command("edit");
    cmd.arg("data.csv");
    cmd.arg("letter");
    cmd.arg("0");
    cmd.arg("character");
    cmd.arg("--no-headers");

    let got: String = wrk.stdout(&mut cmd);
    let expected = "character,number
a,1
b,2"
    .to_string();
    assert_eq!(got, expected);
}

#[test]
fn edit_column_index_priority() {
    let wrk = Workdir::new("edit_column_index_priority");
    wrk.create_from_string(
        "data.csv",
        "letter,number,0
a,1,x
b,2,y",
    );

    let mut cmd = wrk.command("edit");
    cmd.arg("data.csv");
    cmd.arg("0");
    cmd.arg("0");
    cmd.arg("z");

    let got: String = wrk.stdout(&mut cmd);
    let expected = "letter,number,0
z,1,x
b,2,y"
        .to_string();
    assert_eq!(got, expected);
}

#[test]
fn edit_by_col_underscore() {
    let wrk = Workdir::new("edit_by_col_underscore");
    wrk.create(
        "data.csv",
        vec![svec!["letter", "number"], svec!["a", "1"], svec!["b", "2"]],
    );

    let mut cmd = wrk.command("edit");
    cmd.arg("data.csv");
    cmd.arg("_");
    cmd.arg("0");
    cmd.arg("3");

    let got: String = wrk.stdout(&mut cmd);
    let expected = "letter,number
a,3
b,2"
    .to_string();
    assert_eq!(got, expected);
}

#[test]
fn edit_in_place_no_extension() {
    let wrk = Workdir::new("edit_in_place_no_extension");
    wrk.create(
        "data",
        vec![svec!["letter", "number"], svec!["a", "1"], svec!["b", "2"]],
    );

    let mut cmd = wrk.command("edit");
    cmd.env("QSV_SKIP_FORMAT_CHECK", "1");
    cmd.arg("data");
    cmd.arg("number");
    cmd.arg("0");
    cmd.arg("3");
    cmd.arg("--in-place");

    cmd.output().unwrap();

    let test_file = wrk.path("data");
    let backup_file = wrk.path("data.bak");
    let got = std::fs::read_to_string(test_file).unwrap();
    let got_backup = std::fs::read_to_string(backup_file).unwrap();
    let expected = "letter,number\na,3\nb,2\n".to_string();
    let expected_backup = "letter,number\na,1\nb,2\n".to_string();
    assert_eq!(got, expected);
    assert_eq!(got_backup, expected_backup);
}

#[test]
fn edit_in_place_rejects_stdin() {
    let wrk = Workdir::new("edit_in_place_rejects_stdin");

    let mut cmd = wrk.command("edit");
    cmd.arg("-");
    cmd.arg("0");
    cmd.arg("0");
    cmd.arg("x");
    cmd.arg("--in-place");

    let got_stderr = wrk.output_stderr(&mut cmd);
    assert!(got_stderr.contains("--in-place requires an input file path"));
}

#[test]
fn edit_row_out_of_range_warns_on_stdout() {
    let wrk = Workdir::new("edit_row_out_of_range_warns_on_stdout");
    wrk.create(
        "data.csv",
        vec![svec!["letter", "number"], svec!["a", "1"], svec!["b", "2"]],
    );

    let mut cmd = wrk.command("edit");
    cmd.arg("data.csv");
    cmd.arg("number");
    cmd.arg("99");
    cmd.arg("3");

    let got_stderr = wrk.output_stderr(&mut cmd);
    assert!(got_stderr.contains("row 99 not found"));
    assert!(got_stderr.contains("input passed through unchanged"));

    let got: String = wrk.stdout(&mut cmd);
    let expected = "letter,number\na,1\nb,2".to_string();
    assert_eq!(got, expected);
}

#[test]
fn edit_row_out_of_range_in_place_errors() {
    let wrk = Workdir::new("edit_row_out_of_range_in_place_errors");
    wrk.create(
        "data.csv",
        vec![svec!["letter", "number"], svec!["a", "1"], svec!["b", "2"]],
    );

    let mut cmd = wrk.command("edit");
    cmd.arg("data.csv");
    cmd.arg("number");
    cmd.arg("99");
    cmd.arg("3");
    cmd.arg("--in-place");

    let got_stderr = wrk.output_stderr(&mut cmd);
    assert!(got_stderr.contains("Row 99 not found"));

    // input must be untouched and no .bak created
    let test_file = wrk.path("data.csv");
    let backup_file = wrk.path("data.csv.bak");
    let got = std::fs::read_to_string(test_file).unwrap();
    let expected = "letter,number\na,1\nb,2\n".to_string();
    assert_eq!(got, expected);
    assert!(!backup_file.exists());
}

#[test]
fn edit_unknown_column_name_errors() {
    let wrk = Workdir::new("edit_unknown_column_name_errors");
    wrk.create(
        "data.csv",
        vec![svec!["letter", "number"], svec!["a", "1"], svec!["b", "2"]],
    );

    let mut cmd = wrk.command("edit");
    cmd.arg("data.csv");
    cmd.arg("nonexistent");
    cmd.arg("0");
    cmd.arg("3");

    let got_stderr = wrk.output_stderr(&mut cmd);
    assert!(got_stderr.contains("Invalid column selected."));
}

#[cfg(unix)]
#[test]
fn edit_in_place_rejects_symlink() {
    let wrk = Workdir::new("edit_in_place_rejects_symlink");
    wrk.create(
        "real.csv",
        vec![svec!["letter", "number"], svec!["a", "1"], svec!["b", "2"]],
    );
    std::os::unix::fs::symlink(wrk.path("real.csv"), wrk.path("link.csv")).unwrap();

    let mut cmd = wrk.command("edit");
    cmd.arg("link.csv");
    cmd.arg("number");
    cmd.arg("0");
    cmd.arg("3");
    cmd.arg("--in-place");

    let got_stderr = wrk.output_stderr(&mut cmd);
    assert!(got_stderr.contains("does not support symlinks"));

    // real file untouched, no .bak created
    let got_real = std::fs::read_to_string(wrk.path("real.csv")).unwrap();
    assert_eq!(got_real, "letter,number\na,1\nb,2\n");
    assert!(!wrk.path("real.csv.bak").exists());
    assert!(!wrk.path("link.csv.bak").exists());
}

#[test]
fn edit_in_place_existing_bak_errors() {
    let wrk = Workdir::new("edit_in_place_existing_bak_errors");
    wrk.create(
        "data.csv",
        vec![svec!["letter", "number"], svec!["a", "1"], svec!["b", "2"]],
    );
    // pre-existing backup
    std::fs::write(wrk.path("data.csv.bak"), b"old backup\n").unwrap();

    let mut cmd = wrk.command("edit");
    cmd.arg("data.csv");
    cmd.arg("number");
    cmd.arg("0");
    cmd.arg("3");
    cmd.arg("--in-place");

    let got_stderr = wrk.output_stderr(&mut cmd);
    assert!(got_stderr.contains("Backup file"));
    assert!(got_stderr.contains("already exists"));

    // input must be untouched and pre-existing .bak preserved
    let got_input = std::fs::read_to_string(wrk.path("data.csv")).unwrap();
    let got_backup = std::fs::read_to_string(wrk.path("data.csv.bak")).unwrap();
    assert_eq!(got_input, "letter,number\na,1\nb,2\n");
    assert_eq!(got_backup, "old backup\n");
}

#[test]
fn edit_column_index_out_of_range_errors() {
    let wrk = Workdir::new("edit_column_index_out_of_range_errors");
    wrk.create(
        "data.csv",
        vec![svec!["letter", "number"], svec!["a", "1"], svec!["b", "2"]],
    );

    let mut cmd = wrk.command("edit");
    cmd.arg("data.csv");
    cmd.arg("99");
    cmd.arg("0");
    cmd.arg("3");

    let got_stderr = wrk.output_stderr(&mut cmd);
    assert!(got_stderr.contains("Invalid column selected."));
}

#[test]
fn edit_in_place() {
    let wrk = Workdir::new("edit_in_place");
    wrk.create(
        "data.csv",
        vec![svec!["letter", "number"], svec!["a", "1"], svec!["b", "2"]],
    );

    let mut cmd = wrk.command("edit");
    cmd.arg("data.csv");
    cmd.arg("number");
    cmd.arg("0");
    cmd.arg("3");
    cmd.arg("--in-place");

    cmd.output().unwrap();

    let test_file = wrk.path("data.csv");
    let backup_file = wrk.path("data.csv.bak");
    let got = std::fs::read_to_string(test_file).unwrap();
    let got_backup = std::fs::read_to_string(backup_file).unwrap();
    let expected = "letter,number
a,3
b,2
"
    .to_string();
    let expected_backup = "letter,number
a,1
b,2
"
    .to_string();
    assert_eq!(got, expected);
    assert_eq!(got_backup, expected_backup);
}
