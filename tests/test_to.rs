use std::{fs::File, path::Path};

use assert_json_diff::assert_json_eq;

// use postgres::{Client, NoTls};
// use rusqlite::Connection;
use crate::workdir::{Workdir, is_same_file};

#[test]
fn to_xlsx_roundtrip() {
    let wrk = Workdir::new("to_xlsx");

    let thedata = vec![
        svec!["Col1", "Description"],
        svec![
            "1",
            "The quick brown fox jumped over the lazy dog by the zigzag quarry site."
        ],
        svec!["2", "Mary had a little lamb"],
        svec![
            "3",
            "I think that I shall never see a poem lovely as a tree."
        ],
        svec!["4", "I think, therefore I am."],
        svec!["5", "I am a leaf on the wind."],
        svec!["6", "Look at me, I'm the captain now."],
        svec!["7", "Bazinga!"],
        svec!["8", "I'm Batman."],
    ];
    wrk.create("in.csv", thedata.clone());

    let xlsx_file = wrk.path("testxlsx.xlsx").to_string_lossy().to_string();
    log::info!("xlsx_file: {}", xlsx_file);

    let mut cmd = wrk.command("to");
    cmd.arg("xlsx").arg(xlsx_file.clone()).arg("in.csv");

    wrk.assert_success(&mut cmd);

    let mut cmd = wrk.command("excel");
    cmd.arg(xlsx_file);

    let got: Vec<Vec<String>> = wrk.read_stdout(&mut cmd);
    assert_eq!(got, thedata);

    wrk.assert_success(&mut cmd);
}

#[test]
fn to_xlsx_roundtrip_all_strings() {
    let wrk = Workdir::new("to_xlsx_all_strings");

    let thedata = vec![
        svec!["col1", "numbers"],
        svec!["1", "1.23"],
        svec!["2", "4014270163361"],
        svec!["3", "3.14"],
        svec!["4", "1234567890"],
        svec!["5", "12345678901234567890"],
        svec!["6", "123456789012345678901234567890"],
        svec!["7", "1234567890123456789012345678901234567890"],
    ];
    wrk.create("in.csv", thedata.clone());

    let xlsx_file = wrk.path("testxlsx.xlsx").to_string_lossy().to_string();
    log::info!("xlsx_file: {}", xlsx_file);

    let mut cmd = wrk.command("to");
    cmd.arg("xlsx")
        .arg("--all-strings")
        .arg(xlsx_file.clone())
        .arg("in.csv");

    wrk.assert_success(&mut cmd);

    let mut cmd = wrk.command("excel");
    cmd.arg(xlsx_file);

    let got: Vec<Vec<String>> = wrk.read_stdout(&mut cmd);
    assert_eq!(got, thedata);

    wrk.assert_success(&mut cmd);
}

#[test]
fn to_xlsx_dir() {
    let wrk = Workdir::new("to_xlsx_dir");

    let cities = vec![
        svec!["city", "state"],
        svec!["Boston", "MA"],
        svec!["New York", "NY"],
        svec!["San Francisco", "CA"],
        svec!["Buffalo", "NY"],
    ];
    let places = vec![
        svec!["city", "place"],
        svec!["Boston", "Logan Airport"],
        svec!["Boston", "Boston Garden"],
        svec!["Buffalo", "Ralph Wilson Stadium"],
        svec!["Orlando", "Disney World"],
    ];

    // create a directory to put the csv files in
    let csv_dir = wrk.path("csvdir");
    std::fs::create_dir(&csv_dir).unwrap();

    wrk.create("csvdir/cities.csv", cities.clone());
    wrk.create("csvdir/places.csv", places.clone());

    let xlsx_file = wrk.path("testxlsx.xlsx").to_string_lossy().to_string();
    log::info!("xlsx_file: {}", xlsx_file);

    let mut cmd = wrk.command("to");
    cmd.arg("xlsx")
        .arg(xlsx_file.clone())
        .arg(wrk.path("csvdir"));

    wrk.assert_success(&mut cmd);

    let mut cmd = wrk.command("excel");
    cmd.arg(xlsx_file.clone()).args(&["--sheet", "cities"]);

    let got: Vec<Vec<String>> = wrk.read_stdout(&mut cmd);
    assert_eq!(got, cities);

    wrk.assert_success(&mut cmd);

    let mut cmd = wrk.command("excel");
    cmd.arg(xlsx_file).args(&["--sheet", "places"]);

    let got: Vec<Vec<String>> = wrk.read_stdout(&mut cmd);
    assert_eq!(got, places);

    wrk.assert_success(&mut cmd);
}

