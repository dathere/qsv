use uuid::Uuid;

use crate::workdir::Workdir;

fn extract_attr(xml: &str, tag: &str, attr: &str) -> Option<String> {
    let tag_start = xml.find(&format!("<{tag}"))?;
    let rel_tag_end = xml[tag_start..].find('>')?;
    let tag_end = tag_start + rel_tag_end;
    let tag_text = &xml[tag_start..=tag_end];
    let needle = format!("{attr}=\"");
    let attr_start = tag_text.find(&needle)? + needle.len();
    let attr_end = tag_text[attr_start..].find('"')?;
    Some(tag_text[attr_start..attr_start + attr_end].to_string())
}

fn seed_basic_csv(wrk: &Workdir) {
    wrk.create(
        "in.csv",
        vec![
            svec!["city", "kind"],
            svec!["Boston", "store"],
            svec!["Austin", "office"],
            svec!["Seattle", "store"],
        ],
    );
}

#[test]
fn profile_ddic_emits_required_identifiers_and_metadata() {
    let wrk = Workdir::new("profile_ddic_ids");
    seed_basic_csv(&wrk);

    let mut cmd = wrk.command("profile");
    cmd.args(["in.csv", "--ddi-c", "out.xml", "-o", "out.json"]);
    wrk.assert_success(&mut cmd);

    let xml = wrk.read_to_string("out.xml").expect("read ddic xml");

    let codebook_id = extract_attr(&xml, "codeBook", "ID").expect("codeBook/@ID missing");
    assert!(
        codebook_id.starts_with('_'),
        "codeBook/@ID should start with an underscore: {codebook_id}"
    );
    assert!(
        Uuid::parse_str(&codebook_id[1..]).is_ok(),
        "codeBook/@ID suffix is not a UUID: {codebook_id}"
    );

    assert!(xml.contains("<fileDscr ID=\"F1\">"));
    assert!(xml.contains("<var ID=\"V1\""));
    assert!(xml.contains("<var ID=\"V2\""));

    assert!(
        xml.contains("<titl>in.csv</titl>"),
        "expected stdyDscr title from input filename"
    );

    let expected_software = format!(
        "<software version=\"{}\">qsv</software>",
        env!("CARGO_PKG_VERSION")
    );
    assert!(xml.contains(&expected_software), "missing qsv software citation");
}

#[test]
fn profile_ddic_zero_limit_emits_no_categories() {
    let wrk = Workdir::new("profile_ddic_zero_limit");
    seed_basic_csv(&wrk);

    let mut cmd = wrk.command("profile");
    cmd.args([
        "in.csv",
        "--ddi-c",
        "out.xml",
        "--ddi-catgry-limit",
        "0",
        "-o",
        "out.json",
    ]);
    wrk.assert_success(&mut cmd);

    let xml = wrk.read_to_string("out.xml").expect("read ddic xml");
    assert!(
        !xml.contains("<catgry>"),
        "expected no categories when --ddi-catgry-limit is 0"
    );
}

#[test]
fn profile_ddic_json_override_skips_high_cardinality_variable() {
    let wrk = Workdir::new("profile_ddic_json_override_skip");
    wrk.create(
        "in.csv",
        vec![
            svec!["kind"],
            svec!["store"],
            svec!["store"],
            svec!["office"],
            svec!["warehouse"],
        ],
    );

    let mut cmd = wrk.command("profile");
    cmd.args([
        "in.csv",
        "--ddi-c",
        "out.xml",
        "--ddi-catgry-limit",
        "{\"kind\":2}",
        "-o",
        "out.json",
    ]);
    wrk.assert_success(&mut cmd);

    let xml = wrk.read_to_string("out.xml").expect("read ddic xml");
    assert!(
        !xml.contains("<catgry>"),
        "expected high-cardinality variable categories to be omitted"
    );
}

#[test]
fn profile_ddic_json_file_override_is_supported() {
    let wrk = Workdir::new("profile_ddic_json_file_override");
    wrk.create(
        "in.csv",
        vec![
            svec!["kind"],
            svec!["store"],
            svec!["store"],
            svec!["office"],
            svec!["warehouse"],
        ],
    );
    wrk.create_from_string("limits.json", "{\"kind\": 3}\n");

    let mut cmd = wrk.command("profile");
    cmd.args([
        "in.csv",
        "--ddi-c",
        "out.xml",
        "--ddi-catgry-limit",
        "limits.json",
        "-o",
        "out.json",
    ]);
    wrk.assert_success(&mut cmd);

    let xml = wrk.read_to_string("out.xml").expect("read ddic xml");
    let catgry_count = xml.matches("<catgry>").count();
    assert_eq!(catgry_count, 3, "expected all 3 categories to be emitted");
}

