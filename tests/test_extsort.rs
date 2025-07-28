use newline_converter::dos2unix;

use crate::workdir::Workdir;

#[test]
fn extsort_linemode() {
    let wrk = Workdir::new("extsort_linemode").flexible(true);
    wrk.clear_contents().unwrap();

    // copy csv file to workdir
    let unsorted_csv = wrk.load_test_resource("adur-public-toilets.csv");
    wrk.create_from_string("adur-public-toilets.csv", &unsorted_csv);

    let mut cmd = wrk.command("extsort");
    cmd.arg("adur-public-toilets.csv")
        .arg("adur-public-toilets-extsort-test.csv");
    wrk.output(&mut cmd);

    // load sorted output
    let sorted_output: String = wrk.from_str(&wrk.path("adur-public-toilets-extsort-test.csv"));

    let expected_csv = wrk.load_test_resource("adur-public-toilets-sorted.csv");
    wrk.create_from_string("adur-public-toilets-sorted.csv", &expected_csv);

    assert_eq!(dos2unix(&sorted_output), dos2unix(&expected_csv));
}

#[test]
fn extsort_csvmode() {
    let wrk = Workdir::new("extsort_csvmode").flexible(true);
    wrk.clear_contents().unwrap();

    // copy csv file to workdir
    let unsorted_csv = wrk.load_test_resource("adur-public-toilets.csv");
    wrk.create_from_string("adur-public-toilets.csv", &unsorted_csv);

    let mut cmd = wrk.command("extsort");
    cmd.env("QSV_AUTOINDEX_SIZE", "1")
        .arg("adur-public-toilets.csv")
        .args(["--select", "OpeningHours,StreetAddress,LocationText"])
        .arg("adur-public-toilets-extsort-csvmode.csv");
    wrk.output(&mut cmd);

    // load sorted output
    let sorted_output: String = wrk.from_str(&wrk.path("adur-public-toilets-extsort-csvmode.csv"));

    let expected_csv = wrk.load_test_resource("adur-public-toilets-extsorted-csvmode.csv");
    wrk.create_from_string("adur-public-toilets-extsorted-csvmode.csv", &expected_csv);

    assert_eq!(dos2unix(&sorted_output), dos2unix(&expected_csv));
}

#[test]
fn extsort_issue_2391() {
    let wrk = Workdir::new("extsort_issue_2391").flexible(true);
    wrk.clear_contents().unwrap();

    let unsorted_csv = wrk.load_test_resource("issue2391-test_ids.csv");
    wrk.create_from_string("issue2391-test_ids.csv", &unsorted_csv);
    // create index
    let mut cmd_wrk = wrk.command("index");
    cmd_wrk.arg("issue2391-test_ids.csv");

    wrk.assert_success(&mut cmd_wrk);

    // as git mangles line endings, we need to convert manually to CRLF as per issue 2391
    // see https://github.com/dathere/qsv/issues/2391
    // convert LF to CRLF in test file to ensure consistent line endings
    #[cfg(target_os = "windows")]
    {
        let mut cmd = wrk.command("cmd");
        cmd.args([
            "/C",
            "type issue2391-test_ids.csv > issue2391-test_ids.tmp.csv && move /Y \
             issue2391-test_ids.tmp.csv issue2391-test_ids.csv",
        ]);
        wrk.output(&mut cmd);
    }
    #[cfg(not(target_os = "windows"))]
    {
        let mut cmd = wrk.command("sh");
        cmd.args([
            "-c",
            "sed 's/$/\r/' issue2391-test_ids.csv > issue2391-test_ids.tmp.csv && mv \
             issue2391-test_ids.tmp.csv issue2391-test_ids.csv",
        ]);
        wrk.output(&mut cmd);
    }

    let mut cmd = wrk.command("extsort");
    cmd.arg("issue2391-test_ids.csv")
        .args(["--select", "tc_id,pnm,pc_id"]);

    wrk.assert_success(&mut cmd);
    let got: Vec<Vec<String>> = wrk.read_stdout(&mut cmd);
    let expected = vec![
        svec!["pnm", "tc_id", "pc_id"],
        svec!["405", "139280", "9730000630075"],
        svec!["405", "139281", "9730000630075"],
        svec!["252", "139282", "9730000630075"],
        svec!["131", "139282862", "9730065908379"],
        svec!["138", "139282863", "9730065908379"],
        svec!["138", "139282864", "9730065908379"],
        svec!["405", "139282865", "9730065908379"],
        svec!["138", "139282866", "9730065908379"],
        svec!["138", "139282867", "9730065908379"],
        svec!["138", "139282868", "9730065908379"],
        svec!["138", "139282869", "9730065908379"],
        svec!["138", "139282870", "9730065908379"],
        svec!["138", "139282871", "9730065908379"],
        svec!["241", "139283", "9730000630075"],
        svec!["272", "139284", "9730000630075"],
        svec!["273", "139285", "9730000630075"],
    ];
    assert_eq!(got, expected);
}