#[test]
fn to_datapackage() {
    let wrk = Workdir::new("to_datapackage");

    let thedata = vec![
        svec!["Col1", "Description"],
        svec![
            "1",
            "The quick brown fox jumped over the lazy dog by the zigzag quarry site."
        ],
        svec!["2", "Mary had a little lamb"],
        svec![
            "3",
            "I think that I shall never see a poem lovely as a tree."
        ],
        svec!["4", "I think, therefore I am."],
        svec!["5", "I am a leaf on the wind."],
        svec!["6", "Look at me, I'm the captain now."],
        svec!["7", "Bazinga!"],
        svec!["8", "I'm Batman."],
    ];
    wrk.create("in.csv", thedata.clone());

    let generateddp_json_path = wrk.path("generateddp.json");
    let generateddp_json_filename = generateddp_json_path.to_string_lossy().to_string();

    let mut cmd = wrk.command("to");
    cmd.arg("datapackage")
        .arg(generateddp_json_filename.clone())
        .arg("in.csv");

    wrk.assert_success(&mut cmd);

    let got: String = wrk.stdout(&mut cmd);
    let expected: String = r#"Table 'in' (8 rows)

Field Name   Field Type  Field Format
Col1         integer     integer
Description  string      string"#
        .to_string();
    assert_eq!(got, expected);

    let generated_json_file = File::open(generateddp_json_path).unwrap();
    let generated_json: serde_json::Value = serde_json::from_reader(generated_json_file).unwrap();

    let expecteddp_json_string = wrk.load_test_resource("testdp.json");
    let expecteddp_json: serde_json::Value = serde_json::from_str(&expecteddp_json_string).unwrap();

    assert_json_eq!(expecteddp_json, generated_json);
}

#[test]
fn to_datapackage_dir() {
    let wrk = Workdir::new("to_datapackage_dir");

    let cities = vec![
        svec!["city", "state"],
        svec!["Boston", "MA"],
        svec!["New York", "NY"],
        svec!["San Francisco", "CA"],
        svec!["Buffalo", "NY"],
    ];
    let places = vec![
        svec!["city", "place"],
        svec!["Boston", "Logan Airport"],
        svec!["Boston", "Boston Garden"],
        svec!["Buffalo", "Ralph Wilson Stadium"],
        svec!["Orlando", "Disney World"],
    ];

    // create a directory to put the csv files in
    let csv_dir = wrk.path("csvdir");
    std::fs::create_dir(&csv_dir).unwrap();

    wrk.create("csvdir/cities.csv", cities.clone());
    wrk.create("csvdir/places.csv", places.clone());

    let dp_file = wrk.path("dpdir.json");
    let dp_file_filename = dp_file.to_string_lossy().to_string();

    let mut cmd = wrk.command("to");
    cmd.arg("datapackage")
        .arg(dp_file_filename.clone())
        .arg(wrk.path("csvdir"));

    let got: String = wrk.stdout(&mut cmd);
    let expected: String = r#"Table 'places' (4 rows)

Field Name  Field Type  Field Format
city        string      string
place       string      string

Table 'cities' (4 rows)

Field Name  Field Type  Field Format
city        string      string
state       string      string"#
        .to_string();

    let expected2: String = r#"Table 'cities' (4 rows)

Field Name  Field Type  Field Format
city        string      string
state       string      string

Table 'places' (4 rows)

Field Name  Field Type  Field Format
city        string      string
place       string      string"#
        .to_string();

    assert!(got == expected || got == expected2);

    let expected = wrk.load_test_file("dpdir.json");
    let expected_path = Path::new(&expected);

    assert!(is_same_file(&dp_file, expected_path).unwrap());
}

// #[ignore = "disable sqlite tests on CI until we resolve csvs_convert crate"]
// #[test]
// fn to_sqlite_dir() {
//     let wrk = Workdir::new("to_sqlite_dir");

//     let cities = vec![
//         svec!["city", "state"],
//         svec!["Boston", "MA"],
//         svec!["New York", "NY"],
//         svec!["San Francisco", "CA"],
//         svec!["Buffalo", "NY"],
//     ];
//     let places = vec![
//         svec!["city", "place"],
//         svec!["Boston", "Logan Airport"],
//         svec!["Boston", "Boston Garden"],
//         svec!["Buffalo", "Ralph Wilson Stadium"],
//         svec!["Orlando", "Disney World"],
//     ];

//     // create a directory to put the csv files in
//     let csv_dir = wrk.path("csvdir");
//     std::fs::create_dir(&csv_dir).unwrap();

//     wrk.create("csvdir/cities.csv", cities.clone());
//     wrk.create("csvdir/places.csv", places.clone());

