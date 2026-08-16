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
    // single-series bar charts get magnitude-formatted value labels above each bar. These are
    // rendered by qsv (a per-bar literal array), not by a d3 SI template, because d3
    // cannot emit the "B" English pages need for 1e9 (issue #4393).
    assert!(html.contains(r#""texttemplate":["#));
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
fn viz_line_rangeslider() {
    let wrk = Workdir::new("viz_line_rangeslider");
    fruits(&wrk);

    let mut cmd = wrk.command("viz");
    cmd.args([
        "line",
        "fruits.csv",
        "--x",
        "Qty",
        "--y",
        "Price",
        "--rangeslider",
    ]);
    let out = wrk.output(&mut cmd);
    assert!(out.status.success());
    let html = String::from_utf8_lossy(&out.stdout);
    // the x-axis carries a visible range-slider navigator strip
    assert!(html.contains(r#""rangeslider":{"visible":true}"#));
}

#[test]
fn viz_rangeslider_non_cartesian_errors() {
    let wrk = Workdir::new("viz_rangeslider_non_cartesian_errors");
    fruits(&wrk);

    let mut cmd = wrk.command("viz");
    cmd.args([
        "pie",
        "fruits.csv",
        "--x",
        "Fruit",
        "--y",
        "Price",
        "--rangeslider",
    ]);
    let out = wrk.output(&mut cmd);
    assert!(!out.status.success());
    let stderr = wrk.output_stderr(&mut cmd);
    assert!(stderr.contains("--rangeslider only applies to cartesian charts"));
}

#[test]
fn viz_bar_slider_animation() {
    let wrk = Workdir::new("viz_bar_slider_animation");
    wrk.create_from_string(
        "anim.csv",
        "year,fruit,sales\n2020,apple,10\n2021,apple,14\n2022,apple,9\n",
    );

    let mut cmd = wrk.command("viz");
    cmd.args([
        "bar", "anim.csv", "--x", "fruit", "--y", "sales", "--slider", "year",
    ]);
    let out = wrk.output(&mut cmd);
    assert!(out.status.success());
    let html = String::from_utf8_lossy(&out.stdout);

    // a scrub slider + a Play/Pause menu drive the animation
    assert!(html.contains(r#""sliders":["#));
    assert!(html.contains(r#""updatemenus":["#));
    assert!(html.contains("▶ Play"));
    assert!(html.contains("⏸ Pause"));
    // one animation frame per distinct year
    assert!(html.contains(r#""name":"2020""#));
    assert!(html.contains(r#""name":"2021""#));
    assert!(html.contains(r#""name":"2022""#));
    // the y-axis is pinned to a fixed range so it doesn't jump between frames (bars start at 0)
    assert!(html.contains(r#""yaxis""#) && html.contains(r#""range":[0.0,"#));
}

#[test]
fn viz_scatter_slider_numeric_frame_order() {
    // frames must order numerically (2 before 10), not lexically ("10" < "2" as strings)
    let wrk = Workdir::new("viz_scatter_slider_numeric_frame_order");
    wrk.create_from_string("t.csv", "t,x,y\n10,1,1\n2,1,2\n2,2,3\n10,2,4\n");

    let mut cmd = wrk.command("viz");
    cmd.args(["scatter", "t.csv", "--x", "x", "--y", "y", "--slider", "t"]);
    let out = wrk.output(&mut cmd);
    assert!(out.status.success());
    let html = String::from_utf8_lossy(&out.stdout);

    let p2 = html.find(r#""name":"2""#).expect("frame 2 present");
    let p10 = html.find(r#""name":"10""#).expect("frame 10 present");
    assert!(
        p2 < p10,
        "frame 2 must come before frame 10 (numeric order)"
    );
}

#[test]
fn viz_slider_series_stable_trace_count() {
    // banana is absent in 2021 and cherry appears only in 2022, but every frame must still carry
    // all three series (equal trace count + stable indices) or plotly leaves stale traces on screen
    let wrk = Workdir::new("viz_slider_series_stable_trace_count");
    wrk.create_from_string(
        "d.csv",
        "year,fruit,sales\n2020,apple,10\n2020,banana,8\n2021,apple,14\n2022,apple,9\n2022,banana,\
         12\n2022,cherry,4\n",
    );

    let mut cmd = wrk.command("viz");
    cmd.args([
        "scatter", "d.csv", "--x", "fruit", "--y", "sales", "--slider", "year", "--series", "fruit",
    ]);
    let out = wrk.output(&mut cmd);
    assert!(out.status.success());
    let html = String::from_utf8_lossy(&out.stdout);

    // each of the 3 frames maps the same 3 trace indices
    assert_eq!(html.matches(r#""traces":[0,1,2]"#).count(), 3);
}

#[test]
fn viz_slider_agg_count_not_double_aggregated() {
    // regression: frame groups are stored raw and aggregated exactly once. With --agg count,
    // category A has 3 rows in 2020, so its bar height is 3 — a second aggregation pass would
    // collapse each already-counted bucket to 1.
    let wrk = Workdir::new("viz_slider_agg_count_not_double_aggregated");
    wrk.create_from_string(
        "d.csv",
        "year,cat,v\n2020,A,1\n2020,A,1\n2020,A,1\n2020,B,1\n2020,B,1\n2021,A,1\n2021,B,1\n",
    );

    let mut cmd = wrk.command("viz");
    cmd.args([
        "bar", "d.csv", "--x", "cat", "--y", "v", "--agg", "count", "--slider", "year",
    ]);
    let out = wrk.output(&mut cmd);
    assert!(out.status.success());
    let html = String::from_utf8_lossy(&out.stdout);

    // frame 2020 counts A=3, B=2 — a second aggregation pass would collapse each bucket to 1
    // (yielding [1.0,1.0]), so the presence of [3.0,2.0] proves it aggregated exactly once
    assert!(html.contains(r#""y":[3.0,2.0]"#));
}

#[test]
fn viz_slider_categorical_x_pinned() {
    // a categorical x-axis is pinned to a fixed category array (order + membership) so bars don't
    // reorder or drop between frames — here A only appears in 2020 and C only in 2021
    let wrk = Workdir::new("viz_slider_categorical_x_pinned");
    wrk.create_from_string(
        "d.csv",
        "year,cat,v\n2020,A,5\n2020,B,3\n2021,B,4\n2021,C,2\n",
    );

    let mut cmd = wrk.command("viz");
    cmd.args(["bar", "d.csv", "--x", "cat", "--y", "v", "--slider", "year"]);
    let out = wrk.output(&mut cmd);
    assert!(out.status.success());
    let html = String::from_utf8_lossy(&out.stdout);

    assert!(html.contains(r#""categoryorder":"array""#));
    assert!(html.contains(r#""categoryarray":["A","B","C"]"#));
}

#[test]
fn viz_bar_slider_distinct_bar_colors() {
    // a single-series animated bar chart gives each category a distinct, category-keyed color
    // (from PALETTE, in pinned category-array order) AND a stable per-bar `ids` (the category)
    // so bars read as growing/shrinking in place instead of sliding between columns as it plays.
    // `ids` is plotly's animation_group / object-constancy mechanism: without it plotly matches
    // bars across frames by ARRAY INDEX, so a frame whose rows arrive in a different order makes
    // a bar physically slide from one category's slot to another during the transition.
    let wrk = Workdir::new("viz_bar_slider_distinct_bar_colors");
    // frame 2021's rows are deliberately in a DIFFERENT order (C,B,A) than 2020 (A,B,C) so the
    // index-vs-id distinction actually matters
    wrk.create_from_string(
        "d.csv",
        "year,cat,v\n2020,A,5\n2020,B,3\n2020,C,7\n2021,C,2\n2021,B,6\n2021,A,4\n",
    );

    let mut cmd = wrk.command("viz");
    cmd.args(["bar", "d.csv", "--x", "cat", "--y", "v", "--slider", "year"]);
    let out = wrk.output(&mut cmd);
    assert!(out.status.success());
    let html = String::from_utf8_lossy(&out.stdout);

    // pinned category order A,B,C -> PALETTE[0],[1],[2]; the base (2020) trace carries a
    // per-category color array so A stays #4C78A8, B stays #F58518, C stays #54A24B
    assert!(html.contains(r##""marker":{"color":["#4C78A8","#F58518","#54A24B"]}"##));
    // the base trace's bars carry stable category ids in data order (A,B,C)...
    assert!(html.contains(r#""ids":["A","B","C"]"#));
    // ...and the 2021 frame keeps each bar's id even though its rows are in C,B,A order, so
    // plotly transitions each bar in place by identity rather than sliding by index
    assert!(html.contains(r#""ids":["C","B","A"]"#));
}

#[test]
fn viz_slider_auto_standalone_errors() {
    let wrk = Workdir::new("viz_slider_auto_standalone_errors");
    fruits(&wrk);

    let mut cmd = wrk.command("viz");
    cmd.args([
        "bar",
        "fruits.csv",
        "--x",
        "Fruit",
        "--y",
        "Price",
        "--slider",
        "auto",
    ]);
    let out = wrk.output(&mut cmd);
    assert!(!out.status.success());
    let stderr = wrk.output_stderr(&mut cmd);
    assert!(stderr.contains("auto/on/off only apply to `viz smart`"));
}

#[test]
fn viz_slider_unsupported_chart_errors() {
    let wrk = Workdir::new("viz_slider_unsupported_chart_errors");
    fruits(&wrk);

    let mut cmd = wrk.command("viz");
    cmd.args(["histogram", "fruits.csv", "--x", "Price", "--slider", "Qty"]);
    let out = wrk.output(&mut cmd);
    assert!(!out.status.success());
    let stderr = wrk.output_stderr(&mut cmd);
    assert!(stderr.contains("--slider currently supports"));
}

#[test]
fn viz_geo_slider_animation() {
    // scattergeo animates natively (unlike MapLibre scattermap), so `viz geo --slider` builds a
    // full animated point map with frames + a scrub slider + Play/Pause
    let wrk = Workdir::new("viz_geo_slider_animation");
    wrk.create_from_string(
        "pts.csv",
        "year,cat,lat,lon\n2020,A,37,-122\n2020,B,40,-74\n2021,A,38,-121\n2021,B,41,-73\n2022,A,\
         39,-120\n2022,B,42,-72\n",
    );

    let mut cmd = wrk.command("viz");
    cmd.args([
        "geo", "pts.csv", "--lat", "lat", "--lon", "lon", "--slider", "year", "--series", "cat",
    ]);
    let out = wrk.output(&mut cmd);
    assert!(out.status.success());
    let html = String::from_utf8_lossy(&out.stdout);

    assert!(html.contains(r#""type":"scattergeo""#));
    assert!(html.contains(r#""sliders":["#));
    assert!(html.contains("▶ Play"));
    assert!(html.contains(r#""name":"2020""#));
    assert!(html.contains(r#""name":"2022""#));
    // both series appear in every frame with stable trace indices
    assert_eq!(html.matches(r#""traces":[0,1]"#).count(), 3);
}

#[test]
fn viz_map_slider_errors() {
    // MapLibre scattermap can't animate reliably, so --slider on `viz map` is rejected with a
    // pointer to `viz geo`
    let wrk = Workdir::new("viz_map_slider_errors");
    wrk.create_from_string("pts.csv", "year,lat,lon\n2020,37,-122\n2021,40,-74\n");

    let mut cmd = wrk.command("viz");
    cmd.args([
        "map", "pts.csv", "--lat", "lat", "--lon", "lon", "--slider", "year",
    ]);
    let out = wrk.output(&mut cmd);
    assert!(!out.status.success());
    let stderr = wrk.output_stderr(&mut cmd);
    assert!(stderr.contains("isn't supported for `viz map`"));
    assert!(stderr.contains("viz geo"));
}

/// 3 regions x 3 months, with SEVERAL rows per (region, month) cell so the cell aggregation is
/// observable — a 1-row-per-cell fixture makes every --agg the identity and would pin nothing.
/// Region C is deliberately absent from month 2.
fn bubble_csv() -> String {
    // built by joining short row literals rather than one long "a\nb\nc" string: rustfmt's
    // format_strings re-wraps a long literal at max_width and can land the break INSIDE a `\n`
    // escape, turning it into a literal backslash + n and silently corrupting the fixture
    csv_rows(&[
        "region,month,gdp,well,pop",
        "A,2024-01-01,10,20,100",
        "A,2024-01-01,20,40,200",
        "A,2024-02-01,30,30,300",
        "A,2024-03-01,40,50,400",
        "B,2024-01-01,50,10,500",
        "B,2024-02-01,60,20,600",
        "B,2024-03-01,70,30,700",
        "C,2024-01-01,80,60,800",
        "C,2024-03-01,90,70,900",
    ])
}

/// Join CSV rows into a trailing-newline-terminated document. See [`bubble_csv`] for why the
/// fixtures are built this way instead of as one long string literal.
fn csv_rows(rows: &[&str]) -> String {
    let mut s = rows.join("\n");
    s.push('\n');
    s
}

/// Pull the plotly `frames` array out of the emitted HTML by brace-scanning from the `"frames":`
/// key. The bubble tests assert on per-frame VALUES, which a substring probe can't do reliably —
/// the frames are adjacent in the JSON, so a window match silently reads the neighboring frame.
fn frames_json(html: &str) -> serde_json::Value {
    let k = html.find(r#""frames":"#).expect("no frames in plot");
    let start = html[k..].find('[').expect("frames array") + k;
    let (mut depth, mut i) = (0_i32, start);
    let bytes = html.as_bytes();
    loop {
        match bytes[i] {
            b'[' | b'{' => depth += 1,
            b']' | b'}' => depth -= 1,
            _ => {},
        }
        i += 1;
        if depth == 0 {
            break;
        }
    }
    serde_json::from_str(&html[start..i]).expect("frames parse")
}

/// The x value plotted for `entity` in the frame named `frame`, or None when the bubble is hidden
/// that frame (an absent cell renders as an EMPTY coordinate vector, not NaN).
fn bubble_x(frames: &serde_json::Value, frame: &str, entity: &str) -> Option<f64> {
    let f = frames
        .as_array()
        .unwrap()
        .iter()
        .find(|f| f["name"] == frame)
        .expect("frame not found");
    let t = f["data"]
        .as_array()
        .unwrap()
        .iter()
        .find(|t| t["name"] == entity)
        .expect("entity trace not found");
    t["x"].as_array().unwrap().first().and_then(|v| v.as_f64())
}

#[test]
fn viz_scatter_slider_bubble_animation() {
    // the Gapminder combination (--slider + --series + --size) builds one bubble per entity that
    // moves and resizes across frames, with pinned axes so it doesn't reframe as it plays
    let wrk = Workdir::new("viz_scatter_slider_bubble_animation");
    wrk.create_from_string("d.csv", &bubble_csv());

    let mut cmd = wrk.command("viz");
    cmd.args([
        "scatter", "d.csv", "--x", "gdp", "--y", "well", "--size", "pop", "--series", "region",
        "--slider", "month",
    ]);
    let out = wrk.output(&mut cmd);
    assert!(out.status.success());
    let html = String::from_utf8_lossy(&out.stdout);

    assert!(html.contains(r#""sliders":["#));
    assert!(html.contains(r#""updatemenus":["#));
    assert!(html.contains("▶ Play"));
    assert!(html.contains("⏸ Pause"));
    // one frame per month, and EVERY frame maps the same 3 trace indices — a varying trace count
    // would leave stale bubbles on screen
    assert_eq!(html.matches(r#""traces":[0,1,2]"#).count(), 3);
    // both axes are pinned to a fixed range
    assert_eq!(html.matches(r#""range":["#).count(), 2);
    // legend of entities (one trace each)
    for ent in ["A", "B", "C"] {
        assert!(html.contains(&format!(r#""name":"{ent}""#)));
    }
}

#[test]
fn viz_scatter_slider_bubble_cell_mean() {
    // each (entity, frame) cell collapses to ONE bubble via --agg, defaulting to the mean (the
    // centroid `viz smart` has always drawn). A's month-1 cell holds gdp 10 and 20 -> 15.
    let wrk = Workdir::new("viz_scatter_slider_bubble_cell_mean");
    wrk.create_from_string("d.csv", &bubble_csv());

    let mut cmd = wrk.command("viz");
    cmd.args([
        "scatter", "d.csv", "--x", "gdp", "--y", "well", "--size", "pop", "--series", "region",
        "--slider", "month",
    ]);
    let out = wrk.output(&mut cmd);
    assert!(out.status.success());
    let frames = frames_json(&String::from_utf8_lossy(&out.stdout));

    assert_eq!(bubble_x(&frames, "2024-01-01", "A"), Some(15.0));
    // single-row cells are unaffected
    assert_eq!(bubble_x(&frames, "2024-02-01", "A"), Some(30.0));
}

#[test]
fn viz_scatter_slider_bubble_agg_sum() {
    // --agg is honored, and applied exactly once: A's month-1 cell (10 + 20) sums to 30, not to
    // some doubly-aggregated value
    let wrk = Workdir::new("viz_scatter_slider_bubble_agg_sum");
    wrk.create_from_string("d.csv", &bubble_csv());

    let mut cmd = wrk.command("viz");
    cmd.args([
        "scatter", "d.csv", "--x", "gdp", "--y", "well", "--size", "pop", "--series", "region",
        "--slider", "month", "--agg", "sum",
    ]);
    let out = wrk.output(&mut cmd);
    assert!(out.status.success());
    let frames = frames_json(&String::from_utf8_lossy(&out.stdout));

    assert_eq!(bubble_x(&frames, "2024-01-01", "A"), Some(30.0));
}

#[test]
fn viz_scatter_slider_bubble_cumulative() {
    // On a bubble there is one point per entity per frame, so nothing can pile up on screen the
    // way it does for bar/line/geo. --slider-cumulative instead makes each frame a RUNNING
    // aggregate over frames 0..=k: A's month-2 bubble is mean(10, 20, 30) = 20, not 30.
    let wrk = Workdir::new("viz_scatter_slider_bubble_cumulative");
    wrk.create_from_string("d.csv", &bubble_csv());

    let mut cmd = wrk.command("viz");
    cmd.args([
        "scatter",
        "d.csv",
        "--x",
        "gdp",
        "--y",
        "well",
        "--size",
        "pop",
        "--series",
        "region",
        "--slider",
        "month",
        "--slider-cumulative",
    ]);
    let out = wrk.output(&mut cmd);
    assert!(out.status.success());
    let frames = frames_json(&String::from_utf8_lossy(&out.stdout));

    assert_eq!(bubble_x(&frames, "2024-01-01", "A"), Some(15.0));
    assert_eq!(bubble_x(&frames, "2024-02-01", "A"), Some(20.0));
    // mean(10, 20, 30, 40)
    assert_eq!(bubble_x(&frames, "2024-03-01", "A"), Some(25.0));
}

#[test]
fn viz_scatter_slider_bubble_non_date_frames() {
    // a bare year parses as a date too, so the frame axis MUST check numeric first: otherwise the
    // column would be calendar-bucketed and labelled "2020-01-01", and `viz scatter --slider year`
    // would disagree with `viz bar --slider year`, which frames on distinct values
    let wrk = Workdir::new("viz_scatter_slider_bubble_non_date_frames");
    wrk.create_from_string(
        "d.csv",
        &csv_rows(&[
            "year,ent,x,y,p",
            "2020,A,1,2,10",
            "2020,B,5,6,20",
            "2021,A,2,3,11",
            "2021,B,6,7,22",
            "2010,A,9,9,5",
            "2010,B,8,8,6",
        ]),
    );

    let mut cmd = wrk.command("viz");
    cmd.args([
        "scatter", "d.csv", "--x", "x", "--y", "y", "--size", "p", "--series", "ent", "--slider",
        "year",
    ]);
    let out = wrk.output(&mut cmd);
    assert!(out.status.success());
    let html = String::from_utf8_lossy(&out.stdout);

    assert!(html.contains(r#""name":"2020""#));
    assert!(!html.contains(r#""name":"2020-01-01""#));
    // and the frames are ordered numerically, not lexically
    let p2010 = html.find(r#""name":"2010""#).unwrap();
    let p2020 = html.find(r#""name":"2020""#).unwrap();
    let p2021 = html.find(r#""name":"2021""#).unwrap();
    assert!(p2010 < p2020 && p2020 < p2021);
}

#[test]
fn viz_scatter_slider_bubble_size_global_scale() {
    // Bubble sizes are scaled ONCE over every (entity, frame) cell, so sizes stay comparable
    // across frames and entities. Rescaling per trace would see a single value, land it at the
    // midpoint, and render every bubble the same size — silently destroying the size encoding.
    let wrk = Workdir::new("viz_scatter_slider_bubble_size_global_scale");
    wrk.create_from_string("d.csv", &bubble_csv());

    let mut cmd = wrk.command("viz");
    cmd.args([
        "scatter", "d.csv", "--x", "gdp", "--y", "well", "--size", "pop", "--series", "region",
        "--slider", "month",
    ]);
    let out = wrk.output(&mut cmd);
    assert!(out.status.success());
    let html = String::from_utf8_lossy(&out.stdout);

    // the smallest and largest cells hit the ends of the pixel range, and several distinct sizes
    // appear in between — none of which happens under a per-trace rescale
    let sizes: std::collections::BTreeSet<i64> = frames_json(&html)
        .as_array()
        .unwrap()
        .iter()
        .flat_map(|f| f["data"].as_array().unwrap().clone())
        .filter_map(|t| t["marker"]["size"].as_i64())
        .collect();
    assert!(
        sizes.len() > 3,
        "expected varied bubble sizes, got {sizes:?}"
    );
    assert_eq!(sizes.iter().next(), Some(&6));
    assert_eq!(sizes.iter().next_back(), Some(&40));
}

#[test]
fn viz_scatter_slider_bubble_sparse_is_drawn_and_noted() {
    // `viz smart` DROPS an entity that isn't present in most frames, because it auto-selects that
    // panel and a flickering bubble reads as sampling noise. An explicitly requested chart is a
    // different contract: region C is missing month 2, and must still be drawn (hidden only in
    // that frame) with the gap reported on stderr.
    let wrk = Workdir::new("viz_scatter_slider_bubble_sparse_is_drawn_and_noted");
    wrk.create_from_string("d.csv", &bubble_csv());

    let mut cmd = wrk.command("viz");
    cmd.args([
        "scatter", "d.csv", "--x", "gdp", "--y", "well", "--size", "pop", "--series", "region",
        "--slider", "month",
    ]);
    let out = wrk.output(&mut cmd);
    assert!(out.status.success());
    let frames = frames_json(&String::from_utf8_lossy(&out.stdout));

    assert_eq!(bubble_x(&frames, "2024-01-01", "C"), Some(80.0));
    // hidden in the frame it has no data for — an EMPTY coordinate vector keeps the trace count
    // and index order constant
    assert_eq!(bubble_x(&frames, "2024-02-01", "C"), None);
    assert_eq!(bubble_x(&frames, "2024-03-01", "C"), Some(90.0));

    let stderr = wrk.output_stderr(&mut cmd);
    assert!(stderr.contains("cells have no data"));
}

#[test]
fn viz_scatter_slider_bubble_agg_count_errors() {
    // a row count is not a position, so --agg count can't drive a bubble's x/y — and the error
    // points at the fallback that DOES size by count
    let wrk = Workdir::new("viz_scatter_slider_bubble_agg_count_errors");
    wrk.create_from_string("d.csv", &bubble_csv());

    let mut cmd = wrk.command("viz");
    cmd.args([
        "scatter", "d.csv", "--x", "gdp", "--y", "well", "--size", "pop", "--series", "region",
        "--slider", "month", "--agg", "count",
    ]);
    let out = wrk.output(&mut cmd);
    assert!(!out.status.success());
    let stderr = wrk.output_stderr(&mut cmd);
    assert!(stderr.contains("--agg count has no meaning"));
    // must NOT tell the user to omit --size: the bubble path is only dispatched WITH --size, so
    // dropping it silently yields a plain animated scatter (many points per series, no sizing),
    // not a count-sized bubble. That fallback exists only for `viz smart`.
    assert!(!stderr.contains("omit --size"));
}

#[test]
fn viz_scatter_slider_bubble_mostly_numeric_frames_stay_raw() {
    // REGRESSION: `qsv_dateparser` reads a bare year as EPOCH SECONDS ("2010" ->
    // 1970-01-01T00:33:30), so a year column pushed onto the calendar path collapses every
    // frame into a single 1970 bucket and the animation dies with "No plottable bubbles found".
    // Testing "every cell is numeric" was too brittle: ONE non-numeric cell out of 21 flipped
    // the whole column onto that path. The mode is chosen by COVERAGE, and a year ties (it
    // parses as both), so numeric wins.
    let wrk = Workdir::new("viz_scatter_slider_bubble_mostly_numeric_frames_stay_raw");
    let mut rows = vec!["yr,ent,x,y,p".to_string()];
    for (i, y) in [2010, 2015, 2020, 2021, 2022, 2023, 2024, 2025, 2026, 2027]
        .iter()
        .enumerate()
    {
        rows.push(format!("{y},A,{},{},{}", i + 1, i + 2, 10 + i));
        rows.push(format!("{y},B,{},{},{}", i + 5, i + 6, 20 + i));
    }
    // the single non-numeric cell that used to flip the classification
    rows.push("N/A,B,3,3,7".to_string());
    let refs: Vec<&str> = rows.iter().map(String::as_str).collect();
    wrk.create_from_string("d.csv", &csv_rows(&refs));

    let mut cmd = wrk.command("viz");
    cmd.args([
        "scatter", "d.csv", "--x", "x", "--y", "y", "--size", "p", "--series", "ent", "--slider",
        "yr",
    ]);
    let out = wrk.output(&mut cmd);
    assert!(out.status.success());
    let html = String::from_utf8_lossy(&out.stdout);

    // framed on the raw years, NOT on epoch-derived 1970 buckets
    assert!(html.contains(r#""name":"2010""#));
    assert!(html.contains(r#""name":"2027""#));
    assert!(!html.contains("1970-01-01"));
}

#[test]
fn viz_scatter_slider_bubble_numeric_frames_survive_a_date_parseable_stray() {
    // REGRESSION: comparing the two coverages against each other ("more dates than numbers") is
    // NOT enough, because a numeric cell also counts as a parsed date. 20 bare years plus one real
    // "2024-01-01" gives 21 dates vs 20 numbers, which handed a fully numeric column to the
    // calendar path and re-collapsed the years into a single 1970 epoch bucket. The coverages are
    // therefore tested separately: the calendar path needs numeric coverage to be NEGLIGIBLE.
    let wrk =
        Workdir::new("viz_scatter_slider_bubble_numeric_frames_survive_a_date_parseable_stray");
    let mut rows = vec!["yr,ent,x,y,p".to_string()];
    for (i, y) in [2010, 2015, 2020, 2021, 2022, 2023, 2024, 2025, 2026, 2027]
        .iter()
        .enumerate()
    {
        rows.push(format!("{y},A,{},{},{}", i + 1, i + 2, 10 + i));
        rows.push(format!("{y},B,{},{},{}", i + 5, i + 6, 20 + i));
    }
    // the stray cell is itself date-parseable — this is what defeated the old comparison
    rows.push("2024-01-01,B,3,3,7".to_string());
    let refs: Vec<&str> = rows.iter().map(String::as_str).collect();
    wrk.create_from_string("d.csv", &csv_rows(&refs));

    let mut cmd = wrk.command("viz");
    cmd.args([
        "scatter", "d.csv", "--x", "x", "--y", "y", "--size", "p", "--series", "ent", "--slider",
        "yr",
    ]);
    let out = wrk.output(&mut cmd);
    assert!(out.status.success());
    let html = String::from_utf8_lossy(&out.stdout);

    assert!(html.contains(r#""name":"2010""#));
    assert!(html.contains(r#""name":"2027""#));
    assert!(!html.contains("1970-01-01"));
}

#[test]
fn viz_scatter_slider_bubble_dates_with_a_stray_number_stay_calendar() {
    // the other direction of the same rule: requiring negligible NUMERIC coverage must not cost a
    // real date column its calendar bucketing just because one cell happens to be a bare number
    let wrk = Workdir::new("viz_scatter_slider_bubble_dates_with_a_stray_number_stay_calendar");
    let mut rows = vec!["d,ent,x,y,p".to_string()];
    for i in 0..10 {
        rows.push(format!(
            "2024-{:02}-01,A,{},{},{}",
            i + 1,
            i + 1,
            i + 2,
            10 + i
        ));
        rows.push(format!(
            "2024-{:02}-01,B,{},{},{}",
            i + 1,
            i + 5,
            i + 6,
            20 + i
        ));
    }
    rows.push("7,B,3,3,7".to_string());
    let refs: Vec<&str> = rows.iter().map(String::as_str).collect();
    wrk.create_from_string("d.csv", &csv_rows(&refs));

    let mut cmd = wrk.command("viz");
    cmd.args([
        "scatter", "d.csv", "--x", "x", "--y", "y", "--size", "p", "--series", "ent", "--slider",
        "d",
    ]);
    let out = wrk.output(&mut cmd);
    assert!(out.status.success());
    let html = String::from_utf8_lossy(&out.stdout);

    assert!(html.contains(r#""name":"2024-01-01""#));
    assert!(html.contains(r#""name":"2024-10-01""#));
}

#[test]
fn viz_scatter_slider_bubble_date_column_still_calendar_bucketed() {
    // the coverage rule must not cost real date columns their calendar bucketing: "2024-01-01"
    // has zero numeric coverage, so dates win outright and the frames are calendar buckets
    let wrk = Workdir::new("viz_scatter_slider_bubble_date_column_still_calendar_bucketed");
    wrk.create_from_string("d.csv", &bubble_csv());

    let mut cmd = wrk.command("viz");
    cmd.args([
        "scatter", "d.csv", "--x", "gdp", "--y", "well", "--size", "pop", "--series", "region",
        "--slider", "month",
    ]);
    let out = wrk.output(&mut cmd);
    assert!(out.status.success());
    let html = String::from_utf8_lossy(&out.stdout);

    assert!(html.contains(r#""name":"2024-01-01""#));
    assert!(html.contains(r#""name":"2024-03-01""#));
}

#[test]
fn viz_scatter_slider_size_without_series_errors() {
    // --size under --slider means a bubble animation, which needs an entity to put bubbles on
    let wrk = Workdir::new("viz_scatter_slider_size_without_series_errors");
    wrk.create_from_string("d.csv", &bubble_csv());

    let mut cmd = wrk.command("viz");
    cmd.args([
        "scatter", "d.csv", "--x", "gdp", "--y", "well", "--size", "pop", "--slider", "month",
    ]);
    let out = wrk.output(&mut cmd);
    assert!(!out.status.success());
    let stderr = wrk.output_stderr(&mut cmd);
    assert!(stderr.contains("Add --series"));
}

#[test]
fn viz_scatter_slider_color_errors() {
    // --color stays a CONTINUOUS numeric encoding everywhere; animating a colorscale is a
    // separate follow-up, so it is still rejected under --slider
    let wrk = Workdir::new("viz_scatter_slider_color_errors");
    wrk.create_from_string("d.csv", &bubble_csv());

    let mut cmd = wrk.command("viz");
    cmd.args([
        "scatter", "d.csv", "--x", "gdp", "--y", "well", "--color", "pop", "--slider", "month",
    ]);
    let out = wrk.output(&mut cmd);
    assert!(!out.status.success());
    let stderr = wrk.output_stderr(&mut cmd);
    assert!(stderr.contains("--slider cannot yet be combined with --color"));
}

#[test]
fn viz_scatter_slider_color_with_series_still_errors() {
    // REGRESSION GUARD for the bubble relaxation: the exception that lets --size sit beside
    // --series is deliberately narrowed to "--size and NOT --color". Relaxing on "has any marker
    // encoding" instead would wave this combination past the --series conflict check and into the
    // slider dispatch, which only screens for a lone --color.
    let wrk = Workdir::new("viz_scatter_slider_color_with_series_still_errors");
    wrk.create_from_string("d.csv", &bubble_csv());

    let mut cmd = wrk.command("viz");
    cmd.args([
        "scatter", "d.csv", "--x", "gdp", "--y", "well", "--color", "pop", "--series", "region",
        "--slider", "month",
    ]);
    let out = wrk.output(&mut cmd);
    assert!(!out.status.success());
    let stderr = wrk.output_stderr(&mut cmd);
    assert!(stderr.contains("cannot be combined with --series"));
}

/// 36 rows: a TAUTOLOGICAL numeric pair (x, y = 2x → r=1.0, a rigid line) over 3 monthly dates.
/// Its centroids can only slide along the line (curvature ~0), so `viz smart` must NOT animate it —
/// animation is reserved for relationships whose 2-D shape genuinely evolves over time.
fn smart_anim_pair_csv() -> String {
    let mut rows = String::from("date,x,y\n");
    for m in 1..=3 {
        for x in 0..12 {
            let y = x * 2;
            rows.push_str(&format!("2024-0{m}-01,{x},{y}\n"));
        }
    }
    rows
}

/// 45 rows: a pair (a, b) whose per-month centroids trace an inverted-U ARC — a genuinely evolving
/// 2-D relationship with ~zero global linear correlation (r≈0). A 3×3 integer grid per month keeps
/// both columns low-cardinality (not near-unique, so they stay in the correlation matrix). This is
/// the case the curvature selector animates and the old max-|r| logic never could.
fn smart_arc_pair_csv() -> String {
    let cents = [(0, 2), (3, 10), (6, 13), (9, 10), (12, 2)];
    let mut rows = String::from("date,a,b\n");
    for (m, (cx, cy)) in cents.iter().enumerate() {
        for dx in -1..=1 {
            for dy in -1..=1 {
                rows.push_str(&format!("2024-0{}-01,{},{}\n", m + 1, cx + dx, cy + dy));
            }
        }
    }
    rows
}

#[test]
fn viz_smart_animated_scatter_pair_when_temporal() {
    // a pair whose per-month centroids ARC (a genuinely evolving 2-D relationship) + a canonical
    // date column => `viz smart` (auto) animates the pair over time: a time-colored trailing-window
    // scatter with frames + a slider + Play/Pause. Note the relationship has r≈0 — the old max-|r|
    // logic would never have picked it; the curvature selector does.
    let wrk = Workdir::new("viz_smart_animated_scatter_pair_when_temporal");
    wrk.create_from_string("s.csv", &smart_arc_pair_csv());

    let out_html = wrk.path("dash.html").to_string_lossy().to_string();
    let mut cmd = wrk.command("viz");
    // NO_COMPRESS so the panel JSON is assertable in plaintext (the browser check covers the
    // compressed replay-chain path separately).
    cmd.env("QSV_VIZ_NO_COMPRESS", "1");
    cmd.args(["smart", "s.csv", "-o", &out_html]);
    wrk.assert_success(&mut cmd);
    let html = wrk.read_to_string("dash.html").unwrap();

    // the animated pair panel: "<a> vs <b> (r=..) over time"
    assert!(
        html.contains("over time"),
        "expected an animated pair title; html: {html}"
    );
    // frames (one per monthly bucket) + a scrub slider + Play
    assert!(html.contains(r#""sliders":["#));
    assert!(html.contains("▶ Play"));
    assert!(html.contains(r#""name":"2024-01-01""#));
    assert!(html.contains(r#""name":"2024-05-01""#));
    // markers colored by time bucket (an array color + a sequential scale + a legend)
    assert!(html.contains(r#""mode":"markers""#));
    assert!(html.contains(r#""color":["#));
    assert!(html.contains("Viridis"));
    assert!(html.contains(r#""showscale":true"#));
    // 5 monthly buckets => 5 frames, each keeping the single stable trace index
    assert_eq!(html.matches(r#""traces":[0]"#).count(), 5);
}

#[test]
fn viz_smart_animated_pair_title_reports_pearson_under_spearman() {
    // `smart_arc_pair_csv`'s centroids, mapped through 2^n so both columns become tail-dominated
    // (all positive, mean well past 2x the median). That flips the correlation matrix to Spearman,
    // and the animated pair selector reads its coefficient from whichever matrix it is handed —
    // so the title would report a rho while LABELING it "r", and the Pearson-vs-Spearman
    // nonlinearity note could never fire, its gap having collapsed to zero.
    //
    // The two coefficients are deliberately far apart on this fixture: Pearson r = -0.247 (the
    // exponential mapping leaves a linear tilt) while the rank relationship is a symmetric arc, so
    // Spearman rho = 0.00. A title reading "(r=0.00)" is the regression.
    let cents = [(0, 2), (3, 10), (6, 13), (9, 10), (12, 2)];
    let mut rows = String::from("date,a,b\n");
    for (m, (cx, cy)) in cents.iter().enumerate() {
        for dx in -1..=1_i32 {
            for dy in -1..=1_i32 {
                let a = 2_i64.pow(u32::try_from(cx + dx + 1).unwrap());
                let b = 2_i64.pow(u32::try_from(cy + dy + 1).unwrap());
                rows.push_str(&format!("2024-0{}-01,{a},{b}\n", m + 1));
            }
        }
    }

    let wrk = Workdir::new("viz_smart_animated_pair_title_reports_pearson_under_spearman");
    wrk.create_from_string("s.csv", &rows);

    let out_html = wrk.path("dash.html").to_string_lossy().to_string();
    let mut cmd = wrk.command("viz");
    cmd.env("QSV_VIZ_NO_COMPRESS", "1");
    cmd.args(["smart", "s.csv", "-o", &out_html]);
    wrk.assert_success(&mut cmd);
    let html = wrk.read_to_string("dash.html").unwrap();

    // the matrix must actually be in Spearman mode, or this asserts nothing
    assert!(
        html.contains("Correlation (Spearman"),
        "fixture must be tail-dominated enough to select Spearman; html: {html}"
    );
    assert!(
        html.contains("over time"),
        "expected an animated pair panel"
    );
    assert!(
        html.contains("(r=-0.25)"),
        "the animated pair title must carry the PEARSON r, not the Spearman rho it was selected \
         with; html: {html}"
    );
}

/// 1000 rows — below `SMART_CONTOUR_MIN_POINTS`, so the correlated pair routes to the SCATTER
/// branch rather than the density contour. Strictly positive with a heavy right tail: the bulk
/// sits in 1..=20 and a handful of rows are out at ~1e6, so on linear axes every point collapses
/// against the origin, while in log space the cloud spreads cleanly. Values repeat so the columns
/// stay well below the near-unique cutoff that would exclude them from the correlation matrix.
fn heavy_tailed_pair_csv() -> String {
    let mut rows = String::from("spend,commit\n");
    for i in 0..990 {
        let x = 1 + (i % 20);
        rows.push_str(&format!("{x},{}\n", x * 3));
    }
    for i in 0..10 {
        let x = 1_000_000 + i * 1000;
        rows.push_str(&format!("{x},{}\n", x * 3));
    }
    rows
}

#[test]
fn viz_smart_scatter_pair_judges_legibility_on_the_logged_axes() {
    // The scatter branch used to test for a degenerate cloud on the RAW values, before its log
    // axes were resolved — so a heavy-tailed but strictly positive pair was dropped for being
    // unreadable on linear axes it was never going to be drawn on, in exactly the case log
    // scaling exists to rescue (issue #4276). Legibility is now judged in the space the panel
    // renders in, mirroring the contour branch's linear -> log retry.
    let wrk = Workdir::new("viz_smart_scatter_pair_judges_legibility_on_the_logged_axes");
    wrk.create_from_string("h.csv", &heavy_tailed_pair_csv());

    let out_html = wrk.path("dash.html").to_string_lossy().to_string();
    let mut cmd = wrk.command("viz");
    cmd.env("QSV_VIZ_NO_COMPRESS", "1");
    cmd.args(["smart", "h.csv", "-o", &out_html]);
    wrk.assert_success(&mut cmd);

    let html = wrk.read_to_string("dash.html").unwrap();
    assert!(
        html.contains("spend vs commit"),
        "the heavy-tailed pair must survive: it is legible once logged; html: {html}"
    );
    assert!(
        html.contains("log x/y"),
        "both axes span orders of magnitude and hold no zeros, so both should be logged"
    );
}

#[test]
fn viz_smart_scatter_pair_log_scale_off_still_drops_an_illegible_cloud() {
    // `--log-scale off` is the user declining log axes, so the pair is judged — and dropped — on
    // the linear axes it will actually be drawn on. Absent beats unreadable, the same call the
    // contour branch makes under `off`.
    let wrk = Workdir::new("viz_smart_scatter_pair_log_scale_off_still_drops_an_illegible_cloud");
    wrk.create_from_string("h.csv", &heavy_tailed_pair_csv());

    let out_html = wrk.path("dash.html").to_string_lossy().to_string();
    let mut cmd = wrk.command("viz");
    cmd.env("QSV_VIZ_NO_COMPRESS", "1");
    cmd.args(["smart", "h.csv", "--log-scale", "off", "-o", &out_html]);
    wrk.assert_success(&mut cmd);

    let html = wrk.read_to_string("dash.html").unwrap();
    assert!(
        !html.contains("spend vs commit"),
        "with log declined, a cloud that collapses on linear axes should still be dropped"
    );
}

#[test]
fn viz_smart_pair_gated_out_when_tautological() {
    // headline critique fix: a rigid tautological pair (y = 2x, r=1.0) with a date column must NOT
    // animate — its centroids only slide along the line (curvature ~0), so a time reveal adds
    // nothing over a static scatter. The static correlation drill-down still appears.
    let wrk = Workdir::new("viz_smart_pair_gated_out_when_tautological");
    wrk.create_from_string("s.csv", &smart_anim_pair_csv());

    let out_html = wrk.path("dash.html").to_string_lossy().to_string();
    let mut cmd = wrk.command("viz");
    cmd.env("QSV_VIZ_NO_COMPRESS", "1");
    cmd.args(["smart", "s.csv", "-o", &out_html]);
    wrk.assert_success(&mut cmd);
    let html = wrk.read_to_string("dash.html").unwrap();

    // no animation for the tautological pair
    assert!(
        !html.contains("over time"),
        "a tautological pair should not animate; html: {html}"
    );
    assert!(!html.contains(r#""sliders":["#));
    assert!(!html.contains("▶ Play"));
    // but the static strongest-pair drill-down is still present
    assert!(
        html.contains("x vs y (r=1.00)"),
        "static strongest-pair drill-down should remain; html: {html}"
    );
}

#[test]
fn viz_smart_slider_off_no_animation() {
    // `--slider off` keeps the pair drill-down static (no frames/slider), even when the data WOULD
    // otherwise animate (the arc fixture).
    let wrk = Workdir::new("viz_smart_slider_off_no_animation");
    wrk.create_from_string("s.csv", &smart_arc_pair_csv());

    let out_html = wrk.path("dash.html").to_string_lossy().to_string();
    let mut cmd = wrk.command("viz");
    cmd.env("QSV_VIZ_NO_COMPRESS", "1");
    cmd.args(["smart", "s.csv", "--slider", "off", "-o", &out_html]);
    wrk.assert_success(&mut cmd);
    let html = wrk.read_to_string("dash.html").unwrap();

    // no animation chrome under --slider off
    assert!(
        !html.contains("over time"),
        "slider off should not animate; html: {html}"
    );
    assert!(!html.contains(r#""sliders":["#));
    assert!(!html.contains("▶ Play"));
}

#[test]
fn viz_smart_animated_pair_gated_without_temporal_axis() {
    // judiciousness guard: the arc pair but NO date column => no animation under `auto` (the pair
    // drill-down stays a static scatter). Same numbers as the arc fixture, date column removed.
    let wrk = Workdir::new("viz_smart_animated_pair_gated_without_temporal_axis");
    let arc = smart_arc_pair_csv();
    // drop the leading "date," header and the leading "2024-0m-01," of each row
    let mut rows = String::from("a,b\n");
    for line in arc.lines().skip(1) {
        let cols: Vec<&str> = line.splitn(2, ',').collect();
        rows.push_str(cols[1]);
        rows.push('\n');
    }
    wrk.create_from_string("s.csv", &rows);

    let out_html = wrk.path("dash.html").to_string_lossy().to_string();
    let mut cmd = wrk.command("viz");
    cmd.env("QSV_VIZ_NO_COMPRESS", "1");
    cmd.args(["smart", "s.csv", "-o", &out_html]);
    wrk.assert_success(&mut cmd);
    let html = wrk.read_to_string("dash.html").unwrap();

    // no temporal axis => no animated pair
    assert!(
        !html.contains("over time"),
        "no date column should not animate; html: {html}"
    );
    assert!(!html.contains(r#""sliders":["#));
    assert!(!html.contains("▶ Play"));
}

// ~30-row global-extent dated point cloud (5+ continents, lon span ~300°, lat span ~98°) across 5
// monthly dates. Header names `lat`/`lon` so `latlon_indices` detects the coordinate pair.
fn smart_world_dated_csv() -> String {
    let cities = [
        ("Tokyo", 35.68, 139.69),
        ("Santiago", -33.45, -70.67),
        ("Reykjavik", 64.15, -21.94),
        ("Cape Town", -33.92, 18.42),
        ("Sydney", -33.87, 151.21),
        ("Anchorage", 61.22, -149.90),
    ];
    let mut rows = String::from("place,lat,lon,event_date\n");
    for m in 1..=5 {
        for (place, lat, lon) in &cities {
            rows.push_str(&format!("{place},{lat},{lon},2024-0{m}-15\n"));
        }
    }
    rows
}

#[test]
fn viz_smart_geo_animates_large_extent() {
    // continental/global dated points + a canonical date column => `viz smart` (auto) animates a
    // cumulative geographic reveal on the ScatterGeo projection basemap (slider + Play/Pause).
    let wrk = Workdir::new("viz_smart_geo_animates_large_extent");
    wrk.create_from_string("s.csv", &smart_world_dated_csv());

    let out_html = wrk.path("dash.html").to_string_lossy().to_string();
    let mut cmd = wrk.command("viz");
    cmd.env("QSV_VIZ_NO_COMPRESS", "1");
    cmd.args(["smart", "s.csv", "-o", &out_html]);
    wrk.assert_success(&mut cmd);
    let html = wrk.read_to_string("dash.html").unwrap();

    assert!(
        html.contains("locations over time"),
        "expected the animated geo panel title; html: {html}"
    );
    assert!(html.contains(r#""sliders":["#));
    assert!(html.contains("\u{25b6} Play"));
    // one frame per monthly bucket, named by ISO date
    assert!(html.contains(r#""name":"2024-01-15""#));
    assert!(html.contains(r#""name":"2024-05-15""#));
    // rendered on the geo projection basemap (not a cartesian scatter)
    assert!(html.contains(r#""type":"scattergeo""#));
}

#[test]
fn viz_smart_geo_gated_city_scale() {
    // city-scale dated points (~0.3° span) must NOT animate a geo panel — T2 only fires for a
    // continental/global extent (a MapLibre tile map, which can't animate natively, is left
    // static).
    let wrk = Workdir::new("viz_smart_geo_gated_city_scale");
    let pts = [
        (40.70, -74.01),
        (40.75, -73.98),
        (40.68, -73.95),
        (40.80, -73.96),
        (40.72, -74.00),
        (40.78, -73.99),
    ];
    let mut rows = String::from("place,lat,lon,event_date\n");
    for m in 1..=5 {
        for (i, (lat, lon)) in pts.iter().enumerate() {
            rows.push_str(&format!("stop{i},{lat},{lon},2024-0{m}-15\n"));
        }
    }
    wrk.create_from_string("s.csv", &rows);

    let out_html = wrk.path("dash.html").to_string_lossy().to_string();
    let mut cmd = wrk.command("viz");
    cmd.env("QSV_VIZ_NO_COMPRESS", "1");
    cmd.args(["smart", "s.csv", "-o", &out_html]);
    wrk.assert_success(&mut cmd);
    let html = wrk.read_to_string("dash.html").unwrap();

    assert!(
        !html.contains("locations over time"),
        "city-scale points should not animate a geo panel; html: {html}"
    );
}

// 5 regions x 6 quarters x 3 rows: each region traces a DISTINCT curved path in
// (gdp_index, wellbeing_index) space (real Gapminder-like divergence, not parallel lines), with
// exactly 3 rows per region-quarter so the min_cell_rows gate passes and the panel is complete.
// `population_m` is a distinct per-region third measure (Gapminder's bubble-SIZE variable); it is
// weakly/anti-correlated with the pair, so the selector still animates gdp-vs-wellbeing and sizes
// by population.
fn smart_gapminder_csv() -> String {
    let traj: [(&str, i32, [(i32, i32); 6]); 5] = [
        (
            "Northland",
            12,
            [(60, 55), (66, 62), (72, 68), (78, 71), (84, 72), (90, 73)],
        ),
        (
            "Eastmark",
            30,
            [(50, 48), (58, 50), (67, 53), (76, 60), (83, 70), (88, 80)],
        ),
        (
            "Sudland",
            48,
            [(45, 40), (48, 50), (50, 60), (52, 69), (55, 77), (58, 84)],
        ),
        (
            "Westfall",
            9,
            [(70, 58), (64, 61), (60, 63), (63, 66), (70, 70), (78, 74)],
        ),
        (
            "Centra",
            22,
            [(55, 52), (61, 57), (66, 63), (71, 66), (75, 71), (80, 75)],
        ),
    ];
    let q = [
        "2023-01-01",
        "2023-04-01",
        "2023-07-01",
        "2023-10-01",
        "2024-01-01",
        "2024-04-01",
    ];
    let mut rows = String::from("region,quarter_date,gdp_index,wellbeing_index,population_m\n");
    for (region, pop, path) in &traj {
        for (qi, (gx, wy)) in path.iter().enumerate() {
            for k in -1..=1 {
                rows.push_str(&format!("{region},{},{},{},{pop}\n", q[qi], gx + k, wy + k));
            }
        }
    }
    rows
}

#[test]
fn viz_smart_bubble_when_entity_and_drift() {
    // a low-cardinality categorical entity + a measure pair + a canonical date => `viz smart`
    // animates a Gapminder-style bubble chart: one bubble per entity moving through the measure
    // space over time (slider + Play/Pause, legend of entities), SIZED by a third data measure.
    let wrk = Workdir::new("viz_smart_bubble_when_entity_and_drift");
    wrk.create_from_string("s.csv", &smart_gapminder_csv());

    let out_html = wrk.path("dash.html").to_string_lossy().to_string();
    let mut cmd = wrk.command("viz");
    cmd.env("QSV_VIZ_NO_COMPRESS", "1");
    cmd.args(["smart", "s.csv", "-o", &out_html]);
    wrk.assert_success(&mut cmd);
    let html = wrk.read_to_string("dash.html").unwrap();

    // title "<y> vs <x> by <entity> over time"
    assert!(
        html.contains(" by region over time"),
        "expected the animated bubble title; html: {html}"
    );
    assert!(html.contains(r#""sliders":["#));
    assert!(html.contains("\u{25b6} Play"));
    // one trace per entity => the entity names appear as legend/trace names
    assert!(html.contains(r#""name":"Northland""#));
    assert!(html.contains(r#""name":"Sudland""#));
    // one frame per quarter bucket, named by ISO date
    assert!(html.contains(r#""name":"2023-01-01""#));
    assert!(html.contains(r#""name":"2024-04-01""#));
    // Gapminder's defining feature: bubbles are SIZED by a third data measure (population_m),
    // NOT the per-cell row count — so the hover names it and the marker sizes VARY across entities
    // (with a uniform 3-rows-per-cell dataset, count-sizing would collapse every bubble to one
    // size).
    assert!(
        html.contains("population_m"),
        "the bubble should be sized/labeled by the third measure; html: {html}"
    );
    let sizes: std::collections::HashSet<&str> = html
        .match_indices(r#""size":"#)
        .map(|(i, m)| {
            let rest = &html[i + m.len()..];
            let end = rest
                .find(|c: char| !c.is_ascii_digit())
                .unwrap_or(rest.len());
            &rest[..end]
        })
        .filter(|s| !s.is_empty())
        .collect();
    assert!(
        sizes.len() >= 2,
        "bubble marker sizes should vary (sized by the 3rd measure), got: {sizes:?}"
    );
}

/// The gapminder shape, but stretched over `years` ANNUAL frames instead of 6 quarters, so the
/// only thing that varies between two runs is the frame count.
fn smart_annual_bubble_csv(years: i32) -> String {
    let mut rows = String::from("region,year_date,gdp_index,wellbeing_index,population_m\n");
    for (region, pop, base) in [
        ("Northland", 12, 60),
        ("Eastmark", 30, 50),
        ("Sudland", 48, 45),
        ("Westfall", 9, 70),
        ("Centra", 22, 55),
    ] {
        for y in 0..years {
            // monotone drift through the measure space, so the panel is selected on its merits
            let (gx, wy) = (base + y, base - 5 + y * 2);
            for k in -1..=1 {
                rows.push_str(&format!(
                    "{region},{}-06-30,{},{},{pop}\n",
                    1980 + y,
                    gx + k,
                    wy + k
                ));
            }
        }
    }
    rows
}

#[test]
fn viz_smart_bubble_declines_past_the_frame_cap() {
    // The bubble panel resolves its frame axis through `resolve_frame_axis`, not the bucket
    // ladder, so it needs SMART_ANIM_MAX_FRAMES applied at the reader (`EntityBucketOpts`).
    // 25 annual frames animate; the SAME dataset stretched to 40 annual frames cannot be
    // bucketed below the 30-frame cap (each year is its own bucket at every rung), so the panel
    // is declined rather than emitting a 40-step slider that ignores the documented cap.
    let wrk = Workdir::new("viz_smart_bubble_declines_past_the_frame_cap");

    // positive control: under the cap, the animated bubble panel is present
    wrk.create_from_string("under.csv", &smart_annual_bubble_csv(25));
    let under_html = wrk.path("under.html").to_string_lossy().to_string();
    let mut cmd = wrk.command("viz");
    cmd.env("QSV_VIZ_NO_COMPRESS", "1");
    cmd.args(["smart", "under.csv", "-o", &under_html]);
    wrk.assert_success(&mut cmd);
    let html = wrk.read_to_string("under.html").unwrap();
    assert!(
        html.contains(" by region over time"),
        "25 annual frames is under the cap and should animate; html: {html}"
    );

    // over the cap: same shape, same gates, only the frame count differs
    wrk.create_from_string("over.csv", &smart_annual_bubble_csv(40));
    let over_html = wrk.path("over.html").to_string_lossy().to_string();
    let mut cmd = wrk.command("viz");
    cmd.env("QSV_VIZ_NO_COMPRESS", "1");
    cmd.args(["smart", "over.csv", "-o", &over_html]);
    wrk.assert_success(&mut cmd);
    let html = wrk.read_to_string("over.html").unwrap();
    assert!(
        !html.contains(" by region over time"),
        "40 annual frames exceeds SMART_ANIM_MAX_FRAMES and must be declined; html: {html}"
    );
}

#[test]
fn viz_smart_bubble_gated_without_entity() {
    // the same measure pair + date but NO categorical entity column => no bubble (and the
    // near-linear gdp/wellbeing trend doesn't drift, so nothing else animates either). Isolates the
    // entity as the discriminator.
    let wrk = Workdir::new("viz_smart_bubble_gated_without_entity");
    let with_entity = smart_gapminder_csv();
    // drop the leading "region," header and the leading "<region>," of each row
    let mut rows = String::from("quarter_date,gdp_index,wellbeing_index,population_m\n");
    for line in with_entity.lines().skip(1) {
        let cols: Vec<&str> = line.splitn(2, ',').collect();
        rows.push_str(cols[1]);
        rows.push('\n');
    }
    wrk.create_from_string("s.csv", &rows);

    let out_html = wrk.path("dash.html").to_string_lossy().to_string();
    let mut cmd = wrk.command("viz");
    cmd.env("QSV_VIZ_NO_COMPRESS", "1");
    cmd.args(["smart", "s.csv", "-o", &out_html]);
    wrk.assert_success(&mut cmd);
    let html = wrk.read_to_string("dash.html").unwrap();

    assert!(
        !html.contains("over time"),
        "without a categorical entity there is no bubble panel; html: {html}"
    );
}

#[test]
fn viz_smart_bubble_gated_when_cells_too_sparse() {
    // 5 entities x 6 quarters, but each entity has >= 3 rows in only 3 of the 6 quarters (1 row in
    // the rest). Cells clear min_cell_rows in too few buckets, so the panel-completeness gate
    // rejects it — a half-empty, flickering bubble panel is noise, not a Gapminder story. (This is
    // the gate that rejects delivery_stops, whose many daily dates coarsen to monthly buckets with
    // only a fraction dense per zone.)
    let wrk = Workdir::new("viz_smart_bubble_gated_when_cells_too_sparse");
    let ents = ["r1", "r2", "r3", "r4", "r5"];
    let q = [
        "2023-01-01",
        "2023-04-01",
        "2023-07-01",
        "2023-10-01",
        "2024-01-01",
        "2024-04-01",
    ];
    let mut rows = String::from("region,quarter_date,gdp_index,wellbeing_index\n");
    for (ei, e) in ents.iter().enumerate() {
        for (qi, qd) in q.iter().enumerate() {
            let n = if qi < 3 { 3 } else { 1 }; // dense in only the first 3 quarters
            for k in 0..n {
                let g = 50 + ei * 5 + qi * 4 + k;
                let w = 45 + ei * 4 + qi * 5 + k;
                rows.push_str(&format!("{e},{qd},{g},{w}\n"));
            }
        }
    }
    wrk.create_from_string("s.csv", &rows);

    let out_html = wrk.path("dash.html").to_string_lossy().to_string();
    let mut cmd = wrk.command("viz");
    cmd.env("QSV_VIZ_NO_COMPRESS", "1");
    cmd.args(["smart", "s.csv", "-o", &out_html]);
    wrk.assert_success(&mut cmd);
    let html = wrk.read_to_string("dash.html").unwrap();

    assert!(
        !html.contains(" by region over time"),
        "a sparse/incomplete panel should not animate a bubble; html: {html}"
    );
}

#[test]
fn viz_smart_bubble_wins_arbitration_over_scatter_pair() {
    // arbitration (§5): a dataset qualifying for BOTH the T1 scatter-pair animation (the a/b
    // centroid path ARCS) AND the T3 bubble (a complete 3-entity panel) shows ONLY the T3 bubble.
    let wrk = Workdir::new("viz_smart_bubble_wins_arbitration_over_scatter_pair");
    let cents = [(0, 2), (3, 10), (6, 13), (9, 10), (12, 2)];
    let ents = ["north", "south", "east"];
    let mut rows = String::from("date,region,a,b\n");
    for (m, (cx, cy)) in cents.iter().enumerate() {
        for e in &ents {
            for k in -1..=1 {
                rows.push_str(&format!("2024-0{}-01,{e},{},{}\n", m + 1, cx + k, cy + k));
            }
        }
    }
    wrk.create_from_string("s.csv", &rows);

    let out_html = wrk.path("dash.html").to_string_lossy().to_string();
    let mut cmd = wrk.command("viz");
    cmd.env("QSV_VIZ_NO_COMPRESS", "1");
    cmd.args(["smart", "s.csv", "-o", &out_html]);
    wrk.assert_success(&mut cmd);
    let html = wrk.read_to_string("dash.html").unwrap();

    // the bubble is present ...
    assert!(
        html.contains(" by region over time"),
        "expected the T3 bubble to win arbitration; html: {html}"
    );
    // ... and it is the ONLY animated panel (each animated panel contributes exactly one slider)
    assert_eq!(
        html.matches(r#""sliders":["#).count(),
        1,
        "exactly one animated panel (the bubble) should appear; html: {html}"
    );
}

#[test]
fn viz_scatter3d_hover_labels_columns() {
    // plotly's default 3D hover labels the coordinates with the bare letters x/y/z; we override
    // it with a template that names the real columns and comma-groups the values.
    let wrk = Workdir::new("viz_scatter3d_hover_labels_columns");
    wrk.create_from_string("cube.csv", "a,b,c\n1000,2,3\n4,5000,6\n7,8,9000\n");

    let mut cmd = wrk.command("viz");
    cmd.args(["scatter3d", "cube.csv", "--x", "a", "--y", "b", "--z", "c"]);
    let out = wrk.output(&mut cmd);
    assert!(out.status.success());
    let html = String::from_utf8_lossy(&out.stdout);
    assert!(
        html.contains("a: %{x:,.3f}") && html.contains("c: %{z:,.3f}"),
        "3D hover should name x/y/z columns with thousands-grouped values; html: {html}"
    );
}

#[test]
fn viz_radar_hover_shows_axis_means() {
    // Without a hover template plotly shows only "trace 0"; we attach per-vertex hovertext naming
    // each axis with its ACTUAL (comma-grouped) mean.
    let wrk = Workdir::new("viz_radar_hover_shows_axis_means");
    wrk.create_from_string(
        "ratings.csv",
        "brand,speed,power\nx,1000,2\nx,2000,4\ny,10,20\n",
    );

    let mut cmd = wrk.command("viz");
    cmd.args([
        "radar",
        "ratings.csv",
        "--cols",
        "speed,power",
        "--series",
        "brand",
    ]);
    let out = wrk.output(&mut cmd);
    assert!(out.status.success());
    let html = String::from_utf8_lossy(&out.stdout);
    assert!(
        html.contains(r#""hovertext":["#),
        "radar should set per-vertex hovertext; html: {html}"
    );
    // lines+markers so each axis intersection is its own hoverable vertex, not just the ring
    assert!(
        html.contains(r#""mode":"lines+markers""#),
        "radar should render markers at each axis vertex; html: {html}"
    );
    // brand x: mean speed = (1000 + 2000) / 2 = 1500
    assert!(
        html.contains("speed: 1,500"),
        "radar hover should show the axis mean with thousands separators; html: {html}"
    );
}

#[test]
fn viz_scatter_bubble_hover_surfaces_size_and_color() {
    // A bubble/color scatter encodes extra dimensions onto marker size/color; plotly's default
    // hover shows only (x, y), so we pre-render hovertext that also names the size/color values.
    let wrk = Workdir::new("viz_scatter_bubble_hover_surfaces_size_and_color");
    wrk.create_from_string("sales.csv", "x,y,sz,col\n5,10000,3,0.5\n6,20000,4,0.6\n");

    let mut cmd = wrk.command("viz");
    cmd.args([
        "scatter",
        "sales.csv",
        "--x",
        "x",
        "--y",
        "y",
        "--size",
        "sz",
        "--color",
        "col",
    ]);
    let out = wrk.output(&mut cmd);
    assert!(out.status.success());
    let html = String::from_utf8_lossy(&out.stdout);
    assert!(
        html.contains(r#""hovertext":["#),
        "bubble scatter should set a hovertext array; html: {html}"
    );
    assert!(
        html.contains("y: 10,000"),
        "hover should comma-group the y value; html: {html}"
    );
    assert!(
        html.contains("sz: 3") && html.contains("col: 0.5"),
        "hover should surface the size and color dimensions; html: {html}"
    );
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
fn viz_box_y_range_applied() {
    let wrk = Workdir::new("viz_box_y_range_applied");
    fruits(&wrk);

    let mut cmd = wrk.command("viz");
    cmd.args([
        "box",
        "fruits.csv",
        "--y",
        "Price",
        "--x",
        "Fruit",
        "--y-range=-10:55",
    ]);
    let out = wrk.output(&mut cmd);
    assert!(out.status.success());
    let html = String::from_utf8_lossy(&out.stdout);
    // an explicit --y-range fixes the y-axis (autorange off), so out-of-range points clip
    assert!(
        html.contains(r#""range":[-10.0,55.0]"#),
        "expected the fixed y-axis range in the layout; html: {html}"
    );
}

#[test]
fn viz_box_annotation_present() {
    let wrk = Workdir::new("viz_box_annotation_present");
    fruits(&wrk);

    let mut cmd = wrk.command("viz");
    cmd.args([
        "box",
        "fruits.csv",
        "--y",
        "Price",
        "--x",
        "Fruit",
        "--annotation",
        "1 pt clipped below -10",
    ]);
    let out = wrk.output(&mut cmd);
    assert!(out.status.success());
    let html = String::from_utf8_lossy(&out.stdout);
    assert!(
        html.contains("1 pt clipped below -10"),
        "expected the --annotation note in the chart; html: {html}"
    );
}

#[test]
fn viz_box_y_range_invalid_errors() {
    let wrk = Workdir::new("viz_box_y_range_invalid_errors");
    fruits(&wrk);

    // min >= max is rejected
    let mut cmd = wrk.command("viz");
    cmd.args(["box", "fruits.csv", "--y", "Price", "--y-range=5:1"]);
    let out = wrk.output(&mut cmd);
    assert!(!out.status.success());
    let stderr = wrk.output_stderr(&mut cmd);
    assert!(stderr.contains("--y-range"));

    // non-numeric is rejected
    let mut cmd_2 = wrk.command("viz");
    cmd_2.args(["box", "fruits.csv", "--y", "Price", "--y-range=abc"]);
    let out2 = wrk.output(&mut cmd_2);
    assert!(!out2.status.success());
    let stderr2 = wrk.output_stderr(&mut cmd_2);
    assert!(stderr2.contains("--y-range"));
}

#[test]
fn viz_violin_basic() {
    let wrk = Workdir::new("viz_violin_basic");
    fruits(&wrk);

    let mut cmd = wrk.command("viz");
    cmd.args(["violin", "fruits.csv", "--y", "Price"]);
    let out = wrk.output(&mut cmd);
    assert!(out.status.success());
    let html = String::from_utf8_lossy(&out.stdout);
    assert!(html.contains(r#""type":"violin""#));
    // a violin reads the raw values (its KDE can't come from precomputed quartiles) and
    // draws the quartile box + mean line inside the density silhouette by default, with
    // the same default points overlay as `viz box`
    assert!(html.contains(r#""quartilemethod":"linear""#));
    assert!(html.contains(r#""box":{"visible":true}"#));
    assert!(html.contains(r#""meanline":{"visible":true}"#));
    assert!(html.contains(r#""points":"outliers""#));
}

#[test]
fn viz_violin_grouped() {
    let wrk = Workdir::new("viz_violin_grouped");
    fruits(&wrk);

    let mut cmd = wrk.command("viz");
    cmd.args(["violin", "fruits.csv", "--y", "Price", "--x", "Fruit"]);
    let out = wrk.output(&mut cmd);
    assert!(out.status.success());
    let html = String::from_utf8_lossy(&out.stdout);
    assert!(html.contains(r#""type":"violin""#));
    // grouped: one violin per --x category, so the trace carries a category x array
    assert!(html.contains(r#""x":["#));
}

#[test]
fn viz_violin_points_all() {
    let wrk = Workdir::new("viz_violin_points_all");
    fruits(&wrk);

    // violins share the --box-points flag (and modes) with boxes; plotly serializes the
    // violin overlay as "points" rather than "boxpoints"
    let mut cmd = wrk.command("viz");
    cmd.args([
        "violin",
        "fruits.csv",
        "--y",
        "Price",
        "--box-points",
        "all",
    ]);
    let out = wrk.output(&mut cmd);
    assert!(out.status.success());
    let html = String::from_utf8_lossy(&out.stdout);
    assert!(html.contains(r#""points":"all""#));
}

#[test]
fn viz_violin_no_numeric_errors() {
    let wrk = Workdir::new("viz_violin_no_numeric_errors");
    fruits(&wrk);

    let mut cmd = wrk.command("viz");
    cmd.args(["violin", "fruits.csv", "--y", "Fruit"]);
    let out = wrk.output(&mut cmd);
    assert!(!out.status.success());
    let stderr = wrk.output_stderr(&mut cmd);
    assert!(stderr.contains("violin plot"));
}

/// A continuous numeric column whose two clusters sit in the bimodality ambiguity band:
/// Sarle's BC at or above the uniform benchmark (5/9) but below the histogram threshold
/// (0.60), platykurtic, 40 distinct values over 200 rows. Cluster gaps of 5.1-5.8 land in
/// the band with this shape (verified empirically); 5.5 sits mid-band, with smaller gaps
/// classifying as a plain box and larger ones as a histogram.
fn ambiguous_band_column(wrk: &Workdir) {
    let mut rows = String::from("val\n");
    for center in [50.0_f64, 55.5] {
        for i in 0..20 {
            let v = format!("{:.2}\n", center + f64::from(i) * 0.25);
            for _ in 0..5 {
                rows.push_str(&v);
            }
        }
    }
    wrk.create_from_string("band.csv", &rows);
}

#[test]
fn viz_smart_violin_auto_default() {
    let wrk = Workdir::new("viz_smart_violin_auto_default");
    ambiguous_band_column(&wrk);

    // default --violin auto: a continuous column renders as a violin (the default
    // distribution panel), not a box — and this mildly-two-peaked column stays below the
    // histogram threshold, so not a histogram either
    let out_html = wrk.path("dash.html").to_string_lossy().to_string();
    let mut cmd = wrk.command("viz");
    cmd.args(["smart", "band.csv", "-o", &out_html]);
    wrk.assert_success(&mut cmd);
    let html = wrk.read_to_string("dash.html").unwrap();
    assert!(html.contains(r#""type":"violin""#));
    assert!(!html.contains(r#""type":"box""#));
    assert!(!html.contains(r#""type":"histogram""#));
    // the violin panel carries the same raw-values contract as a raw box: inner quartile
    // box + mean line inside the KDE silhouette
    assert!(html.contains(r#""box":{"visible":true}"#));
    assert!(html.contains(r#""meanline":{"visible":true}"#));
    // 200 rows would earn a box the size-based `all` upgrade, but the KDE silhouette
    // already shows the distribution — a smart violin overlays only the outliers
    assert!(html.contains(r#""points":"outliers""#));
    assert!(!html.contains(r#""points":"all""#));
}

#[test]
fn viz_smart_violin_explicit_all_points() {
    let wrk = Workdir::new("viz_smart_violin_explicit_all_points");
    ambiguous_band_column(&wrk);

    // an explicit --box-points mode overrides the violin's outliers-only default
    let out_html = wrk.path("dash.html").to_string_lossy().to_string();
    let mut cmd = wrk.command("viz");
    cmd.args(["smart", "band.csv", "--box-points", "all", "-o", &out_html]);
    wrk.assert_success(&mut cmd);
    let html = wrk.read_to_string("dash.html").unwrap();
    assert!(html.contains(r#""type":"violin""#));
    assert!(html.contains(r#""points":"all""#));
}

#[test]
fn viz_smart_grouped_violin_panel() {
    let wrk = Workdir::new("viz_smart_grouped_violin_panel");
    // one low-cardinality categorical dimension + one numeric measure: `viz smart` adds a
    // grouped-violin overview panel showing the measure's distribution split by category (one
    // violin per category), distinct from any single-column violin. 180 rows => 60 per category,
    // clearing the VIOLIN_MIN_POINTS (50) per-category gate so every category earns a violin.
    let mut rows = String::from("category,amount\n");
    for i in 0..180_u32 {
        let category = match i % 3 {
            0 => "alpha",
            1 => "beta",
            _ => "gamma",
        };
        // per-category offsets so the distributions differ
        let amount = (i % 30) + (i % 3) * 40;
        rows.push_str(&format!("{category},{amount}\n"));
    }
    wrk.create_from_string("grp.csv", &rows);

    let out_html = wrk.path("dash.html").to_string_lossy().to_string();
    let mut cmd = wrk.command("viz");
    cmd.args(["smart", "grp.csv", "-o", &out_html]);
    wrk.assert_success(&mut cmd);
    let html = wrk.read_to_string("dash.html").unwrap();

    // the grouped-violin panel: title "<measure> distribution by <dimension>" and a violin trace
    assert!(html.contains("amount distribution by category"));
    assert!(html.contains(r#""type":"violin""#));
    // grouped => the violin carries an x-array of the category labels (a single-column violin has
    // no x data); each category's label is present as violin x data
    assert!(html.contains(r#""alpha","alpha""#));
    // inner quartile box + mean line inside each KDE silhouette
    assert!(html.contains(r#""box":{"visible":true}"#));
    assert!(html.contains(r#""meanline":{"visible":true}"#));
}

#[test]
fn viz_smart_grouped_violin_alternating_categories_above_sample_cap() {
    let wrk = Workdir::new("viz_smart_grouped_violin_alternating_categories_above_sample_cap");
    // Regression: two categories STRICTLY alternating by row, with a non-null measure count
    // (100_000) that puts the collection stride at exactly 2 (> MAX_SMART_POINTS = 50_000). A
    // global `seen % stride` sampler would keep only the even-position category ("east"), starve
    // "west", and drop the panel to a single violin (or skip it). The per-category stride must
    // sample both, so both categories survive in the grouped violin.
    let mut rows = String::from("grp,val\n");
    for i in 0..100_000_u32 {
        let grp = if i % 2 == 0 { "east" } else { "west" };
        let val = i % 1000;
        rows.push_str(&format!("{grp},{val}\n"));
    }
    wrk.create_from_string("alt.csv", &rows);

    let out_html = wrk.path("dash.html").to_string_lossy().to_string();
    let mut cmd = wrk.command("viz");
    cmd.args(["smart", "alt.csv", "-o", &out_html]);
    wrk.assert_success(&mut cmd);
    let html = wrk.read_to_string("dash.html").unwrap();

    // the grouped-violin panel is present (not collapsed to <2 categories and skipped) ...
    assert!(html.contains("val distribution by grp"));
    // ... and BOTH alternating categories appear as violin x data (the aliasing bug would have
    // sampled only one of them)
    assert!(html.contains(r#""east""#));
    assert!(html.contains(r#""west""#));
}

#[test]
fn viz_smart_grouped_violin_gated_below_min_points() {
    let wrk = Workdir::new("viz_smart_grouped_violin_gated_below_min_points");
    // two categories, each with only 30 rows (< VIOLIN_MIN_POINTS = 50). Under default
    // `--violin auto` a KDE over so few observations is bandwidth artifact, so the per-category
    // sample-size gate drops both, leaving <2 eligible categories and no grouped-violin panel.
    let mut rows = String::from("grp,val\n");
    for i in 0..60_u32 {
        let grp = if i % 2 == 0 { "east" } else { "west" };
        rows.push_str(&format!("{grp},{}\n", i % 25));
    }
    wrk.create_from_string("sparse.csv", &rows);

    let out_html = wrk.path("dash.html").to_string_lossy().to_string();
    let mut cmd = wrk.command("viz");
    cmd.args(["smart", "sparse.csv", "-o", &out_html]);
    wrk.assert_success(&mut cmd);
    let html = wrk.read_to_string("dash.html").unwrap();
    // no grouped-violin overview panel (its title is "<measure> distribution by <dimension>")
    assert!(
        !html.contains("distribution by"),
        "sparse per-category counts should suppress the grouped violin; html: {html}"
    );
}

#[test]
fn viz_smart_grouped_violin_off_suppressed() {
    let wrk = Workdir::new("viz_smart_grouped_violin_off_suppressed");
    // well-populated categories (90 rows each, clearing the sample-size gate) that WOULD earn a
    // grouped violin under auto; `--violin off` must suppress it (there is no grouped-box panel,
    // so the distribution-by-category overview simply doesn't appear).
    let mut rows = String::from("grp,val\n");
    for i in 0..180_u32 {
        let grp = if i % 2 == 0 { "east" } else { "west" };
        rows.push_str(&format!("{grp},{}\n", i % 50));
    }
    wrk.create_from_string("dense.csv", &rows);

    let out_html = wrk.path("dash.html").to_string_lossy().to_string();
    let mut cmd = wrk.command("viz");
    cmd.args(["smart", "dense.csv", "--violin", "off", "-o", &out_html]);
    wrk.assert_success(&mut cmd);
    let html = wrk.read_to_string("dash.html").unwrap();
    assert!(
        !html.contains("distribution by"),
        "--violin off should suppress the grouped-violin panel; html: {html}"
    );
    assert!(!html.contains(r#""type":"violin""#));
}

#[test]
fn viz_smart_grouped_violin_max_points_env_override() {
    let wrk = Workdir::new("viz_smart_grouped_violin_max_points_env_override");
    // 60k rows across two categories (30k each). At the default budget the violin sample target is
    // 30k (MAX_SMART_POINTS/5 = 150k/5), so each category is strided down (~15k kept); raising
    // QSV_VIZ_MAX_POINTS lifts the target above the row count, so all values are kept. The count of
    // a category label in the violin's x-array reflects how many of its points were embedded.
    let mut rows = String::from("grp,val\n");
    for i in 0..60_000_u32 {
        let grp = if i % 2 == 0 { "east" } else { "west" };
        rows.push_str(&format!("{grp},{}\n", i % 1000));
    }
    wrk.create_from_string("big.csv", &rows);

    let count_east = |env: Option<&str>| -> usize {
        let out = wrk.path("d.html").to_string_lossy().to_string();
        let mut cmd = wrk.command("viz");
        cmd.args(["smart", "big.csv", "-o", &out]);
        // don't inherit a QSV_VIZ_MAX_POINTS a developer/CI may already have set, so the default
        // branch actually exercises the default budget
        cmd.env_remove("QSV_VIZ_MAX_POINTS");
        if let Some(v) = env {
            cmd.env("QSV_VIZ_MAX_POINTS", v);
        }
        wrk.assert_success(&mut cmd);
        wrk.read_to_string("d.html")
            .unwrap()
            .matches(r#""east""#)
            .count()
    };

    // default: strided to ~15k per category; raised budget keeps all 30k
    let default_east = count_east(None);
    let raised_east = count_east(Some("1000000"));
    assert!(
        default_east < 20_000,
        "default budget should stride-sample (got {default_east} east points)"
    );
    assert!(
        raised_east > 20_000,
        "raised QSV_VIZ_MAX_POINTS should embed more points (got {raised_east} east points)"
    );
}

#[test]
fn viz_smart_box_all_points_gated_on_nonnull_count() {
    let wrk = Workdir::new("viz_smart_box_all_points_gated_on_nonnull_count");
    // 2,000 rows but only 800 non-null values (60% null): the all-points tier is measured
    // against the column's actual point count — only non-null values are collected and
    // rendered — so this column still earns the `all` overlay. The value shape is a
    // triangular sum of two cycles (unimodal, BC well below 5/9) so it stays a box, not a
    // violin or histogram.
    let mut rows = String::from("id,measure\n");
    for i in 1..=2000_u32 {
        let measure = if i % 5 < 3 {
            String::new()
        } else {
            format!("{}", (i % 20) + ((i * 3) % 23))
        };
        rows.push_str(&format!("{i},{measure}\n"));
    }
    wrk.create_from_string("sparse.csv", &rows);

    // --violin off pins the BOX overlay tiers (violins cap their overlay at outliers)
    let out_html = wrk.path("dash.html").to_string_lossy().to_string();
    let mut cmd = wrk.command("viz");
    cmd.args(["smart", "sparse.csv", "--violin", "off", "-o", &out_html]);
    wrk.assert_success(&mut cmd);
    let html = wrk.read_to_string("dash.html").unwrap();
    assert!(html.contains(r#""type":"box""#));
    assert!(html.contains(r#""boxpoints":"all""#));
}

#[test]
fn viz_smart_dense_grid_demotes_all_points_to_outliers() {
    let wrk = Workdir::new("viz_smart_dense_grid_demotes_all_points_to_outliers");
    // 10 small continuous columns (200 rows each — well under the all-points tier) whose
    // dashboard exceeds SMART_ALL_POINTS_MAX_PANELS: the size-based `all` overlay turns to
    // noise at postage-stamp cell size, so it's demoted to outliers-only. Triangular sums
    // keep every column unimodal (box, not violin/histogram).
    let mut rows = String::from("m0,m1,m2,m3,m4,m5,m6,m7,m8,m9\n");
    for i in 1..=200_u32 {
        let cells: Vec<String> = (0..10_u32)
            .map(|j| format!("{}", (i % 20) + ((i * 3) % 23) + j * 100))
            .collect();
        rows.push_str(&format!("{}\n", cells.join(",")));
    }
    wrk.create_from_string("wide.csv", &rows);

    // --violin off pins the BOX demotion path (violins never carry an `all` overlay
    // unless --box-points is explicit)
    let out_html = wrk.path("dash.html").to_string_lossy().to_string();
    let mut cmd = wrk.command("viz");
    cmd.args(["smart", "wide.csv", "--violin", "off", "-o", &out_html]);
    wrk.assert_success(&mut cmd);
    let html = wrk.read_to_string("dash.html").unwrap();
    assert!(html.contains(r#""type":"box""#));
    assert!(html.contains(r#""boxpoints":"outliers""#));
    assert!(!html.contains(r#""boxpoints":"all""#));

    // an explicit --box-points mode is the user's call and is never demoted
    let out_html2 = wrk.path("dash_all.html").to_string_lossy().to_string();
    let mut cmd_2 = wrk.command("viz");
    cmd_2.args([
        "smart",
        "wide.csv",
        "--violin",
        "off",
        "--box-points",
        "all",
        "-o",
        &out_html2,
    ]);
    wrk.assert_success(&mut cmd_2);
    let html2 = wrk.read_to_string("dash_all.html").unwrap();
    assert!(html2.contains(r#""boxpoints":"all""#));
}

#[test]
fn viz_smart_violin_sampled_above_exact_threshold() {
    let wrk = Workdir::new("viz_smart_violin_sampled_above_exact_threshold");
    // 12,000 non-null values against a pinned 10k violin budget (QSV_VIZ_MAX_POINTS=50000/5):
    // past the exact-data threshold, the violin is drawn from a deterministic stride sample —
    // no point overlay (a sample misses the true extremes) and a "(sampled)" title cue. The
    // budget is pinned so the fixture stays small under the larger default (150k/5 = 30k).
    // Triangular sum keeps it unimodal (violin, not histogram); the +1000 offset keeps the
    // max/min ratio low so `--log-scale auto` stays linear (a would-log panel correctly
    // declines the violin).
    let mut rows = String::from("id,measure\n");
    for i in 1..=12_000_u32 {
        rows.push_str(&format!("{i},{}\n", 1000 + (i % 200) + ((i * 3) % 231)));
    }
    wrk.create_from_string("big.csv", &rows);

    let out_html = wrk.path("dash.html").to_string_lossy().to_string();
    let mut cmd = wrk.command("viz");
    cmd.args(["smart", "big.csv", "-o", &out_html]);
    cmd.env("QSV_VIZ_MAX_POINTS", "50000");
    wrk.assert_success(&mut cmd);
    let html = wrk.read_to_string("dash.html").unwrap();
    assert!(html.contains(r#""type":"violin""#));
    assert!(html.contains("(sampled)"));
    // sampled violins carry no point overlay ("points":false), never "outliers"/"all"
    assert!(html.contains(r#""points":false"#));
    assert!(!html.contains(r#""points":"outliers""#));
    assert!(!html.contains(r#""type":"box""#));
}

#[test]
fn viz_smart_violin_off_keeps_box() {
    let wrk = Workdir::new("viz_smart_violin_off_keeps_box");
    ambiguous_band_column(&wrk);

    // --violin off: even an ambiguity-band column stays a box panel
    let out_html = wrk.path("dash.html").to_string_lossy().to_string();
    let mut cmd = wrk.command("viz");
    cmd.args(["smart", "band.csv", "--violin", "off", "-o", &out_html]);
    wrk.assert_success(&mut cmd);
    let html = wrk.read_to_string("dash.html").unwrap();
    assert!(html.contains(r#""type":"box""#));
    assert!(!html.contains(r#""type":"violin""#));
}

#[test]
fn viz_smart_violin_on_forces_violins() {
    // also pins the docopt coexistence of the `violin` subcommand with the `--violin` flag
    let wrk = Workdir::new("viz_smart_violin_on_forces_violins");
    // a bland continuous column (uniform cycle, no bimodality signal) that --violin auto
    // would keep as a box
    let mut rows = String::from("id,age\n");
    for i in 1..=100 {
        rows.push_str(&format!("{i},{}\n", 20 + i % 50));
    }
    wrk.create_from_string("people.csv", &rows);

    let out_html = wrk.path("dash.html").to_string_lossy().to_string();
    let mut cmd = wrk.command("viz");
    cmd.args(["smart", "people.csv", "--violin", "on", "-o", &out_html]);
    wrk.assert_success(&mut cmd);
    let html = wrk.read_to_string("dash.html").unwrap();
    assert!(html.contains(r#""type":"violin""#));
    assert!(!html.contains(r#""type":"box""#));
    // 100 rows would earn a box the size-based `all` upgrade; a violin defaults to
    // outliers-only (the KDE already shows the shape)
    assert!(html.contains(r#""points":"outliers""#));
}

#[test]
fn viz_smart_violin_box_points_none_still_violin() {
    let wrk = Workdir::new("viz_smart_violin_box_points_none_still_violin");
    ambiguous_band_column(&wrk);

    // `--box-points none` only suppresses the point overlay — the violin (which needs the
    // raw values for its KDE regardless of points) still renders; the cache-only escape
    // hatch is `--violin off --box-points none`
    let out_html = wrk.path("dash.html").to_string_lossy().to_string();
    let mut cmd = wrk.command("viz");
    cmd.args(["smart", "band.csv", "--box-points", "none", "-o", &out_html]);
    wrk.assert_success(&mut cmd);
    let html = wrk.read_to_string("dash.html").unwrap();
    assert!(html.contains(r#""type":"violin""#));
    assert!(html.contains(r#""points":false"#));
    assert!(!html.contains(r#""type":"box""#));
    // exact data (200 rows), so no sampling cue
    assert!(!html.contains("(sampled)"));
}

#[test]
fn viz_smart_violin_invalid_errors() {
    let wrk = Workdir::new("viz_smart_violin_invalid_errors");
    fruits(&wrk);

    let mut cmd = wrk.command("viz");
    cmd.args(["smart", "fruits.csv", "--violin", "bogus"]);
    let out = wrk.output(&mut cmd);
    assert!(!out.status.success());
    let stderr = wrk.output_stderr(&mut cmd);
    assert!(stderr.contains("Unknown --violin"));
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
    // title annotation above each panel, with at least one violin (continuous — the default
    // distribution panel) and one bar (categorical)
    assert!(html.contains(r#""height":"#));
    assert!(html.contains(r#""annotations":["#));
    assert!(html.contains(r#""xaxis2":{"#));
    assert!(html.contains(r#""domain":["#));
    assert!(html.contains(r#""type":"violin""#));
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
    // the histogram panel labels its binned value + count in the hover (comma-grouped), since the
    // dashboard cell has no axis titles
    assert!(
        html.contains("count: %{y:,}"),
        "histogram panel hover should label the comma-grouped count; html: {html}"
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
        !html.contains(r#""type":"box""#) && !html.contains(r#""type":"violin""#),
        "the bimodal column should NOT be a box or violin; html: {html}"
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
        // each column cycles on its OWN modulus, so the 10 columns have 10 DISTINCT
        // cardinalities (2..=11). `(r + c) % 4` gave every column the same 4 values in a
        // rotated order — a bijection between every pair — which the 1:1 collapse
        // (issue #4221) correctly folds down to a single panel.
        let cells: Vec<String> = (0..10).map(|c| format!("v{}", r % (c + 2))).collect();
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
    // ...all sharing the one plotly.js runtime `smart_html_page` puts in <head>
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
        // each column cycles on its OWN modulus, so the 10 columns have 10 DISTINCT
        // cardinalities (2..=11). `(r + c) % 4` gave every column the same 4 values in a
        // rotated order — a bijection between every pair — which the 1:1 collapse
        // (issue #4221) correctly folds down to a single panel.
        let cells: Vec<String> = (0..10).map(|c| format!("v{}", r % (c + 2))).collect();
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

// On image export the MapLibre tile map can't be rendered, so a local-extent coordinate pair is
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

// Premise-pin for the two `viz_static` >8-panel tests below (issue #4343).
//
// Those tests are `#[cfg(feature = "viz_static")]` AND `#[ignore]`d, so they neither compile nor
// run in normal CI. When the 1:1 twin-collapse (issue #4221) landed it silently invalidated their
// fixture — all 12 columns folded into one panel — and nothing noticed for a month.
//
// This test shares their fixture but needs no webdriver: it asserts only that the 12 columns
// really do survive panel selection. If it fails, fix the fixture in all three places.
#[test]
fn viz_smart_twelve_distinct_cardinality_columns_all_charted() {
    let wrk = Workdir::new("viz_smart_twelve_distinct_cardinality_columns_all_charted");
    // each column cycles on its OWN modulus => 12 DISTINCT cardinalities (3..=14), so no two
    // columns are 1:1 twins
    let headers: Vec<String> = (1..=12).map(|i| format!("cat{i:02}")).collect();
    let mut rows = format!("{}\n", headers.join(","));
    for i in 0..90 {
        let cells: Vec<String> = (1..=12).map(|c| format!("v{}", i % (c + 2))).collect();
        rows.push_str(&format!("{}\n", cells.join(",")));
    }
    wrk.create_from_string("wide.csv", &rows);

    let out_html = wrk.path("dash.html").to_string_lossy().to_string();
    let mut cmd = wrk.command("viz");
    cmd.args(["smart", "wide.csv", "-o", &out_html]);
    let stderr = wrk.output_stderr(&mut cmd);
    wrk.assert_success(&mut cmd);

    assert!(
        !stderr.contains("they are 1:1"),
        "no two columns should collapse as 1:1 twins: {stderr}"
    );
    assert!(
        !stderr.contains("skipped"),
        "every column should be charted, none skipped: {stderr}"
    );

    // The two stderr assertions above are the whole test. Deliberately NOT asserted here:
    //   - panel COUNT: these columns are deterministic functions of the row index, so divisible
    //     pairs (i%3 vs i%6) are perfectly associated and the HTML path adds a parcats overview
    //     panel on top of the 12 frequency bars. (The image path used by the viz_static tests
    //     suppresses composites via `!is_image()`, which is why it gets exactly 12.)
    //   - column names in the HTML: the data-viewer drawer embeds EVERY CSV column whether or not
    //     it was charted, so `html.contains("catNN")` holds even for a collapsed column. That looks
    //     like protection but cannot fail — the exact trap this test exists to guard against.
}

// Static image export of >8 panels: plotly's typed Layout only has 8 axis fields, so the grid is
// assembled as raw JSON with domain-positioned xaxis9+ and rendered via StaticExporter::write_fig.
// Requires a browser/webdriver, so ignored by default.
#[cfg(feature = "viz_static")]
#[test]
#[ignore = "requires a browser/webdriver for plotly static export"]
fn viz_static_more_than_eight_panels() {
    let wrk = Workdir::new("viz_static_more_than_eight_panels");
    // 12 low-cardinality categorical columns => 12 frequency-bar panels (well past the 8 cap).
    // Each column cycles on its OWN modulus, so the 12 columns have 12 DISTINCT cardinalities
    // (3..=14). `(i + c) % 4` gave every column the same 4 values in a rotated order — a bijection
    // between every pair — which the 1:1 collapse (issue #4221) correctly folds down to a single
    // panel, silently invalidating this test's premise (issue #4343).
    let headers: Vec<String> = (1..=12).map(|i| format!("cat{i:02}")).collect();
    let mut rows = format!("{}\n", headers.join(","));
    for i in 0..90 {
        let cells: Vec<String> = (1..=12).map(|c| format!("v{}", i % (c + 2))).collect();
        rows.push_str(&format!("{}\n", cells.join(",")));
    }
    wrk.create_from_string("wide.csv", &rows);

    let out_svg = wrk.path("dash.svg").to_string_lossy().to_string();
    let mut cmd = wrk.command("viz");
    cmd.args(["smart", "wide.csv", "-o", &out_svg]);
    let stderr = wrk.output_stderr(&mut cmd);
    wrk.assert_success(&mut cmd);

    // Load-bearing premise check (issue #4343): all 12 columns must actually be charted. stderr is
    // emitted during panel selection, before any rendering, so it is unaffected by how plotly
    // rasterizes text. If this fires, the FIXTURE is wrong — not the >8-panel render path.
    assert!(
        !stderr.contains("they are 1:1"),
        "fixture regression: columns collapsed as 1:1 twins, so there are not >8 panels to \
         render. Give each column its own modulus/cardinality. stderr: {stderr}"
    );
    assert!(
        !stderr.contains("skipped"),
        "fixture regression: some columns were dropped before rendering, so the >8-panel path may \
         not be exercised. stderr: {stderr}"
    );

    let svg = wrk.read_to_string("dash.svg").unwrap();
    assert!(svg.contains("<svg") || svg.contains("<?xml"));
    // All 12 panel titles must be present. >8 titles is itself the proof that the raw-JSON
    // xaxis9+ grid ran, since the typed Layout tops out at 8 axis fields.
    for panel in [
        "cat01", "cat02", "cat03", "cat04", "cat05", "cat06", "cat07", "cat08", "cat09", "cat10",
        "cat11", "cat12",
    ] {
        assert!(
            svg.contains(panel),
            "panel {panel} is missing from the rendered image. NOTE: if EVERY title is missing \
             while the stderr checks above passed, plotly is converting title text to glyph paths \
             rather than emitting literal <text> — that is a text-rendering limitation, not a \
             fixture failure; assert on stderr instead."
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
    // Same 12-distinct-cardinality fixture as viz_static_more_than_eight_panels — see the comment
    // there for why a shared modulus (`(i + c) % 4`) cannot be used (issues #4221 / #4343).
    let headers: Vec<String> = (1..=12).map(|i| format!("cat{i:02}")).collect();
    let mut rows = format!("{}\n", headers.join(","));
    for i in 0..90 {
        let cells: Vec<String> = (1..=12).map(|c| format!("v{}", i % (c + 2))).collect();
        rows.push_str(&format!("{}\n", cells.join(",")));
    }
    wrk.create_from_string("wide.csv", &rows);

    let out_svg = wrk.path("dash.svg").to_string_lossy().to_string();
    let mut cmd = wrk.command("viz");
    cmd.args(["smart", "wide.csv", "--max-charts", "4", "-o", &out_svg]);
    let stderr = wrk.output_stderr(&mut cmd);
    wrk.assert_success(&mut cmd);

    // Load-bearing assertion: the cap is applied during panel selection, so stderr proves it ran.
    // (The previous fixture collapsed to a single panel, which is BELOW the cap — the cap was
    // never exercised and this test passed vacuously. See issue #4343.)
    assert!(
        stderr.contains("charting 4 column(s)"),
        "--max-charts 4 should trim the 12 eligible panels down to 4: {stderr}"
    );
    // The trim keeps the HIGHEST-interest panels; `panel_interest` rewards cardinality, so the
    // low-cardinality cat01 is dropped and the high-cardinality cat12 survives.
    assert!(
        stderr.contains("skipped 8:") && stderr.contains("cat01"),
        "--max-charts 4 should skip the 8 lowest-interest panels, incl. cat01: {stderr}"
    );

    let svg = wrk.read_to_string("dash.svg").unwrap();
    assert!(
        svg.contains("cat12"),
        "cat12 has the highest cardinality, so it should survive the --max-charts 4 trim"
    );
    assert!(
        !svg.contains("cat01"),
        "--max-charts 4 should cap panels; cat01 (lowest interest) must not be drawn"
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
fn viz_heatmap_correlation_excludes_identifier() {
    // A standalone correlation heatmap in auto mode (no --cols) drops near-unique identifier
    // columns — a monotonic order key holds a distinct value in nearly every row and has no
    // meaningful linear relationship — mirroring `viz smart`. The two repeated-value measures
    // remain.
    let wrk = Workdir::new("viz_heatmap_correlation_excludes_identifier");
    let mut rows = String::from("order_id,units,revenue\n");
    for i in 0..60 {
        // order_id: monotonic unique key; units/revenue: low-cardinality, repeated, correlated.
        let u = i % 6;
        rows.push_str(&format!("{},{u},{}\n", 1000 + i, u * 10));
    }
    wrk.create_from_string("s.csv", &rows);

    let mut cmd = wrk.command("viz");
    cmd.args(["heatmap", "s.csv"]);
    let out = wrk.output(&mut cmd);
    assert!(out.status.success());
    let html = String::from_utf8_lossy(&out.stdout);
    assert!(
        !html.contains("order_id"),
        "near-unique identifier column should be excluded from the auto correlation heatmap"
    );
    assert!(html.contains("units") && html.contains("revenue"));

    // an explicit --cols is the user's deliberate selection and is honored as-is (order_id kept).
    let mut cmd = wrk.command("viz");
    cmd.args(["heatmap", "s.csv", "--cols", "order_id,units,revenue"]);
    let out = wrk.output(&mut cmd);
    assert!(out.status.success());
    let html = String::from_utf8_lossy(&out.stdout);
    assert!(
        html.contains("order_id"),
        "explicit --cols must keep the identifier column"
    );
}

#[test]
fn viz_heatmap_correlation_sparse_identifier_keeps_measure_rows() {
    // A sparse identifier (order_id present in only 1 of 60 rows) must be dropped from the kept
    // set BEFORE the listwise row-drop, so its blank rows don't starve the fully-populated
    // measures. With the old post-transpose ordering, order_id was kept through the listwise
    // pass, collapsing the matrix to the single complete row and failing the "at least 2 rows"
    // check. Now the heatmap succeeds on the two dense measures.
    let wrk = Workdir::new("viz_heatmap_correlation_sparse_identifier");
    let mut rows = String::from("order_id,units,revenue\n");
    for i in 0..60 {
        let u = i % 6;
        // order_id is populated only in the first row; units/revenue are dense in every row.
        let id = if i == 0 {
            "1000".to_string()
        } else {
            String::new()
        };
        rows.push_str(&format!("{id},{u},{}\n", u * 10));
    }
    wrk.create_from_string("s.csv", &rows);

    let mut cmd = wrk.command("viz");
    cmd.args(["heatmap", "s.csv"]);
    let out = wrk.output(&mut cmd);
    assert!(
        out.status.success(),
        "a sparse identifier must not starve the dense measures of their rows"
    );
    let html = String::from_utf8_lossy(&out.stdout);
    assert!(!html.contains("order_id"));
    assert!(html.contains("units") && html.contains("revenue"));
}

#[test]
fn viz_heatmap_correlation_excludes_large_int_identifier() {
    // Near-unique detection measures distinctness on raw cell bytes, not the parsed f64. A column
    // of distinct 17-digit ids beyond f64's 53-bit mantissa collapses to far fewer distinct floats
    // (consecutive integers near 1e16 share a float), which would let the identifier evade an
    // f64-based distinct-ratio and pollute the matrix. On the raw bytes every id is distinct, so it
    // is correctly dropped.
    let wrk = Workdir::new("viz_heatmap_correlation_large_int_identifier");
    let mut rows = String::from("big_id,units,revenue\n");
    for i in 0..60 {
        let u = i % 6;
        // 10_000_000_000_000_000 + 2*i keeps every id distinct as text but pairs of them round to
        // the same f64 at this magnitude, halving the f64-distinct count.
        let big_id = 10_000_000_000_000_000_u64 + 2 * i as u64;
        rows.push_str(&format!("{big_id},{u},{}\n", u * 10));
    }
    wrk.create_from_string("s.csv", &rows);

    let mut cmd = wrk.command("viz");
    cmd.args(["heatmap", "s.csv"]);
    let out = wrk.output(&mut cmd);
    assert!(out.status.success());
    let html = String::from_utf8_lossy(&out.stdout);
    assert!(
        !html.contains("big_id"),
        "a large-integer identifier must be detected via raw bytes and excluded"
    );
    assert!(html.contains("units") && html.contains("revenue"));
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
    // links are always tinted from their SOURCE node's PALETTE color (both East links share
    // East's color); nodes carry the PALETTE too.
    assert!(
        html.contains("rgba("),
        "link ribbons should be source-tinted; html: {html}"
    );
    // snap (plotly's crossing-min layout) is the DEFAULT: arrangement snap, and NO explicit
    // per-node x/y positions baked onto the node object (which would serialize as
    // `…,"thickness":20,"x":[…],"y":[…]`). The value-order positions still ride in the toggle
    // button's restyle args, hence the precise node-scoped discriminator.
    assert!(html.contains(r#""arrangement":"snap""#), "html: {html}");
    assert!(
        !html.contains(r#""thickness":20,"x":["#),
        "default (snap) should not bake per-node x positions on the node; html: {html}"
    );
    // an on-screen "node order" toggle (updatemenus button) is baked in either way.
    assert!(
        html.contains(r#""updatemenus""#) && html.contains("node order"),
        "the runtime node-order toggle button should be present; html: {html}"
    );
}

#[test]
fn viz_sankey_value_order_opts_into_flow_ranking() {
    let wrk = Workdir::new("viz_sankey_value_order");
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
        "--sankey-value-order",
    ]);
    let out = wrk.output(&mut cmd);
    assert!(out.status.success());
    let html = String::from_utf8_lossy(&out.stdout);
    // --sankey-value-order opts into the flow-ranked layout: explicit per-column node positions
    // baked onto the node object (which serializes `…,"thickness":20,"x":[…],"y":[…]`) + snap
    // arrangement.
    assert!(html.contains(r#""arrangement":"snap""#), "html: {html}");
    assert!(
        html.contains(r#""thickness":20,"x":["#),
        "--sankey-value-order should bake flow-ranked node positions on the node; html: {html}"
    );
    // the toggle is still offered so the reader can switch back to snap order at runtime.
    assert!(
        html.contains(r#""updatemenus""#) && html.contains("node order"),
        "the runtime node-order toggle button should be present; html: {html}"
    );
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
    // the box (no explicit --box-points needed). --violin off pins the box path (violins are the
    // default distribution panel and cap their overlay at outliers).
    let wrk = Workdir::new("viz_smart_box_points_heuristic_small_overlays_all");
    wrk.create_from_string("d.csv", &continuous_box_csv(100));

    let mut cmd = wrk.command("viz");
    cmd.args(["smart", "d.csv", "--violin", "off"]);
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
    // large dataset (> SMART_BOX_OUTLIERS_MAX rows) with --violin off: the heuristic draws NO
    // points and the box stays a cache-only quartile summary (no `boxpoints` key on the trace).
    // (Under the default --violin auto, such a column becomes a SAMPLED violin instead.)
    let wrk = Workdir::new("viz_smart_box_points_heuristic_large_none");
    wrk.create_from_string("d.csv", &continuous_box_csv(12_000));

    let out_html = wrk.path("dash.html").to_string_lossy().to_string();
    let mut cmd = wrk.command("viz");
    cmd.args(["smart", "d.csv", "--violin", "off", "-o", &out_html]);
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
    // an explicit --box-points wins over the size heuristic: `none` (with --violin off, which
    // pins box panels) keeps the cache-only box even though the small dataset would otherwise
    // overlay all points.
    let wrk = Workdir::new("viz_smart_box_points_explicit_overrides_heuristic");
    wrk.create_from_string("d.csv", &continuous_box_csv(100));

    let mut none_cmd = wrk.command("viz");
    none_cmd.args(["smart", "d.csv", "--violin", "off", "--box-points", "none"]);
    let none_out = wrk.output(&mut none_cmd);
    assert!(none_out.status.success());
    let none_html = String::from_utf8_lossy(&none_out.stdout);
    assert!(none_html.contains(r#""type":"box""#));
    assert!(!none_html.contains(r#""boxpoints":"#));

    // and an explicit `outliers` forces outliers regardless of the small size (which would be
    // `all`); --violin off pins the box trace (a violin serializes the overlay as "points")
    let mut out_cmd = wrk.command("viz");
    out_cmd.args([
        "smart",
        "d.csv",
        "--violin",
        "off",
        "--box-points",
        "outliers",
    ]);
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
    // a > 10k column with NO Tukey outliers (uniform spread), --violin off: stays a cache-only
    // quartile box — a box trace, but no native points (no boxpoints key, no 2D y), no data pass.
    let wrk = Workdir::new("viz_smart_box_no_outliers_large");
    wrk.create_from_string("d.csv", &continuous_box_csv(12_000));

    let out_html = wrk.path("dash.html").to_string_lossy().to_string();
    let mut cmd = wrk.command("viz");
    cmd.args(["smart", "d.csv", "--violin", "off", "-o", &out_html]);
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
    // the uncapped 6000, confirming the cap. The slack absorbs the value's incidental appearances
    // elsewhere in the dashboard's figure JSON (axis ranges, hover text); the plotly.js bundle is
    // extremely unlikely to contribute, as it rides gzip+base64-encoded here.
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
fn viz_smart_collapses_one_to_one_categorical_twins() {
    // `orgcode` <-> `orgfullname`: a code/label pair in strict 1:1 correspondence. Charted
    // separately they produce byte-identical frequency bars and waste a parcats axis on the same
    // variable. Only the SHORTER-valued member is charted (it fits a bar label; the long form
    // truncates on an axis), and the drop is reported so the mapping stays discoverable
    // (issue #4221). Column names are deliberately distinctive: the embedded plotly bundle
    // contains words like "across", so a short name would false-match a substring assertion.
    let wrk = Workdir::new("viz_smart_collapses_one_to_one_categorical_twins");
    let codes = [
        ("DPR", "Department of Parks and Recreation"),
        ("DDC", "Department of Design and Construction"),
        ("HPD", "Housing Preservation and Development"),
        ("DOE", "Department of Education"),
    ];
    let regions = ["north", "south", "east"];
    let mut rows = String::from("orgcode,orgfullname,region,amount\n");
    for i in 0..60 {
        let (code, full) = codes[i % 4];
        rows.push_str(&format!(
            "{code},{full},{},{}\n",
            regions[(i / 4) % 3],
            10 + (i % 7) * 5
        ));
    }
    wrk.create_from_string("t.csv", &rows);

    let out_html = wrk.path("dash.html").to_string_lossy().to_string();
    let mut cmd = wrk.command("viz");
    cmd.env("QSV_VIZ_NO_COMPRESS", "1");
    // the data viewer drawer embeds ALL raw columns by design, including collapsed twins;
    // disable it — this test pins down the charting collapse specifically.
    cmd.args([
        "smart",
        "t.csv",
        "--preview-threshold",
        "0",
        "-o",
        &out_html,
    ]);
    let got = wrk.output_stderr(&mut cmd);
    let html = wrk.read_to_string("dash.html").unwrap();

    assert!(
        got.contains("charting only orgcode for orgfullname"),
        "expected a 1:1 collapse note naming the kept and dropped columns; got: {got}"
    );
    assert!(
        html.contains("orgcode"),
        "the shorter-valued member must still be charted"
    );
    assert!(
        !html.contains("orgfullname"),
        "the redundant 1:1 twin must not reach the dashboard at all"
    );
}

#[test]
fn viz_smart_keeps_equal_cardinality_columns_that_are_not_one_to_one() {
    // The guard against over-collapsing: both columns have the SAME cardinality (3), which is what
    // makes them candidates at all, but their values cross (9 distinct pairs, not 3), so neither
    // determines the other and BOTH panels must survive. Equal cardinality is only a cheap
    // pre-filter — a false positive here would DELETE a panel.
    let wrk = Workdir::new("viz_smart_keeps_equal_cardinality_columns_that_are_not_one_to_one");
    let mut rows = String::from("zonecode,huegroup,amount\n");
    for i in 0..60 {
        rows.push_str(&format!(
            "{},{},{}\n",
            ["red", "green", "blue"][i % 3],
            ["north", "south", "east"][(i / 3) % 3],
            10 + (i % 7) * 5
        ));
    }
    wrk.create_from_string("t.csv", &rows);

    let out_html = wrk.path("dash.html").to_string_lossy().to_string();
    let mut cmd = wrk.command("viz");
    cmd.env("QSV_VIZ_NO_COMPRESS", "1");
    cmd.args(["smart", "t.csv", "-o", &out_html]);
    let got = wrk.output_stderr(&mut cmd);
    let html = wrk.read_to_string("dash.html").unwrap();

    assert!(
        !got.contains("1:1"),
        "equal cardinality alone must not collapse two crossed columns; got: {got}"
    );
    assert!(html.contains("zonecode"), "html missing zonecode");
    assert!(html.contains("huegroup"), "html missing huegroup");
}

#[test]
fn viz_smart_keeps_columns_blank_on_different_rows() {
    // Two columns that map perfectly WHERE BOTH ARE POPULATED but are blank on disjoint row
    // ranges are not the same variable — their bars carry different counts and different (NULL)
    // shares. Their cached cardinalities still match (two values plus the empty), so the
    // pre-filter lets the pair through and the judgment itself has to reject it: an empty cell
    // canonicalizes to its own id, so the two columns separate on the first row where one is
    // blank and the other is not (roborev 3818).
    let wrk = Workdir::new("viz_smart_keeps_columns_blank_on_different_rows");
    let mut rows = String::from("acode,bname,amount\n");
    for i in 0..200 {
        let (a, b) = if i < 50 {
            ("", if i % 2 == 0 { "P" } else { "Q" })
        } else if i < 100 {
            (if i % 3 == 0 { "x" } else { "y" }, "")
        } else if i % 2 == 0 {
            ("x", "P")
        } else {
            ("y", "Q")
        };
        rows.push_str(&format!("{a},{b},{}\n", 10 + (i % 7) * 5));
    }
    wrk.create_from_string("t.csv", &rows);

    let out_html = wrk.path("dash.html").to_string_lossy().to_string();
    let mut cmd = wrk.command("viz");
    cmd.env("QSV_VIZ_NO_COMPRESS", "1");
    cmd.args(["smart", "t.csv", "-o", &out_html]);
    let got = wrk.output_stderr(&mut cmd);

    assert!(
        !got.contains("1:1"),
        "columns blank on different rows must not collapse; got: {got}"
    );
    let html = wrk.read_to_string("dash.html").unwrap();
    assert!(html.contains("acode"), "html missing acode");
    assert!(html.contains("bname"), "html missing bname");
}

#[test]
fn viz_smart_keeps_sparse_columns_that_only_coincide_where_populated() {
    // Two mostly-empty columns that carry a value on the SAME narrow slice would look like a
    // clean bijection if blanks counted as a category: one value each, one pair, and hundreds of
    // rows of "support" supplied entirely by their shared emptiness. Absence is not a value, so
    // the judgment runs only over rows where both columns are populated — where there is a single
    // distinct value and 10 rows, far too little to delete a panel over.
    let wrk = Workdir::new("viz_smart_keeps_sparse_columns_that_only_coincide_where_populated");
    let mut rows = String::from("zonecode,rarecodeone,rarecodetwo,amount\n");
    for i in 0..300 {
        let (a, b) = if i < 10 { ("P", "Q") } else { ("", "") };
        rows.push_str(&format!(
            "{},{a},{b},{}\n",
            ["red", "green", "blue"][i % 3],
            10 + (i % 7) * 5
        ));
    }
    wrk.create_from_string("t.csv", &rows);

    let out_html = wrk.path("dash.html").to_string_lossy().to_string();
    let mut cmd = wrk.command("viz");
    cmd.env("QSV_VIZ_NO_COMPRESS", "1");
    cmd.args(["smart", "t.csv", "-o", &out_html]);
    let got = wrk.output_stderr(&mut cmd);

    assert!(
        !got.contains("1:1"),
        "co-emptiness must not read as a functional dependency; got: {got}"
    );
}

#[test]
fn viz_smart_measure_by_dimension_panel() {
    let wrk = Workdir::new("viz_smart_measure_by_dimension_panel");
    // a low-cardinality dimension (region) that strongly separates a numeric measure (amount):
    // east clusters ~10-19, west ~100-109, so between-group variance dominates -> a high
    // correlation ratio η² that clears the gate and adds an "amount by region" bar.
    let mut rows = String::from("region,amount\n");
    for i in 0..30 {
        rows.push_str(&format!("east,{}\n", 10 + (i % 10)));
    }
    for i in 0..30 {
        rows.push_str(&format!("west,{}\n", 100 + (i % 10)));
    }
    wrk.create_from_string("rev.csv", &rows);

    let out_html = wrk.path("dash.html").to_string_lossy().to_string();
    let mut cmd = wrk.command("viz");
    cmd.args(["smart", "rev.csv", "-o", &out_html]);
    wrk.assert_success(&mut cmd);

    let html = wrk.read_to_string("dash.html").unwrap();
    // the grouped bar's title names the measure, dimension, aggregation, and the explained share.
    // This measure is symmetric (mean == median), so it keeps the mean.
    assert!(
        html.contains("amount by region (mean"),
        "expected a measure-by-dimension bar titled 'amount by region (mean, ...)'"
    );
    // η² is stated as a plain explained-variance share, not a bare coefficient (issue #4220)
    assert!(
        html.contains("% of variance)"),
        "expected the grouped bar to state η² as an explained-variance percentage"
    );
    assert!(
        !html.contains("\\u003b7\\u00b2=") && !html.contains("η²="),
        "the bare 'η²=' coefficient should no longer headline the grouped bar"
    );
}

/// A heavy right tail (30x10, 15x100, 10x1000, 5x5000) split so the big projects sit in one
/// group. Mean ~613 against a median of 55 — an 11x ratio, so the mean describes the tail rather
/// than a typical row (`mean_is_outlier_driven`). Shared by the aggregation tests below.
fn skewed_amount_rows() -> String {
    let mut rows = String::from("region,amount\n");
    for _ in 0..30 {
        rows.push_str("east,10\n");
    }
    for _ in 0..15 {
        rows.push_str("east,100\n");
    }
    for _ in 0..10 {
        rows.push_str("west,1000\n");
    }
    for _ in 0..5 {
        rows.push_str("west,5000\n");
    }
    rows
}

#[test]
fn viz_smart_measure_by_dimension_sums_a_skewed_untagged_measure() {
    let wrk = Workdir::new("viz_smart_measure_by_dimension_sums_a_skewed_untagged_measure");
    // Without a dictionary every measure is un-tagged, which used to mean "average it". On a
    // heavily right-skewed additive measure the mean is set by the largest handful of rows, so
    // the panel ranked whoever held the single biggest item rather than the biggest total
    // (issue #4220). Such a column must now be summed.
    wrk.create_from_string("skew.csv", &skewed_amount_rows());

    let out_html = wrk.path("dash.html").to_string_lossy().to_string();
    let mut cmd = wrk.command("viz");
    cmd.args(["smart", "skew.csv", "-o", &out_html]);
    wrk.assert_success(&mut cmd);

    let html = wrk.read_to_string("dash.html").unwrap();
    assert!(
        html.contains("amount by region (sum"),
        "a heavily right-skewed un-tagged measure should be summed, not averaged"
    );
}

#[test]
fn viz_smart_measure_by_dimension_keeps_mean_for_a_skewed_intensive_measure() {
    let wrk =
        Workdir::new("viz_smart_measure_by_dimension_keeps_mean_for_a_skewed_intensive_measure");
    // Identical distribution to the test above, but the column NAMES an intensive quantity. A
    // rate must never be summed across a group however skewed it is, and `is_intensive_measure`
    // recognizes that from the header alone — no dictionary needed.
    wrk.create_from_string(
        "rate.csv",
        &skewed_amount_rows().replace("region,amount", "region,failure_rate"),
    );

    let out_html = wrk.path("dash.html").to_string_lossy().to_string();
    let mut cmd = wrk.command("viz");
    cmd.args(["smart", "rate.csv", "-o", &out_html]);
    wrk.assert_success(&mut cmd);

    let html = wrk.read_to_string("dash.html").unwrap();
    assert!(
        html.contains("failure_rate by region (mean"),
        "an intensive measure must keep the mean even when heavily right-skewed"
    );
}

#[test]
fn viz_smart_correlation_uses_spearman_on_heavy_tailed_columns() {
    let wrk = Workdir::new("viz_smart_correlation_uses_spearman_on_heavy_tailed_columns");
    // Two heavy-tailed numeric columns (a majority of the matrix) => the correlation panel is
    // computed and LABELED as Spearman's rank rho, whose cells a few extreme rows can't dominate
    // (issue #4220).
    let mut rows = String::from("region,spend,commit\n");
    for i in 0..30 {
        rows.push_str(&format!("east,{},{}\n", 10 + i % 3, 12 + i % 3));
    }
    for i in 0..15 {
        rows.push_str(&format!("east,{},{}\n", 100 + i % 3, 110 + i % 3));
    }
    for i in 0..10 {
        rows.push_str(&format!("west,{},{}\n", 1000 + i % 3, 1100 + i % 3));
    }
    for i in 0..5 {
        rows.push_str(&format!("west,{},{}\n", 5000 + i % 3, 5500 + i % 3));
    }
    wrk.create_from_string("heavy.csv", &rows);

    let out_html = wrk.path("dash.html").to_string_lossy().to_string();
    let mut cmd = wrk.command("viz");
    cmd.args(["smart", "heavy.csv", "-o", &out_html]);
    wrk.assert_success(&mut cmd);

    let html = wrk.read_to_string("dash.html").unwrap();
    assert!(
        html.contains("Correlation (Spearman"),
        "a mostly heavy-tailed numeric table should get a Spearman correlation panel"
    );
}

#[test]
fn viz_smart_correlation_stays_pearson_on_well_behaved_columns() {
    let wrk = Workdir::new("viz_smart_correlation_stays_pearson_on_well_behaved_columns");
    // Symmetric, tame columns keep Pearson — and the panel now says so, rather than leaving the
    // reader to assume which coefficient a bare "Correlation" holds.
    let mut rows = String::from("a,b\n");
    for i in 0..40 {
        rows.push_str(&format!("{},{}\n", 10 + i % 20, 30 + (i * 2) % 20));
    }
    wrk.create_from_string("tame.csv", &rows);

    let out_html = wrk.path("dash.html").to_string_lossy().to_string();
    let mut cmd = wrk.command("viz");
    cmd.args(["smart", "tame.csv", "-o", &out_html]);
    wrk.assert_success(&mut cmd);

    let html = wrk.read_to_string("dash.html").unwrap();
    assert!(
        html.contains("Correlation (Pearson r)"),
        "a well-behaved numeric table should keep — and name — the Pearson correlation panel"
    );
    assert!(!html.contains("Correlation (Spearman"));
}

#[test]
fn viz_smart_measure_by_dimension_keeps_near_unique_dictionary_measure() {
    let wrk = Workdir::new("viz_smart_measure_by_dimension_keeps_near_unique_dictionary_measure");
    // a genuine per-row measure (revenue, all 60 values distinct -> near-unique) strongly explained
    // by a low-card dimension (region). The correlation candidate list drops near-unique columns to
    // keep IDs out of the matrix; MeasureByDim must NOT inherit that exclusion for a column the
    // dictionary explicitly routes as a Measure, or this meaningful panel would be silently
    // omitted.
    let mut rows = String::from("region,revenue\n");
    for i in 0..30 {
        rows.push_str(&format!("east,{}\n", 1000 + i));
    }
    for i in 0..30 {
        rows.push_str(&format!("west,{}\n", 5000 + i));
    }
    wrk.create_from_string("rev.csv", &rows);
    wrk.create_from_string(
        "dict.schema.json",
        r#"{
          "$schema": "https://json-schema.org/draft/2020-12/schema",
          "type": "object",
          "properties": {
            "region": { "type": "string", "title": "Region",
              "x-qsv": { "qsv_type": "String", "role": "dimension", "concept": "category.status" } },
            "revenue": { "type": "integer", "title": "Revenue",
              "x-qsv": { "qsv_type": "Integer", "role": "measure", "concept": "measure.amount" } }
          }
        }"#,
    );

    let out_html = wrk.path("dash.html").to_string_lossy().to_string();
    let mut cmd = wrk.command("viz");
    cmd.args(["smart", "rev.csv", "-o", &out_html, "--dictionary"])
        .arg(wrk.path("dict.schema.json"));
    wrk.assert_success(&mut cmd);

    let html = wrk.read_to_string("dash.html").unwrap();
    // the dictionary friendly labels title the bar; presence proves the near-unique measure was
    // NOT excluded. (An additive `measure.amount` aggregates as "sum", not "mean".)
    assert!(
        html.contains("Revenue by Region ("),
        "a dictionary-tagged near-unique measure should still yield a measure-by-dimension bar"
    );
}

#[test]
fn viz_smart_metadata_table_always_renders() {
    let wrk = Workdir::new("viz_smart_metadata_table_always_renders");
    wrk.create_from_string(
        "data.csv",
        "region,revenue,quarter\neast,1000,Q1\neast,1200,Q2\nwest,5000,Q1\nwest,5200,Q2\nnorth,\
         3000,Q1\n",
    );
    let out_html = wrk.path("dash.html").to_string_lossy().to_string();
    let mut cmd = wrk.command("viz");
    cmd.args(["smart", "data.csv", "-o", &out_html]);
    wrk.assert_success(&mut cmd);

    let html = wrk.read_to_string("dash.html").unwrap();
    // Rows/Columns/Compiled always render in HTML output.
    assert!(html.contains(r#"<table class="qsv-viz-meta">"#));
    // the Rows cell also carries the data viewer's "(Explore)" link (issue #4283)
    assert!(html.contains(r##"<td class="qsv-viz-meta-k">Rows:</td><td>5 <a href="#""##));
    assert!(html.contains(r#"(Explore)<svg class="qsv-link-icon""#));
    assert!(html.contains("</svg></a></td>"));
    assert!(html.contains(r#"<td class="qsv-viz-meta-k">Columns:</td><td>3</td>"#));
    // assert on the label, never the timestamp value (it makes output non-deterministic).
    assert!(html.contains(r#"<td class="qsv-viz-meta-k">Compiled:</td>"#));
    // "Generated by" carries the producing tool AND its version -- MUST 9 (Provenance) of
    // docs/DATA_SCHEMATIC.md. Asserted against CARGO_PKG_VERSION rather than a literal so a
    // version bump can never silently stop this from being the running build's version.
    assert!(html.contains(&format!(
        r#"<td class="qsv-viz-meta-k">Generated by:</td><td>qsv {}</td>"#,
        env!("CARGO_PKG_VERSION")
    )));
    // no --dataset-pid, no --dict-info: neither optional row appears. (Class-qualified so
    // plotly.js's own "axisRefDescription" and any stray "PID" text can't false-match.)
    assert!(!html.contains(r#"qsv-viz-meta-k">PID:"#));
    assert!(!html.contains(r#"qsv-viz-meta-k">Description:"#));
}

#[test]
fn viz_smart_metadata_pid_link() {
    let wrk = Workdir::new("viz_smart_metadata_pid_link");
    wrk.create_from_string(
        "data.csv",
        "region,revenue,quarter\neast,1000,Q1\neast,1200,Q2\nwest,5000,Q1\nwest,5200,Q2\nnorth,\
         3000,Q1\n",
    );
    let out_html = wrk.path("dash.html").to_string_lossy().to_string();
    let mut cmd = wrk.command("viz");
    cmd.args([
        "smart",
        "data.csv",
        "-o",
        &out_html,
        "--dataset-pid",
        "https://doi.org/10.1234/abc",
    ]);
    wrk.assert_success(&mut cmd);

    let html = wrk.read_to_string("dash.html").unwrap();
    // an http(s) PID becomes a link, opened safely in a new tab.
    assert!(html.contains(
        r#"<td class="qsv-viz-meta-k">PID:</td><td><a href="https://doi.org/10.1234/abc" target="_blank" rel="noopener noreferrer">https://doi.org/10.1234/abc</a></td>"#
    ));
}

#[test]
fn viz_smart_metadata_pid_rejects_dangerous_scheme() {
    let wrk = Workdir::new("viz_smart_metadata_pid_rejects_dangerous_scheme");
    wrk.create_from_string("data.csv", "amount,quarter\n1000,Q1\n2000,Q2\n3000,Q1\n");
    let out_html = wrk.path("dash.html").to_string_lossy().to_string();
    let mut cmd = wrk.command("viz");
    cmd.args([
        "smart",
        "data.csv",
        "-o",
        &out_html,
        "--dataset-pid",
        "javascript:alert(1)",
    ]);
    wrk.assert_success(&mut cmd);

    let html = wrk.read_to_string("dash.html").unwrap();
    // a non-http(s) scheme is never turned into an href; the value still shows as plain text
    // rather than being silently dropped.
    assert!(!html.contains(r#"href="javascript:"#));
    assert!(html.contains(r#"<td class="qsv-viz-meta-k">PID:</td><td>javascript:alert(1)</td>"#));
}

#[test]
fn viz_smart_metadata_description_only_with_dict_info() {
    let wrk = Workdir::new("viz_smart_metadata_description_only_with_dict_info");
    wrk.create_from_string(
        "data.csv",
        "region,revenue\neast,1000\neast,1200\nwest,5000\nwest,5200\n",
    );
    wrk.create_from_string(
        "dict.schema.json",
        r#"{
          "$schema": "https://json-schema.org/draft/2020-12/schema",
          "type": "object",
          "description": "**Description**\n\nFirst paragraph of the summary.\n\nSecond paragraph is dropped.",
          "properties": {
            "region": { "type": "string", "title": "Region",
              "x-qsv": { "qsv_type": "String", "role": "dimension", "concept": "category.status" } },
            "revenue": { "type": "integer", "title": "Revenue",
              "x-qsv": { "qsv_type": "Integer", "role": "measure", "concept": "measure.amount" } }
          }
        }"#,
    );

    // without --dict-info: no Description row even though a dictionary description exists.
    let out_nodi = wrk.path("nodi.html").to_string_lossy().to_string();
    let mut cmd = wrk.command("viz");
    cmd.args(["smart", "data.csv", "-o", &out_nodi, "--dictionary"])
        .arg(wrk.path("dict.schema.json"));
    wrk.assert_success(&mut cmd);
    let html = wrk.read_to_string("nodi.html").unwrap();
    // class-qualified so plotly.js's own "axisRefDescription" can't false-match.
    assert!(!html.contains(r#"qsv-viz-meta-k">Description:"#));

    // with --dict-info: the Description row shows the FIRST paragraph only.
    let out_di = wrk.path("di.html").to_string_lossy().to_string();
    let mut cmd = wrk.command("viz");
    cmd.args([
        "smart",
        "data.csv",
        "-o",
        &out_di,
        "--dict-info",
        "--dictionary",
    ])
    .arg(wrk.path("dict.schema.json"));
    wrk.assert_success(&mut cmd);
    let html = wrk.read_to_string("di.html").unwrap();
    // the meta cell holds ONLY the first paragraph (it closes right after it); the embedded
    // dict page still renders the full description, so only assert on the meta cell here.
    assert!(html.contains(
        r#"<td class="qsv-viz-meta-k">Description:</td><td>First paragraph of the summary.</td>"#
    ));
}

#[test]
fn viz_smart_metadata_description_absent_when_dictionary_has_none() {
    let wrk = Workdir::new("viz_smart_metadata_description_absent_when_dictionary_has_none");
    wrk.create_from_string(
        "data.csv",
        "region,revenue\neast,1000\neast,1200\nwest,5000\nwest,5200\n",
    );
    // a valid dictionary WITHOUT a top-level "description".
    wrk.create_from_string(
        "dict.schema.json",
        r#"{
          "$schema": "https://json-schema.org/draft/2020-12/schema",
          "type": "object",
          "properties": {
            "region": { "type": "string", "title": "Region",
              "x-qsv": { "qsv_type": "String", "role": "dimension", "concept": "category.status" } },
            "revenue": { "type": "integer", "title": "Revenue",
              "x-qsv": { "qsv_type": "Integer", "role": "measure", "concept": "measure.amount" } }
          }
        }"#,
    );

    let out_html = wrk.path("dash.html").to_string_lossy().to_string();
    let mut cmd = wrk.command("viz");
    cmd.args([
        "smart",
        "data.csv",
        "-o",
        &out_html,
        "--dict-info",
        "--dictionary",
    ])
    .arg(wrk.path("dict.schema.json"));
    wrk.assert_success(&mut cmd);

    let html = wrk.read_to_string("dash.html").unwrap();
    // --dict-info is set, but with no dataset description there is no Description row.
    assert!(!html.contains(r#"qsv-viz-meta-k">Description:"#));
}

#[test]
fn viz_smart_bubble_scatter_size_encodes_third() {
    let wrk = Workdir::new("viz_smart_bubble_scatter_size_encodes_third");
    // three correlated numeric columns (small dataset -> scatter, not contour). The strongest pair
    // gets a scatter drill-down whose marker SIZE encodes the third (most-associated) column. x is
    // i%40 (cardinality 40, not near-unique) so it qualifies as a continuous measure.
    let mut rows = String::from("x,y,z\n");
    for i in 0..60 {
        let x = i % 40;
        rows.push_str(&format!("{x},{},{}\n", x * 2, x * 3));
    }
    wrk.create_from_string("xyz.csv", &rows);

    let out_html = wrk.path("dash.html").to_string_lossy().to_string();
    let mut cmd = wrk.command("viz");
    cmd.args(["smart", "xyz.csv", "-o", &out_html]);
    wrk.assert_success(&mut cmd);

    let html = wrk.read_to_string("dash.html").unwrap();
    // the scatter pair panel's title notes the size-encoded third column ...
    assert!(
        html.contains("size: z"),
        "bubble scatter should encode z as marker size"
    );
    // ... and the trace carries a per-point size array (bubble markers)
    assert!(html.contains(r#""size":["#));
}

#[test]
fn viz_smart_cyclic_seasonality_panel() {
    let wrk = Workdir::new("viz_smart_cyclic_seasonality_panel");
    // a datetime column with intraday timestamps spread across many hours -> a polar
    // hour-of-day "seasonality" profile (HTML-only, ScatterPolar).
    let mut rows = String::from("ts\n");
    for i in 0..48 {
        let h = i % 24;
        rows.push_str(&format!("2021-06-01T{h:02}:15:00\n"));
    }
    wrk.create_from_string("events.csv", &rows);

    let out_html = wrk.path("dash.html").to_string_lossy().to_string();
    let mut cmd = wrk.command("viz");
    cmd.args(["smart", "events.csv", "-o", &out_html]);
    wrk.assert_success(&mut cmd);

    let html = wrk.read_to_string("dash.html").unwrap();
    assert!(
        html.contains("Records by hour of day"),
        "expected an hour-of-day cyclic panel"
    );
    assert!(html.contains(r#""type":"scatterpolar""#));
    // a polar subplot paints its angular tick labels OUTSIDE its plot area, so the panel carries
    // taller top/bottom margins than the other inline panels — otherwise the 12 o'clock label
    // collides with the title and the 6 o'clock one is clipped by the panel edge.
    assert!(
        html.contains(r#""margin":{"l":20,"r":20,"t":64,"b":44,"pad":4}"#),
        "polar panel should reserve extra top/bottom margin for its angular tick labels"
    );
}

#[test]
fn viz_smart_map_bubble_sizes_by_measure() {
    let wrk = Workdir::new("viz_smart_map_bubble_sizes_by_measure");
    quakes(&wrk);
    // a dictionary tagging `magnitude` as a map measure (measure.amount) -> the smart map sizes
    // each point by magnitude. Without a dictionary no measure is tagged, so maps stay fixed-size.
    wrk.create_from_string(
        "dict.schema.json",
        r#"{
          "$schema": "https://json-schema.org/draft/2020-12/schema",
          "type": "object",
          "properties": {
            "place": { "type": "string",
              "x-qsv": { "qsv_type": "String", "role": "identifier", "concept": "id.natural_key" } },
            "lat": { "type": "number", "x-qsv": { "qsv_type": "Float", "concept": "geo.latitude" } },
            "lon": { "type": "number", "x-qsv": { "qsv_type": "Float", "concept": "geo.longitude" } },
            "magnitude": { "type": "number", "title": "Magnitude",
              "x-qsv": { "qsv_type": "Float", "role": "measure", "concept": "measure.amount" } }
          }
        }"#,
    );

    let mut cmd = wrk.command("viz");
    cmd.args(["smart", "--smarter", "quakes.csv", "--dictionary"])
        .arg(wrk.path("dict.schema.json"));
    let out = wrk.output(&mut cmd);
    assert!(out.status.success());

    let html = String::from_utf8_lossy(&out.stdout);
    // global spread -> ScatterGeo world overview, with magnitude encoded as per-point marker size
    assert!(html.contains(r#""type":"scattergeo""#));
    assert!(
        html.contains(r#""size":["#),
        "map points should be bubble-sized by the dictionary measure"
    );
}

// The x-axis the `viz_smart_timeseries_dmy_dates` rows render as under each reading. Both arrays
// cover the same eight cells, and every cell is ambiguous (day AND month <= 12), so the two
// readings differ in BOTH their values and their chronological order -- which is what makes an
// exact-array assertion able to tell them apart.
const DMY_X_AXIS: &str = r#""x":["2021-01-11","2021-02-07","2021-03-09","2021-04-02","2021-05-03","2021-06-05","2021-07-08","2021-08-06"]"#;
const MDY_X_AXIS: &str = r#""x":["2021-02-04","2021-03-05","2021-05-06","2021-06-08","2021-07-02","2021-08-07","2021-09-03","2021-11-01"]"#;

// The `viz_smart_timeseries_dmy_dates` rows plus a dictionary declaring `content_type` on
// `sale_date`. `revenue` is declared WITHOUT a role or concept so it stays `Route::Defer` and the
// panel renders in raw mode -- an aggregating role would bucket the x-axis into period labels and
// the exact-date assertions could not be made.
fn dict_dmy_fixture(wrk: &Workdir, content_type: &str) {
    let rows = "sale_date,revenue\n07/02/2021,1500\n03/05/2021,1200\n11/01/2021,1000\n06/08/2021,\
                1700\n02/04/2021,1100\n09/03/2021,1300\n05/06/2021,1600\n08/07/2021,1400\n";
    wrk.create_from_string("sales.csv", rows);
    wrk.create_from_string(
        "dict.schema.json",
        &format!(
            r#"{{
              "$schema": "https://json-schema.org/draft/2020-12/schema",
              "type": "object",
              "properties": {{
                "sale_date": {{ "type": "string", "title": "Sale Date",
                  "x-qsv": {{ "qsv_type": "Date", "content_type": "{content_type}",
                    "role": "timestamp", "concept": "time.event_timestamp" }} }},
                "revenue": {{ "type": "integer", "title": "Revenue",
                  "x-qsv": {{ "qsv_type": "Integer", "content_type": "unknown" }} }}
              }}
            }}"#
        ),
    );
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
    assert!(html.contains(DMY_X_AXIS));
}

// issue #4303: a dictionary declares the column's own date format via `x-qsv.content_type`
// (`date:%m/%d/%Y`). That declaration is authoritative for the column, so an opposing
// QSV_PREFER_DMY=1 must NOT flip the reading. Deliberately run WITHOUT `--dict-info`: the
// pre-existing `dict_icons: Option<&DictData>` params are `None` unless `--dict-info` AND HTML,
// so a fix routed through them would be dead on exactly this (plain `--dictionary`) run.
#[test]
fn viz_smart_timeseries_dict_format_overrides_prefer_dmy() {
    let wrk = Workdir::new("viz_smart_timeseries_dict_format_overrides_prefer_dmy");
    dict_dmy_fixture(&wrk, "date:%m/%d/%Y");

    let out_html = wrk.path("dash.html").to_string_lossy().to_string();
    let mut cmd = wrk.command("viz");
    cmd.env("QSV_PREFER_DMY", "1");
    cmd.args(["smart", "sales.csv", "-o", &out_html, "--dictionary"])
        .arg(wrk.path("dict.schema.json"));
    wrk.assert_success(&mut cmd);

    let html = wrk.read_to_string("dash.html").unwrap();
    // MONTH-first, per the dictionary: 11/01 -> 2021-11-01. Both the values AND the ordering
    // differ from the day-first array QSV_PREFER_DMY=1 alone produces (see
    // `viz_smart_timeseries_dmy_dates`), so this cannot pass by coincidence.
    assert!(
        html.contains(MDY_X_AXIS),
        "the dictionary's declared %m/%d/%Y must outrank QSV_PREFER_DMY=1"
    );
}

// Same scenario WITH `--dict-info`. Pinning that both states agree is what proves the fix does
// not ride on `dict_icons()`, which is `Some` only here.
#[test]
fn viz_smart_timeseries_dict_format_overrides_prefer_dmy_with_dict_info() {
    let wrk = Workdir::new("viz_smart_timeseries_dict_format_overrides_prefer_dmy_with_dict_info");
    dict_dmy_fixture(&wrk, "date:%m/%d/%Y");

    let out_html = wrk.path("dash.html").to_string_lossy().to_string();
    let mut cmd = wrk.command("viz");
    cmd.env("QSV_PREFER_DMY", "1");
    cmd.args([
        "smart",
        "sales.csv",
        "-o",
        &out_html,
        "--dict-info",
        "--dictionary",
    ])
    .arg(wrk.path("dict.schema.json"));
    wrk.assert_success(&mut cmd);

    let html = wrk.read_to_string("dash.html").unwrap();
    assert!(html.contains(MDY_X_AXIS));
}

// Guards the reachability of the fix for dates that are NOT ambiguous. A column mixing
// `12/31/2021` (day > 12, so unreadable day-first) with ambiguous cells, under an opposing
// QSV_PREFER_DMY=1, could in principle fail date inference and leave `stats` typing the column
// `String` — which would make `canonical_date_col` skip it and the dictionary preference never
// engage at all. It does not: qsv-dateparser's `slash_mdy_family` tries the preferred order and
// FALLS BACK to the other one (a failed parse yields `None`, not an error), so the column still
// types as `Date` and the declared format still governs the reading. Pinned here because the
// other #4303 tests deliberately use only ambiguous cells and cannot cover this path.
#[test]
fn viz_smart_timeseries_dict_format_handles_unambiguous_mdy_values() {
    let wrk = Workdir::new("viz_smart_timeseries_dict_format_handles_unambiguous_mdy_values");
    // 12/31 and 11/30 have day > 12; 01/03 and 02/04 are ambiguous.
    wrk.create_from_string(
        "sales.csv",
        "sale_date,revenue\n12/31/2021,1500\n01/03/2021,1200\n11/30/2021,1000\n02/04/2021,1100\n",
    );
    wrk.create_from_string(
        "dict.schema.json",
        r#"{
          "$schema": "https://json-schema.org/draft/2020-12/schema",
          "type": "object",
          "properties": {
            "sale_date": { "type": "string", "title": "Sale Date",
              "x-qsv": { "qsv_type": "Date", "content_type": "date:%m/%d/%Y",
                "role": "timestamp", "concept": "time.event_timestamp" } },
            "revenue": { "type": "integer", "title": "Revenue",
              "x-qsv": { "qsv_type": "Integer", "content_type": "unknown" } }
          }
        }"#,
    );

    let out_html = wrk.path("dash.html").to_string_lossy().to_string();
    let mut cmd = wrk.command("viz");
    cmd.env("QSV_PREFER_DMY", "1");
    cmd.args(["smart", "sales.csv", "-o", &out_html, "--dictionary"])
        .arg(wrk.path("dict.schema.json"));
    wrk.assert_success(&mut cmd);

    let html = wrk.read_to_string("dash.html").unwrap();
    // the panel exists at all -> the column was still typed Date, not downgraded to String
    assert!(
        html.contains("Revenue over Sale Date"),
        "a date column mixing unambiguous MDY values must still be typed as a date"
    );
    // ... and every cell reads month-first: 01/03 -> Jan 3 (QSV_PREFER_DMY=1 would say Mar 1),
    // alongside the day>12 cells that only ever had one reading.
    assert!(html.contains(r#""x":["2021-01-03","2021-02-04","2021-11-30","2021-12-31"]"#));
}

// The tri-state fallback. `qsv_dateparser` only consults its DMY preference for slash-separated
// `d/m` values, so a year-leading format expresses no actionable preference and must leave
// QSV_PREFER_DMY in charge -- as must a bare `date` token carrying no format at all. Getting this
// wrong would turn "the dictionary is ignored" into "the dictionary wins when it said nothing".
#[test]
fn viz_smart_timeseries_dict_format_without_preference_defers_to_env() {
    for (case, content_type) in [("iso", "date:%Y-%m-%d"), ("bare", "date")] {
        // one viz run per Workdir: a stats sidecar is reused on mtime alone and ignores the DMY
        // preference, so a second run in the same directory could pass for the wrong reason.
        let wrk = Workdir::new(&format!(
            "viz_smart_timeseries_dict_format_without_preference_defers_to_env_{case}"
        ));
        dict_dmy_fixture(&wrk, content_type);

        let out_html = wrk.path("dash.html").to_string_lossy().to_string();
        let mut cmd = wrk.command("viz");
        cmd.env("QSV_PREFER_DMY", "1");
        cmd.args(["smart", "sales.csv", "-o", &out_html, "--dictionary"])
            .arg(wrk.path("dict.schema.json"));
        wrk.assert_success(&mut cmd);

        let html = wrk.read_to_string("dash.html").unwrap();
        assert!(
            html.contains(DMY_X_AXIS),
            "`{content_type}` expresses no actionable preference, so QSV_PREFER_DMY=1 must stand"
        );
    }
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
    // a token-free ScatterMap point map on OpenStreetMap tiles
    assert!(html.contains("Plotly.newPlot"));
    assert!(html.contains(r#""type":"scattermap""#));
    assert!(html.contains("open-street-map"));
    // auto-centered/zoomed MapLibre map layout
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
    assert!(html.contains(r#""type":"scattermap""#));
    assert!(html.contains(r#""colorscale":"Viridis""#));
    assert!(html.contains(r#""showscale":true"#));
    assert!(html.contains(r#""colorbar":{"title":{"text":"magnitude"#));
    // per-point hover surfaces the --color value (labeled) beside the coordinates, not just
    // the bare lat/lon plotly shows by default
    assert!(html.contains(r#""hoverinfo":"text""#));
    assert!(html.contains(r#""hovertext":["#));
    assert!(html.contains("magnitude: "));
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
    assert!(html.contains(r#""type":"scattermap""#));
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
    assert!(html.contains(r#""type":"densitymap""#));
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
    // one ScatterMap trace per region, named by category
    assert!(html.contains(r#""type":"scattermap""#));
    assert!(html.contains(r#""name":"Asia""#));
    assert!(html.contains(r#""name":"Americas""#));
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
    // rendered as an offline ScatterGeo projection world-overview (not a zoomed MapLibre tile map).
    assert!(html.contains("<!doctype html>"));
    assert!(html.contains(r#""type":"scattergeo""#));
    assert!(!html.contains(r#""type":"scattermap""#));
    // quakes span the globe, so the world-overview panel must NOT be scoped to one continent.
    assert!(!html.contains(r#""scope":"#));
}

#[test]
fn viz_smart_map_pan_bounds_from_full_extent_not_downsampled() {
    let wrk = Workdir::new("viz_smart_map_pan_bounds_from_full_extent_not_downsampled");
    // 200 points on a smooth latitude continuum 40.0..50.0 (constant lon), so the extremes are
    // CORE points, not spatial outliers. The maximum (50.0) is moved to row index 1.
    // downsample_pair is endpoint-inclusive (always keeps index 0 and n-1), so capped to 50
    // points it strides right past index 1 — the RENDERED points top out near 49.75, dropping
    // the true max.
    //
    // The MapLibre pan bounds (layout.map.bounds) must be derived from the FULL pre-downsample
    // extent, not the rendered/downsampled coordinates. With the true extent (min 40.0, max 50.0)
    // and the 100%-of-span padding (MAP_BOUNDS_PAD_FRAC = 1.0), north == 50.0 + (50.0 - 40.0) ==
    // 60.0. A regression to computing bounds from the downsampled points (max ~49.75) would instead
    // yield ~59.5, so the >= 59.99 assertion pins the fix.
    let n = 200usize;
    let mut lats: Vec<f64> = (0..n)
        .map(|r| 40.0 + 10.0 * r as f64 / (n - 1) as f64)
        .collect();
    lats.swap(1, n - 1);
    let mut rows = String::from("lat,lon\n");
    for la in &lats {
        rows.push_str(&format!("{la:.5},-74.0\n"));
    }
    wrk.create_from_string("ext.csv", &rows);

    let out_html = wrk.path("dash.html").to_string_lossy().to_string();
    let mut cmd = wrk.command("viz");
    cmd.args(["smart", "ext.csv", "-o", &out_html]);
    // force core downsampling well below the row count so the true max (at a dropped index) can't
    // survive into the rendered coordinates
    cmd.env("QSV_VIZ_MAX_POINTS", "50");
    wrk.assert_success(&mut cmd);
    let html = wrk.read_to_string("dash.html").unwrap();

    // pull "north" out of the map's bounds object (the only "bounds" in the payload)
    let bpos = html
        .find(r#""bounds":{"#)
        .expect("map pan bounds should be present");
    let seg = &html[bpos..(bpos + 200).min(html.len())];
    let marker = r#""north":"#;
    let start = seg.find(marker).expect("north key in bounds") + marker.len();
    let rest = &seg[start..];
    let end = rest
        .find(|c: char| !(c.is_ascii_digit() || c == '.' || c == '-'))
        .unwrap();
    let north: f64 = rest[..end].parse().unwrap();
    assert!(
        north >= 59.99,
        "pan bounds north should reflect the FULL extent (true max 50.0 -> 60.0), got {north}; \
         bounds computed from downsampled coords would be ~59.5. seg: {seg}"
    );
}

// A downsampled smart map announces the sampling on the panel itself (subtitle honesty cue,
// mirroring the violin "(sampled)" suffix): the HTML reader otherwise has no way to know the
// dots are a stride sample while the Regions choropleth beside it tallies every row.
#[test]
fn viz_smart_map_downsample_subtitle() {
    let wrk = Workdir::new("viz_smart_map_downsample_subtitle");
    // 300 tightly-clustered points (no geographic outliers) around one metro
    let mut rows = String::from("id,lat,lon\n");
    for i in 0..300 {
        rows.push_str(&format!(
            "{i},{:.5},{:.5}\n",
            40.44 + 0.01 * f64::from(i % 30) / 30.0,
            -79.99 - 0.01 * f64::from(i / 30) / 10.0
        ));
    }
    wrk.create_from_string("pts.csv", &rows);

    // cap the embedded points below the row count -> the subtitle names the sample
    let out_html = wrk.path("dash.html").to_string_lossy().to_string();
    let mut cmd = wrk.command("viz");
    cmd.args(["smart", "pts.csv", "-o", &out_html]);
    cmd.env("QSV_VIZ_MAX_POINTS", "100");
    wrk.assert_success(&mut cmd);
    let html = wrk.read_to_string("dash.html").unwrap();
    assert!(
        html.contains("of 300 points (sampled)"),
        "downsampled map should carry a sampling subtitle"
    );

    // without downsampling, no sampling subtitle
    let out_full = wrk.path("dash_full.html").to_string_lossy().to_string();
    let mut cmd = wrk.command("viz");
    cmd.args(["smart", "pts.csv", "-o", &out_full]);
    // don't inherit a QSV_VIZ_MAX_POINTS a developer/CI may already have set
    cmd.env_remove("QSV_VIZ_MAX_POINTS");
    wrk.assert_success(&mut cmd);
    let html = wrk.read_to_string("dash_full.html").unwrap();
    assert!(
        !html.contains("points (sampled)"),
        "fully-embedded map must not claim sampling"
    );
}

/// The `--photos` map trace (the one carrying per-point image URLs as `customdata`), pulled out of
/// an inline dashboard rendered with `QSV_VIZ_NO_COMPRESS` — which keeps each map figure as plain
/// JSON instead of the usual gzip+base64 payload, so the coordinates and photo payload can be
/// read back and compared directly. `None` when no trace carries photos.
fn photo_map_trace(html: &str) -> Option<serde_json::Value> {
    for chunk in html.split("Plotly.newPlot(").skip(1) {
        // `Plotly.newPlot("qsv-viz-panel-N", {figure});` — the figure starts after the div id.
        let Some(comma) = chunk.find(", ") else {
            continue;
        };
        // parse ONE JSON value and ignore the `);` and everything after it, so no brace/quote
        // counting is needed to find where the figure ends.
        let mut vals = serde_json::Deserializer::from_str(&chunk[comma + 2..])
            .into_iter::<serde_json::Value>();
        let Some(Ok(fig)) = vals.next() else {
            continue;
        };
        let Some(traces) = fig.get("data").and_then(|d| d.as_array()) else {
            continue;
        };
        if let Some(t) = traces.iter().find(|t| t.get("customdata").is_some()) {
            return Some(t.clone());
        }
    }
    None
}

fn f64_array(trace: &serde_json::Value, key: &str) -> Vec<f64> {
    trace[key]
        .as_array()
        .expect("array")
        .iter()
        .map(|v| v.as_f64().unwrap_or(f64::NAN))
        .collect()
}

fn str_array(trace: &serde_json::Value, key: &str) -> Vec<String> {
    trace[key]
        .as_array()
        .expect("array")
        .iter()
        .map(|v| v.as_str().unwrap_or("").to_string())
        .collect()
}

// `--photos` embeds each point's image URLs as trace `customdata`, and those payloads must stay
// welded to their own coordinates through the core downsampling stride. This is the property the
// packed-tuple threading in `build_map_panel` exists to guarantee: coordinates, hover lines and
// photos are strided TOGETHER, so a photo can never end up describing a different point's marker.
#[test]
fn viz_smart_photos_stay_aligned_through_downsampling() {
    let wrk = Workdir::new("viz_smart_photos_stay_aligned_through_downsampling");
    // Every row's photo URL encodes its own row number, and its latitude is a function of that
    // same number — so ANY drift between the coordinate arrays and the photo payload surfaces as
    // a URL whose number doesn't match the latitude it was embedded against.
    let n = 120usize;
    let mut rows = String::from("id,lat,lon,photo\n");
    for i in 0..n {
        rows.push_str(&format!(
            "{i},{:.5},-71.05000,https://example.org/p{i}.jpg#spot={i}\n",
            40.30000 + 0.00001 * i as f64
        ));
    }
    wrk.create_from_string("pts.csv", &rows);

    let out_html = wrk.path("dash.html").to_string_lossy().to_string();
    let mut cmd = wrk.command("viz");
    cmd.args(["smart", "pts.csv", "--photos", "-o", &out_html]);
    cmd.env("QSV_VIZ_NO_COMPRESS", "1");
    // force a stride well below the row count so downsampling definitely runs
    cmd.env("QSV_VIZ_MAX_POINTS", "25");
    wrk.assert_success(&mut cmd);
    let html = wrk.read_to_string("dash.html").unwrap();

    let trace = photo_map_trace(&html).expect("a map trace carrying photo customdata");
    let lats = f64_array(&trace, "lat");
    let photos = str_array(&trace, "customdata");
    assert_eq!(
        lats.len(),
        photos.len(),
        "customdata must be row-aligned to lat"
    );
    assert!(
        photos.len() < n,
        "expected a downsampled core (< {n} points), got {}",
        photos.len()
    );
    assert!(photos.len() >= 2, "expected at least a couple of points");

    for (lat, url) in lats.iter().zip(&photos) {
        let i: usize = url
            .trim_start_matches("https://example.org/p")
            .trim_end_matches(".jpg")
            .parse()
            .unwrap_or_else(|_| panic!("unexpected embedded photo url: {url}"));
        let expected = 40.30000 + 0.00001 * i as f64;
        assert!(
            (lat - expected).abs() < 1e-6,
            "photo for row {i} (expected lat {expected}) was embedded against lat {lat} — the \
             photo payload drifted off its coordinate during downsampling"
        );
    }
    // the `#spot=<id>` fragment is stripped from every embedded URL in the PAYLOAD (the hover
    // identifier text may still show the raw cell — that is the identifier-selection behavior,
    // independent of --photos).
    assert!(
        photos.iter().all(|p| !p.contains("#spot=")),
        "url fragments must be stripped from the customdata payload"
    );
}

// `--photos` is strictly opt-in: without it a dashboard must embed NO image URL and none of the
// preview chrome, so opening it makes no request to whatever third-party host the data names.
#[test]
fn viz_smart_photos_absent_unless_requested() {
    let wrk = Workdir::new("viz_smart_photos_absent_unless_requested");
    let mut rows = String::from("id,lat,lon,photo\n");
    for i in 0..40 {
        rows.push_str(&format!(
            "{i},{:.5},-71.05000,https://example.org/p{i}.jpg\n",
            40.30000 + 0.00010 * f64::from(i)
        ));
    }
    wrk.create_from_string("pts.csv", &rows);

    let out_html = wrk.path("dash.html").to_string_lossy().to_string();
    let mut cmd = wrk.command("viz");
    cmd.args(["smart", "pts.csv", "-o", &out_html]);
    cmd.env("QSV_VIZ_NO_COMPRESS", "1");
    wrk.assert_success(&mut cmd);
    let html = wrk.read_to_string("dash.html").unwrap();

    assert!(
        photo_map_trace(&html).is_none(),
        "no trace may carry photo customdata without --photos"
    );
    assert!(
        !html.contains("qsv-photo-box"),
        "the preview chrome must not be injected without --photos"
    );
    // Note: the raw URL CAN still appear as a point's hover identifier text (the photo column is
    // a high-cardinality string, so hover-field selection may pick it) — that is orthogonal to
    // --photos. The feature's opt-in is the customdata payload + chrome asserted above; the page
    // makes no image REQUEST because nothing assigns an <img> src.
}

// `--photos` on a dataset whose only URL column isn't images must stay completely inert — a false
// positive would put a broken preview on the page AND make it reference an unrelated host.
#[test]
fn viz_smart_photos_inert_without_an_image_column() {
    let wrk = Workdir::new("viz_smart_photos_inert_without_an_image_column");
    let mut rows = String::from("id,lat,lon,link\n");
    for i in 0..40 {
        rows.push_str(&format!(
            "{i},{:.5},-71.05000,https://example.org/case/{i}.html\n",
            40.30000 + 0.00010 * f64::from(i)
        ));
    }
    wrk.create_from_string("pts.csv", &rows);

    let out_html = wrk.path("dash.html").to_string_lossy().to_string();
    let mut cmd = wrk.command("viz");
    cmd.args(["smart", "pts.csv", "--photos", "-o", &out_html]);
    cmd.env("QSV_VIZ_NO_COMPRESS", "1");
    wrk.assert_success(&mut cmd);
    let html = wrk.read_to_string("dash.html").unwrap();

    assert!(
        photo_map_trace(&html).is_none(),
        "a .html link column is not photos"
    );
    assert!(
        !html.contains("qsv-photo-box"),
        "no preview chrome for a non-image column"
    );
}

// The hover affordance ("N photos - keep hovering to view") is what makes the 2-second dwell
// discoverable, so it must appear on exactly the points that HAVE photos — never on the ones
// that don't, whose hover stays clean.
#[test]
fn viz_smart_photos_hint_only_on_points_with_photos() {
    let wrk = Workdir::new("viz_smart_photos_hint_only_on_points_with_photos");
    // every 3rd row has photos (and every 6th has two), the rest have none
    let mut rows = String::from("id,lat,lon,photo\n");
    for i in 0..60 {
        let photo = if i % 6 == 0 {
            format!("https://example.org/a{i}.jpg | https://example.org/b{i}.jpg")
        } else if i % 3 == 0 {
            format!("https://example.org/a{i}.jpg")
        } else {
            String::new()
        };
        rows.push_str(&format!(
            "{i},{:.5},-71.05000,{photo}\n",
            40.30000 + 0.00010 * f64::from(i)
        ));
    }
    wrk.create_from_string("pts.csv", &rows);

    let out_html = wrk.path("dash.html").to_string_lossy().to_string();
    let mut cmd = wrk.command("viz");
    cmd.args(["smart", "pts.csv", "--photos", "-o", &out_html]);
    cmd.env("QSV_VIZ_NO_COMPRESS", "1");
    cmd.env_remove("QSV_VIZ_MAX_POINTS");
    wrk.assert_success(&mut cmd);
    let html = wrk.read_to_string("dash.html").unwrap();

    let trace = photo_map_trace(&html).expect("a map trace carrying photo customdata");
    let photos = str_array(&trace, "customdata");
    let hovers = str_array(&trace, "text");
    let with_photos = photos.iter().filter(|p| !p.is_empty()).count();
    let without = photos.iter().filter(|p| p.is_empty()).count();
    assert!(
        with_photos > 0 && without > 0,
        "fixture must mix both kinds"
    );

    // one hint per photo-bearing point, and none anywhere else
    let hinted = hovers
        .iter()
        .filter(|h| h.contains("keep hovering to view"))
        .count();
    assert_eq!(
        hinted, with_photos,
        "the hint must appear on exactly the points that have photos"
    );
    for (photo, hover) in photos.iter().zip(&hovers) {
        if photo.is_empty() {
            assert!(
                !hover.contains("keep hovering"),
                "a photo-less point must keep a clean hover, got: {hover}"
            );
        } else {
            let n = photo.split('|').count();
            let plural = if n == 1 { "photo" } else { "photos" };
            assert!(
                hover.contains(&format!("{n} {plural} - keep hovering to view")),
                "hover must name the photo count ({n}), got: {hover}"
            );
        }
    }

    // the client-side image cache chrome is emitted so a dwelled image is fetched at most once
    // (IndexedDB blob store + in-memory object-URL cache); guards the cache wiring's presence.
    // `qsv-photo-loading` is the neutral box shown while an uncached image resolves, so the card
    // never displays the previous point's photo at the new anchor.
    assert!(
        html.contains("qsv-viz-photos")
            && html.contains("indexedDB")
            && html.contains("qsv-photo-loading"),
        "the --photos chrome must include the IndexedDB image cache and its loading state"
    );
    // the enlarge-in-place toggle (button + size class) must be present
    assert!(
        html.contains("qsv-photo-zoom") && html.contains("qsv-photo-big"),
        "the --photos chrome must include the enlarge toggle"
    );
}

#[test]
fn viz_smart_heatmap_density_threshold() {
    let wrk = Workdir::new("viz_smart_heatmap_density_threshold");
    // a small, locally-clustered lat/lon dataset so smart renders a MapLibre tile map (not the
    // global ScatterGeo world-overview), where the heatmap-vs-markers decision applies.
    wrk.create_from_string(
        "local_geo.csv",
        "id,lat,lon,val\n1,40.440,-79.990,a\n2,40.441,-79.991,b\n3,40.442,-79.992,c\n4,40.443,-79.\
         993,d\n",
    );

    // --heatmap-density 0 => always individual markers (full per-point hover), never a heatmap
    let mut cmd = wrk.command("viz");
    cmd.args(["smart", "local_geo.csv", "--heatmap-density", "0"]);
    let out = wrk.output(&mut cmd);
    assert!(out.status.success());
    let html = String::from_utf8_lossy(&out.stdout);
    assert!(html.contains(r#""type":"scattermap""#));
    assert!(!html.contains(r#""type":"densitymap""#));
    // only 4 points (< SMART_CLUSTER_MIN_POINTS) => sparse map, no clustering
    assert!(!html.contains(r#""cluster":{"enabled":true"#));

    // a low threshold (<= point count) => draw the core cluster as a density heatmap, and emit the
    // explanatory note (per-point hover unavailable in heatmap mode) to stderr.
    let mut cmd = wrk.command("viz");
    cmd.args(["smart", "local_geo.csv", "--heatmap-density", "2"]);
    let out = wrk.output(&mut cmd);
    assert!(out.status.success());
    let html = String::from_utf8_lossy(&out.stdout);
    assert!(html.contains(r#""type":"densitymap""#));
    let stderr = String::from_utf8_lossy(&out.stderr);
    assert!(stderr.contains("--heatmap-density 2"));
}

/// Write a locally-clustered lat/lon CSV of `n` points (all within a ~0.04x0.03 deg box near
/// downtown Pittsburgh, so they stay CORE points on a single MapLibre tile map, never spatial
/// outliers or a global `ScatterGeo` overview). Columns: `id,lat,lon,val`.
fn dense_local_geo(wrk: &Workdir, name: &str, n: usize) {
    let mut rows = String::from("id,lat,lon,val\n");
    for r in 0..n {
        let lat = 40.440 + (r % 40) as f64 * 0.001;
        let lon = -79.990 + (r / 40) as f64 * 0.001;
        rows.push_str(&format!("{},{lat:.4},{lon:.4},{}\n", r + 1, r % 7));
    }
    wrk.create_from_string(name, &rows);
}

// A dense LOCAL smart map (>= SMART_CLUSTER_MIN_POINTS) is cluster-eligible by default (no
// --heatmap-density): the `scattermap` core bakes `cluster.enabled=false` (opens on individual
// points, with a basemap-safe maxzoom for when clustering is toggled on), and it is NOT a density
// heatmap. An explanatory note about the "Clusters" toggle is emitted to stderr.
#[test]
fn viz_smart_map_clusters_dense_markers() {
    let wrk = Workdir::new("viz_smart_map_clusters_dense_markers");
    dense_local_geo(&wrk, "dense.csv", 1200);

    let mut cmd = wrk.command("viz");
    cmd.args(["smart", "dense.csv"]);
    // keep the map figure as scrapable plain JSON (the default gzip-embeds it)
    cmd.env("QSV_VIZ_NO_COMPRESS", "1");
    let out = wrk.output(&mut cmd);
    assert!(out.status.success());
    let html = String::from_utf8_lossy(&out.stdout);
    assert!(html.contains(r#""type":"scattermap""#));
    assert!(
        html.contains(r#""cluster":{"enabled":false,"maxzoom":17.0}"#),
        "dense map should bake clustering disabled-at-load (basemap-safe maxzoom): {html}"
    );
    assert!(!html.contains(r#""type":"densitymap""#));
    let stderr = String::from_utf8_lossy(&out.stderr);
    assert!(stderr.contains("Clusters") && stderr.contains("toggle"));
}

// A cluster-eligible map also ships the client-side cluster UI: plotly renders the count bubble
// labels with an EMPTY paint (so MapLibre's default black text-color applies, unreadable on a dark
// bubble) and gives the bubbles no hover at all, so viz repaints the `-cluster-count` layer and
// installs its own hover/click-to-expand handlers on the GL instance.
#[test]
fn viz_smart_map_cluster_ui_script() {
    let wrk = Workdir::new("viz_smart_map_cluster_ui_script");
    dense_local_geo(&wrk, "dense.csv", 1200);

    let mut cmd = wrk.command("viz");
    cmd.args(["smart", "dense.csv"]);
    cmd.env("QSV_VIZ_NO_COMPRESS", "1");
    let out = wrk.output(&mut cmd);
    assert!(out.status.success());
    let html = String::from_utf8_lossy(&out.stdout);
    // the bubbles are repainted as a sequential ramp keyed to point_count, with the label ink
    // picked per step for contrast (plotly exposes no cluster.textfont attribute at all)
    assert!(
        html.contains("-cluster-count") && html.contains("CLUSTER_SCHEME"),
        "cluster bubbles should carry the contrast-validated ramp: {html}"
    );
    // both basemap ramps ship: the dark one is deliberately paler so a bubble still reads
    assert!(
        html.contains("#86b6ef") && html.contains("#cde2fb"),
        "light and dark cluster ramps should both be present: {html}"
    );
    // a surface ring keeps a bubble legible over a busy patch of basemap
    assert!(
        html.contains("circle-stroke-color"),
        "cluster ring missing: {html}"
    );
    // hover + click-to-expand handlers, and the re-install hook the theme toggle calls
    assert!(
        html.contains("getClusterExpansionZoom"),
        "cluster click-to-expand missing: {html}"
    );
    assert!(
        html.contains("__qsvRefitClusterUi"),
        "cluster UI re-install hook missing: {html}"
    );
    // the note tells the reader the bubbles are interactive
    let stderr = String::from_utf8_lossy(&out.stderr);
    assert!(
        stderr.contains("Hover a bubble"),
        "stderr note should mention hover: {stderr}"
    );
}

// Opting back into the density heatmap (`--heatmap-density` at/below the point count) suppresses
// clustering — a heatmap has no markers to cluster, so the two are mutually exclusive.
#[test]
fn viz_smart_map_heatmap_opt_in_suppresses_cluster() {
    let wrk = Workdir::new("viz_smart_map_heatmap_opt_in_suppresses_cluster");
    dense_local_geo(&wrk, "dense.csv", 1200);

    let mut cmd = wrk.command("viz");
    cmd.args(["smart", "dense.csv", "--heatmap-density", "500"]);
    // keep the map figure as scrapable plain JSON (the default gzip-embeds it)
    cmd.env("QSV_VIZ_NO_COMPRESS", "1");
    let out = wrk.output(&mut cmd);
    assert!(out.status.success());
    let html = String::from_utf8_lossy(&out.stdout);
    assert!(html.contains(r#""type":"densitymap""#));
    assert!(
        !html.contains(r#""cluster":{"enabled":true"#),
        "density heatmap mode must not also cluster: {html}"
    );
}

// A bubble-size encoding (dictionary-tagged map measure under --smarter) suppresses clustering even
// on a dense map: a cluster bubble's size encodes point COUNT, which would conflict with the
// measure-driven marker size.
#[test]
fn viz_smart_map_bubble_sizes_suppress_cluster() {
    let wrk = Workdir::new("viz_smart_map_bubble_sizes_suppress_cluster");
    dense_local_geo(&wrk, "dense.csv", 1200);
    wrk.create_from_string(
        "dict.schema.json",
        r#"{
          "$schema": "https://json-schema.org/draft/2020-12/schema",
          "type": "object",
          "properties": {
            "id": { "type": "string",
              "x-qsv": { "qsv_type": "String", "role": "identifier", "concept": "id.natural_key" } },
            "lat": { "type": "number", "x-qsv": { "qsv_type": "Float", "concept": "geo.latitude" } },
            "lon": { "type": "number", "x-qsv": { "qsv_type": "Float", "concept": "geo.longitude" } },
            "val": { "type": "number", "title": "Value",
              "x-qsv": { "qsv_type": "Float", "role": "measure", "concept": "measure.amount" } }
          }
        }"#,
    );

    let mut cmd = wrk.command("viz");
    cmd.args(["smart", "--smarter", "dense.csv", "--dictionary"])
        .arg(wrk.path("dict.schema.json"));
    // keep the map figure as scrapable plain JSON (the default gzip-embeds it)
    cmd.env("QSV_VIZ_NO_COMPRESS", "1");
    let out = wrk.output(&mut cmd);
    assert!(out.status.success());
    let html = String::from_utf8_lossy(&out.stdout);
    // local extent -> MapLibre tile map with per-point bubble sizes ...
    assert!(html.contains(r#""type":"scattermap""#));
    assert!(
        html.contains(r#""size":["#),
        "map points should be bubble-sized by the dictionary measure: {html}"
    );
    // ... but no clustering, since the bubble size already encodes the measure
    assert!(
        !html.contains(r#""cluster":{"enabled":true"#),
        "bubble-sized map must not cluster: {html}"
    );
}

// `--cluster off` is the escape hatch back to plain dense markers: a dense LOCAL map that would
// cluster by default draws every point as an un-clustered `scattermap` marker instead (and no
// density heatmap either).
#[test]
fn viz_smart_map_cluster_off_draws_plain_markers() {
    let wrk = Workdir::new("viz_smart_map_cluster_off_draws_plain_markers");
    dense_local_geo(&wrk, "dense.csv", 1200);

    let mut cmd = wrk.command("viz");
    cmd.args(["smart", "dense.csv", "--cluster", "off"]);
    // keep the map figure as scrapable plain JSON (the default gzip-embeds it)
    cmd.env("QSV_VIZ_NO_COMPRESS", "1");
    let out = wrk.output(&mut cmd);
    assert!(out.status.success());
    let html = String::from_utf8_lossy(&out.stdout);
    assert!(html.contains(r#""type":"scattermap""#));
    assert!(
        !html.contains(r#""cluster":{"enabled":true"#),
        "--cluster off must draw plain markers, not clusters: {html}"
    );
    assert!(!html.contains(r#""type":"densitymap""#));
}

// `--cluster on` forces clustering even below SMART_CLUSTER_MIN_POINTS: a SPARSE local map (which
// `auto` would leave un-clustered) clusters when the user explicitly opts in.
#[test]
fn viz_smart_map_cluster_on_forces_below_threshold() {
    let wrk = Workdir::new("viz_smart_map_cluster_on_forces_below_threshold");
    // well under SMART_CLUSTER_MIN_POINTS (1,000), so `auto` would NOT cluster
    dense_local_geo(&wrk, "sparse.csv", 60);

    let mut cmd = wrk.command("viz");
    cmd.args(["smart", "sparse.csv", "--cluster", "on"]);
    let out = wrk.output(&mut cmd);
    assert!(out.status.success());
    let html = String::from_utf8_lossy(&out.stdout);
    assert!(html.contains(r#""type":"scattermap""#));
    assert!(
        html.contains(r#""cluster":{"enabled":false,"maxzoom":17.0}"#),
        "--cluster on must make even a sparse map cluster-eligible (toggle available): {html}"
    );
    // the toggle button is present so the user can switch clustering on
    assert!(html.contains(r#"{"cluster.enabled":true},[0]]"#));
}

// A cluster-eligible smart map carries an in-map single "Clusters" toggle button: an updatemenu
// with one restyle button whose `args` (first click) enables and `args2` (next click) disables
// `cluster.enabled` on the core trace (index 0), so a viewer can collapse points into clusters and
// back without re-rendering. Absent when clustering is off (nothing to toggle).
#[test]
fn viz_smart_map_cluster_toggle_menu_present() {
    let wrk = Workdir::new("viz_smart_map_cluster_toggle_menu_present");
    dense_local_geo(&wrk, "dense.csv", 1200);

    // cluster-eligible (default): a single toggle button targeting trace 0 via args/args2 restyle.
    let mut cmd = wrk.command("viz");
    cmd.args(["smart", "dense.csv"]);
    // keep the map figure as scrapable plain JSON (the default gzip-embeds it)
    cmd.env("QSV_VIZ_NO_COMPRESS", "1");
    let out = wrk.output(&mut cmd);
    assert!(out.status.success());
    let html = String::from_utf8_lossy(&out.stdout);
    // match the restyle button's full args fragments (targeting trace [0]) rather than the bare
    // "cluster.enabled" — that attribute name also appears inside the embedded plotly.js bundle.
    // `args` enables, `args2` disables: a native single-button plotly toggle.
    assert!(
        html.contains(r#""args":[{"cluster.enabled":true},[0]]"#)
            && html.contains(r#""args2":[{"cluster.enabled":false},[0]]"#),
        "clustered map should carry the single-button cluster toggle (args/args2 on trace 0): \
         {html}"
    );
    // one "Clusters/Points" toggle button, not a separate pair of "Clusters"/"Points" buttons
    assert!(
        html.contains(r#""label":"Clusters/Points""#)
            && !html.contains(r#""label":"Points""#)
            && !html.contains(r#""label":"Clusters""#)
    );

    // --cluster off: no clustering, so no toggle menu.
    let mut cmd = wrk.command("viz");
    cmd.args(["smart", "dense.csv", "--cluster", "off"]);
    let out = wrk.output(&mut cmd);
    assert!(out.status.success());
    let html = String::from_utf8_lossy(&out.stdout);
    assert!(
        !html.contains(r#"{"cluster.enabled":true},[0]]"#),
        "un-clustered map must not carry the cluster-toggle updatemenu: {html}"
    );
}

// An unrecognized `--cluster` value fails fast with an actionable usage error.
#[test]
fn viz_smart_map_cluster_invalid_mode_errors() {
    let wrk = Workdir::new("viz_smart_map_cluster_invalid_mode_errors");
    dense_local_geo(&wrk, "dense.csv", 60);

    let mut cmd = wrk.command("viz");
    cmd.args(["smart", "dense.csv", "--cluster", "sometimes"]);
    let out = wrk.output(&mut cmd);
    assert!(!out.status.success());
    let stderr = String::from_utf8_lossy(&out.stderr);
    assert!(
        stderr.contains("Unknown --cluster 'sometimes'"),
        "expected an actionable --cluster error: {stderr}"
    );
}

// A user `--geojson` on a LOCAL `viz smart` map overlays the region boundaries + labels on the
// MapLibre tile map: a single gap-separated `scattermap` line trace named "regions", plus a
// "region labels" trace of centroid HOVER markers (the raster basemap culls on-map text, so the
// region name is delivered on hover of a dot instead of as a visible glyph).
#[test]
fn viz_smart_geojson_overlay() {
    let wrk = Workdir::new("viz_smart_geojson_overlay");
    // locally-clustered points so smart renders a MapLibre tile map (not the global ScatterGeo)
    wrk.create_from_string(
        "local_geo.csv",
        "id,lat,lon,val\n1,40.46,-79.98,a\n2,40.47,-79.97,b\n3,40.46,-79.92,c\n4,40.47,-79.91,d\n",
    );
    // two adjacent wards straddling the cluster, each with a human-readable name
    wrk.create_from_string(
        "wards.geojson",
        r#"{"type":"FeatureCollection","features":[{"type":"Feature","properties":{"id":"N","name":"North Ward"},"geometry":{"type":"Polygon","coordinates":[[[-80.0,40.44],[-80.0,40.50],[-79.95,40.50],[-79.95,40.44],[-80.0,40.44]]]}},{"type":"Feature","properties":{"id":"S","name":"South Ward"},"geometry":{"type":"Polygon","coordinates":[[[-79.95,40.44],[-79.95,40.50],[-79.90,40.50],[-79.90,40.44],[-79.95,40.44]]]}}]}"#,
    );

    let mut cmd = wrk.command("viz");
    cmd.args([
        "smart",
        "local_geo.csv",
        "--geojson",
        "wards.geojson",
        "--feature-id-key",
        "properties.id",
        "--feature-name-key",
        "properties.name",
    ]);
    let out = wrk.output(&mut cmd);
    assert!(out.status.success());
    let html = String::from_utf8_lossy(&out.stdout);
    // local MapLibre map with the boundary + label overlay traces
    assert!(html.contains(r#""type":"scattermap""#));
    assert!(html.contains(r#""name":"regions""#));
    assert!(html.contains(r#""name":"region labels""#));
    // labels ride in hover text (the basemap culls on-map glyphs), carrying the --feature-name-key
    // values rather than the ids
    assert!(html.contains(r#""hovertext":["North Ward","South Ward"]"#));
}

// The same `--geojson` overlay on a GLOBE-spanning `viz smart` map renders on the offline
// `ScatterGeo` projection, where on-map text glyphs ARE reliable: a "regions" boundary line trace
// plus a "region labels" `text`-mode trace carrying the visible feature names.
#[test]
fn viz_smart_geojson_overlay_global_geo() {
    let wrk = Workdir::new("viz_smart_geojson_overlay_global_geo");
    // points on multiple continents force the global ScatterGeo world-overview path
    wrk.create_from_string(
        "global_geo.csv",
        "id,lat,lon,val\n1,40.44,-79.99,a\n2,48.85,2.35,b\n3,-33.86,151.20,c\n4,35.68,139.69,d\n",
    );
    wrk.create_from_string(
        "wards.geojson",
        r#"{"type":"FeatureCollection","features":[{"type":"Feature","properties":{"id":"N","name":"North Ward"},"geometry":{"type":"Polygon","coordinates":[[[-80.0,40.44],[-80.0,40.50],[-79.95,40.50],[-79.95,40.44],[-80.0,40.44]]]}},{"type":"Feature","properties":{"id":"S","name":"South Ward"},"geometry":{"type":"Polygon","coordinates":[[[-79.95,40.44],[-79.95,40.50],[-79.90,40.50],[-79.90,40.44],[-79.95,40.44]]]}}]}"#,
    );

    let mut cmd = wrk.command("viz");
    cmd.args([
        "smart",
        "global_geo.csv",
        "--geojson",
        "wards.geojson",
        "--feature-id-key",
        "properties.id",
        "--feature-name-key",
        "properties.name",
    ]);
    let out = wrk.output(&mut cmd);
    assert!(out.status.success());
    let html = String::from_utf8_lossy(&out.stdout);
    // global map => ScatterGeo overlay with VISIBLE on-map text labels
    assert!(html.contains(r#""type":"scattergeo""#));
    assert!(html.contains(r#""name":"regions""#));
    assert!(html.contains(r#""name":"region labels""#));
    assert!(html.contains(r#""mode":"text""#));
    assert!(html.contains(r#""text":["North Ward","South Ward"]"#));
}

// A globe-spanning dataset renders the smart map as an offline ScatterGeo world-overview, NOT a
// DensityMap — so even with a low --heatmap-density threshold, no heatmap is drawn and the
// heatmap note must NOT be emitted (it would misdescribe the ScatterGeo markers as a heatmap).
#[test]
fn viz_smart_heatmap_density_note_suppressed_for_global_extent() {
    let wrk = Workdir::new("viz_smart_heatmap_density_note_suppressed_for_global_extent");
    quakes(&wrk);

    let mut cmd = wrk.command("viz");
    cmd.args(["smart", "quakes.csv", "--heatmap-density", "2"]);
    let out = wrk.output(&mut cmd);
    assert!(out.status.success());

    let html = String::from_utf8_lossy(&out.stdout);
    assert!(html.contains(r#""type":"scattergeo""#));
    assert!(!html.contains(r#""type":"densitymap""#));
    let stderr = String::from_utf8_lossy(&out.stderr);
    assert!(!stderr.contains("--heatmap-density"));
}

// The heatmap note must report the FULL mappable point count that drove the density decision, not
// the downsampled/outlier-excluded count carried on the rendered panel. With more points than the
// internal MAX_SMART_POINTS cap (50_000), the panel's coordinates are downsampled to 50_000, so a
// note sourced from the panel would print a contradictory "50000 >= --heatmap-density 55000".
#[test]
fn viz_smart_heatmap_density_note_reports_full_count() {
    let wrk = Workdir::new("viz_smart_heatmap_density_note_reports_full_count");
    // 60_000 tightly-clustered local points (span ~0.01°) so the map renders as a local MapLibre
    // map heatmap and every point is a core point (no outlier split), exceeding the 50_000 cap.
    let mut csv = String::from("id,lat,lon,val\n");
    for i in 0..60_000u32 {
        let jitter = f64::from(i % 100) * 0.0001;
        csv.push_str(&format!(
            "{i},{:.4},{:.4},a\n",
            40.44 + jitter,
            -79.99 - jitter
        ));
    }
    wrk.create_from_string("dense_local.csv", &csv);

    // threshold between the downsampled count (50_000) and the full count (60_000): density mode
    // engages, and the note must report 60_000 (the source count), never 50_000.
    let mut cmd = wrk.command("viz");
    cmd.args(["smart", "dense_local.csv", "--heatmap-density", "55000"]);
    let out = wrk.output(&mut cmd);
    assert!(out.status.success());
    let stderr = String::from_utf8_lossy(&out.stderr);
    assert!(stderr.contains("map has 60000 mappable points (>= --heatmap-density 55000)"));
    assert!(!stderr.contains("map has 50000 mappable points"));
}

// `viz smart` adds the row identifier (here the near-unique `place` column) to each map point's
// hover, in addition to the coordinates. Dataset-derived, so it holds whether or not the geocode
// index is available.
#[test]
fn viz_smart_geo_hover_has_identifier() {
    let wrk = Workdir::new("viz_smart_geo_hover_has_identifier");
    quakes(&wrk);

    let mut cmd = wrk.command("viz");
    cmd.args(["smart", "quakes.csv"]);
    let out = wrk.output(&mut cmd);
    assert!(out.status.success());

    let html = String::from_utf8_lossy(&out.stdout);
    assert!(
        html.contains(r#""hovertemplate":"%{text}%{lat:.4f}, %{lon:.4f}"#),
        "map points should carry per-point text + the trace-level coordinate hovertemplate"
    );
    // the identifier value is the `place` column (near-unique String)
    assert!(html.contains("Tokyo"), "hover should name the point");
}

// `--smarter` (no dictionary) enriches the hover with a statistical combination: identifier (the
// near-unique `city`) + a numeric measure (`depth_km`, with repeated values so it isn't ID-like) +
// a low-cardinality category (`zone`). Uses a dedicated fixture with repeats so a measure
// qualifies.
#[test]
fn viz_smart_smarter_statistical_combination() {
    let wrk = Workdir::new("viz_smart_smarter_statistical_combination");
    wrk.create_from_string(
        "events.csv",
        "city,lat,lon,depth_km,zone
Tokyo,35.68,139.69,30,Asia
London,51.51,-0.13,20,Europe
NewYork,40.71,-74.01,30,Americas
Sydney,-33.87,151.21,20,Oceania
Lima,-12.04,-77.04,30,Americas
Cairo,30.04,31.24,20,Africa
Paris,48.85,2.35,30,Europe
Nairobi,-1.29,36.82,20,Africa
Delhi,28.61,77.21,30,Asia
Bogota,4.71,-74.07,20,Americas
Oslo,59.91,10.75,30,Europe
Accra,5.56,-0.20,20,Africa
",
    );

    let mut cmd = wrk.command("viz");
    cmd.args(["smart", "--smarter", "events.csv"]);
    let out = wrk.output(&mut cmd);
    assert!(out.status.success());

    let html = String::from_utf8_lossy(&out.stdout);
    assert!(html.contains(r#""hovertemplate":"%{text}%{lat:.4f}, %{lon:.4f}"#));
    // the statistically-chosen measure and category appear as labeled hover lines
    assert!(
        html.contains("depth_km: "),
        "smarter hover should include a numeric measure"
    );
    assert!(
        html.contains("zone: "),
        "smarter hover should include a low-cardinality category"
    );
}

// Regression: when `--smarter` finds no identifier column, the chosen measure/category must be
// rendered as labeled extras ("temperature: 22"), NOT promoted to the bold point identifier. A
// dataset with only repeated numeric/category columns (no near-unique key) exercises this path.
#[test]
fn viz_smart_smarter_no_identifier_keeps_measure_as_extra() {
    let wrk = Workdir::new("viz_smart_smarter_no_identifier_keeps_measure_as_extra");
    wrk.create_from_string(
        "obs.csv",
        "lat,lon,temperature,zone
35.68,139.69,22,A
51.51,-0.13,15,B
40.71,-74.01,22,A
-33.87,151.21,15,B
-1.29,36.82,22,A
59.91,10.75,15,B
",
    );

    let mut cmd = wrk.command("viz");
    cmd.args(["smart", "--smarter", "obs.csv"]);
    let out = wrk.output(&mut cmd);
    assert!(out.status.success());

    let html = String::from_utf8_lossy(&out.stdout);
    assert!(html.contains(r#""hovertemplate":"%{text}%{lat:.4f}, %{lon:.4f}"#));
    // the measure appears as a labeled extra line; before the fix it was the bold "<b>22</b>" id,
    // so this label line would be absent.
    assert!(
        html.contains("temperature: "),
        "no-identifier measure must be a labeled extra, not the bold id"
    );
    assert!(html.contains("zone: "));
}

// `--smarter --dictionary` drives the combination from the dictionary: the friendly label and the
// concept-chosen measure (magnitude, tagged measure.amount) appear in the hover.
#[test]
fn viz_smart_smarter_dictionary_combination() {
    let wrk = Workdir::new("viz_smart_smarter_dictionary_combination");
    quakes(&wrk);
    wrk.create_from_string(
        "dict.schema.json",
        r#"{
          "$schema": "https://json-schema.org/draft/2020-12/schema",
          "type": "object",
          "properties": {
            "place": { "type": "string", "title": "Quake Site",
              "x-qsv": { "qsv_type": "String", "role": "identifier", "concept": "id.natural_key" } },
            "lat": { "type": "number", "x-qsv": { "qsv_type": "Float", "concept": "geo.latitude" } },
            "lon": { "type": "number", "x-qsv": { "qsv_type": "Float", "concept": "geo.longitude" } },
            "magnitude": { "type": "number", "title": "Magnitude",
              "x-qsv": { "qsv_type": "Float", "role": "measure", "concept": "measure.amount" } },
            "region": { "type": "string", "title": "Region",
              "x-qsv": { "qsv_type": "String", "role": "dimension", "concept": "category.status" } }
          }
        }"#,
    );

    let mut cmd = wrk.command("viz");
    cmd.args(["smart", "--smarter", "quakes.csv", "--dictionary"])
        .arg(wrk.path("dict.schema.json"));
    let out = wrk.output(&mut cmd);
    assert!(out.status.success());

    let html = String::from_utf8_lossy(&out.stdout);
    assert!(html.contains(r#""hovertemplate":"%{text}%{lat:.4f}, %{lon:.4f}"#));
    // the dictionary friendly label titles the measure's hover line
    assert!(
        html.contains("Magnitude: "),
        "dictionary-driven hover should use the friendly measure label"
    );
    assert!(html.contains("Tokyo"));
}

// A coordinates-only dataset (no identifier column) still gets a hover when the geocode index is
// available: the reverse-geocoded place serves as the identifier. Tolerant of an unavailable index
// (offline CI) — it must always render, and only assert the hover wiring when geocoding
// contributed.
// geocode-dependent: without the geocode feature no reverse-geocoded identifier is produced,
// so the hover `"text":[` wiring this asserts never renders.
#[cfg(feature = "geocode")]
#[test]
fn viz_smart_geo_hover_geocoded_identifier() {
    let wrk = Workdir::new("viz_smart_geo_hover_geocoded_identifier");
    wrk.create_from_string(
        "pts.csv",
        "lat,lon
35.68,139.69
51.51,-0.13
40.71,-74.01
-33.87,151.21
",
    );

    let mut cmd = wrk.command("viz");
    cmd.args(["smart", "pts.csv"]);
    let out = wrk.output(&mut cmd);
    assert!(out.status.success());

    let html = String::from_utf8_lossy(&out.stdout);
    // a global lat/lon spread renders the ScatterGeo world overview in every build
    assert!(html.contains(r#""type":"scattergeo""#));
    // with no identifier column, hover text only appears when reverse-geocoding resolved a place
    if html.contains(r#""hovertemplate":"%{text}%{lat:.4f}, %{lon:.4f}"#) {
        assert!(html.contains(r#""text":["#));
    }
}

// Geocoding enrichment: the county is always shown in a US map hover; the US FIPS code appears only
// under --smarter. Tolerant of an unavailable geocode index (offline CI) — the enrichment is only
// asserted when reverse-geocoding actually resolved the county.
#[test]
fn viz_smart_geocode_enrichment_county_and_fips() {
    let wrk = Workdir::new("viz_smart_geocode_enrichment_county_and_fips");
    // a tight Allegheny County, PA cluster -> a local MapLibre map with per-point hovers
    wrk.create_from_string(
        "pitt.csv",
        "id,lat,lon,val
Pittsburgh,40.4406,-79.9959,10
McKeesport,40.3487,-79.8642,20
BethelPark,40.3273,-80.0373,30
Monroeville,40.4212,-79.7883,40
Wilkinsburg,40.4445,-79.8811,50
",
    );

    // plain: county appears (when geocoding resolved); the FIPS tail does NOT (it's --smarter-only)
    let mut cmd = wrk.command("viz");
    cmd.args(["smart", "pitt.csv"]);
    let out = wrk.output(&mut cmd);
    assert!(out.status.success());
    let plain = String::from_utf8_lossy(&out.stdout);
    let geocoded = plain.contains("Allegheny County");
    if geocoded {
        assert!(!plain.contains("(FIPS "));
    }

    // --smarter: when the county resolved, the combined 5-digit county FIPS tail is present
    let mut cmd = wrk.command("viz");
    cmd.args(["smart", "pitt.csv", "--smarter"]);
    let out = wrk.output(&mut cmd);
    assert!(out.status.success());
    let smart = String::from_utf8_lossy(&out.stdout);
    if smart.contains("Allegheny County") {
        assert!(smart.contains("(FIPS 42003)"));
    }
}

// The --smarter country-context continent note must reflect EXACTLY the summarized extent: a US
// core with a European (Paris) outlier spans two continents, so the note must be suppressed rather
// than claiming a single continent. Tolerant of an unavailable geocode index (offline CI).
#[test]
fn viz_smart_country_context_suppressed_across_continents() {
    let wrk = Workdir::new("viz_smart_country_context_suppressed_across_continents");
    // a tight US core plus one far outlier in France
    wrk.create_from_string(
        "mixed.csv",
        "id,lat,lon,val
Pittsburgh,40.4406,-79.9959,10
McKeesport,40.3487,-79.8642,20
BethelPark,40.3273,-80.0373,30
Monroeville,40.4212,-79.7883,40
Wilkinsburg,40.4445,-79.8811,50
Paris,48.8566,2.3522,999
",
    );

    let mut cmd = wrk.command("viz");
    cmd.args(["smart", "mixed.csv", "--smarter"]);
    let out = wrk.output(&mut cmd);
    assert!(out.status.success());
    let html = String::from_utf8_lossy(&out.stdout);
    // when geocoding resolved the US core, the outlier call-out is present but NO single-continent
    // annotation may follow (the extent spans North America AND Europe).
    if html.contains("Pennsylvania") && html.contains("outlier") {
        assert!(!html.contains(" · North America"));
        assert!(!html.contains(" · Europe"));
    }
}

// Regression for the removed "all geo fields supplied" hover-geocode skip: even when the dataset
// already exposes dictionary-recognized geo.city / geo.state / geo.country columns, the always-on
// county and --smarter FIPS enrichment must still be added (the dataset's own city/country values
// are deduped, but county/FIPS are net-new). A dictionary makes the geo concepts deterministic (no
// LLM). Anchored on an independent control run WITHOUT the geo columns so the county assertion
// can't pass vacuously: the control proves the geocode index resolves these coordinates (its county
// can ONLY come from geocoding), and whenever it does, the geo-column run must resolve the county
// and FIPS too. Tolerant of an unavailable geocode index (offline CI) — both gate on the control.
#[test]
fn viz_smart_geocode_enrichment_with_geo_columns() {
    let wrk = Workdir::new("viz_smart_geocode_enrichment_with_geo_columns");

    // control: identical coordinates, NO geo-name columns — the county can only be geocoded, so its
    // presence establishes that the index is available for these points.
    wrk.create_from_string(
        "control.csv",
        "id,lat,lon,val
a,40.4406,-79.9959,10
b,40.3487,-79.8642,20
c,40.3273,-80.0373,30
d,40.4212,-79.7883,40
",
    );
    let mut cmd = wrk.command("viz");
    cmd.args(["smart", "--smarter", "control.csv"]);
    let control = wrk.output(&mut cmd);
    assert!(control.status.success());
    let index_resolves = String::from_utf8_lossy(&control.stdout).contains("Allegheny County");

    // main run: city/state/country are recognized (dictionary-tagged) geo concepts.
    wrk.create_from_string(
        "geo_cols.csv",
        "id,city,state,country,lat,lon,val
a,Pittsburgh,Pennsylvania,United States,40.4406,-79.9959,10
b,McKeesport,Pennsylvania,United States,40.3487,-79.8642,20
c,Bethel Park,Pennsylvania,United States,40.3273,-80.0373,30
d,Monroeville,Pennsylvania,United States,40.4212,-79.7883,40
",
    );
    wrk.create_from_string(
        "geo_dict.schema.json",
        r#"{
          "$schema": "https://json-schema.org/draft/2020-12/schema",
          "type": "object",
          "properties": {
            "id": { "type": "string", "x-qsv": { "qsv_type": "String", "role": "identifier", "concept": "id.natural_key" } },
            "city": { "type": "string", "x-qsv": { "qsv_type": "String", "concept": "geo.city" } },
            "state": { "type": "string", "x-qsv": { "qsv_type": "String", "concept": "geo.state" } },
            "country": { "type": "string", "x-qsv": { "qsv_type": "String", "concept": "geo.country" } },
            "lat": { "type": "number", "x-qsv": { "qsv_type": "Float", "concept": "geo.latitude" } },
            "lon": { "type": "number", "x-qsv": { "qsv_type": "Float", "concept": "geo.longitude" } },
            "val": { "type": "number", "x-qsv": { "qsv_type": "Integer", "role": "measure", "concept": "measure.amount" } }
          }
        }"#,
    );
    let mut cmd = wrk.command("viz");
    cmd.args(["smart", "--smarter", "geo_cols.csv", "--dictionary"])
        .arg(wrk.path("geo_dict.schema.json"));
    let out = wrk.output(&mut cmd);
    assert!(out.status.success());
    let html = String::from_utf8_lossy(&out.stdout);

    // gated on the CONTROL (not the main run's own output): whenever geocoding resolves the county,
    // the geo-column run MUST still surface county + FIPS despite already carrying
    // city/state/country.
    if index_resolves {
        assert!(
            html.contains("Allegheny County"),
            "county enrichment dropped when the dataset already has geo columns"
        );
        assert!(
            html.contains("(FIPS 42003)"),
            "FIPS enrichment dropped when the dataset already has geo columns"
        );
    }
}

// A geo overview whose points all fall within a single plotly continent box is framed to that
// continent's geo `scope` (aligning with plotly.js's layout.geo.scope vocabulary) instead of
// showing the whole world. The African cities span ~64 deg of latitude (so the panel renders as
// the ScatterGeo world overview, not a zoomed tile map) yet all sit inside the "africa" box.
#[test]
fn viz_smart_geo_panel_scopes_to_single_continent() {
    let wrk = Workdir::new("viz_smart_geo_panel_scopes_to_single_continent");
    wrk.create_from_string(
        "africa.csv",
        "city,lat,lon
Cairo,30.06,31.25
Cape Town,-33.92,18.42
Lagos,6.45,3.39
Nairobi,-1.29,36.82
Casablanca,33.57,-7.59
Kinshasa,-4.33,15.31
",
    );
    let mut cmd = wrk.command("viz");
    cmd.args(["smart", "africa.csv"]);
    let out = wrk.output(&mut cmd);
    assert!(out.status.success());
    let html = String::from_utf8_lossy(&out.stdout);
    assert!(
        html.contains(r#""type":"scattergeo""#),
        "a ~64 deg latitude extent renders as the world-overview geo panel"
    );
    assert!(
        html.contains(r#""scope":"africa""#),
        "an all-Africa extent frames the geo panel to the africa scope"
    );
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
        html.contains(r#""type":"violin""#),
        "zone should be a violin (the default distribution panel) without a dictionary"
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
        !html.contains(r#""type":"box""#) && !html.contains(r#""type":"violin""#),
        "zone should be a bar (not a distribution panel) with the dictionary"
    );
    assert!(html.contains(r#""type":"bar""#));
}

#[test]
fn viz_smart_dictionary_measure_typed_string_is_explained() {
    // A dictionary-declared `measure` that stats typed String has no quartiles, so
    // `classify_measure` drops it. Without attribution it lands in the generic skip list next to
    // genuine ID columns. Assert the two causes are named — and told apart:
    //   `depth` : numeric range + a "NULL" sentinel  -> one parsing endpoint  -> sentinel suspect
    //   `grade` : genuinely non-numeric content      -> no parsing endpoint   -> mis-roled
    // `status` is role=dimension and ALSO contains "NULL"; it must appear in neither note.
    let wrk = Workdir::new("viz_smart_dictionary_measure_typed_string_is_explained");
    let mut rows = String::from("depth,grade,status\n");
    for i in 0..60 {
        let depth = if i % 5 == 0 {
            "NULL".to_string()
        } else {
            (i + 1).to_string()
        };
        let grade = match i % 3 {
            0 => "NULL",
            1 => "low",
            _ => "high",
        };
        let status = if i % 4 == 0 { "NULL" } else { "Open" };
        rows.push_str(&format!("{depth},{grade},{status}\n"));
    }
    wrk.create_from_string("sentinel.csv", &rows);

    wrk.create_from_string(
        "dict.json",
        r#"{"properties":{
            "depth":  {"type":["string"],"x-qsv":{"role":"measure"}},
            "grade":  {"type":["string"],"x-qsv":{"role":"measure"}},
            "status": {"type":["string"],"x-qsv":{"role":"dimension"}}
        }}"#,
    );

    let mut cmd = wrk.command("viz");
    cmd.args(["smart", "sentinel.csv", "--dictionary"])
        .arg(wrk.path("dict.json"));
    let out = wrk.output(&mut cmd);
    assert!(out.status.success());
    let stderr = String::from_utf8_lossy(&out.stderr);

    // `depth` is diagnosed as a probable null sentinel, with an actionable next step.
    assert!(
        stderr.contains("most often a null sentinel"),
        "expected the sentinel note, got: {stderr}"
    );
    assert!(
        stderr.contains("qsv denull -s depth"),
        "sentinel note should name `depth` and suggest a check, got: {stderr}"
    );

    // `grade` is NOT reported as a sentinel problem — that would send the user hunting for a
    // value that isn't there. It is reported as a dictionary role/concept mismatch instead.
    assert!(
        stderr.contains("hold non-numeric content") && stderr.contains("grade"),
        "expected `grade` to be reported as mis-roled, got: {stderr}"
    );
    assert!(
        !stderr.contains("qsv frequency -s depth,grade")
            && !stderr.contains("qsv frequency -s grade"),
        "`grade` must not be listed as a sentinel suspect, got: {stderr}"
    );

    // a role=dimension column containing the same "NULL" text is charted, not diagnosed.
    assert!(
        !stderr.contains("status"),
        "`status` (role=dimension) must not appear in either note, got: {stderr}"
    );
}

#[test]
fn viz_smart_without_dictionary_hints_at_denull() {
    // WITHOUT a dictionary there is no `measure` verdict, so `classify` drops a
    // sentinel-bearing numeric column as high-cardinality text. One parsing endpoint
    // (min=1, max=NULL) is suggestive but NOT proof -- an address column with a cell `1`
    // and a cell `Zoo` looks identical. So viz must NOT diagnose per-column here; it may
    // only point at `qsv denull`, which decides by scanning the values.
    let wrk = Workdir::new("viz_smart_without_dictionary_hints_at_denull");
    // `depth`: 64 distinct numbers + "NULL" over 80 rows -> String, cardinality > 30 and
    // uniqueness 0.81 (not near-unique), so `classify` drops it as high-cardinality text.
    // `steady`: a low-cardinality numeric that charts, so the dashboard is not empty.
    let mut rows = String::from("depth,steady\n");
    for i in 0..80 {
        if i % 5 == 0 {
            rows.push_str("NULL,");
        } else {
            rows.push_str(&format!("{},", i + 1));
        }
        rows.push_str(&format!("{}\n", i % 20));
    }
    wrk.create_from_string("d.csv", &rows);

    let out_html = wrk.path("out.html").to_string_lossy().to_string();
    let mut cmd = wrk.command("viz");
    cmd.args(["smart", "d.csv", "-o", &out_html]);
    let out = wrk.output(&mut cmd);
    assert!(out.status.success());
    let stderr = String::from_utf8_lossy(&out.stderr);

    assert!(
        stderr.contains("qsv denull"),
        "a skipped String column with one parsing endpoint should point at denull, got: {stderr}"
    );
    assert!(
        stderr.contains("may be numeric data held back"),
        "the no-dictionary note must hedge, not diagnose, got: {stderr}"
    );
    assert!(
        !stderr.contains("data dictionary"),
        "no dictionary was supplied; the measure-contradiction note must not fire: {stderr}"
    );
    assert!(
        !stderr.contains("steady"),
        "an ordinary numeric column must not be named: {stderr}"
    );
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
        html.contains(r#""type":"violin""#),
        "census_tract should be a violin (distribution) without a dictionary"
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
        !html.contains(r#""type":"box""#) && !html.contains(r#""type":"violin""#),
        "census_tract should be a bar (not a distribution panel) with the jsonschema dictionary"
    );
    assert!(html.contains(r#""type":"bar""#));
    // the human labels from `title` are surfaced (now as the panel subtitle beneath the field
    // name); their presence proves the dictionary was applied.
    assert!(
        html.contains("Census Tract"),
        "dictionary label should appear on the panel"
    );
    assert!(html.contains("Case Status"));
}

// `--dictionary infer` reuses a pre-existing `<stem>.schema.json` sidecar beside the input,
// skipping the LLM entirely. CI has no LLM configured, so an actual infer would soft-fall to the
// stats-only dashboard (census_tract -> box). The dictionary routing (bar, not box) + labels
// therefore prove the sidecar was reused rather than re-inferred. The reuse message on stderr
// also surfaces the model the sidecar was generated with (parsed from x-qsv.generated_by).
#[test]
fn viz_smart_dictionary_infer_reuses_sidecar() {
    let wrk = Workdir::new("viz_smart_dictionary_infer_reuses_sidecar");
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

    // Seed the sidecar at the exact path `--dictionary infer` looks for: `<stem>.schema.json`
    // beside the input. Its x-qsv.generated_by carries a `Model:` line so we can also assert the
    // reuse message reports the model.
    wrk.create_from_string(
        "codes.schema.json",
        r#"{
          "$schema": "https://json-schema.org/draft/2020-12/schema",
          "type": "object",
          "properties": {
            "census_tract": { "type": ["integer","null"], "title": "Census Tract",
              "x-qsv": { "qsv_type": "Integer", "role": "dimension", "concept": "geo.census_tract" } },
            "status": { "type": "string", "title": "Case Status",
              "x-qsv": { "qsv_type": "String", "role": "dimension", "concept": "category.status" } }
          },
          "x-qsv": { "grain": "one row = one service request",
            "generated_by": "Generated by qsv v1.0.0 describegpt\nModel: test-model-xyz\nTimestamp: 2026-01-01T00:00:00Z" }
        }"#,
    );

    let mut cmd = wrk.command("viz");
    cmd.args(["smart", "codes.csv", "--dictionary", "infer"]);
    let out = wrk.output(&mut cmd);
    assert!(out.status.success());
    let html = String::from_utf8_lossy(&out.stdout);
    // reused sidecar -> census_tract routed as a bar (not a box) with its human label
    assert!(
        !html.contains(r#""type":"box""#) && !html.contains(r#""type":"violin""#),
        "census_tract should be a bar (not a distribution panel) when the sidecar dictionary is \
         reused"
    );
    assert!(html.contains(r#""type":"bar""#));
    assert!(
        html.contains("Census Tract"),
        "reused dictionary label should appear on the panel"
    );
    assert!(html.contains("Case Status"));

    // the reuse message (stderr) names the sidecar and the model it was generated with
    let stderr = String::from_utf8_lossy(&out.stderr);
    assert!(
        stderr.contains("reusing existing dictionary"),
        "expected a reuse message on stderr, got: {stderr}"
    );
    assert!(
        stderr.contains("test-model-xyz"),
        "reuse message should report the model from x-qsv.generated_by, got: {stderr}"
    );
}

// The seeded sidecar both QSV_VIZ_DICT_FRESH tests below share: a dictionary whose `Model:` line
// (test-model-xyz) is a marker that can ONLY reach stderr via the reuse path, and whose labels can
// only reach the HTML the same way.
const FRESH_SIDECAR_JSON: &str = r#"{
  "$schema": "https://json-schema.org/draft/2020-12/schema",
  "type": "object",
  "properties": {
    "census_tract": { "type": ["integer","null"], "title": "Census Tract",
      "x-qsv": { "qsv_type": "Integer", "role": "dimension", "concept": "geo.census_tract" } },
    "status": { "type": "string", "title": "Case Status",
      "x-qsv": { "qsv_type": "String", "role": "dimension", "concept": "category.status" } }
  },
  "x-qsv": { "grain": "one row = one service request",
    "generated_by": "Generated by qsv v1.0.0 describegpt\nModel: test-model-xyz\nTimestamp: 2026-01-01T00:00:00Z" }
}"#;

fn fresh_dict_workdir(name: &str) -> Workdir {
    let wrk = Workdir::new(name);
    let mut rows = String::from("census_tract,status\n");
    for i in 0..200 {
        let tract = i % 40;
        let status = match i % 3 {
            0 => "Open",
            1 => "Closed",
            _ => "Pending",
        };
        rows.push_str(&format!("{tract},{status}\n"));
    }
    wrk.create_from_string("codes.csv", &rows);
    wrk.create_from_string("codes.schema.json", FRESH_SIDECAR_JSON);
    wrk
}

// QSV_VIZ_DICT_FRESH=1 must make `--dictionary infer` IGNORE the sidecar it would otherwise reuse.
// Hermetic by construction: QSV_LLM_BASE_URL points at a closed port, so describegpt always fails
// fast and viz soft-falls back to the stats-only dashboard. That keeps the test deterministic and
// quick both in CI (no LLM) and on a dev box that happens to have LM Studio on the default port -
// and it lets us assert that a FAILED infer never clobbers the sidecar.
#[test]
fn viz_smart_dictionary_infer_fresh_env_bypasses_sidecar() {
    let wrk = fresh_dict_workdir("viz_smart_dictionary_infer_fresh_env_bypasses_sidecar");
    let sidecar = wrk.path("codes.schema.json");
    let before = std::fs::read_to_string(&sidecar).unwrap();

    let mut cmd = wrk.command("viz");
    cmd.args(["smart", "codes.csv", "--dictionary", "infer"]);
    cmd.env("QSV_VIZ_DICT_FRESH", "1");
    // deliberately unreachable, so the infer fails fast instead of contacting a real local LLM
    cmd.env("QSV_LLM_BASE_URL", "http://127.0.0.1:9/v1");
    cmd.env("QSV_LLM_APIKEY", "NONE");
    let out = wrk.output(&mut cmd);
    assert!(out.status.success(), "viz should soft-fall back, not fail");

    let stderr = String::from_utf8_lossy(&out.stderr);
    // load-bearing: the reuse path must not have run
    assert!(
        !stderr.contains("reusing existing dictionary"),
        "QSV_VIZ_DICT_FRESH=1 should skip sidecar reuse, got: {stderr}"
    );
    assert!(
        !stderr.contains("test-model-xyz"),
        "the seeded sidecar's model must not be reported when reuse is bypassed, got: {stderr}"
    );

    // a dashboard is still produced (stats-only), and the seeded labels are gone with the reuse
    let html = String::from_utf8_lossy(&out.stdout);
    assert!(html.contains("Plotly.newPlot"));
    assert!(
        !html.contains("Census Tract"),
        "reused-dictionary label should be absent once reuse is bypassed"
    );

    // a FAILED infer must leave the sidecar exactly as it was (only a successful one overwrites)
    assert_eq!(
        std::fs::read_to_string(&sidecar).unwrap(),
        before,
        "a failed re-infer must not clobber the existing sidecar"
    );
}

// The inverse, and the guard against inverted predicate logic: with the var explicitly falsy the
// sidecar is still reused, exactly as when it is unset. No LLM is reachable on this path at all.
#[test]
fn viz_smart_dictionary_infer_reuses_sidecar_when_fresh_off() {
    let wrk = fresh_dict_workdir("viz_smart_dictionary_infer_reuses_sidecar_when_fresh_off");

    let mut cmd = wrk.command("viz");
    cmd.args(["smart", "codes.csv", "--dictionary", "infer"]);
    cmd.env("QSV_VIZ_DICT_FRESH", "0");
    let out = wrk.output(&mut cmd);
    assert!(out.status.success());

    let stderr = String::from_utf8_lossy(&out.stderr);
    assert!(
        stderr.contains("reusing existing dictionary"),
        "a falsy QSV_VIZ_DICT_FRESH must not disable sidecar reuse, got: {stderr}"
    );
    assert!(stderr.contains("test-model-xyz"));
    let html = String::from_utf8_lossy(&out.stdout);
    assert!(
        html.contains("Census Tract"),
        "reused dictionary label should appear on the panel"
    );
}

// viz shows progress on stderr by default and honors QSV_PROGRESSBAR, but progress must NEVER
// contaminate the deliverable (the HTML on stdout). Running the same dashboard with the env var
// on vs. off must yield identical stdout, modulo the by-design build timestamp masked below.
// (Under the test harness's piped stderr, indicatif auto-hides in both cases, so this validates
// the env plumbing + stdout purity.)
#[test]
fn viz_progressbar_env_stdout_unaffected() {
    let wrk = Workdir::new("viz_progressbar_env_stdout_unaffected");
    wrk.create(
        "data.csv",
        vec![
            svec!["category", "value"],
            svec!["a", "10"],
            svec!["b", "20"],
            svec!["a", "30"],
            svec!["c", "40"],
            svec!["b", "50"],
        ],
    );

    let mut on = wrk.command("viz");
    on.args(["smart", "data.csv"]).env("QSV_PROGRESSBAR", "1");
    let out_on = wrk.output(&mut on);
    assert!(out_on.status.success());

    let mut off = wrk.command("viz");
    off.args(["smart", "data.csv"]).env("QSV_PROGRESSBAR", "0");
    let out_off = wrk.output(&mut off);
    assert!(out_off.status.success());

    // The dashboard embeds a minute-granularity "Compiled:" build timestamp, so the two runs
    // differ whenever they straddle a minute boundary - by design, and unrelated to what this
    // test asserts. Mask it out before comparing. Anchored on the timestamp FORMAT, not on the
    // "Compiled:" label, which is localized. Keep in sync with the same normalization in
    // scripts/viz-golden-check.sh:77.
    let ts_re = regex::Regex::new(r"\d{4}-\d{2}-\d{2} \d{2}:\d{2} UTC").unwrap();
    let mask = |raw: &[u8]| -> String {
        let html = String::from_utf8(raw.to_vec()).expect("viz smart stdout must be valid UTF-8");
        // Guard against the normalizer silently becoming a no-op if the timestamp format ever
        // changes - without this, the flake would come back with no signal.
        assert!(
            ts_re.is_match(&html),
            "expected a `YYYY-MM-DD HH:MM UTC` build timestamp in the dashboard HTML - if the \
             format changed, update this test AND scripts/viz-golden-check.sh"
        );
        ts_re.replace_all(&html, "<TIMESTAMP>").into_owned()
    };

    assert_eq!(
        mask(&out_on.stdout),
        mask(&out_off.stdout),
        "QSV_PROGRESSBAR must not change the HTML written to stdout"
    );
}

// The field name titles each per-column panel and the dictionary label becomes a smaller muted
// subtitle beneath it (rather than the label replacing the field name outright).
#[test]
fn viz_smart_dictionary_field_name_title_label_subtitle() {
    let wrk = Workdir::new("viz_smart_dictionary_field_name_title_label_subtitle");
    wrk.create(
        "codes.csv",
        vec![
            svec!["census_tract", "status"],
            svec!["101", "Open"],
            svec!["102", "Closed"],
            svec!["103", "Open"],
            svec!["104", "Closed"],
            svec!["105", "Open"],
        ],
    );
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
          }
        }"#,
    );
    let mut cmd = wrk.command("viz");
    cmd.args(["smart", "codes.csv", "--dictionary"])
        .arg(wrk.path("dict.schema.json"));
    let out = wrk.output(&mut cmd);
    assert!(out.status.success());
    let html = String::from_utf8_lossy(&out.stdout);

    // the field name leads (as the title), with the dictionary label on a smaller muted subtitle
    // line, i.e. "<field><br><span style=...>label</span>". Plotly serializes the title annotation
    // into a JSON string, so `<`/`>`/`"` are escaped as < / > / \". Build the expected
    // markup from those escaped pieces so the assertion matches the on-disk bytes exactly.
    let lt = "\\u003c"; // escaped '<'
    let gt = "\\u003e"; // escaped '>'
    let q = "\\\""; // escaped '"'
    let expected = |field: &str, label: &str| {
        format!(
            "{field}{lt}br{gt}{lt}span \
             style={q}font-size:11px;color:#999999{q}{gt}{label}{lt}/span{gt}"
        )
    };
    let tract = expected("census_tract", "Census Tract");
    assert!(
        html.contains(&tract),
        "expected field-name title + dictionary-label subtitle {tract}; html: {html}"
    );
    let status = expected("status", "Case Status");
    assert!(
        html.contains(&status),
        "expected field-name title + dictionary-label subtitle {status}; html: {html}"
    );
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
    // the file dictionary still routes zone -> bar (not a distribution panel)
    let html = wrk.read_to_string("d.html").unwrap();
    assert!(html.contains(r#""type":"bar""#));
    assert!(!html.contains(r#""type":"box""#) && !html.contains(r#""type":"violin""#));
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
        !html.contains(r#""type":"scattermap""#),
        "no map should render for non-standard coord names without a dictionary"
    );
    assert!(
        html.contains(r#""type":"violin""#),
        "without a dictionary the coords are charted as distributions (violins)"
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
        html.contains(r#""type":"scattermap""#),
        "map should render from the dictionary geo.latitude/geo.longitude tags; html: {html}"
    );
    assert!(
        !html.contains(r#""type":"box""#)
            && !html.contains(r#""type":"violin""#)
            && !html.contains(r#""type":"histogram""#),
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
fn viz_smart_dictionary_grain_unit_localizes_count_title() {
    // Issue #4321. Under `--language pt` the trend title template is Portuguese
    // (`"%{q_unit} ao longo de %{q_date}"`), and `q_unit` comes from the dictionary. Before the
    // structured `x-qsv.grain_unit` field existed, viz parsed the unit out of the English `grain`
    // sentence and interpolated an ENGLISH noun into that Portuguese template, producing a
    // half-translated title. The structured field must be used verbatim instead.
    let wrk = Workdir::new("viz_smart_dictionary_grain_unit_localizes_count_title");
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
            "requested_on": { "type": "string", "title": "Data de Solicitação",
              "x-qsv": { "qsv_type": "Date", "role": "timestamp", "concept": "time.created_at" } },
            "status": { "type": "string", "title": "Situação",
              "x-qsv": { "qsv_type": "String", "role": "dimension", "concept": "category.status" } }
          },
          "x-qsv": { "grain": "uma linha = um pedido de licença",
                     "grain_unit": "pedido de licença" }
        }"#,
    );

    let out_html = wrk.path("dash.html").to_string_lossy().to_string();
    let mut cmd = wrk.command("viz");
    cmd.args(["smart", "permits.csv", "--dictionary"])
        .arg(wrk.path("dict.schema.json"))
        .args(["--language", "pt", "-o", &out_html]);
    wrk.assert_success(&mut cmd);

    let html = wrk.read_to_string("dash.html").unwrap();

    // The FULL title is asserted, not just the unit: the title is entirely determined by
    // `q_unit` + the pt template + `q_date`, so an exact match is itself the proof that no
    // English survived anywhere in the interpolated sentence.
    let title = "pedido de licença ao longo de Data de Solicitação";
    assert!(
        html.contains(title),
        "trend title must interpolate the localized grain_unit into the pt template; html: {html}"
    );
    let axis = "pedido de licença por dia";
    assert!(
        html.contains(axis),
        "trend axis title must use the localized grain_unit and pt bucket word; html: {html}"
    );

    // Regression guards, both scoped to the TITLE rather than the whole document (a bare
    // `html.contains(" over ")` would be vacuous -- unrelated English tokens live in the
    // vendored JS bundles).
    //
    // 1. English template: would mean --language never reached the title.
    assert!(
        !html.contains("pedido de licença over "),
        "the English trend template leaked into a --language pt dashboard; html: {html}"
    );
    // 2. Localized fallback: would mean grain_unit stopped being read and viz fell back to
    //    `viz.chart.records`. Correct language, but the entity name is lost.
    assert!(
        !html.contains("registros ao longo de"),
        "grain_unit was ignored and the count unit fell back to the generic `records`; html: \
         {html}"
    );
}

#[test]
fn viz_smart_localized_grain_without_unit_falls_back_localized() {
    // The other half of #4321: a dictionary whose `grain` sentence is localized but which carries
    // no `grain_unit` (e.g. a custom --prompt-file that predates the field). The legacy parser
    // splits on the English literal " one ", which a Portuguese sentence does not contain, so it
    // must degrade to the LOCALIZED `viz.chart.records` -- never to English.
    let wrk = Workdir::new("viz_smart_localized_grain_without_unit_falls_back_localized");
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
            "requested_on": { "type": "string", "title": "Data de Solicitação",
              "x-qsv": { "qsv_type": "Date", "role": "timestamp", "concept": "time.created_at" } },
            "status": { "type": "string", "title": "Situação",
              "x-qsv": { "qsv_type": "String", "role": "dimension", "concept": "category.status" } }
          },
          "x-qsv": { "grain": "uma linha = um pedido de licença" }
        }"#,
    );

    let out_html = wrk.path("dash.html").to_string_lossy().to_string();
    let mut cmd = wrk.command("viz");
    cmd.args(["smart", "permits.csv", "--dictionary"])
        .arg(wrk.path("dict.schema.json"))
        .args(["--language", "pt", "-o", &out_html]);
    wrk.assert_success(&mut cmd);

    let html = wrk.read_to_string("dash.html").unwrap();
    assert!(
        html.contains("registros ao longo de Data de Solicitação"),
        "a localized grain with no grain_unit must fall back to the LOCALIZED records unit; html: \
         {html}"
    );
}

#[test]
fn viz_smart_antimeridian_cluster_stays_local_map() {
    // A tight cluster straddling the +/-180 antimeridian has a small TRUE longitude span but a huge
    // raw max-min span. The global/local test must use the antimeridian-aware span, so this stays a
    // local MapLibre tile map rather than being misclassified as a world ScatterGeo overview.
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
    // local extent (true span ~5 deg) => MapLibre tile map, NOT a world projection overview
    assert!(html.contains(r#""type":"scattermap""#));
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
        html.contains(r#""type":"scattermap""#),
        "map panel should be present"
    );
    assert!(
        html.contains(r#""type":"bar""#),
        "the categorical should still be a bar panel"
    );
    // the coordinates must NOT be re-charted as their own distribution panels
    assert!(
        !html.contains(r#""type":"box""#)
            && !html.contains(r#""type":"violin""#)
            && !html.contains(r#""type":"histogram""#),
        "lat/lon must not be charted as distribution panels; html: {html}"
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
        !html.contains(r#""type":"scattermap""#),
        "no map should render for out-of-range coords"
    );
    assert!(
        html.contains(r#""type":"violin""#),
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
    // per-point hover surfaces the --color value (labeled) beside the coordinates
    assert!(html.contains(r#""hoverinfo":"text""#));
    assert!(html.contains(r#""hovertext":["#));
    assert!(html.contains("magnitude: "));
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
    // each point's hover leads with its series (region) value, then the coordinates, instead of
    // only lat/lon. Asia is the first-seen region (Tokyo), so its trace's hovertext array opens the
    // series; the region name is plain ASCII (the `<br>` separator after it is unicode-escaped).
    assert!(html.contains(r#""hoverinfo":"text""#));
    assert!(html.contains(r#""hovertext":["Asia"#));
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
fn viz_contour_hover_names_both_measures_and_the_row_count() {
    // Plotly's default contour hover is a bare x/y/z triple labeled "trace N", which names
    // neither measure and never says that z is a row count. Both contour paths (this standalone
    // command and `viz smart`'s density panel) must spell that out via one shared template.
    let wrk = Workdir::new("viz_contour_hover_names_both_measures_and_the_row_count");
    quakes(&wrk);

    let mut cmd = wrk.command("viz");
    cmd.args(["contour", "quakes.csv", "--x", "lon", "--y", "lat"]);
    let out = wrk.output(&mut cmd);
    assert!(out.status.success());

    // plotly unicode-escapes angle brackets on serialization, so the template is matched in the
    // form it is actually emitted in
    let html = String::from_utf8_lossy(&out.stdout);
    let want = concat!(
        r"lon: %{x:,.3~f}\u003cbr\u003e",
        r"lat: %{y:,.3~f}\u003cbr\u003e",
        r"%{z:,} rows\u003cextra\u003e\u003c/extra\u003e"
    );
    assert!(
        html.contains(want),
        "the contour cell hover must name both measures and the row count; html: {html}"
    );
}

fn funnel_stages(wrk: &Workdir) {
    wrk.create_from_string(
        "stages.csv",
        "stage,amount\nVisited,48210\nSignedup,12980\nActivated,7412\nSubscribed,2104\nRenewed,\
         861\n",
    );
}

#[test]
fn viz_funnel_keeps_file_order_and_labels_conversion() {
    // `viz funnel` takes a pipeline encoded as ROWS, which `viz smart`'s column-based detector
    // explicitly declines to guess at. Order is first-appearance order in the file -- the user
    // has already answered the question the smart path has to infer -- and is never sorted by
    // value, so a stage that outruns its predecessor stays visible.
    let wrk = Workdir::new("viz_funnel_keeps_file_order_and_labels_conversion");
    funnel_stages(&wrk);

    let mut cmd = wrk.command("viz");
    cmd.args(["funnel", "stages.csv", "--x", "stage", "--y", "amount"]);
    let out = wrk.output(&mut cmd);
    assert!(out.status.success());

    let html = String::from_utf8_lossy(&out.stdout);
    assert!(html.contains(r#""type":"funnel""#), "html: {html}");
    assert!(
        html.contains(r#""y":["Visited","SignedUp","Activated","Subscribed","Renewed"]"#)
            || html.contains(r#""y":["Visited","Signedup","Activated","Subscribed","Renewed"]"#),
        "stages must keep file order, unsorted; html: {html}"
    );
    assert!(
        html.contains(r#""textinfo":"value+percent previous""#),
        "each band should carry its value and conversion; html: {html}"
    );
}

#[test]
fn viz_funnel_counts_stage_rows_when_no_value_column() {
    // --y omitted counts occurrences per stage, mirroring `viz pie`'s behaviour
    let wrk = Workdir::new("viz_funnel_counts_stage_rows_when_no_value_column");
    wrk.create_from_string(
        "ev.csv",
        "stage\nVisited\nVisited\nVisited\nSignedup\nSignedup\nActivated\n",
    );

    let mut cmd = wrk.command("viz");
    cmd.args(["funnel", "ev.csv", "--x", "stage"]);
    let out = wrk.output(&mut cmd);
    assert!(out.status.success());

    let html = String::from_utf8_lossy(&out.stdout);
    assert!(
        html.contains(r#""x":[3.0,2.0,1.0]"#) || html.contains(r#""x":[3,2,1]"#),
        "stage occurrences should be counted; html: {html}"
    );
}

#[test]
fn viz_funnel_rejects_a_negative_stage_total() {
    // a negative band is not something a funnel can represent, so this fails loudly rather than
    // drawing a nonsense chart
    let wrk = Workdir::new("viz_funnel_rejects_a_negative_stage_total");
    wrk.create_from_string("neg.csv", "stage,amount\nVisited,100\nRefunded,-40\n");

    let mut cmd = wrk.command("viz");
    cmd.args(["funnel", "neg.csv", "--x", "stage", "--y", "amount"]);
    wrk.assert_err(&mut cmd);
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
    // plotly's default category hover echoes the TRUNCATED ticktext, so the bar carries the
    // full names as `hovertext` and a template that renders them
    // (bars are ordered by count desc then label asc, so long_b precedes long_a here)
    assert!(
        html.contains(&format!(r#""hovertext":["{long_b}","{long_a}"]"#)),
        "freq bars should carry the full category names as hovertext"
    );
    // plotly unicode-escapes the template's angle brackets, so match the escaped form
    assert!(
        html.contains(r#""hovertemplate":"%{hovertext}\u003cbr\u003ecount: %{y:,}"#),
        "freq bars should render the full name + count in the hover"
    );
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
        // each column cycles on its OWN modulus, so the 10 columns have 10 DISTINCT
        // cardinalities (2..=11). `(r + c) % 4` gave every column the same 4 values in a
        // rotated order — a bijection between every pair — which the 1:1 collapse
        // (issue #4221) correctly folds down to a single panel.
        let cells: Vec<String> = (0..10).map(|c| format!("v{}", r % (c + 2))).collect();
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
    // --qsv-link follows the same paper-not-mode rule: a dark-bg theme gets the light-on-dark
    // accent in :root too, so the chrome links are readable before body.qsv-dark is applied.
    assert!(html.contains(
        ":root { --qsv-page-bg: #111111; --qsv-page-ink: #f2f5fa; --qsv-geo-meta: #9aa4b2; \
         --qsv-link: #6cb6ff; --qsv-link-hover: #9ecbff; }"
    ));
    assert!(html.contains(r#"var themeDefaultMode = "dark""#));
    // and the panels themselves carry the dark template
    assert!(html.contains(r#""template":{"layout""#));
    // regression: an explicit --theme is authoritative. isDark() must consult themeDefaultMode
    // BEFORE the saved localStorage value, so a stale cross-dashboard "light" preference (the
    // key is shared across all qsv viz pages) can't override --theme plotly_dark and leave a
    // dark page with light charts.
    let theme_check = html
        .find(r#"if (themeDefaultMode === "dark") return true;"#)
        .expect("isDark() should check themeDefaultMode");
    let saved_check = html
        .find(r#"localStorage.getItem("qsv-viz-theme")"#)
        .expect("isDark() should read the saved preference");
    assert!(
        theme_check < saved_check,
        "isDark() must check themeDefaultMode before localStorage so an explicit --theme wins"
    );
}

#[test]
fn viz_smart_light_theme_palette_matches_chrome() {
    // a light non-default theme (seaborn) must drive the runtime LIGHT palette so the on-load
    // relayout keeps the panels consistent with the themed page chrome, instead of resetting them
    // to qsv's generic light look (which left a #EAEAF2 seaborn page wrapping #FFFFFF charts).
    let wrk = Workdir::new("viz_smart_light_theme_palette_matches_chrome");
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
    cmd.args(["smart", "people.csv", "--theme", "seaborn", "-o", &out_html]);
    wrk.assert_success(&mut cmd);

    let html = wrk.read_to_string("dash.html").unwrap();
    // the runtime LIGHT palette carries seaborn's own paper/font colors ...
    assert!(html.contains(r##"var LIGHT = { paper: "#EAEAF2", plot: "#EAEAF2", font: "#333333""##));
    // ... matching the seaborn page chrome (so page and charts agree), and it opens in light mode.
    assert!(html.contains("--qsv-page-bg: #EAEAF2"));
    // and a LIGHT-paper theme gets the dark-on-light link accent in :root (the complement of
    // the dark-paper case asserted in viz_smart_inline_theme_drives_page_chrome).
    assert!(html.contains(
        ":root { --qsv-page-bg: #EAEAF2; --qsv-page-ink: #333333; --qsv-geo-meta: #4b5563; \
         --qsv-link: #0a5fb4; --qsv-link-hover: #084b8f; }"
    ));
    assert!(html.contains(r#"var themeDefaultMode = "light""#));
    // the dark complement (toggle target) stays the generic fixed-dark set.
    assert!(html.contains(r##"var DARK = { paper: "#111111""##));
}

#[test]
fn viz_smart_seaborn_dark_palette_matches_chrome() {
    // seaborn_dark is a dark theme whose own shade is #222222, not the generic #111111. Its dark
    // chart palette and dark page chrome must both carry #222222 so the default (dark) view honors
    // the theme instead of collapsing to a plotly_dark look.
    let wrk = Workdir::new("viz_smart_seaborn_dark_palette_matches_chrome");
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
        "seaborn_dark",
        "-o",
        &out_html,
    ]);
    wrk.assert_success(&mut cmd);

    let html = wrk.read_to_string("dash.html").unwrap();
    // the runtime DARK palette carries seaborn_dark's own paper/font ...
    assert!(html.contains(r##"var DARK = { paper: "#222222", plot: "#222222", font: "#eaeaf2""##));
    // ... and the dark page chrome matches it, opening in dark mode.
    assert!(html.contains("body.qsv-dark { --qsv-page-bg: #222222;"));
    assert!(html.contains(r#"var themeDefaultMode = "dark""#));
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
    // the toggle palette includes Carto map styles for both modes so the tile basemap tracks
    // the theme button (map*.style is relayout-ed on each toggle click)
    assert!(html.contains(r#"mapStyle: "carto-positron""#));
    assert!(html.contains(r#"mapStyle: "carto-darkmatter""#));
    assert!(html.contains(r#"/^map\d*$/.test(k)"#));
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
    // Run uncompressed — the version-banner/CommonHTML probes need the plaintext bundle; the
    // default gzip-embedded form is covered by viz_smart_compressed_plotly_bundle.

    // --- ≤8-panel typed grid ---
    let wrk = Workdir::new("viz_smart_embeds_plotly_once_without_mathjax");
    wrk.create_from_string("small.csv", "a,b,c\n1,x,9\n2,y,8\n3,x,7\n4,z,6\n5,y,5\n");
    let grid_html = wrk.path("grid.html").to_string_lossy().to_string();
    let mut cmd = wrk.command("viz");
    cmd.args(["smart", "small.csv", "-o", &grid_html]);
    cmd.env("QSV_VIZ_NO_COMPRESS", "1");
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
        // each column cycles on its OWN modulus, so the 10 columns have 10 DISTINCT
        // cardinalities (2..=11). `(r + c) % 4` gave every column the same 4 values in a
        // rotated order — a bijection between every pair — which the 1:1 collapse
        // (issue #4221) correctly folds down to a single panel.
        let cells: Vec<String> = (0..10).map(|c| format!("v{}", r % (c + 2))).collect();
        rows.push_str(&cells.join(","));
        rows.push('\n');
    }
    wrk.create_from_string("wide.csv", &rows);
    let inline_html = wrk.path("wide.html").to_string_lossy().to_string();
    let mut cmd = wrk.command("viz");
    cmd.args(["smart", "wide.csv", "-o", &inline_html]);
    cmd.env("QSV_VIZ_NO_COMPRESS", "1");
    wrk.assert_success(&mut cmd);
    let inline = wrk.read_to_string("wide.html").unwrap();
    // many panels, but still ONE embedded plotly.js bundle (panels reuse the shared global)
    assert!(inline.matches("Plotly.newPlot").count() > 8);
    assert_eq!(inline.matches("plotly.js v").count(), 1);
    assert!(!inline.contains("CommonHTML"));
}

/// Inflate a `<script id=".." type="application/gzip-b64">` payload embedded in viz HTML back to
/// its plaintext (plotly.js source or figure JSON) — the test-side mirror of the in-browser
/// `DecompressionStream` bootstrap.
fn inflate_gz_payload(html: &str, id: &str) -> String {
    use std::io::Read;

    let marker = format!("id=\"{id}\" type=\"application/gzip-b64\">");
    let start = html.find(&marker).expect("gzip payload tag present") + marker.len();
    let end = start + html[start..].find("</script>").expect("payload close tag");
    let bytes = base64_simd::STANDARD
        .decode_to_vec(html[start..end].trim())
        .expect("valid base64 payload");
    let mut out = String::new();
    flate2::read::GzDecoder::new(&bytes[..])
        .read_to_string(&mut out)
        .expect("valid gzip payload");
    out
}

// By default the plotly.js bundle embeds as ONE gzip+base64 payload plus the queue-stub and
// DecompressionStream bootstrap — ~2.9 MB smaller than the plaintext bundle — and inflates back
// to the exact bundle (version banner present, MathJax still stripped).
#[test]
fn viz_smart_compressed_plotly_bundle() {
    let wrk = Workdir::new("viz_smart_compressed_plotly_bundle");
    wrk.create_from_string("small.csv", "a,b,c\n1,x,9\n2,y,8\n3,x,7\n4,z,6\n5,y,5\n");
    let out_html = wrk.path("dash.html").to_string_lossy().to_string();
    let mut cmd = wrk.command("viz");
    cmd.args(["smart", "small.csv", "-o", &out_html]);
    wrk.assert_success(&mut cmd);
    let html = wrk.read_to_string("dash.html").unwrap();
    // exactly one compressed bundle payload; no plaintext bundle alongside it
    assert_eq!(html.matches("id=\"qsv-plotly-gz\"").count(), 1);
    assert_eq!(html.matches("plotly.js v").count(), 0);
    // the stub queues panel newPlot calls until the bootstrap installs the real global
    assert!(html.contains("window.__qsvPlotQ"));
    assert!(html.contains("DecompressionStream"));
    // unsupported-browser guidance names the escape hatch
    assert!(html.contains("QSV_VIZ_NO_COMPRESS"));
    // the payload inflates back to the plotly.js bundle: banner present, MathJax still stripped
    let bundle = inflate_gz_payload(&html, "qsv-plotly-gz");
    assert_eq!(bundle.matches("plotly.js v").count(), 1);
    assert!(!bundle.contains("CommonHTML"));
}

// A large map panel's figure JSON embeds as a gzip+base64 payload rendered via qsvNewPlotGz; the
// inflated figure carries the scattermap trace with base64 float32 typed-array coordinates
// (bdata) and the baked cluster config.
// geocode-dependent: in a non-geocode build the map panel this asserts on is not emitted as
// panel 0, so `id="qsv-viz-panel-0-fig"` is absent. NOTE the compression behaviour under test is
// itself geocode-independent; a non-geo fixture would keep it covered in lean builds too.
#[cfg(feature = "geocode")]
#[test]
fn viz_smart_compressed_map_figure_payload() {
    let wrk = Workdir::new("viz_smart_compressed_map_figure_payload");
    dense_local_geo(&wrk, "dense.csv", 1200);

    let mut cmd = wrk.command("viz");
    cmd.args(["smart", "dense.csv"]);
    let out = wrk.output(&mut cmd);
    assert!(out.status.success());
    let html = String::from_utf8_lossy(&out.stdout);
    // this dataset has no headline measure (val is low-cardinality), so there is no KPI overview
    // row; the map leads as panel-0: payload + deferred render
    assert!(html.contains("id=\"qsv-viz-panel-0-fig\""));
    assert!(html.contains("qsvNewPlotGz(\"qsv-viz-panel-0\")"));
    let figure = inflate_gz_payload(&html, "qsv-viz-panel-0-fig");
    assert!(figure.contains(r#""type":"scattermap""#));
    assert!(figure.contains(r#""cluster":{"enabled":false,"maxzoom":17.0}"#));
    // coordinates ride as little-endian float32 typed arrays, not decimal-text JSON
    assert!(figure.contains(r#""lat":{"dtype":"float32","bdata":""#));
}

// `viz` renders only plain-text titles/labels, so the ~2.1MB MathJax (tex-svg) bundle that
// plotly's `offline_js_sources` emits next to plotly.js is dead weight. Smart pages have always
// dropped it; single charts used to keep it. `CommonHTML` appears in tex-svg and never in
// plotly.min.js, so it discriminates the two bundles (plain `MathJax` does NOT — plotly.js
// carries a `typeof MathJax` probe).
#[test]
fn viz_single_chart_omits_mathjax_bundle() {
    let wrk = Workdir::new("viz_single_chart_omits_mathjax_bundle");
    wrk.create_from_string("small.csv", "a,b\nx,1\ny,2\nz,3\n");

    // both compression modes: the strip must not depend on the bundle swap
    for no_compress in [false, true] {
        let mut cmd = wrk.command("viz");
        cmd.args(["bar", "small.csv", "-x", "a", "-y", "b"]);
        if no_compress {
            cmd.env("QSV_VIZ_NO_COMPRESS", "1");
        }
        let out = wrk.output(&mut cmd);
        assert!(out.status.success());
        let html = String::from_utf8_lossy(&out.stdout);
        assert!(
            !html.contains("CommonHTML"),
            "MathJax tex-svg bundle leaked into single-chart HTML (no_compress={no_compress})"
        );
        assert!(html.contains("Plotly.newPlot"));
    }
}

// QSV_VIZ_CDN swaps the embedded bundle for a `<script src>` tag on BOTH the single-chart and the
// smart-dashboard paths (they share `plotly_js_block`). None of the embed machinery may survive.
#[test]
fn viz_cdn_replaces_embedded_bundle() {
    let wrk = Workdir::new("viz_cdn_replaces_embedded_bundle");
    wrk.create_from_string("small.csv", "a,b,c\n1,x,9\n2,y,8\n3,x,7\n4,z,6\n5,y,5\n");

    for args in [
        vec!["bar", "small.csv", "-x", "b", "-y", "a"],
        vec!["smart", "small.csv"],
    ] {
        let subcmd = args[0];
        let mut cmd = wrk.command("viz");
        cmd.args(&args);
        cmd.env("QSV_VIZ_CDN", "1");
        let out = wrk.output(&mut cmd);
        assert!(out.status.success());
        let html = String::from_utf8_lossy(&out.stdout);

        assert!(
            html.contains(r#"<script src="https://cdn.plot.ly/"#),
            "{subcmd}: expected a plotly.js CDN tag"
        );
        // Subresource Integrity: without it a tampered CDN response is arbitrary script execution
        // in every viewer of a published dashboard. Assert the attributes land on the plotly tag
        // itself, not merely somewhere in the document.
        let cdn_tag = html
            .split(r#"<script src="https://cdn.plot.ly/"#)
            .nth(1)
            .and_then(|rest| rest.split_once("</script>"))
            .map(|(tag, _)| tag.to_string())
            .unwrap_or_default();
        // a non-empty, well-formed digest — `sha384-` alone would satisfy a prefix check while
        // the browser blocks the script. NOT the literal value: pinning it here would force a
        // test edit on every plotly.rs bump, and the value itself is asserted at its source.
        let digest = cdn_tag
            .split_once(r#"integrity="sha384-"#)
            .and_then(|(_, rest)| rest.split_once('"'))
            .map(|(d, _)| d)
            .unwrap_or_default();
        assert_eq!(
            digest.len(),
            64,
            "{subcmd}: sha384 digest should be 64 base64 chars, got {digest:?}"
        );
        assert!(
            digest
                .bytes()
                .all(|b| b.is_ascii_alphanumeric() || b == b'+' || b == b'/' || b == b'='),
            "{subcmd}: SRI digest is not base64: {digest:?}"
        );
        assert!(
            cdn_tag.contains(r#"crossorigin="anonymous""#),
            "{subcmd}: CDN tag is missing crossorigin: {cdn_tag}"
        );
        // the hash must be pinned to the version actually requested, else it blocks the script
        assert!(
            cdn_tag.contains(".min.js"),
            "{subcmd}: unexpected CDN tag shape: {cdn_tag}"
        );
        // no inline bundle, in either its plain or its gzip+base64 form...
        assert_eq!(html.matches("plotly.js v").count(), 0, "{subcmd}");
        assert_eq!(html.matches("id=\"qsv-plotly-gz\"").count(), 0, "{subcmd}");
        // ...so the queue stub and its bootstrap are unnecessary: Plotly is defined by the time
        // the panel scripts run
        assert!(!html.contains("window.__qsvPlotQ"), "{subcmd}");
        // and no MathJax, same as the embedded modes
        assert!(!html.contains("CommonHTML"), "{subcmd}");
        assert!(html.contains("Plotly.newPlot"), "{subcmd}");
    }
}

// QSV_VIZ_CDN governs only the *bundle*. Figure-payload gzip is still governed by
// QSV_VIZ_NO_COMPRESS, so a gzipped map figure must keep the `__qsvGunzip` prelude that
// `qsvNewPlotGz` depends on — dropping it along with the bootstrap would leave the map blank.
// geocode-dependent for the same reason as viz_smart_compressed_map_figure_payload: the map
// panel is not emitted as panel 0 in a non-geocode build. The `__qsvGunzip` prelude behaviour
// under test is itself geocode-independent.
#[cfg(feature = "geocode")]
#[test]
fn viz_cdn_keeps_gz_prelude_for_compressed_map_figures() {
    let wrk = Workdir::new("viz_cdn_keeps_gz_prelude_for_compressed_map_figures");
    dense_local_geo(&wrk, "dense.csv", 1200);

    let mut cmd = wrk.command("viz");
    cmd.args(["smart", "dense.csv"]);
    cmd.env("QSV_VIZ_CDN", "1");
    let out = wrk.output(&mut cmd);
    assert!(out.status.success());
    let html = String::from_utf8_lossy(&out.stdout);

    assert!(html.contains(r#"<script src="https://cdn.plot.ly/"#));
    assert!(!html.contains("window.__qsvPlotQ"));
    // the figure payload (not the bundle) is still gzipped, and its inflate helpers are present.
    // this dataset has no headline measure, so there is no KPI overview row; the map is panel-0.
    assert!(html.contains("id=\"qsv-viz-panel-0-fig\""));
    assert!(html.contains("qsvNewPlotGz(\"qsv-viz-panel-0\")"));
    assert!(html.contains("__qsvGunzip"));
    let figure = inflate_gz_payload(&html, "qsv-viz-panel-0-fig");
    assert!(figure.contains(r#""type":"scattermap""#));

    // Under CDN there is no bundle bootstrap to raise the "needs DecompressionStream" banner, so
    // the gz figure path must be able to raise it itself — otherwise a pre-2023 browser gets a
    // silently blank map panel with no explanation.
    assert!(html.contains("__qsvNoDecompress"));
    assert!(html.contains("QSV_VIZ_NO_COMPRESS=1"));
}

// CDN + NO_COMPRESS: a bare tag and nothing else — no bundle, no gzip machinery at all.
#[test]
fn viz_cdn_uncompressed_has_no_gz_machinery() {
    let wrk = Workdir::new("viz_cdn_uncompressed_has_no_gz_machinery");
    wrk.create_from_string("small.csv", "a,b,c\n1,x,9\n2,y,8\n3,x,7\n4,z,6\n5,y,5\n");

    let mut cmd = wrk.command("viz");
    // the data viewer's drawer script carries its own (inert, payload-gated) gz dispatch, so
    // disable it here — this test pins down the PLOTLY bundle/figure machinery specifically.
    cmd.args(["smart", "small.csv", "--preview-threshold", "0"]);
    cmd.env("QSV_VIZ_CDN", "1");
    cmd.env("QSV_VIZ_NO_COMPRESS", "1");
    let out = wrk.output(&mut cmd);
    assert!(out.status.success());
    let html = String::from_utf8_lossy(&out.stdout);

    assert!(html.contains(r#"<script src="https://cdn.plot.ly/"#));
    assert_eq!(html.matches("plotly.js v").count(), 0);
    assert!(!html.contains("DecompressionStream"));
    assert!(!html.contains("__qsvGunzip"));
    assert!(!html.contains("window.__qsvPlotQ"));
    assert!(html.contains("Plotly.newPlot"));
}

#[test]
fn viz_smart_inline_has_theme_toggle() {
    // the >8-panel inline-div case also carries the shared toggle.
    let wrk = Workdir::new("viz_smart_inline_has_theme_toggle");
    let headers: Vec<String> = (0..10).map(|c| format!("c{c}")).collect();
    let mut rows = headers.join(",");
    rows.push('\n');
    for r in 0..30 {
        // each column cycles on its OWN modulus, so the 10 columns have 10 DISTINCT
        // cardinalities (2..=11). `(r + c) % 4` gave every column the same 4 values in a
        // rotated order — a bijection between every pair — which the 1:1 collapse
        // (issue #4221) correctly folds down to a single panel.
        let cells: Vec<String> = (0..10).map(|c| format!("v{}", r % (c + 2))).collect();
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

// geocode-dependent: the `qsv-viz-geo-meta` marker renders in non-geocode builds too, so the
// in-body `if html.contains(...)` guard is entered but "Spatial extent:" never appears.
#[cfg(feature = "geocode")]
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
        html.contains(r#""type":"scattermap""#),
        "map panel should be present"
    );
    assert!(
        html.contains("geographic outliers"),
        "outliers should be drawn as a distinct marker trace; html: {html}"
    );
    // smart map panels use Carto tiles (no Referer policy); OSM blocks local-file requests.
    // Assert the serialized Plotly layout key ("style":"carto-positron"), not just the bare
    // string which also appears in the theme-toggle palette (mapStyle: "carto-positron").
    assert!(
        html.contains(r#""style":"carto-positron""#),
        "light-theme smart map panel must set layout.map.style to carto-positron, not \
         open-street-map"
    );
}

// geocode-dependent for the same reason as viz_smart_map_geocode_extent_metadata: the
// `qsv-viz-geo-meta` guard is entered in non-geocode builds but "Spatial extent:" is absent.
#[cfg(feature = "geocode")]
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

/// id (skipped) + three categorical dimensions that form a STRICT rollup tree: each category rolls
/// up to exactly one region and each channel to exactly one category (region → category → channel,
/// a genuine parent→child nesting with no category appearing under two parents). This is the shape
/// `viz smart` auto-selects a treemap/sunburst for — a functional-dependency hierarchy the
/// part-to-whole panel sizes by rolled-up totals. (A many-to-many co-occurrence set of the same
/// cardinalities is instead claimed by the parallel-categories (parcats) panel.) Cardinalities:
/// region=3, category=4, channel=4.
fn three_dim_hierarchy(wrk: &Workdir) {
    let mut rows = String::from("id,region,category,channel\n");
    for i in 1..=120 {
        let (region, category) = match i % 4 {
            0 => ("East", "Widgets"),
            1 => ("East", "Gadgets"),
            2 => ("West", "Gizmos"),
            _ => ("North", "Doohickeys"),
        };
        // `channel` must be strongly associated with `category` (the hierarchy panel needs
        // Cramér's V past its floor) WITHOUT being 1:1 with it: a strict bijection is a
        // relabeling of the same variable, which the 1:1 collapse (issue #4221) folds away,
        // leaving only two levels and a treemap. Doohickeys therefore splits across two
        // channels, so category -> channel is one-to-many.
        let channel = match category {
            "Widgets" => "Web",
            "Gadgets" => "Retail",
            "Gizmos" => "Phone",
            _ if i % 8 == 3 => "Partner",
            _ => "Kiosk",
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

// The composite hierarchy overview title uses the dictionary's human labels (joined with ` › `),
// NOT the raw field names that per-column panel titles now use. Regression guard for the
// title inheriting `Panel.name` (which became the raw field name).
#[test]
fn viz_smart_hierarchy_uses_dictionary_labels_in_title() {
    let wrk = Workdir::new("viz_smart_hierarchy_uses_dictionary_labels_in_title");
    three_dim_hierarchy(&wrk);
    wrk.create_from_string(
        "dict.schema.json",
        r#"{
          "$schema": "https://json-schema.org/draft/2020-12/schema",
          "type": "object",
          "properties": {
            "region":   { "type": "string", "title": "Sales Region",
              "x-qsv": { "qsv_type": "String", "role": "dimension", "concept": "category.status" } },
            "category": { "type": "string", "title": "Product Category",
              "x-qsv": { "qsv_type": "String", "role": "dimension", "concept": "category.status" } },
            "channel":  { "type": "string", "title": "Sales Channel",
              "x-qsv": { "qsv_type": "String", "role": "dimension", "concept": "category.status" } }
          }
        }"#,
    );

    let out_html = wrk.path("dash.html").to_string_lossy().to_string();
    let mut cmd = wrk.command("viz");
    cmd.args(["smart", "three_dim.csv", "--dictionary"])
        .arg(wrk.path("dict.schema.json"))
        .args(["-o", &out_html]);
    wrk.assert_success(&mut cmd);

    let html = wrk.read_to_string("dash.html").unwrap();
    assert!(html.contains(r#""type":"sunburst""#));
    // dims are ordered coarsest-first (region=3 < category=4 < channel=4), so the title reads
    // "Sales Region › Product Category › Sales Channel" — the dictionary labels, not
    // "region › category › channel".
    assert!(
        html.contains("Sales Region › Product Category › Sales Channel"),
        "hierarchy title should use dictionary labels joined with ` › `; html: {html}"
    );
    assert!(
        !html.contains("region › category › channel"),
        "hierarchy title should NOT use raw field names; html: {html}"
    );
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
fn viz_splom_standalone() {
    let wrk = Workdir::new("viz_splom_standalone");
    // three numeric columns; a and b are perfectly correlated, c is independent
    let mut rows = String::from("a,b,c\n");
    for i in 0..40 {
        let a = i % 7;
        let b = (i % 7) * 2;
        let c = (i % 5) + 1;
        rows.push_str(&format!("{a},{b},{c}\n"));
    }
    wrk.create_from_string("nums.csv", &rows);

    let out_html = wrk.path("sp.html").to_string_lossy().to_string();
    let mut cmd = wrk.command("viz");
    cmd.args(["splom", "nums.csv", "--cols", "a,b,c", "-o", &out_html]);
    wrk.assert_success(&mut cmd);

    let html = wrk.read_to_string("sp.html").unwrap();
    assert!(html.contains(r#""type":"splom""#));
    // one dimension per selected numeric column, each carrying its label
    assert!(html.contains(r#""label":"a""#));
    assert!(html.contains(r#""label":"b""#));
    assert!(html.contains(r#""label":"c""#));
}

#[test]
fn viz_splom_needs_two_numeric_cols() {
    // splom cross-plots numeric column pairs, so a single numeric column (the other selected
    // column being non-numeric text) is not enough — it must error, not emit a 1x1 grid.
    let wrk = Workdir::new("viz_splom_needs_two_numeric_cols");
    wrk.create_from_string("one.csv", "a,name\n1,x\n2,y\n3,z\n");

    let mut cmd = wrk.command("viz");
    cmd.args(["splom", "one.csv", "--cols", "a,name"]);
    wrk.assert_err(&mut cmd);
}

#[test]
fn viz_splom_no_joint_rows_errors() {
    // both columns are majority-numeric on their own, but no single row has BOTH populated, so
    // the listwise read drops every row. splom must error rather than emit an empty matrix.
    let wrk = Workdir::new("viz_splom_no_joint_rows_errors");
    wrk.create_from_string("disjoint.csv", "a,b\n1,\n2,\n,3\n,4\n");

    let mut cmd = wrk.command("viz");
    cmd.args(["splom", "disjoint.csv", "--cols", "a,b"]);
    wrk.assert_err(&mut cmd);
}

#[test]
fn viz_parcats_standalone() {
    let wrk = Workdir::new("viz_parcats_standalone");
    // three low-cardinality categorical columns
    let mut rows = String::from("region,tier,status\n");
    let regions = ["north", "south"];
    let tiers = ["gold", "silver"];
    let statuses = ["open", "closed"];
    for i in 0..40 {
        let r = regions[i % regions.len()];
        let t = tiers[(i / 2) % tiers.len()];
        let s = statuses[(i / 3) % statuses.len()];
        rows.push_str(&format!("{r},{t},{s}\n"));
    }
    wrk.create_from_string("cats.csv", &rows);

    let out_html = wrk.path("pc.html").to_string_lossy().to_string();
    let mut cmd = wrk.command("viz");
    cmd.args([
        "parcats",
        "cats.csv",
        "--cols",
        "region,tier,status",
        "-o",
        &out_html,
    ]);
    wrk.assert_success(&mut cmd);

    let html = wrk.read_to_string("pc.html").unwrap();
    assert!(html.contains(r#""type":"parcats""#));
    // identical category tuples are aggregated into weighted paths (`counts`)
    assert!(html.contains(r#""counts":["#));
    assert!(html.contains(r#""label":"region""#));
    assert!(html.contains(r#""label":"status""#));
    // ribbons are colored (by first-dim category) and bundled, like a Sankey — not the default gray
    assert!(html.contains(r#""line":{"color""#));
    assert!(html.contains(r#""bundlecolors":true"#));
    // opens count-ordered (categoryarray baked, categoryorder=array) with an on-screen "category
    // order" toggle (updatemenus button) that flips each axis' categoryorder via restyle
    assert!(html.contains(r#""categoryorder":"array""#));
    assert!(html.contains(r#""categoryarray":["#));
    assert!(
        html.contains(r#""updatemenus""#) && html.contains("category order"),
        "parcats should bake in a category-order toggle button; html: {html}"
    );
    assert!(html.contains("dimensions[0].categoryorder"));
    // the flip is ANIMATED, which plotly's parcats renderer can't do on its own (unlike its sankey
    // renderer, whose replot transitions are what make the node-order toggle animate for free): an
    // injected script intercepts the click and tweens across the restyle
    assert!(
        html.contains("__qsvParcatsAnim"),
        "a standalone parcats page must carry the category-order animation script; html: {html}"
    );
    // ...but the button itself stays a NATIVE plotly toggle, so it still works wherever the figure
    // JSON travels without that script (`gen_gallery.py` reassembles the bare figure). `execute:
    // false` would delegate the restyle to the script and leave a dead button there.
    assert!(
        !html.contains(r#""execute":false"#),
        "the category-order button must not delegate its restyle to the script; html: {html}"
    );
    // the marker `examples/viz/gen_gallery.py` keys on to lift the script into the gallery
    // scaffold (the gallery reassembles bare figure JSON, so without it that figure would snap)
    assert!(
        html.contains("<!--qsv-parcats-order-->"),
        "the animation script must carry the marker gen_gallery.py extracts it by; html: {html}"
    );
}

// deterministic dataset that qualifies for the smart parcats panel: region/tier/segment are 3
// associated, many-to-many categoricals each with >= 3 well-spread distinct values (no
// near-constant column), while a/b/c/d are numeric filler.
fn smart_parcats_csv(wrk: &Workdir) {
    let regions = ["north", "south", "east"];
    let tiers = ["gold", "silver", "bronze"];
    let segs = ["retail", "wholesale", "online", "partner"];
    let mut rows = String::from("a,b,c,d,region,tier,segment\n");
    for i in 0..90usize {
        let a = i % 12;
        let b = a * 2; // perfectly correlated with a
        let c = (i * 7) % 12; // independent
        let d = 12 - a; // negatively correlated with a
        let region = regions[i % 3];
        let tier = tiers[(i % 3 + usize::from(i % 4 == 0)) % 3];
        let ti = tiers.iter().position(|&t| t == tier).unwrap();
        let seg = segs[(ti + usize::from(i % 5 == 0)) % 4];
        rows.push_str(&format!("{a},{b},{c},{d},{region},{tier},{seg}\n"));
    }
    wrk.create_from_string("smart.csv", &rows);
}

#[test]
fn viz_smart_parcats_suppresses_hierarchy() {
    let wrk = Workdir::new("viz_smart_parcats_suppresses_hierarchy");
    smart_parcats_csv(&wrk);

    let out_html = wrk.path("s.html").to_string_lossy().to_string();
    let mut cmd = wrk.command("viz");
    cmd.args(["smart", "smart.csv", "-o", &out_html]);
    wrk.assert_success(&mut cmd);

    let html = wrk.read_to_string("s.html").unwrap();
    // 3 associated but MANY-TO-MANY categorical dimensions (a category appears under several
    // parents) -> a parallel-categories flow panel. Such a co-occurrence set previously
    // auto-selected a sunburst; parcats now claims it, while genuine rollup trees (see
    // `viz_smart_hierarchy_sunburst_for_three_dims`) still auto-select the sunburst.
    assert!(html.contains(r#""type":"parcats""#));
    // ...which owns the 3-4-dimension relationship, so the hierarchy treemap/sunburst is suppressed
    // on the same columns (mutual exclusivity).
    assert!(!html.contains(r#""type":"treemap""#));
    assert!(!html.contains(r#""type":"sunburst""#));
    // the panel's category-order toggle is animated on a dashboard too
    assert!(
        html.contains("__qsvParcatsAnim"),
        "a dashboard with a parcats panel must carry the category-order animation script"
    );
    assert!(!html.contains(r#""execute":false"#));
}

// Length of the comma-separated `"<key>":[...]` array first appearing at/after the position of
// `after` (0 for an empty array, None if not found). Plain string scan so the tests need no regex.
fn array_len_after(html: &str, after: &str, key: &str) -> Option<usize> {
    let start = html.find(after)?;
    let needle = format!("\"{key}\":[");
    let open = html[start..].find(&needle)? + start + needle.len();
    let end = html[open..].find(']')? + open;
    let body = &html[open..end];
    Some(if body.trim().is_empty() {
        0
    } else {
        body.split(',').count()
    })
}

#[test]
fn viz_smart_parcats_caps_paths() {
    // Four associated-but-noisy categoricals produce far more than PARCATS_MAX_PATHS (200) distinct
    // tuples; the panel keeps only the 200 heaviest ribbons so the trace stays bounded (a 30^4
    // worst case would otherwise embed hundreds of thousands of paths).
    let wrk = Workdir::new("viz_smart_parcats_caps_paths");
    let mut rows = String::from("d1,d2,d3,d4\n");
    // deterministic LCG so the fixture (and its tuple spread) is reproducible across runs.
    let mut s: u64 = 0x9E37_79B9_7F4A_7C15;
    let next = |m: u64, s: &mut u64| {
        *s = s
            .wrapping_mul(6364136223846793005)
            .wrapping_add(1442695040888963407);
        (*s >> 33) % m
    };
    for _ in 0..6000 {
        let a = next(15, &mut s);
        // each dim tracks `a`/`b` ~55% of the time (keeps them associated) and draws freely
        // otherwise (spreads the joint distribution across many tuples).
        let b = if next(100, &mut s) < 55 {
            a
        } else {
            next(15, &mut s)
        };
        let c = if next(100, &mut s) < 55 {
            a
        } else {
            next(15, &mut s)
        };
        let d = if next(100, &mut s) < 55 {
            b
        } else {
            next(15, &mut s)
        };
        rows.push_str(&format!("g{a},h{b},k{c},m{d}\n"));
    }
    wrk.create_from_string("pc.csv", &rows);

    let out_html = wrk.path("dash.html").to_string_lossy().to_string();
    let mut cmd = wrk.command("viz");
    cmd.args(["smart", "pc.csv", "-o", &out_html]);
    wrk.assert_success(&mut cmd);

    let html = wrk.read_to_string("dash.html").unwrap();
    assert!(html.contains(r#""type":"parcats""#));
    let ribbons =
        array_len_after(&html, r#""type":"parcats""#, "counts").expect("parcats counts array");
    assert!(
        ribbons <= 200,
        "parcats ribbons must be capped at PARCATS_MAX_PATHS (200), got {ribbons}"
    );
    assert!(
        ribbons > 100,
        "test data should produce enough distinct tuples for the cap to engage, got {ribbons}"
    );
}

#[test]
fn viz_icicle_standalone() {
    let wrk = Workdir::new("viz_icicle_standalone");
    three_dim_hierarchy(&wrk);

    let out_html = wrk.path("ic.html").to_string_lossy().to_string();
    let mut cmd = wrk.command("viz");
    cmd.args([
        "icicle",
        "three_dim.csv",
        "--cols",
        "region,category,channel",
        "-o",
        &out_html,
    ]);
    wrk.assert_success(&mut cmd);

    let html = wrk.read_to_string("ic.html").unwrap();
    assert!(html.contains(r#""type":"icicle""#));
    // shares the sunburst/treemap rolled-up-total contract and the richer per-tile textinfo
    assert!(html.contains(r#""branchvalues":"total""#));
    assert!(html.contains(r#""textinfo":"label+value+percent parent""#));
}

// `viz smart` leads with a KPI overview row of `Indicator` tiles: dataset-summary tiles (record
// count, field count, a completeness gauge) plus the headline numeric measures.
#[test]
fn viz_smart_kpi_row() {
    let wrk = Workdir::new("viz_smart_kpi_row");
    wrk.create_from_string(
        "sales.csv",
        "region,amount,rating\nEast,1200,4.5\nWest,800,3.9\nEast,450,4.1\nNorth,2100,4.8\nSouth,\
         300,3.2\nEast,1750,4.6\nWest,980,4.0\nNorth,600,4.3\n",
    );
    let out_html = wrk.path("k.html").to_string_lossy().to_string();
    let mut cmd = wrk.command("viz");
    cmd.args(["smart", "sales.csv", "-o", &out_html]);
    wrk.assert_success(&mut cmd);

    let html = wrk.read_to_string("k.html").unwrap();
    // the KPI row leads with the headline numeric measures (amount, rating)
    assert!(html.contains(r#""type":"indicator""#));
    // completeness is now a quiet header metadata stat (below "Columns:"), NOT a KPI gauge tile
    assert!(html.contains("Completeness:"));
    assert!(!html.contains(r#""text":"Completeness""#));
    // record/field counts are intentionally NOT KPI tiles (they duplicate the header)
    assert!(!html.contains(r#""text":"Records""#));
    assert!(!html.contains(r#""text":"Fields""#));
}

// A dictionary `gauge_range`/`target` turns a measure tile into a gauge (over its range, averaged
// so the value lands within it) and a "vs target" delta — the LLM-hint mechanism for compelling
// KPIs, guarded so the gauge is only drawn when it actually contains the observed value.
#[test]
fn viz_smart_kpi_dictionary_gauge_and_delta() {
    let wrk = Workdir::new("viz_smart_kpi_dictionary_gauge_and_delta");
    wrk.create_from_string(
        "sales.csv",
        "region,amount,rating\nEast,1200,4.5\nWest,800,3.9\nEast,450,4.1\nNorth,2100,4.8\nSouth,\
         300,3.2\nEast,1750,4.6\nWest,980,4.0\nNorth,600,4.3\n",
    );
    wrk.create_from_string(
        "dict.json",
        r#"{"type":"object","properties":{"rating":{"title":"Customer Rating","type":"number","x-qsv":{"role":"measure","concept":"measure.rating","gauge_range":[0,5],"target":4.0}}}}"#,
    );
    let out_html = wrk.path("k.html").to_string_lossy().to_string();
    let mut cmd = wrk.command("viz");
    cmd.args([
        "smart",
        "sales.csv",
        "--dictionary",
        "dict.json",
        "-o",
        &out_html,
    ]);
    wrk.assert_success(&mut cmd);

    let html = wrk.read_to_string("k.html").unwrap();
    // gauge over the dictionary [0,5] range, averaged (Mean) so the value sits within it
    assert!(html.contains(r#""axis":{"range":[0.0,5.0]}"#));
    assert!(html.contains(r#""text":"Mean Customer Rating""#));
    // and a "vs target" delta against the supplied target
    assert!(html.contains(r#""delta":{"reference":4.0}"#));
}

// Regression (roborev #3602): the leading KPI row is domain-positioned and takes no cartesian
// axis, but it still occupies panel index 0. With exactly MAX_SUBPLOTS (8) cartesian panels behind
// it on the typed-grid path, the axes must number x1..x8 — before the fix the 8th chart was pushed
// to x9, which the typed Layout has no slot for (`assign_typed_axis` drops it), orphaning that
// trace onto the default axis. `--max-charts 8` on a wide numeric dataset forces the boundary.
#[test]
fn viz_smart_kpi_row_does_not_orphan_eighth_axis() {
    let wrk = Workdir::new("viz_smart_kpi_row_does_not_orphan_eighth_axis");
    let mut csv = (0..10)
        .map(|j| format!("n{j}"))
        .collect::<Vec<_>>()
        .join(",");
    csv.push('\n');
    for i in 0..120 {
        let row = (0..10)
            .map(|j| ((i * (j * 37 + 13) + j * 7) % (50 + j)).to_string())
            .collect::<Vec<_>>()
            .join(",");
        csv.push_str(&row);
        csv.push('\n');
    }
    wrk.create_from_string("num_wide.csv", &csv);
    let out_html = wrk.path("n.html").to_string_lossy().to_string();
    let mut cmd = wrk.command("viz");
    cmd.args([
        "smart",
        "num_wide.csv",
        "--max-charts",
        "8",
        "-o",
        &out_html,
    ]);
    wrk.assert_success(&mut cmd);

    let html = wrk.read_to_string("n.html").unwrap();
    assert!(html.contains(r#""type":"indicator""#)); // KPI overview row is present
    // the 8 cartesian charts behind it fill x1..x8 (typed Layout), with NO orphaned x9
    assert!(html.contains(r#""xaxis8":{"#));
    assert!(html.contains(r#""xaxis":"x8""#));
    assert!(!html.contains(r#""xaxis":"x9""#));
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
    // the fill is inserted above the basemap road layers (below="") with a near-opaque fill, so
    // roads don't bleed through and muddy the regions
    assert!(
        html.contains(r#""below":"""#),
        "choropleth --map fill must sit above the basemap roads (below=\"\"): {html}"
    );
    assert!(
        html.contains(r#""opacity":0.9"#),
        "near-opaque fill expected: {html}"
    );
}

// --feature-id-key is optional for --map: when omitted it defaults to "id" (the top-level
// GeoJSON feature id), so a ChoroplethMap still resolves without the flag. Regression guard for
// the relaxed `--map requires --geojson` check (feature-id-key no longer required).
#[test]
fn viz_choropleth_map_default_feature_id_key() {
    let wrk = Workdir::new("viz_choropleth_map_default_feature_id_key");
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
    ]);
    let out = wrk.output(&mut cmd);
    assert!(out.status.success());

    let html = String::from_utf8_lossy(&out.stdout);
    // renders the same ChoroplethMap as the explicit `--feature-id-key id` case
    assert!(html.contains(r#""type":"choroplethmap""#));
    assert!(html.contains(r#""featureidkey":"id""#));
}

// shared GeoJSON fixture body for the shortcut/validation tests below: two id-keyed polygons that
// also carry a `properties.id` label.
const SHORTCUT_GEOJSON: &str = r#"{"type":"FeatureCollection","features":[{"type":"Feature","id":"A","properties":{"id":"A"},"geometry":{"type":"Polygon","coordinates":[[[0,0],[0,1],[1,1],[1,0],[0,0]]]}},{"type":"Feature","id":"B","properties":{"id":"B"},"geometry":{"type":"Polygon","coordinates":[[[1,0],[1,1],[2,1],[2,0],[1,0]]]}}]}"#;

// a --geojson that points at a nonexistent file (and no QSV_GEOJSON_SHORTCUTS defined) fails fast
// with a clear message instead of erroring lazily deep inside a plot builder.
#[test]
fn viz_choropleth_geojson_missing_file_errors() {
    let wrk = Workdir::new("viz_choropleth_geojson_missing_file_errors");
    wrk.create_from_string("rg.csv", "region,val\nA,10\nB,20\n");

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
        "nope.geojson",
    ])
    .env_remove("QSV_GEOJSON_SHORTCUTS");
    let out = wrk.output(&mut cmd);
    assert!(!out.status.success());
    let stderr = wrk.output_stderr(&mut cmd);
    assert!(stderr.contains("no QSV_GEOJSON_SHORTCUTS are defined"));
}

// `viz smart` resolves `--geojson auto` from the region column its DICTIONARY names (issue
// #4416), so without a dictionary there is no region column and nothing to scope a fetch by. The
// error must say that rather than failing obscurely inside the fetch — and must not reach the
// network at all, which is what makes this test hermetic.
#[test]
fn viz_smart_geojson_auto_without_a_region_column_errors() {
    let wrk = Workdir::new("viz_smart_geojson_auto_without_a_region_column_errors");
    wrk.create_from_string("rg.csv", "region,val\n42003,10\n42101,20\n");

    let mut cmd = wrk.command("viz");
    cmd.args(["smart", "rg.csv", "--geojson", "auto"])
        .env_remove("QSV_GEOJSON_SHORTCUTS");
    let out = wrk.output(&mut cmd);
    assert!(!out.status.success());
    let stderr = String::from_utf8_lossy(&out.stderr);
    assert!(
        stderr.contains("no region-code column") && stderr.contains("--dictionary"),
        "expected the deferred-auto diagnostic naming --dictionary: {stderr}"
    );
}

// `--geojson auto` without --locations has no region codes to scope a boundary fetch by, so it
// must say that instead of fetching something arbitrary.
#[test]
fn viz_choropleth_geojson_auto_requires_locations() {
    let wrk = Workdir::new("viz_choropleth_geojson_auto_requires_locations");
    wrk.create_from_string("rg.csv", "lat,lon,val\n40.4,-79.9,10\n39.9,-75.1,20\n");

    let mut cmd = wrk.command("viz");
    cmd.args([
        "choropleth",
        "rg.csv",
        "--lat",
        "lat",
        "--lon",
        "lon",
        "--value",
        "val",
        "--geojson",
        "auto",
    ])
    .env_remove("QSV_GEOJSON_SHORTCUTS");
    let out = wrk.output(&mut cmd);
    assert!(!out.status.success());
    let stderr = wrk.output_stderr(&mut cmd);
    assert!(stderr.contains("--locations"));
}

// `auto` is a keyword, but an existing local file of that name must still win — otherwise the
// keyword would silently shadow a real boundary file and fetch from the network instead.
// Guards the disambiguation order in `resolve_and_validate_geojson`.
#[test]
fn viz_choropleth_geojson_auto_local_file_wins() {
    let wrk = Workdir::new("viz_choropleth_geojson_auto_local_file_wins");
    wrk.create_from_string("rg.csv", "region,val\nA,10\nB,20\n");
    wrk.create_from_string("auto", SHORTCUT_GEOJSON);

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
        "auto",
        "--feature-id-key",
        "properties.id",
    ])
    .env_remove("QSV_GEOJSON_SHORTCUTS");
    let out = wrk.output(&mut cmd);
    assert!(out.status.success());
    let html = String::from_utf8_lossy(&out.stdout);
    // the LOCAL file's ids, not Census GEOIDs — proves no network resolution happened
    assert!(html.contains(r#""featureidkey":"properties.id""#));
    assert!(html.contains(r#""locations":["A","B"]"#));
}

// A numeric region column drops leading zeros (`01001` arrives as `1001`). The emitted locations
// must be canonicalized to the BOUNDARY's spelling, or plotly matches nothing and the map renders
// completely blank — while still exiting 0, because the coverage check accepted the padded form
// the renderer never used. This is the render half of that invariant; no network involved.
#[test]
fn viz_choropleth_canonicalizes_zero_padded_locations() {
    let wrk = Workdir::new("viz_choropleth_canonicalizes_zero_padded_locations");
    wrk.create_from_string("counties.csv", "fips,cases\n1001,10\n1003,20\n");
    wrk.create_from_string(
        "b.geojson",
        r#"{"type":"FeatureCollection","features":[
{"type":"Feature","properties":{"GEOID":"01001"},"geometry":{"type":"Polygon","coordinates":[[[0,0],[0,1],[1,1],[1,0],[0,0]]]}},
{"type":"Feature","properties":{"GEOID":"01003"},"geometry":{"type":"Polygon","coordinates":[[[1,0],[1,1],[2,1],[2,0],[1,0]]]}}]}"#,
    );

    let mut cmd = wrk.command("viz");
    cmd.args([
        "choropleth",
        "counties.csv",
        "--locations",
        "fips",
        "--value",
        "cases",
        "--location-mode",
        "geojson-id",
        "--geojson",
        "b.geojson",
        "--feature-id-key",
        "properties.GEOID",
    ])
    .env_remove("QSV_GEOJSON_SHORTCUTS");
    let out = wrk.output(&mut cmd);
    assert!(out.status.success());
    let html = String::from_utf8_lossy(&out.stdout);

    // the padded ids the features actually carry, not the raw cells
    assert!(
        html.contains(r#""locations":["01001","01003"]"#),
        "locations were not canonicalized to the feature ids"
    );
    assert!(
        !html.contains(r#""locations":["1001""#),
        "raw unpadded locations were emitted; the map would render blank"
    );
    // values must still ride along with their region
    assert!(
        html.contains(r#""z":[10.0,20.0]"#),
        "values lost in canonicalization"
    );
}

// a --geojson value that names a QSV_GEOJSON_SHORTCUTS entry resolves to the entry's path, and the
// entry's `id` fills in --feature-id-key when the user doesn't pass one.
#[test]
fn viz_choropleth_geojson_shortcut_resolves() {
    let wrk = Workdir::new("viz_choropleth_geojson_shortcut_resolves");
    wrk.create_from_string("rg.csv", "region,val\nA,10\nB,20\n");
    wrk.create_from_string("regions.geojson", SHORTCUT_GEOJSON);

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
        "regions",
    ])
    .env(
        "QSV_GEOJSON_SHORTCUTS",
        r#"{"regions":{"path":"regions.geojson","id":"properties.id"}}"#,
    );
    let out = wrk.output(&mut cmd);
    assert!(out.status.success());

    let html = String::from_utf8_lossy(&out.stdout);
    assert!(html.contains(r#""type":"choroplethmap""#));
    // the shortcut's id became the feature-id-key
    assert!(html.contains(r#""featureidkey":"properties.id""#));
    assert!(html.contains(r#""geojson":{"type":"FeatureCollection""#));
}

// an explicitly-passed --feature-id-key wins over the shortcut's id.
#[test]
fn viz_choropleth_geojson_shortcut_explicit_key_wins() {
    let wrk = Workdir::new("viz_choropleth_geojson_shortcut_explicit_key_wins");
    wrk.create_from_string("rg.csv", "region,val\nA,10\nB,20\n");
    wrk.create_from_string("regions.geojson", SHORTCUT_GEOJSON);

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
        "regions",
        "--feature-id-key",
        "properties.id",
    ])
    .env(
        "QSV_GEOJSON_SHORTCUTS",
        r#"{"regions":{"path":"regions.geojson","id":"id"}}"#,
    );
    let out = wrk.output(&mut cmd);
    assert!(out.status.success());
    let html = String::from_utf8_lossy(&out.stdout);
    // the explicit properties.id wins over the shortcut's "id"
    assert!(html.contains(r#""featureidkey":"properties.id""#));
}

// an explicit `--feature-id-key id` (matching the docopt default) still wins over the shortcut's
// id — argv is scanned so the explicit flag is distinguished from the default.
#[test]
fn viz_choropleth_geojson_shortcut_explicit_default_key_wins() {
    let wrk = Workdir::new("viz_choropleth_geojson_shortcut_explicit_default_key_wins");
    wrk.create_from_string("rg.csv", "region,val\nA,10\nB,20\n");
    wrk.create_from_string("regions.geojson", SHORTCUT_GEOJSON);

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
        "regions",
        "--feature-id-key",
        "id",
    ])
    .env(
        "QSV_GEOJSON_SHORTCUTS",
        r#"{"regions":{"path":"regions.geojson","id":"properties.id"}}"#,
    );
    let out = wrk.output(&mut cmd);
    assert!(out.status.success());
    let html = String::from_utf8_lossy(&out.stdout);
    // the explicit "id" wins over the shortcut's "properties.id"
    assert!(html.contains(r#""featureidkey":"id""#));
}

// an unknown shortcut name errors and lists the available shortcut keys.
#[test]
fn viz_choropleth_geojson_shortcut_unknown_errors() {
    let wrk = Workdir::new("viz_choropleth_geojson_shortcut_unknown_errors");
    wrk.create_from_string("rg.csv", "region,val\nA,10\nB,20\n");

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
        "counties",
    ])
    .env(
        "QSV_GEOJSON_SHORTCUTS",
        r#"{"regions":{"path":"regions.geojson"},"wards":{"path":"wards.geojson"}}"#,
    );
    let out = wrk.output(&mut cmd);
    assert!(!out.status.success());
    let stderr = wrk.output_stderr(&mut cmd);
    assert!(stderr.contains("Unknown --geojson shortcut 'counties'"));
    assert!(stderr.contains("regions, wards"));
}

// a malformed QSV_GEOJSON_SHORTCUTS value (when consulted) errors clearly.
#[test]
fn viz_choropleth_geojson_shortcut_malformed_json_errors() {
    let wrk = Workdir::new("viz_choropleth_geojson_shortcut_malformed_json_errors");
    wrk.create_from_string("rg.csv", "region,val\nA,10\nB,20\n");

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
        "regions",
    ])
    .env("QSV_GEOJSON_SHORTCUTS", r#"{"regions": not json}"#);
    let out = wrk.output(&mut cmd);
    assert!(!out.status.success());
    let stderr = wrk.output_stderr(&mut cmd);
    assert!(stderr.contains("QSV_GEOJSON_SHORTCUTS is not valid JSON"));
}

// a --feature-id-key that resolves on no feature fails up front with a helpful message.
#[test]
fn viz_choropleth_geojson_bad_feature_id_key_errors() {
    let wrk = Workdir::new("viz_choropleth_geojson_bad_feature_id_key_errors");
    wrk.create_from_string("rg.csv", "region,val\nA,10\nB,20\n");
    wrk.create_from_string("regions.geojson", SHORTCUT_GEOJSON);

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
        "properties.nonesuch",
    ]);
    let out = wrk.output(&mut cmd);
    assert!(!out.status.success());
    let stderr = wrk.output_stderr(&mut cmd);
    assert!(stderr.contains("resolves on no feature"));
}

// a --feature-id-key that resolves ONLY on a non-polygon feature (here a Point) fails up front
// with the "no usable Polygon/MultiPolygon features" guidance, before any expensive processing —
// the first check (key resolves somewhere) passes, so this exercises the second, geometry-aware
// check specifically.
#[test]
fn viz_choropleth_geojson_id_only_on_non_polygon_errors() {
    let wrk = Workdir::new("viz_choropleth_geojson_id_only_on_non_polygon_errors");
    wrk.create_from_string("rg.csv", "region,val\nA,10\nB,20\n");
    // the id key `properties.id` resolves on the Point feature but on neither Polygon feature
    // (they carry a different property), so no usable Polygon/MultiPolygon feature has the id.
    wrk.create_from_string(
        "regions.geojson",
        r#"{"type":"FeatureCollection","features":[{"type":"Feature","properties":{"id":"A"},"geometry":{"type":"Point","coordinates":[0,0]}},{"type":"Feature","properties":{"other":"B"},"geometry":{"type":"Polygon","coordinates":[[[0,0],[0,1],[1,1],[1,0],[0,0]]]}}]}"#,
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
        "properties.id",
    ]);
    let out = wrk.output(&mut cmd);
    assert!(!out.status.success());
    let stderr = wrk.output_stderr(&mut cmd);
    assert!(stderr.contains("no usable Polygon/MultiPolygon features"));
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
// Exercises the auto-derived snap cap: this fixture's 10-deg regions (~157 km region term) and
// integer-degree lons (~157 km precision floor) both clamp to the 100 km ceiling, so a
// near-boundary stray snaps and a far stray drops.
#[test]
fn viz_choropleth_pip_bins_points() {
    let wrk = Workdir::new("viz_choropleth_pip_bins_points");
    // A = lon 0..10, B = lon 10..20 (both lat 0..10). Points: one in A, two in B, a near stray
    // ~5.6 km north of A's edge (snaps to A under the auto 100 km cap), and one far outside
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
    // A = 1 contained + 1 snapped; B = 2 contained; the far (50,50) stray drops under the auto
    // 100 km cap
    assert!(html.contains(r#""locations":["A","B"]"#));
    assert!(html.contains(r#""z":[2.0,2.0]"#));
    // A absorbed the one snapped stray — its hover flags it as a subset of A's count
    assert!(
        html.contains("includes 1 snapped from outside"),
        "missing snapped-into-region hover note; html was: {html}"
    );
    // any snapping surfaces the snap metadata (count, cap, provenance) beneath the map (no stderr
    // in a saved file)
    assert!(
        html.contains(
            "1 of 5 points snapped to the nearest region (≤100 km, auto-derived from region size \
             and coordinate precision)."
        ),
        "missing snapped below-map note; html was: {html}"
    );
    // the far stray was dropped by the auto cap — also noted beneath the map
    assert!(
        html.contains("1 of 5 points were farther than 100 km from any region and were dropped."),
        "missing dropped-beyond-cap note; html was: {html}"
    );
    // stderr reports both the snap (with the derived cap + its provenance) and the cap-drop so
    // the user knows where points went
    let stderr = String::from_utf8_lossy(&out.stderr);
    assert!(
        stderr.contains(
            "1 of 5 points were snapped to the nearest region (cap 100 km, auto-derived from \
             region size and coordinate precision)."
        ),
        "missing snap coverage note; stderr was: {stderr}"
    );
    assert!(
        stderr.contains(
            "1 of 5 points fell outside every region and were dropped (no region within 100 km)."
        ),
        "missing cap-drop coverage note; stderr was: {stderr}"
    );
}

// the auto snap cap is context-sensitive: ward-scale regions (~3 km bbox diagonal) with precise
// coordinates derive a sub-km cap (10% of the median region diagonal), so a stray ~2 km out —
// which the old fixed 10 km default would have snapped — is dropped, while a ~0.1 km stray still
// snaps.
#[test]
fn viz_choropleth_pip_auto_snap_cap_ward_scale() {
    let wrk = Workdir::new("viz_choropleth_pip_auto_snap_cap_ward_scale");
    // A = lon 0..0.02, B = lon 0.02..0.04 (both lat 0..0.02): 0.02-deg squares, bbox diagonal
    // ~3.15 km -> region term 0.315 km; 5-decimal coordinates make the precision floor
    // negligible -> auto cap 0.31 km. Points: one in A, one in B, a stray 0.001 deg (~0.11 km)
    // north of A (snaps), and a stray 0.018 deg (~2 km) north of A (drops; would snap under the
    // old fixed 10 km cap).
    wrk.create_from_string(
        "pts.csv",
        "lat,lon\n0.01234,0.01234\n0.01234,0.03123\n0.02100,0.01000\n0.03800,0.01000\n",
    );
    wrk.create_from_string(
        "wards.geojson",
        r#"{"type":"FeatureCollection","features":[{"type":"Feature","properties":{"id":"A"},"geometry":{"type":"Polygon","coordinates":[[[0,0],[0,0.02],[0.02,0.02],[0.02,0],[0,0]]]}},{"type":"Feature","properties":{"id":"B"},"geometry":{"type":"Polygon","coordinates":[[[0.02,0],[0.02,0.02],[0.04,0.02],[0.04,0],[0.02,0]]]}}]}"#,
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
        "wards.geojson",
        "--feature-id-key",
        "properties.id",
    ]);
    let out = wrk.output(&mut cmd);
    assert!(out.status.success());
    let html = String::from_utf8_lossy(&out.stdout);
    // A = 1 contained + 1 snapped, B = 1 contained; the ~2 km stray dropped
    assert!(html.contains(r#""locations":["A","B"]"#));
    assert!(html.contains(r#""z":[2.0,1.0]"#));
    assert!(html.contains("includes 1 snapped from outside"));
    assert!(
        html.contains(
            "1 of 4 points snapped to the nearest region (≤0.31 km, auto-derived from region size \
             and coordinate precision)."
        ),
        "missing sub-km snapped note; html was: {html}"
    );
    assert!(
        html.contains("1 of 4 points were farther than 0.31 km from any region and were dropped."),
        "missing sub-km cap drop note; html was: {html}"
    );
    let stderr = String::from_utf8_lossy(&out.stderr);
    assert!(
        stderr.contains("(no region within 0.31 km)"),
        "stderr should name the derived sub-km cap; stderr was: {stderr}"
    );
}

// --language es localizes the below-map coverage note. This is the guard the English assertions
// above CANNOT provide: English is the catalog's own value, so reverting either coverage sentence
// to its old `format!` leaves every English assertion passing. Uses the ward-scale fixture because
// it is the one case where the snap sentence and the cap-drop sentence fire together.
//
// It also pins the deliberate asymmetry: the dashboard note is translated, the stderr diagnostic
// is NOT. Every one of this command's stderr messages is English, so localizing only the phrase
// they share would emit a half-Spanish English sentence.
#[test]
fn viz_choropleth_pip_coverage_note_localizes() {
    let wrk = Workdir::new("viz_choropleth_pip_coverage_note_localizes");
    wrk.create_from_string(
        "pts.csv",
        "lat,lon\n0.01234,0.01234\n0.01234,0.03123\n0.02100,0.01000\n0.03800,0.01000\n",
    );
    wrk.create_from_string(
        "wards.geojson",
        r#"{"type":"FeatureCollection","features":[{"type":"Feature","properties":{"id":"A"},"geometry":{"type":"Polygon","coordinates":[[[0,0],[0,0.02],[0.02,0.02],[0.02,0],[0,0]]]}},{"type":"Feature","properties":{"id":"B"},"geometry":{"type":"Polygon","coordinates":[[[0.02,0],[0.02,0.02],[0.04,0.02],[0.04,0],[0.02,0]]]}}]}"#,
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
        "wards.geojson",
        "--feature-id-key",
        "properties.id",
        "--language",
        "es",
    ]);
    let out = wrk.output(&mut cmd);
    assert!(out.status.success());
    let html = String::from_utf8_lossy(&out.stdout);
    assert!(
        html.contains(
            "1 de 4 puntos se ajustaron a la región más cercana (≤0.31 km, derivado del tamaño de \
             las regiones y de la precisión de las coordenadas)."
        ),
        "missing localized snap note; html was: {html}"
    );
    assert!(
        html.contains(
            "1 de 4 puntos estaban a más de 0.31 km de cualquier región y se descartaron."
        ),
        "missing localized cap-drop note; html was: {html}"
    );
    // no English fragment of either sentence survives, and no raw catalog key leaked
    for english in [
        "points snapped to the nearest region",
        "auto-derived from region size",
        "were farther than",
        "viz.notes.snap_coverage",
        "viz.notes.drop_coverage",
        "viz.notes.snap_basis_region_precision",
        "%{q_",
    ] {
        assert!(
            !html.contains(english),
            "{english:?} survived into the Spanish dashboard"
        );
    }
    // the stderr diagnostic stays English on purpose -- it is not part of the dashboard
    let stderr = String::from_utf8_lossy(&out.stderr);
    assert!(
        stderr.contains(
            "points were snapped to the nearest region (cap 0.31 km, auto-derived from region \
             size and coordinate precision)."
        ),
        "stderr should stay English; stderr was: {stderr}"
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

// a region-code column tagged with the canonical `geo.county_fips` concept (added to the
// describegpt/editor vocab) must be recognized as a summary-choropleth key: `viz smart --geojson`
// keys per-region aggregates off the FIPS column directly (no lat/lon). Before `county_fips` was
// added to REGION_CODE_LEAVES the column was silently excluded and no Regions choropleth appeared.
#[test]
fn viz_smart_summary_choropleth_county_fips_concept() {
    let wrk = Workdir::new("viz_smart_summary_choropleth_county_fips_concept");
    wrk.create_from_string(
        "counties.csv",
        "fips,pop\n42003,100\n42003,200\n36061,300\n36061,400\n",
    );
    wrk.create_from_string(
        "counties.geojson",
        r#"{"type":"FeatureCollection","features":[{"type":"Feature","id":"42003","properties":{},"geometry":{"type":"Polygon","coordinates":[[[0,0],[0,10],[10,10],[10,0],[0,0]]]}},{"type":"Feature","id":"36061","properties":{},"geometry":{"type":"Polygon","coordinates":[[[10,0],[10,10],[20,10],[20,0],[10,0]]]}}]}"#,
    );
    wrk.create_from_string(
        "dict.schema.json",
        r#"{
          "$schema": "https://json-schema.org/draft/2020-12/schema",
          "type": "object",
          "properties": {
            "fips": { "type": "string", "x-qsv": { "qsv_type": "String", "role": "dimension", "concept": "geo.county_fips" } },
            "pop": { "type": "number", "x-qsv": { "qsv_type": "Integer", "role": "measure", "concept": "measure.amount" } }
          }
        }"#,
    );

    let mut cmd = wrk.command("viz");
    cmd.args([
        "smart",
        "counties.csv",
        "--geojson",
        "counties.geojson",
        "--dictionary",
    ])
    .arg(wrk.path("dict.schema.json"));
    let out = wrk.output(&mut cmd);
    assert!(out.status.success());
    let html = String::from_utf8_lossy(&out.stdout);
    assert!(
        html.contains(r#""type":"choropleth""#) && html.contains(r#""locationmode":"geojson-id""#),
        "a geo.county_fips column should drive a summary choropleth keyed off the FIPS values: \
         {html}"
    );
    assert!(
        html.contains("count by fips"),
        "expected the summary choropleth 'count by fips' panel: {html}"
    );
}

// A summary choropleth + the data-viewer drawer emit the region-click -> SearchBuilder filter
// chrome: the hook marker, the region column riding as trace `meta`, and the feature-id -> raw
// spellings map (here the zero-padded "03103" whose raw cells read "3103"). The fixture holds
// BOTH spellings of the same region ("03103" and "3103"): the map must record only the
// non-canonical variant, and the chrome must carry the union step that adds the canonical id
// back into the SearchBuilder values (roborev #4026 — RAWS[loc] alone drops the
// canonically-spelled rows).
#[test]
fn viz_smart_choro_filter_chrome_emitted() {
    let wrk = Workdir::new("viz_smart_choro_filter_chrome_emitted");
    wrk.create_from_string(
        "counties.csv",
        "fips,pop\n42003,100\n42003,200\n3103,300\n3103,400\n03103,500\n",
    );
    wrk.create_from_string(
        "counties.geojson",
        r#"{"type":"FeatureCollection","features":[{"type":"Feature","id":"42003","properties":{},"geometry":{"type":"Polygon","coordinates":[[[0,0],[0,10],[10,10],[10,0],[0,0]]]}},{"type":"Feature","id":"03103","properties":{},"geometry":{"type":"Polygon","coordinates":[[[10,0],[10,10],[20,10],[20,0],[10,0]]]}}]}"#,
    );
    wrk.create_from_string(
        "dict.schema.json",
        r#"{
          "$schema": "https://json-schema.org/draft/2020-12/schema",
          "type": "object",
          "properties": {
            "fips": { "type": "string", "x-qsv": { "qsv_type": "String", "role": "dimension", "concept": "geo.county_fips" } },
            "pop": { "type": "number", "x-qsv": { "qsv_type": "Integer", "role": "measure", "concept": "measure.amount" } }
          }
        }"#,
    );

    let mut cmd = wrk.command("viz");
    cmd.env("QSV_VIZ_NO_COMPRESS", "1");
    cmd.args([
        "smart",
        "counties.csv",
        "--geojson",
        "counties.geojson",
        "--dictionary",
    ])
    .arg(wrk.path("dict.schema.json"));
    let out = wrk.output(&mut cmd);
    assert!(out.status.success());
    let html = String::from_utf8_lossy(&out.stdout);
    assert!(
        html.contains("window.__qsvChoroRehook = hook;"),
        "summary choropleth + drawer should emit the region-click filter chrome"
    );
    assert!(
        html.contains(r#""meta":0"#),
        "the region column index should ride on the choropleth trace as `meta`"
    );
    assert!(
        html.contains(r#""03103":["3103"]"#),
        "the feature-id -> raw spellings map should carry ONLY the non-canonical spelling, even \
         when the canonical one also appears in the data"
    );
    assert!(
        html.contains("[loc].concat(RAWS[loc])"),
        "critFor must union the canonical feature id with the variant spellings — RAWS[loc] alone \
         drops rows stored under the canonical spelling"
    );
}

// `--preview-threshold 0` disables the data viewer, so the region-click filter chrome (which
// needs the drawer end of the bridge) must not be emitted even though the choropleth exists.
#[test]
fn viz_smart_choro_filter_chrome_absent_without_drawer() {
    let wrk = Workdir::new("viz_smart_choro_filter_chrome_absent_without_drawer");
    wrk.create_from_string(
        "counties.csv",
        "fips,pop\n42003,100\n42003,200\n36061,300\n36061,400\n",
    );
    wrk.create_from_string(
        "counties.geojson",
        r#"{"type":"FeatureCollection","features":[{"type":"Feature","id":"42003","properties":{},"geometry":{"type":"Polygon","coordinates":[[[0,0],[0,10],[10,10],[10,0],[0,0]]]}},{"type":"Feature","id":"36061","properties":{},"geometry":{"type":"Polygon","coordinates":[[[10,0],[10,10],[20,10],[20,0],[10,0]]]}}]}"#,
    );
    wrk.create_from_string(
        "dict.schema.json",
        r#"{
          "$schema": "https://json-schema.org/draft/2020-12/schema",
          "type": "object",
          "properties": {
            "fips": { "type": "string", "x-qsv": { "qsv_type": "String", "role": "dimension", "concept": "geo.county_fips" } },
            "pop": { "type": "number", "x-qsv": { "qsv_type": "Integer", "role": "measure", "concept": "measure.amount" } }
          }
        }"#,
    );

    let mut cmd = wrk.command("viz");
    cmd.env("QSV_VIZ_NO_COMPRESS", "1");
    cmd.args([
        "smart",
        "counties.csv",
        "--geojson",
        "counties.geojson",
        "--preview-threshold",
        "0",
        "--dictionary",
    ])
    .arg(wrk.path("dict.schema.json"));
    let out = wrk.output(&mut cmd);
    assert!(out.status.success());
    let html = String::from_utf8_lossy(&out.stdout);
    assert!(
        html.contains(r#""type":"choropleth""#),
        "the summary choropleth itself should still render"
    );
    assert!(
        !html.contains("window.__qsvChoroRehook"),
        "no drawer -> no region-click filter chrome"
    );
}

// A point-in-polygon choropleth derives its regions from lat/lon — no CSV column holds the
// region values, so there is nothing to filter the data viewer BY: the region-click filter
// chrome must not be emitted even though the drawer is present.
#[test]
fn viz_smart_choro_filter_chrome_absent_for_pip() {
    let wrk = Workdir::new("viz_smart_choro_filter_chrome_absent_for_pip");
    wrk.create_from_string("pts.csv", "lat,lon,mag\n5,5,1\n6,6,2\n5,15,3\n6,16,4\n");
    wrk.create_from_string(
        "regions.geojson",
        r#"{"type":"FeatureCollection","features":[{"type":"Feature","properties":{"id":"A"},"geometry":{"type":"Polygon","coordinates":[[[0,0],[0,10],[10,10],[10,0],[0,0]]]}},{"type":"Feature","properties":{"id":"B"},"geometry":{"type":"Polygon","coordinates":[[[10,0],[10,10],[20,10],[20,0],[10,0]]]}}]}"#,
    );

    let mut cmd = wrk.command("viz");
    cmd.env("QSV_VIZ_NO_COMPRESS", "1");
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
    assert!(
        html.contains(r#""type":"choropleth""#),
        "the PIP choropleth itself should render"
    );
    assert!(
        !html.contains("window.__qsvChoroRehook"),
        "a PIP choropleth has no region column -> no region-click filter chrome"
    );
}

// when the `viz smart` PIP choropleth snaps any points, the panel title surfaces the snap
// metadata (count + the cap applied) — a sub-panel has no below-map annotation surface, and the
// note must not sit on the map itself.
#[test]
fn viz_smart_pip_choropleth_snapped_title() {
    let wrk = Workdir::new("viz_smart_pip_choropleth_snapped_title");
    // A = lon 0..10, B = lon 10..20 (both lat 0..10); the (10.05, 5) stray is ~5.6 km north of A.
    // Auto cap: integer lons -> 0-decimal precision floor (~157 km) and 10-deg regions
    // (~157 km region term) both clamp to 100 km, so the stray snaps into A.
    wrk.create_from_string("pts.csv", "lat,lon\n5,5\n6,6\n5,15\n6,16\n10.05,5\n");
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
    assert!(
        html.contains("Regions (1 snapped ≤100 km)"),
        "smart panel title should carry the snap metadata; html was: {html}"
    );
    let stderr = String::from_utf8_lossy(&out.stderr);
    assert!(
        stderr.contains(
            "viz smart: 1 of 5 points were snapped to the nearest region (cap 100 km, \
             auto-derived from region size and coordinate precision)."
        ),
        "stderr should state the derived cap + provenance; stderr was: {stderr}"
    );
}

// A metro-scale `viz smart` choropleth renders on a MapLibre tile basemap (ChoroplethMap). Its fill
// is inserted ABOVE the basemap layers (below="") so the basemap's roads don't bleed through and
// muddy the region colors, and the fill is near-opaque for a clean read.
#[test]
fn viz_smart_choropleth_map_fill_above_basemap_roads() {
    let wrk = Workdir::new("viz_smart_choropleth_map_fill_above_basemap_roads");
    // a tight metro extent (< SMART_CHOROPLETH_MIN_SPAN_DEG in both dims) -> tile ChoroplethMap
    wrk.create_from_string(
        "pts.csv",
        "lat,lon\n40.44,-74.00\n40.45,-74.00\n40.44,-73.90\n40.45,-73.90\n",
    );
    wrk.create_from_string(
        "regions.geojson",
        r#"{"type":"FeatureCollection","features":[{"type":"Feature","properties":{"id":"A"},"geometry":{"type":"Polygon","coordinates":[[[-74.05,40.40],[-74.05,40.50],[-73.95,40.50],[-73.95,40.40],[-74.05,40.40]]]}},{"type":"Feature","properties":{"id":"B"},"geometry":{"type":"Polygon","coordinates":[[[-73.95,40.40],[-73.95,40.50],[-73.85,40.50],[-73.85,40.40],[-73.95,40.40]]]}}]}"#,
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
    assert!(
        html.contains(r#""type":"choroplethmap""#),
        "expected a tile ChoroplethMap at metro scale: {html}"
    );
    // below="" lifts the fill over the basemap's road layers (the fix for road bleed-through)
    assert!(
        html.contains(r#""below":"""#),
        "choropleth fill must be inserted above the basemap roads (below=\"\"): {html}"
    );
    assert!(
        html.contains(r#""opacity":0.9"#),
        "choropleth fill should be near-opaque: {html}"
    );
}

// A metro-scale `viz smart` choropleth opens on the LABELED carto basemap (place names aid
// orientation in the inter-polygon gaps) and carries a "Basemap labels" toggle that relayouts the
// `map` subplot's style between the labeled and label-free carto variants. Default light theme, so
// the baked args reference the carto-positron pair.
#[test]
fn viz_smart_choropleth_map_basemap_labels_toggle() {
    let wrk = Workdir::new("viz_smart_choropleth_map_basemap_labels_toggle");
    // a tight metro extent (< SMART_CHOROPLETH_MIN_SPAN_DEG in both dims) -> tile ChoroplethMap
    wrk.create_from_string(
        "pts.csv",
        "lat,lon\n40.44,-74.00\n40.45,-74.00\n40.44,-73.90\n40.45,-73.90\n",
    );
    wrk.create_from_string(
        "regions.geojson",
        r#"{"type":"FeatureCollection","features":[{"type":"Feature","properties":{"id":"A"},"geometry":{"type":"Polygon","coordinates":[[[-74.05,40.40],[-74.05,40.50],[-73.95,40.50],[-73.95,40.40],[-74.05,40.40]]]}},{"type":"Feature","properties":{"id":"B"},"geometry":{"type":"Polygon","coordinates":[[[-73.95,40.40],[-73.95,40.50],[-73.85,40.50],[-73.85,40.40],[-73.95,40.40]]]}}]}"#,
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
    // the choropleth defaults to the LABELED basemap (carto-positron, not -nolabels)
    assert!(
        html.contains(r#""style":"carto-positron""#),
        "choropleth basemap should default to labeled carto-positron: {html}"
    );
    // the "Basemap labels" toggle: args restore labels (labeled style), args2 hide them (-nolabels)
    assert!(
        html.contains(r#""label":"Basemap labels""#),
        "expected a Basemap labels toggle on the choropleth panel: {html}"
    );
    assert!(
        html.contains(r#""args":[{"map.style":"carto-positron"}]"#),
        "Basemap labels args must relayout map.style to the labeled variant: {html}"
    );
    assert!(
        html.contains(r#""args2":[{"map.style":"carto-positron-nolabels"}]"#),
        "Basemap labels args2 must relayout map.style to the label-free variant: {html}"
    );
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

// The fullscreen modebar button is injected as client-side JS (the plotly-rs `Configuration` can't
// carry a JS `click` handler). These assert the injected chrome is present in both HTML paths: the
// plain single-chart document (`Plot::to_html`) and the hand-assembled `viz smart` dashboard.

#[test]
fn viz_single_chart_has_fullscreen_button() {
    let wrk = Workdir::new("viz_single_chart_has_fullscreen_button");
    fruits(&wrk);

    let mut cmd = wrk.command("viz");
    cmd.args(["scatter", "fruits.csv", "--x", "Price", "--y", "Qty"]);
    let out = wrk.output(&mut cmd);
    assert!(out.status.success());

    let html = String::from_utf8_lossy(&out.stdout);
    // plotly's own render is still present...
    assert!(html.contains("Plotly.newPlot"));
    // ...plus the injected fullscreen chrome: the custom modebar button and the toggle logic.
    assert!(html.contains("modeBarButtonsToAdd"));
    assert!(html.contains(r#"name: "qsv-fullscreen""#));
    assert!(html.contains("requestFullscreen"));
    assert!(html.contains(".js-plotly-plot:fullscreen"));
    // ...plus the legend-toggle button.
    assert!(html.contains(r#"name: "qsv-legend""#));
    assert!(html.contains("Toggle legend"));
}

#[test]
fn viz_smart_has_fullscreen_button() {
    let wrk = Workdir::new("viz_smart_has_fullscreen_button");
    fruits(&wrk);

    let out_html = wrk.path("dash.html").to_string_lossy().to_string();
    let mut cmd = wrk.command("viz");
    cmd.args(["smart", "fruits.csv", "-o", &out_html]);
    wrk.assert_success(&mut cmd);

    let html = wrk.read_to_string("dash.html").unwrap();
    // the smart page carries both the (pre-existing) theme toggle and the fullscreen button.
    assert!(html.contains("qsv-theme-toggle"));
    assert!(html.contains("modeBarButtonsToAdd"));
    assert!(html.contains(r#"name: "qsv-fullscreen""#));
    assert!(html.contains("requestFullscreen"));
    assert!(html.contains(".js-plotly-plot:fullscreen"));
    // ...plus the legend-toggle button.
    assert!(html.contains(r#"name: "qsv-legend""#));
    assert!(html.contains("Toggle legend"));
}

// The map zoom auto-fit recomputes MapLibre zoom for the real container size from the baked
// (assumed-px) zoom on initial display (`applyFitLayout` before the render newPlot) and on
// fullscreen toggle (`applyFitCamera` via the GL camera). These assert the client-side fit logic
// and the per-render-path assumed-dims prelude are emitted.

#[test]
fn viz_map_has_zoom_autofit() {
    let wrk = Workdir::new("viz_map_has_zoom_autofit");
    quakes(&wrk);

    let mut cmd = wrk.command("viz");
    cmd.args(["map", "quakes.csv", "--lat", "lat", "--lon", "lon"]);
    let out = wrk.output(&mut cmd);
    assert!(out.status.success());

    let html = String::from_utf8_lossy(&out.stdout);
    // the fit helpers (layout pre-mutation + GL-camera) + logarithmic zoom math...
    assert!(html.contains("applyFitLayout"));
    assert!(html.contains("applyFitCamera"));
    assert!(html.contains("Math.log2"));
    // ...and the standalone assumed-dims prelude (fit_dims HTML default = 1000x600).
    assert!(html.contains("window.__qsvMapAssumedW=1000"));
    assert!(html.contains("window.__qsvMapAssumedH=600"));
}

#[test]
fn viz_smart_map_has_zoom_autofit() {
    let wrk = Workdir::new("viz_smart_map_has_zoom_autofit");
    quakes(&wrk);

    let out_html = wrk.path("dash.html").to_string_lossy().to_string();
    let mut cmd = wrk.command("viz");
    cmd.args(["smart", "quakes.csv", "-o", &out_html]);
    wrk.assert_success(&mut cmd);

    let html = wrk.read_to_string("dash.html").unwrap();
    assert!(html.contains("applyFitLayout"));
    assert!(html.contains("applyFitCamera"));
    assert!(html.contains("Math.log2"));
    // Core/Full extent buttons re-fit to the current container size via the buttonclicked handler.
    assert!(html.contains("plotly_buttonclicked"));
    // theme toggle restyles MapLibre map traces via newPlot, not relayout (the fork's relayout
    // blanks them).
    assert!(html.contains("hasMapLibre"));
    // smart panels frame against MAP_PANEL_ASSUMED_WIDTH_PX x MAP_PANEL_USABLE_HEIGHT_PX (960x472;
    // MAP_ROW_HEIGHT_PX 540 minus the 48+20 plotly margins).
    assert!(html.contains("window.__qsvMapAssumedW=960"));
    assert!(html.contains("window.__qsvMapAssumedH=472"));
}

// --- `viz smart --bivariate` ----------------------------------------------------------------
// NMI-driven association heatmap + ranked top relationships, sourced from moarstats'
// `--bivariate` sidecar CSV. `--bivariate` implies `--smarter` and (when `--dictionary` isn't
// otherwise set) `--dictionary infer`; most of these tests pass an explicit (often trivial)
// `--dictionary` file so they don't depend on a live LLM being configured, mirroring the existing
// `--dictionary infer` tests' convention of preferring a file dictionary for determinism/speed.

#[test]
fn viz_smart_bivariate_implies_smarter_and_dictionary() {
    let wrk = Workdir::new("viz_smart_bivariate_implies_smarter_and_dictionary");
    // two categorical columns deterministically related (code -> status), so NMI = 1.0 and the
    // association heatmap has something to show.
    let mut rows = String::from("code,status\n");
    for i in 0..80 {
        let code = ["A", "B", "C", "D"][i % 4];
        let status = code; // deterministic mapping
        rows.push_str(&format!("{code},{status}\n"));
    }
    wrk.create_from_string("assoc.csv", &rows);

    // deliberately NO --smarter and NO --dictionary: --bivariate alone must imply both.
    let out_html = wrk.path("dash.html").to_string_lossy().to_string();
    let mut cmd = wrk.command("viz");
    cmd.args(["smart", "assoc.csv", "--bivariate", "-o", &out_html]);
    wrk.assert_success(&mut cmd);

    let html = wrk.read_to_string("dash.html").unwrap();
    // the association heatmap only exists when moarstats' `--bivariate` sidecar was produced,
    // which only happens via the moarstats subprocess that `--smarter` enables -- so its presence
    // proves --bivariate implied --smarter.
    assert!(
        html.contains("Association (NMI)") && html.contains(r#""name":"association""#),
        "expected the NMI association heatmap to be built from an implied --smarter moarstats \
         run; html: {html}"
    );
}

#[test]
fn viz_smart_bivariate_respects_explicit_dictionary() {
    let wrk = Workdir::new("viz_smart_bivariate_respects_explicit_dictionary");
    let mut rows = String::from("code,status\n");
    for i in 0..80 {
        let code = ["A", "B", "C", "D"][i % 4];
        let status = code;
        rows.push_str(&format!("{code},{status}\n"));
    }
    wrk.create_from_string("assoc.csv", &rows);
    // a distinctive title an LLM `--dictionary infer` pass is vanishingly unlikely to produce
    // verbatim, so its presence proves the EXPLICIT file dictionary was used and NOT silently
    // replaced by `--dictionary infer`.
    wrk.create_from_string(
        "dict.schema.json",
        r#"{
          "$schema": "https://json-schema.org/draft/2020-12/schema",
          "type": "object",
          "properties": {
            "code": { "type": "string", "title": "Qsv Test Custom Region Label ABC123",
              "x-qsv": { "qsv_type": "String", "role": "dimension", "concept": "category.status" } },
            "status": { "type": "string", "title": "Status",
              "x-qsv": { "qsv_type": "String", "role": "dimension", "concept": "category.status" } }
          }
        }"#,
    );

    let out_html = wrk.path("dash.html").to_string_lossy().to_string();
    let mut cmd = wrk.command("viz");
    cmd.args([
        "smart",
        "assoc.csv",
        "--bivariate",
        "-o",
        &out_html,
        "--dictionary",
    ])
    .arg(wrk.path("dict.schema.json"));
    wrk.assert_success(&mut cmd);

    let html = wrk.read_to_string("dash.html").unwrap();
    // the explicit --dictionary's custom label must still be USED somewhere (proving it wasn't
    // silently replaced by --dictionary infer) — it drives per-column panel titles/subtitles.
    assert!(
        html.contains("Qsv Test Custom Region Label ABC123"),
        "explicit --dictionary's custom label should be used, not be replaced by --dictionary \
         infer; html: {html}"
    );
    assert!(html.contains(r#""name":"association""#));
    // ...but the NMI Association heatmap axes use the RAW field names (code/status), NOT the
    // dictionary label — matching the Correlation heatmap's convention.
    assert!(
        html.contains(r#""x":["code","status"]"#) && html.contains(r#""y":["code","status"]"#),
        "Association (NMI) axes should use raw field names, not the dictionary label; html: {html}"
    );
}

#[test]
fn viz_smart_bivariate_assoc_heatmap_categorical_pair() {
    let wrk = Workdir::new("viz_smart_bivariate_assoc_heatmap_categorical_pair");
    // two purely categorical columns, near-deterministically mapped (strong NMI) -- with no
    // numeric columns at all, a Pearson CorrHeatmap can never be built, so any heatmap trace in
    // the output must be the new NMI AssocHeatmap.
    let mut rows = String::from("region,tier\n");
    for i in 0..120 {
        let region = ["North", "South", "East", "West"][i % 4];
        let tier = region; // deterministic -> NMI = 1.0
        rows.push_str(&format!("{region},{tier}\n"));
    }
    wrk.create_from_string("catpair.csv", &rows);
    // trivial dictionary (no x-qsv routing) keeps --dictionary explicit (skipping the slow LLM
    // `infer` path) while leaving stats-only routing unchanged.
    wrk.create_from_string(
        "trivial.schema.json",
        r#"{"$schema":"https://json-schema.org/draft/2020-12/schema","type":"object","properties":{}}"#,
    );

    let out_html = wrk.path("dash.html").to_string_lossy().to_string();
    let mut cmd = wrk.command("viz");
    cmd.args([
        "smart",
        "catpair.csv",
        "--bivariate",
        "-o",
        &out_html,
        "--dictionary",
    ])
    .arg(wrk.path("trivial.schema.json"));
    wrk.assert_success(&mut cmd);

    let html = wrk.read_to_string("dash.html").unwrap();
    assert!(
        html.contains(r#""name":"association""#),
        "a categorical/categorical pair should still produce an NMI association heatmap; html: \
         {html}"
    );
    // no numeric columns exist, so no Pearson correlation heatmap should have been built
    assert!(!html.contains(r#""name":"correlation""#));
}

// Build the canonical many-to-many categorical fixture: dept→channel with a 70/30 primary/secondary
// split that cycles (A→P/Q, B→Q/R, C→R/P). Each dept fans out to 2 channels and each channel is fed
// by 2 depts, so the pair is associated but genuinely many-to-many (Theil's U ~0.44 both ways —
// inside the Sankey band, well below the near-functional cutoff). Both columns have cardinality 3.
// `channel_first` controls CSV column order so a test can distinguish concept-based orientation
// from the equal-cardinality column-order fallback.
fn many_to_many_dept_channel(channel_first: bool) -> String {
    let channels = ["P", "Q", "R"];
    let depts = ["A", "B", "C"];
    let mut rows = String::from(if channel_first {
        "channel,dept\n"
    } else {
        "dept,channel\n"
    });
    for d in 0..3 {
        let primary = channels[d];
        let secondary = channels[(d + 1) % 3];
        for k in 0..50 {
            let ch = if k < 35 { primary } else { secondary };
            let dept = depts[d];
            if channel_first {
                rows.push_str(&format!("{ch},{dept}\n"));
            } else {
                rows.push_str(&format!("{dept},{ch}\n"));
            }
        }
    }
    rows
}

#[test]
fn viz_smart_bivariate_sankey_many_to_many() {
    let wrk = Workdir::new("viz_smart_bivariate_sankey_many_to_many");
    wrk.create_from_string("flow.csv", &many_to_many_dept_channel(false));
    wrk.create_from_string(
        "trivial.schema.json",
        r#"{"$schema":"https://json-schema.org/draft/2020-12/schema","type":"object","properties":{}}"#,
    );

    let out_html = wrk.path("dash.html").to_string_lossy().to_string();
    let mut cmd = wrk.command("viz");
    cmd.args([
        "smart",
        "flow.csv",
        "--bivariate",
        "-o",
        &out_html,
        "--dictionary",
    ])
    .arg(wrk.path("trivial.schema.json"));
    wrk.assert_success(&mut cmd);

    let html = wrk.read_to_string("dash.html").unwrap();
    // a genuinely many-to-many categorical pair yields a directed-flow Sankey panel
    assert!(
        html.contains(r#""type":"sankey""#),
        "a many-to-many categorical pair should produce a Sankey panel; html: {html}"
    );
    // mutual exclusivity: the 2-level treemap of exactly this pair is suppressed in favor of the
    // flow view, so no treemap/sunburst hierarchy trace should appear.
    assert!(
        !html.contains(r#""type":"treemap""#) && !html.contains(r#""type":"sunburst""#),
        "the 2-dim hierarchy should be suppressed when the Sankey owns the pair; html: {html}"
    );
}

#[test]
fn viz_smart_bivariate_sankey_concept_orients_to_status() {
    let wrk = Workdir::new("viz_smart_bivariate_sankey_concept_orients_to_status");
    // Put `channel` FIRST in the CSV. Both columns have cardinality 3, so the equal-cardinality
    // column-order fallback would orient the pair `channel -> dept` (source = column 0). The
    // dictionary tags `channel` as `category.status`, which must override that and force the status
    // column to the TARGET, flipping the orientation to `dept -> channel`. Asserting on the flipped
    // title therefore proves the concept-based orientation actually fired (and the dict parsed) --
    // it is not reachable from the cardinality fallback.
    wrk.create_from_string("flow.csv", &many_to_many_dept_channel(true));
    // dictionary tags `channel` as category.status (the outcome) and `dept` as category.type.
    wrk.create_from_string(
        "dict.schema.json",
        r#"{"$schema":"https://json-schema.org/draft/2020-12/schema","type":"object","properties":{
            "dept":{"type":"string","x-qsv":{"qsv_type":"String","role":"dimension","concept":"category.type"}},
            "channel":{"type":"string","x-qsv":{"qsv_type":"String","role":"dimension","concept":"category.status"}}
        }}"#,
    );

    let out_html = wrk.path("dash.html").to_string_lossy().to_string();
    let mut cmd = wrk.command("viz");
    cmd.args([
        "smart",
        "flow.csv",
        "--bivariate",
        "-o",
        &out_html,
        "--dictionary",
    ])
    .arg(wrk.path("dict.schema.json"));
    wrk.assert_success(&mut cmd);

    let html = wrk.read_to_string("dash.html").unwrap();
    assert!(
        html.contains(r#""type":"sankey""#),
        "Sankey expected; html: {html}"
    );
    // the panel title is "<source> → <target>"; the category.status column must be the target.
    // The dictionary labels default to the field names here, so the title reads "dept → channel".
    // The cardinality fallback would instead read "channel → dept" (channel is column 0), so this
    // assertion fails unless the concept-based orientation flipped it.
    assert!(
        html.contains("dept \u{2192} channel"),
        "the category.status column (channel) should be oriented as the flow target; html: {html}"
    );
    // guard the discriminator: the fallback orientation must NOT appear.
    assert!(
        !html.contains("channel \u{2192} dept"),
        "concept orientation must override the equal-cardinality column-order fallback; html: \
         {html}"
    );
}

#[test]
fn viz_smart_bivariate_top_relationships_ranks_beyond_strongest_pair() {
    let wrk = Workdir::new("viz_smart_bivariate_top_relationships_ranks_beyond_strongest_pair");
    // 10 categorical columns: g1..g4 are all deterministically related to each other (NMI ~ 1.0
    // for every pair among them), the rest are independent random noise -- more than
    // CORR_INCELL_MAX_N (8) columns participate in surviving pairs, so the ranked
    // "top relationships" bar is built, and it must rank MORE than just the single strongest pair.
    let mut rows = String::from("g1,g2,g3,g4,noise1,noise2,noise3,noise4,noise5,noise6\n");
    let cats = ["A", "B", "C", "D"];
    for i in 0..300 {
        let g1 = cats[i % 4];
        // g2/g4 track g1/g3 on all but a handful of rows: NMI stays ~1.0 with full 300-row
        // support, but the relation is NOT a bijection, so the 1:1 collapse (issue #4221) leaves
        // both columns charted. An exact duplicate is literally the same variable and folds into
        // a single panel, taking with it the well-supported high-NMI pairs these tests rank.
        let g2 = if i % 50 == 0 { cats[(i + 1) % 4] } else { g1 };
        // block-stepped, NOT `(i * 3) % 4`: multiplying a linear sequence by a coprime factor
        // permutes it, so the old g3 was a relabeling of g1 and folded into it. The divisor sits
        // clear of the noise columns' 2..=7 range so it does not collide with one of them either.
        let g3 = cats[(i / 11) % 4];
        let g4 = if i % 50 == 25 { cats[(i + 1) % 4] } else { g3 };
        // genuine filler: a per-column BLOCK size rather than a per-column offset. Offsets of a
        // linear sequence are rotations of one another, so every "noise" column was a relabeling
        // of g1 and all ten collapsed into one panel.
        let noise: Vec<&str> = (0..6).map(|k| cats[(i / (k + 2)) % 4]).collect();
        rows.push_str(&format!(
            "{g1},{g2},{g3},{g4},{},{},{},{},{},{}\n",
            noise[0], noise[1], noise[2], noise[3], noise[4], noise[5]
        ));
    }
    wrk.create_from_string("wide_assoc.csv", &rows);
    wrk.create_from_string(
        "trivial.schema.json",
        r#"{"$schema":"https://json-schema.org/draft/2020-12/schema","type":"object","properties":{}}"#,
    );

    let out_html = wrk.path("dash.html").to_string_lossy().to_string();
    let mut cmd = wrk.command("viz");
    cmd.args([
        "smart",
        "wide_assoc.csv",
        "--bivariate",
        "-o",
        &out_html,
        "--dictionary",
    ])
    .arg(wrk.path("trivial.schema.json"));
    wrk.assert_success(&mut cmd);

    let html = wrk.read_to_string("dash.html").unwrap();
    assert!(
        html.contains(r#""name":"Top Relationships (NMI)""#),
        "expected a ranked top-relationships bar with > 8 chartable columns; html: {html}"
    );
    // each ranked entry's label is "FieldA × FieldB" -- at least two distinct '×'-joined entries
    // means the ranking goes beyond the single strongest pair.
    let times_count = html.matches('\u{d7}').count();
    assert!(
        times_count >= 2,
        "expected more than one ranked relationship entry (found {times_count} '×' labels); html: \
         {html}"
    );
}

#[test]
fn viz_smart_bivariate_top_relationships_lollipop_encodings() {
    let wrk = Workdir::new("viz_smart_bivariate_top_relationships_lollipop_encodings");
    // same wide-categorical shape as the ranking test (>8 chartable columns → the panel is built),
    // but here we assert the panel is rendered as the multivariate LOLLIPOP: a marker scatter with
    // asymmetric x error-bar stems and a per-dot size array (support encoding), NOT a plain bar.
    let mut rows = String::from("g1,g2,g3,g4,noise1,noise2,noise3,noise4,noise5,noise6\n");
    let cats = ["A", "B", "C", "D"];
    for i in 0..300 {
        let g1 = cats[i % 4];
        // g2/g4 track g1/g3 on all but a handful of rows: NMI stays ~1.0 with full 300-row
        // support, but the relation is NOT a bijection, so the 1:1 collapse (issue #4221) leaves
        // both columns charted. An exact duplicate is literally the same variable and folds into
        // a single panel, taking with it the well-supported high-NMI pairs these tests rank.
        let g2 = if i % 50 == 0 { cats[(i + 1) % 4] } else { g1 };
        // block-stepped, NOT `(i * 3) % 4`: multiplying a linear sequence by a coprime factor
        // permutes it, so the old g3 was a relabeling of g1 and folded into it. The divisor sits
        // clear of the noise columns' 2..=7 range so it does not collide with one of them either.
        let g3 = cats[(i / 11) % 4];
        let g4 = if i % 50 == 25 { cats[(i + 1) % 4] } else { g3 };
        // genuine filler: a per-column BLOCK size rather than a per-column offset. Offsets of a
        // linear sequence are rotations of one another, so every "noise" column was a relabeling
        // of g1 and all ten collapsed into one panel.
        let noise: Vec<&str> = (0..6).map(|k| cats[(i / (k + 2)) % 4]).collect();
        rows.push_str(&format!(
            "{g1},{g2},{g3},{g4},{},{},{},{},{},{}\n",
            noise[0], noise[1], noise[2], noise[3], noise[4], noise[5]
        ));
    }
    wrk.create_from_string("wide_assoc.csv", &rows);
    wrk.create_from_string(
        "trivial.schema.json",
        r#"{"$schema":"https://json-schema.org/draft/2020-12/schema","type":"object","properties":{}}"#,
    );

    let out_html = wrk.path("dash.html").to_string_lossy().to_string();
    let mut cmd = wrk.command("viz");
    cmd.args([
        "smart",
        "wide_assoc.csv",
        "--bivariate",
        "-o",
        &out_html,
        "--dictionary",
    ])
    .arg(wrk.path("trivial.schema.json"));
    wrk.assert_success(&mut cmd);

    let html = wrk.read_to_string("dash.html").unwrap();
    // locate the Top Relationships trace object and assert the lollipop shape within its window.
    let anchor = html
        .find(r#""name":"Top Relationships (NMI)""#)
        .expect("Top Relationships panel should be present");
    let window = &html[anchor.saturating_sub(200)..(anchor + 2000).min(html.len())];
    assert!(
        window.contains(r#""type":"scatter""#) && window.contains(r#""mode":"markers""#),
        "Top Relationships should render as a marker scatter (lollipop), not a bar; window: \
         {window}"
    );
    assert!(
        window.contains(r#""error_x""#) && window.contains(r#""arrayminus""#),
        "lollipop stems (asymmetric x error bars) should be present; window: {window}"
    );
    assert!(
        window.contains(r#""size":["#),
        "dot SIZE should encode support (a per-point size array); window: {window}"
    );
}

#[test]
fn viz_smart_bivariate_ignores_stale_sidecar() {
    let wrk = Workdir::new("viz_smart_bivariate_ignores_stale_sidecar");
    // A single-column input makes moarstats --bivariate produce ZERO pairs, and moarstats then
    // does NOT (re)write the deterministic `<stem>.stats.bivariate.csv` sidecar. So a sidecar left
    // by a PRIOR run (on since-changed data) would otherwise be read as if it described this input.
    // The fix deletes the expected sidecar before moarstats runs, so a pair-less/failed run leaves
    // no sidecar to reuse.
    wrk.create_from_string("oned.csv", "a\n1\n2\n3\n2\n1\n4\n5\n2\n1\n3\n");
    // plant a stale sidecar carrying recognizable ghost pairs that WOULD parse if read.
    wrk.create_from_string(
        "oned.stats.bivariate.csv",
        "field1,field2,normalized_mutual_information,pearson_correlation,spearman_correlation,\
         n_pairs\nghost_field_x,ghost_field_y,0.99,0.5,0.5,100\n",
    );
    wrk.create_from_string(
        "trivial.schema.json",
        r#"{"$schema":"https://json-schema.org/draft/2020-12/schema","type":"object","properties":{}}"#,
    );

    let out_html = wrk.path("dash.html").to_string_lossy().to_string();
    let mut cmd = wrk.command("viz");
    cmd.args([
        "smart",
        "oned.csv",
        "--bivariate",
        "-o",
        &out_html,
        "--dictionary",
    ])
    .arg(wrk.path("trivial.schema.json"));
    wrk.assert_success(&mut cmd);

    // the stale sidecar must have been removed before the (pair-less) moarstats run — moarstats
    // never rewrote it, so its continued existence would mean viz reused stale data.
    assert!(
        !wrk.path("oned.stats.bivariate.csv").exists(),
        "stale bivariate sidecar should be deleted before the moarstats run, not reused"
    );
    // and no association panel sourced from the stale ghost pairs may appear.
    let html = wrk.read_to_string("dash.html").unwrap();
    assert!(
        !html.contains("ghost_field_x"),
        "stale ghost pair must not be charted; html: {html}"
    );
    assert!(
        !html.contains("Association (NMI)") && !html.contains("Top Relationships (NMI)"),
        "no bivariate panels should render for a pair-less input; html: {html}"
    );
}

#[test]
fn viz_smart_bivariate_pii_column_excluded() {
    let wrk = Workdir::new("viz_smart_bivariate_pii_column_excluded");
    // region/tier are strongly associated (NMI = 1.0); email is a low-cardinality (non-near-
    // unique, so moarstats still computes its pairs) PII column that the dictionary tags as an
    // identifier.
    let mut rows = String::from("region,tier,email\n");
    for i in 0..80 {
        let region = ["East", "West"][i % 2];
        let tier = region;
        let domain = ["a.com", "b.com", "c.com", "d.com"][i % 4];
        rows.push_str(&format!("{region},{tier},{domain}\n"));
    }
    wrk.create_from_string("pii.csv", &rows);
    wrk.create_from_string(
        "pii_dict.schema.json",
        r#"{
          "$schema": "https://json-schema.org/draft/2020-12/schema",
          "type": "object",
          "properties": {
            "region": { "type": "string", "title": "Region",
              "x-qsv": { "qsv_type": "String", "role": "dimension", "concept": "category.status" } },
            "tier": { "type": "string", "title": "Tier",
              "x-qsv": { "qsv_type": "String", "role": "dimension", "concept": "category.status" } },
            "email": { "type": "string", "title": "Email Address",
              "x-qsv": { "qsv_type": "String", "role": "identifier", "concept": "pii.email" } }
          }
        }"#,
    );

    let out_html = wrk.path("dash.html").to_string_lossy().to_string();
    let mut cmd = wrk.command("viz");
    // the data viewer drawer embeds ALL raw columns by design (it is the underlying table, not a
    // panel), so it would legitimately carry "email"; disable it — this test pins down panel
    // exclusion specifically.
    cmd.args([
        "smart",
        "pii.csv",
        "--bivariate",
        "--preview-threshold",
        "0",
        "-o",
        &out_html,
        "--dictionary",
    ])
    .arg(wrk.path("pii_dict.schema.json"));
    wrk.assert_success(&mut cmd);

    let html = wrk.read_to_string("dash.html").unwrap();
    assert!(
        html.contains(r#""name":"association""#),
        "the region/tier pair should still produce an association heatmap; html: {html}"
    );
    assert!(
        !html.contains("email") && !html.contains("Email"),
        "the PII-tagged identifier column must not appear in any panel, including the new \
         bivariate ones; html: {html}"
    );
}

#[test]
fn viz_smart_bivariate_skips_on_wide_dataset() {
    let wrk = Workdir::new("viz_smart_bivariate_skips_on_wide_dataset");
    // 55 columns > BIVARIATE_MAX_COLUMNS (50)
    let n_cols = 55;
    let mut rows = (0..n_cols)
        .map(|i| format!("c{i}"))
        .collect::<Vec<_>>()
        .join(",");
    rows.push('\n');
    for r in 0..20 {
        let row: Vec<String> = (0..n_cols).map(|i| ((r + i) % 10).to_string()).collect();
        rows.push_str(&row.join(","));
        rows.push('\n');
    }
    wrk.create_from_string("wide55.csv", &rows);
    wrk.create_from_string(
        "trivial.schema.json",
        r#"{"$schema":"https://json-schema.org/draft/2020-12/schema","type":"object","properties":{}}"#,
    );

    let out_html = wrk.path("dash.html").to_string_lossy().to_string();
    let mut cmd = wrk.command("viz");
    cmd.args([
        "smart",
        "wide55.csv",
        "--bivariate",
        "-o",
        &out_html,
        "--dictionary",
    ])
    .arg(wrk.path("trivial.schema.json"));
    let out = wrk.output(&mut cmd);
    assert!(
        out.status.success(),
        "the dashboard should still succeed overall, just without the bivariate panels"
    );
    let stderr = String::from_utf8_lossy(&out.stderr);
    assert!(
        stderr.contains("exceeds the 50-column cap"),
        "expected a column-cap warning naming the 50-column limit; stderr: {stderr}"
    );

    let html = wrk.read_to_string("dash.html").unwrap();
    assert!(
        !html.contains(r#""name":"association""#) && !html.contains("Top Relationships (NMI)"),
        "the bivariate panels must not appear once the column cap is exceeded; html: {html}"
    );
}

#[test]
fn viz_smart_bivariate_soft_fails_with_no_headers() {
    let wrk = Workdir::new("viz_smart_bivariate_soft_fails_with_no_headers");
    // moarstats can't honor --no-headers (same constraint --smarter already documents), so
    // --bivariate's moarstats sidecar is never produced for a --no-headers input; the dashboard
    // must still render successfully via the standard (non-enriched) path.
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
    wrk.create_from_string(
        "trivial.schema.json",
        r#"{"$schema":"https://json-schema.org/draft/2020-12/schema","type":"object","properties":{}}"#,
    );

    let out_html = wrk.path("headerless.html").to_string_lossy().to_string();
    let mut cmd = wrk.command("viz");
    cmd.args([
        "smart",
        "headerless.csv",
        "--bivariate",
        "--no-headers",
        "-o",
        &out_html,
        "--dictionary",
    ])
    .arg(wrk.path("trivial.schema.json"));
    wrk.assert_success(&mut cmd);

    let html = wrk.read_to_string("headerless.html").unwrap();
    assert!(
        html.contains("Plotly.newPlot"),
        "fallback dashboard should still render; html: {html}"
    );
    assert!(
        !html.contains(r#""name":"association""#),
        "no bivariate panels should be built when moarstats enrichment is skipped under \
         --no-headers; html: {html}"
    );
}

#[test]
fn viz_smart_bivariate_flags_nonlinear_relationship() {
    let wrk = Workdir::new("viz_smart_bivariate_flags_nonlinear_relationship");
    // y = exp(x/3) over a repeated x range: strictly monotonic (Spearman rho = 1.0) but sharply
    // curved, so Pearson r is markedly lower (~0.70) -- a textbook "nonlinear" divergence. x is
    // repeated 5x (cardinality 30 over 150 rows) so moarstats' cardinality == rowcount filter
    // doesn't drop the pair.
    let mut rows = String::from("x,y\n");
    for _ in 0..5 {
        for x in 1..=30 {
            let y = (f64::from(x) / 3.0).exp();
            rows.push_str(&format!("{x},{y:.6}\n"));
        }
    }
    wrk.create_from_string("nonlin.csv", &rows);
    wrk.create_from_string(
        "trivial.schema.json",
        r#"{"$schema":"https://json-schema.org/draft/2020-12/schema","type":"object","properties":{}}"#,
    );

    let out_html = wrk.path("dash.html").to_string_lossy().to_string();
    let mut cmd = wrk.command("viz");
    cmd.args([
        "smart",
        "nonlin.csv",
        "--bivariate",
        "-o",
        &out_html,
        "--dictionary",
    ])
    .arg(wrk.path("trivial.schema.json"));
    wrk.assert_success(&mut cmd);

    let html = wrk.read_to_string("dash.html").unwrap();
    assert!(
        html.contains("Nonlinear: pearson r="),
        "expected the association heatmap's hover to flag the monotonic-but-curved x/y pair as \
         nonlinear; html: {html}"
    );
}

#[test]
fn viz_smart_bivariate_top_relationships_excludes_low_support_pairs() {
    let wrk = Workdir::new("viz_smart_bivariate_top_relationships_excludes_low_support_pairs");
    // g1..g4 are deterministically related (NMI ~ 1.0) and fully populated across all 300 rows
    // (n_pairs = 300, the best-supported pairs in this dataset). sparse_a/sparse_b are empty for
    // every row except the SAME narrow 10-row slice, where they're also deterministically
    // related -- so their NMI is just as high, but their co-occurring row count (n_pairs = 10) is
    // far below BIVARIATE_MIN_SUPPORT_RATIO (10%) of 300. The ranked "Top Relationships" bar
    // should exclude sparse_a x sparse_b even though its NMI rivals the well-supported pairs; the
    // association heatmap (which isn't support-gated) should still chart both columns.
    let mut rows =
        String::from("g1,g2,g3,g4,noise1,noise2,noise3,noise4,noise5,noise6,sparse_a,sparse_b\n");
    let cats = ["A", "B", "C", "D"];
    for i in 0..300 {
        let g1 = cats[i % 4];
        // g2/g4 track g1/g3 on all but a handful of rows: NMI stays ~1.0 with full 300-row
        // support, but the relation is NOT a bijection, so the 1:1 collapse (issue #4221) leaves
        // both columns charted. An exact duplicate is literally the same variable and folds into
        // a single panel, taking with it the well-supported high-NMI pairs these tests rank.
        let g2 = if i % 50 == 0 { cats[(i + 1) % 4] } else { g1 };
        // block-stepped, NOT `(i * 3) % 4`: multiplying a linear sequence by a coprime factor
        // permutes it, so the old g3 was a relabeling of g1 and folded into it. The divisor sits
        // clear of the noise columns' 2..=7 range so it does not collide with one of them either.
        let g3 = cats[(i / 11) % 4];
        let g4 = if i % 50 == 25 { cats[(i + 1) % 4] } else { g3 };
        // genuine filler: a per-column BLOCK size rather than a per-column offset. Offsets of a
        // linear sequence are rotations of one another, so every "noise" column was a relabeling
        // of g1 and all ten collapsed into one panel.
        let noise: Vec<&str> = (0..6).map(|k| cats[(i / (k + 2)) % 4]).collect();
        let (sparse_a, sparse_b) = if i < 10 { ("P", "Q") } else { ("", "") };
        rows.push_str(&format!(
            "{g1},{g2},{g3},{g4},{},{},{},{},{},{},{sparse_a},{sparse_b}\n",
            noise[0], noise[1], noise[2], noise[3], noise[4], noise[5]
        ));
    }
    wrk.create_from_string("sparse_assoc.csv", &rows);
    wrk.create_from_string(
        "trivial.schema.json",
        r#"{"$schema":"https://json-schema.org/draft/2020-12/schema","type":"object","properties":{}}"#,
    );

    let out_html = wrk.path("dash.html").to_string_lossy().to_string();
    let mut cmd = wrk.command("viz");
    cmd.args([
        "smart",
        "sparse_assoc.csv",
        "--bivariate",
        "-o",
        &out_html,
        "--dictionary",
    ])
    .arg(wrk.path("trivial.schema.json"));
    wrk.assert_success(&mut cmd);

    let html = wrk.read_to_string("dash.html").unwrap();
    assert!(
        html.contains(r#""name":"Top Relationships (NMI)""#),
        "expected a ranked top-relationships bar with > 8 chartable columns; html: {html}"
    );
    assert!(
        !html.contains("sparse_a \u{d7} sparse_b") && !html.contains("sparse_b \u{d7} sparse_a"),
        "the sparsely-supported (n_pairs=10 of 300) sparse_a/sparse_b pair should be excluded \
         from the support-gated ranking despite its high NMI; html: {html}"
    );
    assert!(
        html.contains("sparse_a") && html.contains("sparse_b"),
        "the association heatmap itself is not support-gated, so both columns should still appear \
         somewhere in its labels; html: {html}"
    );
}

#[test]
fn viz_smart_derives_skew_hint_without_smarter() {
    // Plain `viz smart` (no moarstats): the skew hint is derived from the BASE stats cache
    // (3 * (mean - median) / stddev — the same formula moarstats uses), so a right-skewed box
    // panel is annotated even though pearson_skewness was never computed.
    let wrk = Workdir::new("viz_smart_derives_skew_hint_without_smarter");
    // continuous right-skewed column: bulk at 1..=40 with a heavy tail of 1000s
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
    cmd.args(["smart", "amounts.csv", "-o", &out_html]);
    wrk.assert_success(&mut cmd);

    let html = wrk.read_to_string("amounts.html").unwrap();
    assert!(
        html.contains("right-skewed"),
        "box panel title should carry the derived skew hint without --smarter; html: {html}"
    );
    assert!(html.contains(r#""type":"box""#));
}

#[test]
fn viz_smart_dominant_category_hint() {
    // when one real category holds >= 90% of all rows, the frequency bar panel's title says so
    let wrk = Workdir::new("viz_smart_dominant_category_hint");
    let mut rows = String::from("status\n");
    for _ in 0..95 {
        rows.push_str("active\n");
    }
    for _ in 0..5 {
        rows.push_str("inactive\n");
    }
    wrk.create_from_string("statuses.csv", &rows);

    let out_html = wrk.path("statuses.html").to_string_lossy().to_string();
    let mut cmd = wrk.command("viz");
    cmd.args(["smart", "statuses.csv", "-o", &out_html]);
    wrk.assert_success(&mut cmd);

    let html = wrk.read_to_string("statuses.html").unwrap();
    assert!(
        html.contains("(dominated by active, 95%)"),
        "dominated freq bar panel should carry the dominance hint; html: {html}"
    );
}

#[test]
fn viz_smart_log_scale_auto_logs_high_range_box() {
    // an all-positive continuous column whose observed max/min ratio clears
    // LOG_SCALE_MIN_RATIO gets a logarithmic value axis under the default --log-scale auto,
    // cued by the "log scale" y-axis title (distinct from the bars' "count (log)").
    let wrk = Workdir::new("viz_smart_log_scale_auto_logs_high_range_box");
    let mut rows = String::from("load\n");
    for i in 1..=300 {
        rows.push_str(&format!("{i}.5\n"));
    }
    wrk.create_from_string("loads.csv", &rows);

    let out_html = wrk.path("loads.html").to_string_lossy().to_string();
    let mut cmd = wrk.command("viz");
    cmd.args(["smart", "loads.csv", "-o", &out_html]);
    wrk.assert_success(&mut cmd);

    let html = wrk.read_to_string("loads.html").unwrap();
    assert!(
        html.contains("log scale"),
        "high-dynamic-range box panel should carry the log value-axis cue; html: {html}"
    );
    assert!(
        html.contains(r#""type":"log""#),
        "box panel's y-axis should be logarithmic; html: {html}"
    );

    // --log-scale off keeps it linear
    let out_lin = wrk.path("loads_linear.html").to_string_lossy().to_string();
    let mut lin = wrk.command("viz");
    lin.args(["smart", "loads.csv", "--log-scale", "off", "-o", &out_lin]);
    wrk.assert_success(&mut lin);
    let linear_html = wrk.read_to_string("loads_linear.html").unwrap();
    assert!(
        !linear_html.contains(r#""type":"log""#),
        "--log-scale off must keep the box value axis linear; html: {linear_html}"
    );
}

#[test]
fn viz_smart_max_charts_keeps_most_interesting_panels() {
    // when the dashboard overflows --max-charts, survivors are chosen by the stats-driven
    // interestingness ranking, not by column position: a varied 12-category bar (later column)
    // outranks a 96%-dominated 2-category bar (earlier column).
    let wrk = Workdir::new("viz_smart_max_charts_keeps_most_interesting_panels");
    let mut rows = String::from("flag,category\n");
    for i in 0..100 {
        let flag = if i < 96 { "y" } else { "n" };
        rows.push_str(&format!("{flag},cat_{:02}\n", i % 12));
    }
    wrk.create_from_string("wide.csv", &rows);

    let out_html = wrk.path("wide.html").to_string_lossy().to_string();
    let mut cmd = wrk.command("viz");
    cmd.args(["smart", "wide.csv", "--max-charts", "1", "-o", &out_html]);
    wrk.assert_success(&mut cmd);

    let html = wrk.read_to_string("wide.html").unwrap();
    assert!(
        html.contains("cat_00"),
        "the varied 12-category panel should survive the overflow ranking; html: {html}"
    );
    assert!(
        !html.contains(r#""name":"flag"#),
        "the dominated 2-category panel (leftmost column) should be the one skipped; html: {html}"
    );
}

#[test]
fn viz_smart_dominance_share_survives_no_nulls() {
    // regression (roborev 3387): the dominance share must come from the full row count, not the
    // rendered bars — with 85% "active" + 15% nulls and --no-nulls suppressing the NULL bar,
    // the surviving bar is 100% of what's drawn but only 85% of the rows: NOT dominant.
    let wrk = Workdir::new("viz_smart_dominance_share_survives_no_nulls");
    let mut rows = String::from("id,status\n");
    for i in 0..100 {
        let status = if i < 85 { "active" } else { "" };
        rows.push_str(&format!("{i},{status}\n"));
    }
    wrk.create_from_string("statuses.csv", &rows);

    let out_html = wrk.path("statuses.html").to_string_lossy().to_string();
    let mut cmd = wrk.command("viz");
    cmd.args(["smart", "statuses.csv", "--no-nulls", "-o", &out_html]);
    wrk.assert_success(&mut cmd);

    let html = wrk.read_to_string("statuses.html").unwrap();
    assert!(
        !html.contains("dominated by"),
        "an 85%-of-rows category must not be reported as dominant just because --no-nulls hid the \
         NULL bar; html: {html}"
    );
}

#[test]
fn viz_smart_no_dominance_hint_on_null_heavy_column() {
    // regression (roborev 3389): with >90% blank rows the stats cache's mode IS the empty
    // bucket (mode_occurrences counts empties), so a share derived from mode stats would label
    // the tallest real category with the null bucket's share. The hint must stay silent: the
    // tallest REAL category here holds only 4% of the rows.
    let wrk = Workdir::new("viz_smart_no_dominance_hint_on_null_heavy_column");
    let mut rows = String::from("id,status\n");
    for i in 0..100 {
        let status = if i < 4 { "active" } else { "" };
        rows.push_str(&format!("{i},{status}\n"));
    }
    wrk.create_from_string("nullheavy.csv", &rows);

    let out_html = wrk.path("nullheavy.html").to_string_lossy().to_string();
    let mut cmd = wrk.command("viz");
    cmd.args(["smart", "nullheavy.csv", "-o", &out_html]);
    wrk.assert_success(&mut cmd);

    let html = wrk.read_to_string("nullheavy.html").unwrap();
    assert!(
        !html.contains("dominated by"),
        "a 4%-of-rows category must not inherit the empty bucket's 96% share; html: {html}"
    );
}

#[test]
fn viz_smart_timeseries_prefers_sorted_date_column() {
    // two undated-concept (no dictionary) date columns tie on timestamp_rank; the sort_order
    // tiebreak must pick the column the file is physically ordered by (`created`, ascending)
    // over the leftmost-but-unsorted `updated` as the trend panel's time axis.
    let wrk = Workdir::new("viz_smart_timeseries_prefers_sorted_date_column");
    let mut rows = String::from("updated,created,value\n");
    for i in 0..100u32 {
        // `created`: strictly ascending ISO timestamps (day 1..5, hour cycling — ISO order ==
        // lexicographic order); `updated`: the same 100 timestamps deterministically shuffled
        // (multiplicative stride mod 100), so stats reads them as UNSORTED
        let stamp = |n: u32| format!("2024-01-{:02}T{:02}:00:00Z", n / 24 + 1, n % 24);
        rows.push_str(&format!(
            "{},{},{}\n",
            stamp((i * 37) % 100),
            stamp(i),
            100.0 + f64::from(i) * 1.37
        ));
    }
    wrk.create_from_string("events.csv", &rows);

    let out_html = wrk.path("events.html").to_string_lossy().to_string();
    let mut cmd = wrk.command("viz");
    cmd.args(["smart", "events.csv", "-o", &out_html]);
    wrk.assert_success(&mut cmd);

    let html = wrk.read_to_string("events.html").unwrap();
    assert!(
        html.contains("over created"),
        "the physically-sorted date column should win the time-series axis; html: {html}"
    );
    assert!(
        !html.contains("over updated"),
        "the unsorted leftmost date column should lose the tiebreak; html: {html}"
    );
}

#[test]
fn viz_smart_dictionary_measure_on_zero_padded_code_becomes_bar() {
    // regression (roborev 3392): stats emits zero_padded_numeric only for String-typed columns
    // (leading zeros force String inference), so the reachable hazard is a DICTIONARY mistag —
    // an LLM calling an ICD-9-style code column a measure. The guardrail must downgrade that
    // verdict to a dimension: without it the column is dropped outright (a String column has no
    // quartiles for the measure path); with it, it charts as a frequency bar.
    let wrk = Workdir::new("viz_smart_dictionary_measure_on_zero_padded_code_becomes_bar");
    let mut rows = String::from("icd9,status\n");
    for i in 0..200 {
        let code_n = i % 40;
        let status = if i % 3 == 0 { "open" } else { "closed" };
        rows.push_str(&format!("0{:02}.{},{status}\n", code_n, code_n % 10));
    }
    wrk.create_from_string("diagnoses.csv", &rows);
    // a dictionary that (wrongly) tags the zero-padded code column as an explicit measure
    wrk.create_from_string(
        "diagnoses.schema.json",
        r#"{
          "$schema": "https://json-schema.org/draft/2020-12/schema",
          "type": "object",
          "properties": {
            "icd9": { "type": "string",
              "x-qsv": { "qsv_type": "String", "role": "measure", "concept": "measure.amount" } },
            "status": { "type": "string",
              "x-qsv": { "qsv_type": "String", "role": "dimension", "concept": "category.status" } }
          }
        }"#,
    );

    let out_html = wrk.path("diagnoses.html").to_string_lossy().to_string();
    let mut cmd = wrk.command("viz");
    cmd.args(["smart", "diagnoses.csv", "-o", &out_html, "--dictionary"])
        .arg(wrk.path("diagnoses.schema.json"));
    wrk.assert_success(&mut cmd);

    let html = wrk.read_to_string("diagnoses.html").unwrap();
    assert!(
        html.contains(r#""name":"icd9"#),
        "the mistagged code column should be rescued as a frequency bar, not dropped; html: {html}"
    );
    assert!(
        !html.contains(r#""type":"box""#) && !html.contains(r#""type":"violin""#),
        "the code column must not chart as a measure; html: {html}"
    );

    // contrast: stats-only routing (no dictionary) skips the same 40-category String column as
    // ID-like noise — proving the bar above comes from the guardrail downgrade, not classify
    let out_plain = wrk.path("plain.html").to_string_lossy().to_string();
    let mut plain = wrk.command("viz");
    plain.args(["smart", "diagnoses.csv", "-o", &out_plain]);
    wrk.assert_success(&mut plain);
    let plain_html = wrk.read_to_string("plain.html").unwrap();
    assert!(
        !plain_html.contains(r#""name":"icd9"#),
        "without a dictionary the high-cardinality code column stays skipped; html: {plain_html}"
    );
}

// ---- --dict-info: embedded Data Dictionary tab + panel info icons ----

/// jsonschema dictionary used by the --dict-info tests: labels, per-column descriptions,
/// a dataset-level description and provenance.
fn dict_info_schema() -> &'static str {
    r#"{
      "$schema": "https://json-schema.org/draft/2020-12/schema",
      "type": "object",
      "description": "Synthetic service requests used to exercise --dict-info.",
      "properties": {
        "census_tract": { "type": ["integer","null"], "title": "Census Tract",
          "description": "The 2020 census tract containing the incident.",
          "x-qsv": { "qsv_type": "Integer", "role": "dimension", "concept": "geo.census_tract",
                     "cardinality": 40, "null_count": 0 } },
        "status": { "type": "string", "title": "Case Status",
          "description": "Lifecycle state of the service request.",
          "enum": ["Open", "Closed", "Pending"],
          "x-qsv": { "qsv_type": "String", "role": "dimension", "concept": "category.status" } }
      },
      "x-qsv": { "grain": "one row = one service request",
        "generated_by": "Generated by qsv describegpt\nModel: test-model-dictinfo" }
    }"#
}

fn dict_info_codes_csv(wrk: &Workdir) {
    let mut rows = String::from("census_tract,status\n");
    for i in 0..200 {
        let tract = i % 40;
        let status = match i % 3 {
            0 => "Open",
            1 => "Closed",
            _ => "Pending",
        };
        rows.push_str(&format!("{tract},{status}\n"));
    }
    wrk.create_from_string("codes.csv", &rows);
}

// <=8 panels -> the typed-grid path: info icons ride on the plotly title annotations
// (hovertext + captureevents + a qsvdict- name for the clickannotation hook), and the page
// embeds the Data Dictionary document + qsvOpenDict chrome.
#[test]
fn viz_smart_dict_info_grid_path() {
    let wrk = Workdir::new("viz_smart_dict_info_grid_path");
    dict_info_codes_csv(&wrk);
    wrk.create_from_string("dict.schema.json", dict_info_schema());

    let mut cmd = wrk.command("viz");
    cmd.args(["smart", "codes.csv", "--dict-info", "--dictionary"])
        .arg(wrk.path("dict.schema.json"));
    let out = wrk.output(&mut cmd);
    assert!(out.status.success());
    let html = String::from_utf8_lossy(&out.stdout);

    // the embedded dictionary document + chrome
    assert!(
        html.contains(r#"id="qsv-dict-src""#),
        "embedded dict template missing"
    );
    assert!(html.contains("qsvOpenDict"), "qsvOpenDict script missing");
    assert!(
        html.contains(r#"Data Dictionary<svg class="qsv-link-icon""#),
        "Data Dictionary link missing"
    );
    // per-column anchors (stable prefix; the trailing hash is an implementation detail)
    assert!(
        html.contains("qsvdict-census_tract-"),
        "census_tract anchor missing"
    );
    assert!(html.contains("qsvdict-status-"), "status anchor missing");
    // dictionary page content: dataset description, per-column description, provenance
    assert!(html.contains("Synthetic service requests used to exercise --dict-info."));
    assert!(html.contains("The 2020 census tract containing the incident."));
    assert!(html.contains("test-model-dictinfo"));
    // typed-grid icons: title annotations carry hovertext + captureevents + qsvdict- name
    assert!(
        html.contains(r#""captureevents":true"#),
        "captureevents missing from plot JSON"
    );
    assert!(
        html.contains(r#""hovertext""#),
        "hovertext missing from plot JSON"
    );
    assert!(
        html.contains(r#""name":"qsvdict-"#),
        "qsvdict- annotation name missing"
    );
    // drawer + tab chrome: the in-page drawer builder, its named-tab escape hatch, and the
    // dictionary page's back link / role-tinted ToC chips / "View chart" reverse links
    assert!(html.contains("qsvDictDrawer"), "drawer builder missing");
    assert!(
        html.contains("qsvOpenDictTab"),
        "named-tab escape hatch missing"
    );
    assert!(html.contains("qsv-dict-back"), "back link missing");
    assert!(html.contains("qsv-dict-chip"), "ToC chips missing");
    assert!(
        html.contains("qsv-dict-role-dimension"),
        "role-tinted chip missing"
    );
    // the typed grid is ONE plot with no per-panel `data-qsv-dict` cells, so its dictionary
    // renders no "View chart" links (they'd silently do nothing in the standalone tab)
    assert!(
        !html.contains("qsv-dict-viewchart\" data-anchor"),
        "grid-path dictionary must not render View chart links"
    );
}

// The Examples row annotates each example with its occurrence count from
// `x-qsv.example_counts`, comma-grouped for readability. Driven by the `examples` array, so
// describegpt's "Other…" aggregation-bucket sentinel (kept in example_counts but filtered out
// of a numeric column's examples) must NOT leak in; a column without example_counts falls back
// to bare values.
#[test]
fn viz_smart_dict_info_example_counts() {
    let wrk = Workdir::new("viz_smart_dict_info_example_counts");
    dict_info_codes_csv(&wrk);
    wrk.create_from_string(
        "dict.schema.json",
        r#"{
      "$schema": "https://json-schema.org/draft/2020-12/schema",
      "type": "object",
      "description": "Example-count rendering fixture.",
      "properties": {
        "census_tract": { "type": ["integer","null"], "title": "Census Tract",
          "description": "The 2020 census tract containing the incident.",
          "examples": [12, 7],
          "x-qsv": { "qsv_type": "Integer", "role": "dimension", "concept": "geo.census_tract",
                     "cardinality": 40, "null_count": 0,
                     "example_counts": "Other… [1500]\n12 [1234]\n7 [56]" } },
        "status": { "type": "string", "title": "Case Status",
          "description": "Lifecycle state of the service request.",
          "examples": ["Open", "Closed", "Pending"],
          "x-qsv": { "qsv_type": "String", "role": "dimension", "concept": "category.status" } }
      }
    }"#,
    );

    let mut cmd = wrk.command("viz");
    cmd.args(["smart", "codes.csv", "--dict-info", "--dictionary"])
        .arg(wrk.path("dict.schema.json"));
    let out = wrk.output(&mut cmd);
    assert!(out.status.success());
    let html = String::from_utf8_lossy(&out.stdout);

    // counts appended and comma-grouped, in the examples array's order
    assert!(
        html.contains("<dt>Examples</dt><dd>12 (1,234), 7 (56)</dd>"),
        "annotated Examples row missing"
    );
    // the "Other…" bucket rides only in example_counts - it must not become an example
    assert!(
        !html.contains("Other…"),
        "example_counts aggregation bucket leaked into the Examples row"
    );
    // no example_counts for this column -> bare values, exactly as before
    assert!(
        html.contains("<dt>Examples</dt><dd>Open, Closed, Pending</dd>"),
        "unannotated fallback Examples row missing"
    );
}

// --dict-info bundles the sidecars the dashboard was built from as base64 `data:` downloads,
// so a recipient of the HTML gets the files without access to the author's machine — and,
// precisely because the dashboard is made to be SHARED, no embedded byte may carry an absolute
// local path. The stats metadata sidecars record `canonical_input_path`/`canonical_stats_path`
// verbatim, so this asserts the end-to-end guarantee (the unit tests cover the redactor itself,
// but only this catches a new sidecar wired up without redaction).
#[test]
fn viz_smart_dict_info_bundles_sidecars_without_leaking_local_paths() {
    let wrk = Workdir::new("viz_smart_dict_info_bundles_sidecars_without_leaking_local_paths");
    dict_info_codes_csv(&wrk);
    wrk.create_from_string("dict.schema.json", dict_info_schema());

    // populate the stats cache (and its `.stats.csv.json` metadata) for codes.csv
    let mut stats_cmd = wrk.command("stats");
    stats_cmd.args(["--stats-jsonl", "--cardinality", "codes.csv"]);
    wrk.output(&mut stats_cmd);

    let mut cmd = wrk.command("viz");
    cmd.args(["smart", "codes.csv", "--dict-info", "--dictionary"])
        .arg(wrk.path("dict.schema.json"));
    let out = wrk.output(&mut cmd);
    assert!(out.status.success());
    let html = String::from_utf8_lossy(&out.stdout);

    // the download row exists, with the stats cache and the always-available charted-frequency
    // CSV (generated in memory — it must NOT be written to disk)
    assert!(
        html.contains(r#"<div class="qsv-dict-downloads">"#),
        "download row missing"
    );
    assert!(
        html.contains(r#"download="codes.stats.csv.data.jsonl""#),
        "stats cache download missing"
    );
    assert!(
        html.contains(r#"download="codes.viz-frequency.csv""#),
        "charted-frequency download missing"
    );
    assert!(
        !wrk.path("codes.viz-frequency.csv").exists(),
        "the charted-frequency CSV is a bundled download, not a file written to disk"
    );
    // the stats run above also wrote a fresh `codes.stats.csv`, but viz never reads it, so it
    // cannot be shown to be the stats behind this dashboard and must not be offered
    assert!(
        wrk.path("codes.stats.csv").exists(),
        "precondition: the stats run wrote a human-readable stats CSV"
    );
    assert!(
        !html.contains(r#"download="codes.stats.csv""#),
        "the unverifiable .stats.csv is never offered"
    );

    // THE guarantee: the work directory's absolute path appears nowhere in the page — not in the
    // markup, and not inside any base64 payload.
    let workdir = wrk.path("").to_string_lossy().into_owned();
    let workdir = workdir.trim_end_matches(std::path::MAIN_SEPARATOR);
    assert!(
        !html.contains(workdir),
        "the dashboard markup leaks the local path {workdir}"
    );
    let re = regex::Regex::new(r#"href="data:[^;]+;base64,([A-Za-z0-9+/=]+)""#).unwrap();
    let mut checked = 0;
    for cap in re.captures_iter(&html) {
        let decoded = base64_simd::STANDARD.decode_to_vec(&cap[1]).unwrap();
        let text = String::from_utf8_lossy(&decoded);
        assert!(
            !text.contains(workdir),
            "an embedded sidecar leaks the local path {workdir}"
        );
        checked += 1;
    }
    assert!(
        checked >= 3,
        "expected the schema + stats + charted-frequency downloads, found {checked}"
    );
}

// --dict-info descriptions render Markdown (bold/bullets/links) as HTML in the embedded
// Data Dictionary page, while raw HTML and unsafe URL schemes in the untrusted LLM text
// stay escaped/neutralized so they can't break out of the `<script type="text/html">`
// embedding template or inject script.
#[test]
fn viz_smart_dict_info_markdown_rendering() {
    let wrk = Workdir::new("viz_smart_dict_info_markdown_rendering");
    dict_info_codes_csv(&wrk);
    wrk.create_from_string(
        "dict.schema.json",
        r#"{
          "$schema": "https://json-schema.org/draft/2020-12/schema",
          "type": "object",
          "description": "A **bold** dataset. See [home](https://example.com).",
          "properties": {
            "census_tract": { "type": ["integer","null"], "title": "Census Tract",
              "description": "The **census** tract.\n\n- point one\n- point two" },
            "status": { "type": "string", "title": "Case Status",
              "description": "Danger: <img src=x onerror=alert(1)> </script> and [x](javascript:alert(1))." }
          }
        }"#,
    );

    let mut cmd = wrk.command("viz");
    cmd.args(["smart", "codes.csv", "--dict-info", "--dictionary"])
        .arg(wrk.path("dict.schema.json"));
    let out = wrk.output(&mut cmd);
    assert!(out.status.success());
    let html = String::from_utf8_lossy(&out.stdout);

    // Markdown rendered to HTML: bold, bullet list, safe link.
    assert!(
        html.contains("<strong>bold</strong>"),
        "dataset bold not rendered"
    );
    assert!(
        html.contains("<strong>census</strong>"),
        "column bold not rendered"
    );
    assert!(
        html.contains("<li>point one</li>"),
        "bullet list not rendered"
    );
    assert!(
        html.contains(
            r#"<a href="https://example.com" target="_blank" rel="noopener noreferrer">home</a>"#
        ),
        "safe link not rendered"
    );
    // Untrusted content stays safe: raw HTML escaped, no </script> breakout, js: link dropped.
    assert!(!html.contains("<img src=x"), "raw <img> leaked");
    assert!(html.contains("&lt;img src=x"), "raw <img> not escaped");
    assert!(
        !html.contains("</script> and"),
        "literal </script> from description leaked into embedded page"
    );
    // The js: link in the description must never become an anchor href (it stays inert
    // plain text in the out-of-scope info-icon tooltip, which is not an XSS vector).
    assert!(
        !html.contains(r#"href="javascript:"#),
        "javascript: URL rendered as an anchor href"
    );
}

// >8 chartable columns -> the inline-div path: icons ride on the panel-title annotations
// (same mechanism as the typed grid), the cells carry `data-qsv-dict` anchors for the
// dictionary page's "View chart" reverse links, and the same dict template + chrome is
// embedded.
#[test]
fn viz_smart_dict_info_inline_path() {
    let wrk = Workdir::new("viz_smart_dict_info_inline_path");
    let mut rows = String::from("c0,c1,c2,c3,c4,c5,c6,c7,c8,c9\n");
    for i in 0..60 {
        // per-column modulus => 10 distinct cardinalities. A shared modulus made every pair of
        // columns a bijection, which the 1:1 collapse (issue #4221) folds into one panel — and
        // this test needs >8 chartable columns to reach the inline-div path.
        let vals: Vec<String> = (0..10).map(|c| format!("v{}", i % (c + 2))).collect();
        rows.push_str(&format!("{}\n", vals.join(",")));
    }
    wrk.create_from_string("wide.csv", &rows);
    // dictionary with descriptions for two of the columns
    wrk.create_from_string(
        "wide.dict.json",
        r#"{
          "$schema": "https://json-schema.org/draft/2020-12/schema",
          "type": "object",
          "description": "Wide synthetic categories.",
          "properties": {
            "c0": { "type": "string", "title": "Category Zero",
              "description": "First synthetic category column." },
            "c1": { "type": "string", "title": "Category One",
              "description": "Second synthetic category column." }
          }
        }"#,
    );

    let mut cmd = wrk.command("viz");
    cmd.args(["smart", "wide.csv", "--dict-info", "--dictionary"])
        .arg(wrk.path("wide.dict.json"));
    let out = wrk.output(&mut cmd);
    assert!(out.status.success());
    let html = String::from_utf8_lossy(&out.stdout);

    // inline page (page <h1> proves the inline path)
    assert!(
        html.contains(r#"<h1 class="qsv-viz-title">"#),
        "expected the inline-div path"
    );
    // info icons ride on the panel-title annotations: hovertext carries the description,
    // captureevents + the qsvdict- name route the click to the drawer
    assert!(
        html.contains(r#""captureevents":true"#),
        "captureevents missing from inline panel JSON"
    );
    assert!(html.contains("First synthetic category column."));
    assert!(
        html.contains(r#""name":"qsvdict-c0-"#),
        "c0 title annotation anchor missing"
    );
    // embedded dict template + chrome
    assert!(html.contains(r#"id="qsv-dict-src""#));
    assert!(html.contains("qsvOpenDict"));
    assert!(html.contains("Wide synthetic categories."));
    // the panel cells carry their dictionary anchor for "View chart" reverse navigation;
    // columns without a description get none: exactly 2 cells for c0/c1
    assert_eq!(html.matches(r#" data-qsv-dict="qsvdict-"#).count(), 2);
    // and the dictionary page renders View chart links for exactly those panels
    assert_eq!(
        html.matches("qsv-dict-viewchart\" data-anchor").count(),
        2,
        "inline-path dictionary should render View chart links for the paneled columns only"
    );
}

// Overview panels get info icons too: the time-series trend panel anchors on the driving
// date column's dictionary entry.
#[test]
fn viz_smart_dict_info_overview_timeseries_anchor() {
    let wrk = Workdir::new("viz_smart_dict_info_overview_timeseries_anchor");
    let mut rows = String::from("created_date,status\n");
    for i in 0..120 {
        let status = match i % 3 {
            0 => "Open",
            1 => "Closed",
            _ => "Pending",
        };
        rows.push_str(&format!(
            "2024-{:02}-{:02},{status}\n",
            (i % 12) + 1,
            (i % 28) + 1
        ));
    }
    wrk.create_from_string("events.csv", &rows);
    wrk.create_from_string(
        "dict.schema.json",
        r#"{
          "$schema": "https://json-schema.org/draft/2020-12/schema",
          "type": "object",
          "description": "Synthetic events.",
          "properties": {
            "created_date": { "type": "string", "title": "Created",
              "description": "When the event was created.",
              "x-qsv": { "role": "timestamp", "concept": "time.event_timestamp" } },
            "status": { "type": "string", "title": "Status",
              "description": "Lifecycle state of the event." }
          }
        }"#,
    );

    let mut cmd = wrk.command("viz");
    cmd.args(["smart", "events.csv", "--dict-info", "--dictionary"])
        .arg(wrk.path("dict.schema.json"));
    let out = wrk.output(&mut cmd);
    assert!(out.status.success());
    let html = String::from_utf8_lossy(&out.stdout);
    // the trend panel's clickable title annotation carries the date column's anchor
    assert!(
        html.contains(r#""name":"qsvdict-created_date-"#),
        "time-series title annotation should anchor on the date column's dictionary entry"
    );
    // and the timestamp role tints its ToC chip
    assert!(
        html.contains("qsv-dict-role-timestamp"),
        "timestamp role chip missing"
    );
}

// --dict-info without a usable dictionary: soft no-op with a note, no dict chrome.
#[test]
fn viz_smart_dict_info_without_dictionary_noop() {
    let wrk = Workdir::new("viz_smart_dict_info_without_dictionary_noop");
    dict_info_codes_csv(&wrk);

    let mut cmd = wrk.command("viz");
    cmd.args(["smart", "codes.csv", "--dict-info"]);
    let out = wrk.output(&mut cmd);
    assert!(out.status.success());
    let html = String::from_utf8_lossy(&out.stdout);
    assert!(
        !html.contains("qsv-dict-src"),
        "no dict chrome expected without a dictionary"
    );
    assert!(!html.contains("qsvOpenDict"));
    let stderr = String::from_utf8_lossy(&out.stderr);
    assert!(
        stderr.contains("no usable data dictionary"),
        "soft note expected on stderr; got: {stderr}"
    );
}

// LLM text is hostile text: descriptions must be HTML-escaped everywhere they are injected
// (the embedded dict document, icon tooltips, annotation hovertext).
#[test]
fn viz_smart_dict_info_escapes_llm_text() {
    let wrk = Workdir::new("viz_smart_dict_info_escapes_llm_text");
    dict_info_codes_csv(&wrk);
    wrk.create_from_string(
        "evil.schema.json",
        r#"{
          "$schema": "https://json-schema.org/draft/2020-12/schema",
          "type": "object",
          "description": "Dataset </script><b>&\"bold claim.",
          "properties": {
            "census_tract": { "type": ["integer","null"], "title": "Census Tract",
              "description": "Tract </script><b>&\"desc." },
            "status": { "type": "string", "title": "Case Status" }
          }
        }"#,
    );

    let mut cmd = wrk.command("viz");
    cmd.args(["smart", "codes.csv", "--dict-info", "--dictionary"])
        .arg(wrk.path("evil.schema.json"));
    let out = wrk.output(&mut cmd);
    assert!(out.status.success());
    let html = String::from_utf8_lossy(&out.stdout);
    // the raw sequences must never survive (they would terminate the template / inject markup)
    assert!(
        !html.contains("</script><b>"),
        "unescaped LLM text leaked into the page"
    );
    // and the escaped forms are what got rendered
    assert!(html.contains("&lt;/script&gt;&lt;b&gt;&amp;&quot;desc."));
    assert!(html.contains("&lt;/script&gt;&lt;b&gt;&amp;&quot;bold claim."));
}

// The dictionary tab's named window is keyed on title AND dictionary content (roborev 3416):
// two dashboards sharing a title but carrying different dictionaries must get distinct
// `qsv_dict_<hash>` window names (else the second shows the first's dictionary, since
// qsvOpenDict skips rewriting an existing tab), while an identical title + dictionary must
// keep a stable name so a reload reuses its tab.
#[test]
fn viz_smart_dict_info_window_key_is_per_dictionary() {
    fn window_key(html: &str) -> String {
        let start = html
            .find("qsv_dict_")
            .expect("qsv_dict_ window name missing");
        html[start..start + "qsv_dict_".len() + 8].to_string()
    }
    fn render(wrk: &Workdir, dict: &str) -> String {
        let mut cmd = wrk.command("viz");
        cmd.args([
            "smart",
            "codes.csv",
            "--title",
            "Same Title",
            "--dict-info",
            "--dictionary",
        ])
        .arg(wrk.path(dict));
        let out = wrk.output(&mut cmd);
        assert!(out.status.success());
        String::from_utf8_lossy(&out.stdout).into_owned()
    }

    let wrk = Workdir::new("viz_smart_dict_info_window_key_is_per_dictionary");
    dict_info_codes_csv(&wrk);
    wrk.create_from_string("dict_a.schema.json", dict_info_schema());
    // same shape, different description -> different embedded dictionary page
    wrk.create_from_string(
        "dict_b.schema.json",
        &dict_info_schema().replace(
            "The 2020 census tract containing the incident.",
            "A completely different tract description.",
        ),
    );

    let key_a = window_key(&render(&wrk, "dict_a.schema.json"));
    let key_b = window_key(&render(&wrk, "dict_b.schema.json"));
    let key_a_again = window_key(&render(&wrk, "dict_a.schema.json"));

    assert_ne!(
        key_a, key_b,
        "same title + different dictionaries must open distinct dictionary tabs"
    );
    assert_eq!(
        key_a, key_a_again,
        "identical title + dictionary must keep a stable window name (tab reuse)"
    );
}

#[test]
fn viz_smart_hints_at_denull_even_when_every_column_is_skipped() {
    // The diagnostics used to run AFTER the empty-dashboard early return, so a file whose
    // every column is a sentinel-suspect got a bare "No chartable columns" and no hint.
    let wrk = Workdir::new("viz_smart_hints_at_denull_even_when_every_column_is_skipped");
    let mut rows = String::from("depth\n");
    for i in 0..60 {
        rows.push_str(
            if i % 2 == 0 {
                "NULL\n".to_string()
            } else {
                format!("{}\n", i * 7)
            }
            .as_str(),
        );
    }
    wrk.create_from_string("d.csv", &rows);

    let mut cmd = wrk.command("viz");
    cmd.arg("smart").arg("d.csv").args(["-o", "out.html"]);
    let stderr = wrk.output_stderr(&mut cmd);
    assert!(
        stderr.contains("qsv denull"),
        "an all-skipped dashboard must still point at denull, got: {stderr}"
    );
}

// a highly unequal additive measure: 250 tiny holders (value 1) plus 50 large holders
// (1000..1049) -> the top ~17% of records hold ~99% of the total, so the Gini is very high. A
// second, near-uniform additive column (widgets, 100..159) has a low Gini. `id` is a near-unique
// integer, so it is skipped.
fn unequal_income_csv() -> String {
    let mut rows = String::from("id,income,widgets\n");
    let mut id = 1;
    for _ in 0..250 {
        rows.push_str(&format!("{id},1,{}\n", 100 + id % 60));
        id += 1;
    }
    for v in 0..50 {
        rows.push_str(&format!("{id},{},{}\n", 1000 + v, 100 + id % 60));
        id += 1;
    }
    rows
}

#[test]
fn viz_smart_smarter_adds_lorenz_for_unequal_measure() {
    // `viz smart --smarter` runs `qsv moarstats --advanced`, populating gini_coefficient. The
    // highly unequal additive `income` column earns a dedicated Lorenz-curve panel (whose geometry
    // IS the Gini), ADDED ALONGSIDE its distribution panel. The near-uniform `widgets` column has
    // a low Gini and gets NO Lorenz, so exactly one Lorenz panel appears.
    let wrk = Workdir::new("viz_smart_smarter_adds_lorenz_for_unequal_measure");
    wrk.create_from_string("inc.csv", &unequal_income_csv());

    let out_html = wrk.path("inc.html").to_string_lossy().to_string();
    let mut cmd = wrk.command("viz");
    cmd.args(["smart", "inc.csv", "--smarter", "-o", &out_html]);
    wrk.assert_success(&mut cmd);

    let html = wrk.read_to_string("inc.html").unwrap();
    // exactly one Lorenz panel: its equality-diagonal trace is drawn once per panel (the title
    // string itself appears twice — as the curve's trace name AND the cell title annotation — so
    // the diagonal is the reliable panel counter).
    assert_eq!(
        html.matches(r#""name":"equality""#).count(),
        1,
        "exactly one Lorenz panel (the unequal income column) expected; html: {html}"
    );
    // and it is the income column, not the near-uniform (low-Gini) widgets column
    assert!(
        html.contains("income \u{2014} Lorenz curve (Gini")
            || html.contains(r"income — Lorenz curve (Gini"),
        "the Lorenz panel should be the income column; html: {html}"
    );
    assert!(
        !html.contains("widgets \u{2014} Lorenz curve")
            && !html.contains(r"widgets — Lorenz curve"),
        "the low-Gini widgets column must NOT get a Lorenz panel; html: {html}"
    );
    // ADDED ALONGSIDE: income still has a distribution (box/violin) panel too
    assert!(
        html.contains(r#""type":"box""#) || html.contains(r#""type":"violin""#),
        "the distribution panel should remain alongside the Lorenz curve; html: {html}"
    );
}

#[test]
fn viz_smart_plain_adds_no_lorenz_without_smarter() {
    // Plain `viz smart` (NO --smarter) never populates gini_coefficient, so no Lorenz panel is
    // built even for the same highly unequal column — the feature is gated on --smarter.
    let wrk = Workdir::new("viz_smart_plain_adds_no_lorenz_without_smarter");
    wrk.create_from_string("inc.csv", &unequal_income_csv());

    let out_html = wrk.path("inc.html").to_string_lossy().to_string();
    let mut cmd = wrk.command("viz");
    cmd.args(["smart", "inc.csv", "-o", &out_html]);
    wrk.assert_success(&mut cmd);

    let html = wrk.read_to_string("inc.html").unwrap();
    assert!(
        !html.contains("Lorenz curve"),
        "plain viz smart (no --smarter) must not add a Lorenz panel; html: {html}"
    );
}

// A zero-inflated unequal additive measure: 300 rows of exactly 0 (60% of the column), 150 small
// holders (value 1) and 50 large holders (1000..1049). The Gini clears the Lorenz gate, and the
// curve's long flat opening run is ENTIRELY the zeros -- the case issue #4222 is about.
fn zero_inflated_spend_csv() -> String {
    let mut rows = String::from("id,spend\n");
    let mut id = 1;
    for _ in 0..300 {
        rows.push_str(&format!("{id},0\n"));
        id += 1;
    }
    for _ in 0..150 {
        rows.push_str(&format!("{id},1\n"));
        id += 1;
    }
    for v in 0..50 {
        rows.push_str(&format!("{id},{}\n", 1000 + v));
        id += 1;
    }
    rows
}

#[test]
fn viz_smart_lorenz_labels_zero_run_and_caveats_unit_heterogeneity() {
    // issue #4222: a Lorenz panel over a zero-inflated column must say that its flat run IS the
    // zeros (a pipeline stage / nothing-recorded-yet population), not a mass of small-but-nonzero
    // records -- and must always carry the unit-heterogeneity caveat, since a high Gini over
    // non-comparable rows is expected rather than an equity finding.
    let wrk = Workdir::new("viz_smart_lorenz_labels_zero_run_and_caveats_unit_heterogeneity");
    wrk.create_from_string("spend.csv", &zero_inflated_spend_csv());

    let out_html = wrk.path("spend.html").to_string_lossy().to_string();
    let mut cmd = wrk.command("viz");
    cmd.args(["smart", "spend.csv", "--smarter", "-o", &out_html])
        .env("QSV_VIZ_NO_COMPRESS", "1");
    wrk.assert_success(&mut cmd);

    let html = wrk.read_to_string("spend.html").unwrap();
    // the panel exists at all (its equality diagonal is the reliable panel marker)
    assert_eq!(
        html.matches(r#""name":"equality""#).count(),
        1,
        "the zero-inflated spend column should earn exactly one Lorenz panel; html: {html}"
    );
    // 300 zeros / 500 numeric values = 60%, reported over the SAME non-null denominator the
    // "% zeros" box hint uses, so the two annotations agree on one dashboard.
    assert!(
        html.contains("flat run = 60% zeros, not small values"),
        "the flat run must be labeled as the zero stage; html: {html}"
    );
    assert!(
        html.contains("concentration is expected unless rows are comparable units"),
        "the Lorenz panel must carry the unit-heterogeneity caveat; html: {html}"
    );
}

#[test]
fn viz_smart_lorenz_caveats_unit_heterogeneity_without_zero_run() {
    // The unit caveat is UNCONDITIONAL (there is no row-unit signal in the stats cache, so gating
    // it would risk silently dropping it where it is most needed), but the zero-run label is not:
    // `unequal_income_csv` holds no zeros at all, so only the caveat appears.
    let wrk = Workdir::new("viz_smart_lorenz_caveats_unit_heterogeneity_without_zero_run");
    wrk.create_from_string("inc.csv", &unequal_income_csv());

    let out_html = wrk.path("inc.html").to_string_lossy().to_string();
    let mut cmd = wrk.command("viz");
    cmd.args(["smart", "inc.csv", "--smarter", "-o", &out_html])
        .env("QSV_VIZ_NO_COMPRESS", "1");
    wrk.assert_success(&mut cmd);

    let html = wrk.read_to_string("inc.html").unwrap();
    assert!(
        html.contains("concentration is expected unless rows are comparable units"),
        "every Lorenz panel carries the unit caveat; html: {html}"
    );
    assert!(
        !html.contains("flat run ="),
        "a column with no zeros must not claim a zero flat run; html: {html}"
    );
}

#[test]
fn viz_smart_density_panel_hover_names_both_measures_and_the_row_count() {
    // `viz smart` swaps the scatter drill-down for a density contour past
    // SMART_CONTOUR_MIN_POINTS (5,000 rows). That contour used to fall back to plotly's default
    // hover -- a bare x/y/z triple over an auto-generated "trace N" -- so the reader could not
    // tell which measure was which, nor that z counts rows. It shares one hover template with the
    // standalone `viz contour` command.
    let wrk = Workdir::new("viz_smart_density_panel_hover_names_both_measures_and_the_row_count");
    let mut rows = String::from("widgetcount,zonescore\n");
    for i in 0..6000 {
        // both axes spread near-uniformly so the linear grid stays legible (a collapsed grid is
        // dropped outright), and strongly correlated so this is the pair drill-down
        let x = i % 100;
        let y = x * 2 + i % 37;
        rows.push_str(&format!("{x},{y}\n"));
    }
    wrk.create_from_string("d.csv", &rows);

    let out_html = wrk.path("d.html").to_string_lossy().to_string();
    let mut cmd = wrk.command("viz");
    cmd.args(["smart", "d.csv", "-o", &out_html])
        .env("QSV_VIZ_NO_COMPRESS", "1");
    wrk.assert_success(&mut cmd);

    let html = wrk.read_to_string("d.html").unwrap();
    assert!(
        html.contains(r#""type":"contour""#),
        "6,000 rows should produce a density contour, not a scatter; html: {html}"
    );
    assert!(
        html.contains(
            r"widgetcount: %{x:,.3~f}\u003cbr\u003ezonescore: %{y:,.3~f}\u003cbr\u003e%{z:,} rows\u003cextra\u003e\u003c/extra\u003e"
        ),
        "the density cell hover must name both measures and the row count; html: {html}"
    );
}

// A nested budget pipeline. Values repeat (a 40-value pool) so the columns stay below the
// near-unique threshold.
fn pipeline_csv() -> String {
    let mut rows = String::from("totalplannedcommit,commitamt,spentamt\n");
    for i in 0..300 {
        let planned = ((i % 40) + 1) * 1000;
        let commit = planned * (i % 3) / 4;
        let spent = commit * (i % 2) / 3;
        rows.push_str(&format!("{planned},{commit},{spent}\n"));
    }
    rows
}

// A dictionary declaring the budget pipeline. Since #4222's rework this is the ONLY way a funnel
// reaches a smart dashboard -- there is no name vocabulary left to infer one from.
fn pipeline_dict(members: &str) -> String {
    format!(
        r#"{{
          "properties": {{
            "totalplannedcommit": {{"type":"integer","x-qsv":{{"qsv_type":"Integer"}}}},
            "commitamt": {{"type":"integer","x-qsv":{{"qsv_type":"Integer"}}}},
            "spentamt": {{"type":"integer","x-qsv":{{"qsv_type":"Integer"}}}}
          }},
          "x-qsv": {{ "relationships": [{{"kind":"pipeline","members":{members}}}] }}
        }}"#
    )
}

// Render `csv` with `dict` and return the HTML.
fn smart_with_dict(wrk: &Workdir, csv: &str, dict: &str) -> String {
    wrk.create_from_string("p.csv", csv);
    wrk.create_from_string("d.schema.json", dict);
    let out_html = wrk.path("p.html").to_string_lossy().to_string();
    let mut cmd = wrk.command("viz");
    cmd.args(["smart", "p.csv", "-o", &out_html, "--dictionary"])
        .arg(wrk.path("d.schema.json"))
        .env("QSV_VIZ_NO_COMPRESS", "1");
    wrk.assert_success(&mut cmd);
    wrk.read_to_string("p.html").unwrap()
}

#[test]
fn viz_smart_no_funnel_without_a_dictionary() {
    // THE headline behavior of the #4222 rework, and the honest replacement for the four
    // `!contains("Pipeline funnel:")` negatives that used to sit below. Those were written when a
    // name vocabulary could produce a funnel on its own, so they proved a guard was working. With
    // detection now dictionary-only they would all pass for a different reason -- no dictionary,
    // no funnel -- and prove nothing. This test asserts that baseline deliberately, on the very
    // table that DOES draw once a dictionary declares it (see the test directly below).
    let wrk = Workdir::new("viz_smart_no_funnel_without_a_dictionary");
    wrk.create_from_string("p.csv", &pipeline_csv());

    let out_html = wrk.path("p.html").to_string_lossy().to_string();
    let mut cmd = wrk.command("viz");
    cmd.args(["smart", "p.csv", "-o", &out_html])
        .env("QSV_VIZ_NO_COMPRESS", "1");
    wrk.assert_success(&mut cmd);

    let html = wrk.read_to_string("p.html").unwrap();
    assert!(
        !html.contains("Pipeline funnel:") && !html.contains(r#""type":"funnel""#),
        "a perfectly nested planned/committed/spent table must NOT produce a funnel without a \
         dictionary -- stage identity is semantics, not a statistic; html: {html}"
    );
}

#[test]
fn viz_smart_builds_pipeline_funnel_from_a_dictionary() {
    // issue #4222 ask 3: the declared member ORDER is the pipeline order, upstream-first.
    let wrk = Workdir::new("viz_smart_builds_pipeline_funnel_from_a_dictionary");
    let html = smart_with_dict(
        &wrk,
        &pipeline_csv(),
        &pipeline_dict(r#"["totalplannedcommit","commitamt","spentamt"]"#),
    );
    assert!(
        html.contains(r#""type":"funnel""#),
        "a declared pipeline should earn a funnel panel; html: {html}"
    );
    assert!(
        html.contains("Pipeline funnel: totalplannedcommit"),
        "the funnel title should lead with the upstream column; html: {html}"
    );
    // a funnel trace draws index 0 at the TOP, so stages are fed upstream-first
    assert!(
        html.contains(r#""y":["totalplannedcommit","commitamt","spentamt"]"#),
        "stages must be fed in declared order, or the funnel renders upside down; html: {html}"
    );
}

#[test]
fn viz_smart_funnel_honors_declared_order_over_magnitude() {
    // The declaration is authoritative: stages are NOT re-sorted by size. Reversing the members
    // must reverse the panel's order -- if the tool silently sorted, a mis-declared pipeline
    // would look plausible and never be noticed. Reversing necessarily makes the totals GROW, so
    // the form becomes a bridge; the order guarantee is what is under test here, not the form.
    let wrk = Workdir::new("viz_smart_funnel_honors_declared_order_over_magnitude");
    let html = smart_with_dict(
        &wrk,
        &pipeline_csv(),
        &pipeline_dict(r#"["spentamt","commitamt","totalplannedcommit"]"#),
    );
    assert!(
        html.contains(
            "\"x\":[\"spentamt\",\"commitamt \u{2212} \
             spentamt\",\"commitamt\",\"totalplannedcommit \u{2212} \
             commitamt\",\"totalplannedcommit\"]"
        ),
        "declared order must survive verbatim, bridged step-by-step; html: {html}"
    );
}

#[test]
fn viz_smart_funnel_hover_reports_both_dollar_and_row_conversion() {
    // At high concentration "42% of dollars committed" and "N of M projects committed anything"
    // are both true and read as a contradiction, so neither may appear alone.
    let wrk = Workdir::new("viz_smart_funnel_hover_reports_both_dollar_and_row_conversion");
    let html = smart_with_dict(
        &wrk,
        &pipeline_csv(),
        &pipeline_dict(r#"["totalplannedcommit","commitamt","spentamt"]"#),
    );
    // the dollar side is plotly's own textinfo, computed from the bar values so it cannot drift
    assert!(
        html.contains(r#""textinfo":"percent previous""#),
        "stage-to-stage conversion must be labeled on the bands; html: {html}"
    );
    // the row side rides in the hover, against a denominator named in the same breath
    assert!(
        html.contains("Rows reached:") && html.contains("complete cases)"),
        "the hover must carry rows-reached and its denominator; html: {html}"
    );
}

#[test]
fn viz_smart_funnel_denominator_covers_only_the_declared_stages() {
    // The funnel now takes its OWN data pass over exactly the declared stages, so an unrelated
    // sparse column no longer dilutes the disclosed denominator. Under the old design this read
    // 90%, because the listwise join spanned every numeric column in the table.
    let wrk = Workdir::new("viz_smart_funnel_denominator_covers_only_the_declared_stages");
    let mut rows = String::from("totalplannedcommit,commitamt,spentamt,othermeasure\n");
    for i in 0..300 {
        let planned = ((i % 40) + 1) * 1000;
        let commit = planned * (i % 3) / 4;
        let spent = commit * (i % 2) / 3;
        // every 10th row leaves an unrelated measure blank
        let other = if i % 10 == 0 {
            String::new()
        } else {
            format!("{}", i % 17)
        };
        rows.push_str(&format!("{planned},{commit},{spent},{other}\n"));
    }
    let dict = format!(
        r#"{{
          "properties": {{
            "totalplannedcommit": {{"type":"integer","x-qsv":{{"qsv_type":"Integer"}}}},
            "commitamt": {{"type":"integer","x-qsv":{{"qsv_type":"Integer"}}}},
            "spentamt": {{"type":"integer","x-qsv":{{"qsv_type":"Integer"}}}},
            "othermeasure": {{"type":"integer","x-qsv":{{"qsv_type":"Integer"}}}}
          }},
          "x-qsv": {{ "relationships": [{{"kind":"pipeline",
             "members":["totalplannedcommit","commitamt","spentamt"]}}] }}
        }}"#
    );
    let html = smart_with_dict(&wrk, &rows, &dict);
    assert!(
        html.contains("complete cases (100% of rows)"),
        "the denominator must cover the declared stages only, not unrelated blanks; html: {html}"
    );
}

#[test]
fn viz_smart_funnel_discloses_containment_violations_instead_of_refusing() {
    // INVERTED by the #4222 rework, then refined again. This table's `spentamt` overruns
    // `commitamt` on ~30% of rows and in TOTAL, so the panel still draws -- containment is a
    // measurement, not a gate -- but it draws as a BRIDGE, because a funnel's band widths are a
    // containment claim these numbers contradict. The subtitle still names the violation share.
    // This is the motivating NYC CPDB shape: three aggregates on different accounting bases.
    let wrk = Workdir::new("viz_smart_funnel_discloses_containment_violations_instead_of_refusing");
    let mut rows = String::from("totalplannedcommit,commitamt,spentamt\n");
    for i in 0..300 {
        let planned = ((i % 40) + 1) * 1000;
        let commit = planned / 2;
        // ~30% of rows spend more than was ever committed
        let spent = if i % 10 < 3 { commit * 3 } else { commit / 2 };
        rows.push_str(&format!("{planned},{commit},{spent}\n"));
    }
    let html = smart_with_dict(
        &wrk,
        &rows,
        &pipeline_dict(r#"["totalplannedcommit","commitamt","spentamt"]"#),
    );
    assert!(
        html.contains(r#""type":"waterfall""#),
        "a declared pipeline whose stages do not nest draws as a bridge, not a funnel; html: \
         {html}"
    );
    assert!(
        !html.contains(r#""type":"funnel""#),
        "a funnel would assert the containment these totals contradict; html: {html}"
    );
    assert!(
        html.contains("exceeds"),
        "the subtitle must NAME the containment violation rather than hiding the panel; html: \
         {html}"
    );
    assert!(
        html.contains("stages do not nest"),
        "the subtitle must say why the form is a bridge; html: {html}"
    );
}

#[test]
fn viz_smart_funnel_warns_when_a_declared_stage_is_a_complement() {
    // INVERTED, and the assertion moved from html to STDERR on purpose. A complement column
    // (`unspentamt`) nests PERFECTLY inside its predecessor, so it reports 0% violations -- the
    // subtitle cannot surface it and no html assertion could either. The warning is the only
    // signal, so the warning is what must be asserted.
    let wrk = Workdir::new("viz_smart_funnel_warns_when_a_declared_stage_is_a_complement");
    let mut rows = String::from("totalplannedcommit,unspentamt\n");
    for i in 0..300 {
        let planned = ((i % 40) + 1) * 1000;
        rows.push_str(&format!("{planned},{}\n", planned / 4));
    }
    wrk.create_from_string("p.csv", &rows);
    wrk.create_from_string(
        "d.schema.json",
        r#"{
          "properties": {
            "totalplannedcommit": {"type":"integer","x-qsv":{"qsv_type":"Integer"}},
            "unspentamt": {"type":"integer","x-qsv":{"qsv_type":"Integer"}}
          },
          "x-qsv": { "relationships": [
            {"kind":"pipeline","members":["totalplannedcommit","unspentamt"]}
          ] }
        }"#,
    );
    let out_html = wrk.path("p.html").to_string_lossy().to_string();
    let mut cmd = wrk.command("viz");
    cmd.args(["smart", "p.csv", "-o", &out_html, "--dictionary"])
        .arg(wrk.path("d.schema.json"))
        .env("QSV_VIZ_NO_COMPRESS", "1");
    let stderr = wrk.output_stderr(&mut cmd);

    let html = wrk.read_to_string("p.html").unwrap();
    assert!(
        html.contains(r#""type":"funnel""#),
        "an explicit declaration outranks a name heuristic, so the funnel still draws; html: \
         {html}"
    );
    assert!(
        stderr.contains("complement") || stderr.contains("remainder"),
        "a complement stage must be WARNED about -- it nests perfectly, so nothing else can \
         reveal it; stderr: {stderr}"
    );
}

#[test]
fn viz_smart_funnel_skips_a_declared_stage_that_cannot_be_summed() {
    // A declaration outranks a NAME heuristic, but it cannot make a meaningless chart sensible:
    // summing an average or a rate is nonsense whoever asked for it, so `spent_pct` is refused.
    let wrk = Workdir::new("viz_smart_funnel_skips_a_declared_stage_that_cannot_be_summed");
    let mut rows = String::from("totalplannedcommit,commitamt,spent_pct\n");
    for i in 0..300 {
        let planned = ((i % 40) + 1) * 1000;
        let commit = planned * (i % 3) / 4;
        rows.push_str(&format!("{planned},{commit},{}\n", i % 100));
    }
    wrk.create_from_string("p.csv", &rows);
    wrk.create_from_string(
        "d.schema.json",
        r#"{
          "properties": {
            "totalplannedcommit": {"type":"integer","x-qsv":{"qsv_type":"Integer"}},
            "commitamt": {"type":"integer","x-qsv":{"qsv_type":"Integer"}},
            "spent_pct": {"type":"integer","x-qsv":{"qsv_type":"Integer"}}
          },
          "x-qsv": { "relationships": [
            {"kind":"pipeline","members":["totalplannedcommit","commitamt","spent_pct"]}
          ] }
        }"#,
    );
    let out_html = wrk.path("p.html").to_string_lossy().to_string();
    let mut cmd = wrk.command("viz");
    cmd.args(["smart", "p.csv", "-o", &out_html, "--dictionary"])
        .arg(wrk.path("d.schema.json"))
        .env("QSV_VIZ_NO_COMPRESS", "1");
    let stderr = wrk.output_stderr(&mut cmd);

    let html = wrk.read_to_string("p.html").unwrap();
    assert!(
        !html.contains(r#""type":"funnel""#),
        "an intensive measure must not be summed into a funnel; html: {html}"
    );
    assert!(
        stderr.contains("rate") || stderr.contains("average"),
        "the skip must say WHY; stderr: {stderr}"
    );
}

#[test]
fn viz_smart_funnel_ignores_a_pipeline_naming_a_missing_column() {
    // A stale hand-edited dictionary must degrade to "no funnel", never to an error -- this is a
    // dashboard, not a validator.
    let wrk = Workdir::new("viz_smart_funnel_ignores_a_pipeline_naming_a_missing_column");
    let html = smart_with_dict(
        &wrk,
        &pipeline_csv(),
        &pipeline_dict(r#"["totalplannedcommit","no_such_column"]"#),
    );
    assert!(
        !html.contains(r#""type":"funnel""#),
        "a declaration naming a column that does not exist yields no funnel; html: {html}"
    );
}

#[test]
fn viz_smart_builds_a_row_encoded_funnel_summing_a_value_column() {
    // The shape `viz smart` previously declined to guess at: stages as VALUES of one column.
    // Safe now because it is DECLARED, not inferred.
    let wrk = Workdir::new("viz_smart_builds_a_row_encoded_funnel_summing_a_value_column");
    let mut rows = String::from("stage,revenue,region\n");
    for i in 0..300 {
        // per-stage spend shrinks down the pipeline, so the stage TOTALS nest and the panel
        // stays a funnel -- this test is about the row encoding, not the form (see
        // `viz_smart_row_pipeline_that_grows_is_bridged` for the other branch)
        let (stage, rev) = match i % 10 {
            0..=4 => ("Impression", 90),
            5..=7 => ("Click", 20),
            8 => ("Lead", 4),
            _ => ("Conversion", 1),
        };
        rows.push_str(&format!("{stage},{rev},r{}\n", i % 3));
    }
    wrk.create_from_string("p.csv", &rows);
    wrk.create_from_string(
        "d.schema.json",
        r#"{
          "properties": {
            "stage": {"type":"string","x-qsv":{"qsv_type":"String","role":"dimension"}},
            "revenue": {"type":"integer","x-qsv":{"qsv_type":"Integer","role":"measure"}},
            "region": {"type":"string","x-qsv":{"qsv_type":"String","role":"dimension"}}
          },
          "x-qsv": { "relationships": [
            {"kind":"pipeline","members":["stage","revenue"],"stage_column":"stage",
             "stages":["Impression","Click","Lead","Conversion"],"value_column":"revenue"}
          ] }
        }"#,
    );
    let out_html = wrk.path("p.html").to_string_lossy().to_string();
    let mut cmd = wrk.command("viz");
    cmd.args(["smart", "p.csv", "-o", &out_html, "--dictionary"])
        .arg(wrk.path("d.schema.json"))
        .env("QSV_VIZ_NO_COMPRESS", "1");
    wrk.assert_success(&mut cmd);

    let html = wrk.read_to_string("p.html").unwrap();
    assert!(
        html.contains(r#""type":"funnel""#),
        "a declared row-encoded pipeline should earn a funnel panel; html: {html}"
    );
    assert!(
        html.contains(r#""y":["Impression","Click","Lead","Conversion"]"#),
        "stage VALUES become the bands, in declared order; html: {html}"
    );
    // the row encoding must NOT borrow the column encoding's complete-case wording: `reached`
    // sums to the denominator here, so "of complete cases" would assert a meaningless 100%
    assert!(
        html.contains("rows in declared stages"),
        "the hover must use the row-encoding wording; html: {html}"
    );
}

#[test]
fn viz_smart_row_funnel_counts_rows_when_no_value_column_is_declared() {
    let wrk = Workdir::new("viz_smart_row_funnel_counts_rows_when_no_value_column_is_declared");
    let mut rows = String::from("stage,region\n");
    for i in 0..300 {
        let stage = match i % 10 {
            0..=4 => "Impression",
            5..=7 => "Click",
            8 => "Lead",
            _ => "Conversion",
        };
        rows.push_str(&format!("{stage},r{}\n", i % 3));
    }
    wrk.create_from_string("p.csv", &rows);
    wrk.create_from_string(
        "d.schema.json",
        r#"{
          "properties": {
            "stage": {"type":"string","x-qsv":{"qsv_type":"String","role":"dimension"}},
            "region": {"type":"string","x-qsv":{"qsv_type":"String","role":"dimension"}}
          },
          "x-qsv": { "relationships": [
            {"kind":"pipeline","members":["stage"],"stage_column":"stage",
             "stages":["Impression","Click","Lead","Conversion"]}
          ] }
        }"#,
    );
    let out_html = wrk.path("p.html").to_string_lossy().to_string();
    let mut cmd = wrk.command("viz");
    cmd.args(["smart", "p.csv", "-o", &out_html, "--dictionary"])
        .arg(wrk.path("d.schema.json"))
        .env("QSV_VIZ_NO_COMPRESS", "1");
    wrk.assert_success(&mut cmd);

    let html = wrk.read_to_string("p.html").unwrap();
    assert!(
        html.contains(r#""type":"funnel""#),
        "a count-only row pipeline should still draw; html: {html}"
    );
    // With no value column there is no Amount line -- the count IS the measure. NB: plotly
    // unicode-escapes angle brackets, so the `<br>` separator is `<br>` in the emitted
    // HTML and must not be matched literally here.
    assert!(
        html.contains("Rows: ") && !html.contains("Rows reached:"),
        "a count-only funnel's hover must report a row count, not an amount or the column \
         encoding's wording; html: {html}"
    );
}

#[test]
fn viz_smart_row_pipeline_that_grows_is_bridged() {
    // The row encoding is not exempt from the form rule. A marketing pipeline summing REVENUE
    // grows down the stages even though the row COUNTS shrink -- revenue at conversion is not a
    // subset of revenue at impression -- so a funnel would widen downward and assert a
    // containment that does not hold. The declaration is still honoured: same stages, same
    // order, bridged instead.
    let wrk = Workdir::new("viz_smart_row_pipeline_that_grows_is_bridged");
    let mut rows = String::from("stage,revenue,region\n");
    for i in 0..300 {
        let (stage, rev) = match i % 10 {
            0..=4 => ("Impression", 1),
            5..=7 => ("Click", 4),
            8 => ("Lead", 20),
            _ => ("Conversion", 90),
        };
        rows.push_str(&format!("{stage},{rev},r{}\n", i % 3));
    }
    wrk.create_from_string("p.csv", &rows);
    wrk.create_from_string(
        "d.schema.json",
        r#"{
          "properties": {
            "stage": {"type":"string","x-qsv":{"qsv_type":"String","role":"dimension"}},
            "revenue": {"type":"integer","x-qsv":{"qsv_type":"Integer","role":"measure"}},
            "region": {"type":"string","x-qsv":{"qsv_type":"String","role":"dimension"}}
          },
          "x-qsv": { "relationships": [
            {"kind":"pipeline","members":["stage","revenue"],"stage_column":"stage",
             "stages":["Impression","Click","Lead","Conversion"],"value_column":"revenue"}
          ] }
        }"#,
    );
    let out_html = wrk.path("p.html").to_string_lossy().to_string();
    let mut cmd = wrk.command("viz");
    cmd.args(["smart", "p.csv", "-o", &out_html, "--dictionary"])
        .arg(wrk.path("d.schema.json"))
        .env("QSV_VIZ_NO_COMPRESS", "1");
    wrk.assert_success(&mut cmd);

    let html = wrk.read_to_string("p.html").unwrap();
    assert!(
        html.contains(r#""type":"waterfall""#),
        "a row-encoded pipeline whose value grows must be bridged; html: {html}"
    );
    assert!(
        html.contains("Pipeline bridge:"),
        "the title must say bridge, not funnel; html: {html}"
    );
    assert!(
        html.contains(
            r#""measure":["absolute","relative","total","relative","total","relative","total"]"#
        ),
        "four stages bridge into seven bars: a seed, then a step and a running total each; html: \
         {html}"
    );
    // the stage hover must use the DECLARED value-column label, not the column encoding's
    // generic "Amount" -- the bridge borrowed that wording and mislabelled every row encoding
    assert!(
        html.contains("revenue: "),
        "a row-encoded measure bridge must label the value with its declared value column; html: \
         {html}"
    );
    assert!(
        !html.contains("Amount: "),
        "\"Amount\" is the COLUMN encoding's wording; html: {html}"
    );
}

#[test]
fn viz_smart_row_count_bridge_says_rows_not_amount() {
    // A count-only row pipeline whose stages GROW. `reached` IS the value here, so the funnel
    // arm folds both into one "Rows: n of m" line; the bridge used to print "Amount: 100"
    // alongside a duplicate "Rows in stage: 100", labelling a row count as an amount and
    // stating the same number twice.
    let wrk = Workdir::new("viz_smart_row_count_bridge_says_rows_not_amount");
    let mut rows = String::from("stage,region\n");
    for i in 0..300 {
        let stage = if i % 6 == 0 {
            "Impression"
        } else if i % 6 < 3 {
            "Click"
        } else {
            "Conversion"
        };
        rows.push_str(&format!("{stage},r{}\n", i % 3));
    }
    wrk.create_from_string("p.csv", &rows);
    wrk.create_from_string(
        "d.schema.json",
        r#"{
          "properties": {
            "stage": {"type":"string","x-qsv":{"qsv_type":"String","role":"dimension"}},
            "region": {"type":"string","x-qsv":{"qsv_type":"String","role":"dimension"}}
          },
          "x-qsv": { "relationships": [
            {"kind":"pipeline","members":["stage"],"stage_column":"stage",
             "stages":["Impression","Click","Conversion"]}
          ] }
        }"#,
    );
    let out_html = wrk.path("p.html").to_string_lossy().to_string();
    let mut cmd = wrk.command("viz");
    cmd.args(["smart", "p.csv", "-o", &out_html, "--dictionary"])
        .arg(wrk.path("d.schema.json"))
        .env("QSV_VIZ_NO_COMPRESS", "1");
    wrk.assert_success(&mut cmd);

    let html = wrk.read_to_string("p.html").unwrap();
    assert!(
        html.contains(r#""type":"waterfall""#),
        "growing stage counts must be bridged; html: {html}"
    );
    assert!(
        html.contains("Rows: "),
        "a count-only bridge must call the value Rows; html: {html}"
    );
    assert!(
        !html.contains("Amount: "),
        "a row count is not an Amount; html: {html}"
    );
    assert!(
        !html.contains("Rows in stage: "),
        "the count line is redundant when the count IS the value; html: {html}"
    );
}

#[test]
fn viz_smart_row_funnel_tolerates_stage_case_drift_and_absent_stages() {
    // The stage values are transcribed by an LLM from the frequency distribution, so case drift
    // is a realistic failure that would otherwise silently zero a band. A declared stage with no
    // rows is legitimate and stays as a zero band -- dropping it would imply the process ends at
    // the last stage that happens to have data.
    let wrk = Workdir::new("viz_smart_row_funnel_tolerates_stage_case_drift_and_absent_stages");
    let mut rows = String::from("stage,region\n");
    for i in 0..300 {
        // impressions outnumber clicks so the counts nest and the panel stays a funnel: this
        // test is about case-insensitive stage matching, not the form
        let stage = if i % 3 == 0 { "click" } else { "impression" };
        rows.push_str(&format!("{stage},r{}\n", i % 3));
    }
    wrk.create_from_string("p.csv", &rows);
    wrk.create_from_string(
        "d.schema.json",
        r#"{
          "properties": {
            "stage": {"type":"string","x-qsv":{"qsv_type":"String","role":"dimension"}},
            "region": {"type":"string","x-qsv":{"qsv_type":"String","role":"dimension"}}
          },
          "x-qsv": { "relationships": [
            {"kind":"pipeline","members":["stage"],"stage_column":"stage",
             "stages":["Impression","Click","Conversion"]}
          ] }
        }"#,
    );
    let out_html = wrk.path("p.html").to_string_lossy().to_string();
    let mut cmd = wrk.command("viz");
    cmd.args(["smart", "p.csv", "-o", &out_html, "--dictionary"])
        .arg(wrk.path("d.schema.json"))
        .env("QSV_VIZ_NO_COMPRESS", "1");
    wrk.assert_success(&mut cmd);

    let html = wrk.read_to_string("p.html").unwrap();
    assert!(
        html.contains(r#""y":["Impression","Click","Conversion"]"#),
        "case-drifted stages must still match, and an absent stage stays as a zero band; html: \
         {html}"
    );
}

/// A dataset `viz smart` can actually profile: a low-cardinality dimension with repeated values
/// (so it earns a frequency bar) plus a numeric measure with a spread (so it earns a box panel).
fn cache_opts_csv() -> String {
    let mut s = String::from("region,amount\n");
    for (i, region) in ["north", "south", "east"]
        .iter()
        .cycle()
        .take(30)
        .enumerate()
    {
        s.push_str(&format!("{region},{}\n", 10 + (i * 7) % 53));
    }
    s
}

#[test]
fn viz_smart_headered_run_is_unaffected_by_a_prior_no_headers_run() {
    // REGRESSION: the stats cache is located by input path and validated by mtime, but was NOT
    // keyed by parsing options. `viz smart` forces the cache to regenerate under non-default
    // parsing (so a headered cache is never misread as headerless) -- but that force REWROTE the
    // shared sidecar, so the next HEADERED run happily reused a headerless cache: field names
    // became positional ("0", "1"), the header row was counted as data, and panels keyed off
    // column names (e.g. the KPI row) silently vanished.
    let wrk = Workdir::new("viz_smart_headered_run_is_unaffected_by_a_prior_no_headers_run");
    wrk.create_from_string("d.csv", &cache_opts_csv());

    let render = |name: &str| {
        let out = wrk.path(name).to_string_lossy().to_string();
        let mut cmd = wrk.command("viz");
        cmd.args(["smart", "d.csv", "-o", &out]);
        wrk.assert_success(&mut cmd);
        wrk.read_to_string(name).unwrap()
    };

    // baseline: a headered run against a clean cache
    let before = render("a.html");
    assert!(
        before.contains("region"),
        "baseline dashboard should name the real columns; html: {before}"
    );

    // poison: a --no-headers run rewrites the shared stats cache with positional field names
    let out_b = wrk.path("b.html").to_string_lossy().to_string();
    let mut cmd = wrk.command("viz");
    cmd.args(["smart", "d.csv", "--no-headers", "-o", &out_b]);
    wrk.assert_success(&mut cmd);

    // the identical headered command must still see the real headers
    let after = render("c.html");
    assert!(
        after.contains("region"),
        "a prior --no-headers run must not leave a cache that renames columns positionally; html: \
         {after}"
    );
}

#[test]
fn viz_smart_headered_run_is_unaffected_by_a_prior_custom_delimiter_run() {
    // Same root cause as above, via the other half of the force-regen condition. This one was
    // worse than wrong output: a wrong-delimiter run cached a SINGLE fused column, and the next
    // plain run then failed outright ("Could not compute statistics") on a perfectly valid file
    // until the sidecar was deleted by hand.
    let wrk = Workdir::new("viz_smart_headered_run_is_unaffected_by_a_prior_custom_delimiter_run");
    wrk.create_from_string("d.csv", &cache_opts_csv());

    // poison with a delimiter the file does not use, so the whole row parses as one column
    let out_a = wrk.path("a.html").to_string_lossy().to_string();
    let mut cmd = wrk.command("viz");
    cmd.args(["smart", "d.csv", "--delimiter", ";", "-o", &out_a]);
    // may or may not produce a usable dashboard; what matters is the cache it leaves behind
    let _ = cmd.output();

    let out_b = wrk.path("b.html").to_string_lossy().to_string();
    let mut cmd = wrk.command("viz");
    cmd.args(["smart", "d.csv", "-o", &out_b]);
    wrk.assert_success(&mut cmd);

    let html = wrk.read_to_string("b.html").unwrap();
    assert!(
        html.contains("region"),
        "a prior --delimiter run must not leave a cache that breaks the default-parsing run; \
         html: {html}"
    );
}

#[cfg(unix)]
#[test]
fn viz_smart_symlinked_input_is_unaffected_by_a_prior_no_headers_run() {
    // REGRESSION (roborev 3756): the JSONL stats cache is looked up beside the CANONICALIZED
    // input, but the `stats` subprocess is invoked with the original path and writes its
    // `<input>.stats.csv.json` metadata beside the SYMLINK. A guard that only consults the
    // canonical location finds no metadata there, waves the cache through, and the mismatched-
    // parsing-options bug returns via a symlink.
    let wrk = Workdir::new("viz_smart_symlinked_input_is_unaffected_by_a_prior_no_headers_run");
    std::fs::create_dir_all(wrk.path("sub")).unwrap();
    let mut csv = String::from("region,amount\n");
    for (i, region) in ["north", "south", "east"]
        .iter()
        .cycle()
        .take(30)
        .enumerate()
    {
        csv.push_str(&format!("{region},{}\n", 10 + (i * 7) % 53));
    }
    std::fs::write(wrk.path("sub/target.csv"), &csv).unwrap();
    std::os::unix::fs::symlink(wrk.path("sub/target.csv"), wrk.path("link.csv")).unwrap();

    let render = |name: &str| {
        let out = wrk.path(name).to_string_lossy().to_string();
        let mut cmd = wrk.command("viz");
        cmd.args(["smart", "link.csv", "-o", &out]);
        wrk.assert_success(&mut cmd);
        wrk.read_to_string(name).unwrap()
    };

    let before = render("a.html");
    assert!(
        before.contains("region"),
        "baseline via symlink should name the real columns; html: {before}"
    );

    let out_b = wrk.path("b.html").to_string_lossy().to_string();
    let mut cmd = wrk.command("viz");
    cmd.args(["smart", "link.csv", "--no-headers", "-o", &out_b]);
    wrk.assert_success(&mut cmd);

    let after = render("c.html");
    assert!(
        after.contains("region"),
        "a prior --no-headers run through the SAME symlink must not leave a cache that renames \
         columns positionally; html: {after}"
    );
}

#[cfg(unix)]
#[test]
fn viz_smart_direct_read_is_unaffected_by_a_prior_symlink_no_headers_run() {
    // REGRESSION (roborev 3760): `get_stats_records` writes the JSONL cache at the CANONICAL path
    // but the `stats` subprocess writes its metadata sidecar beside the path it was GIVEN. Mixed
    // direct/symlink access therefore desynced them: a direct run left a headered canonical
    // sidecar, a `--no-headers` run through the symlink replaced the canonical JSONL underneath
    // it, and the next direct run read stale-but-matching metadata and reused the no-headers
    // cache -- the very mismatch the guard exists to prevent.
    let wrk = Workdir::new("viz_smart_direct_read_is_unaffected_by_a_prior_symlink_no_headers_run");
    std::fs::create_dir_all(wrk.path("sub")).unwrap();
    let mut csv = String::from("region,amount\n");
    for (i, region) in ["north", "south", "east"]
        .iter()
        .cycle()
        .take(30)
        .enumerate()
    {
        csv.push_str(&format!("{region},{}\n", 10 + (i * 7) % 53));
    }
    std::fs::write(wrk.path("sub/target.csv"), &csv).unwrap();
    std::os::unix::fs::symlink(wrk.path("sub/target.csv"), wrk.path("link.csv")).unwrap();

    let render_direct = |name: &str| {
        let out = wrk.path(name).to_string_lossy().to_string();
        let mut cmd = wrk.command("viz");
        cmd.args(["smart", "sub/target.csv", "-o", &out]);
        wrk.assert_success(&mut cmd);
        wrk.read_to_string(name).unwrap()
    };

    // 1. direct headered run -> leaves a headered canonical metadata sidecar
    let before = render_direct("a.html");
    assert!(
        before.contains("region"),
        "baseline direct run should name the real columns; html: {before}"
    );

    // 2. --no-headers run through the SYMLINK -> rewrites the canonical JSONL
    let out_b = wrk.path("b.html").to_string_lossy().to_string();
    let mut cmd = wrk.command("viz");
    cmd.args(["smart", "link.csv", "--no-headers", "-o", &out_b]);
    wrk.assert_success(&mut cmd);

    // 3. the identical direct run must not read the no-headers cache
    let after = render_direct("c.html");
    assert!(
        after.contains("region"),
        "a prior --no-headers run through a SYMLINK to this file must not leave a cache the \
         direct read accepts; html: {after}"
    );
}

#[test]
fn viz_smart_notes_and_drops_dictionary_with_no_headers() {
    // A dictionary is keyed by column NAME, but --no-headers names columns positionally, so no
    // entry can ever match. Rather than let the flag look applied (and, for `--dictionary infer`,
    // pay for an LLM pass whose verdicts are all discarded), viz says so and charts from stats
    // alone -- the dashboard still renders.
    let wrk = Workdir::new("viz_smart_notes_and_drops_dictionary_with_no_headers");
    let mut csv = String::from("region,amount\n");
    for (i, region) in ["north", "south", "east"]
        .iter()
        .cycle()
        .take(30)
        .enumerate()
    {
        csv.push_str(&format!("{region},{}\n", 10 + (i * 7) % 53));
    }
    wrk.create_from_string("d.csv", &csv);
    wrk.create_from_string(
        "dict.json",
        r#"{"fields":[{"name":"amount","concept":"measure.amount"}]}"#,
    );

    let out = wrk.path("d.html").to_string_lossy().to_string();
    let mut cmd = wrk.command("viz");
    cmd.args([
        "smart",
        "d.csv",
        "--no-headers",
        "--dictionary",
        "dict.json",
        "-o",
        &out,
    ]);
    wrk.assert_success(&mut cmd);

    let stderr = wrk.output_stderr(&mut cmd);
    assert!(
        stderr.contains("no entry can match"),
        "expected a notice explaining the dictionary cannot apply; got: {stderr}"
    );
    let html = wrk.read_to_string("d.html").unwrap();
    assert!(
        html.contains("Plotly.newPlot"),
        "the dashboard must still render from statistics alone; html: {html}"
    );
}

// Axis titles go through plotly's pseudo-HTML text renderer, so a CSV header containing markup
// would render as live markup (a `<a href>` link, a `<b>` bold, a `<span style>` — the same
// spoofing sink already closed for the panel title and the 3D/animated-bubble axis titles).
// The standalone chart paths (`build_layout`, and the --slider layout) resolved their axis title
// from the raw header, so they were the twins that kept the sink open.
// An explicit --x-title/--y-title is operator-supplied and deliberately stays raw, like --title.
#[test]
fn viz_axis_and_colorbar_titles_escape_markup_headers_but_not_flags() {
    let wrk = Workdir::new("viz_axis_and_colorbar_titles_escape_markup_headers_but_not_flags");
    wrk.create_from_string(
        "evil.csv",
        "<b>Region</b>,<a href=\"https://evil.example\">Amount</a>\nnorth,10\nsouth,20\n",
    );
    let x = "<b>Region</b>";
    let y = "<a href=\"https://evil.example\">Amount</a>";

    let mut cmd = wrk.command("viz");
    cmd.args(["bar", "evil.csv", "--x", x, "--y", y]);
    let out = wrk.output(&mut cmd);
    assert!(out.status.success());
    let html = String::from_utf8_lossy(&out.stdout);

    // `escape_hover` turns the header into `&lt;b&gt;...`, then plotly's serializer additionally
    // \u-escapes the `&`, so the axis title reaches the page as literal text -- never a tag.
    assert!(
        html.contains(r"\u0026lt;b\u0026gt;Region\u0026lt;/b\u0026gt;"),
        "the x-axis title must be escaped, not rendered as markup"
    );
    assert!(
        html.contains(r"\u0026lt;a href="),
        "the y-axis title must be escaped, not rendered as a live link"
    );
    // the unescaped header must not survive in either serialized form
    assert!(
        !html.contains(r"\u003cb\u003eRegion") && !html.contains("<b>Region"),
        "raw markup header leaked into an axis title"
    );

    // an explicit --x-title is operator-supplied and deliberately stays raw, like --title
    let mut cmd = wrk.command("viz");
    cmd.args([
        "bar",
        "evil.csv",
        "--x",
        x,
        "--y",
        y,
        "--x-title",
        "<b>Mine</b>",
    ]);
    let out = wrk.output(&mut cmd);
    assert!(out.status.success());
    let html = String::from_utf8_lossy(&out.stdout);
    assert!(
        html.contains(r"\u003cb\u003eMine\u003c/b\u003e"),
        "an explicit --x-title is operator-supplied and must not be escaped"
    );

    // colorbar titles are the same plotly markup sink, resolved from the same raw headers. The
    // scatter builder already escaped this very variable for the HOVER text while passing it raw
    // to the colorbar, so it drew a live link as the scale title.
    wrk.create_from_string(
        "cb.csv",
        "x,y,<a href=\"https://evil.example\">Amount</a>\n1,2,3\n4,5,6\n",
    );
    let mut cmd = wrk.command("viz");
    cmd.args(["scatter", "cb.csv", "--x", "x", "--y", "y", "--color", y]);
    let out = wrk.output(&mut cmd);
    assert!(out.status.success());
    let html = String::from_utf8_lossy(&out.stdout);
    // every assertion below is anchored to the distinctive evil host rather than to a bare
    // `<a href=`, so unrelated document chrome or an uncompressed embedded plotly bundle can
    // neither satisfy the positive nor trip the negatives.
    assert!(
        html.contains(r#"\u0026lt;a href=\"https://evil.example\"\u0026gt;Amount"#),
        "the colorbar title must be escaped, not rendered as a live link"
    );
    assert!(
        !html.contains(r#"\u003ca href=\"https://evil.example"#)
            && !html.contains(r#"<a href="https://evil.example"#),
        "raw markup header leaked into the colorbar title"
    );
}

// Slider step labels and the current-value readout are RENDERED surfaces: plotly's
// `drawLabel` (`sliders/draw.js:365`) and `drawCurrentValue` (`:322`) both pipe their text through
// `svgTextUtils.convertToTspans`, so a frame value or slider column header carrying markup renders
// as a tag. Verified in-browser: an `<a href>` frame value produced two live anchors (one in the
// step label, one in the current-value readout), each with a working `xlink:href`. See #4333.
//
// The invariant has TWO halves, and asserting only the first is not enough -- a later sweep that
// escaped all four fields would still pass a label-only assertion while silently desynchronizing
// the slider from the frames:
//   1. the step `label` and the current-value `prefix` ARE escaped, and
//   2. the step `value` and its animate arg are STILL byte-equal to `Frame::name`.
//
// Note the two serialized forms below: `escape_hover` turns `<` into `&lt;`, and plotly's
// serializer then u-escapes the `&` -> `\u0026lt;`. An UNescaped value keeps its `<`, which the
// same serializer emits as `\u003c`. So `\u0026` == escaped, `\u003c` == raw.
#[test]
fn viz_slider_escapes_rendered_labels_but_keeps_frame_identity_raw() {
    let wrk = Workdir::new("viz_slider_escapes_rendered_labels_but_keeps_frame_identity_raw");
    // markup in BOTH the frame values (step labels) and the slider column header (prefix)
    wrk.create_from_string(
        "evil.csv",
        "region,gdp,wellbeing,<b>Period</b>\n\
         North,10,20,<b>Q1</b>\n\
         South,12,22,<b>Q1</b>\n\
         North,14,26,\"<a href=\"\"https://evil.example\"\">Q2</a>\"\n\
         South,16,28,\"<a href=\"\"https://evil.example\"\">Q2</a>\"\n",
    );

    let mut cmd = wrk.command("viz");
    cmd.args([
        "scatter",
        "evil.csv",
        "--x",
        "gdp",
        "--y",
        "wellbeing",
        "--series",
        "region",
        "--slider",
        "<b>Period</b>",
    ]);
    let out = wrk.output(&mut cmd);
    assert!(out.status.success());
    let html = String::from_utf8_lossy(&out.stdout);

    // --- half 1: the two RENDERED strings are escaped --------------------------------------
    assert!(
        html.contains(r#""label":"\u0026lt;b\u0026gt;Q1\u0026lt;/b\u0026gt;""#),
        "the slider step label must be escaped, not rendered as markup; html: {html}"
    );
    assert!(
        html.contains(r#""prefix":"\u0026lt;b\u0026gt;Period\u0026lt;/b\u0026gt;: ""#),
        "the slider current-value prefix must be escaped, not rendered as markup"
    );
    // strongest form: NO step label anywhere keeps a raw `<`, so none can open a tag
    assert!(
        !html.contains(r#""label":"\u003c"#),
        "a raw markup frame value leaked into a rendered slider step label"
    );

    // --- half 2: the IDENTITY fields stay byte-identical to Frame::name ---------------------
    // escaping these would desynchronize step -> frame matching (the #4247 decision), and
    // `sliders/defaults.js:101-102` (`coerce('value', label)`) means dropping the explicit
    // `.value()` would silently make it inherit the escaped label.
    for raw in [
        r#"\u003cb\u003eQ1\u003c/b\u003e"#,
        r#"\u003ca href=\"https://evil.example\"\u003eQ2\u003c/a\u003e"#,
    ] {
        assert!(
            html.contains(&format!(r#""value":"{raw}""#)),
            "the slider step value must stay raw so it still matches Frame::name; missing {raw}"
        );
        assert!(
            html.contains(&format!(r#""name":"{raw}""#)),
            "the animation frame name must stay raw; missing {raw}"
        );
        // `args[0]` is the frame key the step animates to -- must match Frame::name exactly
        assert!(
            html.contains(&format!(r#""args":[["{raw}"]"#)),
            "the slider step animate target must stay raw and match Frame::name; missing {raw}"
        );
    }
}

// ---- viz trace-name escaping sweep (issue #4331) -------------------------------------------

/// The #4247/#4254 sweeps escaped panel titles and axis/colorbar titles, but the plotly TRACE
/// NAME -- which plotly renders as markup in the legend and hover box -- was left raw at 19
/// sites. This covers one `panel.name` site (viz smart) and one `y_label` site (viz box); a
/// single-site test would pass while the other 18 stayed wrong. NOTE it pins two MEMBERS, not
/// the whole class -- notably the TimeSeries `y_label` (the site carrying LLM-derived grain
/// text) takes the count path here, which interpolates a localized unit rather than a header,
/// so it is not directly exercised. All 19 sites share the identical one-line sink.
#[test]
fn viz_trace_names_escape_markup_headers_exactly_once() {
    let wrk = Workdir::new("viz_trace_names_escape_markup_headers_exactly_once");
    // the categorical column carries BOTH `&` and tags: `&` is what catches a double-escape,
    // the tags are what catch a missing escape
    wrk.create_from_string(
        "evil.csv",
        "R&D <b>Region</b>,<b>Amount</b>\nnorth,10\nsouth,20\nnorth,30\nsouth,40\n",
    );

    // --- panel.name sink: the viz smart frequency-bar panel ---
    // `--preview-threshold 0` keeps this panel-scoped: the data viewer embeds every column
    // NAME verbatim in its own `qsv-data-cols` JSON (neutralized there by `inline_json_script`,
    // a different sink with its own escaping), which would otherwise satisfy the negatives below
    let mut cmd = wrk.command("viz");
    cmd.args([
        "smart",
        "evil.csv",
        "--preview-threshold",
        "0",
        "-o",
        "smart.html",
    ]);
    cmd.env("QSV_VIZ_NO_COMPRESS", "1");
    wrk.assert_success(&mut cmd);
    let html = wrk.read_to_string("smart.html").unwrap();

    // escape_hover produces `R&amp;D &lt;b&gt;...`, then plotly's serializer additionally
    // \u-escapes every `&`, so the trace name reaches the page as literal text -- never a tag
    assert!(
        html.contains(r#""name":"R\u0026amp;D \u0026lt;b\u0026gt;Region\u0026lt;/b\u0026gt;"#),
        "the smart panel trace name must be escaped, not rendered as markup"
    );
    // the raw header must not survive as a trace name. Anchored on the `"name":"` field so
    // unrelated chrome (or the embedded plotly bundle) can neither satisfy nor trip it.
    assert!(
        !html.contains(r#""name":"R\u0026D \u003cb\u003eRegion"#),
        "raw markup header leaked into a trace name"
    );
    // ...and escaped exactly ONCE. No committed fixture header carries `&`/`<`/`>`, so the
    // golden check cannot see a double-escape -- this assertion is the only guard.
    assert!(
        !html.contains(r"\u0026amp;amp;") && !html.contains(r"\u0026amp;lt;"),
        "trace name was double-escaped"
    );

    // --- y_label sink: the standalone box chart, resolved from the same raw header ---
    let mut cmd = wrk.command("viz");
    cmd.args(["box", "evil.csv", "--y", "<b>Amount</b>", "-o", "box.html"]);
    wrk.assert_success(&mut cmd);
    let html = wrk.read_to_string("box.html").unwrap();

    assert!(
        html.contains(r#""name":"\u0026lt;b\u0026gt;Amount\u0026lt;/b\u0026gt;"#),
        "the box trace name must be escaped, not rendered as markup"
    );
    assert!(
        !html.contains(r#""name":"\u003cb\u003eAmount"#),
        "raw markup header leaked into the box trace name"
    );
    assert!(
        !html.contains(r"\u0026amp;lt;"),
        "box trace name was double-escaped"
    );
}

// ---- viz smart data viewer drawer (issue #4283) -------------------------------------------

/// A small mixed-type CSV for the data viewer tests: numeric, date, categorical, and text.
fn data_viewer_csv(wrk: &Workdir) {
    wrk.create_from_string(
        "dv.csv",
        "name,amt,when,grade\nalpha,1.5,2023-01-02,A\nbeta,2.25,2023-02-03,B\ngamma,3.75,\
         2023-03-04,A\ndelta,4.5,2023-04-05,C\nepsilon,5.25,2023-05-06,B\n",
    );
}

// Under the (default 50k) threshold every row embeds and the metadata Rows cell links
// "(Explore)"; the page carries the payloads, the drawer chrome, and the DataTables init with
// global search + SearchBuilder + per-column filter inputs.
#[test]
fn viz_smart_data_viewer_explore_link_under_threshold() {
    let wrk = Workdir::new("viz_smart_data_viewer_explore_link_under_threshold");
    data_viewer_csv(&wrk);

    let mut cmd = wrk.command("viz");
    cmd.env("QSV_VIZ_NO_COMPRESS", "1");
    cmd.args(["smart", "dv.csv"]);
    let out = wrk.output(&mut cmd);
    assert!(out.status.success());
    let html = String::from_utf8_lossy(&out.stdout);

    // the metadata Rows cell carries the Explore link
    assert!(html.contains("qsvOpenData()\">(Explore)<svg"));
    // the link must pin :visited too — href="#" means one click would otherwise leave it the
    // UA's #551A8B forever, unreadable on dark paper
    assert!(html.contains(".qsv-viz-meta a.qsv-data-link:visited"));
    assert!(!html.contains("(Preview)"));
    // payloads: plain JSON rows + column config, with a recognizable cell value
    assert!(html.contains(r#"id="qsv-data-rows" type="application/json""#));
    assert!(html.contains(r#"id="qsv-data-cols" type="application/json""#));
    assert!(html.contains("epsilon"));
    // drawer chrome + DataTable init + the searches the issue asks for
    assert!(html.contains("Data — all 5 rows"));
    assert!(html.contains("new DataTable"));
    // SearchBuilder rides in a Buttons popover ("Filter (n)") on the controls row, not a
    // permanent pane; ColumnControl's in-header widgets + Responsive column collapse are on
    assert!(html.contains("searchBuilder"));
    assert!(html.contains(r#"button: { 0: "Advanced Filter", _: "Advanced Filter (%d)" }"#));
    assert!(!html.contains(r#"top1: "searchBuilder""#));
    // scrollX, NOT Responsive: Responsive collapses a column together with its ColumnControl
    // search widget, so on a wide table the per-column controls the drawer exists to offer are
    // exactly the ones a reader cannot reach. The two options are mutually exclusive.
    assert!(html.contains("scrollX: true"));
    // scoped to the emitted indentation: plotly's own config also carries `responsive: true`
    assert!(!html.contains("\n        responsive: true,"));
    // ColumnControl owns both header rows: titles + ordering in row 0, the per-column search
    // widget in a row 1 it creates itself. Cramming both into one cell squeezes the title into
    // a multi-line stack, which is why the search is on its own row (as ColumnControl's own
    // demo does). The hand-rolled filter row and everything that kept it in step are gone --
    // ColumnControl keeps its own row aligned with Responsive and with ordering.
    assert!(html.contains(r#"{ target: 0, content: ["order"] }"#));
    assert!(html.contains(r#"{ target: 1, content: ["search"] }"#));
    assert!(!html.contains("qsv-data-filters"));
    assert!(!html.contains("syncFilters"));
    // the two sticky rows pin as ONE element (sticky on thead), so no measured per-row offset
    assert!(!html.contains("--qsv-data-th1-h"));
    assert!(html.contains("#qsv-data-drawer div.dt-scroll-head { position: sticky !important;"));
    // resizable drawer grip + focus management + stacking above the dictionary drawer
    assert!(html.contains("qsv-data-drawer-grip"));
    // the scroll region must NOT keep DataTables' .dt-layout-row align-items:center — a
    // taller-than-viewport table would be vertically centered, hiding the first rows behind
    // the sticky thead with the top overflow unreachable by scrolling
    assert!(html.contains("overflow: auto; align-items: flex-start;"));
    // the persisted px height is re-clamped in CSS against the current viewport (roborev #3864)
    assert!(html.contains("--qsv-data-h-eff: clamp(calc(20 * var(--qsv-data-vh))"));
    assert!(html.contains(r#"drawer.setAttribute("tabindex", "-1")"#));
    assert!(html.contains("z-index: 1120"));
    // plain embed of the vendored library (NO_COMPRESS): bundle header + real <style>
    assert!(html.contains(r#"id="qsv-data-lib" type="text/javascript""#));
    assert!(html.contains("DataTables 3."));
}

// Above the threshold only the first N rows embed and the link reads "(Preview)"; the drawer
// title says so, and rows past the cut are NOT in the page.
#[test]
fn viz_smart_data_viewer_preview_over_threshold() {
    let wrk = Workdir::new("viz_smart_data_viewer_preview_over_threshold");
    let mut csv = String::from("name,amt,grade\n");
    for i in 1..=10 {
        let grade = if i % 2 == 0 { "A" } else { "B" };
        csv.push_str(&format!("row{i}sentinel,{i},{grade}\n"));
    }
    wrk.create_from_string("ten.csv", &csv);

    let mut cmd = wrk.command("viz");
    cmd.env("QSV_VIZ_NO_COMPRESS", "1");
    cmd.args(["smart", "ten.csv", "--preview-threshold", "5"]);
    let out = wrk.output(&mut cmd);
    assert!(out.status.success());
    let html = String::from_utf8_lossy(&out.stdout);

    assert!(html.contains("qsvOpenData()\">(Preview)<svg"));
    assert!(!html.contains("(Explore)"));
    assert!(html.contains("Data — first 5 of 10 rows (preview)"));
    assert!(html.contains("row5sentinel"));
    assert!(!html.contains("row6sentinel"));
    // `grade` is a 2-value categorical, so on a COMPLETE preview it would get ColumnControl's
    // searchList. It must not here: searchList builds its options from the EMBEDDED rows, and
    // on a truncated preview that list silently omits values present in the dataset. Every
    // column falls back to text search, which is honest about matching only what is shown.
    assert!(html.contains(r#""title":"grade","type":"string","list":false"#));
    assert!(!html.contains(r#""list":true"#));
}

// On a COMPLETE preview the low-cardinality string columns opt into ColumnControl's searchList
// (a checkbox dropdown of the distinct values); high-cardinality strings, dates and numerics do
// not — the value list is only useful, and only correct, for categoricals.
#[test]
fn viz_smart_data_viewer_searchlist_only_for_low_cardinality_strings() {
    let wrk = Workdir::new("viz_smart_data_viewer_searchlist_only_for_low_cardinality_strings");
    let mut csv = String::from("uniq,grade,amt,when\n");
    for i in 1..=40 {
        let grade = if i % 2 == 0 { "A" } else { "B" };
        csv.push_str(&format!(
            "uniqueval{i},{grade},{i},2026-01-{:02}\n",
            (i % 28) + 1
        ));
    }
    wrk.create_from_string("card.csv", &csv);

    let mut cmd = wrk.command("viz");
    cmd.env("QSV_VIZ_NO_COMPRESS", "1");
    cmd.args(["smart", "card.csv"]);
    let out = wrk.output(&mut cmd);
    assert!(out.status.success());
    let html = String::from_utf8_lossy(&out.stdout);

    // 2 distinct values -> list; 40 distinct values -> no list even though it is a string
    assert!(html.contains(r#""title":"grade","type":"string","list":true"#));
    assert!(html.contains(r#""title":"uniq","type":"string","list":false"#));
    // numerics get searchNumber and dates get the picker, both better than a value list
    assert!(html.contains(r#""title":"amt","type":"num","list":false"#));
    assert!(html.contains(r#""title":"when","type":"date","list":false"#));
    // the listed columns are wired through columnDefs. The value list is NESTED (a dropdown):
    // rendered flat it puts Select/Deselect/Search in the header cell and forces the column as
    // wide as those three controls. A per-column columnControl replaces the table-wide one
    // rather than merging, so the override restates row 0 too.
    assert!(html.contains(r#"{ target: 1, content: [["searchList"]] }"#));
    assert!(html.contains(r#"{ target: 0, content: ["order"] }"#));
}

// B': a date column's `filter` orthogonal must hand back the ISO sort key, NOT the source text.
// ColumnControl's searchDateTime compares epochs, and Date.parse reads a slash-formatted date as
// LOCAL midnight while its own picker value is ISO/UTC midnight — so source text never matches
// (and a day-first date parses to NaN outright). The key puts both sides on UTC midnight.
#[test]
fn viz_smart_data_viewer_date_filter_uses_iso_sort_key() {
    let wrk = Workdir::new("viz_smart_data_viewer_date_filter_uses_iso_sort_key");
    let mut csv = String::from("when,amt\n");
    for i in 1..=12 {
        csv.push_str(&format!("{:02}/03/2026,{i}\n", i + 10));
    }
    wrk.create_from_string("dmy.csv", &csv);

    let mut cmd = wrk.command("viz");
    cmd.env("QSV_VIZ_NO_COMPRESS", "1");
    cmd.env("QSV_PREFER_DMY", "1");
    cmd.args(["smart", "dmy.csv"]);
    let out = wrk.output(&mut cmd);
    assert!(out.status.success());
    let html = String::from_utf8_lossy(&out.stdout);

    // filter takes the ISO DAY of the key; display still routes through the escaper
    assert!(html.contains("var day = isoDay(key(d));"));
    assert!(html.contains("return day === null ? text.filter(raw(d)) : day;"));
    assert!(html.contains(r#"display: function (d) { return text.display(raw(d)); }"#));
    // the day-first source text is what the reader sees, and the ISO key rides alongside it
    assert!(html.contains(r#"["11/03/2026","2026-03-11T00:00:00+00:00"]"#));
}

// The day truncation has to apply to the UNPAIRED branch too. An ISO datetime cell is already
// ISO-leading, so collect_datatable_rows never pairs it -- if only the paired branch truncated,
// an all-ISO DateTime column would keep a full timestamp as its filter value and `equals <day>`
// would be unsatisfiable there while working fine on a month-first column. Same rule, both
// branches: `isoDay` is applied to whatever the cell resolves to.
#[test]
fn viz_smart_data_viewer_iso_datetime_filter_truncates_to_day() {
    let wrk = Workdir::new("viz_smart_data_viewer_iso_datetime_filter_truncates_to_day");
    let mut csv = String::from("seen,amt\n");
    for i in 1..=12 {
        csv.push_str(&format!("2026-03-{:02}T{:02}:15:00,{i}\n", i, (i % 12) + 8));
    }
    wrk.create_from_string("isodt.csv", &csv);

    let mut cmd = wrk.command("viz");
    cmd.env("QSV_VIZ_NO_COMPRESS", "1");
    cmd.args(["smart", "isodt.csv"]);
    let out = wrk.output(&mut cmd);
    assert!(out.status.success());
    let html = String::from_utf8_lossy(&out.stdout);

    // stats typed it as a datetime, and the cells stay UNPAIRED (already ISO-leading): the cell
    // sits directly beside the next column's value, with no [raw, key] pair around it
    assert!(html.contains(r#""title":"seen","type":"date""#));
    assert!(html.contains(r#"["2026-03-01T09:15:00","1"]"#));
    assert!(!html.contains(r#""2026-03-01T09:15:00+00:00""#));
    // the single truncation path covers them: isoDay is applied to the resolved cell value
    assert!(html.contains("var day = isoDay(key(d));"));
    assert!(html.contains(r#"/^\d{4}-\d{2}-\d{2}/.test(v) ? v.slice(0, 10) : null"#));
}

// The drawer's controls row carries a CSV export button next to the SearchBuilder popover.
// It is deliberately configured with NO exportOptions: Buttons' defaults already export every
// column (Responsive's collapsed set never touches DataTables' visibility) and the "display"
// orthogonal round-trips markup/entity cell values, which `orthogonal: "export"` would strip.
// Assertions use the emitted spacing so they cannot match the minified bundle, which embeds
// plaintext in NO_COMPRESS pages and contains these words itself.
#[test]
fn viz_smart_data_viewer_csv_export_button() {
    let wrk = Workdir::new("viz_smart_data_viewer_csv_export_button");
    data_viewer_csv(&wrk);

    let mut cmd = wrk.command("viz");
    cmd.env("QSV_VIZ_NO_COMPRESS", "1");
    cmd.args(["smart", "dv.csv"]);
    let out = wrk.output(&mut cmd);
    assert!(out.status.success());
    let html = String::from_utf8_lossy(&out.stdout);

    assert!(html.contains(r#"extend: "csv""#));
    // full embed: plain label, file named for the input stem
    assert!(html.contains(r#"text: "CSV""#));
    assert!(html.contains(r#"filename: "dv""#));
    // neither tempting override is present
    assert!(!html.contains(r#"orthogonal: "export""#));
    assert!(!html.contains(r#"columns: ":all""#));
    // the XSS guard the export fidelity depends on is still on the columns
    assert!(html.contains("DataTable.render.text()"));
    // The thead has TWO rows -- titles and ColumnControl's search row -- and the CSV writer
    // serializes every header row, so the search row would land in the file as a blank line
    // under the header. It is matched by the `data-dt-order="disable"` attribute ColumnControl
    // puts on the row it creates: that row is built from `td` cells, so a class- or th-based
    // matcher misses it entirely. The hook must sit INSIDE exportOptions; a level up it is
    // silently never called.
    assert!(html.contains("exportOptions: {"));
    assert!(html.contains("customizeData: function (d) {"));
    assert!(html.contains(r#"getAttribute("data-dt-order") === "disable""#));
    // supplying exportOptions replaces csvHtml5's own default object, so the CSV-injection
    // guard has to be restated rather than inherited
    assert!(html.contains("escapeExcelFormula: true"));
}

// The controls row carries three buttons in a fixed left-to-right order: the SearchBuilder
// popover, Clear Filters, and the CSV export.
#[test]
fn viz_smart_data_viewer_toolbar_button_order() {
    let wrk = Workdir::new("viz_smart_data_viewer_toolbar_button_order");
    data_viewer_csv(&wrk);

    let mut cmd = wrk.command("viz");
    cmd.env("QSV_VIZ_NO_COMPRESS", "1");
    cmd.args(["smart", "dv.csv"]);
    let out = wrk.output(&mut cmd);
    assert!(out.status.success());
    let html = String::from_utf8_lossy(&out.stdout);

    let at = |needle: &str| {
        html.find(needle)
            .unwrap_or_else(|| panic!("data viewer HTML is missing {needle}"))
    };
    // positions in the emitted buttons array, which is the rendered order
    let sb = at(r#"extend: "searchBuilder""#);
    let clear = at(r#"text: "Clear Filters""#);
    let csv = at(r#"extend: "csv""#);
    assert!(
        sb < clear && clear < csv,
        "controls-row buttons are out of order: searchBuilder@{sb} Clear@{clear} csv@{csv} — \
         expected searchBuilder < Clear Filters < CSV"
    );
}

// Clear Filters has to reset every filter the drawer offers, and they live in three separate
// places that do not clear each other. columns().search("") is NOT redundant with ColumnControl's
// searchClear(): ColumnControl applies its search through its own mechanism, so a column filtered
// by the widget still reports an empty column().search() and clearing one leaves the other.
#[test]
fn viz_smart_data_viewer_clear_filters_resets_all_three_sources() {
    let wrk = Workdir::new("viz_smart_data_viewer_clear_filters_resets_all_three_sources");
    data_viewer_csv(&wrk);

    let mut cmd = wrk.command("viz");
    cmd.env("QSV_VIZ_NO_COMPRESS", "1");
    cmd.args(["smart", "dv.csv"]);
    let out = wrk.output(&mut cmd);
    assert!(out.status.success());
    let html = String::from_utf8_lossy(&out.stdout);

    assert!(html.contains(r#"text: "Clear Filters""#));
    // global search box
    assert!(html.contains(r#"dt.search("");"#));
    // raw per-column search AND ColumnControl's own widget state
    assert!(html.contains(r#"dt.columns().search("");"#));
    assert!(html.contains("dt.columns().columnControl.searchClear();"));
    // SearchBuilder criteria
    assert!(html.contains("dt.searchBuilder.rebuild({});"));
}

// Clear Filters is enabled only while there is something to clear, and its label counts the
// active filters across all three sources. Each source is watched through the only channel
// that actually exposes it: SearchBuilder through its filterChanged hook (criteria edits do
// not always redraw, and the stock button leaves the hook unused), ColumnControl through the
// named fixed searches it applies ("dtcc"/"dtcc-list" — column().search() stays empty, the
// same test its own ccSearchClear button runs), and the global box plus the widgets re-read
// on every draw.
#[test]
fn viz_smart_data_viewer_clear_filters_enablement_and_count() {
    let wrk = Workdir::new("viz_smart_data_viewer_clear_filters_enablement_and_count");
    data_viewer_csv(&wrk);

    let mut cmd = wrk.command("viz");
    cmd.env("QSV_VIZ_NO_COMPRESS", "1");
    cmd.args(["smart", "dv.csv"]);
    let out = wrk.output(&mut cmd);
    assert!(out.status.success());
    let html = String::from_utf8_lossy(&out.stdout);

    // SearchBuilder's share arrives through the filterChanged config hook
    assert!(html.contains("config: { filterChanged: function (n) {"));
    assert!(html.contains("sbCount = n;"));
    // ColumnControl's share is read off the named fixed searches its widgets apply
    assert!(html.contains(r#"this.search.fixed("dtcc") || this.search.fixed("dtcc-list")"#));
    // enablement and the counted label, with the plain label as the zero state
    assert!(html.contains("btn.enable(n > 0);"));
    assert!(html.contains(r#"btn.text(n > 0 ? "Clear Filters (" + n + ")" : "Clear Filters");"#));
    // the global box and the widgets are re-read on every draw
    assert!(html.contains("dt.on(\"draw\", updateClearFilters);"));
}

// scrollX replaced Responsive outright, so the drawer must carry neither the collapsing option
// nor the toggle that used to switch it off -- and the header, which scrollX lifts into its own
// div.dt-scroll-head above the body, has to be pinned there instead of on the body table's thead.
#[test]
fn viz_smart_data_viewer_scrollx_replaces_responsive() {
    let wrk = Workdir::new("viz_smart_data_viewer_scrollx_replaces_responsive");
    data_viewer_csv(&wrk);

    let mut cmd = wrk.command("viz");
    cmd.env("QSV_VIZ_NO_COMPRESS", "1");
    cmd.args(["smart", "dv.csv"]);
    let out = wrk.output(&mut cmd);
    assert!(out.status.success());
    let html = String::from_utf8_lossy(&out.stdout);

    assert!(html.contains("scrollX: true"));
    // scoped to the emitted indentation: plotly's own config also carries `responsive: true`
    // (as `{ responsive: true, scrollZoom: ... }`), so a bare substring test always matches
    assert!(!html.contains("\n        responsive: true,"));
    // no toggle, and none of the machinery it needed
    assert!(!html.contains(r#"text: "Responsive""#));
    assert!(!html.contains("var responsiveOn"));
    assert!(!html.contains("dt.responsive.rebuild()"));
    assert!(!html.contains("qsv-responsive-toggle"));
    // the header is pinned on .dt-scroll-head, and `!important` is required because DataTables
    // writes `position: relative` INLINE on that element, which outranks any selector
    assert!(html.contains("#qsv-data-drawer div.dt-scroll-head { position: sticky !important;"));
    assert!(!html.contains("#qsv-data-table thead { position: sticky"));
}

// A truncated preview must not hand back a file that looks complete: both the button label and
// the download's name say "preview".
#[test]
fn viz_smart_data_viewer_csv_export_preview_is_labeled() {
    let wrk = Workdir::new("viz_smart_data_viewer_csv_export_preview_is_labeled");
    let mut csv = String::from("name,amt,grade\n");
    for i in 1..=10 {
        let grade = if i % 2 == 0 { "A" } else { "B" };
        csv.push_str(&format!("row{i},{i},{grade}\n"));
    }
    wrk.create_from_string("ten.csv", &csv);

    let mut cmd = wrk.command("viz");
    cmd.env("QSV_VIZ_NO_COMPRESS", "1");
    cmd.args(["smart", "ten.csv", "--preview-threshold", "5"]);
    let out = wrk.output(&mut cmd);
    assert!(out.status.success());
    let html = String::from_utf8_lossy(&out.stdout);

    assert!(html.contains(r#"text: "CSV (preview)""#));
    assert!(html.contains(r#"filename: "ten-preview""#));
}

// The export file name is derived from the input, which is not always a plain file name — a
// `dc:<name>` cache input keeps its prefix, and a colon is illegal in a Windows file name. Only
// portable characters survive. (A space stands in for the colon here: it is legal in a file
// name on every platform the tests run on, so the fixture itself stays portable.)
#[test]
fn viz_smart_data_viewer_csv_export_name_is_sanitized() {
    let wrk = Workdir::new("viz_smart_data_viewer_csv_export_name_is_sanitized");
    let mut csv = String::from("name,amt,grade\n");
    for i in 1..=5 {
        let grade = if i % 2 == 0 { "A" } else { "B" };
        csv.push_str(&format!("row{i},{i},{grade}\n"));
    }
    wrk.create_from_string("od d.csv", &csv);

    let mut cmd = wrk.command("viz");
    cmd.env("QSV_VIZ_NO_COMPRESS", "1");
    cmd.args(["smart", "od d.csv"]);
    let out = wrk.output(&mut cmd);
    assert!(out.status.success());
    let html = String::from_utf8_lossy(&out.stdout);

    assert!(html.contains(r#"filename: "od-d""#));
}

// The drawer sizes itself against the viewport it must fit inside. Embedded in the gallery's
// auto-sized iframes a vh resolves against the iframe — grown to the dashboard's full content
// height — so the drawer opened taller than the window. It asks the parent for the real
// viewport and converts the answer to px-per-vh.
#[test]
fn viz_smart_data_viewer_asks_parent_for_viewport() {
    let wrk = Workdir::new("viz_smart_data_viewer_asks_parent_for_viewport");
    data_viewer_csv(&wrk);

    let mut cmd = wrk.command("viz");
    cmd.env("QSV_VIZ_NO_COMPRESS", "1");
    cmd.args(["smart", "dv.csv"]);
    let out = wrk.output(&mut cmd);
    assert!(out.status.success());
    let html = String::from_utf8_lossy(&out.stdout);

    // both halves of the handshake
    assert!(html.contains("qsvVizWantViewport"));
    assert!(html.contains("qsvVizViewport"));
    // the default must sit on :root, never on body — a body declaration would beat the
    // inherited value the handshake writes to documentElement and pin every page to 1vh
    assert!(html.contains(":root { --qsv-data-vh: 1vh; }"));
    assert!(!html.contains("body { --qsv-data-vh"));
    // the height clamp and the grip's drag bounds both go through it
    assert!(html.contains("calc(90 * var(--qsv-data-vh))"));
    assert!(html.contains("vpH * 0.9"));
    // standalone pages never ask, so they keep the plain vh behavior
    assert!(html.contains("window.self !== window.top"));
    // the ask is repeated on every open: an embedder that installs its listener below the
    // iframe markup (the gallery does, ~320KB below) never hears the load-time one, and a
    // dropped answer has no other recovery
    assert!(html.contains("function askViewport()"));
    assert!(html.contains("askViewport();"));
}

// DataTables orders its "date" type through the browser's Date.parse, which rejects day-first
// text (25/07/2026 is month 25) and leaves such a column in source order. Those cells embed as
// [raw, sort-key] so ordering follows qsv's own parse while the display keeps the source text.
// Cells that already lead with an ISO date sort correctly as-is and stay plain strings.
#[test]
fn viz_smart_data_viewer_date_sort_keys() {
    let wrk = Workdir::new("viz_smart_data_viewer_date_sort_keys");
    wrk.create_from_string(
        "dates.csv",
        "cat,dayfirst_date,iso_date,amt\nalpha,25/07/2026,2026-07-25,1\nbeta,30/11/2025,\
         2025-11-30,2\ngamma,13/02/2026,2026-02-13,3\nalpha,19/03/2026,2026-03-19,4\n",
    );

    let mut cmd = wrk.command("viz");
    cmd.env("QSV_VIZ_NO_COMPRESS", "1");
    cmd.args(["smart", "dates.csv"]);
    let out = wrk.output(&mut cmd);
    assert!(out.status.success());
    let html = String::from_utf8_lossy(&out.stdout);

    // stats typed both date columns, so both carry the DataTables date type
    assert!(html.contains(r#""title":"dayfirst_date","type":"date""#));
    assert!(html.contains(r#""title":"iso_date","type":"date""#));
    // day-first cells pair the source text with qsv's reading of it -- 25/07 is July 25, which
    // Date.parse would have rejected outright
    assert!(html.contains(r#"["25/07/2026","2026-07-25T00:00:00+00:00"]"#));
    assert!(html.contains(r#"["30/11/2025","2025-11-30T00:00:00+00:00"]"#));
    // the ISO column pays nothing: its cells stay plain strings
    assert!(!html.contains(r#"["2026-07-25","#));
    // display keeps the source text inside render.text()'s escaping; filter deliberately does
    // NOT -- it hands back the ISO day so ColumnControl's epoch comparison can match (B').
    // text.filter survives only as the fallback for a cell with no ISO day to offer (a blank).
    assert!(html.contains("text.display(raw(d))"));
    assert!(html.contains("var day = isoDay(key(d));"));
    assert!(html.contains("return day === null ? text.filter(raw(d)) : day;"));
}

// A column stats typed as a date can still hold blanks (a cell that fails to parse would have
// made stats type the whole column String instead, so junk never reaches here). A blank has no
// better ordering to offer than itself and stays a plain string rather than becoming a pair.
#[test]
fn viz_smart_data_viewer_unparseable_dates_stay_plain() {
    let wrk = Workdir::new("viz_smart_data_viewer_unparseable_dates_stay_plain");
    wrk.create_from_string(
        "dates.csv",
        "cat,when_date,amt\nalpha,25/07/2026,1\nbeta,,2\ngamma,13/02/2026,3\nalpha,19/03/2026,4\n",
    );

    let mut cmd = wrk.command("viz");
    cmd.env("QSV_VIZ_NO_COMPRESS", "1");
    cmd.args(["smart", "dates.csv"]);
    let out = wrk.output(&mut cmd);
    assert!(out.status.success());
    let html = String::from_utf8_lossy(&out.stdout);

    // the column is still a date column, and its parseable cells still pair
    assert!(html.contains(r#""title":"when_date","type":"date""#));
    assert!(html.contains(r#"["25/07/2026","2026-07-25T00:00:00+00:00"]"#));
    // no pair was minted for the blank: its row embeds it as a plain empty string. Anchored on
    // the whole row — a bare `["","` also occurs inside the plaintext minified bundle.
    assert!(html.contains(r#"["beta","","2"]"#));
}

// issue #4303, data-viewer half: the drawer's `[raw, sort-key]` date pairs must be minted under
// the column's DECLARED format too, not the global QSV_PREFER_DMY. Otherwise the table sorts and
// filters an ambiguous date column by the opposite reading of the one the charts plot.
#[test]
fn viz_smart_data_viewer_dict_format_sort_keys() {
    let wrk = Workdir::new("viz_smart_data_viewer_dict_format_sort_keys");
    // `when_date` is ambiguous (05/03 is 3 May day-first, 5 Mar month-first); `iso_date` is
    // already ISO, so it must stay a plain string and cost nothing extra either way.
    wrk.create_from_string(
        "dates.csv",
        "when_date,iso_date,qty\n05/03/2021,2021-05-03,1\n07/02/2021,2021-07-02,2\n11/01/2021,\
         2021-11-01,3\n02/04/2021,2021-02-04,4\n",
    );
    wrk.create_from_string(
        "dict.schema.json",
        r#"{
          "$schema": "https://json-schema.org/draft/2020-12/schema",
          "type": "object",
          "properties": {
            "when_date": { "type": "string", "title": "When",
              "x-qsv": { "qsv_type": "Date", "content_type": "date:%m/%d/%Y",
                "role": "timestamp", "concept": "time.event_timestamp" } },
            "iso_date": { "type": "string", "title": "ISO",
              "x-qsv": { "qsv_type": "Date", "content_type": "date:%Y-%m-%d" } },
            "qty": { "type": "integer", "title": "Qty",
              "x-qsv": { "qsv_type": "Integer", "content_type": "unknown" } }
          }
        }"#,
    );

    let out_html = wrk.path("dash.html").to_string_lossy().to_string();
    let mut cmd = wrk.command("viz");
    cmd.env("QSV_VIZ_NO_COMPRESS", "1");
    cmd.env("QSV_PREFER_DMY", "1");
    cmd.args(["smart", "dates.csv", "-o", &out_html, "--dictionary"])
        .arg(wrk.path("dict.schema.json"));
    wrk.assert_success(&mut cmd);

    let html = wrk.read_to_string("dash.html").unwrap();
    // still a date column, and its sort key is the MONTH-first reading the dictionary declares:
    // 05/03/2021 -> 3 May would be the QSV_PREFER_DMY=1 answer this must override.
    assert!(html.contains(r#""title":"when_date","type":"date""#));
    assert!(
        html.contains(r#"["05/03/2021","2021-05-03T00:00:00+00:00"]"#),
        "the data viewer's date sort key must follow the dictionary's declared format"
    );
    assert!(html.contains(r#"["11/01/2021","2021-11-01T00:00:00+00:00"]"#));
    // the already-ISO column mints no pair at all
    assert!(!html.contains(r#""2021-05-03","2021-05-03T"#));
}

// `--preview-threshold 0` disables the viewer entirely: no link, no payloads, no drawer, no
// DataTables bundle — the dashboard carries no trace of the feature.
#[test]
fn viz_smart_data_viewer_disabled_at_zero() {
    let wrk = Workdir::new("viz_smart_data_viewer_disabled_at_zero");
    data_viewer_csv(&wrk);

    let mut cmd = wrk.command("viz");
    cmd.env("QSV_VIZ_NO_COMPRESS", "1");
    cmd.args(["smart", "dv.csv", "--preview-threshold", "0"]);
    let out = wrk.output(&mut cmd);
    assert!(out.status.success());
    let html = String::from_utf8_lossy(&out.stdout);

    assert!(!html.contains("(Explore)"));
    assert!(!html.contains("(Preview)"));
    assert!(!html.contains("qsvOpenData"));
    assert!(!html.contains("qsv-data-rows"));
    assert!(!html.contains("qsv-data-drawer"));
    assert!(!html.contains("DataTables 3."));
}

// Column types come from the whole-dataset stats, not client-side detection: numeric columns get
// DataTables type "num", date columns "date", text "string" — driving both sorting and the
// SearchBuilder condition set per column.
#[test]
fn viz_smart_data_viewer_column_types_from_stats() {
    let wrk = Workdir::new("viz_smart_data_viewer_column_types_from_stats");
    data_viewer_csv(&wrk);

    let mut cmd = wrk.command("viz");
    cmd.env("QSV_VIZ_NO_COMPRESS", "1");
    cmd.args(["smart", "dv.csv"]);
    let out = wrk.output(&mut cmd);
    assert!(out.status.success());
    let html = String::from_utf8_lossy(&out.stdout);

    assert!(html.contains(r#"{"title":"name","type":"string","list":true}"#));
    assert!(html.contains(r#"{"title":"amt","type":"num","list":false}"#));
    assert!(html.contains(r#"{"title":"when","type":"date","list":false}"#));
    assert!(html.contains(r#"{"title":"grade","type":"string","list":true}"#));
}

// By default (compression on) a non-trivial rows payload embeds as gzip+base64 — no plaintext
// cell values in the page — and inflates back to the exact rows JSON.
#[test]
fn viz_smart_data_viewer_gzip_payload_when_compressing() {
    let wrk = Workdir::new("viz_smart_data_viewer_gzip_payload_when_compressing");
    let mut csv = String::from("name,amt,grade\n");
    for i in 1..=300 {
        let grade = if i % 2 == 0 { "A" } else { "B" };
        csv.push_str(&format!("padpadpadpad-row{i}-sentinel,{i},{grade}\n"));
    }
    wrk.create_from_string("many.csv", &csv);

    let mut cmd = wrk.command("viz");
    cmd.args(["smart", "many.csv"]);
    let out = wrk.output(&mut cmd);
    assert!(out.status.success());
    let html = String::from_utf8_lossy(&out.stdout);

    assert!(html.contains(r#"id="qsv-data-rows" type="application/gzip-b64""#));
    assert!(!html.contains("row299-sentinel"));
    let rows_json = inflate_gz_payload(&html, "qsv-data-rows");
    assert!(rows_json.contains("padpadpadpad-row299-sentinel"));
    let rows: serde_json::Value = serde_json::from_str(&rows_json).unwrap();
    assert_eq!(rows.as_array().unwrap().len(), 300);
    // the DataTables library rides as its own gzip payload, inflated on first drawer open
    assert!(html.contains(r#"id="qsv-data-lib" type="application/gzip-b64""#));
    assert!(html.contains(r#"id="qsv-data-css" type="application/gzip-b64""#));
}

// A rows payload below the gzip floor stays plain JSON even when compression is on — and the
// column config payload is always plain. Neither plain payload path can reference __qsvGunzip
// unconditionally (it only exists when the page carries gzip payloads elsewhere).
#[test]
fn viz_smart_data_viewer_small_payload_stays_plain() {
    let wrk = Workdir::new("viz_smart_data_viewer_small_payload_stays_plain");
    data_viewer_csv(&wrk);

    let mut cmd = wrk.command("viz");
    cmd.args(["smart", "dv.csv"]);
    let out = wrk.output(&mut cmd);
    assert!(out.status.success());
    let html = String::from_utf8_lossy(&out.stdout);

    // 5 rows serialize well under DATATABLE_GZ_MIN_BYTES: plain even though compression is on
    assert!(html.contains(r#"id="qsv-data-rows" type="application/json""#));
    assert!(html.contains(r#"id="qsv-data-cols" type="application/json""#));
    // the library still ships compressed
    assert!(html.contains(r#"id="qsv-data-lib" type="application/gzip-b64""#));
}

// Cell values are data, not markup: a cell containing `</script>` and `&` embeds \u-escaped in
// the plain payload so it can neither truncate the script tag nor inject HTML.
#[test]
fn viz_smart_data_viewer_cell_escaping() {
    let wrk = Workdir::new("viz_smart_data_viewer_cell_escaping");
    wrk.create_from_string(
        "evil.csv",
        "name,amt,grade\nx</script><b>&y,1,A\nplain,2,B\nother,3,A\nmore,4,B\nlast,5,A\n",
    );

    let mut cmd = wrk.command("viz");
    cmd.env("QSV_VIZ_NO_COMPRESS", "1");
    cmd.args(["smart", "evil.csv"]);
    let out = wrk.output(&mut cmd);
    assert!(out.status.success());
    let html = String::from_utf8_lossy(&out.stdout);

    assert!(html.contains(r"x\u003c/script\u003e\u003cb\u003e\u0026y"));
    assert!(!html.contains("x</script>"));
}

// Under --no-headers the DataTable column titles use the stats' generated names (0-based column
// indices), consistent with the panel titles.
#[test]
fn viz_smart_data_viewer_no_headers_labels() {
    let wrk = Workdir::new("viz_smart_data_viewer_no_headers_labels");
    wrk.create_from_string("nh.csv", "1,A\n2,B\n3,A\n4,B\n5,A\n");

    let mut cmd = wrk.command("viz");
    cmd.env("QSV_VIZ_NO_COMPRESS", "1");
    cmd.args(["smart", "nh.csv", "--no-headers"]);
    let out = wrk.output(&mut cmd);
    assert!(out.status.success());
    let html = String::from_utf8_lossy(&out.stdout);

    assert!(html.contains(r#"{"title":"0","type":"num","list":false}"#));
    assert!(html.contains(r#"{"title":"1","type":"string","list":true}"#));
}

// Under QSV_VIZ_CDN the DataTables bundle is version-pinned CDN tags with Subresource Integrity
// (like plotly's) instead of embedded payloads.
#[test]
fn viz_smart_data_viewer_cdn_tags_carry_sri() {
    let wrk = Workdir::new("viz_smart_data_viewer_cdn_tags_carry_sri");
    data_viewer_csv(&wrk);

    let mut cmd = wrk.command("viz");
    cmd.env("QSV_VIZ_CDN", "1");
    cmd.args(["smart", "dv.csv"]);
    let out = wrk.output(&mut cmd);
    assert!(out.status.success());
    let html = String::from_utf8_lossy(&out.stdout);

    assert!(html.contains(
        "https://cdn.datatables.net/v/dt/dt-3.0.1/b-4.0.1/cc-2.0.0/date-2.0.0/sb-2.0.0/datatables.min.js"
    ));
    assert!(html.contains(
        "https://cdn.datatables.net/v/dt/dt-3.0.1/b-4.0.1/cc-2.0.0/date-2.0.0/sb-2.0.0/datatables.min.css"
    ));
    // both tags carry integrity + crossorigin; no embedded library payloads remain
    assert_eq!(html.matches("cdn.datatables.net").count(), 2);
    assert!(!html.contains(r#"id="qsv-data-lib""#));
    assert!(!html.contains(r#"id="qsv-data-css""#));
    // the rows/cols payloads still embed (client-side processing, per the issue)
    assert!(html.contains(r#"id="qsv-data-rows""#));
    assert!(html.contains(r#"id="qsv-data-cols""#));
}

// ---------------------------------------------------------------------------------------------
// Data viewer <-> map point selection (rows <-> points cross-link)
// ---------------------------------------------------------------------------------------------

/// Local-extent coordinates -> a MapLibre `scattermap` panel. Row 2 has an EMPTY latitude and row
/// 4 an out-of-range one, so both are dropped from the map while still consuming a data-row
/// ordinal — which is exactly what the emitted `ids` have to prove.
fn map_select_csv(wrk: &Workdir) {
    wrk.create_from_string(
        "ms.csv",
        "name,lat,lon,val\na,40.70,-74.00,1\nb,40.71,-74.01,2\nc,,-74.02,3\nd,40.73,-74.03,4\ne,\
         999,-74.04,5\nf,40.75,-74.05,6\n",
    );
}

// The map trace carries each point's 0-based data-row ordinal in `ids`, and the page carries the
// bridge that joins those to the drawer's rows.
#[test]
fn viz_smart_map_select_ids_emitted() {
    let wrk = Workdir::new("viz_smart_map_select_ids_emitted");
    map_select_csv(&wrk);

    let mut cmd = wrk.command("viz");
    cmd.env("QSV_VIZ_NO_COMPRESS", "1");
    cmd.args(["smart", "ms.csv"]);
    let out = wrk.output(&mut cmd);
    assert!(out.status.success());
    let html = String::from_utf8_lossy(&out.stdout);

    // ORDINAL ALIGNMENT: rows 2 (empty lat) and 4 (lat out of range) never reach the map, but
    // they still occupy an ordinal — so the ids skip them rather than renumbering 0..3.
    assert!(
        html.contains(r#""ids":["0","1","3","5"]"#),
        "map trace ids are not the data-row ordinals with the unmappable rows skipped"
    );
    // the selected/unselected styling is baked into the trace so the bridge only sets
    // `selectedpoints`
    assert!(html.contains(r##""selected":{"marker":{"color":"#ff2d95","size":14}}"##));
    assert!(html.contains(r#""unselected":{"marker":{"opacity":0.12}}"#));
    // the bridge itself. `__qsvSelIndex` is unique to MAP_SELECT_CHROME — `__qsvSelRehook` is
    // NOT, since the theme toggle references it unconditionally.
    assert!(html.contains("gd.__qsvSelIndex = index;"));
    assert!(html.contains("window.__qsvSelRehook = hook;"));
    // and the drawer's half of the seam
    assert!(html.contains("window.__qsvDataSelect = function (indexes, extend)"));
    assert!(html.contains(r#"document.dispatchEvent(new CustomEvent("qsv-data-selection""#));
    // The camera half. Assert on the qsv-namespaced marker, NEVER on the MapLibre call names:
    // `easeTo`, `fitBounds` and `getBounds` all occur inside the bundled plotly.js too, so an
    // assertion on those is satisfied by the bundle alone and proves nothing (the same trap that
    // once let a `hasChoropleth` assertion pass against a renamed function).
    assert!(
        html.contains("gd.__qsvSelCamera = k;"),
        "map camera reveal missing from the bridge"
    );
    // only a selection that came FROM the table moves the camera — without this gate a map click
    // echoes back and re-aims the map away from the point just clicked
    assert!(html.contains(r#"ev.detail.source === "rows""#));
    // and the camera mirror that keeps `gd.layout` authoritative across a restyle
    assert!(html.contains("m.__qsvCamMirror = true;"));
}

// With the drawer disabled there is nothing to cross-link to, so neither the ids nor the bridge
// are emitted — the map payload stays exactly as it was before the feature existed.
#[test]
fn viz_smart_map_select_gated_off_without_drawer() {
    let wrk = Workdir::new("viz_smart_map_select_gated_off_without_drawer");
    map_select_csv(&wrk);

    let mut cmd = wrk.command("viz");
    cmd.env("QSV_VIZ_NO_COMPRESS", "1");
    cmd.args(["smart", "ms.csv", "--preview-threshold", "0"]);
    let out = wrk.output(&mut cmd);
    assert!(out.status.success());
    let html = String::from_utf8_lossy(&out.stdout);

    // Assert on the ORDINAL pattern, not on `"ids":[` alone: sunburst/treemap/icicle traces carry
    // their own unrelated `ids` (plotly node identity), which is also why the bridge type-gates on
    // scattermap/scattergeo rather than on the mere presence of the key.
    assert!(
        !html.contains(r#""ids":["0","1","3","5"]"#),
        "row ids leaked into a drawer-less page"
    );
    assert!(
        !html.contains("gd.__qsvSelIndex = index;"),
        "bridge emitted with no drawer"
    );
    assert!(
        !html.contains("gd.__qsvSelCamera = k;"),
        "map camera reveal emitted with no drawer"
    );
    // the drawer really is absent (guards against the assertions above passing vacuously)
    assert!(!html.contains(r#"id="qsv-data-rows""#));
}

// A selected row whose point was never plotted (the map draws at most MAX_SMART_POINTS) still has
// a location in the drawer, and the pin marks it. The pin ships EMPTY and is filled in at runtime.
#[test]
fn viz_smart_map_row_pin_emitted() {
    let wrk = Workdir::new("viz_smart_map_row_pin_emitted");
    map_select_csv(&wrk);

    let mut cmd = wrk.command("viz");
    cmd.env("QSV_VIZ_NO_COMPRESS", "1");
    cmd.args(["smart", "ms.csv"]);
    let out = wrk.output(&mut cmd);
    assert!(out.status.success());
    let html = String::from_utf8_lossy(&out.stdout);

    // Two traces, because the ring CANNOT come from `marker.line`: plotly silently drops it on a
    // scattermap (the MapLibre circle layer gets no `circle-stroke-*` at all), so the halo is a
    // larger white marker drawn beneath. The halo must come FIRST in the payload for that.
    let halo = html
        .find(r##""name":"qsv-row-pin-halo","showlegend":false,"mode":"markers","lat":[],"lon":[],"hoverinfo":"skip","marker":{"opacity":1.0,"size":26,"color":"#ffffff"}"##)
        .expect("row pin halo trace missing or restyled");
    let pin = html
        .find(r##""name":"qsv-row-pin","showlegend":false,"mode":"markers","lat":[],"lon":[]"##)
        .expect("row pin trace missing or not shipped empty");
    assert!(halo < pin, "the halo must be drawn beneath the pin");
    assert!(html.contains(r##""marker":{"opacity":1.0,"size":18,"color":"#ff2d95"}"##));
    // the sentinel name is the bridge's handle and must never reach a reader: legend off on the
    // trace, and a hovertemplate ending in <extra></extra> (escaped by plotly) suppresses the
    // trace-name box
    // plotly \u-escapes angle brackets in emitted JSON, so the assertion has to match the
    // ESCAPED form -- and this file is patched programmatically for exactly that reason
    assert!(html.contains(r#"%{lat:.4f}, %{lon:.4f}\u003cextra\u003e\u003c/extra\u003e"#));
    // the coordinate columns the bridge reads a row's location from: `ms.csv` is name,lat,lon,val
    assert!(
        html.contains("var LAT_COL = 1, LON_COL = 2;"),
        "the resolved coordinate columns did not reach the bridge"
    );
    // and the drawer seam the bridge reads them THROUGH
    assert!(html.contains("window.__qsvDataRowCells = function (i)"));
    // The client-side coordinate parse must stay STRICTER than bare `Number()`, which also accepts
    // 0x/0o/0b literals that the server's `parse_f64` rejects — otherwise a cell the server refused
    // gets pinned at a bogus location instead of reporting that the row has no coordinates.
    assert!(
        html.contains(r"var DECIMAL_FLOAT = /^[+-]?(\d+\.?\d*|\.\d+)([eE][+-]?\d+)?$/;"),
        "the coordinate parse no longer mirrors the server's decimal-float syntax"
    );
}

// A density heatmap has no id-bearing point trace at all — its core carries no `ids`, and it may
// have no outlier markers either — so the bridge used to skip the panel outright while the pin
// traces were still emitted, leaving them unreachable. The pin does not need selectable points:
// on a heatmap EVERY row is "not among plotted points", so every one of them is pinnable.
#[test]
fn viz_smart_density_map_still_reaches_the_row_pin() {
    let wrk = Workdir::new("viz_smart_density_map_still_reaches_the_row_pin");
    map_select_csv(&wrk);

    let mut cmd = wrk.command("viz");
    cmd.env("QSV_VIZ_NO_COMPRESS", "1");
    // 4 mappable rows in the fixture, so this forces the heatmap branch
    cmd.args(["smart", "ms.csv", "--heatmap-density", "2"]);
    let out = wrk.output(&mut cmd);
    assert!(out.status.success());
    let html = String::from_utf8_lossy(&out.stdout);

    // the heatmap branch really was taken (guards against this passing vacuously as a marker map)
    assert!(html.contains(r#""type":"densitymap""#));
    // the core carries no ordinals, so nothing on this panel is clickable back to a row
    assert!(!html.contains(r#""ids":["0","1","3","5"]"#));
    // ...yet the pin ships, and the bridge accepts a pin-only panel as hookable
    assert!(html.contains(r#""name":"qsv-row-pin""#));
    assert!(
        html.contains("gd.data.some(isPointTrace) || hasPinPair(gd)"),
        "a pin-only panel would be skipped, leaving the pin traces unreachable"
    );
    // The pin sentinel is matched on TYPE as well as name. Trace names elsewhere in a dashboard
    // come from the data, so a bare name match would let a row whose category read "qsv-row-pin"
    // make an unrelated panel look pin-bearing -- and that panel, reached first, would answer for
    // the note. Both halves are required too, since the pin is only ever drawn as a pair.
    assert!(html.contains(
        r#"(t.type === "scattermap" || t.type === "scattergeo")
      && (t.name === "qsv-row-pin" || t.name === "qsv-row-pin-halo")"#
    ));
}

// No drawer, no pin: there is no row to pin and nothing that could ever fill it in.
#[test]
fn viz_smart_map_row_pin_gated_off_without_drawer() {
    let wrk = Workdir::new("viz_smart_map_row_pin_gated_off_without_drawer");
    map_select_csv(&wrk);

    let mut cmd = wrk.command("viz");
    cmd.env("QSV_VIZ_NO_COMPRESS", "1");
    cmd.args(["smart", "ms.csv", "--preview-threshold", "0"]);
    let out = wrk.output(&mut cmd);
    assert!(out.status.success());
    let html = String::from_utf8_lossy(&out.stdout);

    assert!(
        !html.contains("qsv-row-pin"),
        "row pin emitted with no drawer"
    );
    assert!(!html.contains("var LAT_COL ="));
    // anti-vacuity: the map itself is still there, it just carries no cross-link chrome
    assert!(html.contains(r#""type":"scattermap""#));
}

// A dashboard with a drawer but NO map gets the row-selection machinery (it is part of the
// drawer) but no bridge — there are no points to link to.
#[test]
fn viz_smart_drawer_select_without_map() {
    let wrk = Workdir::new("viz_smart_drawer_select_without_map");
    data_viewer_csv(&wrk);

    let mut cmd = wrk.command("viz");
    cmd.env("QSV_VIZ_NO_COMPRESS", "1");
    cmd.args(["smart", "dv.csv"]);
    let out = wrk.output(&mut cmd);
    assert!(out.status.success());
    let html = String::from_utf8_lossy(&out.stdout);

    assert!(html.contains(r#"id="qsv-data-rows""#));
    assert!(html.contains("window.__qsvDataSelect = function (indexes, extend)"));
    // paging alone only guarantees the right PAGE; the row still has to be scrolled inside the
    // drawer's own scrollport, clear of the sticky header
    assert!(
        html.contains("function revealRow(node)"),
        "drawer is missing the row scroll-into-view helper"
    );
    assert!(
        !html.contains("gd.__qsvSelIndex = index;"),
        "map<->rows bridge emitted on a dashboard with no map panel"
    );
    assert!(
        !html.contains("gd.__qsvSelCamera = k;"),
        "map camera reveal emitted on a dashboard with no map panel"
    );
}

// Points whose row lies past the drawer's preview prefix get an EMPTY id: they keep their slot
// (so every other point stays aligned) but cannot select anything.
#[test]
fn viz_smart_map_select_ids_empty_beyond_preview() {
    let wrk = Workdir::new("viz_smart_map_select_ids_empty_beyond_preview");
    map_select_csv(&wrk);

    let mut cmd = wrk.command("viz");
    cmd.env("QSV_VIZ_NO_COMPRESS", "1");
    // only the first 2 data rows are in the drawer; mappable rows 3 and 5 are past it
    cmd.args(["smart", "ms.csv", "--preview-threshold", "2"]);
    let out = wrk.output(&mut cmd);
    assert!(out.status.success());
    let html = String::from_utf8_lossy(&out.stdout);

    assert!(
        html.contains(r#""ids":["0","1","",""]"#),
        "ids past the preview prefix are not blanked (a bridge would select the wrong row)"
    );
}

// The same cross-link on the projection basemap: a global extent renders `scattergeo` instead of
// `scattermap`, through a separate build arm that has to emit ids too.
#[test]
fn viz_smart_map_select_scattergeo_variant() {
    let wrk = Workdir::new("viz_smart_map_select_scattergeo_variant");
    wrk.create_from_string(
        "geo.csv",
        "city,lat,lon,val\nnyc,40.7,-74.0,1\nlondon,51.5,-0.12,2\ntokyo,35.6,139.7,3\nsydney,-33.\
         8,151.2,4\nsaopaulo,-23.5,-46.6,5\nmoscow,55.7,37.6,6\n",
    );

    let mut cmd = wrk.command("viz");
    cmd.env("QSV_VIZ_NO_COMPRESS", "1");
    cmd.args(["smart", "geo.csv"]);
    let out = wrk.output(&mut cmd);
    assert!(out.status.success());
    let html = String::from_utf8_lossy(&out.stdout);

    // scoped to the scattergeo trace so the assertion cannot pass on some other trace type
    assert!(html.contains(r#""type":"scattergeo""#));
    assert!(html.contains(r#""ids":["0","1","2","3","4","5"]"#));
    assert!(html.contains("gd.__qsvSelIndex = index;"));
    // the geo panel gets its own pin (downsampling drops rows here too); the camera stays put on
    // a geo panel, but the pin is a highlight, not a camera move
    assert!(html.contains(r#""name":"qsv-row-pin-halo""#));
    assert!(html.contains(r#""name":"qsv-row-pin""#));
}

// The DataTables Select extension must stay OUT of the bundle. It requires a global jQuery that
// this page does not load (it would throw `ReferenceError: jQuery is not defined` and silently
// never register), and its mere presence would also re-scope the CSV export to "the selected rows,
// if any". The export button is deliberately left on Buttons' default row scope, which is correct
// ONLY while Select is absent — this test is what keeps that precondition true. (The export scope
// itself is not asserted here; it follows from the Buttons default plus this absence.)
#[test]
fn viz_smart_data_viewer_has_no_select_extension() {
    let wrk = Workdir::new("viz_smart_data_viewer_has_no_select_extension");
    map_select_csv(&wrk);

    let mut cmd = wrk.command("viz");
    cmd.env("QSV_VIZ_NO_COMPRESS", "1");
    cmd.args(["smart", "ms.csv"]);
    let out = wrk.output(&mut cmd);
    assert!(out.status.success());
    let html = String::from_utf8_lossy(&out.stdout);

    // the jQuery-requiring Select extension must not have crept back into the bundle
    assert!(
        !html.contains("DataTable.select.version"),
        "the DataTables Select extension is present — it requires jQuery, which this page does \
         not load, and it silently changes the CSV export scope"
    );
}

// ---------------------------------------------------------------------------
// Dashboard UI localization (--language / dictionary-detected language).
//
// The dashboard chrome follows the dataset's language: describegpt's whatlang pass
// records `x-qsv.detected_language_code` in the dictionary, and `--language`
// overrides it. English output must stay byte-identical -- the rest of this file
// (and the golden fixtures) is the assertion for that.
// ---------------------------------------------------------------------------

/// A dictionary carrying the top-level `x-qsv` language block describegpt emits when
/// whatlang's verdict holds across the full detection sample AND all 3 of its sub-samples.
/// Mirrors the producer-side shape pinned by tests/test_describegpt.rs.
fn spanish_dictionary() -> &'static str {
    r#"{
      "$schema": "https://json-schema.org/draft/2020-12/schema",
      "type": "object",
      "x-qsv": {
        "detected_language": "Spanish",
        "detected_language_code": "spa",
        "detected_language_confidence": 0.9912
      },
      "properties": {
        "region": { "type": "string", "title": "Region",
          "x-qsv": { "qsv_type": "String", "role": "dimension", "concept": "category.status" } },
        "revenue": { "type": "integer", "title": "Revenue",
          "x-qsv": { "qsv_type": "Integer", "role": "measure", "concept": "measure.amount" } }
      }
    }"#
}

fn localization_rows() -> &'static str {
    "region,revenue\nNorte,100\nSur,220\nEste,150\nOeste,90\nNorte,180\nSur,130\nEste,175\nOeste,\
     142\n"
}

#[test]
fn viz_smart_follows_dictionary_detected_language() {
    let wrk = Workdir::new("viz_smart_follows_dictionary_detected_language");
    wrk.create_from_string("rev.csv", localization_rows());
    wrk.create_from_string("dict.schema.json", spanish_dictionary());

    let out_html = wrk.path("dash.html").to_string_lossy().to_string();
    let mut cmd = wrk.command("viz");
    cmd.args(["smart", "rev.csv", "-o", &out_html, "--dictionary"])
        .arg(wrk.path("dict.schema.json"));
    wrk.assert_success(&mut cmd);

    let html = wrk.read_to_string("dash.html").unwrap();
    assert!(
        html.contains(r#"<html lang="es">"#),
        "a dictionary detecting Spanish should set the document language to es"
    );
    assert!(
        html.contains("\u{1F313} Tema"),
        "dashboard chrome should be translated, not just the lang attribute"
    );
    assert!(
        !html.contains("\u{1F313} Theme"),
        "the English chrome string should be replaced, not duplicated alongside the translation"
    );
}

/// The same dictionary plus the `x-qsv.generated_by` attribution block describegpt bakes in.
/// `\n` is JSON's own escape here, so the parsed value is the multi-line block the drawer
/// renders in its `qsv-dict-prov` footer.
fn spanish_dictionary_with_provenance() -> &'static str {
    r#"{
      "$schema": "https://json-schema.org/draft/2020-12/schema",
      "type": "object",
      "x-qsv": {
        "detected_language": "Spanish",
        "detected_language_code": "spa",
        "detected_language_confidence": 0.9912,
        "generated_by": "Generated by qsv v21.1.0 describegpt\nCommand line: qsv describegpt --dictionary rev.csv\nPrompt file: Default v8.0.0\nModel: google/gemma-4-26b-a4b\nLLM API URL: http://localhost:1234/v1\nLanguage: es\nTimestamp: 2026-07-31T03:09:38.316096+00:00\n\nWARNING: Label, Description and Content Type generated by an LLM and may contain inaccuracies. Verify before using!"
      },
      "properties": {
        "region": { "type": "string", "title": "Region",
          "x-qsv": { "qsv_type": "String", "role": "dimension", "concept": "category.status" } },
        "revenue": { "type": "integer", "title": "Revenue",
          "x-qsv": { "qsv_type": "Integer", "role": "measure", "concept": "measure.amount" } }
      }
    }"#
}

// End-to-end wiring for the drawer's attribution block. The unit tests in viz.rs cover
// `localize_dict_provenance` itself; this one proves the render site actually calls it and that
// the translated block survives into the emitted page.
#[test]
fn viz_smart_dictionary_attribution_is_localized() {
    let wrk = Workdir::new("viz_smart_dictionary_attribution_is_localized");
    wrk.create_from_string("rev.csv", localization_rows());
    wrk.create_from_string("dict.schema.json", spanish_dictionary_with_provenance());

    let out_html = wrk.path("dash.html").to_string_lossy().to_string();
    let mut cmd = wrk.command("viz");
    // `--dict-info` is what renders the dictionary PAGE (and so the provenance footer); without
    // it the dictionary still drives the panels but the drawer carries no attribution block.
    cmd.args([
        "smart",
        "rev.csv",
        "-o",
        &out_html,
        "--language",
        "es",
        "--dict-info",
        "--dictionary",
    ])
    .arg(wrk.path("dict.schema.json"));
    wrk.assert_success(&mut cmd);

    let html = wrk.read_to_string("dash.html").unwrap();

    // Scope every assertion to the footer itself. A page-wide `contains` would be checking the
    // embedded plotly/DataTables bundles too, where a stray "Timestamp:" would fail the
    // English-absence checks below for a reason that has nothing to do with this feature.
    const OPEN: &str = "<footer class=\"qsv-dict-prov\"><pre>";
    let start = html
        .find(OPEN)
        .expect("drawer should carry a provenance footer")
        + OPEN.len();
    let end = start
        + html[start..]
            .find("</pre>")
            .expect("unterminated provenance block");
    let prov = &html[start..end];

    for expected in [
        "Generado por qsv v21.1.0 describegpt",
        "Línea de comandos: qsv describegpt --dictionary rev.csv",
        "Archivo de prompt: Default v8.0.0",
        "Modelo: google/gemma-4-26b-a4b",
        "URL de la API del LLM: http://localhost:1234/v1",
        "Idioma: es",
        "Marca de tiempo: 2026-07-31T03:09:38.316096+00:00",
        "ADVERTENCIA: La etiqueta, la descripción y el tipo de contenido",
    ] {
        assert!(
            prov.contains(expected),
            "attribution not localized -- missing {expected:?} in:\n{prov}"
        );
    }

    for english in [
        "Generated by ",
        "Command line:",
        "Prompt file:",
        "Model:",
        "LLM API URL:",
        "Language:",
        "Timestamp:",
        "WARNING:",
    ] {
        assert!(
            !prov.contains(english),
            "English attribution label {english:?} survived in:\n{prov}"
        );
    }
}

#[test]
fn viz_smart_dict_page_shares_the_dashboard_language() {
    // The dictionary page is the SECOND `<html lang>` site and is assembled separately from
    // the dashboard. Both are rendered in one process off one process-global locale, and the
    // locale is only settled after the dictionary is read -- so this pins the ordering
    // invariant: nothing may assemble HTML before the dictionary's language is resolved.
    let wrk = Workdir::new("viz_smart_dict_page_shares_the_dashboard_language");
    wrk.create_from_string("rev.csv", localization_rows());
    wrk.create_from_string("dict.schema.json", spanish_dictionary());

    let out_html = wrk.path("dash.html").to_string_lossy().to_string();
    let mut cmd = wrk.command("viz");
    cmd.args([
        "smart",
        "rev.csv",
        "-o",
        &out_html,
        "--dict-info",
        "--dictionary",
    ])
    .arg(wrk.path("dict.schema.json"));
    wrk.assert_success(&mut cmd);

    let html = wrk.read_to_string("dash.html").unwrap();
    assert!(
        html.contains("qsv-dict-doc"),
        "--dict-info should embed the dictionary page (otherwise this test proves nothing)"
    );
    assert_eq!(
        html.matches(r#"<html lang="es">"#).count(),
        2,
        "both the dashboard and the embedded dictionary page should carry lang=es"
    );
    assert!(
        !html.contains(r#"<html lang="en">"#),
        "no page in a Spanish dashboard should still declare English"
    );
}

#[test]
fn viz_smart_language_flag_overrides_dictionary() {
    let wrk = Workdir::new("viz_smart_language_flag_overrides_dictionary");
    wrk.create_from_string("rev.csv", localization_rows());
    wrk.create_from_string("dict.schema.json", spanish_dictionary());

    let out_html = wrk.path("dash.html").to_string_lossy().to_string();
    let mut cmd = wrk.command("viz");
    cmd.args([
        "smart",
        "rev.csv",
        "-o",
        &out_html,
        "--language",
        "en",
        "--dictionary",
    ])
    .arg(wrk.path("dict.schema.json"));
    wrk.assert_success(&mut cmd);

    let html = wrk.read_to_string("dash.html").unwrap();
    assert!(
        html.contains(r#"<html lang="en">"#),
        "an explicit --language must win over the dictionary's detected language"
    );
    assert!(html.contains("\u{1F313} Theme"), "chrome should be English");
}

#[test]
fn viz_smart_language_flag_works_without_a_dictionary() {
    let wrk = Workdir::new("viz_smart_language_flag_works_without_a_dictionary");
    wrk.create_from_string("rev.csv", localization_rows());

    let out_html = wrk.path("dash.html").to_string_lossy().to_string();
    let mut cmd = wrk.command("viz");
    cmd.args(["smart", "rev.csv", "-o", &out_html, "--language", "es"]);
    wrk.assert_success(&mut cmd);

    let html = wrk.read_to_string("dash.html").unwrap();
    assert!(
        html.contains(r#"<html lang="es">"#) && html.contains("\u{1F313} Tema"),
        "--language should localize a stats-only dashboard, with no dictionary involved"
    );
}

#[test]
fn viz_smart_language_accepts_iso639_3_and_english_names() {
    // describegpt writes ISO 639-3 ("spa"); a human is far likelier to type "Spanish".
    for lang in ["spa", "Spanish", "es-MX"] {
        // Distinct workdir per iteration: a shared name would put all three in one temp
        // directory, which is benign only as long as the loop stays sequential.
        let wrk = Workdir::new(&format!("viz_smart_language_alias_{lang}"));
        wrk.create_from_string("rev.csv", localization_rows());

        let out_html = wrk.path("dash.html").to_string_lossy().to_string();
        let mut cmd = wrk.command("viz");
        cmd.args(["smart", "rev.csv", "-o", &out_html, "--language", lang]);
        wrk.assert_success(&mut cmd);

        let html = wrk.read_to_string("dash.html").unwrap();
        assert!(
            html.contains(r#"<html lang="es">"#),
            "--language {lang} should resolve to the es locale"
        );
    }
}

#[test]
fn viz_smart_unknown_language_is_a_usage_error() {
    let wrk = Workdir::new("viz_smart_unknown_language_is_a_usage_error");
    wrk.create_from_string("rev.csv", localization_rows());

    let out_html = wrk.path("dash.html").to_string_lossy().to_string();
    let mut cmd = wrk.command("viz");
    cmd.args(["smart", "rev.csv", "-o", &out_html, "--language", "klingon"]);

    // Silently anglicizing an explicit request would hide the typo until someone
    // opened the dashboard, so this fails fast and names the curated set.
    wrk.assert_err(&mut cmd);
    let stderr = wrk.output_stderr(&mut cmd);
    assert!(
        stderr.contains("Unknown --language 'klingon'") && stderr.contains("en, es"),
        "the error should name the bad value and list the curated languages, got: {stderr}"
    );
}

#[test]
fn viz_smart_uncurated_detected_language_falls_back_to_english() {
    let wrk = Workdir::new("viz_smart_uncurated_detected_language_falls_back_to_english");
    wrk.create_from_string("rev.csv", localization_rows());
    wrk.create_from_string(
        "dict.schema.json",
        &spanish_dictionary().replace("\"spa\"", "\"vie\""),
    );

    let out_html = wrk.path("dash.html").to_string_lossy().to_string();
    let mut cmd = wrk.command("viz");
    cmd.args(["smart", "rev.csv", "-o", &out_html, "--dictionary"])
        .arg(wrk.path("dict.schema.json"));
    // A language qsv has no translations for is not a user error -- the dashboard
    // still renders, in English, with a note.
    wrk.assert_success(&mut cmd);

    let html = wrk.read_to_string("dash.html").unwrap();
    assert!(
        html.contains(r#"<html lang="en">"#) && html.contains("\u{1F313} Theme"),
        "an uncurated detected language should render English chrome"
    );
}

#[test]
fn viz_smart_default_and_explicit_english_agree() {
    // NOTE: this does NOT prove English output is unchanged from before localization --
    // both paths resolve to the same locale, so identical bytes are near-tautological.
    // The real English-stability evidence is the rest of this file (every pre-existing viz
    // test still passing) plus scripts/viz-golden-check.sh. What this DOES catch is the
    // default path diverging from the explicit one, e.g. if "auto" ever stopped meaning
    // English in the absence of a detected language.
    let dict_without_language = r#"{
      "$schema": "https://json-schema.org/draft/2020-12/schema",
      "type": "object",
      "properties": {
        "region": { "type": "string", "title": "Region",
          "x-qsv": { "qsv_type": "String", "role": "dimension", "concept": "category.status" } },
        "revenue": { "type": "integer", "title": "Revenue",
          "x-qsv": { "qsv_type": "Integer", "role": "measure", "concept": "measure.amount" } }
      }
    }"#;

    let wrk = Workdir::new("viz_smart_dict_no_language");
    wrk.create_from_string("rev.csv", localization_rows());
    wrk.create_from_string("plain.schema.json", dict_without_language);
    wrk.create_from_string("english.schema.json", dict_without_language);

    let no_lang = wrk.path("no_lang.html").to_string_lossy().to_string();
    let mut cmd = wrk.command("viz");
    cmd.args(["smart", "rev.csv", "-o", &no_lang, "--dictionary"])
        .arg(wrk.path("plain.schema.json"));
    wrk.assert_success(&mut cmd);

    let explicit_en = wrk.path("explicit_en.html").to_string_lossy().to_string();
    let mut cmd = wrk.command("viz");
    cmd.args([
        "smart",
        "rev.csv",
        "-o",
        &explicit_en,
        "--language",
        "en",
        "--dictionary",
    ])
    .arg(wrk.path("english.schema.json"));
    wrk.assert_success(&mut cmd);

    // `viz smart` stamps a minute-granular "Compiled:" wall clock into the metadata table, so two
    // invocations that straddle a minute boundary differ by exactly those bytes. Comparing the
    // raw files would make this test fail roughly once per however-long-the-two-runs-take, for a
    // reason that has nothing to do with locale resolution — so normalize the stamp first and
    // keep the whole-output comparison, which is the actual point of the test.
    let strip_compiled = |html: String| {
        let mut out = String::with_capacity(html.len());
        for (i, chunk) in html.split("<td>").enumerate() {
            if i > 0 {
                out.push_str("<td>");
            }
            // "2026-07-30 00:07 UTC" — fixed shape, so a prefix check is enough to spot it.
            match chunk.find(" UTC</td>") {
                Some(end) if end == 16 => {
                    out.push_str("COMPILED_TIMESTAMP");
                    out.push_str(&chunk[end..]);
                },
                _ => out.push_str(chunk),
            }
        }
        out
    };

    let a = strip_compiled(wrk.read_to_string("no_lang.html").unwrap());
    let b = strip_compiled(wrk.read_to_string("explicit_en.html").unwrap());
    assert!(
        a.contains("COMPILED_TIMESTAMP"),
        "the Compiled: stamp should have been found and normalized; if the metadata format \
         changed, update this normalizer rather than dropping it"
    );
    assert_eq!(
        a, b,
        "defaulting to English and asking for English must produce identical output"
    );
}

#[test]
fn viz_smart_spanish_embeds_vendored_library_locales() {
    // End-to-end proof that the vendored DataTables i18n JSON and plotly locale bundle actually
    // reach the page — the unit tests cover the builders, this covers the wiring.
    let wrk = Workdir::new("viz_smart_spanish_embeds_vendored_library_locales");
    wrk.create_from_string("rev.csv", localization_rows());

    let out_html = wrk.path("dash.html").to_string_lossy().to_string();
    let mut cmd = wrk.command("viz");
    cmd.args(["smart", "rev.csv", "-o", &out_html, "--language", "es"]);
    wrk.assert_success(&mut cmd);

    let html = wrk.read_to_string("dash.html").unwrap();
    assert!(
        html.contains(r#"name:"es""#),
        "the vendored plotly locale bundle should be embedded"
    );
    assert!(
        html.contains(r#"setPlotConfig({locale: "es"})"#),
        "plotly needs setPlotConfig to actually select the registered locale"
    );
    assert!(
        html.contains("No se encontraron resultados"),
        "vendored DataTables strings should reach the drawer"
    );
    assert!(
        html.contains("Filtro avanzado (%d)"),
        "qsv's own SearchBuilder name should override the vendored wording, %d intact"
    );
}

#[test]
fn viz_smart_english_embeds_no_library_locale_assets() {
    // The byte-stability guard, stated as behavior: nothing locale-related may appear on an
    // English page. Both vendored injections are empty-for-English, and both once left a stray
    // separator behind when substituted empty.
    let wrk = Workdir::new("viz_smart_english_embeds_no_library_locale_assets");
    wrk.create_from_string("rev.csv", localization_rows());

    let out_html = wrk.path("dash.html").to_string_lossy().to_string();
    let mut cmd = wrk.command("viz");
    cmd.args(["smart", "rev.csv", "-o", &out_html]);
    wrk.assert_success(&mut cmd);

    let html = wrk.read_to_string("dash.html").unwrap();
    for marker in [
        r#"name:"es""#,
        r#"setPlotConfig({locale: "es"})"#,
        "No se encontraron resultados",
        "Filtro avanzado",
        "__QSVI18N",
    ] {
        assert!(
            !html.contains(marker),
            "an English dashboard must not contain '{marker}'"
        );
    }
    // The English DataTables block must survive exactly as authored (nothing spliced over it).
    assert!(
        html.contains(
            r#"searchBuilder: { button: { 0: "Advanced Filter", _: "Advanced Filter (%d)" } }"#
        ),
        "the English language block should be left byte-for-byte as written"
    );
}

#[test]
fn viz_smart_trend_quarterly_uses_quarter_bucket_and_category_axis() {
    // issue #4216: a 3-year quarterly dataset spans ~1000 days, so span-based selection alone
    // picked Week — ~150 buckets of which only 12 were non-empty, a comb of spikes. The cadence
    // floor must coarsen it to Quarter, rendered as "YYYY-Qn" labels on a category axis (the
    // labels are not date-parseable, and a date axis would re-introduce the empty gaps).
    let wrk = Workdir::new("viz_smart_trend_quarterly_uses_quarter_bucket_and_category_axis");
    let mut rows = String::from("filing_date,status\n");
    for year in 2021..=2023 {
        for (q, month) in [1u8, 4, 7, 10].iter().enumerate() {
            for k in 0..(q + 2) {
                let status = if k % 2 == 0 { "open" } else { "closed" };
                rows.push_str(&format!("{year}-{month:02}-01,{status}\n"));
            }
        }
    }
    wrk.create_from_string("s.csv", &rows);

    let out_html = wrk.path("dash.html").to_string_lossy().to_string();
    let mut cmd = wrk.command("viz");
    cmd.env("QSV_VIZ_NO_COMPRESS", "1");
    cmd.args(["smart", "s.csv", "-o", &out_html]);
    wrk.assert_success(&mut cmd);
    let html = wrk.read_to_string("dash.html").unwrap();

    // quarter-bucketed: the axis title says "per quarter" and the x values are YYYY-Qn
    assert!(
        html.contains("per quarter"),
        "quarterly cadence should coarsen the count trend to quarter buckets; html: {html}"
    );
    assert!(
        html.contains("2021-Q1") && html.contains("2023-Q4"),
        "quarter buckets should be labeled YYYY-Qn; html: {html}"
    );
    // category axis: the quarter labels ride in an explicit ticktext array (date axes have none)
    assert!(
        html.contains(r#""ticktext":["2021-Q1"#),
        "a quarter-bucketed trend must render on a category axis with quarter tick labels; html: \
         {html}"
    );
}

#[test]
fn viz_smart_trend_dictionary_cadence_overrides_detection() {
    // A dictionary carrying `x-qsv.cadence: "quarterly"` sets the trend bucket floor even where
    // the data's own spacing (monthly) would pick a finer bucket (issue #4216).
    let wrk = Workdir::new("viz_smart_trend_dictionary_cadence_overrides_detection");
    // 36 DISTINCT monthly values: a cardinality above CATEGORICAL_MAX_CARDINALITY (30) is what
    // keeps revenue a continuous trend-y candidate rather than a low-card categorical.
    let mut rows = String::from("report_date,revenue\n");
    let mut i = 0;
    for year in 2021..=2023 {
        for month in 1..=12 {
            i += 7;
            rows.push_str(&format!("{year}-{month:02}-01,{}\n", 1000 + i));
        }
    }
    wrk.create_from_string("rev.csv", &rows);
    wrk.create_from_string(
        "dict.schema.json",
        r#"{
          "$schema": "https://json-schema.org/draft/2020-12/schema",
          "type": "object",
          "x-qsv": { "cadence": "quarterly" },
          "properties": {
            "report_date": { "type": "string", "title": "Report Date",
              "x-qsv": { "qsv_type": "Date", "role": "timestamp", "concept": "time.event_timestamp" } },
            "revenue": { "type": "integer", "title": "Revenue",
              "x-qsv": { "qsv_type": "Integer", "role": "measure", "concept": "measure.amount" } }
          }
        }"#,
    );

    let out_html = wrk.path("dash.html").to_string_lossy().to_string();
    let mut cmd = wrk.command("viz");
    cmd.env("QSV_VIZ_NO_COMPRESS", "1");
    cmd.args(["smart", "rev.csv", "-o", &out_html, "--dictionary"])
        .arg(wrk.path("dict.schema.json"));
    wrk.assert_success(&mut cmd);
    let html = wrk.read_to_string("dash.html").unwrap();

    // an additive measure sums per period: "(sum/quarter)" proves both the AggValue mode and
    // the dictionary-imposed Quarter floor (the data alone reads as monthly)
    assert!(
        html.contains("sum/quarter"),
        "the dictionary cadence token must set the bucket floor; html: {html}"
    );
    assert!(
        html.contains("2021-Q1"),
        "quarter buckets should be labeled YYYY-Qn; html: {html}"
    );
}

#[test]
fn viz_smart_trend_bogus_dictionary_cadence_falls_back_to_detection() {
    // An out-of-vocab cadence token (LLM prose, typo, future token) is ignored — never an
    // error — and the trend falls back to detecting cadence from the data itself (monthly here).
    let wrk = Workdir::new("viz_smart_trend_bogus_dictionary_cadence_falls_back_to_detection");
    // 36 DISTINCT monthly values: see the sibling test for why cardinality must exceed 30.
    let mut rows = String::from("report_date,revenue\n");
    let mut i = 0;
    for year in 2021..=2023 {
        for month in 1..=12 {
            i += 7;
            rows.push_str(&format!("{year}-{month:02}-01,{}\n", 1000 + i));
        }
    }
    wrk.create_from_string("rev.csv", &rows);
    wrk.create_from_string(
        "dict.schema.json",
        r#"{
          "$schema": "https://json-schema.org/draft/2020-12/schema",
          "type": "object",
          "x-qsv": { "cadence": "every 3 months" },
          "properties": {
            "report_date": { "type": "string", "title": "Report Date",
              "x-qsv": { "qsv_type": "Date", "role": "timestamp", "concept": "time.event_timestamp" } },
            "revenue": { "type": "integer", "title": "Revenue",
              "x-qsv": { "qsv_type": "Integer", "role": "measure", "concept": "measure.amount" } }
          }
        }"#,
    );

    let out_html = wrk.path("dash.html").to_string_lossy().to_string();
    let mut cmd = wrk.command("viz");
    cmd.env("QSV_VIZ_NO_COMPRESS", "1");
    cmd.args(["smart", "rev.csv", "-o", &out_html, "--dictionary"])
        .arg(wrk.path("dict.schema.json"));
    wrk.assert_success(&mut cmd);
    let html = wrk.read_to_string("dash.html").unwrap();

    assert!(
        html.contains("sum/month"),
        "an unknown cadence token must fall back to scan-based detection (monthly); html: {html}"
    );
    assert!(
        !html.contains("2021-Q1"),
        "no quarter labels should appear when the bogus token is ignored; html: {html}"
    );
}

#[test]
fn viz_smart_trend_multidecade_coarsens_to_year() {
    // A >50-year span exceeds even the Quarter tier: the span ceiling lands on Year, labeled
    // as bare years on a category axis (issue #4216 widened the ladder that previously topped
    // out at Month — which would have been ~720 buckets here).
    let wrk = Workdir::new("viz_smart_trend_multidecade_coarsens_to_year");
    let mut rows = String::from("obs_date,event\n");
    for year in 1960..=2020 {
        for month in [3u8, 6, 9, 12] {
            rows.push_str(&format!("{year}-{month:02}-15,e\n"));
        }
    }
    wrk.create_from_string("s.csv", &rows);

    let out_html = wrk.path("dash.html").to_string_lossy().to_string();
    let mut cmd = wrk.command("viz");
    cmd.env("QSV_VIZ_NO_COMPRESS", "1");
    cmd.args(["smart", "s.csv", "-o", &out_html]);
    wrk.assert_success(&mut cmd);
    let html = wrk.read_to_string("dash.html").unwrap();

    assert!(
        html.contains("per year"),
        "a 60-year span must coarsen the count trend to yearly buckets; html: {html}"
    );
    assert!(
        html.contains(r#""ticktext":["1960","1961"#),
        "year buckets should render bare years on a category axis; html: {html}"
    );
}

// ---- --dict-info: "not charted" notes for the columns `viz smart` skipped ----
//
// `viz smart` has always explained its omissions on stderr; these pin that the same explanation
// reaches the ARTIFACT, in the Data Dictionary drawer, beside the column it is about. All of them
// pass `--dict-info` as well as `--dictionary`: `--dictionary` alone renders no drawer content at
// all, so the assertions would be vacuous.

/// Four columns skipped for four DIFFERENT recorded reasons, plus charted columns so the
/// Data Schematic is never empty:
///   `cust_ref`     dictionary role=identifier          -> DictionaryExcluded
///   `borough_code` redundant twin of `borough`         -> TwinOf("borough")
///   `notes`        high-cardinality free text, and DELIBERATELY absent from the dictionary
///   `when`         dictionary role=timestamp           -> RoutedTemporal
fn skip_notes_fixture(wrk: &Workdir) {
    let boroughs = ["Manhattan", "Brooklyn", "Queens", "Bronx", "Staten Island"];
    let mut rows = String::from("cust_ref,borough,borough_code,notes,amount,status,when\n");
    for i in 0..300 {
        let j = i % 5;
        let amount = (i * 7 % 900) + 1;
        let status = if i % 3 == 0 { "Closed" } else { "Open" };
        rows.push_str(&format!(
            "REF{i:06},{},{j},free text {i},{amount},{status},2024-0{}-15\n",
            boroughs[j],
            i % 9 + 1
        ));
    }
    wrk.create_from_string("d.csv", &rows);
    // `notes` is intentionally NOT described here - a skipped column with no dictionary entry
    // must still reach the drawer.
    wrk.create_from_string(
        "dict.json",
        r#"{"properties":{
            "cust_ref":     {"type":["string"],"description":"Customer reference.","x-qsv":{"role":"identifier","content_type":"identifier"}},
            "borough":      {"type":["string"],"description":"Borough of the request.","x-qsv":{"role":"dimension","content_type":"category"}},
            "borough_code": {"type":["integer"],"description":"Numeric borough code.","x-qsv":{"role":"dimension","content_type":"category"}},
            "amount":       {"type":["number"],"description":"Amount billed.","x-qsv":{"role":"measure"}},
            "status":       {"type":["string"],"description":"Open or closed.","x-qsv":{"role":"dimension","content_type":"category"}},
            "when":         {"type":["string"],"description":"Date of the request.","x-qsv":{"role":"timestamp","content_type":"date"}}
        }}"#,
    );
}

/// Return the `<section>` body for `col` in the embedded dictionary document.
fn dict_section<'h>(html: &'h str, col: &str) -> &'h str {
    let head = format!("<h2>{col}</h2>");
    let at = html
        .find(&head)
        .unwrap_or_else(|| panic!("no dictionary section for `{col}`"));
    // back up to this column's <section ...> and forward to its </section>
    let start = html[..at].rfind("<section class=\"qsv-dict-col\"").unwrap();
    let end = html[at..].find("</section>").unwrap() + at;
    &html[start..end]
}

#[test]
fn viz_smart_dict_info_explains_each_skipped_column() {
    let wrk = Workdir::new("viz_smart_dict_info_explains_each_skipped_column");
    skip_notes_fixture(&wrk);

    let mut cmd = wrk.command("viz");
    cmd.args(["smart", "d.csv", "--dict-info", "--dictionary"])
        .arg(wrk.path("dict.json"));
    let out = wrk.output(&mut cmd);
    assert!(out.status.success());
    let html = String::from_utf8_lossy(&out.stdout);

    // each skipped column carries ITS OWN reason, not a generic "skipped"
    assert!(
        dict_section(&html, "cust_ref").contains("identifier, PII or free text"),
        "cust_ref should report the dictionary exclusion"
    );
    assert!(
        dict_section(&html, "notes").contains("high-cardinality text"),
        "notes should report high-cardinality text"
    );
    // the twin note NAMES THE SURVIVOR - that is the whole value of threading the kept index
    // out of the twin detectors rather than keeping a bare set.
    let twin = dict_section(&html, "borough_code");
    assert!(
        twin.contains("duplicates borough"),
        "borough_code should name `borough` as the column charted instead, got: {twin}"
    );
    // `when` is this dataset's ONLY date column, so it is the canonical time axis and is named
    // as such. The converse case - a date column that drives nothing - is pinned separately by
    // `viz_smart_dict_info_tells_the_time_axis_from_an_unused_date`, because the two must not
    // render the same sentence.
    let when = dict_section(&html, "when");
    assert!(
        when.contains("x-axis of the time-based panels"),
        "the canonical date column should be named as the time axis, got: {when}"
    );

    // a column with no dictionary entry still gets a section, flagged as undescribed
    assert!(
        dict_section(&html, "notes").contains("qsv-dict-noentry"),
        "an undescribed skipped column should say it has no dictionary entry"
    );

    // ... and a CHARTED column gets no note at all. `borough` HAS a description, so this also
    // guards against regressing to inferring skipped-ness from `view_chart_anchors`.
    assert!(
        !dict_section(&html, "borough").contains("qsv-dict-notcharted"),
        "a charted column must carry no not-charted note"
    );

    // the stderr roll-up is unchanged - the drawer is an addition, not a replacement
    let stderr = String::from_utf8_lossy(&out.stderr);
    assert!(
        stderr.contains("viz smart: charting ") && stderr.contains("skipped 4:"),
        "the aggregate stderr note should be unchanged, got: {stderr}"
    );
}

// A panel dropped by `--max-charts` LOST A RANKING CONTEST; it was never "unchartable". The two
// claims must read differently.
//
// This also pins the `stat_idx` rejoin: every dropped panel's title here is DECORATED with a
// shape hint ("m01 (right-skewed)"), so a drawer keyed on the display title would find no
// matching dictionary entry and silently render no note at all.
#[test]
fn viz_smart_dict_info_capped_panel_says_capped_not_unchartable() {
    let wrk = Workdir::new("viz_smart_dict_info_capped_panel_says_capped_not_unchartable");
    let mut rows = String::from("m01,m02,m03,m04,keep\n");
    for r in 0..400 {
        for i in 0..4 {
            if r % 4 == 0 {
                rows.push(',');
            } else {
                // right-skewed, so the box panel title picks up a "(right-skewed)" hint
                rows.push_str(&format!("{}.5,", (r * (i + 3)) % 97 * (r % 11 + 1)));
            }
        }
        rows.push_str(match r % 3 {
            0 => "a\n",
            1 => "b\n",
            _ => "c\n",
        });
    }
    wrk.create_from_string("cap.csv", &rows);
    wrk.create_from_string(
        "dict.json",
        r#"{"properties":{
            "m01":  {"type":["number"],"description":"Measure one.","x-qsv":{"role":"measure"}},
            "m02":  {"type":["number"],"description":"Measure two.","x-qsv":{"role":"measure"}},
            "m03":  {"type":["number"],"description":"Measure three.","x-qsv":{"role":"measure"}},
            "m04":  {"type":["number"],"description":"Measure four.","x-qsv":{"role":"measure"}},
            "keep": {"type":["string"],"description":"A category.","x-qsv":{"role":"dimension","content_type":"category"}}
        }}"#,
    );

    let mut cmd = wrk.command("viz");
    cmd.args([
        "smart",
        "cap.csv",
        "--dict-info",
        "--max-charts",
        "2",
        "--dictionary",
    ])
    .arg(wrk.path("dict.json"));
    let out = wrk.output(&mut cmd);
    assert!(out.status.success());
    let html = String::from_utf8_lossy(&out.stdout);
    let stderr = String::from_utf8_lossy(&out.stderr);

    // the stderr roll-up names the DECORATED title ...
    assert!(
        stderr.contains("(right-skewed)"),
        "expected a decorated panel title in the roll-up, got: {stderr}"
    );
    // ... while the drawer rejoins by HEADER NAME and still finds the column.
    let m01 = dict_section(&html, "m01");
    assert!(
        m01.contains("--max-charts"),
        "a capped column should say the panel limit dropped it, got: {m01}"
    );
    assert!(
        !m01.contains("Not charted -"),
        "a capped column was chartable; it must not read as unchartable, got: {m01}"
    );
}

// Panel refusals are about a COMBINATION of columns, so they get their own dataset-level section
// rather than being hung off any single column's entry.
#[test]
fn viz_smart_dict_info_lists_panels_not_drawn() {
    let wrk = Workdir::new("viz_smart_dict_info_lists_panels_not_drawn");
    // two STATISTICALLY INDEPENDENT dimensions: the hierarchy panel is refused on Cramer's V,
    // and its notice embeds a literal `<` - which must survive as `&lt;`, since an unescaped one
    // could terminate the `<script type="text/html">` template the document is embedded in.
    let regions = ["North", "South", "East", "West", "Central"];
    let cats = ["Alpha", "Beta", "Gamma", "Delta"];
    let mut rows = String::from("region,category,amount\n");
    for i in 0..600 {
        rows.push_str(&format!(
            "{},{},{}\n",
            regions[i * 7 % 5],
            cats[i * 13 % 4],
            i % 500 + 1
        ));
    }
    wrk.create_from_string("ind.csv", &rows);
    wrk.create_from_string(
        "dict.json",
        r#"{"properties":{
            "region":   {"type":["string"],"description":"Region.","x-qsv":{"role":"dimension","content_type":"category"}},
            "category": {"type":["string"],"description":"Category.","x-qsv":{"role":"dimension","content_type":"category"}},
            "amount":   {"type":["number"],"description":"Amount.","x-qsv":{"role":"measure"}}
        }}"#,
    );

    let mut cmd = wrk.command("viz");
    cmd.args(["smart", "ind.csv", "--dict-info", "--dictionary"])
        .arg(wrk.path("dict.json"));
    let out = wrk.output(&mut cmd);
    assert!(out.status.success());
    let html = String::from_utf8_lossy(&out.stdout);
    let stderr = String::from_utf8_lossy(&out.stderr);

    assert!(
        stderr.contains("hierarchy panel"),
        "fixture should refuse the hierarchy panel, got: {stderr}"
    );
    assert!(
        html.contains("qsv-dict-omissions"),
        "the drawer should carry a `Panels not drawn` section"
    );
    let start = html.find("<section class=\"qsv-dict-omissions\">").unwrap();
    let end = html[start..].find("</section>").unwrap() + start;
    let omissions = &html[start..end];
    assert!(
        omissions.contains("hierarchy panel") && omissions.contains("statistically independent"),
        "the refusal reason should appear verbatim, got: {omissions}"
    );
    // escaped exactly once: the `<` of "Cramer's V=0.06 < 0.10" rides as `&lt;`, never raw.
    assert!(
        omissions.contains("&lt;"),
        "the `<` in the notice must be escaped, got: {omissions}"
    );
    assert!(
        !omissions.contains("V=0.06 < "),
        "no raw `<` may reach the embedded template, got: {omissions}"
    );
    // The markup must CLOSE cleanly. rustfmt's `format_strings` once split this block's format
    // literal across a `\`+newline continuation, which rewrote the trailing `\n` into a bare
    // literal `n` and shipped a stray character into the rendered drawer. Every earlier assertion
    // here still passed, because they all read the <li> text. Pin the tail explicitly.
    assert!(
        omissions.ends_with("</ul>\n"),
        "the omissions list must close cleanly - a stray character here means a mangled format \
         string literal, got tail: {:?}",
        &omissions[omissions.len().saturating_sub(40)..]
    );
}

// `RoutedTemporal` fires for EVERY date column, but only ONE becomes the x-axis of the
// time-based panels. Without distinguishing them, the dataset's canonical timestamp and a date
// column used nowhere render the IDENTICAL sentence, and a reader cannot tell "this drives the
// trend panel" from "this was dropped entirely".
#[test]
fn viz_smart_dict_info_tells_the_time_axis_from_an_unused_date() {
    let wrk = Workdir::new("viz_smart_dict_info_tells_the_time_axis_from_an_unused_date");
    let boroughs = ["Manhattan", "Brooklyn", "Queens", "Bronx", "Staten Island"];
    let mut rows = String::from("borough,amount,when,closed_on\n");
    for i in 0..300 {
        rows.push_str(&format!(
            "{},{},2024-0{}-15,2024-0{}-28\n",
            boroughs[i % 5],
            i * 7 % 900 + 1,
            i % 9 + 1,
            i % 9 + 1
        ));
    }
    wrk.create_from_string("two.csv", &rows);
    wrk.create_from_string(
        "dict.json",
        r#"{"properties":{
            "borough":   {"type":["string"],"description":"Borough.","x-qsv":{"role":"dimension","content_type":"category"}},
            "amount":    {"type":["number"],"description":"Amount billed.","x-qsv":{"role":"measure"}},
            "when":      {"type":["string"],"description":"Date opened.","x-qsv":{"role":"timestamp","content_type":"date"}},
            "closed_on": {"type":["string"],"description":"Date closed.","x-qsv":{"role":"timestamp","content_type":"date"}}
        }}"#,
    );

    let mut cmd = wrk.command("viz");
    cmd.args(["smart", "two.csv", "--dict-info", "--dictionary"])
        .arg(wrk.path("dict.json"));
    let out = wrk.output(&mut cmd);
    assert!(out.status.success());
    let html = String::from_utf8_lossy(&out.stdout);

    let axis = dict_section(&html, "when");
    let unused = dict_section(&html, "closed_on");
    assert!(
        axis.contains("x-axis of the time-based panels"),
        "the canonical date column should be named as the time axis, got: {axis}"
    );
    assert!(
        unused.contains("routes this as a timestamp")
            && !unused.contains("x-axis of the time-based panels"),
        "an unused date column must NOT be described as the time axis, got: {unused}"
    );
}

// Two twin detectors, one redundancy group, and they used to elect OPPOSITE survivors:
// `dimension_code_twins` keeps the human-readable `subject` (its whole purpose), while
// `one_to_one_categorical_twins` ranks by shortest values and so keeps the CODE. Merged blindly,
// BOTH columns were suppressed - and once the drawer gained "not charted" notes, each one claimed
// the other was "charted instead", which was false in both directions. (roborev 4193)
#[test]
fn viz_smart_reciprocal_twins_keep_exactly_one_survivor() {
    let wrk = Workdir::new("viz_smart_reciprocal_twins_keep_exactly_one_survivor");
    // `subject_code` must be STRING-typed to reach the 1:1 detector (it skips numerics), and its
    // values must be shorter than `subject`'s for the two detectors to disagree.
    let subjects = [
        "Mathematics",
        "Literature and Composition",
        "Physical Sciences",
        "Social Studies",
        "Fine Arts",
    ];
    let mut rows = String::from("subject,subject_code,amount,status\n");
    for i in 0..400 {
        let j = i % 5;
        rows.push_str(&format!(
            "{},S{j},{},{}\n",
            subjects[j],
            i * 7 % 900 + 1,
            if i % 3 == 0 { "Closed" } else { "Open" }
        ));
    }
    wrk.create_from_string("t.csv", &rows);
    wrk.create_from_string(
        "dict.json",
        r#"{"properties":{
            "subject":      {"type":["string"],"description":"Subject name.","x-qsv":{"role":"dimension","content_type":"category"}},
            "subject_code": {"type":["string"],"description":"Subject code.","x-qsv":{"role":"dimension","content_type":"category"}},
            "amount":       {"type":["number"],"description":"Amount.","x-qsv":{"role":"measure"}},
            "status":       {"type":["string"],"description":"Status.","x-qsv":{"role":"dimension","content_type":"category"}}
        }}"#,
    );

    let mut cmd = wrk.command("viz");
    cmd.args(["smart", "t.csv", "--dict-info", "--dictionary"])
        .arg(wrk.path("dict.json"));
    let out = wrk.output(&mut cmd);
    assert!(out.status.success());
    let html = String::from_utf8_lossy(&out.stdout);
    let stderr = String::from_utf8_lossy(&out.stderr);

    // EXACTLY ONE of the pair is suppressed, and it is the code - the name rule's survivor wins
    assert!(
        stderr.contains("skipped 1: subject_code"),
        "only the code twin should be suppressed, got: {stderr}"
    );
    // the stderr 1:1 note must agree with that verdict rather than contradict it
    assert!(
        stderr.contains("charting only subject for subject_code"),
        "the 1:1 note should keep the human-readable sibling, got: {stderr}"
    );
    // the drawer names a survivor that IS actually charted
    assert!(
        dict_section(&html, "subject_code").contains("duplicates subject, which is charted"),
        "the code twin should point at `subject`"
    );
    assert!(
        !dict_section(&html, "subject").contains("qsv-dict-notcharted"),
        "`subject` is charted, so it must carry no not-charted note"
    );
}

// The refusal notices now come from the catalog rather than a `format!` literal, so the drawer can
// render them in the dashboard's language while stderr stays English. Both halves matter: a
// Spanish dashboard should not carry an English paragraph, and stderr must keep the wording that
// scripts and the other tests in this file pin.
#[test]
fn viz_smart_dict_info_localizes_panels_not_drawn() {
    let wrk = Workdir::new("viz_smart_dict_info_localizes_panels_not_drawn");
    // two statistically independent dimensions -> the hierarchy panel is refused
    let regions = ["North", "South", "East", "West", "Central"];
    let cats = ["Alpha", "Beta", "Gamma", "Delta"];
    let mut rows = String::from("region,category,amount\n");
    for i in 0..600 {
        rows.push_str(&format!(
            "{},{},{}\n",
            regions[i * 7 % 5],
            cats[i * 13 % 4],
            i % 500 + 1
        ));
    }
    wrk.create_from_string("ind.csv", &rows);
    wrk.create_from_string(
        "dict.json",
        r#"{"properties":{
            "region":   {"type":["string"],"description":"Region.","x-qsv":{"role":"dimension","content_type":"category"}},
            "category": {"type":["string"],"description":"Category.","x-qsv":{"role":"dimension","content_type":"category"}},
            "amount":   {"type":["number"],"description":"Amount.","x-qsv":{"role":"measure"}}
        }}"#,
    );

    let mut cmd = wrk.command("viz");
    cmd.args([
        "smart",
        "ind.csv",
        "--dict-info",
        "--language",
        "es",
        "--dictionary",
    ])
    .arg(wrk.path("dict.json"));
    let out = wrk.output(&mut cmd);
    assert!(out.status.success());
    let html = String::from_utf8_lossy(&out.stdout);
    let stderr = String::from_utf8_lossy(&out.stderr);

    let start = html
        .find("<section class=\"qsv-dict-omissions\">")
        .expect("omissions section missing");
    let end = html[start..].find("</section>").unwrap() + start;
    let omissions = &html[start..end];

    // the refusal itself is Spanish ...
    assert!(
        omissions.contains("estad\u{ed}sticamente independientes"),
        "the refusal should be localized, got: {omissions}"
    );
    assert!(
        !omissions.contains("statistically independent"),
        "no English refusal text should remain in a Spanish drawer, got: {omissions}"
    );
    // ... and carries no CLI prefix, which is terminal furniture
    assert!(
        !omissions.contains("viz smart:"),
        "the drawer must not repeat the stderr CLI prefix, got: {omissions}"
    );

    // stderr is unchanged: English, prefixed, and word-for-word what it always was
    assert!(
        stderr.contains("viz smart: skipping the")
            && stderr.contains("its dimensions are statistically independent"),
        "stderr must stay English and prefixed, got: {stderr}"
    );
}

// The UI language is resolved in TWO stages -- `--language` in run(), then the dictionary's
// detected language in SmartCtx::new -- and the bivariate refusals fire in smart_prepare, which
// runs BEFORE the second stage. Rendering a refusal at emission time froze exactly those two in
// the pre-dictionary language while the rest of the page rendered in the detected one, which per
// the ordering invariant at the resolution site is an INVISIBLE failure: English inside a Spanish
// page reads as a missing translation, not a bug. (roborev 4195)
//
// No `--language` here on purpose -- the locale must come from the dictionary alone.
#[test]
fn viz_smart_dict_info_localizes_an_early_refusal_from_the_dictionary_language() {
    let wrk =
        Workdir::new("viz_smart_dict_info_localizes_an_early_refusal_from_the_dictionary_language");
    // > BIVARIATE_MAX_COLUMNS (50) so the cap refusal fires inside smart_prepare
    const NCOLS: usize = 55;
    let header: Vec<String> = (0..NCOLS).map(|c| format!("c{c:02}")).collect();
    let mut rows = header.join(",");
    rows.push('\n');
    for r in 0..200 {
        let cells: Vec<String> = (0..NCOLS)
            .map(|c| format!("v{}", (r + c) % (c % 5 + 2)))
            .collect();
        rows.push_str(&cells.join(","));
        rows.push('\n');
    }
    wrk.create_from_string("wide.csv", &rows);
    // a dictionary whose DETECTED language is Spanish, with no --language flag anywhere
    let props: Vec<String> = header
        .iter()
        .map(|h| format!(r#""{h}":{{"type":["string"],"description":"Columna {h}.","x-qsv":{{"role":"dimension","content_type":"category"}}}}"#))
        .collect();
    wrk.create_from_string(
        "dict.json",
        &format!(
            r#"{{"x-qsv":{{"detected_language_code":"spa"}},"properties":{{{}}}}}"#,
            props.join(",")
        ),
    );

    let mut cmd = wrk.command("viz");
    cmd.args([
        "smart",
        "wide.csv",
        "--dict-info",
        "--bivariate",
        "--dictionary",
    ])
    .arg(wrk.path("dict.json"));
    let out = wrk.output(&mut cmd);
    assert!(out.status.success());
    let html = String::from_utf8_lossy(&out.stdout);
    let stderr = String::from_utf8_lossy(&out.stderr);

    // the fixture must actually trip the early refusal, or this test proves nothing
    assert!(
        stderr.contains("columns exceeds the") && stderr.contains("-column cap"),
        "fixture should trip the bivariate column cap in smart_prepare, got: {stderr}"
    );
    // the page is Spanish (proving the dictionary language won) ...
    assert!(
        html.contains("Diccionario de datos"),
        "the dictionary language should drive the UI"
    );
    // ... and so is the refusal that fired BEFORE that language was known
    let start = html
        .find("<section class=\"qsv-dict-omissions\">")
        .expect("omissions section missing");
    let end = html[start..].find("</section>").unwrap() + start;
    let omissions = &html[start..end];
    assert!(
        omissions.contains("columnas superan el l\u{ed}mite"),
        "an early refusal must render in the dictionary-detected language, got: {omissions}"
    );
    assert!(
        !omissions.contains("columns exceeds the"),
        "no pre-dictionary English should survive into the drawer, got: {omissions}"
    );
    // Deferring the render moved argument substitution too, so pin that the ARGUMENTS actually
    // landed -- a template rendered with no arguments still contains all the Spanish prose above
    // and would sail past those assertions.
    assert!(
        omissions.contains(&NCOLS.to_string()),
        "the refusal should carry its interpolated column count, got: {omissions}"
    );
    assert!(
        !omissions.contains("%{"),
        "no unsubstituted argument token may reach the drawer, got: {omissions}"
    );

    // stderr is still English regardless of the page language
    assert!(
        stderr.contains("viz smart --bivariate:"),
        "stderr must stay English and prefixed, got: {stderr}"
    );
}

// A dictionary `x-qsv.currency` on a monetary measure prefixes its KPI tile with the currency
// symbol, and large values carry the "B" suffix English readers expect rather than d3's SI "G"
// (issue #4393). The whole page must agree — tiles, bar labels and axes — so this also pins that
// no `%{y:.3s}` template and no `exponentformat: "SI"` survive on an English page.
#[test]
fn viz_smart_money_currency_prefix_and_b_suffix() {
    let wrk = Workdir::new("viz_smart_money_currency_prefix_and_b_suffix");
    // values in the billions, repeating so the column is not stamped all-unique
    let mut csv = String::from("agency,spent\n");
    for i in 0..40 {
        csv.push_str(&format!(
            "A{},{}\n",
            i % 5,
            4_000_000_000_u64 + (i % 8) * 5e8 as u64
        ));
    }
    wrk.create_from_string("money.csv", &csv);
    wrk.create_from_string(
        "dict.json",
        r#"{"type":"object","properties":{"spent":{"title":"Total Spent","type":"number","x-qsv":{"role":"measure","concept":"measure.money","currency":"usd"}}}}"#,
    );

    let out_html = wrk.path("m.html").to_string_lossy().to_string();
    let mut cmd = wrk.command("viz");
    cmd.args([
        "smart",
        "money.csv",
        "--dictionary",
        "dict.json",
        "-o",
        &out_html,
    ]);
    wrk.assert_success(&mut cmd);
    let html = wrk.read_to_string("m.html").unwrap();

    // the KPI headline reads "$...B": symbol from the prefix, magnitude from the suffix. The
    // lowercase "usd" in the dictionary proves viz normalizes hand-edited sidecars itself.
    assert!(
        html.contains(r#""prefix":"$""#),
        "KPI tile must carry the currency symbol; html: {html}"
    );
    assert!(
        html.contains(r#""suffix":"B""#),
        "a billions-scale KPI must read B, not G; html: {html}"
    );
    // the currency is named once per panel, in the subtitle
    assert!(
        html.contains("(USD)"),
        "the panel subtitle must name the currency"
    );
    // axes agree with the labels...
    assert!(html.contains(r#""exponentformat":"B""#));
    // ...and nothing on the page still speaks SI
    assert!(
        !html.contains(r#""exponentformat":"SI""#),
        "an English page must not mix B tiles with SI axes"
    );
    assert!(
        !html.contains(r"%{y:.3s}"),
        "bar value labels must be qsv-rendered, not d3 SI templates"
    );
}

// The twin of the test above: every suffix decision is locale-gated off ONE helper, so a non-en
// page must flip ALL of them back to SI together. A half-converted page (B tiles, G axes) is the
// exact defect issue #4393 reports.
#[test]
fn viz_smart_money_non_english_keeps_si_suffix() {
    let wrk = Workdir::new("viz_smart_money_non_english_keeps_si_suffix");
    let mut csv = String::from("agency,spent\n");
    for i in 0..40 {
        csv.push_str(&format!(
            "A{},{}\n",
            i % 5,
            4_000_000_000_u64 + (i % 8) * 5e8 as u64
        ));
    }
    wrk.create_from_string("money.csv", &csv);
    wrk.create_from_string(
        "dict.json",
        r#"{"type":"object","properties":{"spent":{"title":"Total Spent","type":"number","x-qsv":{"role":"measure","concept":"measure.money","currency":"USD"}}}}"#,
    );

    let out_html = wrk.path("m.html").to_string_lossy().to_string();
    let mut cmd = wrk.command("viz");
    cmd.args([
        "smart",
        "money.csv",
        "--dictionary",
        "dict.json",
        "--language",
        "Spanish",
        "-o",
        &out_html,
    ]);
    wrk.assert_success(&mut cmd);
    let html = wrk.read_to_string("m.html").unwrap();

    // the currency symbol is not locale-gated -- only the magnitude suffix is
    assert!(html.contains(r#""prefix":"$""#));
    assert!(
        html.contains(r#""suffix":"G""#),
        "a non-English page keeps the SI prefix; html: {html}"
    );
    assert!(html.contains(r#""exponentformat":"SI""#));
    assert!(
        !html.contains(r#""exponentformat":"B""#),
        "no B may leak onto a non-English page"
    );
}

// Issue #4401: a per-unit money measure ("unit price") is INTENSIVE — summing it produces a
// number with no meaning — but a money noun alone never is. The two columns here are the
// regression pair: they were `Total Unit Price` (wrong) and `Total Shipping Cost` (right) side by
// side in the same committed gallery figure, both tagged `role: measure, concept: measure.amount`.
#[test]
fn viz_smart_per_unit_money_is_averaged_but_plain_money_still_sums() {
    let wrk = Workdir::new("viz_smart_per_unit_money_is_averaged");
    // Cardinalities are tuned deliberately: too LOW and a numeric column charts as a
    // categorical frequency bar, too HIGH (near-unique) and it is skipped as ID-like. Either way
    // it never reaches the KPI row, which is what this test reads.
    let mut csv = String::from("region,unit_price,shipping_cost,units_sold,total_price\n");
    for i in 0..200 {
        csv.push_str(&format!(
            "R{},{}.5,{}.25,{},{}.75\n",
            i % 4,
            5 + i % 89,
            1 + i % 37,
            1 + i * 7 % 97,
            50 + i % 149
        ));
    }
    wrk.create_from_string("sales.csv", &csv);

    let out_html = wrk.path("s.html").to_string_lossy().to_string();
    let mut cmd = wrk.command("viz");
    cmd.args(["smart", "sales.csv", "-o", &out_html]);
    wrk.assert_success(&mut cmd);
    let html = wrk.read_to_string("s.html").unwrap();

    assert!(
        html.contains(r#""text":"Mean unit_price""#),
        "a per-unit price must headline as a MEAN; html: {html}"
    );
    // the canaries: a naive `INTENSIVE_TOKENS += "price"|"cost"` would have flipped all three.
    for additive in ["shipping_cost", "units_sold", "total_price"] {
        assert!(
            html.contains(&format!(r#""text":"Total {additive}""#)),
            "{additive} is additive and must keep its Total tile; html: {html}"
        );
    }
}

// Issue #4401: an explicit `x-qsv.aggregation` is the authoritative, language-neutral signal and
// overrides the label heuristic in BOTH directions.
#[test]
fn viz_smart_explicit_aggregation_overrides_the_label_heuristic() {
    let wrk = Workdir::new("viz_smart_explicit_aggregation_overrides");
    let mut csv = String::from("region,unit_price,shipping_cost\n");
    for i in 0..60 {
        csv.push_str(&format!("R{},{},{}\n", i % 4, 10 + i % 9, 3 + i % 7));
    }
    wrk.create_from_string("sales.csv", &csv);
    // deliberately INVERTED against what the heuristic would say for each column
    wrk.create_from_string(
        "dict.json",
        r#"{"type":"object","properties":{
            "unit_price":{"title":"Unit Price","type":"number",
              "x-qsv":{"role":"measure","concept":"measure.money","aggregation":"sum"}},
            "shipping_cost":{"title":"Shipping Cost","type":"number",
              "x-qsv":{"role":"measure","concept":"measure.money","aggregation":"mean"}}}}"#,
    );

    let out_html = wrk.path("s.html").to_string_lossy().to_string();
    let mut cmd = wrk.command("viz");
    cmd.args([
        "smart",
        "sales.csv",
        "--dictionary",
        "dict.json",
        "-o",
        &out_html,
    ]);
    wrk.assert_success(&mut cmd);
    let html = wrk.read_to_string("s.html").unwrap();

    assert!(
        html.contains(r#""text":"Total Unit Price""#),
        "an explicit `sum` must override the intensive label heuristic; html: {html}"
    );
    assert!(
        html.contains(r#""text":"Mean Shipping Cost""#),
        "an explicit `mean` must override the additive default; html: {html}"
    );
}

// Issue #4401: an explicit `x-qsv.aggregation` of `sum` is documented to override the name
// heuristic in BOTH directions, but the pipeline funnel's stage validation re-ran
// `is_intensive_measure` unconditionally and refused any stage whose NAME looked intensive --
// even one the dictionary had explicitly declared additive. A lead-scoring funnel sums score
// points per stage, and "score" is an intensive token.
#[test]
fn viz_smart_funnel_stage_honors_an_explicit_sum_over_the_name_heuristic() {
    let wrk = Workdir::new("viz_smart_funnel_explicit_sum");
    let mut csv = String::from("raw_score,qualified_score,won_score\n");
    for i in 0..60 {
        let raw = 900 - i * 3;
        csv.push_str(&format!("{raw},{},{}\n", raw / 2, raw / 5));
    }
    let dict = r#"{
      "properties": {
        "raw_score":{"type":"integer","title":"Raw Score",
          "x-qsv":{"qsv_type":"Integer","role":"measure","concept":"measure.amount","aggregation":"sum"}},
        "qualified_score":{"type":"integer","title":"Qualified Score",
          "x-qsv":{"qsv_type":"Integer","role":"measure","concept":"measure.amount","aggregation":"sum"}},
        "won_score":{"type":"integer","title":"Won Score",
          "x-qsv":{"qsv_type":"Integer","role":"measure","concept":"measure.amount","aggregation":"sum"}}
      },
      "x-qsv": { "relationships": [{"kind":"pipeline",
        "members":["raw_score","qualified_score","won_score"]}] }
    }"#;
    let html = smart_with_dict(&wrk, &csv, dict);

    assert!(
        html.contains(r#""type":"funnel""#),
        "an explicitly-additive stage must not be refused as a rate; html: {html}"
    );
}

// ---------------------------------------------------------------------------
// issue #4394: denominator-aware region choropleths.
//
// A region map colored by raw row counts is largely a population map — the region with the most
// people tallies the most rows — so these cover both halves of the fix: the RATE map/panel when a
// denominator is available, and the caveat that says so when one is not.
// ---------------------------------------------------------------------------

/// Four regions whose populations differ by 20x, so a rate map ranks them differently from a count
/// map. Region D carries a zero population: it is the "no usable denominator" case.
fn denom_geojson() -> &'static str {
    r#"{"type":"FeatureCollection","features":[
{"type":"Feature","id":"A","properties":{"name":"Alpha","POP":"10000"},"geometry":{"type":"Polygon","coordinates":[[[0,0],[1,0],[1,1],[0,1],[0,0]]]}},
{"type":"Feature","id":"B","properties":{"name":"Beta","POP":200000},"geometry":{"type":"Polygon","coordinates":[[[1,0],[2,0],[2,1],[1,1],[1,0]]]}},
{"type":"Feature","id":"C","properties":{"name":"Gamma","POP":50000},"geometry":{"type":"Polygon","coordinates":[[[2,0],[3,0],[3,1],[2,1],[2,0]]]}},
{"type":"Feature","id":"D","properties":{"name":"Delta","POP":0},"geometry":{"type":"Polygon","coordinates":[[[3,0],[4,0],[4,1],[3,1],[3,0]]]}}
]}"#
}

/// `region,pop` rows: A=30 rows/10k people, B=300/200k, C=60/50k. B leads on raw count, A leads on
/// rate — the whole point of the issue.
fn denom_csv() -> String {
    let mut s = String::from("region,pop\n");
    for (r, pop, n) in [("A", 10000, 30), ("B", 200000, 300), ("C", 50000, 60)] {
        for _ in 0..n {
            s.push_str(&format!("{r},{pop}\n"));
        }
    }
    s
}

fn denom_dictionary(with_hint: bool) -> String {
    let hint = if with_hint {
        r#", "denominator": {"column": "pop"}"#
    } else {
        ""
    };
    format!(
        r#"{{"$schema":"https://json-schema.org/draft/2020-12/schema","type":"object",
 "x-qsv":{{"grain_unit":"service request"}},
 "properties":{{
   "region":{{"type":"string","title":"Region","x-qsv":{{"concept":"geo.zip_code","role":"dimension"{hint}}}}},
   "pop":{{"type":"integer","title":"Population","x-qsv":{{"concept":"measure.count","role":"measure","qsv_type":"Integer"}}}}
 }}}}"#
    )
}

#[test]
fn viz_choropleth_denominator_key_makes_a_rate_map() {
    let wrk = Workdir::new("viz_choropleth_denominator_key_makes_a_rate_map");
    wrk.create_from_string("rg.csv", &denom_csv());
    wrk.create_from_string("regions.geojson", denom_geojson());

    let mut cmd = wrk.command("viz");
    cmd.args([
        "choropleth",
        "rg.csv",
        "--locations",
        "region",
        "--geojson",
        "regions.geojson",
        "--location-mode",
        "geojson-id",
        "--denominator-key",
        "properties.POP",
    ]);
    let out = wrk.output(&mut cmd);
    assert!(out.status.success());
    let html = String::from_utf8_lossy(&out.stdout);

    // the colorbar names the rate, not the raw count
    assert!(
        html.contains("count per 1,000 residents"),
        "colorbar must state the rate: {html}"
    );
    // hover keeps the raw numerator AND the named denominator beside the rate
    assert!(html.contains("count: 30"), "raw numerator stays visible");
    assert!(html.contains("POP: 10,000"), "denominator is named");
    assert!(html.contains("3 per 1,000 residents"), "A's rate");
    assert!(html.contains("1.5 per 1,000 residents"), "B's rate");
    // a rate is intensive, so no share-of-total may appear on it
    assert!(
        !html.contains("% of total"),
        "a share-of-total on a rate would be a fabricated statistic: {html}"
    );
}

#[test]
fn viz_choropleth_denominator_column_and_agg_sum() {
    // the "total spend per resident" case: a denominator is valid with --agg sum, since a sum is
    // extensive just like a count.
    let wrk = Workdir::new("viz_choropleth_denominator_column_and_agg_sum");
    wrk.create_from_string(
        "rg.csv",
        "region,pop,spend\nA,10000,500\nA,10000,500\nB,200000,1000\nB,200000,1000\n",
    );
    wrk.create_from_string("regions.geojson", denom_geojson());

    let mut cmd = wrk.command("viz");
    cmd.args([
        "choropleth",
        "rg.csv",
        "--locations",
        "region",
        "--value",
        "spend",
        "--agg",
        "sum",
        "--geojson",
        "regions.geojson",
        "--location-mode",
        "geojson-id",
        "--denominator",
        "pop",
    ]);
    let out = wrk.output(&mut cmd);
    assert!(out.status.success());
    let html = String::from_utf8_lossy(&out.stdout);
    // A: 1,000 spend / 10,000 people = 0.1 -> per 1,000 = 100. B: 2,000 / 200,000 = 10 per 1,000.
    assert!(
        html.contains("spend per 1,000 residents"),
        "the summed measure keeps its own name in the rate label: {html}"
    );
    assert!(html.contains("100 per 1,000 residents"), "A's rate: {html}");
    assert!(html.contains("10 per 1,000 residents"), "B's rate: {html}");
}

#[test]
fn viz_choropleth_denominator_rejects_intensive_agg() {
    let wrk = Workdir::new("viz_choropleth_denominator_rejects_intensive_agg");
    wrk.create_from_string("rg.csv", &denom_csv());
    wrk.create_from_string("regions.geojson", denom_geojson());

    let mut cmd = wrk.command("viz");
    cmd.args([
        "choropleth",
        "rg.csv",
        "--locations",
        "region",
        "--value",
        "pop",
        "--agg",
        "mean",
        "--geojson",
        "regions.geojson",
        "--location-mode",
        "geojson-id",
        "--denominator",
        "pop",
    ]);
    wrk.assert_err(&mut cmd);
    let stderr = wrk.output_stderr(&mut cmd);
    assert!(
        stderr.contains("already"),
        "a mean is already intensive; dividing it again is meaningless: {stderr}"
    );
}

#[test]
fn viz_choropleth_denominator_must_be_region_constant() {
    // a denominator that changes row to row is a ROW-level amount passed by mistake; taking the
    // first value would produce a confident wrong rate, so this is a hard error.
    let wrk = Workdir::new("viz_choropleth_denominator_must_be_region_constant");
    wrk.create_from_string(
        "rg.csv",
        "region,pop\nA,10000\nA,99999\nB,200000\nB,200000\n",
    );
    wrk.create_from_string("regions.geojson", denom_geojson());

    let mut cmd = wrk.command("viz");
    cmd.args([
        "choropleth",
        "rg.csv",
        "--locations",
        "region",
        "--geojson",
        "regions.geojson",
        "--location-mode",
        "geojson-id",
        "--denominator",
        "pop",
    ]);
    wrk.assert_err(&mut cmd);
    let stderr = wrk.output_stderr(&mut cmd);
    assert!(
        stderr.contains("not constant within region 'A'"),
        "the offending region must be named: {stderr}"
    );
}

#[test]
fn viz_choropleth_denominator_flags_are_mutually_exclusive() {
    let wrk = Workdir::new("viz_choropleth_denominator_flags_are_mutually_exclusive");
    wrk.create_from_string("rg.csv", &denom_csv());
    wrk.create_from_string("regions.geojson", denom_geojson());

    let mut cmd = wrk.command("viz");
    cmd.args([
        "choropleth",
        "rg.csv",
        "--locations",
        "region",
        "--geojson",
        "regions.geojson",
        "--location-mode",
        "geojson-id",
        "--denominator",
        "pop",
        "--denominator-key",
        "properties.POP",
    ]);
    wrk.assert_err(&mut cmd);
    let stderr = wrk.output_stderr(&mut cmd);
    assert!(stderr.contains("mutually exclusive"), "{stderr}");
}

#[test]
fn viz_choropleth_denominator_key_must_resolve() {
    let wrk = Workdir::new("viz_choropleth_denominator_key_must_resolve");
    wrk.create_from_string("rg.csv", &denom_csv());
    wrk.create_from_string("regions.geojson", denom_geojson());

    let mut cmd = wrk.command("viz");
    cmd.args([
        "choropleth",
        "rg.csv",
        "--locations",
        "region",
        "--geojson",
        "regions.geojson",
        "--location-mode",
        "geojson-id",
        "--denominator-key",
        "properties.NOPE",
    ]);
    wrk.assert_err(&mut cmd);
    let stderr = wrk.output_stderr(&mut cmd);
    assert!(
        stderr.contains("resolves to no positive number"),
        "an unresolvable key is explicit intent gone wrong, not a silent fallback: {stderr}"
    );
}

#[test]
fn viz_smart_denominator_hint_adds_a_rate_panel() {
    let wrk = Workdir::new("viz_smart_denominator_hint_adds_a_rate_panel");
    wrk.create_from_string("rg.csv", &denom_csv());
    wrk.create_from_string("regions.geojson", denom_geojson());
    wrk.create_from_string("d.schema.json", &denom_dictionary(true));

    let mut cmd = wrk.command("viz");
    cmd.args([
        "smart",
        "rg.csv",
        "--geojson",
        "regions.geojson",
        "--dictionary",
        "d.schema.json",
    ]);
    let out = wrk.output(&mut cmd);
    assert!(out.status.success());
    let html = String::from_utf8_lossy(&out.stdout);

    // the rate panel sits BESIDE the count panel — both must be present
    assert!(html.contains("count by Region"), "count panel: {html}");
    assert!(
        html.contains("service request per 1,000 residents by Region"),
        "rate panel, titled with the dictionary's grain unit: {html}"
    );
    assert!(html.contains("Population: 10,000"), "denominator in hover");
    // a rate WAS charted, so the count panel uses the paired caveat, not the "add a flag" one
    assert!(html.contains("not adjusted for region size"), "{html}");
    assert!(!html.contains("add --denominator-key for a rate"), "{html}");
}

#[test]
fn viz_smart_denominator_key_flag_adds_a_rate_panel() {
    let wrk = Workdir::new("viz_smart_denominator_key_flag_adds_a_rate_panel");
    wrk.create_from_string("rg.csv", &denom_csv());
    wrk.create_from_string("regions.geojson", denom_geojson());
    // dictionary WITHOUT the hint: the flag alone must be enough
    wrk.create_from_string("d.schema.json", &denom_dictionary(false));

    let mut cmd = wrk.command("viz");
    cmd.args([
        "smart",
        "rg.csv",
        "--geojson",
        "regions.geojson",
        "--dictionary",
        "d.schema.json",
        "--denominator-key",
        "properties.POP",
    ]);
    let out = wrk.output(&mut cmd);
    assert!(out.status.success());
    let html = String::from_utf8_lossy(&out.stdout);
    assert!(
        html.contains("service request per 1,000 residents by Region"),
        "rate panel from the flag: {html}"
    );
    assert!(
        html.contains("POP: 10,000"),
        "denominator named from the key"
    );
}

#[test]
fn viz_smart_bad_denominator_hint_degrades_to_caveated_counts() {
    // a dictionary is a hand-editable sidecar, so a bad hint must never take the Data Schematic
    // down: it costs the rate panel and nothing else.
    let wrk = Workdir::new("viz_smart_bad_denominator_hint_degrades_to_caveated_counts");
    wrk.create_from_string("rg.csv", &denom_csv());
    wrk.create_from_string("regions.geojson", denom_geojson());
    wrk.create_from_string(
        "d.schema.json",
        &denom_dictionary(true).replace(r#""column": "pop""#, r#""column": "nosuchcolumn""#),
    );

    let mut cmd = wrk.command("viz");
    cmd.args([
        "smart",
        "rg.csv",
        "--geojson",
        "regions.geojson",
        "--dictionary",
        "d.schema.json",
    ]);
    let out = wrk.output(&mut cmd);
    assert!(out.status.success(), "a bad HINT is not a hard error");
    let stderr = wrk.output_stderr(&mut cmd);
    assert!(
        stderr.contains("nosuchcolumn") && stderr.contains("no such column"),
        "the reason must name the column: {stderr}"
    );
    let html = String::from_utf8_lossy(&out.stdout);
    assert!(!html.contains("per 1,000 residents"), "no rate panel");
    assert!(
        html.contains("add --denominator-key for a rate"),
        "the count panel must be caveated: {html}"
    );
}

#[test]
fn viz_smart_region_map_without_a_denominator_is_caveated() {
    let wrk = Workdir::new("viz_smart_region_map_without_a_denominator_is_caveated");
    wrk.create_from_string("rg.csv", &denom_csv());
    wrk.create_from_string("regions.geojson", denom_geojson());
    wrk.create_from_string("d.schema.json", &denom_dictionary(false));

    let mut cmd = wrk.command("viz");
    cmd.args([
        "smart",
        "rg.csv",
        "--geojson",
        "regions.geojson",
        "--dictionary",
        "d.schema.json",
    ]);
    let out = wrk.output(&mut cmd);
    assert!(out.status.success());
    let html = String::from_utf8_lossy(&out.stdout);
    assert!(
        html.contains("add --denominator-key for a rate"),
        "an unqualified count choropleth reads as a map of where the problem is, when it is \
         substantially a map of where the people are: {html}"
    );
}

#[test]
fn viz_smart_zero_denominator_region_is_excluded_and_reported() {
    // region D has POP 0 in the boundary file: it cannot divide, so it is dropped from the rate
    // and the narrowing is stated in the title AND on stderr, never silently.
    let wrk = Workdir::new("viz_smart_zero_denominator_region_is_excluded_and_reported");
    let mut csv = denom_csv();
    csv.push_str(&"D,0\n".repeat(5));
    wrk.create_from_string("rg.csv", &csv);
    wrk.create_from_string("regions.geojson", denom_geojson());
    wrk.create_from_string("d.schema.json", &denom_dictionary(false));

    let mut cmd = wrk.command("viz");
    cmd.args([
        "smart",
        "rg.csv",
        "--geojson",
        "regions.geojson",
        "--dictionary",
        "d.schema.json",
        "--denominator-key",
        "properties.POP",
    ]);
    let out = wrk.output(&mut cmd);
    assert!(out.status.success());
    let stderr = wrk.output_stderr(&mut cmd);
    assert!(
        stderr.contains("no usable denominator for 1 of 4 regions"),
        "the exclusion must be reported: {stderr}"
    );
    let html = String::from_utf8_lossy(&out.stdout);
    assert!(
        html.contains("1 of 4 without a denominator"),
        "a sub-panel carries no below-map note, so the title states it: {html}"
    );
}

#[test]
fn viz_smart_pip_region_map_rate_panel_and_caveat() {
    // the Boston/Pittsburgh shape: regions come from point-in-polygon binning, not a region
    // column, so only --denominator-key can feed the rate here.
    let wrk = Workdir::new("viz_smart_pip_region_map_rate_panel_and_caveat");
    // both coordinates must VARY: `viz smart` skips a constant column, and without a usable
    // lat/lon pair there is no map panel and so no point-in-polygon region panel at all.
    // District D (lon 3-4) has POP 0 in the boundary file and gets a healthy share of the points,
    // so it survives `viz smart`'s geographic-outlier filter and actually exercises the PIP
    // panel's OWN exclusion reporting — that title suffix and skip note are duplicated there, not
    // shared with the region-code path.
    let mut csv = String::from("lat,lon\n");
    for (lon_base, n) in [(0.0_f64, 30), (1.0, 300), (2.0, 60), (3.0, 90)] {
        for i in 0..n {
            let j = f64::from(i % 17);
            csv.push_str(&format!(
                "{:.4},{:.4}\n",
                0.1 + j * 0.05,
                lon_base + 0.1 + j * 0.05
            ));
        }
    }
    wrk.create_from_string("pts.csv", &csv);
    wrk.create_from_string("regions.geojson", denom_geojson());

    // with a denominator: a rate panel appears beside the count panel
    let mut cmd = wrk.command("viz");
    cmd.args([
        "smart",
        "pts.csv",
        "--geojson",
        "regions.geojson",
        "--denominator-key",
        "properties.POP",
    ]);
    let out = wrk.output(&mut cmd);
    assert!(out.status.success());
    let html = String::from_utf8_lossy(&out.stdout);
    assert!(
        html.contains("per 1,000 residents"),
        "PIP rate panel: {html}"
    );
    assert!(html.contains("not adjusted for region size"), "{html}");
    // District D binned points but has POP 0, so it is dropped from the rate and SAID so
    assert!(
        html.contains("1 of 4 without a denominator"),
        "the PIP rate panel must state its own narrowed coverage: {html}"
    );
    let stderr = wrk.output_stderr(&mut cmd);
    assert!(
        stderr.contains("no usable denominator for 1 of 4 regions"),
        "and report it on stderr: {stderr}"
    );

    // without one: the count panel says why it is only showing counts
    let mut cmd = wrk.command("viz");
    cmd.args(["smart", "pts.csv", "--geojson", "regions.geojson"]);
    let out = wrk.output(&mut cmd);
    assert!(out.status.success());
    let html = String::from_utf8_lossy(&out.stdout);
    assert!(!html.contains("per 1,000 residents"));
    assert!(
        html.contains("add --denominator-key for a rate"),
        "PIP count panel must be caveated too: {html}"
    );
}

#[test]
fn viz_choropleth_denominator_constancy_sees_non_positive_rows() {
    // REGRESSION (roborev 4230/4232): constancy used to be checked only AFTER filtering to
    // positive values, so a row-level column holding 0, 50, 50 for one region read as a perfectly
    // constant 50 and went on to divide the map — the exact confident-wrong-rate the check exists
    // to prevent. Every finite value must be compared, and the unusable ones dropped only after.
    let wrk = Workdir::new("viz_choropleth_denominator_constancy_sees_non_positive_rows");
    wrk.create_from_string("rg.csv", "region,fee\nA,0\nA,50\nA,50\nB,0\nB,120\nB,120\n");
    wrk.create_from_string("regions.geojson", denom_geojson());

    let mut cmd = wrk.command("viz");
    cmd.args([
        "choropleth",
        "rg.csv",
        "--locations",
        "region",
        "--geojson",
        "regions.geojson",
        "--location-mode",
        "geojson-id",
        "--denominator",
        "fee",
    ]);
    wrk.assert_err(&mut cmd);
    let stderr = wrk.output_stderr(&mut cmd);
    assert!(
        stderr.contains("not constant within region 'A' (0 then 50)"),
        "the zero row must take part in the comparison: {stderr}"
    );
}

#[test]
fn viz_choropleth_denominator_all_zero_region_is_excluded_not_a_conflict() {
    // the other side of the same line: a region whose denominator is CONSISTENTLY unusable is not
    // a conflict — it simply has no rate, and is reported as excluded. Without this, the fix above
    // would turn every genuinely-zero region into a hard error.
    let wrk = Workdir::new("viz_choropleth_denominator_all_zero_region_is_excluded_not_a_conflict");
    wrk.create_from_string(
        "rg.csv",
        "region,pop\nA,10000\nA,10000\nB,200000\nB,200000\nC,0\nC,0\n",
    );
    wrk.create_from_string("regions.geojson", denom_geojson());

    let mut cmd = wrk.command("viz");
    cmd.args([
        "choropleth",
        "rg.csv",
        "--locations",
        "region",
        "--geojson",
        "regions.geojson",
        "--location-mode",
        "geojson-id",
        "--denominator",
        "pop",
    ]);
    let out = wrk.output(&mut cmd);
    assert!(
        out.status.success(),
        "a consistently-zero region is not an error"
    );
    let html = String::from_utf8_lossy(&out.stdout);
    assert!(
        html.contains("no usable denominator for 1 of 3 regions"),
        "the excluded region must be reported: {html}"
    );
}

#[test]
fn viz_smart_non_constant_hint_denominator_degrades_to_caveated_counts() {
    // REGRESSION (roborev 4235): the constancy-before-positivity fix landed on BOTH the standalone
    // column path and this dictionary-hint path, but only the former was pinned by a test, so the
    // hint path could silently regress to filtering non-positives first. Region A holds 0 and 50,
    // which is not a region-level quantity at all.
    //
    // A bad HINT never fails the run (unlike the flag): it costs the rate panel, names the reason,
    // and the count panel falls back to its raw-count caveat.
    let wrk = Workdir::new("viz_smart_non_constant_hint_denominator_degrades_to_caveated_counts");
    let mut csv = String::from("region,pop\nA,0\n");
    csv.push_str(&"A,50\n".repeat(29));
    csv.push_str(&"B,200000\n".repeat(300));
    csv.push_str(&"C,50000\n".repeat(60));
    wrk.create_from_string("rg.csv", &csv);
    wrk.create_from_string("regions.geojson", denom_geojson());
    wrk.create_from_string("d.schema.json", &denom_dictionary(true));

    let mut cmd = wrk.command("viz");
    cmd.args([
        "smart",
        "rg.csv",
        "--geojson",
        "regions.geojson",
        "--dictionary",
        "d.schema.json",
    ]);
    let out = wrk.output(&mut cmd);
    assert!(out.status.success(), "a bad hint is not a hard error");
    let stderr = wrk.output_stderr(&mut cmd);
    assert!(
        stderr.contains("not constant within region 'A'"),
        "the zero row must take part in the comparison, and name the region: {stderr}"
    );
    let html = String::from_utf8_lossy(&out.stdout);
    assert!(
        !html.contains("per 1,000 residents"),
        "no rate panel: {html}"
    );
    assert!(
        html.contains("add --denominator-key for a rate"),
        "the count panel must fall back to its caveat: {html}"
    );
}

#[test]
fn viz_smart_consistently_zero_hint_denominator_is_an_exclusion() {
    // the other side of the line, on the hint path: a region whose denominator is CONSISTENTLY
    // unusable is not a conflict — the rate panel still renders for the regions that do have one,
    // and the narrowed coverage is reported. Without this, the constancy fix above would turn
    // every genuinely-zero region into a dropped panel.
    let wrk = Workdir::new("viz_smart_consistently_zero_hint_denominator_is_an_exclusion");
    let mut csv = String::from("region,pop\n");
    csv.push_str(&"A,10000\n".repeat(30));
    csv.push_str(&"B,200000\n".repeat(300));
    csv.push_str(&"C,0\n".repeat(60));
    wrk.create_from_string("rg.csv", &csv);
    wrk.create_from_string("regions.geojson", denom_geojson());
    wrk.create_from_string("d.schema.json", &denom_dictionary(true));

    let mut cmd = wrk.command("viz");
    cmd.args([
        "smart",
        "rg.csv",
        "--geojson",
        "regions.geojson",
        "--dictionary",
        "d.schema.json",
    ]);
    let out = wrk.output(&mut cmd);
    assert!(out.status.success());
    let stderr = wrk.output_stderr(&mut cmd);
    assert!(
        stderr.contains("no usable denominator for 1 of 3 regions"),
        "reported, not silently narrowed: {stderr}"
    );
    let html = String::from_utf8_lossy(&out.stdout);
    assert!(
        html.contains("per 1,000 residents"),
        "the rate panel still renders for the covered regions: {html}"
    );
    assert!(html.contains("1 of 3 without a denominator"), "{html}");
}
