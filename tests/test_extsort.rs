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

#[test]
fn extsort_csvmode_no_headers() {
    // Guards the no-headers CSV-mode path: every input row must appear
    // exactly once in the output. Previously, hard-coding position_delta=1
    // duplicated the first row and dropped the last.
    let wrk = Workdir::new("extsort_csvmode_no_headers").flexible(true);
    wrk.clear_contents().unwrap();

    let csv = "9\n2\n5\n1\n7\n4\n8\n3\n6\n";
    wrk.create_from_string("nh.csv", csv);

    let mut idx_cmd = wrk.command("index");
    idx_cmd.arg("nh.csv");
    wrk.assert_success(&mut idx_cmd);

    let mut cmd = wrk.command("extsort");
    cmd.arg("nh.csv")
        .args(["--select", "1"])
        .arg("--no-headers");

    let got: Vec<Vec<String>> = wrk.read_stdout(&mut cmd);
    let expected = vec![
        svec!["1"],
        svec!["2"],
        svec!["3"],
        svec!["4"],
        svec!["5"],
        svec!["6"],
        svec!["7"],
        svec!["8"],
        svec!["9"],
    ];
    assert_eq!(got, expected);
}

#[test]
fn extsort_csvmode_crlf_no_headers() {
    // Guards CRLF + no-headers + CSV mode: combines the two paths the
    // earlier off-by-one fixes regressed individually.
    let wrk = Workdir::new("extsort_csvmode_crlf_no_headers").flexible(true);
    wrk.clear_contents().unwrap();

    let csv = "9\r\n2\r\n5\r\n1\r\n7\r\n4\r\n8\r\n3\r\n6\r\n";
    wrk.create_from_string("nh_crlf.csv", csv);

    let mut idx_cmd = wrk.command("index");
    idx_cmd.arg("nh_crlf.csv");
    wrk.assert_success(&mut idx_cmd);

    let mut cmd = wrk.command("extsort");
    cmd.arg("nh_crlf.csv")
        .args(["--select", "1"])
        .arg("--no-headers");

    let got: Vec<Vec<String>> = wrk.read_stdout(&mut cmd);
    let expected = vec![
        svec!["1"],
        svec!["2"],
        svec!["3"],
        svec!["4"],
        svec!["5"],
        svec!["6"],
        svec!["7"],
        svec!["8"],
        svec!["9"],
    ];
    assert_eq!(got, expected);
}