//     let sqlite_file = wrk.path("test_to_sqlite.db");
//     let sqlite_file_filename = sqlite_file.to_string_lossy().to_string();

//     let mut cmd = wrk.command("to");
//     cmd.arg("sqlite")
//         .arg(sqlite_file_filename.clone())
//         .arg(wrk.path("csvdir"));

//     let got: String = wrk.stdout(&mut cmd);
//     let expected: String = r#"Table 'places' (4 rows)

// Field Name  Field Type  Field Format
// city        string      string
// place       string      string

// Table 'cities' (4 rows)

// Field Name  Field Type  Field Format
// city        string      string
// state       string      string"#
//         .to_string();

//     let expected2: String = r#"Table 'cities' (4 rows)

// Field Name  Field Type  Field Format
// city        string      string
// state       string      string

// Table 'places' (4 rows)

// Field Name  Field Type  Field Format
// city        string      string
// place       string      string"#
//         .to_string();

//     assert!(got == expected || got == expected2);

//     let db = Connection::open(sqlite_file_filename).unwrap();
//     let mut stmt = db.prepare("SELECT * FROM cities ORDER BY city").unwrap();
//     let cities_iter = stmt
//         .query_map([], |row| Ok((row.get(0).unwrap(), row.get(1).unwrap())))
//         .unwrap();
//     let cities: Vec<(String, String)> = cities_iter.map(|r| r.unwrap()).collect();
//     assert_eq!(
//         cities,
//         vec![
//             (String::from("Boston"), String::from("MA")),
//             (String::from("Buffalo"), String::from("NY")),
//             (String::from("New York"), String::from("NY")),
//             (String::from("San Francisco"), String::from("CA")),
//         ]
//     );

//     let mut stmt = db
//         .prepare("SELECT * FROM places ORDER BY city, place")
//         .unwrap();
//     let places_iter = stmt
//         .query_map([], |row| Ok((row.get(0).unwrap(), row.get(1).unwrap())))
//         .unwrap();
//     let places: Vec<(String, String)> = places_iter.map(|r| r.unwrap()).collect();
//     assert_eq!(
//         places,
//         vec![
//             (String::from("Boston"), String::from("Boston Garden")),
//             (String::from("Boston"), String::from("Logan Airport")),
//             (
//                 String::from("Buffalo"),
//                 String::from("Ralph Wilson Stadium")
//             ),
//             (String::from("Orlando"), String::from("Disney World")),
//         ]
//     );
// }

// #[test]
// fn to_sqlite() {
//     let wrk = Workdir::new("to_sqlite");

//     let cities = vec![
//         svec!["city", "state"],
//         svec!["Boston", "MA"],
//         svec!["New York", "NY"],
//         svec!["San Francisco", "CA"],
//         svec!["Buffalo", "NY"],
//     ];
//     let places = vec![
//         svec!["city", "place"],
//         svec!["Boston", "Logan Airport"],
//         svec!["Boston", "Boston Garden"],
//         svec!["Buffalo", "Ralph Wilson Stadium"],
//         svec!["Orlando", "Disney World"],
//     ];

//     wrk.create("cities.csv", cities.clone());
//     wrk.create("places.csv", places.clone());

//     let sqlite_file = wrk.path("test_to_sqlite.db");
//     let sqlite_file_filename = sqlite_file.to_string_lossy().to_string();

//     let mut cmd = wrk.command("to");
//     cmd.arg("sqlite")
//         .arg(sqlite_file_filename.clone())
//         .arg("places.csv")
//         .arg("cities.csv");

//     let got: String = wrk.stdout(&mut cmd);
//     let expected: String = r#"Table 'places' (4 rows)

// Field Name  Field Type  Field Format
// city        string      string
// place       string      string

// Table 'cities' (4 rows)

// Field Name  Field Type  Field Format
// city        string      string
// state       string      string"#
//         .to_string();
//     assert_eq!(got, expected);

//     let db = Connection::open(sqlite_file_filename).unwrap();
//     let mut stmt = db.prepare("SELECT * FROM cities ORDER BY city").unwrap();
//     let cities_iter = stmt
//         .query_map([], |row| Ok((row.get(0).unwrap(), row.get(1).unwrap())))
//         .unwrap();
//     let cities: Vec<(String, String)> = cities_iter.map(|r| r.unwrap()).collect();
//     assert_eq!(
//         cities,
//         vec![
//             (String::from("Boston"), String::from("MA")),
//             (String::from("Buffalo"), String::from("NY")),
//             (String::from("New York"), String::from("NY")),
//             (String::from("San Francisco"), String::from("CA")),
//         ]
//     );

