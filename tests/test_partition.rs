use std::borrow::ToOwned;

use crate::workdir::Workdir;

macro_rules! part_eq {
    ($wrk:expr_2021, $path:expr_2021, $expected:expr_2021) => {
        assert_eq!(
            $wrk.from_str::<String>(&$wrk.path($path)),
            $expected.to_owned()
        );
    };
}

fn data(headers: bool) -> Vec<Vec<String>> {
    let mut rows = vec![
        svec!["NY", "Manhattan"],
        svec!["CA", "San Francisco"],
        svec!["TX", "Dallas"],
        svec!["NY", "Buffalo"],
        svec!["TX", "Fort Worth"],
    ];
    if headers {
        rows.insert(0, svec!["state", "city"]);
    }
    rows
}

#[test]
fn partition() {
    let wrk = Workdir::new("partition");
    wrk.create("in.csv", data(true));

    let mut cmd = wrk.command("partition");
    cmd.arg("state").arg(&wrk.path(".")).arg("in.csv");
    wrk.run(&mut cmd);

    part_eq!(
        wrk,
        "CA.csv",
        "\
state,city
CA,San Francisco
"
    );
    part_eq!(
        wrk,
        "NY.csv",
        "\
state,city
NY,Manhattan
NY,Buffalo
"
    );
    part_eq!(
        wrk,
        "TX.csv",
        "\
state,city
TX,Dallas
TX,Fort Worth
"
    );
}

#[test]
fn partition_drop() {
    let wrk = Workdir::new("partition_drop");
    wrk.create("in.csv", data(true));

    let mut cmd = wrk.command("partition");
    cmd.arg("--drop")
        .arg("state")
        .arg(&wrk.path("."))
        .arg("in.csv");
    wrk.run(&mut cmd);

    part_eq!(
        wrk,
        "CA.csv",
        "\
city
San Francisco
"
    );
    part_eq!(
        wrk,
        "NY.csv",
        "\
city
Manhattan
Buffalo
"
    );
    part_eq!(
        wrk,
        "TX.csv",
        "\
city
Dallas
Fort Worth
"
    );
}

#[test]
fn partition_without_headers() {
    let wrk = Workdir::new("partition_without_headers");
    wrk.create("in.csv", data(false));

    let mut cmd = wrk.command("partition");
    cmd.arg("--no-headers")
        .arg("1")
        .arg(&wrk.path("."))
        .arg("in.csv");
    wrk.run(&mut cmd);

    part_eq!(
        wrk,
        "CA.csv",
        "\
CA,San Francisco
"
    );
    part_eq!(
        wrk,
        "NY.csv",
        "\
NY,Manhattan
NY,Buffalo
"
    );
    part_eq!(
        wrk,
        "TX.csv",
        "\
TX,Dallas
TX,Fort Worth
"
    );
}

#[test]
fn partition_drop_without_headers() {
    let wrk = Workdir::new("partition_drop_without_headers");
    wrk.create("in.csv", data(false));

    let mut cmd = wrk.command("partition");
    cmd.arg("--drop")
        .arg("--no-headers")
        .arg("1")
        .arg(&wrk.path("."))
        .arg("in.csv");
    wrk.run(&mut cmd);

    part_eq!(
        wrk,
        "CA.csv",
        "\
San Francisco
"
    );
    part_eq!(
        wrk,
        "NY.csv",
        "\
Manhattan
Buffalo
"
    );
    part_eq!(
        wrk,
        "TX.csv",
        "\
Dallas
Fort Worth
"
    );
}

#[test]
fn partition_into_new_directory() {
    let wrk = Workdir::new("partition_into_new_directory");
    wrk.create("in.csv", data(true));

    let mut cmd = wrk.command("partition");
    cmd.arg("state").arg(&wrk.path("out")).arg("in.csv");
    wrk.run(&mut cmd);

    assert!(wrk.path("out/NY.csv").exists());
}

#[test]
fn partition_custom_filename() {
    let wrk = Workdir::new("partition_custom_filename");
    wrk.create("in.csv", data(true));

    let mut cmd = wrk.command("partition");
    cmd.args(["--filename", "state-{}-partition.csv"])
        .arg("state")
        .arg(&wrk.path("."))
        .arg("in.csv");
    wrk.run(&mut cmd);

    assert!(wrk.path("state-NY-partition.csv").exists());
}

#[test]
fn partition_custom_filename_with_directory() {
    let wrk = Workdir::new("partition_custom_filename_with_directory");
    wrk.create("in.csv", data(true));

    let mut cmd = wrk.command("partition");
    cmd.args(["--filename", "{}/cities.csv"])
        .arg("state")
        .arg(&wrk.path("."))
        .arg("in.csv");
    wrk.run(&mut cmd);

    // This variation also helps with parallel partition jobs.
    assert!(wrk.path("NY/cities.csv").exists());
}

#[test]
fn partition_invalid_filename() {
    let wrk = Workdir::new("partition_invalid_filename");
    wrk.create("in.csv", data(true));

    let mut cmd = wrk.command("partition");
    cmd.args(["--filename", "foo.csv"])
        .arg("state")
        .arg(&wrk.path("."))
        .arg("in.csv");
    wrk.assert_err(&mut cmd);

    let mut cmd = wrk.command("partition");
    cmd.args(["--filename", "{}{}.csv"])
        .arg("state")
        .arg(&wrk.path("."))
        .arg("in.csv");
    wrk.assert_err(&mut cmd);
}

