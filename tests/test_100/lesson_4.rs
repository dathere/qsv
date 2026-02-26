// Lesson 4: Running Polars SQL queries with qsv
// https://100.dathere.com/lessons/4

use crate::workdir::Workdir;

#[test]
fn lesson_4_task_1() {
    let wrk = Workdir::new("lesson_4_task_1");
    let buses_csv_file = wrk.load_test_file("buses.csv");
    let mut cmd = wrk.command("sqlp");
    cmd.arg(buses_csv_file.as_str())
        .arg("SELECT * FROM buses")
        .arg("-q");

    let got: String = wrk.stdout(&mut cmd);
    let expected = r#"id,primary_color,secondary_color,length,air_conditioner,amenities
1,black,blue,full,true,"wheelchair ramp, tissue boxes, cup holders, USB ports"
2,black,red,full,true,"wheelchair ramp, tissue boxes, USB ports"
3,white,blue,half,true,"wheelchair ramp, tissue boxes"
4,orange,blue,full,false,"wheelchair ramp, tissue boxes, USB ports"
5,black,blue,full,true,"wheelchair ramp, tissue boxes, cup holders, USB ports""#;
    assert_eq!(got, expected);
}

#[test]
fn lesson_4_task_2() {
    let wrk = Workdir::new("lesson_4_task_2");
    let buses_csv_file = wrk.load_test_file("buses.csv");
    let mut cmd = wrk.command("sqlp");
    cmd.arg(buses_csv_file.as_str())
        .arg("SELECT * FROM buses LIMIT 2")
        .arg("-q");

    let got: String = wrk.stdout(&mut cmd);
    let expected = r#"id,primary_color,secondary_color,length,air_conditioner,amenities
1,black,blue,full,true,"wheelchair ramp, tissue boxes, cup holders, USB ports"
2,black,red,full,true,"wheelchair ramp, tissue boxes, USB ports""#;
    assert_eq!(got, expected);
}

#[test]
fn lesson_4_task_3() {
    use std::process;

    let wrk = Workdir::new("lesson_4_task_3");
    let test_file = wrk.load_test_file("buses.csv");

    let mut select_cmd = process::Command::new(wrk.qsv_bin());
    select_cmd.args(vec![
        "sqlp",
        test_file.as_str(),
        "SELECT id,length,air_conditioner FROM buses",
        "-q",
    ]);
    let select_stdout: String = wrk.stdout(&mut select_cmd);

    let mut table_child = process::Command::new(wrk.qsv_bin())
        .stdin(std::process::Stdio::piped())
        .stdout(std::process::Stdio::piped())
        .args(vec!["table"])
        .spawn()
        .unwrap();
    let mut table_stdin = table_child.stdin.take().unwrap();
    std::thread::spawn(move || {
        use std::io::Write;

        table_stdin.write_all(select_stdout.as_bytes()).unwrap();
    });
    let output = table_child.wait_with_output().unwrap();
    let got = String::from_utf8_lossy(&output.stdout);

    let expected = r#"id  length  air_conditioner
1   full    true
2   full    true
3   half    true
4   full    false
5   full    true
"#;
    assert_eq!(got, expected);
}

#[test]
fn lesson_4_task_4() {
    let wrk = Workdir::new("lesson_4_task_4");
    let buses_csv_file = wrk.load_test_file("buses.csv");
    let mut cmd = wrk.command("sqlp");
    cmd.arg(buses_csv_file.as_str())
        .arg("SELECT id FROM buses WHERE air_conditioner = 'true'")
        .arg("--format")
        .arg("json")
        .arg("-q");

    let got: String = wrk.stdout(&mut cmd);
    let expected = r#"[{"id":1},{"id":2},{"id":3},{"id":5}]"#;
    assert_eq!(got, expected);
}

#[test]
fn lesson_4_task_5() {
    let wrk = Workdir::new("lesson_4_task_5");
    let buses_csv_file = wrk.load_test_file("buses.csv");
    let mut cmd = wrk.command("sqlp");
    cmd.arg(buses_csv_file.as_str())
        .arg("SELECT id FROM buses WHERE amenities ILIKE '%cup holders%'")
        .arg("--format")
        .arg("jsonl")
        .arg("-q");

    let got: String = wrk.stdout(&mut cmd);
    let expected = r#"{"id":1}
{"id":5}"#;
    assert_eq!(got, expected);
}

#[test]
fn lesson_4_task_6() {
    let wrk = Workdir::new("lesson_4_task_6");
    let buses_csv_file = wrk.load_test_file("buses.csv");
    let mut cmd = wrk.command("sqlp");
    cmd.arg(buses_csv_file.as_str())
        .arg("SELECT COUNT(*) FROM buses WHERE primary_color = 'black' OR primary_color = 'white'")
        .arg("-q");

    let got: String = wrk.stdout(&mut cmd);
    let expected = r#"len
5"#;
    assert_eq!(got, expected);
}