//     let mut stmt = db
//         .prepare("SELECT * FROM places ORDER BY city, place")
//         .unwrap();
//     let places_iter = stmt
//         .query_map([], |row| Ok((row.get(0).unwrap(), row.get(1).unwrap())))
//         .unwrap();
//     let places: Vec<(String, String)> = places_iter.map(|r| r.unwrap()).collect();
//     assert_eq!(
//         places,
//         vec![
//             (String::from("Boston"), String::from("Boston Garden")),
//             (String::from("Boston"), String::from("Logan Airport")),
//             (
//                 String::from("Buffalo"),
//                 String::from("Ralph Wilson Stadium")
//             ),
//             (String::from("Orlando"), String::from("Disney World")),
//         ]
//     );
// }

#[test]
fn to_ods_roundtrip() {
    let wrk = Workdir::new("to_ods");

    let thedata = vec![
        svec!["Col1", "Description"],
        svec![
            "1",
            "The quick brown fox jumped over the lazy dog by the zigzag quarry site."
        ],
        svec!["2", "Mary had a little lamb"],
        svec![
            "3",
            "I think that I shall never see a poem lovely as a tree."
        ],
        svec!["4", "I think, therefore I am."],
        svec!["5", "I am a leaf on the wind."],
        svec!["6", "Look at me, I'm the captain now."],
        svec!["7", "Bazinga!"],
        svec!["8", "I'm Batman."],
    ];
    wrk.create("in.csv", thedata.clone());

    let ods_file = wrk.path("testods.ods").to_string_lossy().to_string();
    log::info!("ods_file: {}", ods_file);

    let mut cmd = wrk.command("to");
    cmd.arg("ods").arg(ods_file.clone()).arg("in.csv");

    wrk.assert_success(&mut cmd);

    let mut cmd = wrk.command("excel");
    cmd.arg(ods_file);

    let got: Vec<Vec<String>> = wrk.read_stdout(&mut cmd);
    assert_eq!(got, thedata);

    wrk.assert_success(&mut cmd);
}

#[test]
fn to_ods_dir() {
    let wrk = Workdir::new("to_ods_dir");

    // Create test files
    let file1_data = vec![
        svec!["Col1", "Description"],
        svec!["1", "First file data"],
        svec!["2", "More data"],
    ];
    wrk.create("file1.csv", file1_data.clone());

    let file2_data = vec![
        svec!["Col1", "Description"],
        svec!["3", "Second file data"],
        svec!["4", "Even more data"],
    ];
    wrk.create("file2.csv", file2_data.clone());

    // Create a single ODS file that will contain both sheets
    let ods_file = wrk.path("testods.ods").to_string_lossy().to_string();
    log::info!("ods_file: {}", ods_file);

    // Convert files to ODS
    let mut cmd = wrk.command("to");
    cmd.arg("ods")
        .arg(ods_file.clone())
        .arg("file1.csv")
        .arg("file2.csv");

    wrk.assert_success(&mut cmd);

    // Verify the content of the first sheet
    let mut cmd = wrk.command("excel");
    cmd.arg(ods_file.clone()).args(&["--sheet", "file1"]);

    let got: Vec<Vec<String>> = wrk.read_stdout(&mut cmd);
    assert_eq!(got, file1_data);

    wrk.assert_success(&mut cmd);

    // Verify the content of the second sheet
    let mut cmd = wrk.command("excel");
    cmd.arg(ods_file).args(&["--sheet", "file2"]);

    let got: Vec<Vec<String>> = wrk.read_stdout(&mut cmd);
    assert_eq!(got, file2_data);

    wrk.assert_success(&mut cmd);
}

#[test]
fn to_table_xlsx_happy_path() {
    // --table with xlsx should use the custom sheet name
    let wrk = Workdir::new("to_table_xlsx_happy");
    wrk.create(
        "in.csv",
        vec![svec!["city", "state"], svec!["Boston", "MA"]],
    );

    let xlsx_file = wrk.path("test.xlsx").to_string_lossy().to_string();
    let mut cmd = wrk.command("to");
    cmd.arg("xlsx")
        .arg("--table")
        .arg("My Sheet")
        .arg(&xlsx_file)
        .arg("in.csv");

    let output = wrk.output(&mut cmd);
    assert!(
        output.status.success(),
        "Command failed: {}",
        String::from_utf8_lossy(&output.stderr)
    );
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(
        stdout.contains("My Sheet"),
        "Expected sheet name 'My Sheet' in output, got: {stdout}"
    );
    assert!(
        wrk.path("test.xlsx").exists(),
        "Expected output file test.xlsx to exist"
    );
}

