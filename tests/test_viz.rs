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
    // single-series bar charts get SI-formatted value labels above each bar
    assert!(html.contains(r#""texttemplate":"%{y:.3s}""#));
    assert!(html.contains(r#""textposition":"outside""#));
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
    // a multi-panel dashboard: explicit row-scaled height, per-cell axis domains, and a
    // title annotation above each panel, with at least one box (continuous) and one bar
    // (categorical)
    assert!(html.contains(r#""height":"#));
    assert!(html.contains(r#""annotations":["#));
    assert!(html.contains(r#""xaxis2":{"#));
    assert!(html.contains(r#""domain":["#));
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

#[test]
fn viz_pie() {
    let wrk = Workdir::new("viz_pie");
    fruits(&wrk);

    let mut cmd = wrk.command("viz");
    // count occurrences of each Fruit label, rendered as a donut
    cmd.args(["pie", "fruits.csv", "--x", "Fruit", "--donut"]);
    let out = wrk.output(&mut cmd);
    assert!(out.status.success());

    let html = String::from_utf8_lossy(&out.stdout);
    assert!(html.contains(r#""type":"pie""#));
    assert!(html.contains(r#""hole":0.4"#));
    assert!(html.contains("apple"));
}

#[test]
fn viz_heatmap_correlation() {
    let wrk = Workdir::new("viz_heatmap_correlation");
    // three numeric columns with repetition (low uniqueness, not ID-like)
    let mut rows = String::from("a,b,c\n");
    for i in 0..40 {
        let a = i % 7;
        let b = (i % 7) * 2; // perfectly correlated with a
        let c = (i % 5) + 1;
        rows.push_str(&format!("{a},{b},{c}\n"));
    }
    wrk.create_from_string("nums.csv", &rows);

    let mut cmd = wrk.command("viz");
    cmd.args(["heatmap", "nums.csv"]);
    let out = wrk.output(&mut cmd);
    assert!(out.status.success());

    let html = String::from_utf8_lossy(&out.stdout);
    assert!(html.contains(r#""type":"heatmap""#));
    // correlation heatmaps are fixed to the [-1, 1] diverging scale
    assert!(html.contains(r#""zmin":-1.0"#));
    assert!(html.contains(r#""zmax":1.0"#));
}

#[test]
fn viz_heatmap_correlation_constant_column() {
    // a zero-variance (constant) column has an undefined correlation: it must serialize as
    // null (a heatmap gap), never a fabricated 0.0 or 1.0. Column `b` is constant; a vs c is a
    // perfect negative correlation.
    let wrk = Workdir::new("viz_heatmap_correlation_constant_column");
    wrk.create_from_string("c.csv", "a,b,c\n1,5,9\n2,5,8\n3,5,7\n4,5,6\n");

    let mut cmd = wrk.command("viz");
    cmd.args(["heatmap", "c.csv"]);
    let out = wrk.output(&mut cmd);
    assert!(out.status.success());

    let html = String::from_utf8_lossy(&out.stdout);
    assert!(html.contains(r#""type":"heatmap""#));

    // verify the actual correlation matrix, not just that "null"/"-1.0" appear somewhere
    // (e.g. "-1.0" is also the configured zmin). Columns are [a, b, c]; b is constant.
    let z = extract_z_matrix(&html);
    assert_eq!(z.len(), 3);
    assert!(z.iter().all(|row| row.len() == 3));
    // the constant column b (index 1) has undefined correlation everywhere — its entire row
    // AND column are null, including its own diagonal (no fabricated 1.0)
    assert!(z[1].iter().all(Option::is_none), "row b should be all null");
    assert!(
        z.iter().all(|row| row[1].is_none()),
        "col b should be all null"
    );
    // a and c are perfectly anti-correlated; diagonals are 1, a vs c is -1 (within FP tolerance)
    assert!((z[0][0].unwrap() - 1.0).abs() < 1e-9);
    assert!((z[2][2].unwrap() - 1.0).abs() < 1e-9);
    assert!((z[0][2].unwrap() + 1.0).abs() < 1e-9);
    assert!((z[2][0].unwrap() + 1.0).abs() < 1e-9);
}

/// Extract the heatmap trace's `z` matrix from the embedded plotly JSON in the HTML output,
/// matching the balanced brackets after `"z":`. `null` cells parse to `None`.
fn extract_z_matrix(html: &str) -> Vec<Vec<Option<f64>>> {
    let start = html.find(r#""z":["#).expect("z array present") + 4;
    let bytes = html.as_bytes();
    let mut depth = 0_i32;
    let mut end = start;
    for (i, &b) in bytes[start..].iter().enumerate() {
        match b {
            b'[' => depth += 1,
            b']' => {
                depth -= 1;
                if depth == 0 {
                    end = start + i + 1;
                    break;
                }
            },
            _ => {},
        }
    }
    serde_json::from_str(&html[start..end]).expect("valid z matrix json")
}

#[test]
fn viz_heatmap_correlation_large_values() {
    // regression: large-but-valid variances must not overflow the Pearson denominator. With
    // the old `(var_x * var_y).sqrt()` these identical columns overflowed to infinity and
    // yielded NaN/null; the fix `var_x.sqrt() * var_y.sqrt()` stays finite -> perfect 1.0.
    let wrk = Workdir::new("viz_heatmap_correlation_large_values");
    wrk.create_from_string("big.csv", "a,b\n0,0\n1e100,1e100\n");

    let mut cmd = wrk.command("viz");
    cmd.args(["heatmap", "big.csv"]);
    let out = wrk.output(&mut cmd);
    assert!(out.status.success());

    let html = String::from_utf8_lossy(&out.stdout);
    let z = extract_z_matrix(&html);
    // assert the exact 2x2 shape so the cell checks below aren't vacuously true on empty rows
    assert_eq!(z.len(), 2, "expected 2 rows, got {z:?}");
    assert!(
        z.iter().all(|row| row.len() == 2),
        "expected 2x2, got {z:?}"
    );
    // every cell (incl. the a-vs-b off-diagonal) is a finite, perfect correlation, not null
    for (r, row) in z.iter().enumerate() {
        for (c, cell) in row.iter().enumerate() {
            let v = cell.unwrap_or_else(|| panic!("z[{r}][{c}] is null, expected 1.0; got {z:?}"));
            assert!((v - 1.0).abs() < 1e-9, "z[{r}][{c}] = {v}, expected 1.0");
        }
    }
}

#[test]
fn viz_heatmap_correlation_insufficient_rows_errors() {
    // fewer than 2 rows where all selected numeric columns are present => cannot correlate
    let wrk = Workdir::new("viz_heatmap_correlation_insufficient_rows_errors");
    wrk.create_from_string("one.csv", "a,b\n1,2\n");

    let mut cmd = wrk.command("viz");
    cmd.args(["heatmap", "one.csv"]);
    let out = wrk.output(&mut cmd);
    assert!(!out.status.success());
    let stderr = String::from_utf8_lossy(&out.stderr);
    assert!(stderr.contains("at least 2 rows"));
}

#[test]
fn viz_heatmap_pivot() {
    let wrk = Workdir::new("viz_heatmap_pivot");
    wrk.create_from_string(
        "sales.csv",
        "region,product,amount\nEast,Widget,100\nWest,Widget,150\nEast,Gadget,80\nWest,Gadget,90\n",
    );

    let mut cmd = wrk.command("viz");
    cmd.args([
        "heatmap",
        "sales.csv",
        "--x",
        "region",
        "--y",
        "product",
        "--z",
        "amount",
    ]);
    let out = wrk.output(&mut cmd);
    assert!(out.status.success());

    let html = String::from_utf8_lossy(&out.stdout);
    assert!(html.contains(r#""type":"heatmap""#));
    assert!(html.contains("Widget"));
}

#[test]
fn viz_candlestick() {
    let wrk = Workdir::new("viz_candlestick");
    wrk.create_from_string(
        "prices.csv",
        "date,open,high,low,close\n2024-01-01,10,12,9,11\n2024-01-02,11,13,10,12\n2024-01-03,12,\
         14,11,13\n",
    );

    let mut cmd = wrk.command("viz");
    cmd.args([
        "candlestick",
        "prices.csv",
        "--x",
        "date",
        "--ohlc-open",
        "open",
        "--high",
        "high",
        "--low",
        "low",
        "--close",
        "close",
    ]);
    let out = wrk.output(&mut cmd);
    assert!(out.status.success());

    let html = String::from_utf8_lossy(&out.stdout);
    assert!(html.contains(r#""type":"candlestick""#));
}

#[test]
fn viz_ohlc() {
    let wrk = Workdir::new("viz_ohlc");
    wrk.create_from_string(
        "prices.csv",
        "date,open,high,low,close\n2024-01-01,10,12,9,11\n2024-01-02,11,13,10,12\n",
    );

    let mut cmd = wrk.command("viz");
    cmd.args([
        "ohlc",
        "prices.csv",
        "--x",
        "date",
        "--ohlc-open",
        "open",
        "--high",
        "high",
        "--low",
        "low",
        "--close",
        "close",
    ]);
    let out = wrk.output(&mut cmd);
    assert!(out.status.success());

    let html = String::from_utf8_lossy(&out.stdout);
    assert!(html.contains(r#""type":"ohlc""#));
}

#[test]
fn viz_sankey() {
    let wrk = Workdir::new("viz_sankey");
    // two rows share the same East->Widget flow; they must aggregate into one link
    wrk.create_from_string(
        "flows.csv",
        "from,to,weight\nEast,Widget,5\nEast,Widget,3\nWest,Gadget,4\n",
    );

    let mut cmd = wrk.command("viz");
    cmd.args([
        "sankey",
        "flows.csv",
        "--source",
        "from",
        "--target",
        "to",
        "--value",
        "weight",
    ]);
    let out = wrk.output(&mut cmd);
    assert!(out.status.success());

    let html = String::from_utf8_lossy(&out.stdout);
    assert!(html.contains(r#""type":"sankey""#));
    // East->Widget (5+3=8) and West->Gadget (4): exactly two aggregated links
    assert!(html.contains(r#""value":[8.0,4.0]"#));
}

#[test]
fn viz_radar() {
    let wrk = Workdir::new("viz_radar");
    wrk.create_from_string(
        "teams.csv",
        "team,speed,power,range\nAlpha,80,70,60\nBeta,60,85,75\nAlpha,82,72,64\n",
    );

    let mut cmd = wrk.command("viz");
    cmd.args([
        "radar",
        "teams.csv",
        "--cols",
        "speed,power,range",
        "--series",
        "team",
    ]);
    let out = wrk.output(&mut cmd);
    assert!(out.status.success());

    let html = String::from_utf8_lossy(&out.stdout);
    assert!(html.contains(r#""type":"scatterpolar""#));
    assert!(html.contains(r#""fill":"toself""#));
    assert!(html.contains("Alpha"));
}

#[test]
fn viz_smart_correlation_panel() {
    let wrk = Workdir::new("viz_smart_correlation_panel");
    // two continuous-but-repeating numeric columns (not near-unique) plus a categorical one,
    // so `viz smart` adds a correlation heatmap panel alongside the frequency bar
    let mut rows = String::from("metric_a,metric_b,city\n");
    for i in 0..60 {
        let a = i % 9;
        let b = (i % 9) + (i % 3);
        let city = match i % 3 {
            0 => "NYC",
            1 => "LA",
            _ => "SF",
        };
        rows.push_str(&format!("{a},{b},{city}\n"));
    }
    wrk.create_from_string("metrics.csv", &rows);

    let out_html = wrk.path("dash.html").to_string_lossy().to_string();
    let mut cmd = wrk.command("viz");
    cmd.args(["smart", "metrics.csv", "-o", &out_html]);
    wrk.assert_success(&mut cmd);

    let html = wrk.read_to_string("dash.html").unwrap();
    // the correlation panel is a heatmap drawn into the subplot grid
    assert!(html.contains(r#""type":"heatmap""#));
    // polish: a clean hovertemplate (drops plotly's default "trace 0") ...
    assert!(html.contains("hovertemplate"));
    // ... in-cell r value labels as annotations (metric_a vs metric_a is a perfect 1.00) ...
    assert!(html.contains(r#""text":"1.00""#));
    // ... and a widened left margin (> the default 60) so long y tick labels aren't clipped.
    // "metric_a" is 8 chars => 8*7 + 24 = 80px.
    assert!(html.contains(r#""l":80"#));
}
