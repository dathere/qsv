use crate::workdir::Workdir;

fn fruits(wrk: &Workdir) {
    wrk.create_from_string(
        "fruits.csv",
        "Fruit,Price,Qty\napple,3,10\nbanana,2,20\napple,4,5\ncherry,5,8\nbanana,3,12\n",
    );
}

#[test]
fn viz_bar_html_to_stdout() {
    let wrk = Workdir::new("viz_bar_html_to_stdout");
    fruits(&wrk);

    let mut cmd = wrk.command("viz");
    cmd.args(["bar", "fruits.csv", "--x", "Fruit", "--y", "Price"]);
    let out = wrk.output(&mut cmd);
    assert!(out.status.success());

    let html = String::from_utf8_lossy(&out.stdout);
    // self-contained interactive HTML with a bar trace
    assert!(html.contains("Plotly.newPlot"));
    assert!(html.contains(r#""type":"bar""#));
    assert!(html.contains("apple"));
}

#[test]
fn viz_bar_agg_to_file() {
    let wrk = Workdir::new("viz_bar_agg_to_file");
    fruits(&wrk);

    let out_html = wrk.path("chart.html").to_string_lossy().to_string();
    let mut cmd = wrk.command("viz");
    cmd.args([
        "bar",
        "fruits.csv",
        "--x",
        "Fruit",
        "--y",
        "Price",
        "--agg",
        "sum",
        "-o",
        &out_html,
    ]);
    wrk.assert_success(&mut cmd);

    let html = wrk.read_to_string("chart.html").unwrap();
    assert!(html.contains(r#""type":"bar""#));
    // apple appears once after aggregation (sum of 3 + 4 = 7)
    assert!(html.contains("apple"));
    assert!(html.contains(r#"7.0"#) || html.contains(r#"7,"#));
}

#[test]
fn viz_scatter() {
    let wrk = Workdir::new("viz_scatter");
    fruits(&wrk);

    let mut cmd = wrk.command("viz");
    cmd.args(["scatter", "fruits.csv", "--x", "Qty", "--y", "Price"]);
    let out = wrk.output(&mut cmd);
    assert!(out.status.success());
    let html = String::from_utf8_lossy(&out.stdout);
    assert!(html.contains(r#""type":"scatter""#));
    assert!(html.contains(r#""mode":"markers""#));
}

#[test]
fn viz_histogram() {
    let wrk = Workdir::new("viz_histogram");
    fruits(&wrk);

    let mut cmd = wrk.command("viz");
    cmd.args(["histogram", "fruits.csv", "--x", "Price", "--bins", "5"]);
    let out = wrk.output(&mut cmd);
    assert!(out.status.success());
    let html = String::from_utf8_lossy(&out.stdout);
    assert!(html.contains(r#""type":"histogram""#));
}

#[test]
fn viz_box_grouped() {
    let wrk = Workdir::new("viz_box_grouped");
    fruits(&wrk);

    let mut cmd = wrk.command("viz");
    cmd.args(["box", "fruits.csv", "--y", "Price", "--x", "Fruit"]);
    let out = wrk.output(&mut cmd);
    assert!(out.status.success());
    let html = String::from_utf8_lossy(&out.stdout);
    assert!(html.contains(r#""type":"box""#));
}

#[test]
fn viz_smart_dashboard() {
    let wrk = Workdir::new("viz_smart_dashboard");
    // a mix of: near-unique id (skipped), continuous numeric (box), categorical (bar),
    // boolean (bar)
    // id   -> near-unique Integer (skipped)
    // age  -> continuous Integer, cardinality 50 over 100 rows (box plot)
    // city -> low-cardinality String (frequency bar)
    // active -> boolean (frequency bar)
    let mut rows = String::from("id,age,city,active\n");
    for i in 1..=100 {
        let city = match i % 3 {
            0 => "NYC",
            1 => "LA",
            _ => "SF",
        };
        let active = if i % 2 == 0 { "true" } else { "false" };
        rows.push_str(&format!("{i},{},{city},{active}\n", 20 + i % 50));
    }
    wrk.create_from_string("people.csv", &rows);

    let out_html = wrk.path("dash.html").to_string_lossy().to_string();
    let mut cmd = wrk.command("viz");
    cmd.args(["smart", "people.csv", "-o", &out_html]);
    wrk.assert_success(&mut cmd);

    let html = wrk.read_to_string("dash.html").unwrap();
    // a subplot grid dashboard with at least one box (continuous) and one bar (categorical)
    assert!(html.contains(r#""grid":{"#));
    assert!(html.contains(r#""pattern":"independent""#));
    assert!(html.contains(r#""type":"box""#));
    assert!(html.contains(r#""type":"bar""#));
}

#[test]
fn viz_smart_caps_charts() {
    let wrk = Workdir::new("viz_smart_caps_charts");
    // four low-cardinality categorical columns (all chartable as frequency bars)
    wrk.create_from_string(
        "d.csv",
        "c1,c2,c3,c4\na,x,p,m\nb,y,q,n\na,x,p,m\nb,y,q,n\na,x,p,m\nb,y,q,n\n",
    );

    let out_html = wrk.path("d.html").to_string_lossy().to_string();
    let mut cmd = wrk.command("viz");
    cmd.args(["smart", "d.csv", "--max-charts", "2", "-o", &out_html]);
    let out = wrk.output(&mut cmd);
    assert!(out.status.success());
    // capped at 2 of the 4 eligible columns; the skip notice is emitted to stderr
    let stderr = String::from_utf8_lossy(&out.stderr);
    assert!(stderr.contains("viz smart: charting 2 column(s)"));
}

#[test]
fn viz_missing_y_errors() {
    let wrk = Workdir::new("viz_missing_y_errors");
    fruits(&wrk);

    let mut cmd = wrk.command("viz");
    cmd.args(["bar", "fruits.csv", "--x", "Fruit"]);
    let out = wrk.output(&mut cmd);
    assert!(!out.status.success());
    let stderr = String::from_utf8_lossy(&out.stderr);
    assert!(stderr.contains("--y is required"));
}

#[test]
fn viz_bad_extension_errors() {
    let wrk = Workdir::new("viz_bad_extension_errors");
    fruits(&wrk);

    let out_path = wrk.path("chart.txt").to_string_lossy().to_string();
    let mut cmd = wrk.command("viz");
    cmd.args([
        "bar",
        "fruits.csv",
        "--x",
        "Fruit",
        "--y",
        "Price",
        "-o",
        &out_path,
    ]);
    let out = wrk.output(&mut cmd);
    assert!(!out.status.success());
    let stderr = String::from_utf8_lossy(&out.stderr);
    assert!(stderr.contains("Unsupported output extension"));
}

// Without the viz_static feature, requesting an image format is a clear, actionable error.
#[cfg(not(feature = "viz_static"))]
#[test]
fn viz_image_without_static_feature_errors() {
    let wrk = Workdir::new("viz_image_without_static_feature_errors");
    fruits(&wrk);

    let out_png = wrk.path("chart.png").to_string_lossy().to_string();
    let mut cmd = wrk.command("viz");
    cmd.args([
        "bar",
        "fruits.csv",
        "--x",
        "Fruit",
        "--y",
        "Price",
        "-o",
        &out_png,
    ]);
    let out = wrk.output(&mut cmd);
    assert!(!out.status.success());
    let stderr = String::from_utf8_lossy(&out.stderr);
    assert!(stderr.contains("viz_static"));
}

// Static PNG export. Requires a Chromium/Firefox browser + webdriver at runtime, which CI
// runners typically lack, so this is ignored by default; run with `--ignored` locally.
#[cfg(feature = "viz_static")]
#[test]
#[ignore = "requires a browser/webdriver for plotly static export"]
fn viz_static_png_export() {
    let wrk = Workdir::new("viz_static_png_export");
    fruits(&wrk);

    let out_png = wrk.path("chart.png").to_string_lossy().to_string();
    let mut cmd = wrk.command("viz");
    cmd.args([
        "bar",
        "fruits.csv",
        "--x",
        "Fruit",
        "--y",
        "Price",
        "-o",
        &out_png,
    ]);
    wrk.assert_success(&mut cmd);

    let bytes = std::fs::read(wrk.path("chart.png")).unwrap();
    // PNG magic number
    assert_eq!(&bytes[..4], b"\x89PNG");
}