#[test]
fn to_table_error_xlsx_invalid_chars() {
    // --table with invalid sheet name characters should fail for xlsx
    let wrk = Workdir::new("to_table_error_xlsx_chars");
    wrk.create("in.csv", vec![svec!["col1"], svec!["a"]]);

    let xlsx_file = wrk.path("test.xlsx").to_string_lossy().to_string();
    let mut cmd = wrk.command("to");
    cmd.arg("xlsx")
        .arg("--table")
        .arg("bad[name")
        .arg(xlsx_file)
        .arg("in.csv");

    let stderr = wrk.output_stderr(&mut cmd);
    assert!(
        stderr.contains("sheet name cannot contain"),
        "Expected invalid chars error, got: {stderr}"
    );
}

#[test]
fn to_table_error_xlsx_too_long() {
    // --table with sheet name > 31 chars should fail for xlsx
    let wrk = Workdir::new("to_table_error_xlsx_long");
    wrk.create("in.csv", vec![svec!["col1"], svec!["a"]]);

    let xlsx_file = wrk.path("test.xlsx").to_string_lossy().to_string();
    let mut cmd = wrk.command("to");
    cmd.arg("xlsx")
        .arg("--table")
        .arg("a".repeat(32))
        .arg(xlsx_file)
        .arg("in.csv");

    let stderr = wrk.output_stderr(&mut cmd);
    assert!(
        stderr.contains("must not exceed 31 characters"),
        "Expected length error, got: {stderr}"
    );
}

#[test]
fn to_table_error_ods_too_long() {
    // --table with sheet name > 31 chars should fail for ods
    let wrk = Workdir::new("to_table_error_ods_long");
    wrk.create("in.csv", vec![svec!["col1"], svec!["a"]]);

    let ods_file = wrk.path("test.ods").to_string_lossy().to_string();
    let mut cmd = wrk.command("to");
    cmd.arg("ods")
        .arg("--table")
        .arg("a".repeat(32))
        .arg(ods_file)
        .arg("in.csv");

    let stderr = wrk.output_stderr(&mut cmd);
    assert!(
        stderr.contains("must not exceed 31 characters"),
        "Expected length error, got: {stderr}"
    );
}

#[test]
fn to_table_ods_happy_path() {
    // --table with ods should use the custom sheet name
    let wrk = Workdir::new("to_table_ods_happy");
    wrk.create(
        "in.csv",
        vec![svec!["city", "state"], svec!["Boston", "MA"]],
    );

    let ods_file = wrk.path("test.ods").to_string_lossy().to_string();
    let mut cmd = wrk.command("to");
    cmd.arg("ods")
        .arg("--table")
        .arg("My Sheet")
        .arg(&ods_file)
        .arg("in.csv");

    let output = wrk.output(&mut cmd);
    assert!(
        output.status.success(),
        "Command failed: {}",
        String::from_utf8_lossy(&output.stderr)
    );
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(
        stdout.contains("My Sheet"),
        "Expected sheet name 'My Sheet' in output, got: {stdout}"
    );
    assert!(
        wrk.path("test.ods").exists(),
        "Expected output file test.ods to exist"
    );
}

#[test]
fn to_table_error_ods_invalid_chars() {
    // --table with invalid sheet name characters should fail for ods
    let wrk = Workdir::new("to_table_error_ods_chars");
    wrk.create("in.csv", vec![svec!["col1"], svec!["a"]]);

    let ods_file = wrk.path("test.ods").to_string_lossy().to_string();
    let mut cmd = wrk.command("to");
    cmd.arg("ods")
        .arg("--table")
        .arg("bad:name")
        .arg(ods_file)
        .arg("in.csv");

    let stderr = wrk.output_stderr(&mut cmd);
    assert!(
        stderr.contains("sheet name cannot contain"),
        "Expected invalid chars error, got: {stderr}"
    );
}

#[test]
fn to_table_error_datapackage() {
    // --table should fail with datapackage subcommand
    let wrk = Workdir::new("to_table_error_dp");
    wrk.create("in.csv", vec![svec!["col1"], svec!["a"]]);

    let dp_file = wrk.path("dp.json").to_string_lossy().to_string();
    let mut cmd = wrk.command("to");
    cmd.arg("datapackage")
        .arg("--table")
        .arg("custom_name")
        .arg(dp_file)
        .arg("in.csv");

    let stderr = wrk.output_stderr(&mut cmd);
    assert!(
        stderr.contains("--table cannot be used with the datapackage subcommand"),
        "Expected unsupported subcommand error, got: {stderr}"
    );
}

