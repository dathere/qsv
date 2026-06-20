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
fn viz_scatter_bubble_size() {
    let wrk = Workdir::new("viz_scatter_bubble_size");
    fruits(&wrk);

    let mut cmd = wrk.command("viz");
    cmd.args([
        "scatter",
        "fruits.csv",
        "--x",
        "Qty",
        "--y",
        "Price",
        "--size",
        "Qty",
    ]);
    let out = wrk.output(&mut cmd);
    assert!(out.status.success());
    let html = String::from_utf8_lossy(&out.stdout);
    assert!(html.contains(r#""type":"scatter""#));
    // --size rescales the raw values into a readable pixel range, so the marker carries a
    // per-point size array (not a scalar)
    assert!(html.contains(r#""marker":{"size":["#));
}

#[test]
fn viz_scatter_color_scale() {
    let wrk = Workdir::new("viz_scatter_color_scale");
    fruits(&wrk);

    let mut cmd = wrk.command("viz");
    cmd.args([
        "scatter",
        "fruits.csv",
        "--x",
        "Qty",
        "--y",
        "Price",
        "--color",
        "Price",
    ]);
    let out = wrk.output(&mut cmd);
    assert!(out.status.success());
    let html = String::from_utf8_lossy(&out.stdout);
    // --color maps a numeric column onto a continuous colorscale with a colorbar
    assert!(html.contains(r#""colorscale":"Viridis""#));
    assert!(html.contains(r#""showscale":true"#));
    assert!(html.contains(r#""colorbar":{"title":{"text":"Price"#));
}

#[test]
fn viz_scatter_color_size_with_series_errors() {
    let wrk = Workdir::new("viz_scatter_color_size_with_series_errors");
    fruits(&wrk);

    let mut cmd = wrk.command("viz");
    cmd.args([
        "scatter",
        "fruits.csv",
        "--x",
        "Qty",
        "--y",
        "Price",
        "--size",
        "Qty",
        "--series",
        "Fruit",
    ]);
    let out = wrk.output(&mut cmd);
    assert!(!out.status.success());
    let stderr = wrk.output_stderr(&mut cmd);
    assert!(stderr.contains("cannot be combined with --series"));
}

#[test]
fn viz_color_size_non_scatter_errors() {
    let wrk = Workdir::new("viz_color_size_non_scatter_errors");
    fruits(&wrk);

    let mut cmd = wrk.command("viz");
    cmd.args([
        "bar",
        "fruits.csv",
        "--x",
        "Fruit",
        "--y",
        "Price",
        "--color",
        "Price",
    ]);
    let out = wrk.output(&mut cmd);
    assert!(!out.status.success());
    let stderr = wrk.output_stderr(&mut cmd);
    assert!(stderr.contains("only apply to `viz scatter`"));
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
fn viz_box_tukey_outliers_default() {
    let wrk = Workdir::new("viz_box_tukey_outliers_default");
    fruits(&wrk);

    let mut cmd = wrk.command("viz");
    cmd.args(["box", "fruits.csv", "--y", "Price"]);
    let out = wrk.output(&mut cmd);
    assert!(out.status.success());
    let html = String::from_utf8_lossy(&out.stdout);
    assert!(html.contains(r#""type":"box""#));
    // explicit `viz box` reads raw data, so it draws true Tukey whiskers (linear
    // quartile method) and shows the points beyond the 1.5*IQR fences as outliers
    assert!(html.contains(r#""boxpoints":"outliers""#));
    assert!(html.contains(r#""quartilemethod":"linear""#));
}

#[test]
fn viz_box_points_all() {
    let wrk = Workdir::new("viz_box_points_all");
    fruits(&wrk);

    let mut cmd = wrk.command("viz");
    cmd.args(["box", "fruits.csv", "--y", "Price", "--box-points", "all"]);
    let out = wrk.output(&mut cmd);
    assert!(out.status.success());
    let html = String::from_utf8_lossy(&out.stdout);
    assert!(html.contains(r#""boxpoints":"all""#));
}

#[test]
fn viz_box_points_invalid_errors() {
    let wrk = Workdir::new("viz_box_points_invalid_errors");
    fruits(&wrk);

    let mut cmd = wrk.command("viz");
    cmd.args(["box", "fruits.csv", "--y", "Price", "--box-points", "bogus"]);
    let out = wrk.output(&mut cmd);
    assert!(!out.status.success());
    let stderr = wrk.output_stderr(&mut cmd);
    assert!(stderr.contains("Unknown --box-points"));
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
fn viz_smart_uses_moarstats_box_hints() {
    // End-to-end: when `moarstats` has extended the stats cache, `viz smart` reuses that cache
    // (rather than regenerating a base-stats one) and annotates a continuous column's box panel
    // with the moarstats shape stats — skew direction and the outlier share.
    let wrk = Workdir::new("viz_smart_uses_moarstats_box_hints");
    // `amount`: a continuous, right-skewed Integer column (cardinality 41, not near-unique) with
    // a heavy right tail of 1000s -> box plot with positive Pearson skewness and ~6.7% outliers.
    let mut rows = String::from("id,amount\n");
    for i in 1..=280 {
        rows.push_str(&format!("{i},{}\n", i % 40 + 1));
    }
    for i in 281..=300 {
        rows.push_str(&format!("{i},1000\n"));
    }
    wrk.create_from_string("amounts.csv", &rows);

    // 1) extend the stats cache with moarstats (adds pearson_skewness, outliers_percentage, ...)
    let mut moar = wrk.command("moarstats");
    moar.arg("amounts.csv");
    wrk.assert_success(&mut moar);

    // 2) viz smart should reuse that cache and surface the hints in the box panel title
    let out_html = wrk.path("amounts.html").to_string_lossy().to_string();
    let mut cmd = wrk.command("viz");
    cmd.args(["smart", "amounts.csv", "-o", &out_html]);
    wrk.assert_success(&mut cmd);

    let html = wrk.read_to_string("amounts.html").unwrap();
    assert!(
        html.contains("right-skewed"),
        "box panel title should carry the moarstats skew hint; html: {html}"
    );
    assert!(
        html.contains("% outliers"),
        "box panel title should carry the moarstats outlier-share hint; html: {html}"
    );
    assert!(html.contains(r#""type":"box""#));
}

#[test]
fn viz_smart_smarter_promotes_bimodal_to_histogram() {
    // `viz smart --smarter` runs `qsv moarstats --advanced` itself (no manual prior step), so the
    // bimodality_coefficient is populated and a clearly-bimodal continuous column is rendered as a
    // histogram instead of a box plot. Without --smarter the same column would be a box plot.
    let wrk = Workdir::new("viz_smart_smarter_promotes_bimodal_to_histogram");
    // `measure`: two well-separated clusters (0..39 and 1000..1039), 150 rows each. Cardinality 80
    // (> CATEGORICAL_MAX_CARDINALITY=30, so it takes the continuous branch, not a freq bar) and a
    // symmetric two-peak shape -> bimodality coefficient comfortably above the 0.555 threshold.
    let mut rows = String::from("id,measure\n");
    let mut id = 1;
    for v in 0..150 {
        rows.push_str(&format!("{id},{}\n", v % 40));
        id += 1;
    }
    for v in 0..150 {
        rows.push_str(&format!("{id},{}\n", 1000 + v % 40));
        id += 1;
    }
    wrk.create_from_string("bimodal.csv", &rows);

    let out_html = wrk.path("bimodal.html").to_string_lossy().to_string();
    let mut cmd = wrk.command("viz");
    cmd.args(["smart", "bimodal.csv", "--smarter", "-o", &out_html]);
    wrk.assert_success(&mut cmd);

    let html = wrk.read_to_string("bimodal.html").unwrap();
    assert!(
        html.contains(r#""type":"histogram""#),
        "--smarter should populate bimodality_coefficient and render a histogram; html: {html}"
    );
}

#[test]
fn viz_smart_smarter_matches_manual_moarstats() {
    // `viz smart --smarter` is a drop-in for the manual `moarstats` + `viz smart` two-step: the
    // box-panel skew/outlier hints appear without a prior moarstats run.
    let wrk = Workdir::new("viz_smart_smarter_matches_manual_moarstats");
    // same right-skewed fixture as viz_smart_uses_moarstats_box_hints, but no manual moarstats step
    let mut rows = String::from("id,amount\n");
    for i in 1..=280 {
        rows.push_str(&format!("{i},{}\n", i % 40 + 1));
    }
    for i in 281..=300 {
        rows.push_str(&format!("{i},1000\n"));
    }
    wrk.create_from_string("amounts.csv", &rows);

    let out_html = wrk.path("amounts.html").to_string_lossy().to_string();
    let mut cmd = wrk.command("viz");
    cmd.args(["smart", "amounts.csv", "--smarter", "-o", &out_html]);
    wrk.assert_success(&mut cmd);

    let html = wrk.read_to_string("amounts.html").unwrap();
    assert!(
        html.contains("right-skewed"),
        "--smarter should surface the moarstats skew hint; html: {html}"
    );
    assert!(
        html.contains("% outliers"),
        "--smarter should surface the moarstats outlier-share hint; html: {html}"
    );
    assert!(html.contains(r#""type":"box""#));
}

#[test]
fn viz_smart_smarter_no_headers_falls_back() {
    // moarstats can't honor --no-headers for its advanced-stat readers, so `--smarter` skips the
    // enrichment for --no-headers inputs and still renders a standard dashboard (no error).
    let wrk = Workdir::new("viz_smart_smarter_no_headers_falls_back");
    let mut rows = String::new();
    for i in 1..=100 {
        let city = match i % 3 {
            0 => "NYC",
            1 => "LA",
            _ => "SF",
        };
        rows.push_str(&format!("{i},{},{city}\n", 20 + i % 50));
    }
    wrk.create_from_string("headerless.csv", &rows);

    let out_html = wrk.path("headerless.html").to_string_lossy().to_string();
    let mut cmd = wrk.command("viz");
    cmd.args([
        "smart",
        "headerless.csv",
        "--smarter",
        "--no-headers",
        "-o",
        &out_html,
    ]);
    wrk.assert_success(&mut cmd);

    let html = wrk.read_to_string("headerless.html").unwrap();
    // the standard (non-enriched) dashboard still renders chart panels
    assert!(
        html.contains("Plotly.newPlot"),
        "fallback dashboard should still render; html: {html}"
    );
}

#[test]
fn viz_smart_smarter_no_headers_rebuilds_stale_cache() {
    // Regression: a pre-existing DEFAULT-parsing stats cache must not be reused by the
    // `--smarter --no-headers` fallback. get_stats_records keys its cache only by mtime + stat
    // sufficiency (not by parsing options), so the fallback forces a regeneration; the cache must
    // come back with no-headers field names ("0","1",...), not the stale header-derived names.
    let wrk = Workdir::new("viz_smart_smarter_no_headers_rebuilds_stale_cache");
    wrk.create_from_string("data.csv", "category\nNYC\nLA\nNYC\nSF\nLA\nNYC\n");

    // 1) build a default-parsing (headered) stats cache: the column is named by its header
    let mut stats = wrk.command("stats");
    stats.args([
        "data.csv",
        "--cardinality",
        "--quartiles",
        "--mode",
        "--stats-jsonl",
    ]);
    wrk.assert_success(&mut stats);
    let cache = wrk.read_to_string("data.stats.csv.data.jsonl").unwrap();
    assert!(
        cache.contains(r#""field":"category""#),
        "precondition: default cache should be header-named; got: {cache}"
    );

    // 2) the fallback must force-regenerate the cache with no-headers parsing
    let out_html = wrk.path("dash.html").to_string_lossy().to_string();
    let mut cmd = wrk.command("viz");
    cmd.args([
        "smart",
        "data.csv",
        "--smarter",
        "--no-headers",
        "-o",
        &out_html,
    ]);
    wrk.assert_success(&mut cmd);

    let rebuilt = wrk.read_to_string("data.stats.csv.data.jsonl").unwrap();
    assert!(
        rebuilt.contains(r#""field":"0""#),
        "fallback should force-rebuild the cache with no-headers field names; got: {rebuilt}"
    );
    assert!(
        !rebuilt.contains(r#""field":"category""#),
        "stale header-named field must not survive the no-headers fallback; got: {rebuilt}"
    );
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
fn viz_smart_inline_many_panels() {
    let wrk = Workdir::new("viz_smart_inline_many_panels");
    // 10 low-cardinality categorical columns -> 10 frequency-bar panels, more than the
    // typed-subplot limit of 8. With the default auto `--max-charts` (0), an HTML dashboard
    // draws every eligible column, switching to the inline-div grid renderer.
    let headers: Vec<String> = (0..10).map(|c| format!("c{c}")).collect();
    let mut rows = headers.join(",");
    rows.push('\n');
    for r in 0..30 {
        let cells: Vec<String> = (0..10).map(|c| format!("v{}", (r + c) % 4)).collect();
        rows.push_str(&cells.join(","));
        rows.push('\n');
    }
    wrk.create_from_string("wide.csv", &rows);

    let out_html = wrk.path("wide.html").to_string_lossy().to_string();
    let mut cmd = wrk.command("viz");
    // no --max-charts: rely on the auto default to draw all 10 eligible panels
    cmd.args(["smart", "wide.csv", "-o", &out_html]);
    wrk.assert_success(&mut cmd);

    let html = wrk.read_to_string("wide.html").unwrap();
    // inline-div grid markers (not the single-Plot subplot grid)
    assert!(html.contains(r#"class="qsv-viz-grid""#));
    assert!(html.contains(r#"class="qsv-viz-cell""#));
    // one independent plot per panel; with 10 panels there must be >8 newPlot calls
    let newplots = html.matches("Plotly.newPlot").count();
    assert!(
        newplots > 8,
        "expected more than 8 inline plots, found {newplots}"
    );
    // the plotly.js bundle is embedded once in <head>, before the first panel div
    assert!(html.contains("<!doctype html>"));
}

// `--open` on a >8-panel smart dashboard with NO --output must succeed: it writes the inline
// HTML to stdout AND opens a temp copy (it must not bail with a usage error after writing
// stdout, the pre-fix regression). `BROWSER=true` neutralizes the actual launch so the test is
// CI-safe; gated to unix since `true` is the harmless no-op opener there.
#[cfg(unix)]
#[test]
fn viz_smart_inline_open_no_output() {
    let wrk = Workdir::new("viz_smart_inline_open_no_output");
    let headers: Vec<String> = (0..10).map(|c| format!("c{c}")).collect();
    let mut rows = headers.join(",");
    rows.push('\n');
    for r in 0..30 {
        let cells: Vec<String> = (0..10).map(|c| format!("v{}", (r + c) % 4)).collect();
        rows.push_str(&cells.join(","));
        rows.push('\n');
    }
    wrk.create_from_string("wide.csv", &rows);

    let mut cmd = wrk.command("viz");
    cmd.env("BROWSER", "true")
        .args(["smart", "wide.csv", "--open"]);
    let out = wrk.output(&mut cmd);
    assert!(
        out.status.success(),
        "viz smart --open without --output should succeed; stderr: {}",
        String::from_utf8_lossy(&out.stderr)
    );
    // the inline dashboard HTML is still written to stdout
    let stdout = String::from_utf8_lossy(&out.stdout);
    assert!(stdout.contains(r#"class="qsv-viz-grid""#));
    assert!(stdout.contains("Plotly.newPlot"));
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

#[test]
fn viz_smart_scatter_pair_panel() {
    let wrk = Workdir::new("viz_smart_scatter_pair_panel");
    // two strongly-correlated, non-near-unique numeric columns => `viz smart` adds a
    // correlation heatmap AND a drill-down scatter of the strongest pair.
    let mut rows = String::from("metric_a,metric_b,city\n");
    for i in 0..60 {
        let a = i % 10;
        let b = a * 2 + (i % 2); // essentially perfectly correlated with metric_a
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
    assert!(html.contains(r#""type":"heatmap""#));
    // a scatter trace whose panel title names the pair and its (rounded) r value
    assert!(html.contains(r#""type":"scatter""#));
    assert!(html.contains("metric_a vs metric_b (r="));
}

#[test]
fn viz_smart_no_scatter_pair_when_weakly_correlated() {
    let wrk = Workdir::new("viz_smart_no_scatter_pair_when_weakly_correlated");
    // metric_a and metric_b are the two "digits" of i, so over 60 rows they enumerate the full
    // 10x6 grid exactly once => independent (r == 0). The correlation heatmap still appears, but
    // the weak pair is below the threshold, so NO drill-down scatter is added.
    let mut rows = String::from("metric_a,metric_b\n");
    for i in 0..60 {
        let a = i % 10;
        let b = i / 10;
        rows.push_str(&format!("{a},{b}\n"));
    }
    wrk.create_from_string("metrics.csv", &rows);

    let out_html = wrk.path("dash.html").to_string_lossy().to_string();
    let mut cmd = wrk.command("viz");
    cmd.args(["smart", "metrics.csv", "-o", &out_html]);
    wrk.assert_success(&mut cmd);

    let html = wrk.read_to_string("dash.html").unwrap();
    assert!(html.contains(r#""type":"heatmap""#)); // correlation panel present
    assert!(!html.contains(r#""type":"scatter""#)); // but no drill-down scatter
    assert!(!html.contains(" vs metric_")); // and no scatter-pair title
}

#[test]
fn viz_smart_timeseries_panel() {
    let wrk = Workdir::new("viz_smart_timeseries_panel");
    // a date column + a continuous (high-cardinality) numeric column => `viz smart` adds a
    // time-series line panel of the numeric column over the date. A low-card categorical
    // column becomes a frequency bar.
    let mut rows = String::from("txn_date,revenue,region\n");
    for i in 0..40 {
        let day = (i % 28) + 1;
        let month = (i / 28) + 1;
        let revenue = 1000 + i * 13;
        let region = if i % 2 == 0 { "east" } else { "west" };
        rows.push_str(&format!("2021-{month:02}-{day:02},{revenue},{region}\n"));
    }
    wrk.create_from_string("sales.csv", &rows);

    let out_html = wrk.path("dash.html").to_string_lossy().to_string();
    let mut cmd = wrk.command("viz");
    cmd.args(["smart", "sales.csv", "-o", &out_html]);
    wrk.assert_success(&mut cmd);

    let html = wrk.read_to_string("dash.html").unwrap();
    // a line trace drawn on a date-typed x-axis ...
    assert!(html.contains(r#""mode":"lines""#));
    assert!(html.contains(r#""type":"date""#));
    // ... titled "<numeric> over <date>"; revenue is the continuous numeric column chosen as y
    assert!(html.contains("revenue over txn_date"));
}

#[test]
fn viz_smart_timeseries_dmy_dates() {
    let wrk = Workdir::new("viz_smart_timeseries_dmy_dates");
    // AMBIGUOUS DMY dates (day AND month both <= 12, so each parses to a *different valid date*
    // under DMY vs MDY) in deliberately non-chronological input order, plus QSV_PREFER_DMY.
    // stats infers these as dates with the DMY preference; the time-series builder must use the
    // SAME preference, else the dates are parsed as MDY -> different values AND a different sort
    // order. Asserting the exact rendered x-axis (ISO, chronologically sorted) catches that.
    let rows = "sale_date,revenue\n07/02/2021,1500\n03/05/2021,1200\n11/01/2021,1000\n06/08/2021,\
                1700\n02/04/2021,1100\n09/03/2021,1300\n05/06/2021,1600\n08/07/2021,1400\n";
    wrk.create_from_string("sales.csv", rows);

    let out_html = wrk.path("dash.html").to_string_lossy().to_string();
    let mut cmd = wrk.command("viz");
    cmd.env("QSV_PREFER_DMY", "1");
    cmd.args(["smart", "sales.csv", "-o", &out_html]);
    wrk.assert_success(&mut cmd);

    let html = wrk.read_to_string("dash.html").unwrap();
    assert!(html.contains(r#""mode":"lines""#));
    assert!(html.contains("revenue over sale_date"));
    // x-axis dates parsed as DMY (e.g. 11/01 -> 2021-01-11, not 2021-11-01) and sorted
    // chronologically. Under the buggy MDY parse this array would have different values/order.
    assert!(html.contains(
        r#""x":["2021-01-11","2021-02-07","2021-03-09","2021-04-02","2021-05-03","2021-06-05","2021-07-08","2021-08-06"]"#
    ));
}

fn quakes(wrk: &Workdir) {
    wrk.create_from_string(
        "quakes.csv",
        "place,lat,lon,magnitude,depth_km,region\nTokyo,35.68,139.69,5.2,30,Asia\nLima,-12.04,-77.\
         04,6.1,45,Americas\nAnchorage,61.22,-149.90,4.8,20,Americas\nWellington,-41.29,174.78,5.\
         5,12,Oceania\nReykjavik,64.13,-21.90,3.9,8,Europe\nSantiago,-33.45,-70.66,6.8,60,\
         Americas\nJakarta,-6.21,106.85,5.0,25,Asia\nAthens,37.98,23.73,4.2,15,Europe\n",
    );
}

#[test]
fn viz_map_basic() {
    let wrk = Workdir::new("viz_map_basic");
    quakes(&wrk);

    let mut cmd = wrk.command("viz");
    cmd.args(["map", "quakes.csv", "--lat", "lat", "--lon", "lon"]);
    let out = wrk.output(&mut cmd);
    assert!(out.status.success());

    let html = String::from_utf8_lossy(&out.stdout);
    // a token-free ScatterMapbox point map on OpenStreetMap tiles
    assert!(html.contains("Plotly.newPlot"));
    assert!(html.contains(r#""type":"scattermapbox""#));
    assert!(html.contains("open-street-map"));
    // auto-centered/zoomed mapbox layout
    assert!(html.contains(r#""center""#));
    assert!(html.contains(r#""zoom""#));
}

#[test]
fn viz_map_color_scale() {
    let wrk = Workdir::new("viz_map_color_scale");
    quakes(&wrk);

    let mut cmd = wrk.command("viz");
    cmd.args([
        "map",
        "quakes.csv",
        "--lat",
        "lat",
        "--lon",
        "lon",
        "--color",
        "magnitude",
    ]);
    let out = wrk.output(&mut cmd);
    assert!(out.status.success());

    let html = String::from_utf8_lossy(&out.stdout);
    assert!(html.contains(r#""type":"scattermapbox""#));
    assert!(html.contains(r#""colorscale":"Viridis""#));
    assert!(html.contains(r#""showscale":true"#));
    assert!(html.contains(r#""colorbar":{"title":{"text":"magnitude"#));
}

#[test]
fn viz_map_bubble_size() {
    let wrk = Workdir::new("viz_map_bubble_size");
    quakes(&wrk);

    let mut cmd = wrk.command("viz");
    cmd.args([
        "map",
        "quakes.csv",
        "--lat",
        "lat",
        "--lon",
        "lon",
        "--size",
        "depth_km",
    ]);
    let out = wrk.output(&mut cmd);
    assert!(out.status.success());

    let html = String::from_utf8_lossy(&out.stdout);
    assert!(html.contains(r#""type":"scattermapbox""#));
    assert!(html.contains(r#""marker":{"size":["#));
}

#[test]
fn viz_map_density() {
    let wrk = Workdir::new("viz_map_density");
    quakes(&wrk);

    let mut cmd = wrk.command("viz");
    cmd.args([
        "map",
        "quakes.csv",
        "--lat",
        "lat",
        "--lon",
        "lon",
        "--density",
    ]);
    let out = wrk.output(&mut cmd);
    assert!(out.status.success());

    let html = String::from_utf8_lossy(&out.stdout);
    assert!(html.contains(r#""type":"densitymapbox""#));
}

#[test]
fn viz_map_style_carto() {
    let wrk = Workdir::new("viz_map_style_carto");
    quakes(&wrk);

    let mut cmd = wrk.command("viz");
    cmd.args([
        "map",
        "quakes.csv",
        "--lat",
        "lat",
        "--lon",
        "lon",
        "--style",
        "carto-positron",
    ]);
    let out = wrk.output(&mut cmd);
    assert!(out.status.success());

    let html = String::from_utf8_lossy(&out.stdout);
    assert!(html.contains("carto-positron"));
}

#[test]
fn viz_map_series_traces() {
    let wrk = Workdir::new("viz_map_series_traces");
    quakes(&wrk);

    let mut cmd = wrk.command("viz");
    cmd.args([
        "map",
        "quakes.csv",
        "--lat",
        "lat",
        "--lon",
        "lon",
        "--series",
        "region",
    ]);
    let out = wrk.output(&mut cmd);
    assert!(out.status.success());

    let html = String::from_utf8_lossy(&out.stdout);
    // one ScatterMapbox trace per region, named by category
    assert!(html.contains(r#""type":"scattermapbox""#));
    assert!(html.contains(r#""name":"Asia""#));
    assert!(html.contains(r#""name":"Americas""#));
}

#[test]
fn viz_map_mapbox_style_needs_token_errors() {
    let wrk = Workdir::new("viz_map_mapbox_style_needs_token_errors");
    quakes(&wrk);

    let mut cmd = wrk.command("viz");
    cmd.args([
        "map",
        "quakes.csv",
        "--lat",
        "lat",
        "--lon",
        "lon",
        "--style",
        "satellite",
    ]);
    let out = wrk.output(&mut cmd);
    assert!(!out.status.success());
    let stderr = wrk.output_stderr(&mut cmd);
    assert!(stderr.contains("requires --mapbox-token"));
}

#[test]
fn viz_map_unknown_style_errors() {
    let wrk = Workdir::new("viz_map_unknown_style_errors");
    quakes(&wrk);

    let mut cmd = wrk.command("viz");
    cmd.args([
        "map",
        "quakes.csv",
        "--lat",
        "lat",
        "--lon",
        "lon",
        "--style",
        "bogus",
    ]);
    let out = wrk.output(&mut cmd);
    assert!(!out.status.success());
    let stderr = wrk.output_stderr(&mut cmd);
    assert!(stderr.contains("Unknown --style"));
}

#[test]
fn viz_map_density_with_series_errors() {
    let wrk = Workdir::new("viz_map_density_with_series_errors");
    quakes(&wrk);

    let mut cmd = wrk.command("viz");
    cmd.args([
        "map",
        "quakes.csv",
        "--lat",
        "lat",
        "--lon",
        "lon",
        "--density",
        "--series",
        "region",
    ]);
    let out = wrk.output(&mut cmd);
    assert!(!out.status.success());
    let stderr = wrk.output_stderr(&mut cmd);
    assert!(stderr.contains("cannot be combined with --series"));
}

#[test]
fn viz_smart_with_coords_has_map_panel() {
    let wrk = Workdir::new("viz_smart_with_coords_has_map_panel");
    quakes(&wrk);

    let mut cmd = wrk.command("viz");
    cmd.args(["smart", "quakes.csv"]);
    let out = wrk.output(&mut cmd);
    assert!(out.status.success());

    let html = String::from_utf8_lossy(&out.stdout);
    // smart auto-detects the lat/lon pair and adds a map panel; a map forces the inline
    // (self-contained HTML page) render path
    assert!(html.contains("<!doctype html>"));
    assert!(html.contains(r#""type":"scattermapbox""#));
    assert!(html.contains("open-street-map"));
}

#[test]
fn viz_smart_map_coords_not_charted_as_distributions() {
    // Columns recognized as the map's lat/lon pair are charted on the map only — not redundantly
    // as their own box/histogram distribution panels (and not picked as the time-series y).
    let wrk = Workdir::new("viz_smart_map_coords_not_charted_as_distributions");
    // lat/lon (continuous, near-unique) + one low-cardinality categorical. Without the exclusion,
    // each coordinate would fall through to a box/histogram panel; with it, only the map + the bar.
    let mut rows = String::from("lat,lon,category\n");
    for i in 0..60 {
        let lat = 34.0 + (i as f64) * 0.1;
        let lon = -118.0 + (i as f64) * 0.1;
        let cat = match i % 3 {
            0 => "A",
            1 => "B",
            _ => "C",
        };
        rows.push_str(&format!("{lat:.4},{lon:.4},{cat}\n"));
    }
    wrk.create_from_string("geo.csv", &rows);

    let out_html = wrk.path("geo.html").to_string_lossy().to_string();
    let mut cmd = wrk.command("viz");
    cmd.args(["smart", "geo.csv", "-o", &out_html]);
    wrk.assert_success(&mut cmd);

    let html = wrk.read_to_string("geo.html").unwrap();
    assert!(
        html.contains(r#""type":"scattermapbox""#),
        "map panel should be present"
    );
    assert!(
        html.contains(r#""type":"bar""#),
        "the categorical should still be a bar panel"
    );
    // the coordinates must NOT be re-charted as their own distribution panels
    assert!(
        !html.contains(r#""type":"box""#) && !html.contains(r#""type":"histogram""#),
        "lat/lon must not be charted as box/histogram distribution panels; html: {html}"
    );
}

#[test]
fn viz_smart_named_coords_without_valid_range_still_charted() {
    // Edge case: columns named lat/lon are numeric but have NO in-range coordinate, so no map panel
    // renders. The exclusion must NOT hide them then — they should be charted as normal numeric
    // distributions rather than vanishing from the dashboard entirely.
    let wrk = Workdir::new("viz_smart_named_coords_without_valid_range_still_charted");
    // float values well outside [-90,90] / [-180,180] -> build_map_panel finds no valid coords
    let mut rows = String::from("lat,lon\n");
    for i in 0..60 {
        rows.push_str(&format!(
            "{:.2},{:.2}\n",
            100.0 + i as f64 * 0.5,
            200.0 + i as f64 * 0.5
        ));
    }
    wrk.create_from_string("notgeo.csv", &rows);

    let out_html = wrk.path("notgeo.html").to_string_lossy().to_string();
    let mut cmd = wrk.command("viz");
    cmd.args(["smart", "notgeo.csv", "-o", &out_html]);
    wrk.assert_success(&mut cmd);

    let html = wrk.read_to_string("notgeo.html").unwrap();
    assert!(
        !html.contains(r#""type":"scattermapbox""#),
        "no map should render for out-of-range coords"
    );
    assert!(
        html.contains(r#""type":"box""#),
        "out-of-range lat/lon should still be charted as distributions, not hidden; html: {html}"
    );
}

/// Tamper with a frequency JSONL cache by replacing the first occurrence of
/// `old_count` with `new_count`. Used to prove `viz smart` reads the cache.
fn tamper_freq_cache(path: &std::path::Path, old_count: u64, new_count: u64) {
    let contents = std::fs::read_to_string(path).expect("read cache");
    let mut lines: Vec<String> = contents.lines().map(String::from).collect();
    let mut found = false;
    // lines[0] is metadata; lines[1..] are per-column entries
    'outer: for line in lines.iter_mut().skip(1) {
        let mut entry: serde_json::Value = serde_json::from_str(line).expect("parse entry");
        for freq in entry["frequencies"]
            .as_array_mut()
            .expect("frequencies array")
        {
            if freq["count"].as_u64() == Some(old_count) {
                freq["count"] = serde_json::Value::from(new_count);
                found = true;
                *line = serde_json::to_string(&entry).expect("re-encode entry");
                break 'outer;
            }
        }
    }
    assert!(found, "count {old_count} not found in cache to tamper");
    std::fs::write(path, lines.join("\n")).expect("write tampered cache");
}

// `viz smart` builds its frequency bars from the data; here we prove it reuses a
// pre-existing `frequency` JSONL cache instead of re-scanning. A tampered count
// (987654 — distinctive enough not to collide with the embedded plotly.min.js)
// must surface in the rendered bar, which can only happen on a cache read.
#[test]
fn viz_smart_uses_frequency_cache() {
    let wrk = Workdir::new("viz_smart_uses_frequency_cache");
    wrk.create_from_string(
        "people.csv",
        "name,color\nAlice,red\nBob,blue\nAlice,red\nCarol,green\n",
    );

    // create the frequency cache (color: red=2, blue=1, green=1)
    let mut fc = wrk.command("frequency");
    fc.arg("people.csv").arg("--frequency-jsonl");
    wrk.assert_success(&mut fc);
    let cache_path = wrk.path("people.freq.csv.data.jsonl");
    assert!(cache_path.exists(), "frequency cache should exist");

    tamper_freq_cache(&cache_path, 2, 987_654);

    let out_html = wrk.path("dash.html").to_string_lossy().to_string();
    let mut cmd = wrk.command("viz");
    cmd.args(["smart", "people.csv", "-o", &out_html]);
    wrk.assert_success(&mut cmd);

    let html = wrk.read_to_string("dash.html").unwrap();
    assert!(html.contains(r#""type":"bar""#));
    assert!(
        html.contains("987654"),
        "tampered cache count should appear in the bar (proving cache read)"
    );
}

// A cache older than the source CSV is stale: `viz smart` must ignore it and
// recompute, so a tampered (stale) count must NOT surface.
#[test]
fn viz_smart_stale_frequency_cache_fallback() {
    let wrk = Workdir::new("viz_smart_stale_frequency_cache_fallback");
    wrk.create_from_string(
        "people.csv",
        "name,color\nAlice,red\nBob,blue\nAlice,red\nCarol,green\n",
    );

    let mut fc = wrk.command("frequency");
    fc.arg("people.csv").arg("--frequency-jsonl");
    wrk.assert_success(&mut fc);
    let cache_path = wrk.path("people.freq.csv.data.jsonl");
    tamper_freq_cache(&cache_path, 2, 987_654);

    // rewrite the source so it is newer than the cache => cache is stale
    wrk.create_from_string(
        "people.csv",
        "name,color\nAlice,red\nBob,blue\nAlice,red\nCarol,green\nDave,red\n",
    );

    let out_html = wrk.path("dash.html").to_string_lossy().to_string();
    let mut cmd = wrk.command("viz");
    cmd.args(["smart", "people.csv", "-o", &out_html]);
    wrk.assert_success(&mut cmd);

    let html = wrk.read_to_string("dash.html").unwrap();
    assert!(html.contains(r#""type":"bar""#));
    assert!(
        !html.contains("987654"),
        "stale cache must be ignored; recomputed bars should not show the tampered count"
    );
}

// A frequency cache with duplicate column names is ambiguous for a name-keyed
// reader (last column shadows the earlier one), so `viz smart` must reject it
// and recompute — the tampered (cached) count must NOT surface.
#[test]
fn viz_smart_duplicate_headers_frequency_cache_fallback() {
    let wrk = Workdir::new("viz_smart_duplicate_headers_frequency_cache_fallback");
    // two columns both named "color"
    wrk.create_from_string("people.csv", "color,color\nred,x\nblue,y\nred,x\ngreen,z\n");

    let mut fc = wrk.command("frequency");
    fc.arg("people.csv").arg("--frequency-jsonl");
    wrk.assert_success(&mut fc);
    let cache_path = wrk.path("people.freq.csv.data.jsonl");
    tamper_freq_cache(&cache_path, 2, 987_654);

    let out_html = wrk.path("dash.html").to_string_lossy().to_string();
    let mut cmd = wrk.command("viz");
    cmd.args(["smart", "people.csv", "-o", &out_html]);
    wrk.assert_success(&mut cmd);

    let html = wrk.read_to_string("dash.html").unwrap();
    assert!(html.contains(r#""type":"bar""#));
    assert!(
        !html.contains("987654"),
        "duplicate-header cache is ambiguous and must be ignored; bars should be recomputed"
    );
}

// `viz smart --no-headers` reads the whole file in original order, while a
// frequency cache built with the same (default, full) selection keys columns
// positionally. Those line up, so the cache IS reused — the tampered count
// surfaces. Guards that the no-headers selection-signature check does not
// over-reject a legitimate full-selection cache.
#[test]
fn viz_smart_no_headers_frequency_cache_used() {
    let wrk = Workdir::new("viz_smart_no_headers_frequency_cache_used");
    // headerless: two low-cardinality categorical columns
    wrk.create_from_string("people.csv", "red,x\nblue,y\nred,x\ngreen,z\n");

    let mut fc = wrk.command("frequency");
    fc.arg("people.csv")
        .arg("--no-headers")
        .arg("--frequency-jsonl");
    wrk.assert_success(&mut fc);
    let cache_path = wrk.path("people.freq.csv.data.jsonl");
    tamper_freq_cache(&cache_path, 2, 987_654);

    let out_html = wrk.path("dash.html").to_string_lossy().to_string();
    let mut cmd = wrk.command("viz");
    cmd.args(["smart", "people.csv", "--no-headers", "-o", &out_html]);
    wrk.assert_success(&mut cmd);

    let html = wrk.read_to_string("dash.html").unwrap();
    assert!(html.contains(r#""type":"bar""#));
    assert!(
        html.contains("987654"),
        "full-selection no-headers cache should be reused (tampered count expected)"
    );
}

// A frequency cache built with a reordered `--no-headers --select` keys columns
// positionally within that selection. `viz smart --no-headers` reads columns in
// original order, so the cache's selection signature won't match and the cache
// must be rejected — the tampered count must NOT surface (no silent mis-mapping).
#[test]
fn viz_smart_no_headers_reordered_select_cache_rejected() {
    let wrk = Workdir::new("viz_smart_no_headers_reordered_select_cache_rejected");
    wrk.create_from_string("people.csv", "red,x\nblue,y\nred,x\ngreen,z\n");

    // cache built over a reordered selection (col 2 then col 1)
    let mut fc = wrk.command("frequency");
    fc.arg("people.csv")
        .arg("--no-headers")
        .args(["--select", "2,1"])
        .arg("--frequency-jsonl");
    wrk.assert_success(&mut fc);
    let cache_path = wrk.path("people.freq.csv.data.jsonl");
    tamper_freq_cache(&cache_path, 2, 987_654);

    let out_html = wrk.path("dash.html").to_string_lossy().to_string();
    let mut cmd = wrk.command("viz");
    cmd.args(["smart", "people.csv", "--no-headers", "-o", &out_html]);
    wrk.assert_success(&mut cmd);

    let html = wrk.read_to_string("dash.html").unwrap();
    assert!(html.contains(r#""type":"bar""#));
    assert!(
        !html.contains("987654"),
        "reordered-select no-headers cache must be rejected to avoid mis-mapping columns"
    );
}

// The no-headers selection signature is built from first-row bytes, so when two
// columns share the same first-row value a reordered `--select` can produce an
// identical signature. `viz smart --no-headers` must therefore reject a
// no-headers cache whose first row has repeated values (the order can't be
// proven) — the tampered count must NOT surface.
#[test]
fn viz_smart_no_headers_colliding_firstrow_cache_rejected() {
    let wrk = Workdir::new("viz_smart_no_headers_colliding_firstrow_cache_rejected");
    // first row is "red,red" — equal values in both columns
    wrk.create_from_string("people.csv", "red,red\nblue,red\nred,blue\ngreen,red\n");

    // reordered selection whose signature collides with the full-order signature
    let mut fc = wrk.command("frequency");
    fc.arg("people.csv")
        .arg("--no-headers")
        .args(["--select", "2,1"])
        .arg("--frequency-jsonl");
    wrk.assert_success(&mut fc);
    let cache_path = wrk.path("people.freq.csv.data.jsonl");
    tamper_freq_cache(&cache_path, 2, 987_654);

    let out_html = wrk.path("dash.html").to_string_lossy().to_string();
    let mut cmd = wrk.command("viz");
    cmd.args(["smart", "people.csv", "--no-headers", "-o", &out_html]);
    wrk.assert_success(&mut cmd);

    let html = wrk.read_to_string("dash.html").unwrap();
    assert!(html.contains(r#""type":"bar""#));
    assert!(
        !html.contains("987654"),
        "ambiguous (colliding-first-row) no-headers cache must be rejected"
    );
}

// Duplicate column names must be detected even when one duplicate is a sentinel
// that the build loop skips. `qsv frequency` can't emit this mix for duplicate
// names (it classifies same-named columns identically), so the only way to reach
// it is a hand-edited/corrupt cache — which the view must still reject. Here a
// crafted cache pairs an <ALL_UNIQUE> "id" (skipped) with a data "id" carrying a
// distinctive count; `viz smart` must ignore the cache and recompute, so that
// count must NOT surface.
#[test]
fn viz_smart_duplicate_headers_with_sentinel_cache_fallback() {
    let wrk = Workdir::new("viz_smart_duplicate_headers_with_sentinel_cache_fallback");
    // col1 "id" all-unique; col2 "id" low-cardinality (the charted bar)
    wrk.create_from_string("people.csv", "id,id\na,red\nb,red\nc,blue\nd,red\n");

    // hand-craft a cache: sentinel "id" then a data "id" with a planted count.
    // (Written after the CSV so it is newer / not stale.)
    // headed cache: selection_signature is not validated, so a placeholder is fine
    let cache = concat!(
        r#"{"arg_input":"people.csv","flag_high_card_threshold":100,"flag_high_card_pct":90,"flag_no_nulls":false,"flag_no_headers":false,"flag_delimiter":",","record_count":4,"column_count":2,"date_generated":"2026-06-20T00:00:00+00:00","qsv_version":"21.1.0","selection_signature":"","canonical_input_path":""}"#,
        "\n",
        r#"{"field":"id","cardinality":4,"frequencies":[{"value":"<ALL_UNIQUE>","count":4,"percentage":100.0}]}"#,
        "\n",
        r#"{"field":"id","cardinality":2,"frequencies":[{"value":"red","count":987654,"percentage":75.0},{"value":"blue","count":1,"percentage":25.0}]}"#,
        "\n",
    );
    std::fs::write(wrk.path("people.freq.csv.data.jsonl"), cache).unwrap();

    let out_html = wrk.path("dash.html").to_string_lossy().to_string();
    let mut cmd = wrk.command("viz");
    cmd.args(["smart", "people.csv", "-o", &out_html]);
    wrk.assert_success(&mut cmd);

    let html = wrk.read_to_string("dash.html").unwrap();
    assert!(html.contains(r#""type":"bar""#));
    assert!(
        !html.contains("987654"),
        "duplicate name with a sentinel duplicate must still be detected and rejected"
    );
}

// The no-headers selection signature joins first-row bytes with a 0x1f (Unit
// Separator) WITHOUT escaping, so a first-row value that itself contains 0x1f
// makes the join ambiguous (a reordered selection could collide even with
// distinct values). `viz smart --no-headers` must therefore reject such a cache
// conservatively — even a legitimate full-selection cache — so the tampered
// count must NOT surface.
#[test]
fn viz_smart_no_headers_separator_in_data_cache_rejected() {
    let wrk = Workdir::new("viz_smart_no_headers_separator_in_data_cache_rejected");
    // col1's first-row value embeds the 0x1f separator
    wrk.create_from_string("people.csv", "a\u{1f}b,c\nx,y\na\u{1f}b,c\nz,w\n");

    let mut fc = wrk.command("frequency");
    fc.arg("people.csv")
        .arg("--no-headers")
        .arg("--frequency-jsonl");
    wrk.assert_success(&mut fc);
    let cache_path = wrk.path("people.freq.csv.data.jsonl");
    tamper_freq_cache(&cache_path, 2, 987_654);

    let out_html = wrk.path("dash.html").to_string_lossy().to_string();
    let mut cmd = wrk.command("viz");
    cmd.args(["smart", "people.csv", "--no-headers", "-o", &out_html]);
    wrk.assert_success(&mut cmd);

    let html = wrk.read_to_string("dash.html").unwrap();
    assert!(html.contains(r#""type":"bar""#));
    assert!(
        !html.contains("987654"),
        "no-headers cache with an embedded signature separator must be rejected"
    );
}

// The no-headers selection signature stringifies each first-row value with a
// LOSSY UTF-8 conversion, so two distinct invalid-UTF8 values could collapse to
// the same replacement text and let a reordered selection collide. `viz smart
// --no-headers` must therefore reject a cache whose first row has any non-UTF8
// value — even a legitimate full selection — so the tampered count must NOT
// surface. (Raw bytes are written directly since invalid UTF-8 isn't a &str.)
#[test]
fn viz_smart_no_headers_invalid_utf8_cache_rejected() {
    let wrk = Workdir::new("viz_smart_no_headers_invalid_utf8_cache_rejected");
    // col1's first-row value is an invalid UTF-8 byte (0xFF)
    std::fs::write(wrk.path("people.csv"), b"\xff,c\nx,y\n\xff,c\nz,w\n").unwrap();

    let mut fc = wrk.command("frequency");
    fc.arg("people.csv")
        .arg("--no-headers")
        .arg("--frequency-jsonl");
    wrk.assert_success(&mut fc);
    let cache_path = wrk.path("people.freq.csv.data.jsonl");
    tamper_freq_cache(&cache_path, 2, 987_654);

    let out_html = wrk.path("dash.html").to_string_lossy().to_string();
    let mut cmd = wrk.command("viz");
    cmd.args(["smart", "people.csv", "--no-headers", "-o", &out_html]);
    wrk.assert_success(&mut cmd);

    let html = wrk.read_to_string("dash.html").unwrap();
    assert!(html.contains(r#""type":"bar""#));
    assert!(
        !html.contains("987654"),
        "no-headers cache with non-UTF8 first-row data must be rejected"
    );
}
