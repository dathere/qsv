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
fn to_parquet_dir() {
    let wrk = Workdir::new("to_parquet_dir");

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

    wrk.create("cities.csv", cities.clone());
    wrk.create("places.csv", places.clone());

    let parquet_dir = wrk.path("test_parquet_dir");
    let parquet_dirname = parquet_dir.to_string_lossy().to_string();

    let mut cmd = wrk.command("to");
    cmd.arg("parquet")
        .arg(parquet_dirname.clone())
        .arg("places.csv")
        .arg("cities.csv");

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
    assert_eq!(got, expected);

    // check that the parquet files were created
    let files = std::fs::read_dir(parquet_dir).unwrap();
    let mut file_names = files
        .map(|f| f.unwrap().file_name().into_string().unwrap())
        .collect::<Vec<_>>();
    file_names.sort();
    assert_eq!(file_names, vec!["cities.parquet", "places.parquet"]);

    // TODO: check that the parquet files are valid and contain the correct data
}

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