#[test]
fn to_table_error_empty_name() {
    // --table with empty name should fail
    let wrk = Workdir::new("to_table_error_empty");
    wrk.create("in.csv", vec![svec!["col1"], svec!["a"]]);

    let sqlite_file = wrk.path("test.db").to_string_lossy().to_string();
    let mut cmd = wrk.command("to");
    cmd.arg("sqlite")
        .arg("--table")
        .arg("")
        .arg(sqlite_file)
        .arg("in.csv");

    let stderr = wrk.output_stderr(&mut cmd);
    assert!(
        stderr.contains("--table name must not be empty"),
        "Expected empty name error, got: {stderr}"
    );
}

#[test]
fn to_table_error_invalid_chars() {
    // --table with special characters should fail
    let wrk = Workdir::new("to_table_error_chars");
    wrk.create("in.csv", vec![svec!["col1"], svec!["a"]]);

    let sqlite_file = wrk.path("test.db").to_string_lossy().to_string();
    let mut cmd = wrk.command("to");
    cmd.arg("sqlite")
        .arg("--table")
        .arg("my table!@#")
        .arg(sqlite_file)
        .arg("in.csv");

    let stderr = wrk.output_stderr(&mut cmd);
    assert!(
        stderr.contains("--table name must contain only alphanumeric"),
        "Expected invalid chars error, got: {stderr}"
    );
}

#[test]
fn to_table_error_multiple_inputs() {
    // --table with multiple input files should fail
    let wrk = Workdir::new("to_table_error_multi");
    wrk.create("in1.csv", vec![svec!["col1"], svec!["a"]]);
    wrk.create("in2.csv", vec![svec!["col1"], svec!["b"]]);

    let sqlite_file = wrk.path("test.db").to_string_lossy().to_string();
    let mut cmd = wrk.command("to");
    cmd.arg("sqlite")
        .arg("--table")
        .arg("custom_name")
        .arg(sqlite_file)
        .arg("in1.csv")
        .arg("in2.csv");

    let stderr = wrk.output_stderr(&mut cmd);
    assert!(
        stderr.contains("--table can only be used with a single input file"),
        "Expected multiple inputs error, got: {stderr}"
    );
}

#[test]
fn to_table_sqlite_happy_path() {
    // --table with sqlite should use the custom table name
    let wrk = Workdir::new("to_table_sqlite_happy");
    wrk.create(
        "in.csv",
        vec![svec!["city", "state"], svec!["Boston", "MA"]],
    );

    let dump_file = wrk.path("dump.sql").to_string_lossy().to_string();
    let mut cmd = wrk.command("to");
    cmd.arg("sqlite")
        .arg("--table")
        .arg("my_custom_table")
        .arg("--dump")
        .arg(dump_file.clone())
        .arg("in.csv");

    wrk.assert_success(&mut cmd);

    // Verify the dump file references the custom table name
    let dump_content = std::fs::read_to_string(&dump_file).unwrap();
    assert!(
        dump_content.contains("my_custom_table"),
        "Expected dump to contain custom table name 'my_custom_table', got:\n{dump_content}"
    );
    // Ensure the original filename-based table name is NOT used (check multiple quoting styles:
    // double-quoted, single-quoted, backtick-quoted, and unquoted)
    assert!(
        !dump_content.contains("CREATE TABLE \"in\"")
            && !dump_content.contains("CREATE TABLE 'in'")
            && !dump_content.contains("CREATE TABLE `in`")
            && !dump_content.contains("CREATE TABLE in(")
            && !dump_content.contains("CREATE TABLE in ")
            && !dump_content.contains("CREATE TABLE in\n"),
        "Dump should not contain original table name 'in', got:\n{dump_content}"
    );
}

#[test]
fn to_table_error_leading_digit() {
    // --table with a name starting with a digit should fail
    let wrk = Workdir::new("to_table_error_digit");
    wrk.create("in.csv", vec![svec!["col1"], svec!["a"]]);

    let sqlite_file = wrk.path("test.db").to_string_lossy().to_string();
    let mut cmd = wrk.command("to");
    cmd.arg("sqlite")
        .arg("--table")
        .arg("123table")
        .arg(sqlite_file)
        .arg("in.csv");

    let stderr = wrk.output_stderr(&mut cmd);
    assert!(
        stderr.contains("--table name must start with a letter or underscore"),
        "Expected leading digit error, got: {stderr}"
    );
}

#[test]
fn to_table_error_hyphen() {
    // --table with hyphens should fail (invalid SQL identifier)
    let wrk = Workdir::new("to_table_error_hyphen");
    wrk.create("in.csv", vec![svec!["col1"], svec!["a"]]);

    let sqlite_file = wrk.path("test.db").to_string_lossy().to_string();
    let mut cmd = wrk.command("to");
    cmd.arg("sqlite")
        .arg("--table")
        .arg("my-table")
        .arg(sqlite_file)
        .arg("in.csv");

    let stderr = wrk.output_stderr(&mut cmd);
    assert!(
        stderr.contains("--table name must contain only alphanumeric"),
        "Expected hyphen error, got: {stderr}"
    );
}

