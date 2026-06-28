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
fn viz_pie_advises_bar_for_near_equal_slices() {
    let wrk = Workdir::new("viz_pie_advises_bar_for_near_equal_slices");

    // 5 near-equal categories (each ~20%): a pie is hard to read, so the advisory fires.
    let mut near = String::from("cat\n");
    for i in 0..100 {
        let cat = match i % 5 {
            0 => "A",
            1 => "B",
            2 => "C",
            3 => "D",
            _ => "E",
        };
        near.push_str(&format!("{cat}\n"));
    }
    wrk.create_from_string("near.csv", &near);
    let out_html = wrk.path("near.html").to_string_lossy().to_string();
    let mut cmd = wrk.command("viz");
    cmd.args(["pie", "near.csv", "--x", "cat", "-o", &out_html]);
    let out = wrk.output(&mut cmd);
    assert!(out.status.success());
    let stderr = String::from_utf8_lossy(&out.stderr);
    assert!(
        stderr.contains("near-equal"),
        "near-equal slices should trigger the pie advisory; stderr: {stderr}"
    );

    // a dominant-slice distribution (A ~80%): a pie reads fine here, so NO advisory.
    let mut dom = String::from("cat\n");
    for i in 0..100 {
        let cat = if i < 80 {
            "A"
        } else {
            match i % 4 {
                0 => "B",
                1 => "C",
                2 => "D",
                _ => "E",
            }
        };
        dom.push_str(&format!("{cat}\n"));
    }
    wrk.create_from_string("dom.csv", &dom);
    let out_html2 = wrk.path("dom.html").to_string_lossy().to_string();
    let mut cmd_2 = wrk.command("viz");
    cmd_2.args(["pie", "dom.csv", "--x", "cat", "-o", &out_html2]);
    let out2 = wrk.output(&mut cmd_2);
    assert!(out2.status.success());
    let stderr2 = String::from_utf8_lossy(&out2.stderr);
    assert!(
        !stderr2.contains("near-equal"),
        "a dominant-slice pie should NOT trigger the advisory; stderr: {stderr2}"
    );
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

/// A low-cardinality categorical column with more distinct values than `--limit` and some empty
/// cells. `id` is near-unique (skipped); `category` has 15 distinct values (cat00..cat14) plus
/// empty cells, so a `viz smart` frequency bar should show the top-10 categories, an aggregate
/// `Other (5)` bar, and a `(NULL)` bar.
fn categories_with_nulls(wrk: &Workdir) {
    let mut rows = String::from("id,category\n");
    for i in 1..=150 {
        // every 10th row leaves the category empty -> 15 NULLs
        let cat = if i % 10 == 0 {
            String::new()
        } else {
            format!("cat{:02}", i % 15)
        };
        rows.push_str(&format!("{i},{cat}\n"));
    }
    wrk.create_from_string("cats.csv", &rows);
}

#[test]
fn viz_smart_freq_bars_null_and_other() {
    let wrk = Workdir::new("viz_smart_freq_bars_null_and_other");
    categories_with_nulls(&wrk);

    let out_html = wrk.path("cats.html").to_string_lossy().to_string();
    let mut cmd = wrk.command("viz");
    cmd.args(["smart", "cats.csv", "-o", &out_html]);
    wrk.assert_success(&mut cmd);

    let html = wrk.read_to_string("cats.html").unwrap();
    assert!(html.contains(r#""type":"bar""#));
    // empty cells become a "(NULL)" bar; the 5 categories beyond --limit 10 roll up into
    // "Other (5)"; both aggregate bars are drawn in the muted-grey #999999.
    assert!(
        html.contains("(NULL)"),
        "expected a (NULL) bar; html: {html}"
    );
    assert!(
        html.contains("Other (5)"),
        "expected an Other (5) aggregate bar; html: {html}"
    );
    assert!(
        html.contains("#999999"),
        "expected the muted-grey aggregate-bar color; html: {html}"
    );
}

#[test]
fn viz_smart_freq_bars_no_nulls_no_other() {
    let wrk = Workdir::new("viz_smart_freq_bars_no_nulls_no_other");
    categories_with_nulls(&wrk);

    let out_html = wrk.path("cats.html").to_string_lossy().to_string();
    let mut cmd = wrk.command("viz");
    cmd.args([
        "smart",
        "cats.csv",
        "--no-nulls",
        "--no-other",
        "-o",
        &out_html,
    ]);
    wrk.assert_success(&mut cmd);

    let html = wrk.read_to_string("cats.html").unwrap();
    assert!(html.contains(r#""type":"bar""#));
    // both aggregate bars suppressed
    assert!(
        !html.contains("(NULL)"),
        "--no-nulls should drop the (NULL) bar; html: {html}"
    );
    assert!(
        !html.contains("Other ("),
        "--no-other should drop the Other bar; html: {html}"
    );
}

#[test]
fn viz_smart_freq_bars_from_cache_match_rawscan() {
    // The frequency cache stores the complete per-value distribution including the null bucket,
    // so the cache-driven path (freq_from_cache) must produce the same (NULL)/Other bars as the
    // raw-scan path (count_values).
    let wrk = Workdir::new("viz_smart_freq_bars_from_cache");
    categories_with_nulls(&wrk);

    // pre-build the frequency JSONL cache
    let mut freq = wrk.command("frequency");
    freq.args(["cats.csv", "--frequency-jsonl"]);
    wrk.assert_success(&mut freq);

    let out_html = wrk.path("cats.html").to_string_lossy().to_string();
    let mut cmd = wrk.command("viz");
    cmd.args(["smart", "cats.csv", "-o", &out_html]);
    wrk.assert_success(&mut cmd);

    let html = wrk.read_to_string("cats.html").unwrap();
    assert!(
        html.contains("(NULL)"),
        "cache path should keep (NULL); html: {html}"
    );
    assert!(
        html.contains("Other (5)"),
        "cache path should keep Other (5); html: {html}"
    );
}

#[test]
fn viz_smart_freq_bars_whitespace_counts_as_null() {
    // `qsv frequency` trims values by default (and the frequency cache is always trimmed), so a
    // whitespace-only cell is a NULL. The raw-scan path must trim too, otherwise whitespace-only
    // cells would become a literal blank category instead of "(NULL)" — diverging from the cache
    // and escaping --no-nulls. Here the ONLY nulls are whitespace-only cells (no byte-empty
    // cells), so a "(NULL)" bar can only appear if the raw path trims.
    let wrk = Workdir::new("viz_smart_freq_bars_whitespace_null");
    let mut rows = String::from("id,category\n");
    for i in 1..=60 {
        let cat = if i % 5 == 0 {
            "   ".to_string() // whitespace-only -> NULL after trim
        } else {
            match i % 3 {
                0 => "apple",
                1 => "banana",
                _ => "cherry",
            }
            .to_string()
        };
        rows.push_str(&format!("{i},{cat}\n"));
    }
    wrk.create_from_string("ws.csv", &rows);

    // raw-scan path (no frequency cache present)
    let out_html = wrk.path("ws.html").to_string_lossy().to_string();
    let mut cmd = wrk.command("viz");
    cmd.args(["smart", "ws.csv", "-o", &out_html]);
    wrk.assert_success(&mut cmd);

    let html = wrk.read_to_string("ws.html").unwrap();
    assert!(
        html.contains("(NULL)"),
        "whitespace-only cells should be trimmed and counted as (NULL) on the raw-scan path; \
         html: {html}"
    );

    // --no-nulls must then suppress them (it couldn't if they were a literal blank category)
    let out_html2 = wrk.path("ws_nonulls.html").to_string_lossy().to_string();
    let mut cmd_2 = wrk.command("viz");
    cmd_2.args(["smart", "ws.csv", "--no-nulls", "-o", &out_html2]);
    wrk.assert_success(&mut cmd_2);
    let html2 = wrk.read_to_string("ws_nonulls.html").unwrap();
    assert!(
        !html2.contains("(NULL)"),
        "--no-nulls should suppress the whitespace-derived (NULL) bar; html: {html2}"
    );
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
fn viz_smart_plain_promotes_bimodal_to_histogram() {
    // Plain `viz smart` (NO --smarter) must ALSO detect a clearly-bimodal column and render a
    // histogram, not a misleading box (whose median sits in the empty gap between the two peaks).
    // `enrich_bimodality` computes Sarle's BC in one streaming pass — no moarstats required.
    let wrk = Workdir::new("viz_smart_plain_promotes_bimodal_to_histogram");
    // two well-separated symmetric clusters (0..39 and 1000..1039), 150 rows each: cardinality 80
    // (continuous) and a flat-topped two-peak shape -> high BC and platykurtic -> histogram.
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
    cmd.args(["smart", "bimodal.csv", "-o", &out_html]);
    wrk.assert_success(&mut cmd);

    let html = wrk.read_to_string("bimodal.html").unwrap();
    assert!(
        html.contains(r#""type":"histogram""#),
        "plain viz smart should detect bimodality and render a histogram; html: {html}"
    );
    assert!(
        !html.contains(r#""type":"box""#),
        "the bimodal column should NOT be a box plot; html: {html}"
    );
}

#[test]
fn viz_smart_plain_skewed_outliers_stay_box_not_histogram() {
    // A heavily right-skewed UNIMODAL column (long tail of large values) has a high Sarle BC purely
    // from skewness, but it's leptokurtic — plain `viz smart`'s platykurtic guard must keep it a
    // box (with outlier points), NOT a one-tall-bar histogram. Guards against Sarle's BC skew
    // false positive in the plain path.
    let wrk = Workdir::new("viz_smart_plain_skewed_outliers_stay_box_not_histogram");
    let mut rows = String::from("id,amount\n");
    for i in 1..=280 {
        rows.push_str(&format!("{i},{}\n", i % 40 + 1)); // tight bulk 1..40
    }
    for i in 281..=300 {
        rows.push_str(&format!("{i},5000\n")); // heavy right tail (leptokurtic)
    }
    wrk.create_from_string("skewed.csv", &rows);

    let out_html = wrk.path("skewed.html").to_string_lossy().to_string();
    let mut cmd = wrk.command("viz");
    cmd.args(["smart", "skewed.csv", "-o", &out_html]);
    wrk.assert_success(&mut cmd);

    let html = wrk.read_to_string("skewed.html").unwrap();
    assert!(
        html.contains(r#""type":"box""#),
        "a skewed/long-tailed unimodal column should stay a box; html: {html}"
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

#[test]
fn viz_smart_overview_panel_spans_full_width_typed_grid() {
    // 2 numeric columns -> a correlation heatmap + correlated-pair scatter (both "overview"
    // panels), plus 2 low-cardinality categoricals. With <= 8 cartesian panels this renders as the
    // typed subplot grid, where each overview panel must get a full-width x-axis domain ([0,1]).
    // The numeric columns are low-cardinality (repeated values) so they pass the correlation
    // panel's near-unique filter (uniqueness_ratio <= 0.95).
    let wrk = Workdir::new("viz_smart_overview_panel_spans_full_width_typed_grid");
    let mut rows = String::from("x,y,cat,grp\n");
    for i in 0..60 {
        let x = i % 10;
        let y = 2 * (i % 10) + (i % 2); // strongly correlated with x, low cardinality
        let cat = match i % 3 {
            0 => "A",
            1 => "B",
            _ => "C",
        };
        let grp = if i % 2 == 0 { "east" } else { "west" };
        rows.push_str(&format!("{x},{y},{cat},{grp}\n"));
    }
    wrk.create_from_string("corr.csv", &rows);

    let out_html = wrk.path("corr.html").to_string_lossy().to_string();
    let mut cmd = wrk.command("viz");
    cmd.args(["smart", "corr.csv", "-o", &out_html]);
    wrk.assert_success(&mut cmd);

    let html = wrk.read_to_string("corr.html").unwrap();
    // a correlation heatmap overview panel is present and spans the full page width
    assert!(
        html.contains(r#""type":"heatmap""#),
        "expected a correlation heatmap: {html}"
    );
    assert!(
        html.contains(r#""domain":[0.0,1.0]"#),
        "an overview panel's x-axis should span the full width ([0,1]): {html}"
    );
}

#[test]
fn viz_smart_overview_panels_full_width_inline() {
    // the global-extent quakes data forces the inline-div render path (geo panel). Its leading
    // overview panels (geo map + correlation heatmap) must be marked full-width so the CSS grid
    // spans them across all columns.
    let wrk = Workdir::new("viz_smart_overview_panels_full_width_inline");
    quakes(&wrk);

    let out_html = wrk.path("quakes.html").to_string_lossy().to_string();
    let mut cmd = wrk.command("viz");
    cmd.args(["smart", "quakes.csv", "-o", &out_html]);
    wrk.assert_success(&mut cmd);

    let html = wrk.read_to_string("quakes.html").unwrap();
    // the inline path is in use, with a full-width CSS rule and at least one full-width cell
    assert!(html.contains(r#"class="qsv-viz-grid""#));
    assert!(
        html.contains("grid-column: 1 / -1;"),
        "the full-width CSS rule should be present: {html}"
    );
    assert!(
        html.contains(r#"class="qsv-viz-cell full-width""#),
        "the overview (geo) panel cell should be marked full-width: {html}"
    );
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

// On image export the mapbox tile map can't be rendered, so a local-extent coordinate pair is
// drawn as an offline ScatterGeo projection fit to the extent (the lat/lon columns are consumed by
// that geo panel, not charted as distributions). A coordinates-only dataset must still produce a
// chart. Requires a browser/webdriver, so ignored by default.
#[cfg(feature = "viz_static")]
#[test]
#[ignore = "requires a browser/webdriver for plotly static export"]
fn viz_static_geo_map_rendered_on_image_export() {
    let wrk = Workdir::new("viz_static_geo_map_rendered_on_image_export");
    // valid in-range lat/lon are the ONLY chartable columns; the offline geo map renders them, so
    // the export still produces a chart (the LA-area extent fits a local Mercator view)
    let mut rows = String::from("lat,lon\n");
    for i in 0..60 {
        rows.push_str(&format!(
            "{:.4},{:.4}\n",
            34.0 + i as f64 * 0.1,
            -118.0 + i as f64 * 0.1
        ));
    }
    wrk.create_from_string("geo.csv", &rows);

    let out_svg = wrk.path("geo.svg").to_string_lossy().to_string();
    let mut cmd = wrk.command("viz");
    cmd.args(["smart", "geo.csv", "-o", &out_svg]);
    wrk.assert_success(&mut cmd);

    let svg = wrk.read_to_string("geo.svg").unwrap();
    assert!(
        svg.contains("<svg") || svg.contains("<?xml"),
        "image export of a coords-only dataset should render the offline geo map"
    );
}

// A US-spanning coordinate extent must export without panicking (exercises the `albers usa`
// projection branch of the static geo map and the geo-subplot JSON injection alongside other
// panels). Requires a browser/webdriver, so ignored by default.
#[cfg(feature = "viz_static")]
#[test]
#[ignore = "requires a browser/webdriver for plotly static export"]
fn viz_static_us_extent_geo_albersusa() {
    let wrk = Workdir::new("viz_static_us_extent_geo_albersusa");
    // coordinates spread across the continental US (lon ~-122..-71, lat ~33..47) -> albers usa,
    // plus a low-cardinality categorical so the dashboard mixes a geo subplot with a bar panel
    let lats = [40.7_f64, 34.0, 41.9, 29.8, 33.4, 39.7, 47.6, 25.8];
    let lons = [
        -74.0_f64, -118.2, -87.6, -95.4, -112.1, -105.0, -122.3, -80.2,
    ];
    let mut rows = String::from("lat,lon,region\n");
    for i in 0..64 {
        let j = i % lats.len();
        let region = if i % 2 == 0 { "east" } else { "west" };
        rows.push_str(&format!("{:.4},{:.4},{region}\n", lats[j], lons[j]));
    }
    wrk.create_from_string("us.csv", &rows);

    let out_svg = wrk.path("us.svg").to_string_lossy().to_string();
    let mut cmd = wrk.command("viz");
    cmd.args(["smart", "us.csv", "-o", &out_svg]);
    wrk.assert_success(&mut cmd);

    let svg = wrk.read_to_string("us.svg").unwrap();
    assert!(svg.contains("<svg") || svg.contains("<?xml"));
}

#[cfg(feature = "viz_static")]
#[test]
#[ignore = "requires a browser/webdriver for plotly static export"]
fn viz_static_three_numeric_no_scatter3d_panic() {
    // 3+ strongly-correlated numeric columns would add a smart Scatter3D panel; a 3D scene can't
    // render in the typed subplot grid used for image export, so it must be excluded rather than
    // panicking on `panel_trace`'s unreachable arm.
    let wrk = Workdir::new("viz_static_three_numeric_no_scatter3d_panic");
    let mut rows = String::from("a,b,c,city\n");
    for i in 0..120 {
        let a = i % 10;
        let b = a * 2 + (i % 2);
        let c = a * 3 - (i % 3);
        let city = match i % 3 {
            0 => "NYC",
            1 => "LA",
            _ => "SF",
        };
        rows.push_str(&format!("{a},{b},{c},{city}\n"));
    }
    wrk.create_from_string("metrics.csv", &rows);

    let out_svg = wrk.path("dash.svg").to_string_lossy().to_string();
    let mut cmd = wrk.command("viz");
    cmd.args(["smart", "metrics.csv", "-o", &out_svg]);
    wrk.assert_success(&mut cmd);

    let svg = wrk.read_to_string("dash.svg").unwrap();
    assert!(
        svg.contains("<svg") || svg.contains("<?xml"),
        "image export with 3+ numeric columns should render (no 3D panel) instead of panicking"
    );
}

// Static image export of >8 panels: plotly's typed Layout only has 8 axis fields, so the grid is
// assembled as raw JSON with domain-positioned xaxis9+ and rendered via StaticExporter::write_fig.
// Requires a browser/webdriver, so ignored by default.
#[cfg(feature = "viz_static")]
#[test]
#[ignore = "requires a browser/webdriver for plotly static export"]
fn viz_static_more_than_eight_panels() {
    let wrk = Workdir::new("viz_static_more_than_eight_panels");
    // 12 low-cardinality categorical columns => 12 frequency-bar panels (well past the 8 cap)
    let headers: Vec<String> = (1..=12).map(|i| format!("cat{i:02}")).collect();
    let mut rows = format!("{}\n", headers.join(","));
    for i in 0..90 {
        let cells: Vec<String> = (1..=12).map(|c| format!("v{}", (i + c) % 4)).collect();
        rows.push_str(&format!("{}\n", cells.join(",")));
    }
    wrk.create_from_string("wide.csv", &rows);

    let out_svg = wrk.path("dash.svg").to_string_lossy().to_string();
    let mut cmd = wrk.command("viz");
    cmd.args(["smart", "wide.csv", "-o", &out_svg]);
    let stderr = wrk.output_stderr(&mut cmd);
    wrk.assert_success(&mut cmd);

    // the old 8-panel ceiling warning must be gone
    assert!(
        !stderr.contains("limited to"),
        "static export should no longer cap at 8 panels: {stderr}"
    );

    let svg = wrk.read_to_string("dash.svg").unwrap();
    assert!(svg.contains("<svg") || svg.contains("<?xml"));
    // panels beyond the typed-Layout limit (their column-name titles) must be present in the image
    for late in ["cat09", "cat10", "cat11", "cat12"] {
        assert!(
            svg.contains(late),
            "panel {late} (beyond the 8-axis limit) is missing from the rendered image"
        );
    }
}

// `--max-charts` still caps the panel count for static export. Requires a browser/webdriver, so
// ignored by default.
#[cfg(feature = "viz_static")]
#[test]
#[ignore = "requires a browser/webdriver for plotly static export"]
fn viz_static_max_charts_caps_panels() {
    let wrk = Workdir::new("viz_static_max_charts_caps_panels");
    let headers: Vec<String> = (1..=12).map(|i| format!("cat{i:02}")).collect();
    let mut rows = format!("{}\n", headers.join(","));
    for i in 0..90 {
        let cells: Vec<String> = (1..=12).map(|c| format!("v{}", (i + c) % 4)).collect();
        rows.push_str(&format!("{}\n", cells.join(",")));
    }
    wrk.create_from_string("wide.csv", &rows);

    let out_svg = wrk.path("dash.svg").to_string_lossy().to_string();
    let mut cmd = wrk.command("viz");
    cmd.args(["smart", "wide.csv", "--max-charts", "4", "-o", &out_svg]);
    wrk.assert_success(&mut cmd);

    let svg = wrk.read_to_string("dash.svg").unwrap();
    // only the first 4 panels are drawn; later columns are capped out
    assert!(svg.contains("cat01"));
    assert!(
        !svg.contains("cat12"),
        "--max-charts 4 should cap panels; cat12 must not be drawn"
    );
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
    // plotly.js 3.6 hover_template (O/H/L/C readout) + defensive fallback
    assert!(html.contains("Open: %{open}"));
    assert!(html.contains(r#""hovertemplatefallback":"-""#));
    // x-unified hover scoped to ordered-x chart kinds (line/candlestick/ohlc)
    assert!(html.contains(r#""hovermode":"x unified""#));
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
    // plotly.js 3.6 hover_template (O/H/L/C readout)
    assert!(html.contains("Open: %{open}"));
    assert!(html.contains(r#""hovertemplatefallback":"-""#));
}

#[test]
fn viz_line_unified_hover() {
    // a line chart has an ordered x-axis, so build_layout opts it into `x unified` hover
    // (one tooltip per x across series). Unordered charts (scatter/bar/box) must NOT get it.
    let wrk = Workdir::new("viz_line_unified_hover");
    wrk.create_from_string(
        "t.csv",
        "date,close\n2024-01-01,10\n2024-01-02,12\n2024-01-03,11\n",
    );

    let mut cmd = wrk.command("viz");
    cmd.args(["line", "t.csv", "--x", "date", "--y", "close"]);
    let out = wrk.output(&mut cmd);
    assert!(out.status.success());
    let line_html = String::from_utf8_lossy(&out.stdout);
    assert!(line_html.contains(r#""hovermode":"x unified""#));

    // a scatter of the same data must not carry the unified hover mode
    let mut scmd = wrk.command("viz");
    scmd.args(["scatter", "t.csv", "--x", "date", "--y", "close"]);
    let sout = wrk.output(&mut scmd);
    assert!(sout.status.success());
    let scatter_html = String::from_utf8_lossy(&sout.stdout);
    assert!(!scatter_html.contains(r#""hovermode":"x unified""#));
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
        let b = (i % 9) * 2; // perfectly linear in a => corr(metric_a, metric_b) = 1.00
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
    // ... in-cell r value labels as annotations: metric_b vs metric_a is a perfect 1.00, shown in
    // the kept LOWER triangle (the redundant upper triangle and the trivial 1.0 diagonal are masked
    // to NaN / blank, so this 1.00 is the off-diagonal cell, not a self-correlation) ...
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
fn viz_smart_scatter3d_triple_panel() {
    let wrk = Workdir::new("viz_smart_scatter3d_triple_panel");
    // a moderately-correlated pair (a,b: r~0.78, below the collinear cutoff) plus a third column c
    // that is nearly independent of both => `viz smart` adds a 3D scatter of the strongest pair
    // (a,b) and the LEAST-redundant third axis (c), so the cloud genuinely uses all three
    // dimensions instead of collapsing onto the a-b plane.
    let mut rows = String::from("a,b,c,city\n");
    for i in 0..120 {
        let a = i % 20;
        let b = (i % 20) as f64 + (i % 11) as f64 * 1.5;
        let c = (i * 7) % 13;
        let city = match i % 3 {
            0 => "NYC",
            1 => "LA",
            _ => "SF",
        };
        rows.push_str(&format!("{a},{b},{c},{city}\n"));
    }
    wrk.create_from_string("metrics.csv", &rows);

    let out_html = wrk.path("dash.html").to_string_lossy().to_string();
    let mut cmd = wrk.command("viz");
    cmd.args(["smart", "metrics.csv", "-o", &out_html]);
    wrk.assert_success(&mut cmd);

    let html = wrk.read_to_string("dash.html").unwrap();
    assert!(html.contains(r#""type":"heatmap""#));
    // a 3D scatter trace whose panel title names the triple; a 3D scene forces the inline page
    assert!(html.contains("<!doctype html>"));
    assert!(html.contains(r#""type":"scatter3d""#));
    assert!(html.contains("a / b / c (3D)"));
}

#[test]
fn viz_smart_no_scatter3d_when_strongest_pair_collinear() {
    // The strongest numeric pair (a,b) is perfectly collinear (b = 2a, r=1.0). A 3D built on it
    // would be a degenerate plane, so `viz smart` skips the 3D drill-down even though there are 3+
    // numeric columns. The 2D pair drill-down (and the heatmap) still render.
    let wrk = Workdir::new("viz_smart_no_scatter3d_when_strongest_pair_collinear");
    let mut rows = String::from("a,b,c,city\n");
    for i in 0..120 {
        let a = i % 20;
        let b = a * 2; // perfectly collinear with a (r = 1.0)
        let c = (i * 7) % 13; // nearly independent
        let city = match i % 3 {
            0 => "NYC",
            1 => "LA",
            _ => "SF",
        };
        rows.push_str(&format!("{a},{b},{c},{city}\n"));
    }
    wrk.create_from_string("metrics.csv", &rows);

    let out_html = wrk.path("dash.html").to_string_lossy().to_string();
    let mut cmd = wrk.command("viz");
    cmd.args(["smart", "metrics.csv", "-o", &out_html]);
    wrk.assert_success(&mut cmd);

    let html = wrk.read_to_string("dash.html").unwrap();
    assert!(html.contains(r#""type":"heatmap""#));
    assert!(
        !html.contains(r#""type":"scatter3d""#),
        "a near-collinear strongest pair should NOT get a 3D drill-down; html: {html}"
    );
}

#[test]
fn viz_smart_contour_pair_for_big_data() {
    let wrk = Workdir::new("viz_smart_contour_pair_for_big_data");
    // a strongly-correlated pair over a LARGE row count (>= SMART_CONTOUR_MIN_POINTS): the pair
    // drill-down is rendered as a 2D density contour (a scatter would overplot) rather than a
    // scatter.
    let mut rows = String::from("p,q\n");
    for i in 0..6_000 {
        let p = i % 100;
        let q = p * 2 + (i % 7);
        rows.push_str(&format!("{p},{q}\n"));
    }
    wrk.create_from_string("big.csv", &rows);

    let out_html = wrk.path("dash.html").to_string_lossy().to_string();
    let mut cmd = wrk.command("viz");
    cmd.args(["smart", "big.csv", "-o", &out_html]);
    wrk.assert_success(&mut cmd);

    let html = wrk.read_to_string("dash.html").unwrap();
    assert!(html.contains(r#""type":"heatmap""#));
    // the correlated pair is a contour density, NOT a scatter
    assert!(html.contains(r#""type":"contour""#));
    assert!(html.contains("p vs q (r="));
    assert!(!html.contains(r#""type":"scatter""#));
}

// a continuous numeric column with cardinality > 30 (so it's a box, not a frequency bar) and
// uniqueness < 0.95 (so it's not skipped as an ID). `n` rows of distinct-ish floats.
fn continuous_box_csv(rows: usize) -> String {
    let mut s = String::from("measure,grp\n");
    for i in 0..rows {
        // ~ (rows mod 400) distinct values: high cardinality, low uniqueness for large `rows`
        let v = (i % 400) as f64 * 0.37 + (i % 7) as f64 * 0.013;
        let grp = if i % 2 == 0 { "a" } else { "b" };
        s.push_str(&format!("{v:.3},{grp}\n"));
    }
    s
}

#[test]
fn viz_smart_box_points_heuristic_small_overlays_all() {
    // small dataset (<= SMART_BOX_ALL_MAX rows): the size heuristic overlays every sample point on
    // the box (no explicit --box-points needed).
    let wrk = Workdir::new("viz_smart_box_points_heuristic_small_overlays_all");
    wrk.create_from_string("d.csv", &continuous_box_csv(100));

    let mut cmd = wrk.command("viz");
    cmd.args(["smart", "d.csv"]);
    let out = wrk.output(&mut cmd);
    assert!(out.status.success());

    let html = String::from_utf8_lossy(&out.stdout);
    assert!(html.contains(r#""type":"box""#));
    assert!(html.contains(r#""boxpoints":"all""#));
    // box hover shows only the y stats ("median: ...") — NOT plotly's default
    // "(<trace name>, median: ...)" which repeats the long column name on every stat line
    assert!(html.contains(r#""hoverinfo":"y""#));
}

#[test]
fn viz_smart_box_points_heuristic_large_none() {
    // large dataset (> SMART_BOX_OUTLIERS_MAX rows): the heuristic draws NO points and the box
    // stays a cache-only quartile summary (no `boxpoints` key on the trace).
    let wrk = Workdir::new("viz_smart_box_points_heuristic_large_none");
    wrk.create_from_string("d.csv", &continuous_box_csv(12_000));

    let out_html = wrk.path("dash.html").to_string_lossy().to_string();
    let mut cmd = wrk.command("viz");
    cmd.args(["smart", "d.csv", "-o", &out_html]);
    wrk.assert_success(&mut cmd);

    let html = wrk.read_to_string("dash.html").unwrap();
    assert!(html.contains(r#""type":"box""#));
    // the JSON key `"boxpoints":` is only emitted for raw boxes; a cache-only box omits it
    assert!(!html.contains(r#""boxpoints":"#));
    // even the cache-only quartile box shows only y stats in the hover (no repeated column name)
    assert!(html.contains(r#""hoverinfo":"y""#));
}

#[test]
fn viz_smart_box_points_explicit_overrides_heuristic() {
    // an explicit --box-points wins over the size heuristic: `none` keeps the cache-only box even
    // though the small dataset would otherwise overlay all points.
    let wrk = Workdir::new("viz_smart_box_points_explicit_overrides_heuristic");
    wrk.create_from_string("d.csv", &continuous_box_csv(100));

    let mut none_cmd = wrk.command("viz");
    none_cmd.args(["smart", "d.csv", "--box-points", "none"]);
    let none_out = wrk.output(&mut none_cmd);
    assert!(none_out.status.success());
    let none_html = String::from_utf8_lossy(&none_out.stdout);
    assert!(none_html.contains(r#""type":"box""#));
    assert!(!none_html.contains(r#""boxpoints":"#));

    // and an explicit `outliers` forces outliers regardless of the small size (which would be
    // `all`)
    let mut out_cmd = wrk.command("viz");
    out_cmd.args(["smart", "d.csv", "--box-points", "outliers"]);
    let out = wrk.output(&mut out_cmd);
    assert!(out.status.success());
    let html = String::from_utf8_lossy(&out.stdout);
    assert!(html.contains(r#""boxpoints":"outliers""#));
}

/// A single continuous numeric column (high enough cardinality for a box) of `bulk` tightly
/// clustered values in ~[100,150), plus `n_out` copies of `outlier_val` far beyond the Tukey
/// fences. With `bulk` >> `n_out`, the quartiles are set by the cluster so `outlier_val` reads as
/// a genuine outlier.
fn box_with_outliers_csv(bulk: usize, n_out: usize, outlier_val: f64) -> String {
    let mut s = String::from("measure\n");
    for i in 0..bulk {
        let v = 100.0 + (i % 500) as f64 * 0.1; // ~500 distinct -> continuous
        s.push_str(&format!("{v:.3}\n"));
    }
    for _ in 0..n_out {
        s.push_str(&format!("{outlier_val}\n"));
    }
    s
}

#[test]
fn viz_smart_box_outliers_large() {
    // a > SMART_BOX_OUTLIERS_MAX (10k) column WITH outliers: a precomputed quartile box plus the
    // out-of-fence values overlaid as native box points (no scatter overlay, no full re-embed).
    let wrk = Workdir::new("viz_smart_box_outliers_large");
    wrk.create_from_string("d.csv", &box_with_outliers_csv(12_000, 10, 99999.0));

    let out_html = wrk.path("dash.html").to_string_lossy().to_string();
    let mut cmd = wrk.command("viz");
    cmd.args(["smart", "d.csv", "-o", &out_html]);
    wrk.assert_success(&mut cmd);

    let html = wrk.read_to_string("dash.html").unwrap();
    assert!(html.contains(r#""type":"box""#));
    // native outlier points require a 2D `y` ([[...]]); a 1D y renders the box but drops the points
    assert!(html.contains(r#""y":[["#));
    assert!(html.contains(r#""boxpoints":"all""#));
    // the box is precomputed (carries q1), NOT recomputed from the outlier points
    assert!(html.contains(r#""q1":["#));
    // the injected extreme is embedded as an outlier point
    assert!(html.contains("99999"));
}

#[test]
fn viz_smart_box_no_outliers_large() {
    // a > 10k column with NO Tukey outliers (uniform spread) stays a cache-only quartile box:
    // a box trace, but no native points (no boxpoints key, no 2D y) and so no data pass.
    let wrk = Workdir::new("viz_smart_box_no_outliers_large");
    wrk.create_from_string("d.csv", &continuous_box_csv(12_000));

    let out_html = wrk.path("dash.html").to_string_lossy().to_string();
    let mut cmd = wrk.command("viz");
    cmd.args(["smart", "d.csv", "-o", &out_html]);
    wrk.assert_success(&mut cmd);

    let html = wrk.read_to_string("dash.html").unwrap();
    assert!(html.contains(r#""type":"box""#));
    assert!(!html.contains(r#""boxpoints":"#));
    assert!(!html.contains(r#""y":[["#));
}

#[test]
fn viz_smart_box_outliers_capped() {
    // 6000 outliers but only SMART_BOX_OUTLIERS_CAP (5000) are embedded, keeping the HTML bounded.
    // bulk (60k) >> outliers (6k) so q3 stays inside the cluster, 99999 reads as a heavy-tailed
    // outlier (leptokurtic -> stays a box, not flagged bimodal), and the column stays a box plot.
    let wrk = Workdir::new("viz_smart_box_outliers_capped");
    wrk.create_from_string("d.csv", &box_with_outliers_csv(60_000, 6_000, 99999.0));

    let out_html = wrk.path("dash.html").to_string_lossy().to_string();
    let mut cmd = wrk.command("viz");
    cmd.args(["smart", "d.csv", "-o", &out_html]);
    wrk.assert_success(&mut cmd);

    let html = wrk.read_to_string("dash.html").unwrap();
    assert!(html.contains(r#""type":"box""#));
    // the distinctive outlier value appears ~the cap number of times (5000 of 6000) — well below
    // the uncapped 6000, confirming the cap. A few extra matches come from the plotly.js bundle.
    let n = html.matches("99999").count();
    assert!(
        (5000..=5050).contains(&n),
        "expected ~5000 (cap) embedded outliers, not the uncapped 6000; got {n}"
    );
}

#[test]
fn viz_smart_box_explicit_none_large() {
    // explicit `--box-points none` keeps a cache-only quartile box even on a large file WITH
    // outliers: no points, no pass — guards the user-intent path.
    let wrk = Workdir::new("viz_smart_box_explicit_none_large");
    wrk.create_from_string("d.csv", &box_with_outliers_csv(12_000, 10, 99999.0));

    let out_html = wrk.path("dash.html").to_string_lossy().to_string();
    let mut cmd = wrk.command("viz");
    cmd.args(["smart", "d.csv", "--box-points", "none", "-o", &out_html]);
    wrk.assert_success(&mut cmd);

    let html = wrk.read_to_string("dash.html").unwrap();
    assert!(html.contains(r#""type":"box""#));
    // cache-only path: no native points (no boxpoints, no 2D `y` array). (We can't assert the
    // outlier value is absent — a cache-only box draws its whisker to the observed max, which here
    // IS the outlier value.)
    assert!(!html.contains(r#""boxpoints":"#));
    assert!(!html.contains(r#""y":[["#));
}

#[test]
fn viz_smart_two_outlier_boxes_single_pass() {
    // two large continuous columns, each with distinctive outliers, are collected (fence-filtered)
    // for BOTH columns in the same single pass; assert each column's outliers are embedded.
    let wrk = Workdir::new("viz_smart_two_outlier_boxes_single_pass");
    let mut s = String::from("a,b\n");
    for i in 0..12_000 {
        let va = if i < 8 {
            88888.0
        } else {
            100.0 + (i % 500) as f64 * 0.1
        };
        let vb = if i < 8 {
            77777.0
        } else {
            200.0 + (i % 500) as f64 * 0.1
        };
        s.push_str(&format!("{va:.3},{vb:.3}\n"));
    }
    wrk.create_from_string("d.csv", &s);

    let out_html = wrk.path("dash.html").to_string_lossy().to_string();
    let mut cmd = wrk.command("viz");
    cmd.args(["smart", "d.csv", "-o", &out_html]);
    wrk.assert_success(&mut cmd);

    let html = wrk.read_to_string("dash.html").unwrap();
    assert!(html.contains("88888"));
    assert!(html.contains("77777"));
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
fn viz_smart_flags_nonlinear_correlation_pair() {
    let wrk = Workdir::new("viz_smart_flags_nonlinear_correlation_pair");
    // y = x^6: a perfectly monotonic but strongly curved relationship. Spearman rho ~1.0 far
    // exceeds Pearson r (~0.78), so the drill-down pair title flags it as nonlinear and shows the
    // rho — the single Pearson number alone would read as merely "moderately linear".
    let mut rows = String::from("x,y\n");
    for i in 0..120 {
        let x = i % 30;
        let y = i64::from(x).pow(6);
        rows.push_str(&format!("{x},{y}\n"));
    }
    wrk.create_from_string("curve.csv", &rows);

    let out_html = wrk.path("dash.html").to_string_lossy().to_string();
    let mut cmd = wrk.command("viz");
    cmd.args(["smart", "curve.csv", "-o", &out_html]);
    wrk.assert_success(&mut cmd);

    let html = wrk.read_to_string("dash.html").unwrap();
    assert!(
        html.contains("nonlinear"),
        "a monotonic-but-curved pair should be flagged nonlinear; html: {html}"
    );
    // the drill-down scatter is still rendered (the pair clears the |r| >= 0.5 threshold)
    assert!(html.contains(r#""type":"scatter""#));
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

#[test]
fn viz_smart_timeseries_skips_non_finite() {
    let wrk = Workdir::new("viz_smart_timeseries_skips_non_finite");
    // a time-series numeric column with NaN and inf rows interleaved among finite ones. parse_f64
    // accepts "NaN"/"inf", but a single non-finite value would poison LTTB's bucket averages and
    // area comparisons -> the builder must drop them at collection so the rendered series stays
    // finite. (serde_json serializes a non-finite f64 as `null`, which would also be a chart gap.)
    let rows = "txn_date,revenue,region\n2021-01-01,1000,east\n2021-01-02,NaN,west\n2021-01-03,\
                1200,east\n2021-01-04,inf,west\n2021-01-05,1400,east\n";
    wrk.create_from_string("sales.csv", rows);

    let out_html = wrk.path("dash.html").to_string_lossy().to_string();
    let mut cmd = wrk.command("viz");
    cmd.args(["smart", "sales.csv", "-o", &out_html]);
    wrk.assert_success(&mut cmd);

    let html = wrk.read_to_string("dash.html").unwrap();
    // the time-series panel is still drawn ...
    assert!(html.contains(r#""mode":"lines""#));
    assert!(html.contains("revenue over txn_date"));
    // ... and the NaN/inf rows are gone: the line-trace y-array holds only the 3 finite values,
    // not the `[1000.0,null,1200.0,null,1400.0]` it would be if non-finite rows slipped through.
    assert!(html.contains(r#""y":[1000.0,1200.0,1400.0]"#));
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
    // isolate from any inherited QSV_MAPBOX_TOKEN, which would satisfy the token
    // requirement via the env-var fallback and make this error-path test fail.
    cmd.env_remove("QSV_MAPBOX_TOKEN");
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
fn viz_map_mapbox_token_from_env() {
    let wrk = Workdir::new("viz_map_mapbox_token_from_env");
    quakes(&wrk);

    // QSV_MAPBOX_TOKEN satisfies the token requirement for mapbox-hosted styles,
    // exactly as if --mapbox-token had been passed on the command line.
    let mut cmd = wrk.command("viz");
    cmd.env("QSV_MAPBOX_TOKEN", "pk.test_env_token_value");
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
    assert!(out.status.success());

    let html = String::from_utf8_lossy(&out.stdout);
    // the env-supplied token is embedded in the mapbox layout
    assert!(html.contains("pk.test_env_token_value"));
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
    // smart auto-detects the lat/lon pair and adds a geographic panel; it forces the inline
    // (self-contained HTML page) render path. The quakes data spans the globe, so the panel is
    // rendered as an offline ScatterGeo projection world-overview (not a zoomed mapbox tile map).
    assert!(html.contains("<!doctype html>"));
    assert!(html.contains(r#""type":"scattergeo""#));
    assert!(!html.contains(r#""type":"scattermapbox""#));
}

// A numeric administrative code (40 distinct values, > the categorical cardinality threshold) is
// charted as a box plot by the statistical heuristic, because it looks like a continuous measure.
// A describegpt dictionary that tags it `content_type: category` routes it to a frequency bar
// instead — and being categorical, it's also excluded from the numeric/correlation pool.
#[test]
fn viz_smart_dictionary_recodes_numeric_to_bar() {
    let wrk = Workdir::new("viz_smart_dictionary_recodes_numeric_to_bar");
    let mut rows = String::from("zone,status\n");
    for i in 0..200 {
        let zone = i % 40; // 40 distinct integer codes
        let status = match i % 3 {
            0 => "Open",
            1 => "Closed",
            _ => "Pending",
        };
        rows.push_str(&format!("{zone},{status}\n"));
    }
    wrk.create_from_string("codes.csv", &rows);

    // WITHOUT a dictionary: the heuristic treats `zone` as a continuous numeric -> box plot.
    let mut cmd = wrk.command("viz");
    cmd.args(["smart", "codes.csv"]);
    let out = wrk.output(&mut cmd);
    assert!(out.status.success());
    let html = String::from_utf8_lossy(&out.stdout);
    assert!(
        html.contains(r#""type":"box""#),
        "zone should be a box without a dictionary"
    );

    // WITH a dictionary tagging `zone` as a category: it becomes a frequency bar, no box.
    wrk.create_from_string(
        "dict.json",
        r#"{"Dictionary":{"response":{"fields":[
            {"name":"zone","type":"Integer","content_type":"category"},
            {"name":"status","type":"String","content_type":"category"}
        ]}}}"#,
    );
    let mut cmd = wrk.command("viz");
    cmd.args(["smart", "codes.csv", "--dictionary"])
        .arg(wrk.path("dict.json"));
    let out = wrk.output(&mut cmd);
    assert!(out.status.success());
    let html = String::from_utf8_lossy(&out.stdout);
    assert!(
        !html.contains(r#""type":"box""#),
        "zone should be a bar (not a box) with the dictionary"
    );
    assert!(html.contains(r#""type":"bar""#));
}

// A bad/missing --dictionary path must not abort: it warns and degrades to the stats-only
// dashboard.
#[test]
fn viz_smart_dictionary_missing_file_soft_falls_back() {
    let wrk = Workdir::new("viz_smart_dictionary_missing_file_soft_falls_back");
    let mut rows = String::from("status\n");
    for i in 0..30 {
        rows.push_str(if i % 2 == 0 { "Open\n" } else { "Closed\n" });
    }
    wrk.create_from_string("d.csv", &rows);

    let mut cmd = wrk.command("viz");
    cmd.args(["smart", "d.csv", "--dictionary", "does_not_exist.json"]);
    let out = wrk.output(&mut cmd);
    // soft fallback: still produces a dashboard
    assert!(out.status.success());
    let html = String::from_utf8_lossy(&out.stdout);
    assert!(html.contains(r#""type":"bar""#));
}

// A describegpt --format jsonschema dictionary (the channel `--dictionary infer` produces): a
// numeric admin code tagged `x-qsv.concept = geo.census_tract` routes to a bar (not a box), and the
// human `title` becomes the panel title.
#[test]
fn viz_smart_dictionary_jsonschema_routes_and_labels() {
    let wrk = Workdir::new("viz_smart_dictionary_jsonschema_routes_and_labels");
    let mut rows = String::from("census_tract,status\n");
    for i in 0..200 {
        let tract = i % 40; // 40 distinct integer codes -> a box without semantics
        let status = match i % 3 {
            0 => "Open",
            1 => "Closed",
            _ => "Pending",
        };
        rows.push_str(&format!("{tract},{status}\n"));
    }
    wrk.create_from_string("codes.csv", &rows);

    // WITHOUT a dictionary: census_tract (40 distinct ints) -> box plot
    let mut cmd = wrk.command("viz");
    cmd.args(["smart", "codes.csv"]);
    let out = wrk.output(&mut cmd);
    assert!(out.status.success());
    let html = String::from_utf8_lossy(&out.stdout);
    assert!(
        html.contains(r#""type":"box""#),
        "census_tract should be a box without a dictionary"
    );

    // WITH a jsonschema dictionary: concept geo.census_tract (a place key) -> bar, label via
    // `title`
    wrk.create_from_string(
        "dict.schema.json",
        r#"{
          "$schema": "https://json-schema.org/draft/2020-12/schema",
          "type": "object",
          "properties": {
            "census_tract": { "type": ["integer","null"], "title": "Census Tract",
              "x-qsv": { "qsv_type": "Integer", "role": "dimension", "concept": "geo.census_tract" } },
            "status": { "type": "string", "title": "Case Status",
              "x-qsv": { "qsv_type": "String", "role": "dimension", "concept": "category.status" } }
          },
          "x-qsv": { "grain": "one row = one service request" }
        }"#,
    );
    let mut cmd = wrk.command("viz");
    cmd.args(["smart", "codes.csv", "--dictionary"])
        .arg(wrk.path("dict.schema.json"));
    let out = wrk.output(&mut cmd);
    assert!(out.status.success());
    let html = String::from_utf8_lossy(&out.stdout);
    assert!(
        !html.contains(r#""type":"box""#),
        "census_tract should be a bar (not a box) with the jsonschema dictionary"
    );
    assert!(html.contains(r#""type":"bar""#));
    // the human labels from `title` become panel titles
    assert!(
        html.contains("Census Tract"),
        "label should title the panel"
    );
    assert!(html.contains("Case Status"));
}

// `--dictionary-context` only applies to `--dictionary infer` (it's forwarded to describegpt as
// --context-file). When reading an existing dictionary file it's ignored with a warning, and the
// file dictionary still drives the dashboard. (The infer passthrough itself needs a live LLM.)
#[test]
fn viz_smart_dictionary_context_ignored_with_file_dict() {
    let wrk = Workdir::new("viz_smart_dictionary_context_ignored_with_file_dict");
    let mut rows = String::from("zone,status\n");
    for i in 0..200 {
        let zone = i % 40;
        let status = if i % 2 == 0 { "Open" } else { "Closed" };
        rows.push_str(&format!("{zone},{status}\n"));
    }
    wrk.create_from_string("codes.csv", &rows);
    wrk.create_from_string(
        "dict.schema.json",
        r#"{
          "$schema": "https://json-schema.org/draft/2020-12/schema",
          "type": "object",
          "properties": {
            "zone": { "type": ["integer","null"], "title": "Zone",
              "x-qsv": { "qsv_type": "Integer", "role": "dimension", "concept": "geo.census_tract" } },
            "status": { "type": "string", "title": "Status",
              "x-qsv": { "qsv_type": "String", "role": "dimension", "concept": "category.status" } }
          }
        }"#,
    );
    wrk.create_from_string("ctx.md", "Zone is an administrative district code.\n");

    let out_html = wrk.path("d.html").to_string_lossy().to_string();
    let mut cmd = wrk.command("viz");
    cmd.args(["smart", "codes.csv", "--dictionary"])
        .arg(wrk.path("dict.schema.json"))
        .arg("--dictionary-context")
        .arg(wrk.path("ctx.md"))
        .args(["-o", &out_html]);
    let out = wrk.output(&mut cmd);
    assert!(out.status.success());
    // context is ignored (with a warning) when reading an existing dictionary file
    let stderr = String::from_utf8_lossy(&out.stderr);
    assert!(
        stderr.contains("--dictionary-context"),
        "expected an ignore warning on stderr; got: {stderr}"
    );
    // the file dictionary still routes zone -> bar (not a box)
    let html = wrk.read_to_string("d.html").unwrap();
    assert!(html.contains(r#""type":"bar""#));
    assert!(!html.contains(r#""type":"box""#));
}

// Coordinates with non-standard headers (`X Coordinate` / `Y Coordinate`) aren't found by the
// header-name heuristic, so without a dictionary no map renders and they're charted as numeric
// distributions. A jsonschema dictionary tagging them geo.latitude/geo.longitude must render the
// map (and so NOT chart them as box/histogram distributions).
#[test]
fn viz_smart_dictionary_maps_nonstandard_coord_names() {
    let wrk = Workdir::new("viz_smart_dictionary_maps_nonstandard_coord_names");
    let mut rows = String::from("Y Coordinate,X Coordinate,category\n");
    for i in 0..60 {
        let lat = 34.00 + (i as f64) * 0.01; // local LA-ish cluster, all in-range
        let lon = -118.40 + (i as f64) * 0.01;
        let cat = match i % 3 {
            0 => "A",
            1 => "B",
            _ => "C",
        };
        rows.push_str(&format!("{lat:.4},{lon:.4},{cat}\n"));
    }
    wrk.create_from_string("xy.csv", &rows);

    // WITHOUT a dictionary: names unknown -> no map; the coordinates fall through to box panels.
    let mut cmd = wrk.command("viz");
    cmd.args(["smart", "xy.csv"]);
    let out = wrk.output(&mut cmd);
    assert!(out.status.success());
    let html = String::from_utf8_lossy(&out.stdout);
    assert!(
        !html.contains(r#""type":"scattermapbox""#),
        "no map should render for non-standard coord names without a dictionary"
    );
    assert!(
        html.contains(r#""type":"box""#),
        "without a dictionary the coords are charted as distributions"
    );

    // WITH a jsonschema dictionary tagging them geo.latitude/geo.longitude: the map renders and the
    // coordinates are consumed by it (not charted as their own distributions).
    wrk.create_from_string(
        "dict.schema.json",
        r#"{
          "$schema": "https://json-schema.org/draft/2020-12/schema",
          "type": "object",
          "properties": {
            "Y Coordinate": { "type": "number", "title": "Y Coordinate",
              "x-qsv": { "qsv_type": "Float", "role": "dimension", "concept": "geo.latitude" } },
            "X Coordinate": { "type": "number", "title": "X Coordinate",
              "x-qsv": { "qsv_type": "Float", "role": "dimension", "concept": "geo.longitude" } },
            "category": { "type": "string", "title": "Category",
              "x-qsv": { "qsv_type": "String", "role": "dimension", "concept": "category.type" } }
          }
        }"#,
    );
    let mut cmd = wrk.command("viz");
    cmd.args(["smart", "xy.csv", "--dictionary"])
        .arg(wrk.path("dict.schema.json"));
    let out = wrk.output(&mut cmd);
    assert!(out.status.success());
    let html = String::from_utf8_lossy(&out.stdout);
    assert!(
        html.contains(r#""type":"scattermapbox""#),
        "map should render from the dictionary geo.latitude/geo.longitude tags; html: {html}"
    );
    assert!(
        !html.contains(r#""type":"box""#) && !html.contains(r#""type":"histogram""#),
        "dictionary-mapped coords must not also be charted as distributions; html: {html}"
    );
}

// A date column with NO numeric measure yields a count-over-time line (records per period) — the
// "volume over time" overview. Works without a dictionary.
#[test]
fn viz_smart_count_over_time_without_measure() {
    let wrk = Workdir::new("viz_smart_count_over_time_without_measure");
    let mut rows = String::from("created_date,status\n");
    for i in 0..60 {
        let day = (i % 28) + 1;
        let status = if i % 2 == 0 { "Open" } else { "Closed" };
        rows.push_str(&format!("2021-03-{day:02},{status}\n"));
    }
    wrk.create_from_string("events.csv", &rows);

    let out_html = wrk.path("dash.html").to_string_lossy().to_string();
    let mut cmd = wrk.command("viz");
    cmd.args(["smart", "events.csv", "-o", &out_html]);
    wrk.assert_success(&mut cmd);

    let html = wrk.read_to_string("dash.html").unwrap();
    // a count line over the date axis, titled "records over <date>"
    assert!(html.contains(r#""mode":"lines""#));
    assert!(html.contains(r#""type":"date""#));
    assert!(
        html.contains("records over created_date"),
        "count-over-time should be titled 'records over created_date'; html: {html}"
    );
}

// The dataset `grain` from a jsonschema dictionary names the count-over-time unit ("permit
// application" instead of "records"), and a `time.created_at` concept selects the canonical x-axis.
#[test]
fn viz_smart_dictionary_grain_labels_count() {
    let wrk = Workdir::new("viz_smart_dictionary_grain_labels_count");
    let mut rows = String::from("requested_on,status\n");
    for i in 0..60 {
        let day = (i % 28) + 1;
        let status = if i % 2 == 0 { "Submitted" } else { "Approved" };
        rows.push_str(&format!("2021-03-{day:02},{status}\n"));
    }
    wrk.create_from_string("permits.csv", &rows);

    wrk.create_from_string(
        "dict.schema.json",
        r#"{
          "$schema": "https://json-schema.org/draft/2020-12/schema",
          "type": "object",
          "properties": {
            "requested_on": { "type": "string", "title": "Requested On",
              "x-qsv": { "qsv_type": "Date", "role": "timestamp", "concept": "time.created_at" } },
            "status": { "type": "string", "title": "Status",
              "x-qsv": { "qsv_type": "String", "role": "dimension", "concept": "category.status" } }
          },
          "x-qsv": { "grain": "one row = one permit application" }
        }"#,
    );

    let out_html = wrk.path("dash.html").to_string_lossy().to_string();
    let mut cmd = wrk.command("viz");
    cmd.args(["smart", "permits.csv", "--dictionary"])
        .arg(wrk.path("dict.schema.json"))
        .args(["-o", &out_html]);
    wrk.assert_success(&mut cmd);

    let html = wrk.read_to_string("dash.html").unwrap();
    assert!(html.contains(r#""mode":"lines""#));
    // grain names the count unit; the date axis uses the dictionary label ("Requested On"), not the
    // raw header.
    assert!(
        html.contains("permit application over Requested On"),
        "grain should name the count unit and the date label should be the dictionary title; \
         html: {html}"
    );
}

#[test]
fn viz_smart_antimeridian_cluster_stays_local_map() {
    // A tight cluster straddling the +/-180 antimeridian has a small TRUE longitude span but a huge
    // raw max-min span. The global/local test must use the antimeridian-aware span, so this stays a
    // local mapbox tile map rather than being misclassified as a world ScatterGeo overview.
    let wrk = Workdir::new("viz_smart_antimeridian_cluster_stays_local_map");
    let lons = [177.0_f64, 178.0, 179.0, -179.0, -178.0];
    let mut rows = String::from("lat,lon,grp\n");
    for i in 0..60 {
        let lat = -17.0 + (i % 5) as f64 * 0.1;
        let lon = lons[i % lons.len()];
        let grp = if i % 2 == 0 { "a" } else { "b" };
        rows.push_str(&format!("{lat:.3},{lon:.3},{grp}\n"));
    }
    wrk.create_from_string("fiji.csv", &rows);

    let mut cmd = wrk.command("viz");
    cmd.args(["smart", "fiji.csv"]);
    let out = wrk.output(&mut cmd);
    assert!(out.status.success());

    let html = String::from_utf8_lossy(&out.stdout);
    // local extent (true span ~5 deg) => mapbox tile map, NOT a world projection overview
    assert!(html.contains(r#""type":"scattermapbox""#));
    assert!(!html.contains(r#""type":"scattergeo""#));
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

#[test]
fn viz_geo_basic() {
    let wrk = Workdir::new("viz_geo_basic");
    quakes(&wrk);

    let mut cmd = wrk.command("viz");
    cmd.args(["geo", "quakes.csv", "--lat", "lat", "--lon", "lon"]);
    let out = wrk.output(&mut cmd);
    assert!(out.status.success());

    let html = String::from_utf8_lossy(&out.stdout);
    // a token-free ScatterGeo point map on a projection basemap (no tiles)
    assert!(html.contains("Plotly.newPlot"));
    assert!(html.contains(r#""type":"scattergeo""#));
    // default projection is natural-earth, with land/countries drawn
    assert!(html.contains(r#""type":"natural earth""#));
    assert!(html.contains(r#""showcountries":true"#));
    // higher-detail 1:50,000,000 base layers (coastlines/borders) via GeoResolution
    assert!(html.contains(r#""resolution":50"#));
}

#[test]
fn viz_geo_projection_and_color() {
    let wrk = Workdir::new("viz_geo_projection_and_color");
    quakes(&wrk);

    let mut cmd = wrk.command("viz");
    cmd.args([
        "geo",
        "quakes.csv",
        "--lat",
        "lat",
        "--lon",
        "lon",
        "--color",
        "magnitude",
        "--projection",
        "orthographic",
    ]);
    let out = wrk.output(&mut cmd);
    assert!(out.status.success());

    let html = String::from_utf8_lossy(&out.stdout);
    assert!(html.contains(r#""type":"scattergeo""#));
    assert!(html.contains(r#""type":"orthographic""#));
    // --color maps a numeric column onto a continuous colorscale with a colorbar
    assert!(html.contains(r#""colorscale":"Viridis""#));
    assert!(html.contains(r#""colorbar":{"title":{"text":"magnitude"#));
}

#[test]
fn viz_geo_series_traces() {
    let wrk = Workdir::new("viz_geo_series_traces");
    quakes(&wrk);

    let mut cmd = wrk.command("viz");
    cmd.args([
        "geo",
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
    assert!(html.contains(r#""type":"scattergeo""#));
    // one trace per region category, with a legend
    assert!(html.contains(r#""name":"Asia""#));
    assert!(html.contains(r#""name":"Europe""#));
    assert!(html.contains(r#""showlegend":true"#));
}

#[test]
fn viz_geo_bad_projection_errors() {
    let wrk = Workdir::new("viz_geo_bad_projection_errors");
    quakes(&wrk);

    let mut cmd = wrk.command("viz");
    cmd.args([
        "geo",
        "quakes.csv",
        "--lat",
        "lat",
        "--lon",
        "lon",
        "--projection",
        "bogus",
    ]);
    let got = wrk.output_stderr(&mut cmd);
    assert!(got.contains("Unknown --projection"));
    wrk.assert_err(&mut cmd);
}

#[test]
fn viz_scatter3d_basic() {
    let wrk = Workdir::new("viz_scatter3d_basic");
    quakes(&wrk);

    let mut cmd = wrk.command("viz");
    cmd.args([
        "scatter3d",
        "quakes.csv",
        "--x",
        "lon",
        "--y",
        "lat",
        "--z",
        "magnitude",
    ]);
    let out = wrk.output(&mut cmd);
    assert!(out.status.success());

    let html = String::from_utf8_lossy(&out.stdout);
    assert!(html.contains(r#""type":"scatter3d""#));
    assert!(html.contains(r#""mode":"markers""#));
    // a 3D scene layout with z-axis title from the --z column
    assert!(html.contains(r#""scene""#));
    assert!(html.contains(r#""text":"magnitude"#));
}

#[test]
fn viz_scatter3d_color_encoding() {
    let wrk = Workdir::new("viz_scatter3d_color_encoding");
    quakes(&wrk);

    let mut cmd = wrk.command("viz");
    cmd.args([
        "scatter3d",
        "quakes.csv",
        "--x",
        "lon",
        "--y",
        "lat",
        "--z",
        "magnitude",
        "--color",
        "depth_km",
    ]);
    let out = wrk.output(&mut cmd);
    assert!(out.status.success());

    let html = String::from_utf8_lossy(&out.stdout);
    assert!(html.contains(r#""type":"scatter3d""#));
    assert!(html.contains(r#""colorscale":"Viridis""#));
    assert!(html.contains(r#""colorbar":{"title":{"text":"depth_km"#));
}

#[test]
fn viz_contour_density() {
    let wrk = Workdir::new("viz_contour_density");
    quakes(&wrk);

    let mut cmd = wrk.command("viz");
    cmd.args([
        "contour",
        "quakes.csv",
        "--x",
        "lon",
        "--y",
        "lat",
        "--bins",
        "10",
    ]);
    let out = wrk.output(&mut cmd);
    assert!(out.status.success());

    let html = String::from_utf8_lossy(&out.stdout);
    assert!(html.contains(r#""type":"contour""#));
    assert!(html.contains(r#""colorscale":"Viridis""#));
    // x/y axis titles come from the column names
    assert!(html.contains(r#""text":"lon"#));
    assert!(html.contains(r#""text":"lat"#));
}

#[test]
fn viz_contour_non_numeric_errors() {
    let wrk = Workdir::new("viz_contour_non_numeric_errors");
    quakes(&wrk);

    // `place` and `region` are non-numeric, so there are no plottable rows
    let mut cmd = wrk.command("viz");
    cmd.args(["contour", "quakes.csv", "--x", "place", "--y", "region"]);
    let got = wrk.output_stderr(&mut cmd);
    assert!(got.contains("No rows with numeric"));
    wrk.assert_err(&mut cmd);
}

#[test]
fn viz_theme_dark_applies_template() {
    let wrk = Workdir::new("viz_theme_dark_applies_template");
    fruits(&wrk);

    let mut cmd = wrk.command("viz");
    cmd.args([
        "bar",
        "fruits.csv",
        "--x",
        "Fruit",
        "--y",
        "Price",
        "--theme",
        "plotly_dark",
    ]);
    let out = wrk.output(&mut cmd);
    assert!(out.status.success());

    let html = String::from_utf8_lossy(&out.stdout);
    // the chosen built-in theme is injected as a plotly layout template ...
    assert!(html.contains(r#""template":{"layout""#));
    // ... carrying the dark theme's backgrounds
    assert!(html.contains(r##""paper_bgcolor":"#111111""##));
    assert!(html.contains(r##""plot_bgcolor":"#111111""##));
}

// a choropleth's geo subplot is theme-aware: a dark theme uses dark land + a dark geo background
// (the sea) so the map is legible on a dark page, while the default/light look stays light gray.
#[test]
fn viz_choropleth_geo_theme_aware() {
    let wrk = Workdir::new("viz_choropleth_geo_theme_aware");
    wrk.create_from_string("rg.csv", "iso3,val\nUSA,10\nCAN,5\nMEX,3\n");

    // dark theme -> dark land + painted dark ocean
    let mut cmd = wrk.command("viz");
    cmd.args([
        "choropleth",
        "rg.csv",
        "--locations",
        "iso3",
        "--value",
        "val",
        "--theme",
        "plotly_dark",
    ]);
    let out = wrk.output(&mut cmd);
    assert!(out.status.success());
    let dark = String::from_utf8_lossy(&out.stdout);
    assert!(
        dark.contains(r##""landcolor":"#2a3138""##),
        "dark land missing"
    );
    // a choropleth paints no ocean; the sea is geo.bgcolor, which carries the dark theme
    assert!(
        dark.contains(r##""bgcolor":"#111111""##),
        "dark geo background missing"
    );

    // default (no theme) -> built-in light gray land + white geo background
    let mut cmd = wrk.command("viz");
    cmd.args([
        "choropleth",
        "rg.csv",
        "--locations",
        "iso3",
        "--value",
        "val",
    ]);
    let out = wrk.output(&mut cmd);
    assert!(out.status.success());
    let light = String::from_utf8_lossy(&out.stdout);
    assert!(
        light.contains(r##""landcolor":"#d3d3d3""##),
        "light land missing"
    );
    // the dark palette must not bleed into the light/default render
    assert!(
        !light.contains("#16202b") && !light.contains("#2a3138"),
        "light path must not use the dark geo palette"
    );
}

#[test]
fn viz_no_theme_has_no_template() {
    let wrk = Workdir::new("viz_no_theme_has_no_template");
    fruits(&wrk);

    let mut cmd = wrk.command("viz");
    cmd.args(["bar", "fruits.csv", "--x", "Fruit", "--y", "Price"]);
    let out = wrk.output(&mut cmd);
    assert!(out.status.success());

    let html = String::from_utf8_lossy(&out.stdout);
    // without --theme, qsv's built-in look is used: no layout template is emitted
    assert!(!html.contains(r#""template":{"layout""#));
}

#[test]
fn viz_theme_unknown_errors() {
    let wrk = Workdir::new("viz_theme_unknown_errors");
    fruits(&wrk);

    let mut cmd = wrk.command("viz");
    cmd.args([
        "bar",
        "fruits.csv",
        "--x",
        "Fruit",
        "--y",
        "Price",
        "--theme",
        "bogus",
    ]);
    let got = wrk.output_stderr(&mut cmd);
    assert!(got.contains("Unknown --theme 'bogus'"));
    // the error lists the valid theme names
    assert!(got.contains("plotly_dark"));
    assert!(got.contains("seaborn_whitegrid"));
    wrk.assert_err(&mut cmd);
}

#[test]
fn viz_smart_theme_drives_dashboard() {
    let wrk = Workdir::new("viz_smart_theme_drives_dashboard");
    // continuous numeric (box) + categorical (bar) gives a multi-panel dashboard
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
    cmd.args([
        "smart",
        "people.csv",
        "--theme",
        "plotly_dark",
        "-o",
        &out_html,
    ]);
    wrk.assert_success(&mut cmd);

    let html = wrk.read_to_string("dash.html").unwrap();
    // the theme drives the (single-Plot grid) dashboard's look: dark template + dark
    // backgrounds, with the qsv built-in white paper override suppressed
    assert!(html.contains(r#""template":{"layout""#));
    assert!(html.contains(r##""paper_bgcolor":"#111111""##));
    assert!(!html.contains(r##""paper_bgcolor":"#FFFFFF""##));
    // qsv's hardcoded ink color must not leak into a themed dashboard's plots (e.g. the bar
    // value-labels) — it would be near-invisible on the dark background. (This dataset
    // has no correlation panel, the one place ink is intentionally kept for cell contrast.)
    // Scoped to the JSON color form: the light/dark toggle script legitimately embeds the ink
    // as its LIGHT-mode font (`font: "#2A3F5F"`), which is theme-independent page chrome, not
    // part of the serialized plot.
    assert!(!html.contains(r##""color":"#2A3F5F""##));
}

#[test]
fn viz_smart_truncates_long_bar_labels() {
    let wrk = Workdir::new("viz_smart_truncates_long_bar_labels");
    // two distinct long category names that share their first 19 characters, so both truncate
    // to the SAME display label ("Department of Trans…"). As raw x-axis tick labels these long
    // names rotate tall and squeeze the plot area (clipping the top value labels); truncation
    // must therefore be display-only via the axis ticktext, NOT applied to the bar x data —
    // otherwise the two categories would collapse onto a single ambiguous bar.
    let long_a = "Department of Transportation and Infrastructure";
    let long_b = "Department of Transparency and Public Records";
    let mut rows = String::from("agency,val\n");
    for i in 0..60 {
        let agency = if i % 2 == 0 { long_a } else { long_b };
        rows.push_str(&format!("\"{agency}\",{}\n", i));
    }
    wrk.create_from_string("agencies.csv", &rows);

    let out_html = wrk.path("dash.html").to_string_lossy().to_string();
    let mut cmd = wrk.command("viz");
    cmd.args(["smart", "agencies.csv", "-o", &out_html]);
    wrk.assert_success(&mut cmd);

    let html = wrk.read_to_string("dash.html").unwrap();
    // truncation is display-only: the axis uses array tickmode with truncated ticktext ...
    assert!(html.contains(r#""tickmode":"array""#));
    assert!(html.contains('…'));
    // ... while BOTH full category names remain as the bar's x data, so the two categories
    // that truncate to the same label stay distinct (not collapsed onto one bar).
    assert!(html.contains(long_a));
    assert!(html.contains(long_b));
}

#[test]
fn viz_smart_log_scale_skewed_freq_panel() {
    let wrk = Workdir::new("viz_smart_log_scale_skewed_freq_panel");
    // a low-cardinality categorical dominated by one value ("A" ~ 96%), so its frequency panel
    // has a huge dynamic range. Under --log-scale auto (the default) the panel switches to a log
    // y-axis with a "count (log)" title cue; the second, uniform column stays linear & untitled.
    let cats = ["A", "B", "C", "D", "E", "F", "G", "H", "I", "J", "K", "L"];
    let mut rows = String::from("dominated,balanced\n");
    for i in 0..2400usize {
        // ~96% "A", the rest spread thinly across the other categories -> high dynamic range
        let dominated = if i % 25 == 0 { cats[1 + (i % 11)] } else { "A" };
        rows.push_str(&format!("{dominated},{}\n", cats[i % 10]));
    }
    wrk.create_from_string("skew.csv", &rows);

    // auto (default): the dominated panel logs, the balanced one does not
    let auto_html = wrk.path("auto.html").to_string_lossy().to_string();
    let mut cmd = wrk.command("viz");
    cmd.args(["smart", "skew.csv", "-o", &auto_html]);
    wrk.assert_success(&mut cmd);
    let auto = wrk.read_to_string("auto.html").unwrap();
    assert!(auto.contains(r#""type":"log""#));
    // the y-axis title cue is present exactly once (only the dominated panel is log)
    assert_eq!(auto.matches("count (log)").count(), 1);

    // off: no log axis, no cue
    let off_html = wrk.path("off.html").to_string_lossy().to_string();
    let mut cmd = wrk.command("viz");
    cmd.args(["smart", "skew.csv", "--log-scale", "off", "-o", &off_html]);
    wrk.assert_success(&mut cmd);
    let off = wrk.read_to_string("off.html").unwrap();
    assert!(!off.contains(r#""type":"log""#));
    assert!(!off.contains("count (log)"));

    // on: both frequency panels log, so the cue appears twice
    let on_html = wrk.path("on.html").to_string_lossy().to_string();
    let mut cmd = wrk.command("viz");
    cmd.args(["smart", "skew.csv", "--log-scale", "on", "-o", &on_html]);
    wrk.assert_success(&mut cmd);
    let on = wrk.read_to_string("on.html").unwrap();
    assert_eq!(on.matches("count (log)").count(), 2);
}

#[test]
fn viz_smart_log_scale_invalid_errors() {
    let wrk = Workdir::new("viz_smart_log_scale_invalid_errors");
    fruits(&wrk);

    let out_html = wrk.path("dash.html").to_string_lossy().to_string();
    let mut cmd = wrk.command("viz");
    cmd.args([
        "smart",
        "fruits.csv",
        "--log-scale",
        "bogus",
        "-o",
        &out_html,
    ]);
    wrk.assert_err(&mut cmd);
}

#[test]
fn viz_smart_bar_numeric_categories_use_category_axis() {
    let wrk = Workdir::new("viz_smart_bar_numeric_categories_use_category_axis");
    // a low-cardinality column whose category values look NUMERIC ("2", "10", "100"). The
    // frequency-bar truncation positions ticks at integer indices 0..n, which only line up with
    // the bars if the axis is category-typed; otherwise plotly would infer a linear axis and the
    // ticks at 0/1/2 would not match bars at x=2/10/100. Force category mode for bar panels.
    let mut rows = String::from("rating,note\n");
    for i in 0..90 {
        let rating = match i % 3 {
            0 => "100",
            1 => "2",
            _ => "10",
        };
        rows.push_str(&format!("{rating},n\n"));
    }
    wrk.create_from_string("ratings.csv", &rows);

    let out_html = wrk.path("dash.html").to_string_lossy().to_string();
    let mut cmd = wrk.command("viz");
    cmd.args(["smart", "ratings.csv", "-o", &out_html]);
    wrk.assert_success(&mut cmd);

    let html = wrk.read_to_string("dash.html").unwrap();
    // the bar's x data are the numeric-looking category strings ...
    assert!(html.contains(r#""x":["#));
    // ... and the axis is forced to category mode so the array ticks align with the bars
    assert!(html.contains(r#""tickmode":"array""#));
    assert!(html.contains(r#""type":"category""#));
}

#[test]
fn viz_smart_inline_theme_drives_page_chrome() {
    let wrk = Workdir::new("viz_smart_inline_theme_drives_page_chrome");
    // 10 low-cardinality categorical columns -> 10 panels > the typed-subplot limit of 8,
    // so the inline-div HTML page renderer is used (which carries its own page chrome).
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
    cmd.args([
        "smart",
        "wide.csv",
        "--theme",
        "plotly_dark",
        "-o",
        &out_html,
    ]);
    wrk.assert_success(&mut cmd);

    let html = wrk.read_to_string("wide.html").unwrap();
    // inline-div grid renderer ...
    assert!(html.contains(r#"class="qsv-viz-grid""#));
    // ... page chrome is now CSS-variable driven (so the light/dark toggle can flip it): the
    // body references the var, and a dark theme seeds the var with its dark page color and
    // opens the toggle in dark mode by default.
    assert!(html.contains("background: var(--qsv-page-bg)"));
    assert!(html.contains("--qsv-page-bg: #111111"));
    // dark-bg themes must seed --qsv-geo-meta with a light value in :root so the caption is
    // readable even before body.qsv-dark is applied (regression: was hardcoded #4b5563 = dark
    // gray on dark background, nearly invisible). Assert the full :root block to avoid a false
    // pass from the always-present body.qsv-dark { --qsv-geo-meta: #9aa4b2 } rule.
    assert!(html.contains(
        ":root { --qsv-page-bg: #111111; --qsv-page-ink: #f2f5fa; --qsv-geo-meta: #9aa4b2; }"
    ));
    assert!(html.contains(r#"var themeDefaultMode = "dark""#));
    // and the panels themselves carry the dark template
    assert!(html.contains(r#""template":{"layout""#));
}

#[test]
fn viz_smart_grid_has_theme_toggle() {
    // the common ≤8-panel case: the single typed-Plot grid is now wrapped in qsv's own HTML
    // page so it carries the always-on light/dark toggle (plotly's to_html() has no hook).
    let wrk = Workdir::new("viz_smart_grid_has_theme_toggle");
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
    // the toggle button, its re-theming script, and the CSS-variable page chrome are present
    assert!(html.contains(r#"id="qsv-theme-toggle""#));
    assert!(html.contains("qsv-viz-theme")); // localStorage key
    assert!(html.contains(".js-plotly-plot")); // script enumerates live plots
    assert!(html.contains("Plotly.relayout"));
    assert!(html.contains("--qsv-page-bg"));
    assert!(html.contains("body.qsv-dark"));
    // light-bg default theme seeds --qsv-geo-meta with a dark value in :root (#4b5563), suitable
    // for the light page background (body.qsv-dark overrides it to #9aa4b2 for dark mode).
    assert!(html.contains("--qsv-geo-meta: #4b5563"));
    // the typed grid is now embedded inline in qsv's page (not plotly's own to_html document)
    assert!(html.contains(r#"id="qsv-viz-smart-grid""#));
    // no --theme given -> the toggle defers to the viewer's prefers-color-scheme
    assert!(html.contains(r#"var themeDefaultMode = "system""#));
    // the actual subplot grid is still there (typed-Layout multi-axis)
    assert!(html.contains(r#""xaxis2":{"#));
    // the typed plot already bakes the dashboard title into its layout, so the page <h1> is
    // suppressed (no double title); the document <title> tab is still set.
    assert!(!html.contains(r#"<h1 class="qsv-viz-title""#));
    assert!(html.contains("<title>people.csv \u{2014} data overview</title>"));
    // regression (roborev #3176): the page shell must not split the `\n{script}` escape into a
    // literal `\` + `n` before the toggle script. The toggle <script> follows clean markup.
    assert!(html.contains("<script>\n(function () {"));
    assert!(!html.contains("n<script>\n(function () {"));
    // the qsv/datHere logo links to the qsv site and embeds both theme variants (CSS-swapped).
    assert!(html.contains(r#"id="qsv-logo""#));
    assert!(html.contains(r#"href="https://qsv.dathere.com/""#));
    assert!(html.contains("qsv-logo-light"));
    assert!(html.contains("qsv-logo-dark"));
    assert!(html.contains("data:image/png;base64,"));
    // the toggle palette includes Carto mapbox styles for both modes so the tile basemap tracks
    // the theme button (mapbox*.style is relayout-ed on each toggle click)
    assert!(html.contains(r#"mapbox: "carto-positron""#));
    assert!(html.contains(r#"mapbox: "carto-darkmatter""#));
    assert!(html.contains(r#"/^mapbox\d*$/.test(k)"#));
}

#[test]
fn viz_smart_explicit_light_theme_opens_light() {
    // an explicit light --theme must open light, NOT defer to a dark-mode OS
    // (prefers-color-scheme). Only the absence of --theme falls back to "system".
    let wrk = Workdir::new("viz_smart_explicit_light_theme_opens_light");
    wrk.create_from_string("small.csv", "a,b,c\n1,x,9\n2,y,8\n3,x,7\n4,z,6\n5,y,5\n");
    let out_html = wrk.path("dash.html").to_string_lossy().to_string();
    let mut cmd = wrk.command("viz");
    cmd.args([
        "smart",
        "small.csv",
        "--theme",
        "plotly_white",
        "-o",
        &out_html,
    ]);
    wrk.assert_success(&mut cmd);
    let html = wrk.read_to_string("dash.html").unwrap();
    assert!(html.contains(r#"var themeDefaultMode = "light""#));
    assert!(!html.contains(r#"var themeDefaultMode = "system""#));
}

#[test]
fn viz_smart_embeds_plotly_once_without_mathjax() {
    // smart dashboards embed plotly.js exactly once, and DROP the ~2MB tex-svg MathJax bundle
    // that plotly's offline_js_sources() also embeds (dashboards render plain-text labels, never
    // LaTeX). Checked on both HTML paths: the ≤8-panel typed grid and the >8-panel inline grid.

    // --- ≤8-panel typed grid ---
    let wrk = Workdir::new("viz_smart_embeds_plotly_once_without_mathjax");
    wrk.create_from_string("small.csv", "a,b,c\n1,x,9\n2,y,8\n3,x,7\n4,z,6\n5,y,5\n");
    let grid_html = wrk.path("grid.html").to_string_lossy().to_string();
    let mut cmd = wrk.command("viz");
    cmd.args(["smart", "small.csv", "-o", &grid_html]);
    wrk.assert_success(&mut cmd);
    let grid = wrk.read_to_string("grid.html").unwrap();
    // plotly.js embedded exactly once (its version banner) ...
    assert_eq!(grid.matches("plotly.js v").count(), 1);
    // ... and the tex-svg MathJax bundle is gone ("CommonHTML" is unique to that bundle; the
    // residual guarded `typeof MathJax` references inside plotly.js itself are expected).
    assert!(!grid.contains("CommonHTML"));

    // --- >8-panel inline grid ---
    let headers: Vec<String> = (0..10).map(|c| format!("c{c}")).collect();
    let mut rows = headers.join(",");
    rows.push('\n');
    for r in 0..30 {
        let cells: Vec<String> = (0..10).map(|c| format!("v{}", (r + c) % 4)).collect();
        rows.push_str(&cells.join(","));
        rows.push('\n');
    }
    wrk.create_from_string("wide.csv", &rows);
    let inline_html = wrk.path("wide.html").to_string_lossy().to_string();
    let mut cmd = wrk.command("viz");
    cmd.args(["smart", "wide.csv", "-o", &inline_html]);
    wrk.assert_success(&mut cmd);
    let inline = wrk.read_to_string("wide.html").unwrap();
    // many panels, but still ONE embedded plotly.js bundle (panels reuse the shared global)
    assert!(inline.matches("Plotly.newPlot").count() > 8);
    assert_eq!(inline.matches("plotly.js v").count(), 1);
    assert!(!inline.contains("CommonHTML"));
}

#[test]
fn viz_smart_inline_has_theme_toggle() {
    // the >8-panel inline-div case also carries the shared toggle.
    let wrk = Workdir::new("viz_smart_inline_has_theme_toggle");
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
    cmd.args(["smart", "wide.csv", "-o", &out_html]);
    wrk.assert_success(&mut cmd);

    let html = wrk.read_to_string("wide.html").unwrap();
    assert!(html.contains(r#"id="qsv-theme-toggle""#));
    assert!(html.contains("qsv-viz-theme"));
    assert!(html.contains(".js-plotly-plot"));
    assert!(html.contains("Plotly.relayout"));
    assert!(html.contains("--qsv-page-bg"));
    assert!(html.contains("body.qsv-dark"));
    assert!(html.contains(r#"class="qsv-viz-grid""#));
    // >8 panels -> more than the typed-subplot limit, so it's the inline-div renderer
    assert!(html.matches("Plotly.newPlot").count() > 8);
    // inline panels carry no overall title, so the dashboard title IS shown as the page <h1>
    // (unlike the typed-grid path, which suppresses it because the plot bakes the title in).
    assert!(html.contains(r#"<h1 class="qsv-viz-title""#));
    // regression (roborev #3176): no split `\n{script}` escape (stray `\` + `n`) before the toggle.
    assert!(html.contains("<script>\n(function () {"));
    assert!(!html.contains("n<script>\n(function () {"));
    // the qsv/datHere logo links to the qsv site and embeds both theme variants (CSS-swapped).
    assert!(html.contains(r#"id="qsv-logo""#));
    assert!(html.contains(r#"href="https://qsv.dathere.com/""#));
    assert!(html.contains("qsv-logo-light"));
    assert!(html.contains("qsv-logo-dark"));
    assert!(html.contains("data:image/png;base64,"));
}

#[test]
fn viz_smart_map_geocode_extent_metadata() {
    // a tightly-clustered NYC-area lat/lon dataset: every bounding-box corner + the center
    // reverse-geocode to US/New-York-area cities, so the consolidated summary is stable.
    let wrk = Workdir::new("viz_smart_map_geocode_extent_metadata");
    wrk.create_from_string(
        "places.csv",
        "name,lat,lon,score\nA,40.71,-74.01,10\nB,40.75,-73.98,20\nC,40.68,-73.95,30\nD,40.73,-74.\
         00,40\nE,40.70,-73.99,50\n",
    );

    let mut cmd = wrk.command("viz");
    cmd.args(["smart", "places.csv"]);
    let out = wrk.output(&mut cmd);
    // the command must always succeed, even if the Geonames index can't be loaded (offline CI).
    assert!(out.status.success());

    let html = String::from_utf8_lossy(&out.stdout);
    // a lat/lon pair always yields a (full-width) map panel in the inline HTML dashboard.
    assert!(html.contains("Plotly.newPlot"));

    // When qsv is built with the `geocode` feature AND the index is available, the spatial-extent
    // overlay + consolidated summary caption render. Guarded so a build/run without the index
    // (geocode feature off, or offline first-use) still passes the structural check.
    if html.contains("qsv-viz-geo-meta") {
        assert!(html.contains("Spatial extent:"));
        assert!(html.contains("United States") || html.contains("New York"));
    }
}

#[test]
fn viz_smart_map_outlier_markers() {
    // A tight NYC-area cluster plus two far-flung (but in-range) strays. The strays fall outside
    // the lat/lon Tukey fences, so they're drawn as a distinct "geographic outliers" marker
    // trace. This is pure plotly styling (no geocoding), so it must appear in every build.
    let wrk = Workdir::new("viz_smart_map_outlier_markers");
    let mut rows = String::from("name,lat,lon\n");
    for i in 0..30 {
        let lat = 40.70 + (i as f64) * 0.003;
        let lon = -74.02 + (i as f64) * 0.002;
        rows.push_str(&format!("p{i},{lat:.4},{lon:.4}\n"));
    }
    // two clear geographic outliers, still within valid coordinate ranges
    rows.push_str("far_north,41.90,-74.00\n");
    rows.push_str("far_east,40.72,-72.00\n");
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
        html.contains("geographic outliers"),
        "outliers should be drawn as a distinct marker trace; html: {html}"
    );
    // smart map panels use Carto tiles (no Referer policy); OSM blocks local-file requests.
    // Assert the serialized Plotly layout key ("style":"carto-positron"), not just the bare
    // string which also appears in the theme-toggle palette (mapbox: "carto-positron").
    assert!(
        html.contains(r#""style":"carto-positron""#),
        "light-theme smart map panel must set layout.mapbox.style to carto-positron, not \
         open-street-map"
    );
}

#[test]
fn viz_smart_map_outlier_extent_callout() {
    // A tight NYC cluster plus one point in Pennsylvania. With the `geocode` feature and a usable
    // Geonames index, the PA point is a geographic outlier: it's excluded from the (core) spatial
    // extent summary and called out separately. Guarded like viz_smart_map_geocode_extent_metadata
    // so a build/run without the index still passes the structural check.
    let wrk = Workdir::new("viz_smart_map_outlier_extent_callout");
    let mut rows = String::from("name,lat,lon\n");
    for i in 0..20 {
        let lat = 40.70 + (i as f64) * 0.004;
        let lon = -74.02 + (i as f64) * 0.003;
        rows.push_str(&format!("nyc{i},{lat:.4},{lon:.4}\n"));
    }
    // Harrisburg, PA — clearly outside the NYC cluster's lat/lon fences
    rows.push_str("harrisburg,40.27,-76.88\n");
    wrk.create_from_string("places.csv", &rows);

    let mut cmd = wrk.command("viz");
    cmd.args(["smart", "places.csv"]);
    let out = wrk.output(&mut cmd);
    assert!(out.status.success());

    let html = String::from_utf8_lossy(&out.stdout);
    assert!(html.contains("Plotly.newPlot"));
    // the distinct outlier marker trace is non-gated, so it's always present
    assert!(html.contains("geographic outliers"));
    // the jurisdiction call-out + full-extent box only render with the geocode feature + index
    if html.contains("qsv-viz-geo-meta") {
        assert!(html.contains("Spatial extent:"));
        assert!(html.contains("outlier"));
        // the second (no-fill) bounding box covering core + outliers
        assert!(html.contains("full extent (incl. outliers)"));
        // the Core/Full extent zoom buttons
        assert!(html.contains("Core extent") && html.contains("Full extent"));
        // the buttons pin an explicit ink label color (white pill over light tiles) so the
        // dark-mode toggle / a dark --theme can't flip the label to a light, invisible color.
        assert!(html.contains(r##""size":11,"color":"#2A3F5F""##));
    }
}

// ---- treemap / sunburst hierarchy panels ----

/// id (near-unique, skipped) + two low-cardinality categorical dimensions.
/// id (skipped) + two ASSOCIATED categorical dimensions: category nests under region (East and
/// West sell different products), so the dims are statistically dependent — a genuine hierarchy
/// that clears `viz smart`'s independence screen (corrected Cramér's V ~0.69).
fn two_dim_hierarchy(wrk: &Workdir) {
    let mut rows = String::from("id,region,category\n");
    for i in 1..=90 {
        let (region, category) = match i % 6 {
            0 => ("East", "Widgets"),
            1 => ("East", "Gadgets"),
            2 => ("West", "Gizmos"),
            3 => ("West", "Doohickeys"),
            4 => ("North", "Widgets"),
            _ => ("North", "Gizmos"),
        };
        rows.push_str(&format!("{i},{region},{category}\n"));
    }
    wrk.create_from_string("two_dim.csv", &rows);
}

/// id (skipped) + three ASSOCIATED low-cardinality categorical dimensions (region → category →
/// channel all co-vary), so every pair is dependent and the hierarchy clears the independence
/// screen (max corrected Cramér's V ~0.86).
fn three_dim_hierarchy(wrk: &Workdir) {
    let mut rows = String::from("id,region,category,channel\n");
    for i in 1..=120 {
        let (region, category, channel) = match i % 6 {
            0 => ("East", "Widgets", "Web"),
            1 => ("East", "Gadgets", "Retail"),
            2 => ("West", "Gizmos", "Phone"),
            3 => ("West", "Doohickeys", "Partner"),
            4 => ("North", "Widgets", "Web"),
            _ => ("North", "Gizmos", "Retail"),
        };
        rows.push_str(&format!("{i},{region},{category},{channel}\n"));
    }
    wrk.create_from_string("three_dim.csv", &rows);
}

/// id (skipped) + two INDEPENDENT categorical dimensions (region = i%3, payment = i%4; coprime
/// moduli make them statistically independent), so `viz smart` should NOT auto-build a hierarchy —
/// the per-column bars already say everything the nested chart would.
fn independent_dims(wrk: &Workdir) {
    let mut rows = String::from("id,region,payment\n");
    for i in 1..=120 {
        let region = match i % 3 {
            0 => "East",
            1 => "West",
            _ => "North",
        };
        let payment = match i % 4 {
            0 => "Cash",
            1 => "Card",
            2 => "PayPal",
            _ => "Wire",
        };
        rows.push_str(&format!("{i},{region},{payment}\n"));
    }
    wrk.create_from_string("independent.csv", &rows);
}

#[test]
fn viz_smart_hierarchy_treemap_for_two_dims() {
    let wrk = Workdir::new("viz_smart_hierarchy_treemap_for_two_dims");
    two_dim_hierarchy(&wrk);

    let out_html = wrk.path("dash.html").to_string_lossy().to_string();
    let mut cmd = wrk.command("viz");
    cmd.args(["smart", "two_dim.csv", "-o", &out_html]);
    wrk.assert_success(&mut cmd);

    let html = wrk.read_to_string("dash.html").unwrap();
    // a shallow (2-level) hierarchy auto-selects a treemap with rolled-up totals
    assert!(html.contains(r#""type":"treemap""#));
    assert!(html.contains(r#""branchvalues":"total""#));
    // and not a sunburst
    assert!(!html.contains(r#""type":"sunburst""#));
}

#[test]
fn viz_smart_hierarchy_sunburst_for_three_dims() {
    let wrk = Workdir::new("viz_smart_hierarchy_sunburst_for_three_dims");
    three_dim_hierarchy(&wrk);

    let out_html = wrk.path("dash.html").to_string_lossy().to_string();
    let mut cmd = wrk.command("viz");
    cmd.args(["smart", "three_dim.csv", "-o", &out_html]);
    wrk.assert_success(&mut cmd);

    let html = wrk.read_to_string("dash.html").unwrap();
    // a deep (3-level) hierarchy auto-selects a sunburst
    assert!(html.contains(r#""type":"sunburst""#));
}

#[test]
fn viz_smart_hierarchy_style_override() {
    let wrk = Workdir::new("viz_smart_hierarchy_style_override");
    three_dim_hierarchy(&wrk);

    let out_html = wrk.path("dash.html").to_string_lossy().to_string();
    let mut cmd = wrk.command("viz");
    cmd.args([
        "smart",
        "three_dim.csv",
        "--hierarchy-style",
        "treemap",
        "-o",
        &out_html,
    ]);
    wrk.assert_success(&mut cmd);

    let html = wrk.read_to_string("dash.html").unwrap();
    // explicit override beats the depth-based auto rule
    assert!(html.contains(r#""type":"treemap""#));
    assert!(!html.contains(r#""type":"sunburst""#));
}

#[test]
fn viz_smart_skips_hierarchy_for_independent_dims() {
    // Two statistically INDEPENDENT categoricals must NOT auto-build a treemap/sunburst — nesting
    // them just replicates each level's marginal, so the per-column bars say it all.
    let wrk = Workdir::new("viz_smart_skips_hierarchy_for_independent_dims");
    independent_dims(&wrk);

    let out_html = wrk.path("dash.html").to_string_lossy().to_string();
    let mut cmd = wrk.command("viz");
    cmd.args(["smart", "independent.csv", "-o", &out_html]);
    wrk.assert_success(&mut cmd);

    let html = wrk.read_to_string("dash.html").unwrap();
    assert!(
        !html.contains(r#""type":"treemap""#) && !html.contains(r#""type":"sunburst""#),
        "independent dims should NOT auto-build a hierarchy; html: {html}"
    );
    // but the per-column frequency bars are still there
    assert!(html.contains(r#""type":"bar""#));
}

#[test]
fn viz_smart_independent_dims_hierarchy_forced_by_style() {
    // An explicit --hierarchy-style is a deliberate request, so it bypasses the independence screen
    // and builds the chart even though the dims are independent.
    let wrk = Workdir::new("viz_smart_independent_dims_hierarchy_forced_by_style");
    independent_dims(&wrk);

    let out_html = wrk.path("dash.html").to_string_lossy().to_string();
    let mut cmd = wrk.command("viz");
    cmd.args([
        "smart",
        "independent.csv",
        "--hierarchy-style",
        "treemap",
        "-o",
        &out_html,
    ]);
    wrk.assert_success(&mut cmd);

    let html = wrk.read_to_string("dash.html").unwrap();
    assert!(
        html.contains(r#""type":"treemap""#),
        "explicit --hierarchy-style should force the panel despite independence; html: {html}"
    );
}

#[test]
fn viz_treemap_standalone() {
    let wrk = Workdir::new("viz_treemap_standalone");
    two_dim_hierarchy(&wrk);

    let out_html = wrk.path("tm.html").to_string_lossy().to_string();
    let mut cmd = wrk.command("viz");
    cmd.args([
        "treemap",
        "two_dim.csv",
        "--cols",
        "region,category",
        "-o",
        &out_html,
    ]);
    wrk.assert_success(&mut cmd);

    let html = wrk.read_to_string("tm.html").unwrap();
    assert!(html.contains(r#""type":"treemap""#));
    assert!(html.contains(r#""branchvalues":"total""#));
    // Regression guard (PR #4083): the treemap marker pad must keep left/right/bottom inner
    // padding but OMIT `top`, so plotly auto-sizes a header band tall enough to render each
    // parent's label. Pinning `top` to a few px collapses the header and the top hierarchy level
    // shows as bare color. Catch a `top(..)` being reintroduced into the pad.
    assert!(html.contains(r#""pad":{"l":3.0,"r":3.0,"b":3.0}"#));
    assert!(!html.contains(r#""pad":{"t":"#));
}

#[test]
fn viz_sunburst_standalone() {
    let wrk = Workdir::new("viz_sunburst_standalone");
    three_dim_hierarchy(&wrk);

    let out_html = wrk.path("sb.html").to_string_lossy().to_string();
    let mut cmd = wrk.command("viz");
    cmd.args([
        "sunburst",
        "three_dim.csv",
        "--cols",
        "region,category,channel",
        "-o",
        &out_html,
    ]);
    wrk.assert_success(&mut cmd);

    let html = wrk.read_to_string("sb.html").unwrap();
    assert!(html.contains(r#""type":"sunburst""#));
    // A deep sunburst still caps the initial view to two rings (`maxdepth`) so the outer ring isn't
    // drawn until drill-down, but renders the richer `label+value+percent parent` textinfo
    // (restored to match the smart-viz sunburst panels) so each sector exposes its value and
    // share of parent.
    assert!(html.contains(r#""maxdepth":3"#));
    assert!(html.contains(r#""textinfo":"label+value+percent parent""#));
    // plotly.js 3.6 radial in-sector text keeps deep-ring labels legible along each spoke
    assert!(html.contains(r#""insidetextorientation":"radial""#));
}

#[test]
fn viz_treemap_requires_two_cols() {
    let wrk = Workdir::new("viz_treemap_requires_two_cols");
    two_dim_hierarchy(&wrk);

    let out_html = wrk.path("tm.html").to_string_lossy().to_string();
    let mut cmd = wrk.command("viz");
    cmd.args([
        "treemap",
        "two_dim.csv",
        "--cols",
        "region",
        "-o",
        &out_html,
    ]);
    wrk.assert_err(&mut cmd);
}

/// region/category dims + a numeric `amount` measure and an all-text `label` column, for
/// exercising `--value` validation on the hierarchy subcommands.
fn value_hierarchy(wrk: &Workdir) {
    let mut rows = String::from("region,category,amount,label\n");
    for i in 1..=30 {
        let region = if i % 2 == 0 { "East" } else { "West" };
        let category = match i % 3 {
            0 => "A",
            1 => "B",
            _ => "C",
        };
        rows.push_str(&format!("{region},{category},{},lbl{i}\n", i * 10));
    }
    wrk.create_from_string("v.csv", &rows);
}

#[test]
fn viz_treemap_value_sum() {
    let wrk = Workdir::new("viz_treemap_value_sum");
    value_hierarchy(&wrk);

    let out = wrk.path("t.html").to_string_lossy().to_string();
    let mut cmd = wrk.command("viz");
    cmd.args([
        "treemap",
        "v.csv",
        "--cols",
        "region,category",
        "--value",
        "amount",
        "--agg",
        "sum",
        "-o",
        &out,
    ]);
    wrk.assert_success(&mut cmd);

    let html = wrk.read_to_string("t.html").unwrap();
    assert!(html.contains(r#""type":"treemap""#));
}

#[test]
fn viz_treemap_value_all_invalid_errors() {
    let wrk = Workdir::new("viz_treemap_value_all_invalid_errors");
    value_hierarchy(&wrk);

    let out = wrk.path("t.html").to_string_lossy().to_string();
    let mut cmd = wrk.command("viz");
    // `label` is entirely non-numeric, so there's no usable measure to size the chart -> error
    // (rather than silently coercing every cell to 0 and emitting a blank treemap).
    cmd.args([
        "treemap",
        "v.csv",
        "--cols",
        "region,category",
        "--value",
        "label",
        "-o",
        &out,
    ]);
    wrk.assert_err(&mut cmd);
}

#[test]
fn viz_treemap_value_mixed_invalid_errors() {
    let wrk = Workdir::new("viz_treemap_value_mixed_invalid_errors");
    // `amount` is numeric except for two non-numeric cells. A part-to-whole chart would silently
    // drop those rows and misstate every proportion, so a partially-invalid measure must error
    // (not just warn) rather than produce a deceptively "successful" chart.
    let mut rows = String::from("region,category,amount\n");
    for i in 1..=20 {
        let region = if i % 2 == 0 { "East" } else { "West" };
        let category = if i % 3 == 0 { "A" } else { "B" };
        let amount = if i == 5 || i == 12 {
            "n/a".to_string()
        } else {
            (i * 10).to_string()
        };
        rows.push_str(&format!("{region},{category},{amount}\n"));
    }
    wrk.create_from_string("m.csv", &rows);

    let out = wrk.path("t.html").to_string_lossy().to_string();
    let mut cmd = wrk.command("viz");
    cmd.args([
        "treemap",
        "m.csv",
        "--cols",
        "region,category",
        "--value",
        "amount",
        "-o",
        &out,
    ]);
    wrk.assert_err(&mut cmd);
}

// ---- choropleth ----

fn countries(wrk: &Workdir) {
    wrk.create_from_string(
        "countries.csv",
        "country,value\nUSA,10\nCAN,5\nMEX,7\nUSA,3\n",
    );
}

#[test]
fn viz_choropleth_basic() {
    let wrk = Workdir::new("viz_choropleth_basic");
    countries(&wrk);

    let mut cmd = wrk.command("viz");
    cmd.args([
        "choropleth",
        "countries.csv",
        "--locations",
        "country",
        "--value",
        "value",
        "--agg",
        "sum",
    ]);
    let out = wrk.output(&mut cmd);
    assert!(out.status.success());

    let html = String::from_utf8_lossy(&out.stdout);
    assert!(html.contains("Plotly.newPlot"));
    assert!(html.contains(r#""type":"choropleth""#));
    assert!(html.contains(r#""locationmode":"ISO-3""#));
    // USA's two rows are summed (10 + 3 = 13); regions are deduplicated in first-seen order
    assert!(html.contains(r#""locations":["USA","CAN","MEX"]"#));
    assert!(html.contains(r#""z":[13.0,5.0,7.0]"#));
    // the colorbar is titled by the measure column
    assert!(html.contains(r#""colorbar":{"title":{"text":"value"#));
}

#[test]
fn viz_choropleth_count_default() {
    let wrk = Workdir::new("viz_choropleth_count_default");
    countries(&wrk);

    let mut cmd = wrk.command("viz");
    cmd.args(["choropleth", "countries.csv", "--locations", "country"]);
    let out = wrk.output(&mut cmd);
    assert!(out.status.success());

    let html = String::from_utf8_lossy(&out.stdout);
    assert!(html.contains(r#""type":"choropleth""#));
    // no --value: z is the per-region row count (USA appears twice)
    assert!(html.contains(r#""z":[2.0,1.0,1.0]"#));
    assert!(html.contains(r#""colorbar":{"title":{"text":"count"#));
}

#[test]
fn viz_choropleth_color_scale() {
    let wrk = Workdir::new("viz_choropleth_color_scale");
    countries(&wrk);

    let mut cmd = wrk.command("viz");
    cmd.args([
        "choropleth",
        "countries.csv",
        "--locations",
        "country",
        "--color-scale",
        "cividis",
    ]);
    let out = wrk.output(&mut cmd);
    assert!(out.status.success());

    let html = String::from_utf8_lossy(&out.stdout);
    assert!(html.contains(r#""colorscale":"Cividis""#));
}

#[test]
fn viz_choropleth_usa_states() {
    let wrk = Workdir::new("viz_choropleth_usa_states");
    wrk.create_from_string("states.csv", "st,n\nNY,5\nCA,9\nTX,4\n");

    let mut cmd = wrk.command("viz");
    cmd.args([
        "choropleth",
        "states.csv",
        "--locations",
        "st",
        "--value",
        "n",
        "--location-mode",
        "usa-states",
    ]);
    let out = wrk.output(&mut cmd);
    assert!(out.status.success());

    let html = String::from_utf8_lossy(&out.stdout);
    assert!(html.contains(r#""type":"choropleth""#));
    assert!(html.contains(r#""locationmode":"USA-states""#));
    // usa-states frames itself with the albers-usa projection (CONUS + AK/HI insets), not the
    // default whole-world view where the states would be tiny
    assert!(html.contains(r#""projection":{"type":"albers usa""#));
    // scope:"usa" restricts the basemap to the US extent so neighbouring land (e.g. British
    // Columbia) does not bleed above the northern US border in the albers-usa composite canvas
    assert!(html.contains(r#""scope":"usa""#));
}

#[test]
fn viz_choropleth_map() {
    let wrk = Workdir::new("viz_choropleth_map");
    wrk.create_from_string("rg.csv", "region,val\nA,10\nB,20\n");
    wrk.create_from_string(
        "regions.geojson",
        r#"{"type":"FeatureCollection","features":[{"type":"Feature","id":"A","properties":{},"geometry":{"type":"Polygon","coordinates":[[[0,0],[0,1],[1,1],[1,0],[0,0]]]}},{"type":"Feature","id":"B","properties":{},"geometry":{"type":"Polygon","coordinates":[[[1,0],[1,1],[2,1],[2,0],[1,0]]]}}]}"#,
    );

    let mut cmd = wrk.command("viz");
    cmd.args([
        "choropleth",
        "rg.csv",
        "--locations",
        "region",
        "--value",
        "val",
        "--map",
        "--geojson",
        "regions.geojson",
        "--feature-id-key",
        "id",
    ]);
    let out = wrk.output(&mut cmd);
    assert!(out.status.success());

    let html = String::from_utf8_lossy(&out.stdout);
    // MapLibre ChoroplethMap on a `map` subplot, matched by the geojson feature id
    assert!(html.contains(r#""type":"choroplethmap""#));
    assert!(html.contains(r#""featureidkey":"id""#));
    assert!(html.contains(r#""geojson":{"type":"FeatureCollection""#));
    assert!(html.contains(r#""map":{"#));
    // the basemap is framed to the geojson extent (center + zoom), not left at plotly's default
    // whole-world view where local regions would be invisible
    assert!(html.contains(r#""center":{"#));
    assert!(html.contains(r#""zoom":"#));
}

// the geojson-extent framing must read coordinates ONLY from geometry, never from numeric arrays in
// feature `properties` — otherwise a stray property array would drag the map center off the data.
#[test]
fn viz_choropleth_map_frames_ignore_properties() {
    let wrk = Workdir::new("viz_choropleth_map_frames_ignore_properties");
    wrk.create_from_string("rg.csv", "region,val\nCA,40\nNY,30\n");
    // two boxes firmly in the US (lon ~ -120 / -75); a decoy property array near lon/lat 0
    wrk.create_from_string(
        "regions.geojson",
        r#"{"type":"FeatureCollection","features":[{"type":"Feature","id":"CA","properties":{"decoy":[0.0,0.0]},"geometry":{"type":"Polygon","coordinates":[[[-124,32],[-124,42],[-114,42],[-114,32],[-124,32]]]}},{"type":"Feature","id":"NY","properties":{},"geometry":{"type":"Polygon","coordinates":[[[-79,40],[-79,45],[-72,45],[-72,40],[-79,40]]]}}]}"#,
    );

    let mut cmd = wrk.command("viz");
    cmd.args([
        "choropleth",
        "rg.csv",
        "--locations",
        "region",
        "--value",
        "val",
        "--map",
        "--geojson",
        "regions.geojson",
        "--feature-id-key",
        "id",
    ]);
    let out = wrk.output(&mut cmd);
    assert!(out.status.success());

    let html = String::from_utf8_lossy(&out.stdout);
    // center longitude must be a US value (west of -70), proving the (0,0) decoy in `properties`
    // was not folded into the bounds.
    let i = html.find(r#""center":{"#).expect("center present");
    let lon_at = html[i..].find(r#""lon":"#).expect("lon present") + i + 6;
    let lon_str: String = html[lon_at..]
        .chars()
        .take_while(|c| c.is_ascii_digit() || *c == '-' || *c == '.')
        .collect();
    let lon: f64 = lon_str.parse().expect("parse center lon");
    assert!(
        lon < -70.0,
        "center lon {lon} should be in the US (decoy property coord leaked into framing?)"
    );
}

// point-in-polygon binning: lat/lon points + a custom --geojson (no --geocode) bins each point into
// the region whose polygon contains it; the location IS the feature id (exact, no name/code match).
// Exercises the default 10 km snap cap: a near-boundary stray snaps, a far stray drops.
#[test]
fn viz_choropleth_pip_bins_points() {
    let wrk = Workdir::new("viz_choropleth_pip_bins_points");
    // A = lon 0..10, B = lon 10..20 (both lat 0..10). Points: one in A, two in B, a near stray
    // ~5.6 km north of A's edge (snaps to A under the default 10 km cap), and one far outside
    // (drops)
    wrk.create_from_string("pts.csv", "lat,lon\n5,5\n5,15\n5,15\n10.05,5\n50,50\n");
    wrk.create_from_string(
        "regions.geojson",
        r#"{"type":"FeatureCollection","features":[{"type":"Feature","properties":{"id":"A"},"geometry":{"type":"Polygon","coordinates":[[[0,0],[0,10],[10,10],[10,0],[0,0]]]}},{"type":"Feature","properties":{"id":"B"},"geometry":{"type":"Polygon","coordinates":[[[10,0],[10,10],[20,10],[20,0],[10,0]]]}}]}"#,
    );

    let mut cmd = wrk.command("viz");
    cmd.args([
        "choropleth",
        "pts.csv",
        "--lat",
        "lat",
        "--lon",
        "lon",
        "--geojson",
        "regions.geojson",
        "--feature-id-key",
        "properties.id",
    ]);
    let out = wrk.output(&mut cmd);
    assert!(out.status.success());
    let html = String::from_utf8_lossy(&out.stdout);
    // a geo Choropleth in geojson-id mode, matched on properties.id, with the geojson embedded
    assert!(html.contains(r#""type":"choropleth""#));
    assert!(html.contains(r#""locationmode":"geojson-id""#));
    assert!(html.contains(r#""featureidkey":"properties.id""#));
    assert!(html.contains(r#""geojson":{"type":"FeatureCollection""#));
    // A = 1 contained + 1 snapped; B = 2 contained; the far (50,50) stray drops under the 10 km cap
    assert!(html.contains(r#""locations":["A","B"]"#));
    assert!(html.contains(r#""z":[2.0,2.0]"#));
    // A absorbed the one snapped stray — its hover flags it as a subset of A's count
    assert!(
        html.contains("includes 1 snapped from outside"),
        "missing snapped-into-region hover note; html was: {html}"
    );
    // the far stray was dropped by the default cap — noted beneath the map (no stderr in a saved
    // file)
    assert!(
        html.contains("1 of 5 points were farther than 10 km from any region and were dropped."),
        "missing dropped-beyond-cap note; html was: {html}"
    );
    // stderr reports both the snap and the cap-drop so the user knows where points went
    let stderr = String::from_utf8_lossy(&out.stderr);
    assert!(
        stderr.contains("1 of 5 points were snapped to the nearest region."),
        "missing snap coverage note; stderr was: {stderr}"
    );
    assert!(
        stderr.contains(
            "1 of 5 points fell outside every region and were dropped (no region within 10 km)."
        ),
        "missing cap-drop coverage note; stderr was: {stderr}"
    );
}

// --no-snap drops points outside every region (instead of snapping to nearest) and reports coverage
// on stderr.
#[test]
fn viz_choropleth_pip_no_snap_drops_and_reports() {
    let wrk = Workdir::new("viz_choropleth_pip_no_snap_drops_and_reports");
    wrk.create_from_string("pts.csv", "lat,lon\n5,5\n5,15\n5,15\n50,50\n");
    wrk.create_from_string(
        "regions.geojson",
        r#"{"type":"FeatureCollection","features":[{"type":"Feature","properties":{"id":"A"},"geometry":{"type":"Polygon","coordinates":[[[0,0],[0,10],[10,10],[10,0],[0,0]]]}},{"type":"Feature","properties":{"id":"B"},"geometry":{"type":"Polygon","coordinates":[[[10,0],[10,10],[20,10],[20,0],[10,0]]]}}]}"#,
    );

    let mut cmd = wrk.command("viz");
    cmd.args([
        "choropleth",
        "pts.csv",
        "--lat",
        "lat",
        "--lon",
        "lon",
        "--geojson",
        "regions.geojson",
        "--feature-id-key",
        "properties.id",
        "--no-snap",
    ]);
    let out = wrk.output(&mut cmd);
    assert!(out.status.success());
    let html = String::from_utf8_lossy(&out.stdout);
    // the (50,50) point is dropped: B keeps only its two contained points
    assert!(html.contains(r#""z":[1.0,2.0]"#));
    // the saved HTML carries the coverage note beneath the map as a paper-anchored annotation
    // (a saved file has no stderr to fall back on); nothing was snapped, so no hover snap line
    assert!(
        html.contains(
            "--no-snap: 1 of 4 points fell outside every GeoJSON region and were dropped."
        ),
        "missing below-map coverage note; html was: {html}"
    );
    assert!(!html.contains("snapped from outside"));
    let stderr = String::from_utf8_lossy(&out.stderr);
    assert!(
        stderr.contains("1 of 4 points fell outside every region and were dropped (--no-snap)."),
        "missing coverage note; stderr was: {stderr}"
    );
}

// --no-snap is only meaningful on the point-in-polygon path; reject it otherwise.
#[test]
fn viz_choropleth_no_snap_requires_pip() {
    let wrk = Workdir::new("viz_choropleth_no_snap_requires_pip");
    wrk.create_from_string("rg.csv", "iso3,val\nUSA,10\nCAN,5\n");
    let mut cmd = wrk.command("viz");
    cmd.args(["choropleth", "rg.csv", "--locations", "iso3", "--no-snap"]);
    wrk.assert_err(&mut cmd);
}

// an explicit --snap-max-dist tightens the cap (km): a ~5.6 km stray that snaps under the default
// 10 km cap is dropped under a 4 km cap, and the drop is noted beneath the map.
#[test]
fn viz_choropleth_snap_max_dist() {
    let wrk = Workdir::new("viz_choropleth_snap_max_dist");
    // one point in A, one ~5.6 km north of A's top edge (lat 10)
    wrk.create_from_string("pts.csv", "lat,lon\n5,5\n10.05,5\n");
    wrk.create_from_string(
        "regions.geojson",
        r#"{"type":"FeatureCollection","features":[{"type":"Feature","properties":{"id":"A"},"geometry":{"type":"Polygon","coordinates":[[[0,0],[0,10],[10,10],[10,0],[0,0]]]}}]}"#,
    );
    let mut cmd = wrk.command("viz");
    cmd.args([
        "choropleth",
        "pts.csv",
        "--lat",
        "lat",
        "--lon",
        "lon",
        "--geojson",
        "regions.geojson",
        "--feature-id-key",
        "properties.id",
        "--snap-max-dist",
        "4",
    ]);
    let out = wrk.output(&mut cmd);
    assert!(out.status.success());
    let html = String::from_utf8_lossy(&out.stdout);
    // only the contained point counts; the stray is beyond the 4 km cap -> dropped
    assert!(html.contains(r#""z":[1.0]"#));
    assert!(
        html.contains("1 of 2 points were farther than 4 km from any region and were dropped."),
        "missing cap-drop note; html was: {html}"
    );
    assert!(!html.contains("snapped from outside"));
}

// --snap-max-dist only applies to point-in-polygon binning; reject it on a --locations run.
#[test]
fn viz_choropleth_snap_max_dist_requires_pip() {
    let wrk = Workdir::new("viz_choropleth_snap_max_dist_requires_pip");
    wrk.create_from_string("rg.csv", "iso3,val\nUSA,10\nCAN,5\n");
    let mut cmd = wrk.command("viz");
    cmd.args([
        "choropleth",
        "rg.csv",
        "--locations",
        "iso3",
        "--snap-max-dist",
        "5",
    ]);
    wrk.assert_err(&mut cmd);
}

// --snap-max-dist and --no-snap are contradictory (cap how far to snap vs. don't snap at all).
#[test]
fn viz_choropleth_snap_max_dist_conflicts_no_snap() {
    let wrk = Workdir::new("viz_choropleth_snap_max_dist_conflicts_no_snap");
    wrk.create_from_string("pts.csv", "lat,lon\n5,5\n");
    wrk.create_from_string(
        "regions.geojson",
        r#"{"type":"FeatureCollection","features":[{"type":"Feature","properties":{"id":"A"},"geometry":{"type":"Polygon","coordinates":[[[0,0],[0,10],[10,10],[10,0],[0,0]]]}}]}"#,
    );
    let mut cmd = wrk.command("viz");
    cmd.args([
        "choropleth",
        "pts.csv",
        "--lat",
        "lat",
        "--lon",
        "lon",
        "--geojson",
        "regions.geojson",
        "--feature-id-key",
        "properties.id",
        "--no-snap",
        "--snap-max-dist",
        "5",
    ]);
    wrk.assert_err(&mut cmd);
}

// `viz smart` honors the same --snap-max-dist validation as the command (the constraints are
// enforced up front in run(), before dispatch): a negative value is rejected, not silently clamped.
#[test]
fn viz_smart_snap_max_dist_negative_errors() {
    let wrk = Workdir::new("viz_smart_snap_max_dist_negative_errors");
    wrk.create_from_string("pts.csv", "lat,lon\n5,5\n5,15\n");
    wrk.create_from_string(
        "regions.geojson",
        r#"{"type":"FeatureCollection","features":[{"type":"Feature","properties":{"id":"A"},"geometry":{"type":"Polygon","coordinates":[[[0,0],[0,10],[10,10],[10,0],[0,0]]]}}]}"#,
    );
    let mut cmd = wrk.command("viz");
    cmd.args([
        "smart",
        "pts.csv",
        "--geojson",
        "regions.geojson",
        "--feature-id-key",
        "properties.id",
        "--snap-max-dist",
        "-1",
    ]);
    wrk.assert_err(&mut cmd);
}

// `viz smart` rejects --snap-max-dist combined with --no-snap, same as the command.
#[test]
fn viz_smart_snap_max_dist_conflicts_no_snap() {
    let wrk = Workdir::new("viz_smart_snap_max_dist_conflicts_no_snap");
    wrk.create_from_string("pts.csv", "lat,lon\n5,5\n5,15\n");
    wrk.create_from_string(
        "regions.geojson",
        r#"{"type":"FeatureCollection","features":[{"type":"Feature","properties":{"id":"A"},"geometry":{"type":"Polygon","coordinates":[[[0,0],[0,10],[10,10],[10,0],[0,0]]]}}]}"#,
    );
    let mut cmd = wrk.command("viz");
    cmd.args([
        "smart",
        "pts.csv",
        "--geojson",
        "regions.geojson",
        "--feature-id-key",
        "properties.id",
        "--no-snap",
        "--snap-max-dist",
        "5",
    ]);
    wrk.assert_err(&mut cmd);
}

// --lat/--lon + --geojson (point-in-polygon) and --locations (pre-keyed regions) are mutually
// exclusive without --geocode; supplying both must error rather than silently ignore --locations.
#[test]
fn viz_choropleth_pip_and_locations_is_ambiguous() {
    let wrk = Workdir::new("viz_choropleth_pip_and_locations_is_ambiguous");
    wrk.create_from_string("pts.csv", "lat,lon,region\n5,5,A\n5,15,B\n");
    wrk.create_from_string(
        "regions.geojson",
        r#"{"type":"FeatureCollection","features":[{"type":"Feature","properties":{"id":"A"},"geometry":{"type":"Polygon","coordinates":[[[0,0],[0,10],[10,10],[10,0],[0,0]]]}},{"type":"Feature","properties":{"id":"B"},"geometry":{"type":"Polygon","coordinates":[[[10,0],[10,10],[20,10],[20,0],[10,0]]]}}]}"#,
    );
    let mut cmd = wrk.command("viz");
    cmd.args([
        "choropleth",
        "pts.csv",
        "--lat",
        "lat",
        "--lon",
        "lon",
        "--locations",
        "region",
        "--geojson",
        "regions.geojson",
        "--feature-id-key",
        "properties.id",
    ]);
    wrk.assert_err(&mut cmd);
}

// `viz smart --geojson` with an explicit-but-broken GeoJSON (here a --feature-id-key that matches
// no feature) must error, not silently produce a dashboard without the Regions panel.
#[test]
fn viz_smart_pip_bad_feature_id_key_errors() {
    let wrk = Workdir::new("viz_smart_pip_bad_feature_id_key_errors");
    wrk.create_from_string("pts.csv", "lat,lon\n5,5\n5,15\n6,16\n");
    wrk.create_from_string(
        "regions.geojson",
        r#"{"type":"FeatureCollection","features":[{"type":"Feature","properties":{"id":"A"},"geometry":{"type":"Polygon","coordinates":[[[0,0],[0,10],[10,10],[10,0],[0,0]]]}},{"type":"Feature","properties":{"id":"B"},"geometry":{"type":"Polygon","coordinates":[[[10,0],[10,10],[20,10],[20,0],[10,0]]]}}]}"#,
    );
    let mut cmd = wrk.command("viz");
    cmd.args([
        "smart",
        "pts.csv",
        "--geojson",
        "regions.geojson",
        "--feature-id-key",
        "properties.nonexistent",
    ]);
    wrk.assert_err(&mut cmd);
}

// `viz smart` builds a point-in-polygon prefecture/region choropleth panel when given a --geojson,
// with no geocode engine involved.
#[test]
fn viz_smart_pip_choropleth_panel() {
    let wrk = Workdir::new("viz_smart_pip_choropleth_panel");
    wrk.create_from_string("pts.csv", "lat,lon,mag\n5,5,1\n6,6,2\n5,15,3\n6,16,4\n");
    wrk.create_from_string(
        "regions.geojson",
        r#"{"type":"FeatureCollection","features":[{"type":"Feature","properties":{"id":"A"},"geometry":{"type":"Polygon","coordinates":[[[0,0],[0,10],[10,10],[10,0],[0,0]]]}},{"type":"Feature","properties":{"id":"B"},"geometry":{"type":"Polygon","coordinates":[[[10,0],[10,10],[20,10],[20,0],[10,0]]]}}]}"#,
    );

    let mut cmd = wrk.command("viz");
    cmd.args([
        "smart",
        "pts.csv",
        "--geojson",
        "regions.geojson",
        "--feature-id-key",
        "properties.id",
    ]);
    let out = wrk.output(&mut cmd);
    assert!(out.status.success());
    let html = String::from_utf8_lossy(&out.stdout);
    assert!(html.contains(r#""type":"choropleth""#));
    assert!(html.contains(r#""locationmode":"geojson-id""#));
    assert!(html.contains(r#""featureidkey":"properties.id""#));
}

// PIP choropleth hover shows the human-readable region name (auto-detected from properties.name),
// the labeled count, the share of total, and the rank.
#[test]
fn viz_choropleth_pip_hover_names() {
    let wrk = Workdir::new("viz_choropleth_pip_hover_names");
    // 1 point in A, 3 in B
    wrk.create_from_string("pts.csv", "lat,lon\n5,5\n5,15\n5,15\n6,16\n");
    wrk.create_from_string(
        "regions.geojson",
        r#"{"type":"FeatureCollection","features":[{"type":"Feature","properties":{"id":"A","name":"Alpha"},"geometry":{"type":"Polygon","coordinates":[[[0,0],[0,10],[10,10],[10,0],[0,0]]]}},{"type":"Feature","properties":{"id":"B","name":"Bravo"},"geometry":{"type":"Polygon","coordinates":[[[10,0],[10,10],[20,10],[20,0],[10,0]]]}}]}"#,
    );

    let mut cmd = wrk.command("viz");
    cmd.args([
        "choropleth",
        "pts.csv",
        "--lat",
        "lat",
        "--lon",
        "lon",
        "--geojson",
        "regions.geojson",
        "--feature-id-key",
        "properties.id",
    ]);
    let out = wrk.output(&mut cmd);
    assert!(out.status.success());
    let html = String::from_utf8_lossy(&out.stdout);
    assert!(html.contains(r#""hovertext":["#), "hovertext array missing");
    assert!(
        html.contains(r#""hoverinfo":"text""#),
        "hoverinfo:text missing"
    );
    // names auto-detected from properties.name; labeled count, share, and rank present
    assert!(html.contains("Alpha"), "region name Alpha missing");
    assert!(html.contains("Bravo"), "region name Bravo missing");
    assert!(html.contains("count: 1"), "labeled count missing");
    assert!(html.contains("% of total"), "share-of-total missing");
    assert!(html.contains("rank 1 of 2"), "rank missing");
}

// literal choropleth with a non-count aggregation (mean): hover is labeled and ranked, but the
// share-of-total line is suppressed (a share is meaningless for a mean).
#[test]
fn viz_choropleth_literal_hover_labeled() {
    let wrk = Workdir::new("viz_choropleth_literal_hover_labeled");
    wrk.create_from_string("rg.csv", "region,mag\nUSA,2\nUSA,4\nCAN,5\n");
    let mut cmd = wrk.command("viz");
    cmd.args([
        "choropleth",
        "rg.csv",
        "--locations",
        "region",
        "--value",
        "mag",
        "--agg",
        "mean",
    ]);
    let out = wrk.output(&mut cmd);
    assert!(out.status.success());
    let html = String::from_utf8_lossy(&out.stdout);
    assert!(html.contains(r#""hovertext":["#), "hovertext array missing");
    assert!(html.contains("mag: 3"), "labeled mean value missing");
    assert!(html.contains("rank "), "rank missing");
    assert!(
        !html.contains("% of total"),
        "share-of-total must be suppressed for mean agg"
    );
}

// a literal --locations choropleth backed by a custom --geojson resolves region names from the
// GeoJSON (auto-detected properties.name) into the hover, same as the point-in-polygon path.
#[test]
fn viz_choropleth_literal_geojson_hover_names() {
    let wrk = Workdir::new("viz_choropleth_literal_geojson_hover_names");
    wrk.create_from_string("rg.csv", "state,val\nA,10\nB,30\n");
    wrk.create_from_string(
        "regions.geojson",
        r#"{"type":"FeatureCollection","features":[{"type":"Feature","properties":{"id":"A","name":"Alpha"},"geometry":{"type":"Polygon","coordinates":[[[0,0],[0,10],[10,10],[10,0],[0,0]]]}},{"type":"Feature","properties":{"id":"B","name":"Bravo"},"geometry":{"type":"Polygon","coordinates":[[[10,0],[10,10],[20,10],[20,0],[10,0]]]}}]}"#,
    );
    let mut cmd = wrk.command("viz");
    cmd.args([
        "choropleth",
        "rg.csv",
        "--locations",
        "state",
        "--value",
        "val",
        "--location-mode",
        "geojson-id",
        "--geojson",
        "regions.geojson",
        "--feature-id-key",
        "properties.id",
    ]);
    let out = wrk.output(&mut cmd);
    assert!(out.status.success());
    let html = String::from_utf8_lossy(&out.stdout);
    assert!(html.contains(r#""hovertext":["#), "hovertext array missing");
    // names auto-detected from the GeoJSON properties.name, shown as "<name> (<id>)"
    assert!(html.contains("Alpha"), "region name Alpha missing");
    assert!(html.contains("Bravo"), "region name Bravo missing");
    assert!(html.contains("val: 10"), "labeled value missing");
}

// the --map (MapLibre ChoroplethMap) path also carries the enriched hover.
#[test]
fn viz_choropleth_map_hover() {
    let wrk = Workdir::new("viz_choropleth_map_hover");
    wrk.create_from_string("pts.csv", "lat,lon\n5,5\n5,15\n");
    wrk.create_from_string(
        "regions.geojson",
        r#"{"type":"FeatureCollection","features":[{"type":"Feature","properties":{"id":"A","name":"Alpha"},"geometry":{"type":"Polygon","coordinates":[[[0,0],[0,10],[10,10],[10,0],[0,0]]]}},{"type":"Feature","properties":{"id":"B","name":"Bravo"},"geometry":{"type":"Polygon","coordinates":[[[10,0],[10,10],[20,10],[20,0],[10,0]]]}}]}"#,
    );
    let mut cmd = wrk.command("viz");
    cmd.args([
        "choropleth",
        "pts.csv",
        "--lat",
        "lat",
        "--lon",
        "lon",
        "--geojson",
        "regions.geojson",
        "--feature-id-key",
        "properties.id",
        "--map",
    ]);
    let out = wrk.output(&mut cmd);
    assert!(out.status.success());
    let html = String::from_utf8_lossy(&out.stdout);
    assert!(
        html.contains(r#""type":"choroplethmap""#),
        "not a choroplethmap"
    );
    assert!(html.contains(r#""hovertext":["#), "hovertext array missing");
    assert!(
        html.contains("Alpha") || html.contains("Bravo"),
        "region name missing"
    );
    assert!(html.contains("rank "), "rank missing");
}

// `viz smart` PIP choropleth panel carries the enriched hover (names + count + share + rank).
#[test]
fn viz_smart_pip_choropleth_hover_names() {
    let wrk = Workdir::new("viz_smart_pip_choropleth_hover_names");
    wrk.create_from_string("pts.csv", "lat,lon,mag\n5,5,1\n6,6,2\n5,15,3\n6,16,4\n");
    wrk.create_from_string(
        "regions.geojson",
        r#"{"type":"FeatureCollection","features":[{"type":"Feature","properties":{"id":"A","name":"Alpha"},"geometry":{"type":"Polygon","coordinates":[[[0,0],[0,10],[10,10],[10,0],[0,0]]]}},{"type":"Feature","properties":{"id":"B","name":"Bravo"},"geometry":{"type":"Polygon","coordinates":[[[10,0],[10,10],[20,10],[20,0],[10,0]]]}}]}"#,
    );
    let mut cmd = wrk.command("viz");
    cmd.args([
        "smart",
        "pts.csv",
        "--geojson",
        "regions.geojson",
        "--feature-id-key",
        "properties.id",
    ]);
    let out = wrk.output(&mut cmd);
    assert!(out.status.success());
    let html = String::from_utf8_lossy(&out.stdout);
    assert!(html.contains(r#""hovertext":["#), "hovertext array missing");
    assert!(
        html.contains(r#""hoverinfo":"text""#),
        "hoverinfo:text missing"
    );
    assert!(
        html.contains("Alpha") && html.contains("Bravo"),
        "region names missing"
    );
    assert!(html.contains("% of total"), "share-of-total missing");
    assert!(html.contains("rank "), "rank missing");
}

// the projection (non-`--map`) path must frame the `geo` subplot to a custom GeoJSON extent —
// plotly only auto-scopes its built-in location modes, so without framing the polygons would sit
// tiny on the default whole-world view.
#[test]
fn viz_choropleth_geojson_id_geo_framed() {
    let wrk = Workdir::new("viz_choropleth_geojson_id_geo_framed");
    wrk.create_from_string("rg.csv", "region,val\nFR,10\nDE,25\n");
    // two boxes over France/Germany (a local, non-US extent → mercator fit with lon/lat ranges)
    wrk.create_from_string(
        "regions.geojson",
        r#"{"type":"FeatureCollection","features":[{"type":"Feature","id":"FR","properties":{},"geometry":{"type":"Polygon","coordinates":[[[2,45],[2,49],[6,49],[6,45],[2,45]]]}},{"type":"Feature","id":"DE","properties":{},"geometry":{"type":"Polygon","coordinates":[[[8,48],[8,52],[13,52],[13,48],[8,48]]]}}]}"#,
    );

    let mut cmd = wrk.command("viz");
    cmd.args([
        "choropleth",
        "rg.csv",
        "--locations",
        "region",
        "--value",
        "val",
        "--location-mode",
        "geojson-id",
        "--geojson",
        "regions.geojson",
        "--feature-id-key",
        "id",
    ]);
    let out = wrk.output(&mut cmd);
    assert!(out.status.success());

    let html = String::from_utf8_lossy(&out.stdout);
    assert!(html.contains(r#""type":"choropleth""#));
    // framed to the GeoJSON extent: a fitted projection plus lon/lat axis ranges (a local European
    // extent fits with mercator), not the unframed default whole-world view
    assert!(html.contains(r#""projection":{"type":"mercator""#));
    assert!(html.contains(r#""lonaxis":{"range":["#));
    assert!(html.contains(r#""lataxis":{"range":["#));
}

// custom GeoJSON is framed from its FULL vertex extent (no outlier trimming) — every vertex is
// intentional geometry, so a far edge/island vertex must not be clipped out of the fitted view.
#[test]
fn viz_choropleth_geojson_framing_keeps_edge_vertices() {
    let wrk = Workdir::new("viz_choropleth_geojson_framing_keeps_edge_vertices");
    wrk.create_from_string("rg.csv", "region,val\nR,5\n");
    // one polygon: 39 vertices densely packed near lon 0 plus a lone far vertex at lon 50. With
    // 2.5% outlier trimming the lone far vertex is dropped (lon range stops near 0); full-extent
    // framing keeps it, so the fitted lon range must reach well past 40.
    let mut coords = String::new();
    for i in 0..39 {
        coords.push_str(&format!("[{:.3},0.0],", f64::from(i) * 0.02));
    }
    coords.push_str("[50.0,0.0],[50.0,1.0],[0.0,1.0],[0.0,0.0]");
    let geojson = format!(
        r#"{{"type":"FeatureCollection","features":[{{"type":"Feature","id":"R","properties":{{}},"geometry":{{"type":"Polygon","coordinates":[[{coords}]]}}}}]}}"#
    );
    wrk.create_from_string("regions.geojson", &geojson);

    let mut cmd = wrk.command("viz");
    cmd.args([
        "choropleth",
        "rg.csv",
        "--locations",
        "region",
        "--value",
        "val",
        "--location-mode",
        "geojson-id",
        "--geojson",
        "regions.geojson",
        "--feature-id-key",
        "id",
    ]);
    let out = wrk.output(&mut cmd);
    assert!(out.status.success());

    let html = String::from_utf8_lossy(&out.stdout);
    // the fitted longitude range's max must include the far (lon 50) vertex, not a trimmed ~0
    let marker = r#""lonaxis":{"range":["#;
    let i = html.find(marker).expect("lonaxis range present");
    let tail = &html[i + marker.len()..];
    let max_str: String = tail
        .split(',')
        .nth(1)
        .unwrap()
        .chars()
        .take_while(|c| c.is_ascii_digit() || *c == '.' || *c == '-')
        .collect();
    let lon_max: f64 = max_str.parse().expect("parse lon max");
    assert!(
        lon_max > 40.0,
        "full-extent framing must keep the far vertex (lon_max={lon_max})"
    );
}

#[test]
fn viz_choropleth_map_requires_geojson_errors() {
    let wrk = Workdir::new("viz_choropleth_map_requires_geojson_errors");
    wrk.create_from_string("rg.csv", "region,val\nA,10\nB,20\n");

    let mut cmd = wrk.command("viz");
    cmd.args(["choropleth", "rg.csv", "--locations", "region", "--map"]);
    wrk.assert_err(&mut cmd);
}

#[test]
fn viz_choropleth_geojson_id_requires_geojson_errors() {
    let wrk = Workdir::new("viz_choropleth_geojson_id_requires_geojson_errors");
    countries(&wrk);

    let mut cmd = wrk.command("viz");
    cmd.args([
        "choropleth",
        "countries.csv",
        "--locations",
        "country",
        "--location-mode",
        "geojson-id",
    ]);
    wrk.assert_err(&mut cmd);
}

#[test]
fn viz_choropleth_color_rejected() {
    let wrk = Workdir::new("viz_choropleth_color_rejected");
    countries(&wrk);

    let mut cmd = wrk.command("viz");
    cmd.args([
        "choropleth",
        "countries.csv",
        "--locations",
        "country",
        "--color",
        "value",
    ]);
    wrk.assert_err(&mut cmd);
}

// geocode-dependent: the source-conflict guard fires inside the geocode-gated resolver, before any
// index lookup, so it needs no network/index — but it only exists in a geocode build.
#[cfg(feature = "geocode")]
#[test]
fn viz_choropleth_geocode_source_conflict_errors() {
    let wrk = Workdir::new("viz_choropleth_geocode_source_conflict_errors");
    wrk.create_from_string("pts.csv", "name,lat,lon\nnyc,40.71,-74.01\n");

    let mut cmd = wrk.command("viz");
    // --geocode with BOTH a lat/lon source and a --locations name column is ambiguous
    cmd.args([
        "choropleth",
        "pts.csv",
        "--geocode",
        "--lat",
        "lat",
        "--lon",
        "lon",
        "--locations",
        "name",
    ]);
    wrk.assert_err(&mut cmd);
}

// actual reverse-geocoding needs the Geonames index (downloaded on first use); skipped in CI like
// the webdriver-dependent static-export tests.
#[cfg(feature = "geocode")]
#[test]
#[ignore = "requires the Geonames geocode index (downloaded on first use)"]
fn viz_choropleth_geocode_reverse() {
    let wrk = Workdir::new("viz_choropleth_geocode_reverse");
    wrk.create_from_string(
        "pts.csv",
        "name,lat,lon\nnyc,40.71,-74.01\nla,34.05,-118.24\nlondon,51.51,-0.13\n",
    );

    let mut cmd = wrk.command("viz");
    cmd.args([
        "choropleth",
        "pts.csv",
        "--geocode",
        "--lat",
        "lat",
        "--lon",
        "lon",
    ]);
    let out = wrk.output(&mut cmd);
    assert!(out.status.success());

    let html = String::from_utf8_lossy(&out.stdout);
    assert!(html.contains(r#""type":"choropleth""#));
    // NYC + LA reverse-geocode to the USA (count 2); London to GBR
    assert!(html.contains("USA"));
    assert!(html.contains("GBR"));
}

// `viz smart` frames the per-country choropleth to its own extent, so a region-confined multi-
// country dataset (here Western Europe) zooms to that region (mercator + fitted lon/lat axes)
// instead of sitting tiny on the world projection. Requires the geonames index.
#[cfg(feature = "geocode")]
#[test]
#[ignore = "requires the Geonames geocode index (downloaded on first use)"]
fn viz_smart_choropleth_frames_to_region() {
    let wrk = Workdir::new("viz_smart_choropleth_frames_to_region");
    // real newlines (not "\n" escapes) so rustfmt's string wrapping can't corrupt an escape at a
    // line boundary
    wrk.create_from_string(
        "eu.csv",
        "name,lat,lon
london,51.51,-0.13
paris,48.85,2.35
berlin,52.52,13.40
rome,41.90,12.50
madrid,40.42,-3.70
",
    );

    let mut cmd = wrk.command("viz");
    cmd.args(["smart", "eu.csv"]);
    let out = wrk.output(&mut cmd);
    assert!(out.status.success());

    let html = String::from_utf8_lossy(&out.stdout);
    // a per-country ("Countries") choropleth framed to the filled region GEOMETRIES (not the source
    // points, which would clip the countries) via `fitbounds: "locations"` on a natural-earth geo
    assert!(html.contains(r#""type":"choropleth""#));
    assert!(html.contains(r#""locationmode":"ISO-3""#));
    assert!(html.contains(r#""fitbounds":"locations""#));
    assert!(html.contains(r#""projection":{"type":"natural earth""#));
}

// the smart choropleth scope is chosen from the reverse-geocoded countries, NOT the broad US
// bounding box: a US + Mexico dataset (both inside that box) must render the per-COUNTRY panel,
// not a US-states panel that silently drops the Mexican points. Requires the geonames index.
#[cfg(feature = "geocode")]
#[test]
#[ignore = "requires the Geonames geocode index (downloaded on first use)"]
fn viz_smart_choropleth_us_bbox_multicountry_is_per_country() {
    let wrk = Workdir::new("viz_smart_choropleth_us_bbox_multicountry_is_per_country");
    // real newlines so rustfmt's string wrapping can't corrupt an escape at a line boundary
    wrk.create_from_string(
        "pts.csv",
        "n,lat,lon
nyc,40.71,-74.01
la,34.05,-118.24
chicago,41.88,-87.63
mexicocity,19.43,-99.13
guadalajara,20.67,-103.35
monterrey,25.69,-100.32
",
    );

    let mut cmd = wrk.command("viz");
    cmd.args(["smart", "pts.csv"]);
    let out = wrk.output(&mut cmd);
    assert!(out.status.success());

    let html = String::from_utf8_lossy(&out.stdout);
    // per-country (ISO-3) choropleth covering the USA and Mexico — not a US-states panel
    assert!(html.contains(r#""type":"choropleth""#));
    assert!(html.contains(r#""locationmode":"ISO-3""#));
    assert!(html.contains("MEX"));
    assert!(!html.contains(r#""locationmode":"USA-states""#));
}