fn tricky_data() -> Vec<Vec<String>> {
    vec![
        svec!["key", "explanation"],
        svec!["", "empty key"],
        svec!["empty", "the string empty"],
        svec!["unsafe _1$!,\"", "unsafe in shell"],
        svec!["collision", "ordinary value"],
        svec!["collision", "in same file"],
        svec!["coll ision", "collides"],
        svec!["collision!", "collides again"],
        svec!["collision_2", "collides with disambiguated"],
    ]
}

#[test]
fn partition_with_tricky_key_values() {
    let wrk = Workdir::new("partition_with_tricky_key_values");
    wrk.create("in.csv", tricky_data());

    let mut cmd = wrk.command("partition");
    cmd.arg("key").arg(&wrk.path(".")).arg("in.csv");
    wrk.run(&mut cmd);

    part_eq!(
        wrk,
        "empty.csv",
        "\
key,explanation
,empty key
"
    );
    part_eq!(
        wrk,
        "empty_1.csv",
        "\
key,explanation
empty,the string empty
"
    );
    part_eq!(
        wrk,
        "unsafe_1.csv",
        r#"key,explanation
"unsafe _1$!,""",unsafe in shell
"#
    );
    part_eq!(
        wrk,
        "collision.csv",
        "\
key,explanation
collision,ordinary value
collision,in same file
"
    );
    part_eq!(
        wrk,
        "collision_2.csv",
        "\
key,explanation
coll ision,collides
"
    );
    part_eq!(
        wrk,
        "collision_3.csv",
        "\
key,explanation
collision!,collides again
"
    );
    // Tricky! We didn't see this an input, but we did generate it as an
    // output already.
    part_eq!(
        wrk,
        "collision_2_4.csv",
        "\
key,explanation
collision_2,collides with disambiguated
"
    );
}

#[test]
fn partition_with_limit() {
    let wrk = Workdir::new("partition_with_limit");
    // Create data with more unique values than the limit
    let data = vec![
        svec!["state", "city"],
        svec!["NY", "Manhattan"],
        svec!["CA", "San Francisco"],
        svec!["TX", "Dallas"],
        svec!["FL", "Miami"],
        svec!["WA", "Seattle"],
        svec!["NY", "Buffalo"],
        svec!["CA", "Los Angeles"],
        svec!["TX", "Houston"],
        svec!["FL", "Orlando"],
        svec!["WA", "Spokane"],
    ];
    wrk.create("in.csv", data);

    let mut cmd = wrk.command("partition");
    cmd.args(["--limit", "3"])
        .arg("state")
        .arg(&wrk.path("."))
        .arg("in.csv");
    wrk.run(&mut cmd);

    // Should still create all 5 unique state files despite the limit
    assert!(wrk.path("NY.csv").exists());
    assert!(wrk.path("CA.csv").exists());
    assert!(wrk.path("TX.csv").exists());
    assert!(wrk.path("FL.csv").exists());
    assert!(wrk.path("WA.csv").exists());

    // Verify that all rows for each state are properly grouped
    part_eq!(
        wrk,
        "NY.csv",
        "\
state,city
NY,Manhattan
NY,Buffalo
"
    );
    part_eq!(
        wrk,
        "CA.csv",
        "\
state,city
CA,San Francisco
CA,Los Angeles
"
    );
    part_eq!(
        wrk,
        "TX.csv",
        "\
state,city
TX,Dallas
TX,Houston
"
    );
}

fn prefix_data() -> Vec<Vec<String>> {
    vec![
        svec!["state", "city"],
        svec!["MA", "Boston"],
        svec!["ME", "Portland"],
        svec!["M", "Too short"],
        svec!["CA", "San Francisco"],
        svec!["CO", "Denver"],
    ]
}

#[test]
fn partition_with_prefix_length() {
    let wrk = Workdir::new("partition_with_prefix_length");
    wrk.create("in.csv", prefix_data());

    let mut cmd = wrk.command("partition");
    cmd.args(["--prefix-length", "1"])
        .arg("state")
        .arg(&wrk.path("."))
        .arg("in.csv");
    wrk.run(&mut cmd);

    part_eq!(
        wrk,
        "M.csv",
        "\
state,city
MA,Boston
ME,Portland
M,Too short
"
    );
    part_eq!(
        wrk,
        "C.csv",
        "\
state,city
CA,San Francisco
CO,Denver
"
    );
}

#[test]
fn partition_case_insensitive_collision() {
    let wrk = Workdir::new("partition_case_insensitive_collision");
    wrk.create(
        "in.csv",
        vec![
            svec!["state", "city"],
            svec!["CO", "Denver"],
            svec!["Co", "Boulder"],
            svec!["CO", "Colorado Springs"],
            svec!["Co", "Aspen"],
        ],
    );

    let mut cmd = wrk.command("partition");
    cmd.arg("state").arg(&wrk.path(".")).arg("in.csv");
    wrk.run(&mut cmd);

    // On case-insensitive file systems, "CO" and "Co" would collide
    // The fix should create separate files: CO.csv and Co_1.csv
    part_eq!(
        wrk,
        "CO.csv",
        "\
state,city
CO,Denver
CO,Colorado Springs
"
    );
    part_eq!(
        wrk,
        "Co_1.csv",
        "\
state,city
Co,Boulder
Co,Aspen
"
    );
}