#[test]
fn to_table_underscore_prefix() {
    // --table with underscore-prefixed name should succeed
    let wrk = Workdir::new("to_table_underscore_prefix");
    wrk.create(
        "data.csv",
        vec![svec!["city", "state"], svec!["Boston", "MA"]],
    );

    let dump_file = wrk.path("dump.sql").to_string_lossy().to_string();
    let mut cmd = wrk.command("to");
    cmd.arg("sqlite")
        .arg("--table")
        .arg("_staging")
        .arg("--dump")
        .arg(dump_file)
        .arg("data.csv");

    wrk.assert_success(&mut cmd);
}

#[test]
fn to_table_preserves_original_file() {
    // --table should not rename/modify the user's original file
    let wrk = Workdir::new("to_table_preserves_file");
    wrk.create(
        "original.csv",
        vec![svec!["city", "state"], svec!["Boston", "MA"]],
    );

    let original_path = wrk.path("original.csv");
    assert!(original_path.exists(), "original file should exist before");

    let dump_file = wrk.path("dump.sql").to_string_lossy().to_string();
    let mut cmd = wrk.command("to");
    cmd.arg("sqlite")
        .arg("--table")
        .arg("custom_name")
        .arg("--dump")
        .arg(dump_file)
        .arg("original.csv");

    wrk.assert_success(&mut cmd);

    // The original file must still exist with its original name
    assert!(
        original_path.exists(),
        "original file should still exist after --table"
    );
}

// #[test]
// #[ignore = "Testing postgres support requires a running, properly configured postgres server, \
//             which is not available on CI"]
// fn to_postgres() {
//     let wrk = Workdir::new("to_postgres");

//     let cities = vec![
//         svec!["city", "state"],
//         svec!["Boston", "MA"],
//         svec!["New York", "NY"],
//         svec!["San Francisco", "CA"],
//         svec!["Buffalo", "NY"],
//     ];
//     let places = vec![
//         svec!["city", "place"],
//         svec!["Boston", "Logan Airport"],
//         svec!["Boston", "Boston Garden"],
//         svec!["Buffalo", "Ralph Wilson Stadium"],
//         svec!["Orlando", "Disney World"],
//     ];

//     wrk.create("cities.csv", cities.clone());
//     wrk.create("places.csv", places.clone());

//     let mut cmd = wrk.command("to");
//     cmd.arg("postgres")
//         .arg("postgres://testuser:test123@localhost/testdb")
//         .arg("places.csv")
//         .arg("cities.csv")
//         .arg("--drop");

//     let got: String = wrk.stdout(&mut cmd);
//     let expected: String = r#"Table 'places' (4 rows)

// Field Name  Field Type  Field Format
// city        string      string
// place       string      string

// Table 'cities' (4 rows)

// Field Name  Field Type  Field Format
// city        string      string
// state       string      string"#
//         .to_string();
//     assert_eq!(got, expected);

//     let mut client =
//         Client::connect("postgres://testuser:test123@localhost/testdb", NoTls).unwrap();
//     let mut cities_result: Vec<(String, String)> = vec![];
//     for row in client
//         .query("SELECT * FROM cities ORDER BY city", &[])
//         .unwrap()
//     {
//         let city: String = row.get(0);
//         let state: String = row.get(1);
//         cities_result.push((city, state));
//     }
//     assert_eq!(
//         cities_result,
//         vec![
//             (String::from("Boston"), String::from("MA")),
//             (String::from("Buffalo"), String::from("NY")),
//             (String::from("New York"), String::from("NY")),
//             (String::from("San Francisco"), String::from("CA")),
//         ]
//     );

//     let mut places_result: Vec<(String, String)> = vec![];
//     for row in client
//         .query("SELECT * FROM places ORDER BY city, place", &[])
//         .unwrap()
//     {
//         let city: String = row.get(0);
//         let place: String = row.get(1);
//         places_result.push((city, place));
//     }
//     assert_eq!(
//         places_result,
//         vec![
//             (String::from("Boston"), String::from("Boston Garden")),
//             (String::from("Boston"), String::from("Logan Airport")),
//             (
//                 String::from("Buffalo"),
//                 String::from("Ralph Wilson Stadium")
//             ),
//             (String::from("Orlando"), String::from("Disney World")),
//         ]
//     );
// }