#[test]
fn profile_ddic_emits_sumstats_var_attrs_and_varformat_after_categories() {
    let wrk = Workdir::new("profile_ddic_sumstats_and_order");
    wrk.create(
        "in.csv",
        vec![
            svec!["score", "kind"],
            svec!["1.5", "store"],
            svec!["2.5", "store"],
            svec!["3.5", "office"],
        ],
    );

    let mut cmd = wrk.command("profile");
    cmd.args(["in.csv", "--ddi-c", "out.xml", "-o", "out.json"]);
    wrk.assert_success(&mut cmd);

    let xml = wrk.read_to_string("out.xml").expect("read ddic xml");

    assert!(
        xml.contains("xmlns=\"http://www.icpsr.umich.edu/DDI\""),
        "expected DDI namespace on root"
    );
    assert!(
        xml.contains("<sumStat type=\"mean\">2.5</sumStat>"),
        "expected numeric mean sumStat"
    );
    assert!(
        xml.contains("<sumStat type=\"invd\">0</sumStat>"),
        "expected invalid count sumStat"
    );
    assert!(
        xml.contains("<sumStat type=\"vald\">3</sumStat>"),
        "expected valid count sumStat"
    );
    assert!(
        xml.contains("<var ID=\"V1\" name=\"score\" files=\"F1\" intrvl=\"contin\" representationType=\"numeric\""),
        "expected enriched var attributes for numeric column"
    );
    assert!(
        xml.contains("<catStat type=\"percent\">"),
        "expected category percent stat"
    );
    assert!(
        xml.contains("<varFormat type=\"numeric\" schema=\"other\" otherSchema=\"qsv\" formatname=\"Float\"/>"),
        "expected DDI-compliant varFormat schema attributes"
    );

    let kind_var_start = xml.find("<var ID=\"V2\"").expect("V2 var missing");
    let kind_var_end = xml[kind_var_start..]
        .find("</var>")
        .map(|idx| kind_var_start + idx)
        .expect("V2 var end missing");
    let kind_var = &xml[kind_var_start..kind_var_end];
    let catgry_pos = kind_var.find("<catgry>").expect("catgry missing in V2");
    let varformat_pos = kind_var
        .find("<varFormat")
        .expect("varFormat missing in V2");
    assert!(
        varformat_pos > catgry_pos,
        "expected varFormat to appear after catgry"
    );
}

#[test]
fn profile_ddic_keeps_categorical_classification_when_catgry_omitted_by_limit() {
    let wrk = Workdir::new("profile_ddic_classify_categorical_without_catgry_output");
    wrk.create(
        "in.csv",
        vec![
            svec!["kind"],
            svec!["store"],
            svec!["store"],
            svec!["office"],
            svec!["office"],
        ],
    );

    let mut cmd = wrk.command("profile");
    cmd.args([
        "in.csv",
        "--ddi-c",
        "out.xml",
        "--ddi-catgry-limit",
        "1",
        "-o",
        "out.json",
    ]);
    wrk.assert_success(&mut cmd);

    let xml = wrk.read_to_string("out.xml").expect("read ddic xml");
    assert!(
        !xml.contains("<catgry>"),
        "expected categories to be omitted with low category limit"
    );
    assert!(
        xml.contains("<var ID=\"V1\" name=\"kind\" files=\"F1\" intrvl=\"discrete\" representationType=\"coded\""),
        "expected categorical variable to remain coded even when catgry output is omitted"
    );
}

#[test]
fn profile_ddic_classifies_high_cardinality_string_as_non_categorical() {
    let wrk = Workdir::new("profile_ddic_classify_high_card_string_as_text");
    let mut csv = String::from("id\n");
    for i in 1..=51 {
        csv.push_str(&format!("id_{i:03}\n"));
    }
    wrk.create_from_string("in.csv", &csv);

    let mut cmd = wrk.command("profile");
    cmd.args(["in.csv", "--ddi-c", "out.xml", "-o", "out.json"]);
    wrk.assert_success(&mut cmd);

    let xml = wrk.read_to_string("out.xml").expect("read ddic xml");
    assert!(
        xml.contains("<var ID=\"V1\" name=\"id\" files=\"F1\" intrvl=\"discrete\" representationType=\"text\""),
        "expected high-cardinality string variable to be non-categorical text"
    );
}

#[test]
fn profile_ddic_uses_schema_enum_signal_for_integer_categorical_detection() {
    let wrk = Workdir::new("profile_ddic_schema_enum_signal_integer");
    wrk.create(
        "in.csv",
        vec![
            svec!["status_code"],
            svec!["1"],
            svec!["2"],
            svec!["1"],
            svec!["3"],
            svec!["2"],
        ],
    );

    let mut cmd = wrk.command("profile");
    cmd.args([
        "in.csv",
        "--ddi-c",
        "out.xml",
        "--ddi-catgry-limit",
        "1",
        "-o",
        "out.json",
    ]);
    wrk.assert_success(&mut cmd);

    let xml = wrk.read_to_string("out.xml").expect("read ddic xml");
    assert!(
        xml.contains("<var ID=\"V1\" name=\"status_code\" files=\"F1\" intrvl=\"discrete\" representationType=\"coded\""),
        "expected enum-like integer column to be classified as coded categorical"
    );
}

#[test]
fn profile_ddic_does_not_emit_categories_for_continuous_numeric_weights() {
    let wrk = Workdir::new("profile_ddic_no_catgry_for_continuous_weights");
    wrk.create(
        "in.csv",
        vec![
            svec!["hhweight"],
            svec!["156.66755315036724"],
            svec!["156.66755315036724"],
            svec!["269.65684351914035"],
            svec!["269.65684351914035"],
            svec!["302.41537022900758"],
            svec!["357.69991350549861"],
        ],
    );

    let mut cmd = wrk.command("profile");
    cmd.args(["in.csv", "--ddi-c", "out.xml", "-o", "out.json"]);
    wrk.assert_success(&mut cmd);

    let xml = wrk.read_to_string("out.xml").expect("read ddic xml");
    assert!(
        xml.contains("<var ID=\"V1\" name=\"hhweight\" files=\"F1\" intrvl=\"contin\" representationType=\"numeric\""),
        "expected continuous numeric classification for hhweight"
    );
    assert!(
        !xml.contains("<catgry>"),
        "expected no categories for continuous non-categorical numeric weight"
    );
}