// ===== Parquet tests (require polars feature) =====

#[test]
#[cfg(feature = "polars")]
fn to_parquet_basic() {
    let wrk = Workdir::new("to_parquet_basic");
    wrk.create(
        "data.csv",
        vec![
            svec!["city", "pop"],
            svec!["Boston", "685000"],
            svec!["New York", "8300000"],
        ],
    );

    let output_dir = wrk.path("parquet_out");
    let mut cmd = wrk.command("to");
    cmd.arg("parquet")
        .arg(output_dir.to_string_lossy().as_ref())
        .arg("data.csv");

    wrk.assert_success(&mut cmd);

    let parquet_file = output_dir.join("data.parquet");
    assert!(parquet_file.exists(), "parquet file should be created");

    // Read back and verify contents
    let df = polars::prelude::LazyFrame::scan_parquet(
        polars::prelude::PlRefPath::new(parquet_file.to_string_lossy().as_ref()),
        Default::default(),
    )
    .unwrap()
    .collect()
    .unwrap();
    assert_eq!(df.height(), 2, "should have 2 rows");
    assert_eq!(df.width(), 2, "should have 2 columns");
    assert_eq!(
        df.get_column_names(),
        &["city", "pop"],
        "column names should match"
    );
}

#[test]
#[cfg(feature = "polars")]
fn to_parquet_multiple() {
    let wrk = Workdir::new("to_parquet_multiple");
    wrk.create(
        "cities.csv",
        vec![
            svec!["city", "state"],
            svec!["Boston", "MA"],
            svec!["Albany", "NY"],
        ],
    );
    wrk.create(
        "places.csv",
        vec![
            svec!["place", "city"],
            svec!["Fenway Park", "Boston"],
            svec!["Capitol", "Albany"],
        ],
    );

    let output_dir = wrk.path("parquet_out");
    let mut cmd = wrk.command("to");
    cmd.arg("parquet")
        .arg(output_dir.to_string_lossy().as_ref())
        .arg("cities.csv")
        .arg("places.csv");

    wrk.assert_success(&mut cmd);

    assert!(
        output_dir.join("cities.parquet").exists(),
        "cities.parquet should be created"
    );
    assert!(
        output_dir.join("places.parquet").exists(),
        "places.parquet should be created"
    );
}

#[test]
#[cfg(feature = "polars")]
fn to_parquet_compression_snappy() {
    let wrk = Workdir::new("to_parquet_snappy");
    wrk.create(
        "data.csv",
        vec![
            svec!["name", "value"],
            svec!["alpha", "1"],
            svec!["beta", "2"],
        ],
    );

    let output_dir = wrk.path("parquet_out");
    let mut cmd = wrk.command("to");
    cmd.arg("parquet")
        .arg(output_dir.to_string_lossy().as_ref())
        .arg("--compression")
        .arg("snappy")
        .arg("data.csv");

    wrk.assert_success(&mut cmd);

    assert!(output_dir.join("data.parquet").exists());
}

#[test]
#[cfg(feature = "polars")]
fn to_parquet_custom_table() {
    let wrk = Workdir::new("to_parquet_table");
    wrk.create(
        "data.csv",
        vec![svec!["col1", "col2"], svec!["a", "1"], svec!["b", "2"]],
    );

    let output_dir = wrk.path("parquet_out");
    let mut cmd = wrk.command("to");
    cmd.arg("parquet")
        .arg(output_dir.to_string_lossy().as_ref())
        .arg("--table")
        .arg("custom_name")
        .arg("data.csv");

    wrk.assert_success(&mut cmd);

    assert!(
        output_dir.join("custom_name.parquet").exists(),
        "parquet file should use custom table name"
    );
}

#[test]
#[cfg(feature = "polars")]
fn to_parquet_dir() {
    let wrk = Workdir::new("to_parquet_dir");

    // Create a subdirectory with CSV files
    let input_dir = wrk.path("input_csvs");
    std::fs::create_dir_all(&input_dir).unwrap();
    std::fs::write(input_dir.join("file1.csv"), "name,value\nalpha,1\nbeta,2\n").unwrap();
    std::fs::write(
        input_dir.join("file2.csv"),
        "city,state\nBoston,MA\nAlbany,NY\n",
    )
    .unwrap();

    let output_dir = wrk.path("parquet_out");
    let mut cmd = wrk.command("to");
    cmd.arg("parquet")
        .arg(output_dir.to_string_lossy().as_ref())
        .arg(input_dir.to_string_lossy().as_ref());

    wrk.assert_success(&mut cmd);

    assert!(output_dir.join("file1.parquet").exists());
    assert!(output_dir.join("file2.parquet").exists());
}
