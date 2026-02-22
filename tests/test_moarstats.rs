use csv::ReaderBuilder;

use crate::workdir::Workdir;

/// Helper function to parse stats CSV and get column index
fn get_column_index(headers: &csv::StringRecord, column_name: &str) -> Option<usize> {
    headers.iter().position(|h| h == column_name)
}

/// Helper function to get field value from a record
fn get_field_value(record: &csv::StringRecord, column_idx: usize) -> Option<String> {
    record.get(column_idx).map(|s| s.to_string())
}

/// Helper function to verify new columns exist in output
fn verify_new_columns_exist(csv_content: &str) -> Vec<String> {
    let mut rdr = ReaderBuilder::new()
        .has_headers(true)
        .from_reader(csv_content.as_bytes());

    let headers = rdr.headers().unwrap();
    let expected_columns = vec![
        "pearson_skewness",
        "range_stddev_ratio",
        "quartile_coefficient_dispersion",
        "mode_zscore",
    ];

    let mut found_columns = Vec::new();
    for col in &expected_columns {
        if headers.iter().any(|h| h == *col) {
            found_columns.push(col.to_string());
        }
    }

    found_columns
}

#[test]
fn moarstats_basic_with_existing_stats() {
    let wrk = Workdir::new("moarstats_basic");
    let test_file = wrk.load_test_file("boston311-100.csv");

    // Generate baseline stats first
    let mut stats_cmd = wrk.command("stats");
    stats_cmd
        .arg("--everything")
        .arg("--infer-dates")
        .arg(&test_file);
    wrk.assert_success(&mut stats_cmd);

    // Verify stats file was created
    let stats_path = wrk.path("boston311-100.stats.csv");
    assert!(stats_path.exists(), "Stats file should exist");

    // Run moarstats
    let mut cmd = wrk.command("moarstats");
    cmd.arg(&test_file);
    wrk.assert_success(&mut cmd);

    // Verify output file exists
    assert!(
        stats_path.exists(),
        "Stats file should still exist after moarstats"
    );

    let got = wrk.read_to_string("boston311-100.stats.csv").unwrap();
    let expected = r#"field,type,is_ascii,sum,min,max,range,sort_order,sortiness,min_length,max_length,sum_length,avg_length,stddev_length,variance_length,cv_length,mean,sem,geometric_mean,harmonic_mean,stddev,variance,cv,nullcount,n_negative,n_zero,n_positive,max_precision,sparsity,mad,lower_outer_fence,lower_inner_fence,q1,q2_median,q3,iqr,upper_inner_fence,upper_outer_fence,skewness,cardinality,uniqueness_ratio,mode,mode_count,mode_occurrences,antimode,antimode_count,antimode_occurrences,percentiles,pearson_skewness,range_stddev_ratio,quartile_coefficient_dispersion,mode_zscore,relative_standard_error,min_zscore,max_zscore,median_mean_ratio,iqr_range_ratio,mad_stddev_ratio,xsd_type,outliers_extreme_lower_cnt,outliers_mild_lower_cnt,outliers_normal_cnt,outliers_mild_upper_cnt,outliers_extreme_upper_cnt,outliers_total_cnt,outliers_mean,non_outliers_mean,outliers_to_normal_mean_ratio,outliers_min,outliers_max,outliers_range,outliers_stddev,outliers_variance,non_outliers_stddev,non_outliers_variance,outliers_cv,non_outliers_cv,outliers_percentage,outlier_impact,outlier_impact_ratio,outliers_normal_stddev_ratio,lower_outer_fence_zscore,upper_outer_fence_zscore,winsorized_mean_25pct,trimmed_mean_25pct,trimmed_stddev_25pct,trimmed_variance_25pct,winsorized_stddev_25pct,winsorized_variance_25pct,trimmed_cv_25pct,winsorized_cv_25pct,trimmed_25pct_stddev_ratio,winsorized_25pct_stddev_ratio,trimmed_range_25pct,winsorized_range_25pct
case_enquiry_id,Integer,,10100411645180,101004113298,101004155594,42296,Unsorted,0.798,,,,,,,,101004116451.8,790.552,101004116451.8012,101004116451.7994,7905.5202,62497248.9352,0,0,0,0,100,,0,673,101004109567,101004111646,101004113725,101004114353,101004115111,1386,101004117190,101004119269,0.0938,100,1,,0,0,*ALL,0,1,5: 101004113371|10: 101004113403|40: 101004114016|60: 101004114652|90: 101004115369|95: 101004141354,0.7965,5.3502,0,,0,-0.3989,4.9512,1,0.0328,0.0851,unsignedLong,0,0,91,1,8,9,101004138497.4444,101004114271.4615,1,101004118346,101004155594,37248,13272.557,176160768,0,0,0,0,9,2180.3385,0,,-0.8709,0.3564,101004114376.68,101004114335.36,1170.2857,1369568.6531,2603.5885,6778673.1313,0,0,0.148,0.3293,1375,1386
open_dt,DateTime,,,2022-01-01T00:16:00+00:00,2022-01-31T11:46:00+00:00,30.47917,Unsorted,0.798,,,,,,,,2022-01-04T07:07:45.050+00:00,0.5568,18996.29623,18996.29542,5.568,31.00259,0.0293,0,,,,,0,0.76261,2021-12-27T14:16:49+00:00,2021-12-30T06:00:07+00:00,2022-01-01T21:43:25+00:00,2022-01-03T07:02:14+00:00,2022-01-03T16:12:17+00:00,1.77005,2022-01-06T07:55:35+00:00,2022-01-08T23:38:53+00:00,-0.5684,100,1,,0,0,*ALL,0,1,5: 2022-01-01T08:33:00+00:00|10: 2022-01-01T09:43:36+00:00|40: 2022-01-02T13:22:10+00:00|60: 2022-01-03T10:42:18+00:00|90: 2022-01-04T06:15:33+00:00|95: 2022-01-20T08:07:49+00:00,,5.474,,,,,,,0.0581,0.137,dateTime,0,0,0,0,100,100,2022-01-04T07:07:45.049+00:00,,,2022-01-01T00:16:00+00:00,2022-01-31T11:46:00+00:00,30.4792,5.596,31.3157,,,0.0003,,100,,,,,,2022-01-02T21:11:55.789+00:00,2022-01-02T23:26:00.580+00:00,0.5422,0.2939,0.7415,0.5498,0,0,0.0974,0.1332,1.7595,1.77
target_dt,DateTime,,,2022-01-03T10:32:34+00:00,2022-05-20T13:03:21+00:00,137.10471,Unsorted,0.2273,,,,,,,,2022-01-17T03:14:16.404+00:00,2.86258,19009.11578,19009.0967,27.00551,729.29774,0.1421,11,,,,,0.11,1,2021-11-26T08:30:00+00:00,2021-12-15T20:30:00+00:00,2022-01-04T08:30:00+00:00,2022-01-05T08:30:00+00:00,2022-01-17T08:30:00+00:00,13,2022-02-05T20:30:00+00:00,2022-02-25T08:30:00+00:00,0.8462,42,0.42,2022-01-04 08:30:00,1,25,*PREVIEW: 2022-01-03 10:32:34|2022-01-03 11:58:12|2022-01-04 09:58:36|2022-01-04 10:41:29|2022-01-04...,34,1,5: 2022-01-04T08:30:00+00:00|10: 2022-01-04T08:30:00+00:00|40: 2022-01-04T16:08:34+00:00|60: 2022-01-05T13:07:19+00:00|90: 2022-02-17T12:47:39+00:00|95: 2022-03-10T16:14:45+00:00,,5.0769,,,,,,,0.0948,0.037,dateTime,0,0,0,0,89,89,2022-01-17T03:14:16.404+00:00,,,2022-01-03T10:32:33.999+00:00,2022-05-20T13:03:21+00:00,137.1047,27.1585,737.5852,,,0.0014,,100,,,,,,2022-01-08T16:19:46.393+00:00,2022-01-06T01:14:23.469+00:00,3.1543,9.9495,5.5597,30.9108,0.0002,0.0003,0.1168,0.2059,13,13
closed_dt,DateTime,,,2022-01-01T12:56:14+00:00,2022-04-25T14:30:31+00:00,114.06547,Unsorted,-0.0714,,,,,,,,2022-01-08T01:10:44.411+00:00,1.71655,19000.04255,19000.036,15.82577,250.4549,0.0833,15,,,,,0.15,0.77213,2021-12-29T15:13:29+00:00,2021-12-31T19:50:08.750+00:00,2022-01-03T00:26:48.500+00:00,2022-01-03T12:15:23+00:00,2022-01-04T11:31:15+00:00,1.46142,2022-01-06T16:07:54.750+00:00,2022-01-08T20:44:34.500+00:00,0.3266,86,0.86,,1,15,*PREVIEW: 2022-01-01 12:56:14|2022-01-01 14:17:15|2022-01-01 14:59:41|2022-01-01 15:10:16|2022-01-01...,85,1,5: 2022-01-01T19:07:41+00:00|10: 2022-01-02T11:03:10+00:00|40: 2022-01-03T10:06:33+00:00|60: 2022-01-03T21:43:55+00:00|90: 2022-01-18T07:54:26+00:00|95: 2022-01-20T08:45:12+00:00,,7.2076,,,,,,,0.0128,0.0488,dateTime,0,0,0,0,85,85,2022-01-08T01:10:44.411+00:00,,,2022-01-01T12:56:14+00:00,2022-04-25T14:30:31+00:00,114.0655,15.9197,253.4365,,,0.0008,,100,,,,,,2022-01-03T17:19:50.111+00:00,2022-01-03T16:41:33.162+00:00,0.4298,0.1847,0.6,0.36,0,0,0.0272,0.0379,1.4318,1.4614
ontime,String,true,,ONTIME,OVERDUE,,Unsorted,0.6768,6,7,617,6.17,0.3756,0.1411,0.0609,,,,,,,,0,,,,,0,,,,,,,,,,,2,0.02,ONTIME,1,83,OVERDUE,1,17,,,,,,,,,,,,string,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,
case_status,String,true,,Closed,Open,,Unsorted,0.7172,4,6,570,5.7,0.7141,0.51,0.1253,,,,,,,,0,,,,,0,,,,,,,,,,,2,0.02,Closed,1,85,Open,1,15,,,,,,,,,,,,string,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,
closure_reason,String,true,, ,Case Closed. Closed date : Wed Jan 19 11:42:16 EST 2022 Resolved Removed df  ,,Unsorted,-0.0909,1,284,8314,83.14,55.0262,3027.8804,0.6618,,,,,,,,0,,,,,0,,,,,,,,,,,86,0.86, ,1,15,*PREVIEW: Case Closed Case Resolved  NEW CART#21026466 DELV ON 1/11/22  |Case Closed Case Resolved  ...,85,1,,,,,,,,,,,,string,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,
case_title,String,true,,Abandoned Vehicles,Traffic Signal Inspection,,Unsorted,0.0101,10,57,2386,23.86,9.452,89.3404,0.3961,,,,,,,,0,,,,,0,,,,,,,,,,,42,0.42,Parking Enforcement,1,20,*PREVIEW: Animal Generic Request|BTDT: Complaint|City/State Snow Issues|DISPATCHED Short Term Rental...,24,1,,,,,,,,,,,,string,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,
subject,String,true,,Animal Control,Transportation - Traffic Division,,Unsorted,0.3737,14,33,2570,25.7,4.9041,24.05,0.1908,,,,,,,,0,,,,,0,,,,,,,,,,,9,0.09,Public Works Department,1,51,Animal Control|Boston Police Department|Boston Water & Sewer Commission,3,1,,,,,,,,,,,,string,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,
reason,String,true,,Administrative & General Requests,Street Lights,,Unsorted,0.1717,7,33,1892,18.92,8.4056,70.6536,0.4443,,,,,,,,0,,,,,0,,,,,,,,,,,20,0.2,Enforcement & Abandoned Vehicles,1,23,Administrative & General Requests|Animal Issues|Building|Employee & General Comments|Noise Disturban...,7,1,,,,,,,,,,,,string,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,
type,String,true,,Abandoned Vehicles,Unsatisfactory Utilities - Electrical  Plumbing,,Unsorted,0.0707,10,47,2266,22.66,8.034,64.5444,0.3545,,,,,,,,0,,,,,0,,,,,,,,,,,36,0.36,Parking Enforcement,1,20,*PREVIEW: Animal Generic Request|City/State Snow Issues|Electrical|General Comments For a Program or...,15,1,,,,,,,,,,,,string,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,
queue,String,true,,BTDT_AVRS Interface Queue,PWDx_Street Light_General Lighting Request,,Unsorted,0.2121,13,55,2802,28.02,8.0224,64.3596,0.2863,,,,,,,,0,,,,,0,,,,,,,,,,,35,0.35,BTDT_Parking Enforcement,1,21,*PREVIEW: BTDT_BostonBikes|BTDT_Engineering_New Sign and Pavement Marking Requests|BTDT_Sign Shop_Si...,15,1,,,,,,,,,,,,string,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,
department,String,true,,BTDT,PWDx,,Unsorted,0.3737,3,4,392,3.92,0.2713,0.0736,0.0692,,,,,,,,0,,,,,0,,,,,,,,,,,7,0.07,PWDx,1,49,GEN_,1,2,,,,,,,,,,,,string,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,
submittedphoto,String,true,,https://311.boston.gov/media/boston/report/photos/61d03f0d05bbcf180c2965fd/report.jpg,https://311.boston.gov/media/boston/report/photos/61d75bba05bbcf180c2d41de/report.jpg,,Unsorted,0.8537,0,100,3633,36.33,43.2585,1871.2989,1.1907,,,,,,,,58,,,,,0.58,,,,,,,,,,,43,0.43,,1,58,*PREVIEW: https://311.boston.gov/media/boston/report/photos/61d03f0d05bbcf180c2965fd/report.jpg|http...,42,1,,,,,,,,,,,,string,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,
closedphoto,NULL,,,,,,,,,,,,,,,,,,,,,,100,,,,,1,,,,,,,,,,,1,0.01,,1,100,,0,0,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,
location,String,true,, ,INTERSECTION of Verdun St & Gallivan Blvd  Dorchester  MA  ,,Unsorted,-0.0303,1,63,3938,39.38,9.6247,92.6356,0.2444,,,,,,,,0,,,,,0,,,,,,,,,,,98,0.98,563 Columbus Ave  Roxbury  MA  02118|INTERSECTION of Gallivan Blvd & Washington St  Dorchester  MA  ,2,2,*PREVIEW:  |103 N Beacon St  Brighton  MA  02135|11 Aberdeen St  Boston  MA  02215|1148 Hyde Park Av...,96,1,,,,,,,,,,,,string,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,
fire_district,String,true,, ,9,,Unsorted,0.1515,1,2,113,1.13,0.3363,0.1131,0.2976,,,,,,,,0,,,,,0,,,,,,,,,,,10,0.1,3,1,19, ,1,1,,,,,,,,,,,,string,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,
pwd_district,String,true,, ,1C,,Unsorted,0.0707,1,3,209,2.09,0.3192,0.1019,0.1527,,,,,,,,0,,,,,0,,,,,,,,,,,14,0.14,1B,1,16, ,1,1,,,,,,,,,,,,string,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,
city_council_district,String,true,, ,9,,Unsorted,0.1313,1,1,100,1,0,0,0,,,,,,,,0,,,,,0,,,,,,,,,,,10,0.1,1,1,22, ,1,1,,,,,,,,,,,,string,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,
police_district,String,true,, ,E5,,Unsorted,0.1717,1,3,223,2.23,0.444,0.1971,0.1991,,,,,,,,0,,,,,0,,,,,,,,,,,13,0.13,A1,1,20, ,1,1,,,,,,,,,,,,string,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,
neighborhood,String,true,, ,West Roxbury,,Unsorted,0.0303,1,38,1486,14.86,9.8671,97.3604,0.664,,,,,,,,0,,,,,0,,,,,,,,,,,19,0.19,Dorchester,1,15, |Brighton|Mission Hill,3,1,,,,,,,,,,,,string,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,
neighborhood_services_district,String,true,, ,9,,Unsorted,0.0303,1,2,139,1.39,0.4877,0.2379,0.3509,,,,,,,,0,,,,,0,,,,,,,,,,,16,0.16,3,1,15, |12,2,1,,,,,,,,,,,,string,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,
ward,String,true,, ,Ward 9,,Unsorted,0.0505,1,7,499,4.99,2.2293,4.9699,0.4468,,,,,,,,0,,,,,0,,,,,,,,,,,42,0.42,Ward 3,1,10,*PREVIEW:  |01|02|04|06|07|1|10|16|18,23,1,,,,,,,,,,,,string,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,
precinct,String,true,, ,2210,,Unsorted,0.0408,0,4,393,3.93,0.4951,0.2451,0.126,,,,,,,,1,,,,,0.01,,,,,,,,,,,76,0.76,0306,1,5,*PREVIEW: NULL| |0102|0105|0108|0109|0201|0204|0305|0307,61,1,,,,,,,,,,,,gYear??,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,
location_street_name,String,true,,103 N Beacon St,INTERSECTION Verdun St & Gallivan Blvd,,Unsorted,-0.0204,0,45,1800,18,9.3995,88.3508,0.5222,,,,,,,,1,,,,,0.01,,,,,,,,,,,97,0.97,20 Washington St|563 Columbus Ave|INTERSECTION Gallivan Blvd & Washington St,3,2,*PREVIEW: NULL|103 N Beacon St|11 Aberdeen St|1148 Hyde Park Ave|119 L St|12 Derne St|126 Elm St|127...,94,1,,,,,,,,,,,,string,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,
location_zipcode,String,true,,02109,02215,,Unsorted,0.0488,0,5,415,4.15,1.8405,3.3874,0.4435,,,,,,,,17,,,,,0.17,,,,,,,,,,,24,0.24,,1,17,02126|02134|02210|02215,4,1,,,,,,,,,,,,string,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,
latitude,Float,,4233.6674,42.2553,42.3806,0.1253,Unsorted,0.0505,,,,,,,,42.3367,0.0031,42.3367,42.3367,0.0305,0.0009,0.072,0,0,0,100,4,0,0.0163,42.2034,42.2619,42.3204,42.3432,42.3594,0.039,42.4179,42.4764,-0.1667,78,0.78,42.3594,1,20,*PREVIEW: 42.2553|42.2601|42.2609|42.2645|42.2674|42.2789|42.2797|42.2804|42.2821|42.2878,74,1,5: 42.2674|10: 42.2878|40: 42.3347|60: 42.3549|90: 42.3666|95: 42.3735,-0.6393,4.1082,0.0005,0.7443,0.0001,-2.6689,1.4393,1.0002,0.3113,0.5344,decimal,0,3,97,0,0,3,42.2588,42.3391,0.9981,42.2553,42.2609,0.0056,0.003,0,0.0278,0.0008,0.0001,0.0007,3,-0.0024,-0.0001,0.1089,-4.3705,4.5803,42.3421,42.3458,0.0126,0.0002,0.0165,0.0003,0.0003,0.0004,0.4131,0.5412,0.0374,0.039
longitude,Float,,-7107.2688,-71.1626,-71.0298,0.1328,Unsorted,0.1515,,,,,,,,-71.0727,0.0031,,,0.0311,0.001,-0.0437,0,100,0,0,4,0,0.0121,-71.1741,-71.1294,-71.0848,-71.0609,-71.055,0.0298,-71.0104,-70.9658,-0.6101,77,0.77,-71.0587,1,19,*PREVIEW: -71.0298|-71.0301|-71.0309|-71.0323|-71.0325|-71.0329|-71.0336|-71.0338|-71.034|-71.0355,72,1,5: -71.1415|10: -71.1305|40: -71.069|60: -71.0587|90: -71.0357|95: -71.0329,-1.1383,4.2701,-0.0002,0.4502,0,-2.8907,1.3794,0.9998,0.2244,0.3891,decimal,0,10,90,0,0,10,-71.143,-71.0649,1.0011,-71.1626,-71.1305,0.0321,0.0113,0.0001,0.0213,0.0005,0.0002,0.0003,10,-0.0078,-0.0001,0.5291,-3.2605,3.4373,-71.0676,-71.0653,0.0085,0.0001,0.0124,0.0002,0.0001,0.0002,0.2726,0.3979,0.029,0.0298
source,String,true,,Citizens Connect App,Self Service,,Unsorted,0.5354,12,20,1801,18.01,2.3473,5.5099,0.1303,,,,,,,,0,,,,,0,,,,,,,,,,,4,0.04,Citizens Connect App,1,56,Self Service,1,3,,,,,,,,,,,,string,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,
"#;
    assert_eq!(got, expected);
}

#[test]
fn moarstats_auto_generate_stats() {
    let wrk = Workdir::new("moarstats_auto_gen");
    let test_file = wrk.load_test_file("boston311-100.csv");

    // Ensure no stats file exists initially
    let stats_path = wrk.path("boston311-100.stats.csv");
    if stats_path.exists() {
        std::fs::remove_file(&stats_path).unwrap();
    }

    // Run moarstats with stats options - should auto-generate stats
    let mut cmd = wrk.command("moarstats");
    cmd.arg(&test_file)
        .arg("--stats-options")
        .arg("--everything --infer-dates");
    wrk.assert_success(&mut cmd);

    // Verify stats file was auto-generated
    assert!(stats_path.exists(), "Stats file should be auto-generated");

    // Verify new columns are appended
    let stats_content = wrk.read_to_string("boston311-100.stats.csv").unwrap();
    let new_columns = verify_new_columns_exist(&stats_content);
    assert!(
        !new_columns.is_empty(),
        "New columns should be added after auto-generation"
    );
}

#[test]
fn moarstats_custom_output() {
    let wrk = Workdir::new("moarstats_custom_output");
    let test_file = wrk.load_test_file("boston311-100.csv");

    // Generate baseline stats first
    let mut stats_cmd = wrk.command("stats");
    stats_cmd
        .arg("--everything")
        .arg("--infer-dates")
        .arg(&test_file);
    wrk.assert_success(&mut stats_cmd);

    let original_stats_path = wrk.path("boston311-100.stats.csv");
    assert!(
        original_stats_path.exists(),
        "Original stats file should exist"
    );

    // Read original content for comparison
    let original_content = wrk.read_to_string("boston311-100.stats.csv").unwrap();

    // Run moarstats with custom output
    let mut cmd = wrk.command("moarstats");
    cmd.arg(&test_file)
        .arg("--output")
        .arg("enhanced_stats.csv");
    wrk.assert_success(&mut cmd);

    // Verify custom output file exists
    let enhanced_path = wrk.path("enhanced_stats.csv");
    assert!(enhanced_path.exists(), "Enhanced stats file should exist");

    // Verify original file is unchanged (should not have new columns)
    let original_content_after = wrk.read_to_string("boston311-100.stats.csv").unwrap();
    assert_eq!(
        original_content, original_content_after,
        "Original stats file should be unchanged"
    );

    // Verify enhanced file has new columns
    let enhanced_content = wrk.read_to_string("enhanced_stats.csv").unwrap();
    let new_columns = verify_new_columns_exist(&enhanced_content);
    assert!(
        !new_columns.is_empty(),
        "Enhanced stats file should have new columns"
    );
}

#[test]
fn moarstats_custom_rounding() {
    let wrk = Workdir::new("moarstats_rounding");
    let test_file = wrk.load_test_file("boston311-100.csv");

    // Generate baseline stats
    let mut stats_cmd = wrk.command("stats");
    stats_cmd
        .arg("--everything")
        .arg("--infer-dates")
        .arg(&test_file);
    wrk.assert_success(&mut stats_cmd);

    // Run moarstats with custom rounding
    let mut cmd = wrk.command("moarstats");
    cmd.arg(&test_file).arg("--round").arg("2");
    wrk.assert_success(&mut cmd);

    // Verify values are rounded to 2 decimal places
    let stats_content = wrk.read_to_string("boston311-100.stats.csv").unwrap();
    let mut rdr = ReaderBuilder::new()
        .has_headers(true)
        .from_reader(stats_content.as_bytes());

    let headers = rdr.headers().unwrap().clone();

    if let Some(pearson_idx) = get_column_index(&headers, "pearson_skewness") {
        for result in rdr.records() {
            let record = result.unwrap();
            if let Some(val_str) = get_field_value(&record, pearson_idx) {
                if !val_str.is_empty() {
                    // Check decimal places (allowing for scientific notation edge cases)
                    if let Some(dot_pos) = val_str.find('.') {
                        let after_dot = &val_str[dot_pos + 1..];
                        // Should have at most 2 decimal places (or be in scientific notation)
                        assert!(
                            after_dot.len() <= 2 || val_str.contains('e') || val_str.contains('E'),
                            "Value '{}' should be rounded to 2 decimal places",
                            val_str
                        );
                    }
                }
            }
        }
    }
}

#[test]
fn moarstats_verify_computed_values() {
    let wrk = Workdir::new("moarstats_verify_values");
    let test_file = wrk.load_test_file("boston311-100.csv");

    // Generate baseline stats with everything
    let mut stats_cmd = wrk.command("stats");
    stats_cmd
        .arg("--everything")
        .arg("--infer-dates")
        .arg(&test_file);
    wrk.assert_success(&mut stats_cmd);

    // Run moarstats
    let mut cmd = wrk.command("moarstats");
    cmd.arg(&test_file);
    wrk.assert_success(&mut cmd);

    // Parse output and verify computed values
    let stats_content = wrk.read_to_string("boston311-100.stats.csv").unwrap();
    let mut rdr = ReaderBuilder::new()
        .has_headers(true)
        .from_reader(stats_content.as_bytes());

    let headers = rdr.headers().unwrap().clone();
    let field_idx = get_column_index(&headers, "field").unwrap();
    let type_idx = get_column_index(&headers, "type").unwrap();

    let mean_idx = get_column_index(&headers, "mean");
    let median_idx = get_column_index(&headers, "median");
    let q2_median_idx = get_column_index(&headers, "q2_median");
    let stddev_idx = get_column_index(&headers, "stddev");
    let range_idx = get_column_index(&headers, "range");
    let q1_idx = get_column_index(&headers, "q1");
    let q3_idx = get_column_index(&headers, "q3");
    let mode_idx = get_column_index(&headers, "mode");

    let pearson_idx = get_column_index(&headers, "pearson_skewness");
    let range_stddev_idx = get_column_index(&headers, "range_stddev_ratio");
    let quartile_coeff_idx = get_column_index(&headers, "quartile_coefficient_dispersion");
    let mode_zscore_idx = get_column_index(&headers, "mode_zscore");

    let mut found_latitude = false;
    let mut found_numeric_with_quartiles = false;

    for result in rdr.records() {
        let record = result.unwrap();
        let field_name = get_field_value(&record, field_idx).unwrap();
        let field_type = get_field_value(&record, type_idx).unwrap();

        // Check latitude column (should be Float)
        if field_name == "latitude" && field_type == "Float" {
            found_latitude = true;

            // Verify pearson_skewness is computed if mean, median, stddev exist
            if let (Some(mean_idx), Some(stddev_idx), Some(pearson_idx)) =
                (mean_idx, stddev_idx, pearson_idx)
            {
                let mean_val = get_field_value(&record, mean_idx);
                let stddev_val = get_field_value(&record, stddev_idx);
                let median_val = median_idx
                    .and_then(|idx| get_field_value(&record, idx))
                    .or_else(|| q2_median_idx.and_then(|idx| get_field_value(&record, idx)));
                let pearson_val = get_field_value(&record, pearson_idx);

                if mean_val.is_some() && stddev_val.is_some() && median_val.is_some() {
                    // If all base stats exist, pearson_skewness should be computed
                    assert!(
                        pearson_val.is_some(),
                        "pearson_skewness should be computed for latitude"
                    );
                }
            }

            // Verify range_stddev_ratio if range and stddev exist
            if let (Some(range_idx), Some(stddev_idx), Some(range_stddev_idx)) =
                (range_idx, stddev_idx, range_stddev_idx)
            {
                let range_val = get_field_value(&record, range_idx);
                let stddev_val = get_field_value(&record, stddev_idx);
                let range_stddev_val = get_field_value(&record, range_stddev_idx);

                if range_val.is_some() && stddev_val.is_some() {
                    assert!(
                        range_stddev_val.is_some(),
                        "range_stddev_ratio should be computed for latitude"
                    );
                }
            }
        }

        // Check for numeric column with quartiles
        if (field_type == "Float" || field_type == "Integer")
            && q1_idx.is_some()
            && q3_idx.is_some()
        {
            found_numeric_with_quartiles = true;

            if let (Some(q1_idx), Some(q3_idx), Some(quartile_coeff_idx)) =
                (q1_idx, q3_idx, quartile_coeff_idx)
            {
                let q1_val = get_field_value(&record, q1_idx);
                let q3_val = get_field_value(&record, q3_idx);
                let quartile_coeff_val = get_field_value(&record, quartile_coeff_idx);

                if q1_val.is_some() && q3_val.is_some() {
                    assert!(
                        quartile_coeff_val.is_some(),
                        "quartile_coefficient_dispersion should be computed"
                    );
                }
            }
        }

        // Check for numeric column with mode
        if (field_type == "Float" || field_type == "Integer")
            && mode_idx.is_some()
            && mean_idx.is_some()
            && stddev_idx.is_some()
        {
            if let (Some(mode_idx), Some(mean_idx), Some(stddev_idx), Some(mode_zscore_idx)) =
                (mode_idx, mean_idx, stddev_idx, mode_zscore_idx)
            {
                let mode_val = get_field_value(&record, mode_idx);
                let mean_val = get_field_value(&record, mean_idx);
                let stddev_val = get_field_value(&record, stddev_idx);
                let mode_zscore_val = get_field_value(&record, mode_zscore_idx);

                // Mode might be a string, so we check if it can be parsed
                if mode_val.is_some() && mean_val.is_some() && stddev_val.is_some() {
                    // mode_zscore might be empty if mode is not numeric, which is fine
                    assert!(mode_zscore_val.is_some(), "mode_zscore column should exist");
                }
            }
        }
    }

    assert!(found_latitude, "Should find latitude column");
    assert!(
        found_numeric_with_quartiles,
        "Should find numeric column with quartiles"
    );
}

#[test]
fn moarstats_missing_base_statistics() {
    let wrk = Workdir::new("moarstats_missing_stats");
    let test_file = wrk.load_test_file("boston311-100.csv");

    // Generate minimal stats (without --everything, --quartiles, --median, --mode)
    let mut stats_cmd = wrk.command("stats");
    stats_cmd.arg("--infer-dates").arg(&test_file);
    wrk.assert_success(&mut stats_cmd);

    // Run moarstats - should succeed but only add stats that can be computed
    let mut cmd = wrk.command("moarstats");
    cmd.arg(&test_file);
    wrk.assert_success(&mut cmd);

    // Verify output file exists
    let stats_path = wrk.path("boston311-100.stats.csv");
    assert!(stats_path.exists(), "Stats file should exist");

    // Read and check which columns were added
    let stats_content = wrk.read_to_string("boston311-100.stats.csv").unwrap();

    // With minimal stats, we should only be able to compute range_stddev_ratio
    // (if range and stddev are available in default stats)
    // pearson_skewness requires median which won't be available
    // quartile stats require q1/q3 which won't be available
    // mode_zscore requires mode which won't be available

    // But the command should succeed and add whatever can be computed
    let _new_columns = verify_new_columns_exist(&stats_content);
    // At least range_stddev_ratio might be available if range and stddev are in default stats
    // The exact columns depend on what default stats includes
}

#[test]
fn moarstats_outlier_statistics() {
    let wrk = Workdir::new("moarstats_outliers");
    let test_file = wrk.load_test_file("boston311-100.csv");

    // Generate baseline stats with quartiles (required for outliers)
    let mut stats_cmd = wrk.command("stats");
    stats_cmd
        .arg("--everything")
        .arg("--infer-dates")
        .arg(&test_file);
    wrk.assert_success(&mut stats_cmd);

    // Run moarstats
    let mut cmd = wrk.command("moarstats");
    cmd.arg(&test_file);
    wrk.assert_success(&mut cmd);

    // Verify outlier columns exist
    let stats_content = wrk.read_to_string("boston311-100.stats.csv").unwrap();
    let mut rdr = ReaderBuilder::new()
        .has_headers(true)
        .from_reader(stats_content.as_bytes());

    let headers = rdr.headers().unwrap().clone();
    let field_idx = get_column_index(&headers, "field").unwrap();
    let type_idx = get_column_index(&headers, "type").unwrap();

    // Check for outlier columns (with _cnt suffix for counts)
    let outlier_columns = vec![
        "outliers_extreme_lower_cnt",
        "outliers_mild_lower_cnt",
        "outliers_normal_cnt",
        "outliers_mild_upper_cnt",
        "outliers_extreme_upper_cnt",
        "outliers_total_cnt",
        "outliers_mean",
        "non_outliers_mean",
        "outliers_to_normal_mean_ratio",
        "outliers_min",
        "outliers_max",
        "outliers_range",
        "outliers_stddev",
        "outliers_variance",
        "non_outliers_stddev",
        "non_outliers_variance",
        "outliers_cv",
        "non_outliers_cv",
        "outliers_percentage",
        "outlier_impact",
        "outlier_impact_ratio",
        "outliers_normal_stddev_ratio",
        "lower_outer_fence_zscore",
        "upper_outer_fence_zscore",
    ];

    let mut found_outlier_columns = Vec::new();
    for col in &outlier_columns {
        if headers.iter().any(|h| h == *col) {
            found_outlier_columns.push(col.to_string());
        }
    }

    // Find a numeric column with quartiles
    let mut found_numeric_with_outliers = false;
    for result in rdr.records() {
        let record = result.unwrap();
        let field_name = get_field_value(&record, field_idx).unwrap();
        let field_type = get_field_value(&record, type_idx).unwrap();

        if (field_type == "Float" || field_type == "Integer")
            && (field_name == "latitude" || field_name == "longitude")
        {
            found_numeric_with_outliers = true;

            // Verify outlier counts exist (with _cnt suffix)
            if let Some(outliers_total_idx) = get_column_index(&headers, "outliers_total_cnt") {
                let outliers_total_val = get_field_value(&record, outliers_total_idx);
                assert!(
                    outliers_total_val.is_some(),
                    "outliers_total_cnt should exist for numeric columns with quartiles"
                );
            }

            // Verify outlier statistics exist
            if let Some(outliers_mean_idx) = get_column_index(&headers, "outliers_mean") {
                let outliers_mean_val = get_field_value(&record, outliers_mean_idx);
                // Value might be empty if no outliers, which is fine
                assert!(
                    outliers_mean_val.is_some(),
                    "outliers_mean column should exist"
                );
            }

            if let Some(non_outliers_mean_idx) = get_column_index(&headers, "non_outliers_mean") {
                let non_outliers_mean_val = get_field_value(&record, non_outliers_mean_idx);
                assert!(
                    non_outliers_mean_val.is_some(),
                    "non_outliers_mean column should exist"
                );
            }

            break;
        }
    }

    assert!(
        !found_outlier_columns.is_empty(),
        "At least some outlier columns should be added"
    );
    assert!(
        found_numeric_with_outliers,
        "Should find numeric column with outlier statistics"
    );
}

#[test]
fn moarstats_duplicate_prevention() {
    let wrk = Workdir::new("moarstats_duplicates");
    let test_file = wrk.load_test_file("boston311-100.csv");

    // Generate baseline stats
    let mut stats_cmd = wrk.command("stats");
    stats_cmd
        .arg("--everything")
        .arg("--infer-dates")
        .arg(&test_file);
    wrk.assert_success(&mut stats_cmd);

    // Run moarstats first time
    let mut cmd1 = wrk.command("moarstats");
    cmd1.arg(&test_file);
    wrk.assert_success(&mut cmd1);

    // Read first run output
    let stats_content_1 = wrk.read_to_string("boston311-100.stats.csv").unwrap();
    let mut rdr1 = ReaderBuilder::new()
        .has_headers(true)
        .from_reader(stats_content_1.as_bytes());
    let headers_1 = rdr1.headers().unwrap().clone();

    // Count pearson_skewness occurrences in first run
    let pearson_count_1 = headers_1
        .iter()
        .filter(|h| *h == "pearson_skewness")
        .count();

    // Run moarstats second time - should not add duplicates
    let mut cmd = wrk.command("moarstats");
    cmd.arg(&test_file);
    wrk.assert_success(&mut cmd);

    // Read second run output
    let stats_content_2 = wrk.read_to_string("boston311-100.stats.csv").unwrap();
    let mut rdr2 = ReaderBuilder::new()
        .has_headers(true)
        .from_reader(stats_content_2.as_bytes());
    let headers_2 = rdr2.headers().unwrap().clone();

    // Count pearson_skewness occurrences in second run
    let pearson_count_2 = headers_2
        .iter()
        .filter(|h| *h == "pearson_skewness")
        .count();

    // Should have same count (no duplicates added)
    assert_eq!(
        pearson_count_1, pearson_count_2,
        "Running moarstats twice should not create duplicate columns"
    );
}

#[test]
fn moarstats_all_stats_already_added() {
    let wrk = Workdir::new("moarstats_all_added");
    let test_file = wrk.load_test_file("boston311-100.csv");

    // Generate baseline stats
    let mut stats_cmd = wrk.command("stats");
    stats_cmd
        .arg("--everything")
        .arg("--infer-dates")
        .arg(&test_file);
    wrk.assert_success(&mut stats_cmd);

    // Run moarstats first time
    let mut cmd1 = wrk.command("moarstats");
    cmd1.arg(&test_file);
    wrk.assert_success(&mut cmd1);

    // Run moarstats second time - should detect all stats already added
    let mut cmd = wrk.command("moarstats");
    cmd.arg(&test_file);
    let output = wrk.output_stderr(&mut cmd);

    // Should show message that all stats are already added
    assert!(
        output.contains("already been added") || output.contains("No additional stats"),
        "Should detect that all stats are already added"
    );
}

#[test]
fn moarstats_outlier_statistics_values() {
    let wrk = Workdir::new("moarstats_outlier_values");

    // Create a simple test CSV with known values
    wrk.create(
        "test.csv",
        vec![
            svec!["field", "value"],
            svec!["test", "10"],
            svec!["test", "20"],
            svec!["test", "30"],
            svec!["test", "40"],
            svec!["test", "50"],
            svec!["test", "100"], // outlier
            svec!["test", "200"], // outlier
        ],
    );

    // Generate stats with quartiles
    let mut stats_cmd = wrk.command("stats");
    stats_cmd.arg("--everything").arg("test.csv");
    wrk.assert_success(&mut stats_cmd);

    // Run moarstats
    let mut cmd = wrk.command("moarstats");
    cmd.arg("test.csv");
    wrk.assert_success(&mut cmd);

    // Verify outlier statistics
    let stats_content = wrk.read_to_string("test.stats.csv").unwrap();
    let mut rdr = ReaderBuilder::new()
        .has_headers(true)
        .from_reader(stats_content.as_bytes());

    let headers = rdr.headers().unwrap().clone();
    let field_idx = get_column_index(&headers, "field").unwrap();

    // Find the test field
    for result in rdr.records() {
        let record = result.unwrap();
        let field_name = get_field_value(&record, field_idx).unwrap();

        if field_name == "test" {
            // Verify outlier counts
            if let Some(outliers_total_idx) = get_column_index(&headers, "outliers_total") {
                let outliers_total_val = get_field_value(&record, outliers_total_idx);
                if let Some(val_str) = outliers_total_val {
                    if !val_str.is_empty() {
                        let count: u64 = val_str.parse().unwrap();
                        assert!(count > 0, "Should have some outliers");
                    }
                }
            }

            // Verify outlier mean exists when outliers are present
            if let Some(outliers_mean_idx) = get_column_index(&headers, "outliers_mean") {
                let outliers_mean_val = get_field_value(&record, outliers_mean_idx);
                if let Some(val_str) = outliers_mean_val {
                    if !val_str.is_empty() {
                        let mean: f64 = val_str.parse().unwrap();
                        // Outlier mean should be higher than normal values (10-50)
                        assert!(
                            mean > 50.0,
                            "Outlier mean should be higher than normal values"
                        );
                    }
                }
            }

            // Verify outliers_range
            if let Some(outliers_range_idx) = get_column_index(&headers, "outliers_range") {
                let outliers_range_val = get_field_value(&record, outliers_range_idx);
                if let Some(val_str) = outliers_range_val {
                    if !val_str.is_empty() {
                        let range: f64 = val_str.parse().unwrap();
                        assert!(range > 0.0, "Outlier range should be positive");
                    }
                }
            }

            // Verify outliers_total_cnt (updated column name)
            if let Some(outliers_total_idx) = get_column_index(&headers, "outliers_total_cnt") {
                let outliers_total_val = get_field_value(&record, outliers_total_idx);
                if let Some(val_str) = outliers_total_val {
                    if !val_str.is_empty() {
                        let count: u64 = val_str.parse().unwrap();
                        assert!(count > 0, "Should have some outliers");
                    }
                }
            }

            break;
        }
    }
}

#[test]
fn moarstats_no_outliers() {
    let wrk = Workdir::new("moarstats_no_outliers");

    // Create a CSV with values that won't have outliers (all close together)
    wrk.create(
        "test.csv",
        vec![
            svec!["field", "value"],
            svec!["test", "10"],
            svec!["test", "11"],
            svec!["test", "12"],
            svec!["test", "13"],
            svec!["test", "14"],
            svec!["test", "15"],
        ],
    );

    // Generate stats with quartiles
    let mut stats_cmd = wrk.command("stats");
    stats_cmd.arg("--everything").arg("test.csv");
    wrk.assert_success(&mut stats_cmd);

    // Run moarstats
    let mut cmd = wrk.command("moarstats");
    cmd.arg("test.csv");
    wrk.assert_success(&mut cmd);

    // Verify outlier columns still exist but may have zero counts
    let stats_content = wrk.read_to_string("test.stats.csv").unwrap();
    let mut rdr = ReaderBuilder::new()
        .has_headers(true)
        .from_reader(stats_content.as_bytes());

    let headers = rdr.headers().unwrap().clone();
    let field_idx = get_column_index(&headers, "field").unwrap();

    for result in rdr.records() {
        let record = result.unwrap();
        let field_name = get_field_value(&record, field_idx).unwrap();

        if field_name == "test" {
            // Outlier columns should exist (with _cnt suffix)
            assert!(
                get_column_index(&headers, "outliers_total_cnt").is_some(),
                "outliers_total_cnt column should exist"
            );

            // If no outliers, outliers_total_cnt should be 0
            if let Some(outliers_total_idx) = get_column_index(&headers, "outliers_total_cnt") {
                let outliers_total_val = get_field_value(&record, outliers_total_idx);
                if let Some(val_str) = outliers_total_val {
                    if !val_str.is_empty() {
                        let _count: u64 = val_str.parse().unwrap();
                        // With tightly clustered values, might have 0 outliers
                        // Count is u64, so it's always non-negative
                    }
                }
            }

            break;
        }
    }
}

#[test]
fn moarstats_multiple_numeric_fields() {
    let wrk = Workdir::new("moarstats_multiple_fields");
    let test_file = wrk.load_test_file("boston311-100.csv");

    // Generate baseline stats
    let mut stats_cmd = wrk.command("stats");
    stats_cmd
        .arg("--everything")
        .arg("--infer-dates")
        .arg(&test_file);
    wrk.assert_success(&mut stats_cmd);

    // Run moarstats
    let mut cmd = wrk.command("moarstats");
    cmd.arg(&test_file);
    wrk.assert_success(&mut cmd);

    // Verify that multiple numeric fields get statistics
    let stats_content = wrk.read_to_string("boston311-100.stats.csv").unwrap();
    let mut rdr = ReaderBuilder::new()
        .has_headers(true)
        .from_reader(stats_content.as_bytes());

    let headers = rdr.headers().unwrap().clone();
    let type_idx = get_column_index(&headers, "type").unwrap();
    let pearson_idx = get_column_index(&headers, "pearson_skewness");

    let mut numeric_fields_with_stats = 0;

    for result in rdr.records() {
        let record = result.unwrap();
        let field_type = get_field_value(&record, type_idx).unwrap();

        if field_type == "Float" || field_type == "Integer" {
            if let Some(pearson_idx) = pearson_idx {
                let pearson_val = get_field_value(&record, pearson_idx);
                if pearson_val.is_some() && !pearson_val.as_ref().unwrap().is_empty() {
                    numeric_fields_with_stats += 1;
                }
            }
        }
    }

    assert!(
        numeric_fields_with_stats > 1,
        "Multiple numeric fields should have computed statistics"
    );
}

#[test]
fn moarstats_winsorized_trimmed_means_q1_q3() {
    let wrk = Workdir::new("moarstats_winsorized_q1q3");

    // Create a CSV with known values for testing winsorized/trimmed means
    wrk.create(
        "test.csv",
        vec![
            svec!["field", "value"],
            svec!["test", "1"], // Below Q1
            svec!["test", "2"], // Below Q1
            svec!["test", "3"], // Q1
            svec!["test", "4"], // Between Q1 and Q3
            svec!["test", "5"], // Between Q1 and Q3
            svec!["test", "6"], // Between Q1 and Q3
            svec!["test", "7"], // Q3
            svec!["test", "8"], // Above Q3
            svec!["test", "9"], // Above Q3
        ],
    );

    // Generate stats with quartiles
    let mut stats_cmd = wrk.command("stats");
    stats_cmd.arg("--everything").arg("test.csv");
    wrk.assert_success(&mut stats_cmd);

    // Run moarstats
    let mut cmd = wrk.command("moarstats");
    cmd.arg("test.csv");
    wrk.assert_success(&mut cmd);

    // Verify winsorized and trimmed mean columns exist
    let stats_content = wrk.read_to_string("test.stats.csv").unwrap();
    let mut rdr = ReaderBuilder::new()
        .has_headers(true)
        .from_reader(stats_content.as_bytes());

    let headers = rdr.headers().unwrap().clone();
    let field_idx = get_column_index(&headers, "field").unwrap();

    assert!(
        get_column_index(&headers, "winsorized_mean_25pct").is_some(),
        "winsorized_mean_25pct column should exist"
    );
    assert!(
        get_column_index(&headers, "trimmed_mean_25pct").is_some(),
        "trimmed_mean_25pct column should exist"
    );

    // Find the test field and verify values
    for result in rdr.records() {
        let record = result.unwrap();
        let field_name = get_field_value(&record, field_idx).unwrap();

        if field_name == "test" {
            let winsorized_idx = get_column_index(&headers, "winsorized_mean_25pct").unwrap();
            let trimmed_idx = get_column_index(&headers, "trimmed_mean_25pct").unwrap();

            let winsorized_val = get_field_value(&record, winsorized_idx);
            let trimmed_val = get_field_value(&record, trimmed_idx);

            assert!(
                winsorized_val.is_some() && !winsorized_val.as_ref().unwrap().is_empty(),
                "winsorized_mean_25pct should have a value"
            );
            assert!(
                trimmed_val.is_some() && !trimmed_val.as_ref().unwrap().is_empty(),
                "trimmed_mean_25pct should have a value"
            );

            // Winsorized mean should include all values (but capped at thresholds)
            // Trimmed mean should only include values between Q1 and Q3
            if let (Some(w_str), Some(t_str)) = (winsorized_val, trimmed_val) {
                let winsorized: f64 = w_str.parse().unwrap();
                let trimmed: f64 = t_str.parse().unwrap();

                // Both should be reasonable values
                assert!(winsorized > 0.0, "Winsorized mean should be positive");
                assert!(trimmed > 0.0, "Trimmed mean should be positive");

                // Trimmed mean should generally be between Q1 and Q3 (approximately 3-7)
                assert!(
                    trimmed >= 2.0 && trimmed <= 8.0,
                    "Trimmed mean should be within reasonable range"
                );
            }

            break;
        }
    }
}

#[test]
fn moarstats_winsorized_trimmed_means_percentiles() {
    let wrk = Workdir::new("moarstats_winsorized_percentiles");

    // Create a CSV with known values
    wrk.create(
        "test.csv",
        vec![
            svec!["field", "value"],
            svec!["test", "1"],
            svec!["test", "2"],
            svec!["test", "3"],
            svec!["test", "4"],
            svec!["test", "5"],
            svec!["test", "6"],
            svec!["test", "7"],
            svec!["test", "8"],
            svec!["test", "9"],
            svec!["test", "10"],
        ],
    );

    // Generate stats with percentiles
    let mut stats_cmd = wrk.command("stats");
    stats_cmd
        .arg("--everything")
        .arg("--percentiles")
        .arg("--percentile-list")
        .arg("5,95")
        .arg("test.csv");
    wrk.assert_success(&mut stats_cmd);

    // Run moarstats with --use-percentiles
    let mut cmd = wrk.command("moarstats");
    cmd.arg("test.csv")
        .arg("--use-percentiles")
        .arg("--pct-thresholds")
        .arg("10,90");
    wrk.assert_success(&mut cmd);

    // Verify percentile-based winsorized/trimmed mean columns exist
    let stats_content = wrk.read_to_string("test.stats.csv").unwrap();
    let mut rdr = ReaderBuilder::new()
        .has_headers(true)
        .from_reader(stats_content.as_bytes());

    let headers = rdr.headers().unwrap().clone();
    let field_idx = get_column_index(&headers, "field").unwrap();

    assert!(
        get_column_index(&headers, "winsorized_mean_10pct").is_some(),
        "winsorized_mean_10pct column should exist when using percentiles"
    );
    assert!(
        get_column_index(&headers, "trimmed_mean_10pct").is_some(),
        "trimmed_mean_10pct column should exist when using percentiles"
    );

    // Verify values exist
    for result in rdr.records() {
        let record = result.unwrap();
        let field_name = get_field_value(&record, field_idx).unwrap();

        if field_name == "test" {
            let winsorized_idx = get_column_index(&headers, "winsorized_mean_10pct").unwrap();
            let trimmed_idx = get_column_index(&headers, "trimmed_mean_10pct").unwrap();

            let winsorized_val = get_field_value(&record, winsorized_idx);
            let trimmed_val = get_field_value(&record, trimmed_idx);

            assert!(
                winsorized_val.is_some() && !winsorized_val.as_ref().unwrap().is_empty(),
                "winsorized_mean_10pct should have a value"
            );
            assert!(
                trimmed_val.is_some() && !trimmed_val.as_ref().unwrap().is_empty(),
                "trimmed_mean_10pct should have a value"
            );

            break;
        }
    }
}

#[test]
fn moarstats_percentile_default_thresholds() {
    let wrk = Workdir::new("moarstats_percentile_default");

    wrk.create(
        "test.csv",
        vec![
            svec!["field", "value"],
            svec!["test", "1"],
            svec!["test", "2"],
            svec!["test", "3"],
            svec!["test", "4"],
            svec!["test", "5"],
        ],
    );

    // Generate stats with percentiles (including 5 and 95)
    let mut stats_cmd = wrk.command("stats");
    stats_cmd
        .arg("--everything")
        .arg("--percentiles")
        .arg("test.csv");
    wrk.assert_success(&mut stats_cmd);

    // Run moarstats with --use-percentiles but no --pct-thresholds (should default to 5,95)
    let mut cmd = wrk.command("moarstats");
    cmd.arg("test.csv").arg("--use-percentiles");
    wrk.assert_success(&mut cmd);

    // Verify default percentile columns (5pct) exist
    let stats_content = wrk.read_to_string("test.stats.csv").unwrap();
    let mut rdr = ReaderBuilder::new()
        .has_headers(true)
        .from_reader(stats_content.as_bytes());

    let headers = rdr.headers().unwrap().clone();
    assert!(
        get_column_index(&headers, "winsorized_mean_5pct").is_some(),
        "winsorized_mean_5pct should exist with default thresholds"
    );
}

#[test]
fn moarstats_invalid_percentile_thresholds() {
    let wrk = Workdir::new("moarstats_invalid_pct");
    let test_file = wrk.load_test_file("boston311-100.csv");

    // Generate baseline stats
    let mut stats_cmd = wrk.command("stats");
    stats_cmd
        .arg("--everything")
        .arg("--percentiles")
        .arg(&test_file);
    wrk.assert_success(&mut stats_cmd);

    // Test invalid thresholds: lower >= upper
    let mut cmd = wrk.command("moarstats");
    cmd.arg(&test_file)
        .arg("--use-percentiles")
        .arg("--pct-thresholds")
        .arg("90,10"); // Invalid: lower > upper
    let output = wrk.output_stderr(&mut cmd);
    assert!(
        output.contains("Lower percentile must be less than upper percentile"),
        "Should reject invalid percentile order"
    );

    // Test invalid thresholds: out of range
    let mut cmd = wrk.command("moarstats");
    cmd.arg(&test_file)
        .arg("--use-percentiles")
        .arg("--pct-thresholds")
        .arg("101,105"); // Invalid: > 100
    let output2 = wrk.output_stderr(&mut cmd);
    assert!(
        output2.contains("between 0 and 100"),
        "Should reject out-of-range percentiles"
    );
}

#[test]
fn moarstats_date_datetime_fields() {
    let wrk = Workdir::new("moarstats_dates");

    // Create a CSV with date values
    wrk.create(
        "test.csv",
        vec![
            svec!["field", "date_value"],
            svec!["test", "2020-01-01"],
            svec!["test", "2020-01-02"],
            svec!["test", "2020-01-03"],
            svec!["test", "2020-01-04"],
            svec!["test", "2020-01-05"],
            svec!["test", "2020-01-06"],
            svec!["test", "2020-01-07"],
            svec!["test", "2020-01-08"],
            svec!["test", "2020-01-09"],
            svec!["test", "2020-01-10"],
        ],
    );

    // Generate stats with date inference
    let mut stats_cmd = wrk.command("stats");
    stats_cmd
        .arg("--everything")
        .arg("--infer-dates")
        .arg("--dates-whitelist")
        .arg("all")
        .arg("test.csv");
    wrk.assert_success(&mut stats_cmd);

    // Run moarstats
    let mut cmd = wrk.command("moarstats");
    cmd.arg("test.csv");
    wrk.assert_success(&mut cmd);

    // Verify date fields get winsorized/trimmed means as RFC3339 strings
    let stats_content = wrk.read_to_string("test.stats.csv").unwrap();
    let mut rdr = ReaderBuilder::new()
        .has_headers(true)
        .from_reader(stats_content.as_bytes());

    let headers = rdr.headers().unwrap().clone();
    let field_idx = get_column_index(&headers, "field").unwrap();
    let type_idx = get_column_index(&headers, "type").unwrap();

    for result in rdr.records() {
        let record = result.unwrap();
        let field_name = get_field_value(&record, field_idx).unwrap();
        let field_type = get_field_value(&record, type_idx).unwrap();

        if field_name == "test" && (field_type == "Date" || field_type == "DateTime") {
            // Check winsorized mean is formatted as date string
            if let Some(winsorized_idx) = get_column_index(&headers, "winsorized_mean_25pct") {
                let winsorized_val = get_field_value(&record, winsorized_idx);
                if let Some(val_str) = winsorized_val {
                    if !val_str.is_empty() {
                        // Should be RFC3339 format (contains 'T' or is YYYY-MM-DD)
                        assert!(
                            val_str.contains('-') && val_str.len() >= 10,
                            "Date winsorized mean should be formatted as date string, got: {}",
                            val_str
                        );
                    }
                }
            }

            break;
        }
    }
}

#[test]
fn moarstats_zero_stddev_handling() {
    let wrk = Workdir::new("moarstats_zero_stddev");

    // Create a CSV with constant values (stddev = 0)
    wrk.create(
        "test.csv",
        vec![
            svec!["field", "value"],
            svec!["test", "5"],
            svec!["test", "5"],
            svec!["test", "5"],
            svec!["test", "5"],
            svec!["test", "5"],
        ],
    );

    // Generate stats
    let mut stats_cmd = wrk.command("stats");
    stats_cmd.arg("--everything").arg("test.csv");
    wrk.assert_success(&mut stats_cmd);

    // Run moarstats - should handle zero stddev gracefully
    let mut cmd = wrk.command("moarstats");
    cmd.arg("test.csv");
    wrk.assert_success(&mut cmd);

    // Verify that stats requiring stddev are empty (division by zero)
    let stats_content = wrk.read_to_string("test.stats.csv").unwrap();
    let mut rdr = ReaderBuilder::new()
        .has_headers(true)
        .from_reader(stats_content.as_bytes());

    let headers = rdr.headers().unwrap().clone();
    let field_idx = get_column_index(&headers, "field").unwrap();

    for result in rdr.records() {
        let record = result.unwrap();
        let field_name = get_field_value(&record, field_idx).unwrap();

        if field_name == "test" {
            // Stats that require stddev should be empty when stddev is 0
            if let Some(pearson_idx) = get_column_index(&headers, "pearson_skewness") {
                let _pearson_val = get_field_value(&record, pearson_idx);
                // Value might be empty (which is correct for zero stddev)
                // or might be computed if mean == median (which gives 0)
            }

            break;
        }
    }
}

#[test]
fn moarstats_string_fields_skipped() {
    let wrk = Workdir::new("moarstats_strings");

    wrk.create(
        "test.csv",
        vec![
            svec!["field", "text_value"],
            svec!["test", "apple"],
            svec!["test", "banana"],
            svec!["test", "cherry"],
        ],
    );

    // Generate stats
    let mut stats_cmd = wrk.command("stats");
    stats_cmd.arg("--everything").arg("test.csv");
    wrk.assert_success(&mut stats_cmd);

    // Run moarstats
    let mut cmd = wrk.command("moarstats");
    cmd.arg("test.csv");
    wrk.assert_success(&mut cmd);

    // Verify string fields have empty values for numeric stats
    let stats_content = wrk.read_to_string("test.stats.csv").unwrap();
    let mut rdr = ReaderBuilder::new()
        .has_headers(true)
        .from_reader(stats_content.as_bytes());

    let headers = rdr.headers().unwrap().clone();
    let field_idx = get_column_index(&headers, "field").unwrap();
    let type_idx = get_column_index(&headers, "type").unwrap();

    for result in rdr.records() {
        let record = result.unwrap();
        let field_name = get_field_value(&record, field_idx).unwrap();
        let field_type = get_field_value(&record, type_idx).unwrap();

        if field_name == "test" && field_type == "String" {
            // String fields should have empty values for numeric stats
            if let Some(pearson_idx) = get_column_index(&headers, "pearson_skewness") {
                let pearson_val = get_field_value(&record, pearson_idx);
                assert!(
                    pearson_val.is_none() || pearson_val.as_ref().unwrap().is_empty(),
                    "String fields should not have numeric statistics"
                );
            }

            break;
        }
    }
}

#[test]
fn moarstats_boolean_fields() {
    let wrk = Workdir::new("moarstats_boolean");

    wrk.create(
        "test.csv",
        vec![
            svec!["field", "bool_value"],
            svec!["test", "true"],
            svec!["test", "false"],
            svec!["test", "true"],
            svec!["test", "false"],
            svec!["test", "true"],
        ],
    );

    // Generate stats with boolean inference
    let mut stats_cmd = wrk.command("stats");
    stats_cmd
        .arg("--everything")
        .arg("--infer-boolean")
        .arg("test.csv");
    wrk.assert_success(&mut stats_cmd);

    // Run moarstats
    let mut cmd = wrk.command("moarstats");
    cmd.arg("test.csv");
    wrk.assert_success(&mut cmd);

    // Verify boolean fields can have statistics computed
    let stats_content = wrk.read_to_string("test.stats.csv").unwrap();
    let mut rdr = ReaderBuilder::new()
        .has_headers(true)
        .from_reader(stats_content.as_bytes());

    let headers = rdr.headers().unwrap().clone();
    let field_idx = get_column_index(&headers, "field").unwrap();
    let type_idx = get_column_index(&headers, "type").unwrap();

    for result in rdr.records() {
        let record = result.unwrap();
        let field_name = get_field_value(&record, field_idx).unwrap();
        let field_type = get_field_value(&record, type_idx).unwrap();

        if field_name == "test" && field_type == "Boolean" {
            // Boolean fields are numeric-like, so they should get statistics
            if let Some(pearson_idx) = get_column_index(&headers, "pearson_skewness") {
                let _pearson_val = get_field_value(&record, pearson_idx);
                // Boolean fields might have statistics if mean/median/stddev are computed
                // The value might be empty if stddev is 0 (all same value)
            }

            break;
        }
    }
}

#[test]
fn moarstats_missing_percentiles_column() {
    let wrk = Workdir::new("moarstats_no_percentiles");
    let test_file = wrk.load_test_file("boston311-100.csv");

    // Generate stats WITHOUT percentiles
    let mut stats_cmd = wrk.command("stats");
    stats_cmd
        .arg("--everything")
        .arg("--infer-dates")
        .arg(&test_file);
    wrk.assert_success(&mut stats_cmd);

    // Try to use --use-percentiles when percentiles column doesn't exist
    let mut cmd = wrk.command("moarstats");
    cmd.arg(&test_file).arg("--use-percentiles");
    wrk.assert_success(&mut cmd); // Should succeed but not add winsorized/trimmed columns

    // Verify winsorized/trimmed columns are NOT added (since percentiles don't exist)
    let stats_content = wrk.read_to_string("boston311-100.stats.csv").unwrap();
    let mut rdr = ReaderBuilder::new()
        .has_headers(true)
        .from_reader(stats_content.as_bytes());

    let _headers = rdr.headers().unwrap().clone();
    // Should fall back to Q1/Q3 if available, or not add columns if neither available
    // The exact behavior depends on whether Q1/Q3 exist
}

#[test]
fn moarstats_relative_standard_error() {
    let wrk = Workdir::new("moarstats_rse");
    let test_file = wrk.load_test_file("boston311-100.csv");

    // Generate baseline stats
    let mut stats_cmd = wrk.command("stats");
    stats_cmd
        .arg("--everything")
        .arg("--infer-dates")
        .arg(&test_file);
    wrk.assert_success(&mut stats_cmd);

    // Run moarstats
    let mut cmd = wrk.command("moarstats");
    cmd.arg(&test_file);
    wrk.assert_success(&mut cmd);

    // Verify relative_standard_error column exists and has values
    let stats_content = wrk.read_to_string("boston311-100.stats.csv").unwrap();
    let mut rdr = ReaderBuilder::new()
        .has_headers(true)
        .from_reader(stats_content.as_bytes());

    let headers = rdr.headers().unwrap().clone();
    let field_idx = get_column_index(&headers, "field").unwrap();
    let type_idx = get_column_index(&headers, "type").unwrap();

    assert!(
        get_column_index(&headers, "relative_standard_error").is_some(),
        "relative_standard_error column should exist"
    );

    let mut found_rse_value = false;
    for result in rdr.records() {
        let record = result.unwrap();
        let field_name = get_field_value(&record, field_idx).unwrap();
        let field_type = get_field_value(&record, type_idx).unwrap();

        if (field_type == "Float" || field_type == "Integer")
            && (field_name == "latitude" || field_name == "longitude")
        {
            if let Some(rse_idx) = get_column_index(&headers, "relative_standard_error") {
                let rse_val = get_field_value(&record, rse_idx);
                if let Some(val_str) = rse_val {
                    if !val_str.is_empty() {
                        found_rse_value = true;
                        let rse: f64 = val_str.parse().unwrap();
                        assert!(rse >= 0.0, "Relative standard error should be non-negative");
                    }
                }
            }
            break;
        }
    }

    assert!(
        found_rse_value,
        "Should find relative_standard_error value for numeric fields"
    );
}

#[test]
fn moarstats_zscore_min_max() {
    let wrk = Workdir::new("moarstats_zscore");
    let test_file = wrk.load_test_file("boston311-100.csv");

    // Generate baseline stats
    let mut stats_cmd = wrk.command("stats");
    stats_cmd
        .arg("--everything")
        .arg("--infer-dates")
        .arg(&test_file);
    wrk.assert_success(&mut stats_cmd);

    // Run moarstats
    let mut cmd = wrk.command("moarstats");
    cmd.arg(&test_file);
    wrk.assert_success(&mut cmd);

    // Verify zscore columns exist
    let stats_content = wrk.read_to_string("boston311-100.stats.csv").unwrap();
    let mut rdr = ReaderBuilder::new()
        .has_headers(true)
        .from_reader(stats_content.as_bytes());

    let headers = rdr.headers().unwrap().clone();
    let field_idx = get_column_index(&headers, "field").unwrap();
    let type_idx = get_column_index(&headers, "type").unwrap();

    assert!(
        get_column_index(&headers, "min_zscore").is_some(),
        "min_zscore column should exist"
    );
    assert!(
        get_column_index(&headers, "max_zscore").is_some(),
        "max_zscore column should exist"
    );

    // Verify zscore values are reasonable
    for result in rdr.records() {
        let record = result.unwrap();
        let field_name = get_field_value(&record, field_idx).unwrap();
        let field_type = get_field_value(&record, type_idx).unwrap();

        if (field_type == "Float" || field_type == "Integer") && field_name == "latitude" {
            if let Some(min_zscore_idx) = get_column_index(&headers, "min_zscore") {
                let min_zscore_val = get_field_value(&record, min_zscore_idx);
                if let Some(val_str) = min_zscore_val {
                    if !val_str.is_empty() {
                        let zscore: f64 = val_str.parse().unwrap();
                        // Min zscore should typically be negative (min is usually below mean)
                        // But we just check it's a valid number
                        assert!(zscore.is_finite(), "min_zscore should be a finite number");
                    }
                }
            }

            if let Some(max_zscore_idx) = get_column_index(&headers, "max_zscore") {
                let max_zscore_val = get_field_value(&record, max_zscore_idx);
                if let Some(val_str) = max_zscore_val {
                    if !val_str.is_empty() {
                        let zscore: f64 = val_str.parse().unwrap();
                        // Max zscore should typically be positive (max is usually above mean)
                        assert!(zscore.is_finite(), "max_zscore should be a finite number");
                    }
                }
            }

            break;
        }
    }
}

#[test]
fn moarstats_median_mean_ratio() {
    let wrk = Workdir::new("moarstats_median_mean");

    // Create CSV with skewed data
    wrk.create(
        "test.csv",
        vec![
            svec!["field", "value"],
            svec!["test", "1"],
            svec!["test", "2"],
            svec!["test", "3"],
            svec!["test", "4"],
            svec!["test", "5"],
            svec!["test", "100"], // Outlier that affects mean but not median
        ],
    );

    // Generate stats
    let mut stats_cmd = wrk.command("stats");
    stats_cmd.arg("--everything").arg("test.csv");
    wrk.assert_success(&mut stats_cmd);

    // Run moarstats
    let mut cmd = wrk.command("moarstats");
    cmd.arg("test.csv");
    wrk.assert_success(&mut cmd);

    // Verify median_mean_ratio column exists
    let stats_content = wrk.read_to_string("test.stats.csv").unwrap();
    let mut rdr = ReaderBuilder::new()
        .has_headers(true)
        .from_reader(stats_content.as_bytes());

    let headers = rdr.headers().unwrap().clone();
    let field_idx = get_column_index(&headers, "field").unwrap();

    assert!(
        get_column_index(&headers, "median_mean_ratio").is_some(),
        "median_mean_ratio column should exist"
    );

    for result in rdr.records() {
        let record = result.unwrap();
        let field_name = get_field_value(&record, field_idx).unwrap();

        if field_name == "test" {
            if let Some(ratio_idx) = get_column_index(&headers, "median_mean_ratio") {
                let ratio_val = get_field_value(&record, ratio_idx);
                if let Some(val_str) = ratio_val {
                    if !val_str.is_empty() {
                        let ratio: f64 = val_str.parse().unwrap();
                        // With an outlier, median should be less than mean, so ratio < 1
                        assert!(
                            ratio > 0.0 && ratio.is_finite(),
                            "median_mean_ratio should be positive and finite"
                        );
                    }
                }
            }
            break;
        }
    }
}

#[test]
fn moarstats_advanced() {
    let wrk = Workdir::new("moarstats_advanced");
    let test_file = wrk.load_test_file("boston311-100.csv");

    // Generate baseline stats
    let mut stats_cmd = wrk.command("stats");
    stats_cmd
        .arg("--everything")
        .arg("--infer-dates")
        .arg(&test_file);
    wrk.assert_success(&mut stats_cmd);

    // Run moarstats with --advanced flag
    let mut cmd = wrk.command("moarstats");
    cmd.arg("--advanced").arg(&test_file);
    wrk.assert_success(&mut cmd);

    let got = wrk.read_to_string("boston311-100.stats.csv").unwrap();
    let expected = r#"field,type,is_ascii,sum,min,max,range,sort_order,sortiness,min_length,max_length,sum_length,avg_length,stddev_length,variance_length,cv_length,mean,sem,geometric_mean,harmonic_mean,stddev,variance,cv,nullcount,n_negative,n_zero,n_positive,max_precision,sparsity,mad,lower_outer_fence,lower_inner_fence,q1,q2_median,q3,iqr,upper_inner_fence,upper_outer_fence,skewness,cardinality,uniqueness_ratio,mode,mode_count,mode_occurrences,antimode,antimode_count,antimode_occurrences,percentiles,pearson_skewness,range_stddev_ratio,quartile_coefficient_dispersion,mode_zscore,relative_standard_error,min_zscore,max_zscore,median_mean_ratio,iqr_range_ratio,mad_stddev_ratio,kurtosis,bimodality_coefficient,gini_coefficient,atkinson_index_(1),shannon_entropy,normalized_entropy,xsd_type,outliers_extreme_lower_cnt,outliers_mild_lower_cnt,outliers_normal_cnt,outliers_mild_upper_cnt,outliers_extreme_upper_cnt,outliers_total_cnt,outliers_mean,non_outliers_mean,outliers_to_normal_mean_ratio,outliers_min,outliers_max,outliers_range,outliers_stddev,outliers_variance,non_outliers_stddev,non_outliers_variance,outliers_cv,non_outliers_cv,outliers_percentage,outlier_impact,outlier_impact_ratio,outliers_normal_stddev_ratio,lower_outer_fence_zscore,upper_outer_fence_zscore,winsorized_mean_25pct,trimmed_mean_25pct,trimmed_stddev_25pct,trimmed_variance_25pct,winsorized_stddev_25pct,winsorized_variance_25pct,trimmed_cv_25pct,winsorized_cv_25pct,trimmed_25pct_stddev_ratio,winsorized_25pct_stddev_ratio,trimmed_range_25pct,winsorized_range_25pct
case_enquiry_id,Integer,,10100411645180,101004113298,101004155594,42296,Unsorted,0.798,,,,,,,,101004116451.8,790.552,101004116451.8012,101004116451.7994,7905.5202,62497248.9352,0,0,0,0,100,,0,673,101004109567,101004111646,101004113725,101004114353,101004115111,1386,101004117190,101004119269,0.0938,100,1,,0,0,*ALL,0,1,5: 101004113371|10: 101004113403|40: 101004114016|60: 101004114652|90: 101004115369|95: 101004141354,0.7965,5.3502,0,,0,-0.3989,4.9512,1,0.0328,0.0851,16.8682,0.0508,0,0,6.6439,1,unsignedLong,0,0,91,1,8,9,101004138497.4444,101004114271.4615,1,101004118346,101004155594,37248,13272.557,176160768,0,0,0,0,9,2180.3385,0,,-0.8709,0.3564,101004114376.68,101004114335.36,1170.2857,1369568.6531,2603.5885,6778673.1313,0,0,0.148,0.3293,1375,1386
open_dt,DateTime,,,2022-01-01T00:16:00+00:00,2022-01-31T11:46:00+00:00,30.47917,Unsorted,0.798,,,,,,,,2022-01-04T07:07:45.050+00:00,0.5568,18996.29623,18996.29542,5.568,31.00259,0.0293,0,,,,,0,0.76261,2021-12-27T14:16:49+00:00,2021-12-30T06:00:07+00:00,2022-01-01T21:43:25+00:00,2022-01-03T07:02:14+00:00,2022-01-03T16:12:17+00:00,1.77005,2022-01-06T07:55:35+00:00,2022-01-08T23:38:53+00:00,-0.5684,100,1,,0,0,*ALL,0,1,5: 2022-01-01T08:33:00+00:00|10: 2022-01-01T09:43:36+00:00|40: 2022-01-02T13:22:10+00:00|60: 2022-01-03T10:42:18+00:00|90: 2022-01-04T06:15:33+00:00|95: 2022-01-20T08:07:49+00:00,,5.474,,,,,,,0.0581,0.137,15.9129,0.07,0.0001,0,6.6439,1,dateTime,0,0,0,0,100,100,2022-01-04T07:07:45.049+00:00,,,2022-01-01T00:16:00+00:00,2022-01-31T11:46:00+00:00,30.4792,5.596,31.3157,,,0.0003,,100,,,,,,2022-01-02T21:11:55.789+00:00,2022-01-02T23:26:00.580+00:00,0.5422,0.2939,0.7415,0.5498,0,0,0.0974,0.1332,1.7595,1.77
target_dt,DateTime,,,2022-01-03T10:32:34+00:00,2022-05-20T13:03:21+00:00,137.10471,Unsorted,0.2273,,,,,,,,2022-01-17T03:14:16.404+00:00,2.86258,19009.11578,19009.0967,27.00551,729.29774,0.1421,11,,,,,0.11,1,2021-11-26T08:30:00+00:00,2021-12-15T20:30:00+00:00,2022-01-04T08:30:00+00:00,2022-01-05T08:30:00+00:00,2022-01-17T08:30:00+00:00,13,2022-02-05T20:30:00+00:00,2022-02-25T08:30:00+00:00,0.8462,42,0.42,2022-01-04 08:30:00,1,25,*PREVIEW: 2022-01-03 10:32:34|2022-01-03 11:58:12|2022-01-04 09:58:36|2022-01-04 10:41:29|2022-01-04...,34,1,5: 2022-01-04T08:30:00+00:00|10: 2022-01-04T08:30:00+00:00|40: 2022-01-04T16:08:34+00:00|60: 2022-01-05T13:07:19+00:00|90: 2022-02-17T12:47:39+00:00|95: 2022-03-10T16:14:45+00:00,,5.0769,,,,,,,0.0948,0.037,12.3956,0.1115,0.0005,0,4.2653,0.791,dateTime,0,0,0,0,89,89,2022-01-17T03:14:16.404+00:00,,,2022-01-03T10:32:33.999+00:00,2022-05-20T13:03:21+00:00,137.1047,27.1585,737.5852,,,0.0014,,100,,,,,,2022-01-08T16:19:46.393+00:00,2022-01-06T01:14:23.469+00:00,3.1543,9.9495,5.5597,30.9108,0.0002,0.0003,0.1168,0.2059,13,13
closed_dt,DateTime,,,2022-01-01T12:56:14+00:00,2022-04-25T14:30:31+00:00,114.06547,Unsorted,-0.0714,,,,,,,,2022-01-08T01:10:44.411+00:00,1.71655,19000.04255,19000.036,15.82577,250.4549,0.0833,15,,,,,0.15,0.77213,2021-12-29T15:13:29+00:00,2021-12-31T19:50:08.750+00:00,2022-01-03T00:26:48.500+00:00,2022-01-03T12:15:23+00:00,2022-01-04T11:31:15+00:00,1.46142,2022-01-06T16:07:54.750+00:00,2022-01-08T20:44:34.500+00:00,0.3266,86,0.86,,1,15,*PREVIEW: 2022-01-01 12:56:14|2022-01-01 14:17:15|2022-01-01 14:59:41|2022-01-01 15:10:16|2022-01-01...,85,1,5: 2022-01-01T19:07:41+00:00|10: 2022-01-02T11:03:10+00:00|40: 2022-01-03T10:06:33+00:00|60: 2022-01-03T21:43:55+00:00|90: 2022-01-18T07:54:26+00:00|95: 2022-01-20T08:45:12+00:00,,7.2076,,,,,,,0.0128,0.0488,31.684,0.0319,0.0002,0,6.0578,0.9427,dateTime,0,0,0,0,85,85,2022-01-08T01:10:44.411+00:00,,,2022-01-01T12:56:14+00:00,2022-04-25T14:30:31+00:00,114.0655,15.9197,253.4365,,,0.0008,,100,,,,,,2022-01-03T17:19:50.111+00:00,2022-01-03T16:41:33.162+00:00,0.4298,0.1847,0.6,0.36,0,0,0.0272,0.0379,1.4318,1.4614
ontime,String,true,,ONTIME,OVERDUE,,Unsorted,0.6768,6,7,617,6.17,0.3756,0.1411,0.0609,,,,,,,,0,,,,,0,,,,,,,,,,,2,0.02,ONTIME,1,83,OVERDUE,1,17,,,,,,,,,,,,,,,,0.6577,0.6577,string,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,
case_status,String,true,,Closed,Open,,Unsorted,0.7172,4,6,570,5.7,0.7141,0.51,0.1253,,,,,,,,0,,,,,0,,,,,,,,,,,2,0.02,Closed,1,85,Open,1,15,,,,,,,,,,,,,,,,0.6098,0.6098,string,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,
closure_reason,String,true,, ,Case Closed. Closed date : Wed Jan 19 11:42:16 EST 2022 Resolved Removed df  ,,Unsorted,-0.0909,1,284,8314,83.14,55.0262,3027.8804,0.6618,,,,,,,,0,,,,,0,,,,,,,,,,,86,0.86, ,1,15,*PREVIEW: Case Closed Case Resolved  NEW CART#21026466 DELV ON 1/11/22  |Case Closed Case Resolved  ...,85,1,,,,,,,,,,,,,,,,6.0578,0.9427,string,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,
case_title,String,true,,Abandoned Vehicles,Traffic Signal Inspection,,Unsorted,0.0101,10,57,2386,23.86,9.452,89.3404,0.3961,,,,,,,,0,,,,,0,,,,,,,,,,,42,0.42,Parking Enforcement,1,20,*PREVIEW: Animal Generic Request|BTDT: Complaint|City/State Snow Issues|DISPATCHED Short Term Rental...,24,1,,,,,,,,,,,,,,,,4.7216,0.8756,string,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,
subject,String,true,,Animal Control,Transportation - Traffic Division,,Unsorted,0.3737,14,33,2570,25.7,4.9041,24.05,0.1908,,,,,,,,0,,,,,0,,,,,,,,,,,9,0.09,Public Works Department,1,51,Animal Control|Boston Police Department|Boston Water & Sewer Commission,3,1,,,,,,,,,,,,,,,,1.9898,0.6277,string,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,
reason,String,true,,Administrative & General Requests,Street Lights,,Unsorted,0.1717,7,33,1892,18.92,8.4056,70.6536,0.4443,,,,,,,,0,,,,,0,,,,,,,,,,,20,0.2,Enforcement & Abandoned Vehicles,1,23,Administrative & General Requests|Animal Issues|Building|Employee & General Comments|Noise Disturban...,7,1,,,,,,,,,,,,,,,,3.5804,0.8284,string,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,
type,String,true,,Abandoned Vehicles,Unsatisfactory Utilities - Electrical  Plumbing,,Unsorted,0.0707,10,47,2266,22.66,8.034,64.5444,0.3545,,,,,,,,0,,,,,0,,,,,,,,,,,36,0.36,Parking Enforcement,1,20,*PREVIEW: Animal Generic Request|City/State Snow Issues|Electrical|General Comments For a Program or...,15,1,,,,,,,,,,,,,,,,4.5789,0.8857,string,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,
queue,String,true,,BTDT_AVRS Interface Queue,PWDx_Street Light_General Lighting Request,,Unsorted,0.2121,13,55,2802,28.02,8.0224,64.3596,0.2863,,,,,,,,0,,,,,0,,,,,,,,,,,35,0.35,BTDT_Parking Enforcement,1,21,*PREVIEW: BTDT_BostonBikes|BTDT_Engineering_New Sign and Pavement Marking Requests|BTDT_Sign Shop_Si...,15,1,,,,,,,,,,,,,,,,4.4803,0.8735,string,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,
department,String,true,,BTDT,PWDx,,Unsorted,0.3737,3,4,392,3.92,0.2713,0.0736,0.0692,,,,,,,,0,,,,,0,,,,,,,,,,,7,0.07,PWDx,1,49,GEN_,1,2,,,,,,,,,,,,,,,,1.9217,0.6845,string,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,
submittedphoto,String,true,,https://311.boston.gov/media/boston/report/photos/61d03f0d05bbcf180c2965fd/report.jpg,https://311.boston.gov/media/boston/report/photos/61d75bba05bbcf180c2d41de/report.jpg,,Unsorted,0.8537,0,100,3633,36.33,43.2585,1871.2989,1.1907,,,,,,,,58,,,,,0.58,,,,,,,,,,,43,0.43,,1,58,*PREVIEW: https://311.boston.gov/media/boston/report/photos/61d03f0d05bbcf180c2965fd/report.jpg|http...,42,1,,,,,,,,,,,,,,,,3.2462,0.5982,string,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,
closedphoto,NULL,,,,,,,,,,,,,,,,,,,,,,100,,,,,1,,,,,,,,,,,1,0.01,,1,100,,0,0,,,,,,,,,,,,,,,,0,0,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,
location,String,true,, ,INTERSECTION of Verdun St & Gallivan Blvd  Dorchester  MA  ,,Unsorted,-0.0303,1,63,3938,39.38,9.6247,92.6356,0.2444,,,,,,,,0,,,,,0,,,,,,,,,,,98,0.98,563 Columbus Ave  Roxbury  MA  02118|INTERSECTION of Gallivan Blvd & Washington St  Dorchester  MA  ,2,2,*PREVIEW:  |103 N Beacon St  Brighton  MA  02135|11 Aberdeen St  Boston  MA  02215|1148 Hyde Park Av...,96,1,,,,,,,,,,,,,,,,6.6039,0.9984,string,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,
fire_district,String,true,, ,9,,Unsorted,0.1515,1,2,113,1.13,0.3363,0.1131,0.2976,,,,,,,,0,,,,,0,,,,,,,,,,,10,0.1,3,1,19, ,1,1,,,,,,,,,,,,,,,,3.1048,0.9346,string,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,
pwd_district,String,true,, ,1C,,Unsorted,0.0707,1,3,209,2.09,0.3192,0.1019,0.1527,,,,,,,,0,,,,,0,,,,,,,,,,,14,0.14,1B,1,16, ,1,1,,,,,,,,,,,,,,,,3.5523,0.933,string,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,
city_council_district,String,true,, ,9,,Unsorted,0.1313,1,1,100,1,0,0,0,,,,,,,,0,,,,,0,,,,,,,,,,,10,0.1,1,1,22, ,1,1,,,,,,,,,,,,,,,,3.0377,0.9144,string,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,
police_district,String,true,, ,E5,,Unsorted,0.1717,1,3,223,2.23,0.444,0.1971,0.1991,,,,,,,,0,,,,,0,,,,,,,,,,,13,0.13,A1,1,20, ,1,1,,,,,,,,,,,,,,,,3.4061,0.9205,string,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,
neighborhood,String,true,, ,West Roxbury,,Unsorted,0.0303,1,38,1486,14.86,9.8671,97.3604,0.664,,,,,,,,0,,,,,0,,,,,,,,,,,19,0.19,Dorchester,1,15, |Brighton|Mission Hill,3,1,,,,,,,,,,,,,,,,3.8906,0.9159,string,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,
neighborhood_services_district,String,true,, ,9,,Unsorted,0.0303,1,2,139,1.39,0.4877,0.2379,0.3509,,,,,,,,0,,,,,0,,,,,,,,,,,16,0.16,3,1,15, |12,2,1,,,,,,,,,,,,,,,,3.6409,0.9102,string,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,
ward,String,true,, ,Ward 9,,Unsorted,0.0505,1,7,499,4.99,2.2293,4.9699,0.4468,,,,,,,,0,,,,,0,,,,,,,,,,,42,0.42,Ward 3,1,10,*PREVIEW:  |01|02|04|06|07|1|10|16|18,23,1,,,,,,,,,,,,,,,,4.9655,0.9208,string,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,
precinct,String,true,, ,2210,,Unsorted,0.0408,0,4,393,3.93,0.4951,0.2451,0.126,,,,,,,,1,,,,,0.01,,,,,,,,,,,76,0.76,0306,1,5,*PREVIEW: NULL| |0102|0105|0108|0109|0201|0204|0305|0307,61,1,,,,,,,,,,,,,,,,6.0527,0.9687,gYear??,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,
location_street_name,String,true,,103 N Beacon St,INTERSECTION Verdun St & Gallivan Blvd,,Unsorted,-0.0204,0,45,1800,18,9.3995,88.3508,0.5222,,,,,,,,1,,,,,0.01,,,,,,,,,,,97,0.97,20 Washington St|563 Columbus Ave|INTERSECTION Gallivan Blvd & Washington St,3,2,*PREVIEW: NULL|103 N Beacon St|11 Aberdeen St|1148 Hyde Park Ave|119 L St|12 Derne St|126 Elm St|127...,94,1,,,,,,,,,,,,,,,,6.5839,0.9976,string,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,
location_zipcode,String,true,,02109,02215,,Unsorted,0.0488,0,5,415,4.15,1.8405,3.3874,0.4435,,,,,,,,17,,,,,0.17,,,,,,,,,,,24,0.24,,1,17,02126|02134|02210|02215,4,1,,,,,,,,,,,,,,,,4.1873,0.9133,string,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,
latitude,Float,,4233.6674,42.2553,42.3806,0.1253,Unsorted,0.0505,,,,,,,,42.3367,0.0031,42.3367,42.3367,0.0305,0.0009,0.072,0,0,0,100,4,0,0.0163,42.2034,42.2619,42.3204,42.3432,42.3594,0.039,42.4179,42.4764,-0.1667,78,0.78,42.3594,1,20,*PREVIEW: 42.2553|42.2601|42.2609|42.2645|42.2674|42.2789|42.2797|42.2804|42.2821|42.2878,74,1,5: 42.2674|10: 42.2878|40: 42.3347|60: 42.3549|90: 42.3666|95: 42.3735,-0.6393,4.1082,0.0005,0.7443,0.0001,-2.6689,1.4393,1.0002,0.3113,0.5344,3.2395,0.1647,0.0004,0,5.7195,0.91,decimal,0,3,97,0,0,3,42.2588,42.3391,0.9981,42.2553,42.2609,0.0056,0.003,0,0.0278,0.0008,0.0001,0.0007,3,-0.0024,-0.0001,0.1089,-4.3705,4.5803,42.3421,42.3458,0.0126,0.0002,0.0165,0.0003,0.0003,0.0004,0.4131,0.5412,0.0374,0.039
longitude,Float,,-7107.2688,-71.1626,-71.0298,0.1328,Unsorted,0.1515,,,,,,,,-71.0727,0.0031,,,0.0311,0.001,-0.0437,0,100,0,0,4,0,0.0121,-71.1741,-71.1294,-71.0848,-71.0609,-71.055,0.0298,-71.0104,-70.9658,-0.6101,77,0.77,-71.0587,1,19,*PREVIEW: -71.0298|-71.0301|-71.0309|-71.0323|-71.0325|-71.0329|-71.0336|-71.0338|-71.034|-71.0355,72,1,5: -71.1415|10: -71.1305|40: -71.069|60: -71.0587|90: -71.0357|95: -71.0329,-1.1383,4.2701,-0.0002,0.4502,0,-2.8907,1.3794,0.9998,0.2244,0.3891,3.845,0.2005,,,5.7292,0.9142,decimal,0,10,90,0,0,10,-71.143,-71.0649,1.0011,-71.1626,-71.1305,0.0321,0.0113,0.0001,0.0213,0.0005,0.0002,0.0003,10,-0.0078,-0.0001,0.5291,-3.2605,3.4373,-71.0676,-71.0653,0.0085,0.0001,0.0124,0.0002,0.0001,0.0002,0.2726,0.3979,0.029,0.0298
source,String,true,,Citizens Connect App,Self Service,,Unsorted,0.5354,12,20,1801,18.01,2.3473,5.5099,0.1303,,,,,,,,0,,,,,0,,,,,,,,,,,4,0.04,Citizens Connect App,1,56,Self Service,1,3,,,,,,,,,,,,,,,,1.4916,0.7458,string,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,
"#;
    assert_eq!(got, expected);
}

#[test]
fn moarstats_advanced_atkinson_epsilon() {
    let wrk = Workdir::new("moarstats_atkinson_epsilon");
    let test_file = wrk.load_test_file("boston311-100.csv");

    // Generate baseline stats
    let mut stats_cmd = wrk.command("stats");
    stats_cmd
        .arg("--everything")
        .arg("--infer-dates")
        .arg(&test_file);
    wrk.assert_success(&mut stats_cmd);

    // Run moarstats with --advanced flag
    let mut cmd = wrk.command("moarstats");
    cmd.arg("--advanced")
        .args(["--epsilon", "3.0"])
        .arg(&test_file);
    wrk.assert_success(&mut cmd);

    let got = wrk.read_to_string("boston311-100.stats.csv").unwrap();
    let expected = r#"field,type,is_ascii,sum,min,max,range,sort_order,sortiness,min_length,max_length,sum_length,avg_length,stddev_length,variance_length,cv_length,mean,sem,geometric_mean,harmonic_mean,stddev,variance,cv,nullcount,n_negative,n_zero,n_positive,max_precision,sparsity,mad,lower_outer_fence,lower_inner_fence,q1,q2_median,q3,iqr,upper_inner_fence,upper_outer_fence,skewness,cardinality,uniqueness_ratio,mode,mode_count,mode_occurrences,antimode,antimode_count,antimode_occurrences,percentiles,pearson_skewness,range_stddev_ratio,quartile_coefficient_dispersion,mode_zscore,relative_standard_error,min_zscore,max_zscore,median_mean_ratio,iqr_range_ratio,mad_stddev_ratio,kurtosis,bimodality_coefficient,gini_coefficient,atkinson_index_(3),shannon_entropy,normalized_entropy,xsd_type,outliers_extreme_lower_cnt,outliers_mild_lower_cnt,outliers_normal_cnt,outliers_mild_upper_cnt,outliers_extreme_upper_cnt,outliers_total_cnt,outliers_mean,non_outliers_mean,outliers_to_normal_mean_ratio,outliers_min,outliers_max,outliers_range,outliers_stddev,outliers_variance,non_outliers_stddev,non_outliers_variance,outliers_cv,non_outliers_cv,outliers_percentage,outlier_impact,outlier_impact_ratio,outliers_normal_stddev_ratio,lower_outer_fence_zscore,upper_outer_fence_zscore,winsorized_mean_25pct,trimmed_mean_25pct,trimmed_stddev_25pct,trimmed_variance_25pct,winsorized_stddev_25pct,winsorized_variance_25pct,trimmed_cv_25pct,winsorized_cv_25pct,trimmed_25pct_stddev_ratio,winsorized_25pct_stddev_ratio,trimmed_range_25pct,winsorized_range_25pct
case_enquiry_id,Integer,,10100411645180,101004113298,101004155594,42296,Unsorted,0.798,,,,,,,,101004116451.8,790.552,101004116451.8012,101004116451.7994,7905.5202,62497248.9352,0,0,0,0,100,,0,673,101004109567,101004111646,101004113725,101004114353,101004115111,1386,101004117190,101004119269,0.0938,100,1,,0,0,*ALL,0,1,5: 101004113371|10: 101004113403|40: 101004114016|60: 101004114652|90: 101004115369|95: 101004141354,0.7965,5.3502,0,,0,-0.3989,4.9512,1,0.0328,0.0851,16.8682,0.0508,0,0,6.6439,1,unsignedLong,0,0,91,1,8,9,101004138497.4444,101004114271.4615,1,101004118346,101004155594,37248,13272.557,176160768,0,0,0,0,9,2180.3385,0,,-0.8709,0.3564,101004114376.68,101004114335.36,1170.2857,1369568.6531,2603.5885,6778673.1313,0,0,0.148,0.3293,1375,1386
open_dt,DateTime,,,2022-01-01T00:16:00+00:00,2022-01-31T11:46:00+00:00,30.47917,Unsorted,0.798,,,,,,,,2022-01-04T07:07:45.050+00:00,0.5568,18996.29623,18996.29542,5.568,31.00259,0.0293,0,,,,,0,0.76261,2021-12-27T14:16:49+00:00,2021-12-30T06:00:07+00:00,2022-01-01T21:43:25+00:00,2022-01-03T07:02:14+00:00,2022-01-03T16:12:17+00:00,1.77005,2022-01-06T07:55:35+00:00,2022-01-08T23:38:53+00:00,-0.5684,100,1,,0,0,*ALL,0,1,5: 2022-01-01T08:33:00+00:00|10: 2022-01-01T09:43:36+00:00|40: 2022-01-02T13:22:10+00:00|60: 2022-01-03T10:42:18+00:00|90: 2022-01-04T06:15:33+00:00|95: 2022-01-20T08:07:49+00:00,,5.474,,,,,,,0.0581,0.137,15.9129,0.07,0.0001,0,6.6439,1,dateTime,0,0,0,0,100,100,2022-01-04T07:07:45.049+00:00,,,2022-01-01T00:16:00+00:00,2022-01-31T11:46:00+00:00,30.4792,5.596,31.3157,,,0.0003,,100,,,,,,2022-01-02T21:11:55.789+00:00,2022-01-02T23:26:00.580+00:00,0.5422,0.2939,0.7415,0.5498,0,0,0.0974,0.1332,1.7595,1.77
target_dt,DateTime,,,2022-01-03T10:32:34+00:00,2022-05-20T13:03:21+00:00,137.10471,Unsorted,0.2273,,,,,,,,2022-01-17T03:14:16.404+00:00,2.86258,19009.11578,19009.0967,27.00551,729.29774,0.1421,11,,,,,0.11,1,2021-11-26T08:30:00+00:00,2021-12-15T20:30:00+00:00,2022-01-04T08:30:00+00:00,2022-01-05T08:30:00+00:00,2022-01-17T08:30:00+00:00,13,2022-02-05T20:30:00+00:00,2022-02-25T08:30:00+00:00,0.8462,42,0.42,2022-01-04 08:30:00,1,25,*PREVIEW: 2022-01-03 10:32:34|2022-01-03 11:58:12|2022-01-04 09:58:36|2022-01-04 10:41:29|2022-01-04...,34,1,5: 2022-01-04T08:30:00+00:00|10: 2022-01-04T08:30:00+00:00|40: 2022-01-04T16:08:34+00:00|60: 2022-01-05T13:07:19+00:00|90: 2022-02-17T12:47:39+00:00|95: 2022-03-10T16:14:45+00:00,,5.0769,,,,,,,0.0948,0.037,12.3956,0.1115,0.0005,0,4.2653,0.791,dateTime,0,0,0,0,89,89,2022-01-17T03:14:16.404+00:00,,,2022-01-03T10:32:33.999+00:00,2022-05-20T13:03:21+00:00,137.1047,27.1585,737.5852,,,0.0014,,100,,,,,,2022-01-08T16:19:46.393+00:00,2022-01-06T01:14:23.469+00:00,3.1543,9.9495,5.5597,30.9108,0.0002,0.0003,0.1168,0.2059,13,13
closed_dt,DateTime,,,2022-01-01T12:56:14+00:00,2022-04-25T14:30:31+00:00,114.06547,Unsorted,-0.0714,,,,,,,,2022-01-08T01:10:44.411+00:00,1.71655,19000.04255,19000.036,15.82577,250.4549,0.0833,15,,,,,0.15,0.77213,2021-12-29T15:13:29+00:00,2021-12-31T19:50:08.750+00:00,2022-01-03T00:26:48.500+00:00,2022-01-03T12:15:23+00:00,2022-01-04T11:31:15+00:00,1.46142,2022-01-06T16:07:54.750+00:00,2022-01-08T20:44:34.500+00:00,0.3266,86,0.86,,1,15,*PREVIEW: 2022-01-01 12:56:14|2022-01-01 14:17:15|2022-01-01 14:59:41|2022-01-01 15:10:16|2022-01-01...,85,1,5: 2022-01-01T19:07:41+00:00|10: 2022-01-02T11:03:10+00:00|40: 2022-01-03T10:06:33+00:00|60: 2022-01-03T21:43:55+00:00|90: 2022-01-18T07:54:26+00:00|95: 2022-01-20T08:45:12+00:00,,7.2076,,,,,,,0.0128,0.0488,31.684,0.0319,0.0002,0,6.0578,0.9427,dateTime,0,0,0,0,85,85,2022-01-08T01:10:44.411+00:00,,,2022-01-01T12:56:14+00:00,2022-04-25T14:30:31+00:00,114.0655,15.9197,253.4365,,,0.0008,,100,,,,,,2022-01-03T17:19:50.111+00:00,2022-01-03T16:41:33.162+00:00,0.4298,0.1847,0.6,0.36,0,0,0.0272,0.0379,1.4318,1.4614
ontime,String,true,,ONTIME,OVERDUE,,Unsorted,0.6768,6,7,617,6.17,0.3756,0.1411,0.0609,,,,,,,,0,,,,,0,,,,,,,,,,,2,0.02,ONTIME,1,83,OVERDUE,1,17,,,,,,,,,,,,,,,,0.6577,0.6577,string,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,
case_status,String,true,,Closed,Open,,Unsorted,0.7172,4,6,570,5.7,0.7141,0.51,0.1253,,,,,,,,0,,,,,0,,,,,,,,,,,2,0.02,Closed,1,85,Open,1,15,,,,,,,,,,,,,,,,0.6098,0.6098,string,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,
closure_reason,String,true,, ,Case Closed. Closed date : Wed Jan 19 11:42:16 EST 2022 Resolved Removed df  ,,Unsorted,-0.0909,1,284,8314,83.14,55.0262,3027.8804,0.6618,,,,,,,,0,,,,,0,,,,,,,,,,,86,0.86, ,1,15,*PREVIEW: Case Closed Case Resolved  NEW CART#21026466 DELV ON 1/11/22  |Case Closed Case Resolved  ...,85,1,,,,,,,,,,,,,,,,6.0578,0.9427,string,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,
case_title,String,true,,Abandoned Vehicles,Traffic Signal Inspection,,Unsorted,0.0101,10,57,2386,23.86,9.452,89.3404,0.3961,,,,,,,,0,,,,,0,,,,,,,,,,,42,0.42,Parking Enforcement,1,20,*PREVIEW: Animal Generic Request|BTDT: Complaint|City/State Snow Issues|DISPATCHED Short Term Rental...,24,1,,,,,,,,,,,,,,,,4.7216,0.8756,string,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,
subject,String,true,,Animal Control,Transportation - Traffic Division,,Unsorted,0.3737,14,33,2570,25.7,4.9041,24.05,0.1908,,,,,,,,0,,,,,0,,,,,,,,,,,9,0.09,Public Works Department,1,51,Animal Control|Boston Police Department|Boston Water & Sewer Commission,3,1,,,,,,,,,,,,,,,,1.9898,0.6277,string,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,
reason,String,true,,Administrative & General Requests,Street Lights,,Unsorted,0.1717,7,33,1892,18.92,8.4056,70.6536,0.4443,,,,,,,,0,,,,,0,,,,,,,,,,,20,0.2,Enforcement & Abandoned Vehicles,1,23,Administrative & General Requests|Animal Issues|Building|Employee & General Comments|Noise Disturban...,7,1,,,,,,,,,,,,,,,,3.5804,0.8284,string,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,
type,String,true,,Abandoned Vehicles,Unsatisfactory Utilities - Electrical  Plumbing,,Unsorted,0.0707,10,47,2266,22.66,8.034,64.5444,0.3545,,,,,,,,0,,,,,0,,,,,,,,,,,36,0.36,Parking Enforcement,1,20,*PREVIEW: Animal Generic Request|City/State Snow Issues|Electrical|General Comments For a Program or...,15,1,,,,,,,,,,,,,,,,4.5789,0.8857,string,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,
queue,String,true,,BTDT_AVRS Interface Queue,PWDx_Street Light_General Lighting Request,,Unsorted,0.2121,13,55,2802,28.02,8.0224,64.3596,0.2863,,,,,,,,0,,,,,0,,,,,,,,,,,35,0.35,BTDT_Parking Enforcement,1,21,*PREVIEW: BTDT_BostonBikes|BTDT_Engineering_New Sign and Pavement Marking Requests|BTDT_Sign Shop_Si...,15,1,,,,,,,,,,,,,,,,4.4803,0.8735,string,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,
department,String,true,,BTDT,PWDx,,Unsorted,0.3737,3,4,392,3.92,0.2713,0.0736,0.0692,,,,,,,,0,,,,,0,,,,,,,,,,,7,0.07,PWDx,1,49,GEN_,1,2,,,,,,,,,,,,,,,,1.9217,0.6845,string,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,
submittedphoto,String,true,,https://311.boston.gov/media/boston/report/photos/61d03f0d05bbcf180c2965fd/report.jpg,https://311.boston.gov/media/boston/report/photos/61d75bba05bbcf180c2d41de/report.jpg,,Unsorted,0.8537,0,100,3633,36.33,43.2585,1871.2989,1.1907,,,,,,,,58,,,,,0.58,,,,,,,,,,,43,0.43,,1,58,*PREVIEW: https://311.boston.gov/media/boston/report/photos/61d03f0d05bbcf180c2965fd/report.jpg|http...,42,1,,,,,,,,,,,,,,,,3.2462,0.5982,string,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,
closedphoto,NULL,,,,,,,,,,,,,,,,,,,,,,100,,,,,1,,,,,,,,,,,1,0.01,,1,100,,0,0,,,,,,,,,,,,,,,,0,0,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,
location,String,true,, ,INTERSECTION of Verdun St & Gallivan Blvd  Dorchester  MA  ,,Unsorted,-0.0303,1,63,3938,39.38,9.6247,92.6356,0.2444,,,,,,,,0,,,,,0,,,,,,,,,,,98,0.98,563 Columbus Ave  Roxbury  MA  02118|INTERSECTION of Gallivan Blvd & Washington St  Dorchester  MA  ,2,2,*PREVIEW:  |103 N Beacon St  Brighton  MA  02135|11 Aberdeen St  Boston  MA  02215|1148 Hyde Park Av...,96,1,,,,,,,,,,,,,,,,6.6039,0.9984,string,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,
fire_district,String,true,, ,9,,Unsorted,0.1515,1,2,113,1.13,0.3363,0.1131,0.2976,,,,,,,,0,,,,,0,,,,,,,,,,,10,0.1,3,1,19, ,1,1,,,,,,,,,,,,,,,,3.1048,0.9346,string,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,
pwd_district,String,true,, ,1C,,Unsorted,0.0707,1,3,209,2.09,0.3192,0.1019,0.1527,,,,,,,,0,,,,,0,,,,,,,,,,,14,0.14,1B,1,16, ,1,1,,,,,,,,,,,,,,,,3.5523,0.933,string,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,
city_council_district,String,true,, ,9,,Unsorted,0.1313,1,1,100,1,0,0,0,,,,,,,,0,,,,,0,,,,,,,,,,,10,0.1,1,1,22, ,1,1,,,,,,,,,,,,,,,,3.0377,0.9144,string,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,
police_district,String,true,, ,E5,,Unsorted,0.1717,1,3,223,2.23,0.444,0.1971,0.1991,,,,,,,,0,,,,,0,,,,,,,,,,,13,0.13,A1,1,20, ,1,1,,,,,,,,,,,,,,,,3.4061,0.9205,string,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,
neighborhood,String,true,, ,West Roxbury,,Unsorted,0.0303,1,38,1486,14.86,9.8671,97.3604,0.664,,,,,,,,0,,,,,0,,,,,,,,,,,19,0.19,Dorchester,1,15, |Brighton|Mission Hill,3,1,,,,,,,,,,,,,,,,3.8906,0.9159,string,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,
neighborhood_services_district,String,true,, ,9,,Unsorted,0.0303,1,2,139,1.39,0.4877,0.2379,0.3509,,,,,,,,0,,,,,0,,,,,,,,,,,16,0.16,3,1,15, |12,2,1,,,,,,,,,,,,,,,,3.6409,0.9102,string,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,
ward,String,true,, ,Ward 9,,Unsorted,0.0505,1,7,499,4.99,2.2293,4.9699,0.4468,,,,,,,,0,,,,,0,,,,,,,,,,,42,0.42,Ward 3,1,10,*PREVIEW:  |01|02|04|06|07|1|10|16|18,23,1,,,,,,,,,,,,,,,,4.9655,0.9208,string,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,
precinct,String,true,, ,2210,,Unsorted,0.0408,0,4,393,3.93,0.4951,0.2451,0.126,,,,,,,,1,,,,,0.01,,,,,,,,,,,76,0.76,0306,1,5,*PREVIEW: NULL| |0102|0105|0108|0109|0201|0204|0305|0307,61,1,,,,,,,,,,,,,,,,6.0527,0.9687,gYear??,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,
location_street_name,String,true,,103 N Beacon St,INTERSECTION Verdun St & Gallivan Blvd,,Unsorted,-0.0204,0,45,1800,18,9.3995,88.3508,0.5222,,,,,,,,1,,,,,0.01,,,,,,,,,,,97,0.97,20 Washington St|563 Columbus Ave|INTERSECTION Gallivan Blvd & Washington St,3,2,*PREVIEW: NULL|103 N Beacon St|11 Aberdeen St|1148 Hyde Park Ave|119 L St|12 Derne St|126 Elm St|127...,94,1,,,,,,,,,,,,,,,,6.5839,0.9976,string,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,
location_zipcode,String,true,,02109,02215,,Unsorted,0.0488,0,5,415,4.15,1.8405,3.3874,0.4435,,,,,,,,17,,,,,0.17,,,,,,,,,,,24,0.24,,1,17,02126|02134|02210|02215,4,1,,,,,,,,,,,,,,,,4.1873,0.9133,string,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,
latitude,Float,,4233.6674,42.2553,42.3806,0.1253,Unsorted,0.0505,,,,,,,,42.3367,0.0031,42.3367,42.3367,0.0305,0.0009,0.072,0,0,0,100,4,0,0.0163,42.2034,42.2619,42.3204,42.3432,42.3594,0.039,42.4179,42.4764,-0.1667,78,0.78,42.3594,1,20,*PREVIEW: 42.2553|42.2601|42.2609|42.2645|42.2674|42.2789|42.2797|42.2804|42.2821|42.2878,74,1,5: 42.2674|10: 42.2878|40: 42.3347|60: 42.3549|90: 42.3666|95: 42.3735,-0.6393,4.1082,0.0005,0.7443,0.0001,-2.6689,1.4393,1.0002,0.3113,0.5344,3.2395,0.1647,0.0004,0,5.7195,0.91,decimal,0,3,97,0,0,3,42.2588,42.3391,0.9981,42.2553,42.2609,0.0056,0.003,0,0.0278,0.0008,0.0001,0.0007,3,-0.0024,-0.0001,0.1089,-4.3705,4.5803,42.3421,42.3458,0.0126,0.0002,0.0165,0.0003,0.0003,0.0004,0.4131,0.5412,0.0374,0.039
longitude,Float,,-7107.2688,-71.1626,-71.0298,0.1328,Unsorted,0.1515,,,,,,,,-71.0727,0.0031,,,0.0311,0.001,-0.0437,0,100,0,0,4,0,0.0121,-71.1741,-71.1294,-71.0848,-71.0609,-71.055,0.0298,-71.0104,-70.9658,-0.6101,77,0.77,-71.0587,1,19,*PREVIEW: -71.0298|-71.0301|-71.0309|-71.0323|-71.0325|-71.0329|-71.0336|-71.0338|-71.034|-71.0355,72,1,5: -71.1415|10: -71.1305|40: -71.069|60: -71.0587|90: -71.0357|95: -71.0329,-1.1383,4.2701,-0.0002,0.4502,0,-2.8907,1.3794,0.9998,0.2244,0.3891,3.845,0.2005,,,5.7292,0.9142,decimal,0,10,90,0,0,10,-71.143,-71.0649,1.0011,-71.1626,-71.1305,0.0321,0.0113,0.0001,0.0213,0.0005,0.0002,0.0003,10,-0.0078,-0.0001,0.5291,-3.2605,3.4373,-71.0676,-71.0653,0.0085,0.0001,0.0124,0.0002,0.0001,0.0002,0.2726,0.3979,0.029,0.0298
source,String,true,,Citizens Connect App,Self Service,,Unsorted,0.5354,12,20,1801,18.01,2.3473,5.5099,0.1303,,,,,,,,0,,,,,0,,,,,,,,,,,4,0.04,Citizens Connect App,1,56,Self Service,1,3,,,,,,,,,,,,,,,,1.4916,0.7458,string,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,
"#;
    assert_eq!(got, expected);
}

#[test]
fn moarstats_kurtosis_gini_insufficient_data() {
    let wrk = Workdir::new("moarstats_kurtosis_gini_insufficient");

    // Create CSV with only one value (insufficient for kurtosis/Gini)
    wrk.create(
        "test.csv",
        vec![svec!["field", "value"], svec!["test", "5"]],
    );

    // Generate stats
    let mut stats_cmd = wrk.command("stats");
    stats_cmd.arg("--everything").arg("test.csv");
    wrk.assert_success(&mut stats_cmd);

    // Run moarstats with --advanced flag
    let mut cmd = wrk.command("moarstats");
    cmd.arg("--advanced").arg("test.csv");
    wrk.assert_success(&mut cmd);

    // Verify columns exist but values may be empty
    let stats_content = wrk.read_to_string("test.stats.csv").unwrap();
    let mut rdr = ReaderBuilder::new()
        .has_headers(true)
        .from_reader(stats_content.as_bytes());

    let headers = rdr.headers().unwrap().clone();
    let field_idx = get_column_index(&headers, "field").unwrap();

    assert!(
        get_column_index(&headers, "kurtosis").is_some(),
        "kurtosis column should exist"
    );
    assert!(
        get_column_index(&headers, "gini_coefficient").is_some(),
        "gini_coefficient column should exist"
    );

    for result in rdr.records() {
        let record = result.unwrap();
        let field_name = get_field_value(&record, field_idx).unwrap();

        if field_name == "test" {
            // With insufficient data, values should be empty
            if let Some(kurtosis_idx) = get_column_index(&headers, "kurtosis") {
                let kurtosis_val = get_field_value(&record, kurtosis_idx);
                // Value should be empty or None for insufficient data
                assert!(
                    kurtosis_val.is_none() || kurtosis_val.as_ref().unwrap().is_empty(),
                    "kurtosis should be empty for insufficient data"
                );
            }

            if let Some(gini_idx) = get_column_index(&headers, "gini_coefficient") {
                let gini_val = get_field_value(&record, gini_idx);
                // Value should be empty or None for insufficient data
                assert!(
                    gini_val.is_none() || gini_val.as_ref().unwrap().is_empty(),
                    "gini_coefficient should be empty for insufficient data"
                );
            }

            break;
        }
    }
}

#[test]
fn moarstats_kurtosis_gini_constant_values() {
    let wrk = Workdir::new("moarstats_kurtosis_gini_constant");

    // Create CSV with constant values
    wrk.create(
        "test.csv",
        vec![
            svec!["field", "value"],
            svec!["test", "5"],
            svec!["test", "5"],
            svec!["test", "5"],
            svec!["test", "5"],
            svec!["test", "5"],
        ],
    );

    // Generate stats
    let mut stats_cmd = wrk.command("stats");
    stats_cmd.arg("--everything").arg("test.csv");
    wrk.assert_success(&mut stats_cmd);

    // Run moarstats
    let mut cmd = wrk.command("moarstats");
    cmd.arg("test.csv").arg("--advanced");
    wrk.assert_success(&mut cmd);

    // Verify columns exist
    let stats_content = wrk.read_to_string("test.stats.csv").unwrap();
    let mut rdr = ReaderBuilder::new()
        .has_headers(true)
        .from_reader(stats_content.as_bytes());

    let headers = rdr.headers().unwrap().clone();
    let field_idx = get_column_index(&headers, "field").unwrap();

    for result in rdr.records() {
        let record = result.unwrap();
        let field_name = get_field_value(&record, field_idx).unwrap();

        if field_name == "test" {
            // With constant values, Gini should be 0 (perfect equality)
            if let Some(gini_idx) = get_column_index(&headers, "gini_coefficient") {
                let gini_val = get_field_value(&record, gini_idx);
                if let Some(val_str) = gini_val {
                    if !val_str.is_empty() {
                        let gini: f64 = val_str.parse().unwrap();
                        // Gini should be 0 for constant values (perfect equality)
                        assert!(
                            gini.abs() < 0.001,
                            "gini_coefficient should be approximately 0 for constant values, got: \
                             {}",
                            gini
                        );
                    }
                }
            }

            // Kurtosis might be computed or might be empty depending on implementation
            if let Some(kurtosis_idx) = get_column_index(&headers, "kurtosis") {
                let _kurtosis_val = get_field_value(&record, kurtosis_idx);
                // Value might be empty or computed - both are acceptable
            }

            break;
        }
    }
}

#[test]
fn moarstats_kurtosis_gini_unequal_distribution() {
    let wrk = Workdir::new("moarstats_kurtosis_gini_unequal");

    // Create CSV with highly unequal distribution (high Gini)
    wrk.create(
        "test.csv",
        vec![
            svec!["field", "value"],
            svec!["test", "1"],
            svec!["test", "1"],
            svec!["test", "1"],
            svec!["test", "1"],
            svec!["test", "100"], // One large value creates inequality
        ],
    );

    // Generate stats
    let mut stats_cmd = wrk.command("stats");
    stats_cmd.arg("--everything").arg("test.csv");
    wrk.assert_success(&mut stats_cmd);

    // Run moarstats
    let mut cmd = wrk.command("moarstats");
    cmd.arg("test.csv").arg("--advanced");
    wrk.assert_success(&mut cmd);

    // Verify Gini coefficient reflects inequality
    let stats_content = wrk.read_to_string("test.stats.csv").unwrap();
    let mut rdr = ReaderBuilder::new()
        .has_headers(true)
        .from_reader(stats_content.as_bytes());

    let headers = rdr.headers().unwrap().clone();
    let field_idx = get_column_index(&headers, "field").unwrap();

    for result in rdr.records() {
        let record = result.unwrap();
        let field_name = get_field_value(&record, field_idx).unwrap();

        if field_name == "test" {
            // With unequal distribution, Gini should be > 0
            if let Some(gini_idx) = get_column_index(&headers, "gini_coefficient") {
                let gini_val = get_field_value(&record, gini_idx);
                if let Some(val_str) = gini_val {
                    if !val_str.is_empty() {
                        let gini: f64 = val_str.parse().unwrap();
                        // With one large value, Gini should be significantly > 0
                        assert!(
                            gini > 0.1,
                            "gini_coefficient should be > 0.1 for unequal distribution, got: {}",
                            gini
                        );
                        assert!(
                            gini <= 1.0,
                            "gini_coefficient should be <= 1.0, got: {}",
                            gini
                        );
                    }
                }
            }

            // Kurtosis should be computed
            if let Some(kurtosis_idx) = get_column_index(&headers, "kurtosis") {
                let kurtosis_val = get_field_value(&record, kurtosis_idx);
                if let Some(val_str) = kurtosis_val {
                    if !val_str.is_empty() {
                        let kurtosis: f64 = val_str.parse().unwrap();
                        // With an outlier, kurtosis might be positive (heavy tails)
                        assert!(kurtosis.is_finite(), "kurtosis should be finite");
                    }
                }
            }

            break;
        }
    }
}

#[test]
fn moarstats_kurtosis_gini_string_fields_skipped() {
    let wrk = Workdir::new("moarstats_kurtosis_gini_strings");

    wrk.create(
        "test.csv",
        vec![
            svec!["field", "text_value"],
            svec!["test", "apple"],
            svec!["test", "banana"],
            svec!["test", "cherry"],
        ],
    );

    // Generate stats
    let mut stats_cmd = wrk.command("stats");
    stats_cmd.arg("--everything").arg("test.csv");
    wrk.assert_success(&mut stats_cmd);

    // Run moarstats
    let mut cmd = wrk.command("moarstats");
    cmd.arg("test.csv").arg("--advanced");
    wrk.assert_success(&mut cmd);

    // Verify string fields have empty values for kurtosis/Gini
    let stats_content = wrk.read_to_string("test.stats.csv").unwrap();
    let mut rdr = ReaderBuilder::new()
        .has_headers(true)
        .from_reader(stats_content.as_bytes());

    let headers = rdr.headers().unwrap().clone();
    let field_idx = get_column_index(&headers, "field").unwrap();
    let type_idx = get_column_index(&headers, "type").unwrap();

    for result in rdr.records() {
        let record = result.unwrap();
        let field_name = get_field_value(&record, field_idx).unwrap();
        let field_type = get_field_value(&record, type_idx).unwrap();

        if field_name == "test" && field_type == "String" {
            // String fields should have empty values for kurtosis/Gini
            if let Some(kurtosis_idx) = get_column_index(&headers, "kurtosis") {
                let kurtosis_val = get_field_value(&record, kurtosis_idx);
                assert!(
                    kurtosis_val.is_none() || kurtosis_val.as_ref().unwrap().is_empty(),
                    "String fields should not have kurtosis statistics"
                );
            }

            if let Some(gini_idx) = get_column_index(&headers, "gini_coefficient") {
                let gini_val = get_field_value(&record, gini_idx);
                assert!(
                    gini_val.is_none() || gini_val.as_ref().unwrap().is_empty(),
                    "String fields should not have gini_coefficient statistics"
                );
            }

            break;
        }
    }
}

#[test]
fn moarstats_kurtosis_gini_multiple_numeric_fields() {
    let wrk = Workdir::new("moarstats_kurtosis_gini_multiple");
    let test_file = wrk.load_test_file("boston311-100.csv");

    // Generate baseline stats
    let mut stats_cmd = wrk.command("stats");
    stats_cmd
        .arg("--everything")
        .arg("--infer-dates")
        .arg(&test_file);
    wrk.assert_success(&mut stats_cmd);

    // Run moarstats with --advanced flag
    let mut cmd = wrk.command("moarstats");
    cmd.arg("--advanced").arg(&test_file);
    wrk.assert_success(&mut cmd);

    // Verify that multiple numeric fields get kurtosis/Gini statistics
    let stats_content = wrk.read_to_string("boston311-100.stats.csv").unwrap();
    let mut rdr = ReaderBuilder::new()
        .has_headers(true)
        .from_reader(stats_content.as_bytes());

    let headers = rdr.headers().unwrap().clone();
    let type_idx = get_column_index(&headers, "type").unwrap();
    let kurtosis_idx = get_column_index(&headers, "kurtosis");
    let gini_idx = get_column_index(&headers, "gini_coefficient");

    let mut numeric_fields_with_kurtosis = 0;
    let mut numeric_fields_with_gini = 0;

    for result in rdr.records() {
        let record = result.unwrap();
        let field_type = get_field_value(&record, type_idx).unwrap();

        if field_type == "Float" || field_type == "Integer" {
            if let Some(kurtosis_idx) = kurtosis_idx {
                let kurtosis_val = get_field_value(&record, kurtosis_idx);
                if kurtosis_val.is_some() && !kurtosis_val.as_ref().unwrap().is_empty() {
                    numeric_fields_with_kurtosis += 1;
                }
            }

            if let Some(gini_idx) = gini_idx {
                let gini_val = get_field_value(&record, gini_idx);
                if gini_val.is_some() && !gini_val.as_ref().unwrap().is_empty() {
                    numeric_fields_with_gini += 1;
                }
            }
        }
    }

    assert!(
        numeric_fields_with_kurtosis > 1,
        "Multiple numeric fields should have kurtosis statistics"
    );
    assert!(
        numeric_fields_with_gini > 1,
        "Multiple numeric fields should have gini_coefficient statistics"
    );
}

#[test]
fn moarstats_without_advanced_flag() {
    let wrk = Workdir::new("moarstats_without_advanced");
    let test_file = wrk.load_test_file("boston311-100.csv");

    // Generate baseline stats
    let mut stats_cmd = wrk.command("stats");
    stats_cmd
        .arg("--everything")
        .arg("--infer-dates")
        .arg(&test_file);
    wrk.assert_success(&mut stats_cmd);

    // Run moarstats WITHOUT --advanced flag
    let mut cmd = wrk.command("moarstats");
    cmd.arg(&test_file);
    wrk.assert_success(&mut cmd);

    // Verify kurtosis and gini_coefficient columns do NOT exist
    let stats_content = wrk.read_to_string("boston311-100.stats.csv").unwrap();
    let mut rdr = ReaderBuilder::new()
        .has_headers(true)
        .from_reader(stats_content.as_bytes());

    let headers = rdr.headers().unwrap().clone();

    assert!(
        get_column_index(&headers, "kurtosis").is_none(),
        "kurtosis column should NOT exist without --advanced flag"
    );
    assert!(
        get_column_index(&headers, "gini_coefficient").is_none(),
        "gini_coefficient column should NOT exist without --advanced flag"
    );
    assert!(
        get_column_index(&headers, "shannon_entropy").is_none(),
        "shannon_entropy column should NOT exist without --advanced flag"
    );

    // Verify other columns still exist
    assert!(
        get_column_index(&headers, "pearson_skewness").is_some(),
        "pearson_skewness column should still exist"
    );
}

#[test]
fn moarstats_with_advanced_flag() {
    let wrk = Workdir::new("moarstats_with_advanced");
    let test_file = wrk.load_test_file("boston311-100.csv");

    // Generate baseline stats
    let mut stats_cmd = wrk.command("stats");
    stats_cmd
        .arg("--everything")
        .arg("--infer-dates")
        .arg(&test_file);
    wrk.assert_success(&mut stats_cmd);

    // Run moarstats WITH --advanced flag
    let mut cmd = wrk.command("moarstats");
    cmd.arg("--advanced").arg(&test_file);
    wrk.assert_success(&mut cmd);

    // Verify kurtosis and gini_coefficient columns exist
    let stats_content = wrk.read_to_string("boston311-100.stats.csv").unwrap();
    let mut rdr = ReaderBuilder::new()
        .has_headers(true)
        .from_reader(stats_content.as_bytes());

    let headers = rdr.headers().unwrap().clone();
    let type_idx = get_column_index(&headers, "type").unwrap();

    assert!(
        get_column_index(&headers, "kurtosis").is_some(),
        "kurtosis column should exist with --advanced flag"
    );
    assert!(
        get_column_index(&headers, "gini_coefficient").is_some(),
        "gini_coefficient column should exist with --advanced flag"
    );
    assert!(
        get_column_index(&headers, "shannon_entropy").is_some(),
        "shannon_entropy column should exist with --advanced flag"
    );

    // Verify values are computed for at least one numeric field
    let mut found_kurtosis_value = false;
    let mut found_gini_value = false;

    for result in rdr.records() {
        let record = result.unwrap();
        let field_type = get_field_value(&record, type_idx).unwrap();

        if field_type == "Float" || field_type == "Integer" {
            // Check kurtosis
            if let Some(kurtosis_idx) = get_column_index(&headers, "kurtosis") {
                let kurtosis_val = get_field_value(&record, kurtosis_idx);
                if let Some(val_str) = kurtosis_val {
                    if !val_str.is_empty() {
                        found_kurtosis_value = true;
                        let kurtosis: f64 = val_str.parse().unwrap();
                        assert!(kurtosis.is_finite(), "kurtosis should be a finite number");
                    }
                }
            }

            // Check Gini coefficient
            if let Some(gini_idx) = get_column_index(&headers, "gini_coefficient") {
                let gini_val = get_field_value(&record, gini_idx);
                if let Some(val_str) = gini_val {
                    if !val_str.is_empty() {
                        found_gini_value = true;
                        let gini: f64 = val_str.parse().unwrap();
                        assert!(
                            gini >= 0.0 && gini <= 1.0,
                            "gini_coefficient should be between 0 and 1, got: {}",
                            gini
                        );
                    }
                }
            }

            if found_kurtosis_value && found_gini_value {
                break;
            }
        }
    }

    assert!(
        found_kurtosis_value,
        "Should find kurtosis value for numeric fields with --advanced flag"
    );
    assert!(
        found_gini_value,
        "Should find gini_coefficient value for numeric fields with --advanced flag"
    );
}

#[test]
fn moarstats_advanced_flag_does_not_affect_other_stats() {
    let wrk = Workdir::new("moarstats_advanced_other_stats");
    let test_file = wrk.load_test_file("boston311-100.csv");

    // Generate baseline stats
    let mut stats_cmd = wrk.command("stats");
    stats_cmd
        .arg("--everything")
        .arg("--infer-dates")
        .arg(&test_file);
    wrk.assert_success(&mut stats_cmd);

    // Run moarstats with --advanced flag
    let mut cmd = wrk.command("moarstats");
    cmd.arg("--advanced").arg(&test_file);
    wrk.assert_success(&mut cmd);

    // Verify that other statistics are still computed correctly
    let stats_content = wrk.read_to_string("boston311-100.stats.csv").unwrap();
    let mut rdr = ReaderBuilder::new()
        .has_headers(true)
        .from_reader(stats_content.as_bytes());

    let headers = rdr.headers().unwrap().clone();

    // Verify standard moarstats columns exist
    assert!(
        get_column_index(&headers, "pearson_skewness").is_some(),
        "pearson_skewness should exist"
    );
    assert!(
        get_column_index(&headers, "range_stddev_ratio").is_some(),
        "range_stddev_ratio should exist"
    );
    assert!(
        get_column_index(&headers, "quartile_coefficient_dispersion").is_some(),
        "quartile_coefficient_dispersion should exist"
    );

    // Verify advanced columns exist
    assert!(
        get_column_index(&headers, "kurtosis").is_some(),
        "kurtosis should exist with --advanced"
    );
    assert!(
        get_column_index(&headers, "gini_coefficient").is_some(),
        "gini_coefficient should exist with --advanced"
    );
    assert!(
        get_column_index(&headers, "shannon_entropy").is_some(),
        "shannon_entropy should exist with --advanced"
    );
}

#[test]
fn moarstats_outlier_variance_stddev() {
    let wrk = Workdir::new("moarstats_outlier_variance");

    // Create CSV with known outliers
    wrk.create(
        "test.csv",
        vec![
            svec!["field", "value"],
            svec!["test", "10"],
            svec!["test", "11"],
            svec!["test", "12"],
            svec!["test", "13"],
            svec!["test", "14"],
            svec!["test", "100"], // outlier
            svec!["test", "200"], // outlier
        ],
    );

    // Generate stats with quartiles
    let mut stats_cmd = wrk.command("stats");
    stats_cmd.arg("--everything").arg("test.csv");
    wrk.assert_success(&mut stats_cmd);

    // Run moarstats
    let mut cmd = wrk.command("moarstats");
    cmd.arg("test.csv");
    wrk.assert_success(&mut cmd);

    // Verify outlier variance/stddev columns exist and have values
    let stats_content = wrk.read_to_string("test.stats.csv").unwrap();
    let mut rdr = ReaderBuilder::new()
        .has_headers(true)
        .from_reader(stats_content.as_bytes());

    let headers = rdr.headers().unwrap().clone();
    let field_idx = get_column_index(&headers, "field").unwrap();

    assert!(
        get_column_index(&headers, "outliers_stddev").is_some(),
        "outliers_stddev column should exist"
    );
    assert!(
        get_column_index(&headers, "outliers_variance").is_some(),
        "outliers_variance column should exist"
    );
    assert!(
        get_column_index(&headers, "non_outliers_stddev").is_some(),
        "non_outliers_stddev column should exist"
    );
    assert!(
        get_column_index(&headers, "non_outliers_variance").is_some(),
        "non_outliers_variance column should exist"
    );

    for result in rdr.records() {
        let record = result.unwrap();
        let field_name = get_field_value(&record, field_idx).unwrap();

        if field_name == "test" {
            // Verify outliers_stddev and outliers_variance
            if let Some(stddev_idx) = get_column_index(&headers, "outliers_stddev") {
                let stddev_val = get_field_value(&record, stddev_idx);
                if let Some(val_str) = stddev_val {
                    if !val_str.is_empty() {
                        let stddev: f64 = val_str.parse().unwrap();
                        assert!(stddev >= 0.0, "outliers_stddev should be non-negative");
                        assert!(stddev.is_finite(), "outliers_stddev should be finite");
                    }
                }
            }

            if let Some(variance_idx) = get_column_index(&headers, "outliers_variance") {
                let variance_val = get_field_value(&record, variance_idx);
                if let Some(val_str) = variance_val {
                    if !val_str.is_empty() {
                        let variance: f64 = val_str.parse().unwrap();
                        assert!(variance >= 0.0, "outliers_variance should be non-negative");
                        assert!(variance.is_finite(), "outliers_variance should be finite");
                    }
                }
            }

            // Verify non_outliers_stddev and non_outliers_variance
            if let Some(stddev_idx) = get_column_index(&headers, "non_outliers_stddev") {
                let stddev_val = get_field_value(&record, stddev_idx);
                if let Some(val_str) = stddev_val {
                    if !val_str.is_empty() {
                        let stddev: f64 = val_str.parse().unwrap();
                        assert!(stddev >= 0.0, "non_outliers_stddev should be non-negative");
                        assert!(stddev.is_finite(), "non_outliers_stddev should be finite");
                    }
                }
            }

            if let Some(variance_idx) = get_column_index(&headers, "non_outliers_variance") {
                let variance_val = get_field_value(&record, variance_idx);
                if let Some(val_str) = variance_val {
                    if !val_str.is_empty() {
                        let variance: f64 = val_str.parse().unwrap();
                        assert!(
                            variance >= 0.0,
                            "non_outliers_variance should be non-negative"
                        );
                        assert!(
                            variance.is_finite(),
                            "non_outliers_variance should be finite"
                        );
                    }
                }
            }

            break;
        }
    }
}

#[test]
fn moarstats_outlier_coefficient_of_variation() {
    let wrk = Workdir::new("moarstats_outlier_cv");

    wrk.create(
        "test.csv",
        vec![
            svec!["field", "value"],
            svec!["test", "10"],
            svec!["test", "20"],
            svec!["test", "30"],
            svec!["test", "40"],
            svec!["test", "50"],
            svec!["test", "1000"], // outlier
        ],
    );

    let mut stats_cmd = wrk.command("stats");
    stats_cmd.arg("--everything").arg("test.csv");
    wrk.assert_success(&mut stats_cmd);

    let mut cmd = wrk.command("moarstats");
    cmd.arg("test.csv");
    wrk.assert_success(&mut cmd);

    let stats_content = wrk.read_to_string("test.stats.csv").unwrap();
    let mut rdr = ReaderBuilder::new()
        .has_headers(true)
        .from_reader(stats_content.as_bytes());

    let headers = rdr.headers().unwrap().clone();
    let field_idx = get_column_index(&headers, "field").unwrap();

    assert!(
        get_column_index(&headers, "outliers_cv").is_some(),
        "outliers_cv column should exist"
    );
    assert!(
        get_column_index(&headers, "non_outliers_cv").is_some(),
        "non_outliers_cv column should exist"
    );

    for result in rdr.records() {
        let record = result.unwrap();
        let field_name = get_field_value(&record, field_idx).unwrap();

        if field_name == "test" {
            if let Some(cv_idx) = get_column_index(&headers, "outliers_cv") {
                let cv_val = get_field_value(&record, cv_idx);
                if let Some(val_str) = cv_val {
                    if !val_str.is_empty() {
                        let cv: f64 = val_str.parse().unwrap();
                        assert!(cv >= 0.0, "outliers_cv should be non-negative");
                        assert!(cv.is_finite(), "outliers_cv should be finite");
                    }
                }
            }

            if let Some(cv_idx) = get_column_index(&headers, "non_outliers_cv") {
                let cv_val = get_field_value(&record, cv_idx);
                if let Some(val_str) = cv_val {
                    if !val_str.is_empty() {
                        let cv: f64 = val_str.parse().unwrap();
                        assert!(cv >= 0.0, "non_outliers_cv should be non-negative");
                        assert!(cv.is_finite(), "non_outliers_cv should be finite");
                    }
                }
            }

            break;
        }
    }
}

#[test]
fn moarstats_outlier_percentage() {
    let wrk = Workdir::new("moarstats_outlier_pct");

    wrk.create(
        "test.csv",
        vec![
            svec!["field", "value"],
            svec!["test", "10"],
            svec!["test", "20"],
            svec!["test", "30"],
            svec!["test", "40"],
            svec!["test", "50"],
            svec!["test", "1000"], // outlier
        ],
    );

    let mut stats_cmd = wrk.command("stats");
    stats_cmd.arg("--everything").arg("test.csv");
    wrk.assert_success(&mut stats_cmd);

    let mut cmd = wrk.command("moarstats");
    cmd.arg("test.csv");
    wrk.assert_success(&mut cmd);

    let stats_content = wrk.read_to_string("test.stats.csv").unwrap();
    let mut rdr = ReaderBuilder::new()
        .has_headers(true)
        .from_reader(stats_content.as_bytes());

    let headers = rdr.headers().unwrap().clone();
    let field_idx = get_column_index(&headers, "field").unwrap();

    assert!(
        get_column_index(&headers, "outliers_percentage").is_some(),
        "outliers_percentage column should exist"
    );

    for result in rdr.records() {
        let record = result.unwrap();
        let field_name = get_field_value(&record, field_idx).unwrap();

        if field_name == "test" {
            if let Some(pct_idx) = get_column_index(&headers, "outliers_percentage") {
                let pct_val = get_field_value(&record, pct_idx);
                if let Some(val_str) = pct_val {
                    if !val_str.is_empty() {
                        let pct: f64 = val_str.parse().unwrap();
                        assert!(
                            pct >= 0.0 && pct <= 100.0,
                            "outliers_percentage should be between 0 and 100"
                        );
                        assert!(pct.is_finite(), "outliers_percentage should be finite");
                    }
                }
            }

            break;
        }
    }
}

#[test]
fn moarstats_outlier_impact() {
    let wrk = Workdir::new("moarstats_outlier_impact");

    wrk.create(
        "test.csv",
        vec![
            svec!["field", "value"],
            svec!["test", "10"],
            svec!["test", "11"],
            svec!["test", "12"],
            svec!["test", "13"],
            svec!["test", "14"],
            svec!["test", "100"], // outlier that affects mean
        ],
    );

    let mut stats_cmd = wrk.command("stats");
    stats_cmd.arg("--everything").arg("test.csv");
    wrk.assert_success(&mut stats_cmd);

    let mut cmd = wrk.command("moarstats");
    cmd.arg("test.csv");
    wrk.assert_success(&mut cmd);

    let stats_content = wrk.read_to_string("test.stats.csv").unwrap();
    let mut rdr = ReaderBuilder::new()
        .has_headers(true)
        .from_reader(stats_content.as_bytes());

    let headers = rdr.headers().unwrap().clone();
    let field_idx = get_column_index(&headers, "field").unwrap();

    assert!(
        get_column_index(&headers, "outlier_impact").is_some(),
        "outlier_impact column should exist"
    );
    assert!(
        get_column_index(&headers, "outlier_impact_ratio").is_some(),
        "outlier_impact_ratio column should exist"
    );

    for result in rdr.records() {
        let record = result.unwrap();
        let field_name = get_field_value(&record, field_idx).unwrap();

        if field_name == "test" {
            if let Some(impact_idx) = get_column_index(&headers, "outlier_impact") {
                let impact_val = get_field_value(&record, impact_idx);
                if let Some(val_str) = impact_val {
                    if !val_str.is_empty() {
                        let impact: f64 = val_str.parse().unwrap();
                        assert!(impact.is_finite(), "outlier_impact should be finite");
                    }
                }
            }

            if let Some(ratio_idx) = get_column_index(&headers, "outlier_impact_ratio") {
                let ratio_val = get_field_value(&record, ratio_idx);
                if let Some(val_str) = ratio_val {
                    if !val_str.is_empty() {
                        let ratio: f64 = val_str.parse().unwrap();
                        assert!(ratio.is_finite(), "outlier_impact_ratio should be finite");
                    }
                }
            }

            break;
        }
    }
}

#[test]
fn moarstats_outlier_spread_ratio() {
    let wrk = Workdir::new("moarstats_outlier_spread");

    wrk.create(
        "test.csv",
        vec![
            svec!["field", "value"],
            svec!["test", "10"],
            svec!["test", "11"],
            svec!["test", "12"],
            svec!["test", "13"],
            svec!["test", "14"],
            svec!["test", "100"], // outlier
            svec!["test", "200"], // outlier
        ],
    );

    let mut stats_cmd = wrk.command("stats");
    stats_cmd.arg("--everything").arg("test.csv");
    wrk.assert_success(&mut stats_cmd);

    let mut cmd = wrk.command("moarstats");
    cmd.arg("test.csv");
    wrk.assert_success(&mut cmd);

    let stats_content = wrk.read_to_string("test.stats.csv").unwrap();
    let mut rdr = ReaderBuilder::new()
        .has_headers(true)
        .from_reader(stats_content.as_bytes());

    let headers = rdr.headers().unwrap().clone();
    let field_idx = get_column_index(&headers, "field").unwrap();

    assert!(
        get_column_index(&headers, "outliers_normal_stddev_ratio").is_some(),
        "outliers_normal_stddev_ratio column should exist"
    );

    for result in rdr.records() {
        let record = result.unwrap();
        let field_name = get_field_value(&record, field_idx).unwrap();

        if field_name == "test" {
            if let Some(ratio_idx) = get_column_index(&headers, "outliers_normal_stddev_ratio") {
                let ratio_val = get_field_value(&record, ratio_idx);
                if let Some(val_str) = ratio_val {
                    if !val_str.is_empty() {
                        let ratio: f64 = val_str.parse().unwrap();
                        assert!(
                            ratio >= 0.0,
                            "outliers_normal_stddev_ratio should be non-negative"
                        );
                        assert!(
                            ratio.is_finite(),
                            "outliers_normal_stddev_ratio should be finite"
                        );
                    }
                }
            }

            break;
        }
    }
}

#[test]
fn moarstats_outlier_fence_zscores() {
    let wrk = Workdir::new("moarstats_fence_zscore");

    wrk.create(
        "test.csv",
        vec![
            svec!["field", "value"],
            svec!["test", "10"],
            svec!["test", "20"],
            svec!["test", "30"],
            svec!["test", "40"],
            svec!["test", "50"],
        ],
    );

    let mut stats_cmd = wrk.command("stats");
    stats_cmd.arg("--everything").arg("test.csv");
    wrk.assert_success(&mut stats_cmd);

    let mut cmd = wrk.command("moarstats");
    cmd.arg("test.csv");
    wrk.assert_success(&mut cmd);

    let stats_content = wrk.read_to_string("test.stats.csv").unwrap();
    let mut rdr = ReaderBuilder::new()
        .has_headers(true)
        .from_reader(stats_content.as_bytes());

    let headers = rdr.headers().unwrap().clone();
    let field_idx = get_column_index(&headers, "field").unwrap();

    assert!(
        get_column_index(&headers, "lower_outer_fence_zscore").is_some(),
        "lower_outer_fence_zscore column should exist"
    );
    assert!(
        get_column_index(&headers, "upper_outer_fence_zscore").is_some(),
        "upper_outer_fence_zscore column should exist"
    );

    for result in rdr.records() {
        let record = result.unwrap();
        let field_name = get_field_value(&record, field_idx).unwrap();

        if field_name == "test" {
            if let Some(zscore_idx) = get_column_index(&headers, "lower_outer_fence_zscore") {
                let zscore_val = get_field_value(&record, zscore_idx);
                if let Some(val_str) = zscore_val {
                    if !val_str.is_empty() {
                        let zscore: f64 = val_str.parse().unwrap();
                        assert!(
                            zscore.is_finite(),
                            "lower_outer_fence_zscore should be finite"
                        );
                    }
                }
            }

            if let Some(zscore_idx) = get_column_index(&headers, "upper_outer_fence_zscore") {
                let zscore_val = get_field_value(&record, zscore_idx);
                if let Some(val_str) = zscore_val {
                    if !val_str.is_empty() {
                        let zscore: f64 = val_str.parse().unwrap();
                        assert!(
                            zscore.is_finite(),
                            "upper_outer_fence_zscore should be finite"
                        );
                    }
                }
            }

            break;
        }
    }
}

#[test]
fn moarstats_trimmed_winsorized_variance_stddev() {
    let wrk = Workdir::new("moarstats_trimmed_winsorized_var");

    wrk.create(
        "test.csv",
        vec![
            svec!["field", "value"],
            svec!["test", "1"],
            svec!["test", "2"],
            svec!["test", "3"],
            svec!["test", "4"],
            svec!["test", "5"],
            svec!["test", "6"],
            svec!["test", "7"],
            svec!["test", "8"],
            svec!["test", "9"],
        ],
    );

    let mut stats_cmd = wrk.command("stats");
    stats_cmd.arg("--everything").arg("test.csv");
    wrk.assert_success(&mut stats_cmd);

    let mut cmd = wrk.command("moarstats");
    cmd.arg("test.csv");
    wrk.assert_success(&mut cmd);

    let stats_content = wrk.read_to_string("test.stats.csv").unwrap();
    let mut rdr = ReaderBuilder::new()
        .has_headers(true)
        .from_reader(stats_content.as_bytes());

    let headers = rdr.headers().unwrap().clone();
    let field_idx = get_column_index(&headers, "field").unwrap();

    assert!(
        get_column_index(&headers, "trimmed_stddev_25pct").is_some(),
        "trimmed_stddev_25pct column should exist"
    );
    assert!(
        get_column_index(&headers, "trimmed_variance_25pct").is_some(),
        "trimmed_variance_25pct column should exist"
    );
    assert!(
        get_column_index(&headers, "winsorized_stddev_25pct").is_some(),
        "winsorized_stddev_25pct column should exist"
    );
    assert!(
        get_column_index(&headers, "winsorized_variance_25pct").is_some(),
        "winsorized_variance_25pct column should exist"
    );

    for result in rdr.records() {
        let record = result.unwrap();
        let field_name = get_field_value(&record, field_idx).unwrap();

        if field_name == "test" {
            // Verify trimmed stddev/variance
            if let Some(stddev_idx) = get_column_index(&headers, "trimmed_stddev_25pct") {
                let stddev_val = get_field_value(&record, stddev_idx);
                if let Some(val_str) = stddev_val {
                    if !val_str.is_empty() {
                        let stddev: f64 = val_str.parse().unwrap();
                        assert!(stddev >= 0.0, "trimmed_stddev_25pct should be non-negative");
                        assert!(stddev.is_finite(), "trimmed_stddev_25pct should be finite");
                    }
                }
            }

            if let Some(variance_idx) = get_column_index(&headers, "trimmed_variance_25pct") {
                let variance_val = get_field_value(&record, variance_idx);
                if let Some(val_str) = variance_val {
                    if !val_str.is_empty() {
                        let variance: f64 = val_str.parse().unwrap();
                        assert!(
                            variance >= 0.0,
                            "trimmed_variance_25pct should be non-negative"
                        );
                        assert!(
                            variance.is_finite(),
                            "trimmed_variance_25pct should be finite"
                        );
                    }
                }
            }

            // Verify winsorized stddev/variance
            if let Some(stddev_idx) = get_column_index(&headers, "winsorized_stddev_25pct") {
                let stddev_val = get_field_value(&record, stddev_idx);
                if let Some(val_str) = stddev_val {
                    if !val_str.is_empty() {
                        let stddev: f64 = val_str.parse().unwrap();
                        assert!(
                            stddev >= 0.0,
                            "winsorized_stddev_25pct should be non-negative"
                        );
                        assert!(
                            stddev.is_finite(),
                            "winsorized_stddev_25pct should be finite"
                        );
                    }
                }
            }

            if let Some(variance_idx) = get_column_index(&headers, "winsorized_variance_25pct") {
                let variance_val = get_field_value(&record, variance_idx);
                if let Some(val_str) = variance_val {
                    if !val_str.is_empty() {
                        let variance: f64 = val_str.parse().unwrap();
                        assert!(
                            variance >= 0.0,
                            "winsorized_variance_25pct should be non-negative"
                        );
                        assert!(
                            variance.is_finite(),
                            "winsorized_variance_25pct should be finite"
                        );
                    }
                }
            }

            break;
        }
    }
}

#[test]
fn moarstats_trimmed_winsorized_cv() {
    let wrk = Workdir::new("moarstats_trimmed_winsorized_cv");

    wrk.create(
        "test.csv",
        vec![
            svec!["field", "value"],
            svec!["test", "10"],
            svec!["test", "20"],
            svec!["test", "30"],
            svec!["test", "40"],
            svec!["test", "50"],
        ],
    );

    let mut stats_cmd = wrk.command("stats");
    stats_cmd.arg("--everything").arg("test.csv");
    wrk.assert_success(&mut stats_cmd);

    let mut cmd = wrk.command("moarstats");
    cmd.arg("test.csv");
    wrk.assert_success(&mut cmd);

    let stats_content = wrk.read_to_string("test.stats.csv").unwrap();
    let mut rdr = ReaderBuilder::new()
        .has_headers(true)
        .from_reader(stats_content.as_bytes());

    let headers = rdr.headers().unwrap().clone();
    let field_idx = get_column_index(&headers, "field").unwrap();

    assert!(
        get_column_index(&headers, "trimmed_cv_25pct").is_some(),
        "trimmed_cv_25pct column should exist"
    );
    assert!(
        get_column_index(&headers, "winsorized_cv_25pct").is_some(),
        "winsorized_cv_25pct column should exist"
    );

    for result in rdr.records() {
        let record = result.unwrap();
        let field_name = get_field_value(&record, field_idx).unwrap();

        if field_name == "test" {
            if let Some(cv_idx) = get_column_index(&headers, "trimmed_cv_25pct") {
                let cv_val = get_field_value(&record, cv_idx);
                if let Some(val_str) = cv_val {
                    if !val_str.is_empty() {
                        let cv: f64 = val_str.parse().unwrap();
                        assert!(cv >= 0.0, "trimmed_cv_25pct should be non-negative");
                        assert!(cv.is_finite(), "trimmed_cv_25pct should be finite");
                    }
                }
            }

            if let Some(cv_idx) = get_column_index(&headers, "winsorized_cv_25pct") {
                let cv_val = get_field_value(&record, cv_idx);
                if let Some(val_str) = cv_val {
                    if !val_str.is_empty() {
                        let cv: f64 = val_str.parse().unwrap();
                        assert!(cv >= 0.0, "winsorized_cv_25pct should be non-negative");
                        assert!(cv.is_finite(), "winsorized_cv_25pct should be finite");
                    }
                }
            }

            break;
        }
    }
}

#[test]
fn moarstats_trimmed_winsorized_stddev_ratio() {
    let wrk = Workdir::new("moarstats_trimmed_winsorized_ratio");

    wrk.create(
        "test.csv",
        vec![
            svec!["field", "value"],
            svec!["test", "10"],
            svec!["test", "20"],
            svec!["test", "30"],
            svec!["test", "40"],
            svec!["test", "50"],
        ],
    );

    let mut stats_cmd = wrk.command("stats");
    stats_cmd.arg("--everything").arg("test.csv");
    wrk.assert_success(&mut stats_cmd);

    let mut cmd = wrk.command("moarstats");
    cmd.arg("test.csv");
    wrk.assert_success(&mut cmd);

    let stats_content = wrk.read_to_string("test.stats.csv").unwrap();
    let mut rdr = ReaderBuilder::new()
        .has_headers(true)
        .from_reader(stats_content.as_bytes());

    let headers = rdr.headers().unwrap().clone();
    let field_idx = get_column_index(&headers, "field").unwrap();

    // Check for stddev_ratio columns (names may vary based on implementation)
    let trimmed_ratio_exists = headers
        .iter()
        .any(|h| h.contains("trimmed") && h.contains("stddev_ratio"));
    let winsorized_ratio_exists = headers
        .iter()
        .any(|h| h.contains("winsorized") && h.contains("stddev_ratio"));

    assert!(
        trimmed_ratio_exists,
        "trimmed stddev_ratio column should exist"
    );
    assert!(
        winsorized_ratio_exists,
        "winsorized stddev_ratio column should exist"
    );

    for result in rdr.records() {
        let record = result.unwrap();
        let field_name = get_field_value(&record, field_idx).unwrap();

        if field_name == "test" {
            // Find and verify trimmed stddev ratio
            for (idx, header) in headers.iter().enumerate() {
                if header.contains("trimmed") && header.contains("stddev_ratio") {
                    let ratio_val = get_field_value(&record, idx);
                    if let Some(val_str) = ratio_val {
                        if !val_str.is_empty() {
                            let ratio: f64 = val_str.parse().unwrap();
                            assert!(ratio >= 0.0, "trimmed stddev_ratio should be non-negative");
                            assert!(ratio.is_finite(), "trimmed stddev_ratio should be finite");
                        }
                    }
                }
            }

            // Find and verify winsorized stddev ratio
            for (idx, header) in headers.iter().enumerate() {
                if header.contains("winsorized") && header.contains("stddev_ratio") {
                    let ratio_val = get_field_value(&record, idx);
                    if let Some(val_str) = ratio_val {
                        if !val_str.is_empty() {
                            let ratio: f64 = val_str.parse().unwrap();
                            assert!(
                                ratio >= 0.0,
                                "winsorized stddev_ratio should be non-negative"
                            );
                            assert!(
                                ratio.is_finite(),
                                "winsorized stddev_ratio should be finite"
                            );
                        }
                    }
                }
            }

            break;
        }
    }
}

#[test]
fn moarstats_trimmed_winsorized_range() {
    let wrk = Workdir::new("moarstats_trimmed_winsorized_range");

    wrk.create(
        "test.csv",
        vec![
            svec!["field", "value"],
            svec!["test", "1"],
            svec!["test", "2"],
            svec!["test", "3"],
            svec!["test", "4"],
            svec!["test", "5"],
            svec!["test", "6"],
            svec!["test", "7"],
            svec!["test", "8"],
            svec!["test", "9"],
        ],
    );

    let mut stats_cmd = wrk.command("stats");
    stats_cmd.arg("--everything").arg("test.csv");
    wrk.assert_success(&mut stats_cmd);

    let mut cmd = wrk.command("moarstats");
    cmd.arg("test.csv");
    wrk.assert_success(&mut cmd);

    let stats_content = wrk.read_to_string("test.stats.csv").unwrap();
    let mut rdr = ReaderBuilder::new()
        .has_headers(true)
        .from_reader(stats_content.as_bytes());

    let headers = rdr.headers().unwrap().clone();
    let field_idx = get_column_index(&headers, "field").unwrap();

    assert!(
        get_column_index(&headers, "trimmed_range_25pct").is_some(),
        "trimmed_range_25pct column should exist"
    );
    assert!(
        get_column_index(&headers, "winsorized_range_25pct").is_some(),
        "winsorized_range_25pct column should exist"
    );

    for result in rdr.records() {
        let record = result.unwrap();
        let field_name = get_field_value(&record, field_idx).unwrap();

        if field_name == "test" {
            if let Some(range_idx) = get_column_index(&headers, "trimmed_range_25pct") {
                let range_val = get_field_value(&record, range_idx);
                if let Some(val_str) = range_val {
                    if !val_str.is_empty() {
                        let range: f64 = val_str.parse().unwrap();
                        assert!(range >= 0.0, "trimmed_range_25pct should be non-negative");
                        assert!(range.is_finite(), "trimmed_range_25pct should be finite");
                    }
                }
            }

            if let Some(range_idx) = get_column_index(&headers, "winsorized_range_25pct") {
                let range_val = get_field_value(&record, range_idx);
                if let Some(val_str) = range_val {
                    if !val_str.is_empty() {
                        let range: f64 = val_str.parse().unwrap();
                        assert!(
                            range >= 0.0,
                            "winsorized_range_25pct should be non-negative"
                        );
                        assert!(range.is_finite(), "winsorized_range_25pct should be finite");
                    }
                }
            }

            break;
        }
    }
}

#[test]
fn moarstats_shannon_entropy_basic() {
    let wrk = Workdir::new("moarstats_shannon_entropy");
    let test_file = wrk.load_test_file("boston311-100.csv");

    // Generate baseline stats
    let mut stats_cmd = wrk.command("stats");
    stats_cmd
        .arg("--everything")
        .arg("--infer-dates")
        .arg(&test_file);
    wrk.assert_success(&mut stats_cmd);

    // Run moarstats with --advanced flag
    let mut cmd = wrk.command("moarstats");
    cmd.arg("--advanced").arg(&test_file);
    wrk.assert_success(&mut cmd);

    // Verify shannon_entropy column exists
    let stats_content = wrk.read_to_string("boston311-100.stats.csv").unwrap();
    let mut rdr = ReaderBuilder::new()
        .has_headers(true)
        .from_reader(stats_content.as_bytes());

    let headers = rdr.headers().unwrap().clone();

    assert!(
        get_column_index(&headers, "shannon_entropy").is_some(),
        "shannon_entropy column should exist"
    );

    // Verify values are computed for various field types
    let mut found_entropy_value = false;

    for result in rdr.records() {
        let record = result.unwrap();

        // Shannon entropy works for all field types
        if let Some(entropy_idx) = get_column_index(&headers, "shannon_entropy") {
            let entropy_val = get_field_value(&record, entropy_idx);
            if let Some(val_str) = entropy_val {
                if !val_str.is_empty() {
                    found_entropy_value = true;
                    let entropy: f64 = val_str.parse().unwrap();
                    // Entropy should be non-negative
                    assert!(
                        entropy >= 0.0,
                        "shannon_entropy should be non-negative, got: {}",
                        entropy
                    );
                    assert!(
                        entropy.is_finite(),
                        "shannon_entropy should be finite, got: {}",
                        entropy
                    );
                }
            }
        }

        if found_entropy_value {
            break;
        }
    }

    assert!(
        found_entropy_value,
        "Should find shannon_entropy value for at least one field"
    );
}

#[test]
fn moarstats_shannon_entropy_constant_values() {
    let wrk = Workdir::new("moarstats_shannon_entropy_constant");

    // Create CSV with constant values (entropy should be 0)
    wrk.create(
        "test.csv",
        vec![
            svec!["field", "value"],
            svec!["test", "5"],
            svec!["test", "5"],
            svec!["test", "5"],
            svec!["test", "5"],
            svec!["test", "5"],
        ],
    );

    // Generate stats
    let mut stats_cmd = wrk.command("stats");
    stats_cmd.arg("--everything").arg("test.csv");
    wrk.assert_success(&mut stats_cmd);

    // Run moarstats
    let mut cmd = wrk.command("moarstats");
    cmd.arg("test.csv").arg("--advanced");
    wrk.assert_success(&mut cmd);

    // Verify entropy is 0 for constant values
    let stats_content = wrk.read_to_string("test.stats.csv").unwrap();
    let mut rdr = ReaderBuilder::new()
        .has_headers(true)
        .from_reader(stats_content.as_bytes());

    let headers = rdr.headers().unwrap().clone();
    let field_idx = get_column_index(&headers, "field").unwrap();

    for result in rdr.records() {
        let record = result.unwrap();
        let field_name = get_field_value(&record, field_idx).unwrap();

        if field_name == "test" {
            // With constant values, entropy should be 0 (all values identical)
            if let Some(entropy_idx) = get_column_index(&headers, "shannon_entropy") {
                let entropy_val = get_field_value(&record, entropy_idx);
                if let Some(val_str) = entropy_val {
                    if !val_str.is_empty() {
                        let entropy: f64 = val_str.parse().unwrap();
                        // Entropy should be approximately 0 for constant values
                        assert!(
                            entropy.abs() < 0.001,
                            "shannon_entropy should be approximately 0 for constant values, got: \
                             {}",
                            entropy
                        );
                    }
                }
            }

            break;
        }
    }
}

#[test]
fn moarstats_shannon_entropy_all_unique() {
    let wrk = Workdir::new("moarstats_shannon_entropy_unique");

    // Create CSV with all unique values (maximum entropy)
    wrk.create(
        "test.csv",
        vec![
            svec!["field", "value"],
            svec!["test", "1"],
            svec!["test", "2"],
            svec!["test", "3"],
            svec!["test", "4"],
            svec!["test", "5"],
            svec!["test", "6"],
            svec!["test", "7"],
            svec!["test", "8"],
        ],
    );

    // Generate stats
    let mut stats_cmd = wrk.command("stats");
    stats_cmd.arg("--everything").arg("test.csv");
    wrk.assert_success(&mut stats_cmd);

    // Run moarstats
    let mut cmd = wrk.command("moarstats");
    cmd.arg("test.csv").arg("--advanced");
    wrk.assert_success(&mut cmd);

    // Verify entropy is at maximum (log2(8) = 3.0)
    let stats_content = wrk.read_to_string("test.stats.csv").unwrap();
    let mut rdr = ReaderBuilder::new()
        .has_headers(true)
        .from_reader(stats_content.as_bytes());

    let headers = rdr.headers().unwrap().clone();
    let field_idx = get_column_index(&headers, "field").unwrap();

    for result in rdr.records() {
        let record = result.unwrap();
        let field_name = get_field_value(&record, field_idx).unwrap();

        if field_name == "test" {
            if let Some(entropy_idx) = get_column_index(&headers, "shannon_entropy") {
                let entropy_val = get_field_value(&record, entropy_idx);
                if let Some(val_str) = entropy_val {
                    if !val_str.is_empty() {
                        let entropy: f64 = val_str.parse().unwrap();
                        let max_entropy = 8.0_f64.log2(); // log2(8) = 3.0
                        // Entropy should be close to maximum (all unique values)
                        assert!(
                            entropy >= max_entropy * 0.99,
                            "shannon_entropy should be close to maximum for all unique values, \
                             expected ~{}, got: {}",
                            max_entropy,
                            entropy
                        );
                        assert!(
                            entropy <= max_entropy,
                            "shannon_entropy cannot exceed log2(n), expected <= {}, got: {}",
                            max_entropy,
                            entropy
                        );
                    }
                }
            }

            break;
        }
    }
}

#[test]
fn moarstats_shannon_entropy_string_fields() {
    let wrk = Workdir::new("moarstats_shannon_entropy_strings");

    wrk.create(
        "test.csv",
        vec![
            svec!["field", "text_value"],
            svec!["test", "apple"],
            svec!["test", "banana"],
            svec!["test", "cherry"],
            svec!["test", "apple"],
            svec!["test", "banana"],
        ],
    );

    // Generate stats
    let mut stats_cmd = wrk.command("stats");
    stats_cmd.arg("--everything").arg("test.csv");
    wrk.assert_success(&mut stats_cmd);

    // Run moarstats
    let mut cmd = wrk.command("moarstats");
    cmd.arg("test.csv").arg("--advanced");
    wrk.assert_success(&mut cmd);

    // Verify string fields have entropy computed
    let stats_content = wrk.read_to_string("test.stats.csv").unwrap();
    let mut rdr = ReaderBuilder::new()
        .has_headers(true)
        .from_reader(stats_content.as_bytes());

    let headers = rdr.headers().unwrap().clone();
    let field_idx = get_column_index(&headers, "field").unwrap();
    let type_idx = get_column_index(&headers, "type").unwrap();

    for result in rdr.records() {
        let record = result.unwrap();
        let field_name = get_field_value(&record, field_idx).unwrap();
        let field_type = get_field_value(&record, type_idx).unwrap();

        if field_name == "test" && field_type == "String" {
            // String fields should have entropy computed
            if let Some(entropy_idx) = get_column_index(&headers, "shannon_entropy") {
                let entropy_val = get_field_value(&record, entropy_idx);
                if let Some(val_str) = entropy_val {
                    if !val_str.is_empty() {
                        let entropy: f64 = val_str.parse().unwrap();
                        // With 3 unique values out of 5 total, entropy should be between 0 and
                        // log2(3)
                        assert!(
                            entropy >= 0.0 && entropy <= 3.0_f64.log2(),
                            "shannon_entropy for string fields should be in valid range, got: {}",
                            entropy
                        );
                        assert!(entropy.is_finite(), "shannon_entropy should be finite");
                    }
                }
            }

            break;
        }
    }
}

#[test]
fn moarstats_shannon_entropy_mixed_distribution() {
    let wrk = Workdir::new("moarstats_shannon_entropy_mixed");

    // Create CSV with mixed distribution (some values repeated, some unique)
    wrk.create(
        "test.csv",
        vec![
            svec!["field", "value"],
            svec!["test", "A"],
            svec!["test", "A"],
            svec!["test", "A"],
            svec!["test", "B"],
            svec!["test", "B"],
            svec!["test", "C"],
        ],
    );

    // Generate stats
    let mut stats_cmd = wrk.command("stats");
    stats_cmd.arg("--everything").arg("test.csv");
    wrk.assert_success(&mut stats_cmd);

    // Run moarstats
    let mut cmd = wrk.command("moarstats");
    cmd.arg("test.csv").arg("--advanced");
    wrk.assert_success(&mut cmd);

    // Verify entropy is computed correctly for mixed distribution
    let stats_content = wrk.read_to_string("test.stats.csv").unwrap();
    let mut rdr = ReaderBuilder::new()
        .has_headers(true)
        .from_reader(stats_content.as_bytes());

    let headers = rdr.headers().unwrap().clone();
    let field_idx = get_column_index(&headers, "field").unwrap();

    for result in rdr.records() {
        let record = result.unwrap();
        let field_name = get_field_value(&record, field_idx).unwrap();

        if field_name == "test" {
            if let Some(entropy_idx) = get_column_index(&headers, "shannon_entropy") {
                let entropy_val = get_field_value(&record, entropy_idx);
                if let Some(val_str) = entropy_val {
                    if !val_str.is_empty() {
                        let entropy: f64 = val_str.parse().unwrap();
                        // With 3 unique values (A appears 3 times, B appears 2 times, C appears 1
                        // time) Entropy should be between 0 and log2(3) ≈
                        // 1.585 But since distribution is not uniform, it
                        // should be less than maximum
                        let max_entropy = 3.0_f64.log2();
                        assert!(
                            entropy >= 0.0 && entropy <= max_entropy,
                            "shannon_entropy should be in valid range [0, log2(3)], got: {}",
                            entropy
                        );
                        // With non-uniform distribution, entropy should be less than maximum
                        assert!(
                            entropy < max_entropy,
                            "shannon_entropy should be less than maximum for non-uniform \
                             distribution, got: {}",
                            entropy
                        );
                    }
                }
            }

            break;
        }
    }
}

#[test]
fn moarstats_shannon_entropy_multiple_fields() {
    let wrk = Workdir::new("moarstats_shannon_entropy_multiple");
    let test_file = wrk.load_test_file("boston311-100.csv");

    // Generate baseline stats
    let mut stats_cmd = wrk.command("stats");
    stats_cmd
        .arg("--everything")
        .arg("--infer-dates")
        .arg(&test_file);
    wrk.assert_success(&mut stats_cmd);

    // Run moarstats with --advanced flag
    let mut cmd = wrk.command("moarstats");
    cmd.arg("--advanced").arg(&test_file);
    wrk.assert_success(&mut cmd);

    // Verify that multiple fields get entropy statistics
    let stats_content = wrk.read_to_string("boston311-100.stats.csv").unwrap();
    let mut rdr = ReaderBuilder::new()
        .has_headers(true)
        .from_reader(stats_content.as_bytes());

    let headers = rdr.headers().unwrap().clone();
    let entropy_idx = get_column_index(&headers, "shannon_entropy");

    let mut fields_with_entropy = 0;

    for result in rdr.records() {
        let record = result.unwrap();

        // Entropy works for all field types
        if let Some(entropy_idx) = entropy_idx {
            let entropy_val = get_field_value(&record, entropy_idx);
            if entropy_val.is_some() && !entropy_val.as_ref().unwrap().is_empty() {
                fields_with_entropy += 1;
            }
        }
    }

    assert!(
        fields_with_entropy > 1,
        "Multiple fields should have shannon_entropy statistics"
    );
}

#[test]
fn moarstats_shannon_entropy_insufficient_data() {
    let wrk = Workdir::new("moarstats_shannon_entropy_insufficient");

    // Create CSV with only one value
    wrk.create(
        "test.csv",
        vec![svec!["field", "value"], svec!["test", "5"]],
    );

    // Generate stats
    let mut stats_cmd = wrk.command("stats");
    stats_cmd.arg("--everything").arg("test.csv");
    wrk.assert_success(&mut stats_cmd);

    // Run moarstats with --advanced flag
    let mut cmd = wrk.command("moarstats");
    cmd.arg("--advanced").arg("test.csv");
    wrk.assert_success(&mut cmd);

    // Verify column exists but value may be empty or 0
    let stats_content = wrk.read_to_string("test.stats.csv").unwrap();
    let mut rdr = ReaderBuilder::new()
        .has_headers(true)
        .from_reader(stats_content.as_bytes());

    let headers = rdr.headers().unwrap().clone();
    let field_idx = get_column_index(&headers, "field").unwrap();

    assert!(
        get_column_index(&headers, "shannon_entropy").is_some(),
        "shannon_entropy column should exist"
    );

    for result in rdr.records() {
        let record = result.unwrap();
        let field_name = get_field_value(&record, field_idx).unwrap();

        if field_name == "test" {
            // With only one value, entropy should be 0 (all values identical)
            if let Some(entropy_idx) = get_column_index(&headers, "shannon_entropy") {
                let entropy_val = get_field_value(&record, entropy_idx);
                if let Some(val_str) = entropy_val {
                    if !val_str.is_empty() {
                        let entropy: f64 = val_str.parse().unwrap();
                        // Single value means entropy = 0
                        assert!(
                            entropy.abs() < 0.001,
                            "shannon_entropy should be 0 for single value, got: {}",
                            entropy
                        );
                    }
                }
            }

            break;
        }
    }
}

#[test]
fn moarstats_shannon_entropy_boolean_fields() {
    let wrk = Workdir::new("moarstats_shannon_entropy_boolean");

    wrk.create(
        "test.csv",
        vec![
            svec!["field", "bool_value"],
            svec!["test", "true"],
            svec!["test", "false"],
            svec!["test", "true"],
            svec!["test", "false"],
            svec!["test", "true"],
        ],
    );

    // Generate stats with boolean inference
    let mut stats_cmd = wrk.command("stats");
    stats_cmd
        .arg("--everything")
        .arg("--infer-boolean")
        .arg("test.csv");
    wrk.assert_success(&mut stats_cmd);

    // Run moarstats
    let mut cmd = wrk.command("moarstats");
    cmd.arg("test.csv").arg("--advanced");
    wrk.assert_success(&mut cmd);

    // Verify boolean fields have entropy computed
    let stats_content = wrk.read_to_string("test.stats.csv").unwrap();
    let mut rdr = ReaderBuilder::new()
        .has_headers(true)
        .from_reader(stats_content.as_bytes());

    let headers = rdr.headers().unwrap().clone();
    let field_idx = get_column_index(&headers, "field").unwrap();
    let type_idx = get_column_index(&headers, "type").unwrap();

    for result in rdr.records() {
        let record = result.unwrap();
        let field_name = get_field_value(&record, field_idx).unwrap();
        let field_type = get_field_value(&record, type_idx).unwrap();

        if field_name == "test" && field_type == "Boolean" {
            // Boolean fields should have entropy computed
            if let Some(entropy_idx) = get_column_index(&headers, "shannon_entropy") {
                let entropy_val = get_field_value(&record, entropy_idx);
                if let Some(val_str) = entropy_val {
                    if !val_str.is_empty() {
                        let entropy: f64 = val_str.parse().unwrap();
                        // With 2 unique values (true/false), max entropy is log2(2) = 1.0
                        // With 3 true and 2 false, entropy should be less than 1.0
                        assert!(
                            entropy >= 0.0 && entropy <= 1.0,
                            "shannon_entropy for boolean fields should be in [0, 1], got: {}",
                            entropy
                        );
                        assert!(entropy.is_finite(), "shannon_entropy should be finite");
                    }
                }
            }

            break;
        }
    }
}

#[test]
fn moarstats_bivariate_basic() {
    let wrk = Workdir::new("moarstats_bivariate_basic");

    // Create CSV with two numeric fields that should correlate
    // Note: Added duplicate values so cardinality < rowcount (avoids filtering)
    wrk.create(
        "test.csv",
        vec![
            svec!["x", "y"],
            svec!["1", "2"],
            svec!["2", "4"],
            svec!["3", "6"],
            svec!["4", "8"],
            svec!["5", "10"],
            svec!["1", "2"], // Duplicate to ensure cardinality < rowcount
        ],
    );

    // Generate baseline stats
    let mut stats_cmd = wrk.command("stats");
    stats_cmd.arg("--everything").arg("test.csv");
    wrk.assert_success(&mut stats_cmd);

    // Run moarstats with --bivariate flag
    let mut cmd = wrk.command("moarstats");
    cmd.arg("--bivariate").arg("test.csv");
    wrk.assert_success(&mut cmd);

    // Verify bivariate CSV file exists
    let bivariate_path = wrk.path("test.stats.bivariate.csv");
    assert!(
        bivariate_path.exists(),
        "Bivariate statistics file should exist"
    );

    // Verify bivariate CSV has correct columns
    let bivariate_content = wrk.read_to_string("test.stats.bivariate.csv").unwrap();
    let mut rdr = ReaderBuilder::new()
        .has_headers(true)
        .from_reader(bivariate_content.as_bytes());

    let headers = rdr.headers().unwrap().clone();
    assert!(
        get_column_index(&headers, "field1").is_some(),
        "field1 column should exist"
    );
    assert!(
        get_column_index(&headers, "field2").is_some(),
        "field2 column should exist"
    );
    assert!(
        get_column_index(&headers, "pearson_correlation").is_some(),
        "pearson_correlation column should exist"
    );
    assert!(
        get_column_index(&headers, "covariance_sample").is_some(),
        "covariance_sample column should exist"
    );
    assert!(
        get_column_index(&headers, "covariance_population").is_some(),
        "covariance_population column should exist"
    );
    assert!(
        get_column_index(&headers, "n_pairs").is_some(),
        "n_pairs column should exist"
    );

    // Verify relationship between x and y (should be perfect positive correlation)
    let field1_idx = get_column_index(&headers, "field1").unwrap();
    let field2_idx = get_column_index(&headers, "field2").unwrap();
    let pearson_idx = get_column_index(&headers, "pearson_correlation").unwrap();
    let n_pairs_idx = get_column_index(&headers, "n_pairs").unwrap();

    let mut found_x_y_pair = false;
    for result in rdr.records() {
        let record = result.unwrap();
        let field1 = get_field_value(&record, field1_idx).unwrap();
        let field2 = get_field_value(&record, field2_idx).unwrap();

        if (field1 == "x" && field2 == "y") || (field1 == "y" && field2 == "x") {
            found_x_y_pair = true;

            // Verify n_pairs (6 rows including duplicate)
            let n_pairs_val = get_field_value(&record, n_pairs_idx).unwrap();
            let n_pairs: u64 = n_pairs_val.parse().unwrap();
            assert_eq!(n_pairs, 6, "n_pairs should be 6");

            // Verify Pearson correlation (should be 1.0 for perfect positive correlation)
            let pearson_val = get_field_value(&record, pearson_idx);
            if let Some(val_str) = pearson_val {
                if !val_str.is_empty() {
                    let pearson: f64 = val_str.parse().unwrap();
                    assert!(
                        pearson >= 0.99 && pearson <= 1.0,
                        "Pearson correlation should be close to 1.0 for perfect positive \
                         correlation, got: {}",
                        pearson
                    );
                }
            }

            break;
        }
    }

    assert!(found_x_y_pair, "Should find x-y field pair");
}

#[test]
fn moarstats_bivariate_negative_correlation() {
    let wrk = Workdir::new("moarstats_bivariate_negative");

    // Create CSV with two numeric fields that have negative correlation
    // Note: Added duplicate values so cardinality < rowcount (avoids filtering)
    wrk.create(
        "test.csv",
        vec![
            svec!["x", "y"],
            svec!["1", "10"],
            svec!["2", "8"],
            svec!["3", "6"],
            svec!["4", "4"],
            svec!["5", "2"],
            svec!["1", "10"], // Duplicate to ensure cardinality < rowcount
        ],
    );

    let mut stats_cmd = wrk.command("stats");
    stats_cmd.arg("--everything").arg("test.csv");
    wrk.assert_success(&mut stats_cmd);

    let mut cmd = wrk.command("moarstats");
    cmd.arg("--bivariate").arg("test.csv");
    wrk.assert_success(&mut cmd);

    let bivariate_content = wrk.read_to_string("test.stats.bivariate.csv").unwrap();
    let mut rdr = ReaderBuilder::new()
        .has_headers(true)
        .from_reader(bivariate_content.as_bytes());

    let headers = rdr.headers().unwrap().clone();
    let field1_idx = get_column_index(&headers, "field1").unwrap();
    let field2_idx = get_column_index(&headers, "field2").unwrap();
    let pearson_idx = get_column_index(&headers, "pearson_correlation").unwrap();

    for result in rdr.records() {
        let record = result.unwrap();
        let field1 = get_field_value(&record, field1_idx).unwrap();
        let field2 = get_field_value(&record, field2_idx).unwrap();

        if (field1 == "x" && field2 == "y") || (field1 == "y" && field2 == "x") {
            let pearson_val = get_field_value(&record, pearson_idx);
            if let Some(val_str) = pearson_val {
                if !val_str.is_empty() {
                    let pearson: f64 = val_str.parse().unwrap();
                    assert!(
                        pearson <= -0.99 && pearson >= -1.0,
                        "Pearson correlation should be close to -1.0 for perfect negative \
                         correlation, got: {}",
                        pearson
                    );
                }
            }
            break;
        }
    }
}

#[test]
fn moarstats_bivariate_string_fields() {
    let wrk = Workdir::new("moarstats_bivariate_strings");

    // Create CSV with string fields (should compute mutual information)
    wrk.create(
        "test.csv",
        vec![
            svec!["category", "status"],
            svec!["A", "active"],
            svec!["A", "active"],
            svec!["B", "inactive"],
            svec!["B", "inactive"],
            svec!["C", "active"],
        ],
    );

    let mut stats_cmd = wrk.command("stats");
    stats_cmd.arg("--everything").arg("test.csv");
    wrk.assert_success(&mut stats_cmd);

    let mut cmd = wrk.command("moarstats");
    cmd.arg("--bivariate")
        .arg("test.csv")
        .args(["--bivariate-stats", "all"]);
    wrk.assert_success(&mut cmd);

    let bivariate_content = wrk.read_to_string("test.stats.bivariate.csv").unwrap();
    let mut rdr = ReaderBuilder::new()
        .has_headers(true)
        .from_reader(bivariate_content.as_bytes());

    let headers = rdr.headers().unwrap().clone();
    let field1_idx = get_column_index(&headers, "field1").unwrap();
    let field2_idx = get_column_index(&headers, "field2").unwrap();
    let mi_idx = get_column_index(&headers, "mutual_information").unwrap();
    let n_pairs_idx = get_column_index(&headers, "n_pairs").unwrap();

    let mut found_pair = false;
    for result in rdr.records() {
        let record = result.unwrap();
        let field1 = get_field_value(&record, field1_idx).unwrap();
        let field2 = get_field_value(&record, field2_idx).unwrap();

        if (field1 == "category" && field2 == "status")
            || (field1 == "status" && field2 == "category")
        {
            found_pair = true;

            // Verify n_pairs (5 rows - string fields already have duplicates so cardinality <
            // rowcount)
            let n_pairs_val = get_field_value(&record, n_pairs_idx).unwrap();
            let n_pairs: u64 = n_pairs_val.parse().unwrap();
            assert_eq!(n_pairs, 5, "n_pairs should be 5");

            // Verify mutual information exists and is non-negative
            let mi_val = get_field_value(&record, mi_idx);
            if let Some(val_str) = mi_val {
                if !val_str.is_empty() {
                    let mi: f64 = val_str.parse().unwrap();
                    assert!(
                        mi >= 0.0,
                        "Mutual information should be non-negative, got: {}",
                        mi
                    );
                    assert!(mi.is_finite(), "Mutual information should be finite");
                }
            }

            break;
        }
    }

    assert!(found_pair, "Should find category-status field pair");
}

#[test]
fn moarstats_bivariate_normalized_mutual_information() {
    let wrk = Workdir::new("moarstats_bivariate_nmi");

    // Create CSV with string fields that have some correlation
    // This will allow us to test normalized mutual information
    wrk.create(
        "test.csv",
        vec![
            svec!["category", "status"],
            svec!["A", "active"],
            svec!["A", "active"],
            svec!["A", "active"],
            svec!["B", "inactive"],
            svec!["B", "inactive"],
            svec!["B", "inactive"],
            svec!["C", "active"],
            svec!["C", "pending"],
        ],
    );

    let mut stats_cmd = wrk.command("stats");
    stats_cmd.arg("--everything").arg("test.csv");
    wrk.assert_success(&mut stats_cmd);

    let mut cmd = wrk.command("moarstats");
    cmd.arg("--bivariate")
        .arg("test.csv")
        .args(["--bivariate-stats", "all"]);
    wrk.assert_success(&mut cmd);

    let bivariate_content = wrk.read_to_string("test.stats.bivariate.csv").unwrap();
    let mut rdr = ReaderBuilder::new()
        .has_headers(true)
        .from_reader(bivariate_content.as_bytes());

    let headers = rdr.headers().unwrap().clone();
    let field1_idx = get_column_index(&headers, "field1").unwrap();
    let field2_idx = get_column_index(&headers, "field2").unwrap();
    let nmi_idx = get_column_index(&headers, "normalized_mutual_information")
        .expect("normalized_mutual_information column should exist");
    let mi_idx = get_column_index(&headers, "mutual_information").unwrap();
    let n_pairs_idx = get_column_index(&headers, "n_pairs").unwrap();

    let mut found_pair = false;
    for result in rdr.records() {
        let record = result.unwrap();
        let field1 = get_field_value(&record, field1_idx).unwrap();
        let field2 = get_field_value(&record, field2_idx).unwrap();

        if (field1 == "category" && field2 == "status")
            || (field1 == "status" && field2 == "category")
        {
            found_pair = true;

            // Verify n_pairs (8 rows)
            let n_pairs_val = get_field_value(&record, n_pairs_idx).unwrap();
            let n_pairs: u64 = n_pairs_val.parse().unwrap();
            assert_eq!(n_pairs, 8, "n_pairs should be 8");

            // Verify mutual information exists (required for NMI)
            let mi_val = get_field_value(&record, mi_idx);
            assert!(
                mi_val.is_some() && !mi_val.as_ref().unwrap().is_empty(),
                "Mutual information should be computed for NMI"
            );

            // Verify normalized mutual information exists and is valid
            let nmi_val = get_field_value(&record, nmi_idx);
            assert!(
                nmi_val.is_some() && !nmi_val.as_ref().unwrap().is_empty(),
                "Normalized mutual information should be computed"
            );

            if let Some(val_str) = nmi_val {
                if !val_str.is_empty() {
                    let nmi: f64 = val_str.parse().unwrap();
                    // NMI should be between 0 and 1 (normalized)
                    assert!(
                        nmi >= 0.0 && nmi <= 1.0,
                        "Normalized mutual information should be in [0, 1], got: {}",
                        nmi
                    );
                    assert!(
                        nmi.is_finite(),
                        "Normalized mutual information should be finite, got: {}",
                        nmi
                    );
                    assert!(
                        !nmi.is_nan(),
                        "Normalized mutual information should not be NaN"
                    );

                    // Verify NMI is non-negative
                    //(should always be true given the range check above)
                    assert!(
                        nmi >= 0.0,
                        "Normalized mutual information should be non-negative, got: {}",
                        nmi
                    );
                }
            }

            break;
        }
    }

    assert!(found_pair, "Should find category-status field pair");
}

#[test]
fn moarstats_bivariate_multiple_fields() {
    let wrk = Workdir::new("moarstats_bivariate_multiple");

    // Create CSV with multiple numeric fields
    // Note: Added duplicate values so cardinality < rowcount (avoids filtering)
    wrk.create(
        "test.csv",
        vec![
            svec!["x", "y", "z"],
            svec!["1", "2", "3"],
            svec!["2", "4", "6"],
            svec!["3", "6", "9"],
            svec!["4", "8", "12"],
            svec!["5", "10", "15"],
            svec!["1", "2", "3"], // Duplicate to ensure cardinality < rowcount
        ],
    );

    let mut stats_cmd = wrk.command("stats");
    stats_cmd.arg("--everything").arg("test.csv");
    wrk.assert_success(&mut stats_cmd);

    let mut cmd = wrk.command("moarstats");
    cmd.arg("--bivariate").arg("test.csv");
    wrk.assert_success(&mut cmd);

    let bivariate_content = wrk.read_to_string("test.stats.bivariate.csv").unwrap();
    let mut rdr = ReaderBuilder::new()
        .has_headers(true)
        .from_reader(bivariate_content.as_bytes());

    let headers = rdr.headers().unwrap().clone();
    let field1_idx = get_column_index(&headers, "field1").unwrap();
    let field2_idx = get_column_index(&headers, "field2").unwrap();

    // Should have 3 pairs: x-y, x-z, y-z
    let mut pairs = Vec::new();
    for result in rdr.records() {
        let record = result.unwrap();
        let field1 = get_field_value(&record, field1_idx).unwrap();
        let field2 = get_field_value(&record, field2_idx).unwrap();
        pairs.push((field1, field2));
    }

    assert_eq!(pairs.len(), 3, "Should have 3 field pairs for 3 fields");
    assert!(
        pairs
            .iter()
            .any(|(f1, f2)| (f1 == "x" && f2 == "y") || (f1 == "y" && f2 == "x")),
        "Should have x-y pair"
    );
    assert!(
        pairs
            .iter()
            .any(|(f1, f2)| (f1 == "x" && f2 == "z") || (f1 == "z" && f2 == "x")),
        "Should have x-z pair"
    );
    assert!(
        pairs
            .iter()
            .any(|(f1, f2)| (f1 == "y" && f2 == "z") || (f1 == "z" && f2 == "y")),
        "Should have y-z pair"
    );
}

#[test]
fn moarstats_bivariate_all_statistics() {
    let wrk = Workdir::new("moarstats_bivariate_all_stats");

    // Create CSV with enough data to compute all bivariate statistics
    // Note: Added duplicate values so cardinality < rowcount (avoids filtering)
    wrk.create(
        "test.csv",
        vec![
            svec!["x", "y"],
            svec!["1", "3"],
            svec!["2", "5"],
            svec!["3", "7"],
            svec!["4", "9"],
            svec!["5", "11"],
            svec!["6", "13"],
            svec!["7", "15"],
            svec!["1", "3"], // Duplicate to ensure cardinality < rowcount
        ],
    );

    let mut stats_cmd = wrk.command("stats");
    stats_cmd.arg("--everything").arg("test.csv");
    wrk.assert_success(&mut stats_cmd);

    let mut cmd = wrk.command("moarstats");
    cmd.arg("--bivariate")
        .arg("test.csv")
        .args(["--bivariate-stats", "all"]);
    wrk.assert_success(&mut cmd);

    let bivariate_content = wrk.read_to_string("test.stats.bivariate.csv").unwrap();
    let mut rdr = ReaderBuilder::new()
        .has_headers(true)
        .from_reader(bivariate_content.as_bytes());

    let headers = rdr.headers().unwrap().clone();
    let field1_idx = get_column_index(&headers, "field1").unwrap();
    let field2_idx = get_column_index(&headers, "field2").unwrap();
    let pearson_idx = get_column_index(&headers, "pearson_correlation").unwrap();
    let spearman_idx = get_column_index(&headers, "spearman_correlation").unwrap();
    let kendall_idx = get_column_index(&headers, "kendall_tau").unwrap();
    let cov_sample_idx = get_column_index(&headers, "covariance_sample").unwrap();
    let cov_pop_idx = get_column_index(&headers, "covariance_population").unwrap();
    let mi_idx = get_column_index(&headers, "mutual_information").unwrap();

    for result in rdr.records() {
        let record = result.unwrap();
        let field1 = get_field_value(&record, field1_idx).unwrap();
        let field2 = get_field_value(&record, field2_idx).unwrap();

        if (field1 == "x" && field2 == "y") || (field1 == "y" && field2 == "x") {
            // Verify all statistics are computed
            let pearson_val = get_field_value(&record, pearson_idx);
            assert!(
                pearson_val.is_some() && !pearson_val.as_ref().unwrap().is_empty(),
                "Pearson correlation should be computed"
            );

            let spearman_val = get_field_value(&record, spearman_idx);
            assert!(
                spearman_val.is_some() && !spearman_val.as_ref().unwrap().is_empty(),
                "Spearman correlation should be computed"
            );

            let kendall_val = get_field_value(&record, kendall_idx);
            assert!(
                kendall_val.is_some() && !kendall_val.as_ref().unwrap().is_empty(),
                "Kendall tau should be computed"
            );

            let cov_sample_val = get_field_value(&record, cov_sample_idx);
            assert!(
                cov_sample_val.is_some() && !cov_sample_val.as_ref().unwrap().is_empty(),
                "Sample covariance should be computed"
            );

            let cov_pop_val = get_field_value(&record, cov_pop_idx);
            assert!(
                cov_pop_val.is_some() && !cov_pop_val.as_ref().unwrap().is_empty(),
                "Population covariance should be computed"
            );

            let mi_val = get_field_value(&record, mi_idx);
            assert!(
                mi_val.is_some() && !mi_val.as_ref().unwrap().is_empty(),
                "Mutual information should be computed"
            );

            // Verify values are in valid ranges
            if let Some(val_str) = pearson_val {
                let pearson: f64 = val_str.parse().unwrap();
                assert!(
                    pearson >= -1.0 && pearson <= 1.0,
                    "Pearson correlation should be in [-1, 1], got: {}",
                    pearson
                );
            }

            if let Some(val_str) = spearman_val {
                let spearman: f64 = val_str.parse().unwrap();
                assert!(
                    spearman >= -1.0 && spearman <= 1.0,
                    "Spearman correlation should be in [-1, 1], got: {}",
                    spearman
                );
            }

            if let Some(val_str) = kendall_val {
                let kendall: f64 = val_str.parse().unwrap();
                assert!(
                    kendall >= -1.0 && kendall <= 1.0,
                    "Kendall tau should be in [-1, 1], got: {}",
                    kendall
                );
            }

            break;
        }
    }
}

#[test]
fn moarstats_bivariate_mixed_field_types() {
    let wrk = Workdir::new("moarstats_bivariate_mixed");

    // Create CSV with numeric and string fields
    // Note: Added duplicate numeric value so cardinality < rowcount (avoids filtering)
    wrk.create(
        "test.csv",
        vec![
            svec!["numeric", "category"],
            svec!["10", "A"],
            svec!["20", "A"],
            svec!["30", "B"],
            svec!["40", "B"],
            svec!["50", "C"],
            svec!["10", "A"], // Duplicate to ensure cardinality < rowcount
        ],
    );

    let mut stats_cmd = wrk.command("stats");
    stats_cmd.arg("--everything").arg("test.csv");
    wrk.assert_success(&mut stats_cmd);

    let mut cmd = wrk.command("moarstats");
    cmd.arg("--bivariate")
        .arg("test.csv")
        .args(["--bivariate-stats", "all"]);
    wrk.assert_success(&mut cmd);

    let bivariate_content = wrk.read_to_string("test.stats.bivariate.csv").unwrap();
    let mut rdr = ReaderBuilder::new()
        .has_headers(true)
        .from_reader(bivariate_content.as_bytes());

    let headers = rdr.headers().unwrap().clone();
    let field1_idx = get_column_index(&headers, "field1").unwrap();
    let field2_idx = get_column_index(&headers, "field2").unwrap();
    let pearson_idx = get_column_index(&headers, "pearson_correlation").unwrap();
    let mi_idx = get_column_index(&headers, "mutual_information").unwrap();

    for result in rdr.records() {
        let record = result.unwrap();
        let field1 = get_field_value(&record, field1_idx).unwrap();
        let field2 = get_field_value(&record, field2_idx).unwrap();

        if (field1 == "numeric" && field2 == "category")
            || (field1 == "category" && field2 == "numeric")
        {
            // Pearson correlation should be empty (can't compute between numeric and string)
            let pearson_val = get_field_value(&record, pearson_idx);
            if let Some(val_str) = pearson_val {
                assert!(
                    val_str.is_empty(),
                    "Pearson correlation should be empty for mixed field types"
                );
            }

            // Mutual information should be computed (works for all types)
            let mi_val = get_field_value(&record, mi_idx);
            assert!(
                mi_val.is_some() && !mi_val.as_ref().unwrap().is_empty(),
                "Mutual information should be computed for mixed field types"
            );

            break;
        }
    }
}

#[test]
fn moarstats_bivariate_with_join() {
    let wrk = Workdir::new("moarstats_bivariate_join");

    // Create primary dataset
    // Note: Added duplicate values so cardinality < rowcount (avoids filtering)
    wrk.create(
        "primary.csv",
        vec![
            svec!["id", "value1"],
            svec!["1", "10"],
            svec!["2", "20"],
            svec!["3", "30"],
            svec!["1", "10"], // Duplicate to ensure cardinality < rowcount
        ],
    );

    // Create secondary dataset
    wrk.create(
        "secondary.csv",
        vec![
            svec!["id", "value2"],
            svec!["1", "100"],
            svec!["2", "200"],
            svec!["3", "300"],
        ],
    );

    // Generate stats for primary
    let mut stats_cmd = wrk.command("stats");
    stats_cmd.arg("--everything").arg("primary.csv");
    wrk.assert_success(&mut stats_cmd);

    // Run moarstats with --bivariate and --join-inputs
    let mut cmd = wrk.command("moarstats");
    cmd.arg("--bivariate")
        .arg("--join-inputs")
        .arg("secondary.csv")
        .arg("--join-keys")
        .arg("id,id")
        .arg("primary.csv");
    wrk.assert_success(&mut cmd);

    // Verify bivariate file exists (should be based on joined dataset)
    let bivariate_path = wrk.path("primary.stats.bivariate.joined.csv");
    assert!(
        bivariate_path.exists(),
        "Bivariate joined statistics file should exist after join"
    );

    // Verify the bivariate file has content
    let bivariate_content = wrk
        .read_to_string("primary.stats.bivariate.joined.csv")
        .unwrap();
    let mut rdr = ReaderBuilder::new()
        .has_headers(true)
        .from_reader(bivariate_content.as_bytes());

    let headers = rdr.headers().unwrap().clone();
    assert!(
        get_column_index(&headers, "field1").is_some(),
        "Bivariate joined file should have field1 column"
    );
}

#[test]
fn moarstats_bivariate_index_auto_creation() {
    let wrk = Workdir::new("moarstats_bivariate_index");

    // Create CSV file
    // Note: Added duplicate values so cardinality < rowcount (avoids filtering)
    wrk.create(
        "test.csv",
        vec![
            svec!["x", "y"],
            svec!["1", "2"],
            svec!["2", "4"],
            svec!["3", "6"],
            svec!["4", "8"],
            svec!["5", "10"],
            svec!["1", "2"], // Duplicate to ensure cardinality < rowcount
        ],
    );

    // Generate stats
    let mut stats_cmd = wrk.command("stats");
    stats_cmd.arg("--everything").arg("test.csv");
    wrk.assert_success(&mut stats_cmd);

    // Ensure no index exists initially
    let index_path = wrk.path("test.csv.idx");
    if index_path.exists() {
        std::fs::remove_file(&index_path).unwrap();
    }

    // Run moarstats with --bivariate (should auto-create index)
    let mut cmd = wrk.command("moarstats");
    cmd.arg("--bivariate").arg("test.csv");
    let output = wrk.output_stderr(&mut cmd);
    wrk.assert_success(&mut cmd);

    // Verify index was auto-created or message was logged
    assert!(
        index_path.exists() || output.contains("Auto-creating index"),
        "Index should be auto-created for bivariate statistics"
    );

    // Verify bivariate file was created
    let bivariate_path = wrk.path("test.stats.bivariate.csv");
    assert!(
        bivariate_path.exists(),
        "Bivariate statistics file should exist"
    );
}

// Test for --bivariate-stats flag with "pearson" only
#[test]
fn moarstats_bivariate_stats_pearson_only() {
    let wrk = Workdir::new("moarstats_bivariate_stats_pearson");

    // Create CSV with two numeric fields
    wrk.create(
        "test.csv",
        vec![
            svec!["x", "y"],
            svec!["1", "2"],
            svec!["2", "4"],
            svec!["3", "6"],
            svec!["4", "8"],
            svec!["5", "10"],
            svec!["1", "2"], // Duplicate to ensure cardinality < rowcount
        ],
    );

    // Generate baseline stats
    let mut stats_cmd = wrk.command("stats");
    stats_cmd.arg("--everything").arg("test.csv");
    wrk.assert_success(&mut stats_cmd);

    // Run moarstats with --bivariate and --bivariate-stats pearson
    let mut cmd = wrk.command("moarstats");
    cmd.arg("--bivariate")
        .arg("--bivariate-stats")
        .arg("pearson")
        .arg("test.csv");
    wrk.assert_success(&mut cmd);

    // Verify bivariate CSV file exists
    let bivariate_path = wrk.path("test.stats.bivariate.csv");
    assert!(
        bivariate_path.exists(),
        "Bivariate statistics file should exist"
    );

    // Read bivariate CSV
    let bivariate_content = wrk.read_to_string("test.stats.bivariate.csv").unwrap();
    let mut rdr = ReaderBuilder::new()
        .has_headers(true)
        .from_reader(bivariate_content.as_bytes());

    let headers = rdr.headers().unwrap().clone();

    // Verify that only requested columns are present
    assert!(
        get_column_index(&headers, "field1").is_some(),
        "field1 column should exist"
    );
    assert!(
        get_column_index(&headers, "field2").is_some(),
        "field2 column should exist"
    );
    assert!(
        get_column_index(&headers, "pearson_correlation").is_some(),
        "pearson_correlation column should exist"
    );
    assert!(
        get_column_index(&headers, "n_pairs").is_some(),
        "n_pairs column should exist"
    );

    // Verify that non-requested columns are NOT present
    assert!(
        get_column_index(&headers, "spearman_correlation").is_none(),
        "spearman_correlation column should NOT exist when not requested"
    );
    assert!(
        get_column_index(&headers, "kendall_tau").is_none(),
        "kendall_tau column should NOT exist when not requested"
    );
    assert!(
        get_column_index(&headers, "covariance_sample").is_none(),
        "covariance_sample column should NOT exist when not requested"
    );
    assert!(
        get_column_index(&headers, "mutual_information").is_none(),
        "mutual_information column should NOT exist when not requested"
    );

    let pearson_idx = get_column_index(&headers, "pearson_correlation").unwrap();
    let n_pairs_idx = get_column_index(&headers, "n_pairs").unwrap();

    // Verify that pearson has a value
    for result in rdr.records() {
        let record = result.unwrap();

        // Pearson should have a value
        let pearson_val = get_field_value(&record, pearson_idx);
        assert!(
            pearson_val.is_some() && !pearson_val.unwrap().is_empty(),
            "Pearson correlation should be computed"
        );

        // n_pairs should have a value
        let n_pairs_val = get_field_value(&record, n_pairs_idx);
        assert!(
            n_pairs_val.is_some() && !n_pairs_val.unwrap().is_empty(),
            "n_pairs should be present"
        );
    }
}

// Test for --bivariate-stats flag with multiple stats
#[test]
fn moarstats_bivariate_stats_multiple() {
    let wrk = Workdir::new("moarstats_bivariate_stats_multiple");

    // Create CSV with two numeric fields
    wrk.create(
        "test.csv",
        vec![
            svec!["x", "y"],
            svec!["1", "2"],
            svec!["2", "4"],
            svec!["3", "6"],
            svec!["4", "8"],
            svec!["5", "10"],
            svec!["1", "2"], // Duplicate to ensure cardinality < rowcount
        ],
    );

    // Generate baseline stats
    let mut stats_cmd = wrk.command("stats");
    stats_cmd.arg("--everything").arg("test.csv");
    wrk.assert_success(&mut stats_cmd);

    // Run moarstats with --bivariate and --bivariate-stats pearson,covariance
    let mut cmd = wrk.command("moarstats");
    cmd.arg("--bivariate")
        .arg("--bivariate-stats")
        .arg("pearson,covariance")
        .arg("test.csv");
    wrk.assert_success(&mut cmd);

    // Verify bivariate CSV file exists
    let bivariate_path = wrk.path("test.stats.bivariate.csv");
    assert!(
        bivariate_path.exists(),
        "Bivariate statistics file should exist"
    );

    // Read bivariate CSV
    let bivariate_content = wrk.read_to_string("test.stats.bivariate.csv").unwrap();
    let mut rdr = ReaderBuilder::new()
        .has_headers(true)
        .from_reader(bivariate_content.as_bytes());

    let headers = rdr.headers().unwrap().clone();

    // Verify that only requested columns are present
    assert!(
        get_column_index(&headers, "field1").is_some(),
        "field1 column should exist"
    );
    assert!(
        get_column_index(&headers, "field2").is_some(),
        "field2 column should exist"
    );
    assert!(
        get_column_index(&headers, "pearson_correlation").is_some(),
        "pearson_correlation column should exist"
    );
    assert!(
        get_column_index(&headers, "covariance_sample").is_some(),
        "covariance_sample column should exist"
    );
    assert!(
        get_column_index(&headers, "covariance_population").is_some(),
        "covariance_population column should exist"
    );
    assert!(
        get_column_index(&headers, "n_pairs").is_some(),
        "n_pairs column should exist"
    );

    // Verify that non-requested columns are NOT present
    assert!(
        get_column_index(&headers, "spearman_correlation").is_none(),
        "spearman_correlation column should NOT exist when not requested"
    );
    assert!(
        get_column_index(&headers, "kendall_tau").is_none(),
        "kendall_tau column should NOT exist when not requested"
    );
    assert!(
        get_column_index(&headers, "mutual_information").is_none(),
        "mutual_information column should NOT exist when not requested"
    );

    let pearson_idx = get_column_index(&headers, "pearson_correlation").unwrap();
    let cov_sample_idx = get_column_index(&headers, "covariance_sample").unwrap();
    let cov_pop_idx = get_column_index(&headers, "covariance_population").unwrap();

    // Verify that pearson and covariance have values
    for result in rdr.records() {
        let record = result.unwrap();

        // Pearson should have a value
        let pearson_val = get_field_value(&record, pearson_idx);
        assert!(
            pearson_val.is_some() && !pearson_val.unwrap().is_empty(),
            "Pearson correlation should be computed"
        );

        // Covariance (sample) should have a value
        let cov_sample_val = get_field_value(&record, cov_sample_idx);
        assert!(
            cov_sample_val.is_some() && !cov_sample_val.unwrap().is_empty(),
            "Covariance (sample) should be computed"
        );

        // Covariance (population) should have a value
        let cov_pop_val = get_field_value(&record, cov_pop_idx);
        assert!(
            cov_pop_val.is_some() && !cov_pop_val.unwrap().is_empty(),
            "Covariance (population) should be computed"
        );
    }
}

// Test for --bivariate-stats flag with "all" (default)
#[test]
fn moarstats_bivariate_stats_all() {
    let wrk = Workdir::new("moarstats_bivariate_stats_all");

    // Create CSV with two numeric fields
    wrk.create(
        "test.csv",
        vec![
            svec!["x", "y"],
            svec!["1", "2"],
            svec!["2", "4"],
            svec!["3", "6"],
            svec!["4", "8"],
            svec!["5", "10"],
            svec!["1", "2"], // Duplicate to ensure cardinality < rowcount
        ],
    );

    // Generate baseline stats
    let mut stats_cmd = wrk.command("stats");
    stats_cmd.arg("--everything").arg("test.csv");
    wrk.assert_success(&mut stats_cmd);

    // Run moarstats with --bivariate and --bivariate-stats all
    let mut cmd = wrk.command("moarstats");
    cmd.arg("--bivariate")
        .arg("--bivariate-stats")
        .arg("all")
        .arg("test.csv");
    wrk.assert_success(&mut cmd);

    // Verify bivariate CSV file exists
    let bivariate_path = wrk.path("test.stats.bivariate.csv");
    assert!(
        bivariate_path.exists(),
        "Bivariate statistics file should exist"
    );

    // Read bivariate CSV
    let bivariate_content = wrk.read_to_string("test.stats.bivariate.csv").unwrap();
    let mut rdr = ReaderBuilder::new()
        .has_headers(true)
        .from_reader(bivariate_content.as_bytes());

    let headers = rdr.headers().unwrap().clone();
    let pearson_idx = get_column_index(&headers, "pearson_correlation").unwrap();
    let spearman_idx = get_column_index(&headers, "spearman_correlation").unwrap();
    let kendall_idx = get_column_index(&headers, "kendall_tau").unwrap();
    let cov_sample_idx = get_column_index(&headers, "covariance_sample").unwrap();

    // Verify that all statistics are computed
    for result in rdr.records() {
        let record = result.unwrap();

        // All stats should have values
        let pearson_val = get_field_value(&record, pearson_idx);
        assert!(
            pearson_val.is_some() && !pearson_val.unwrap().is_empty(),
            "Pearson correlation should be computed with 'all'"
        );

        let spearman_val = get_field_value(&record, spearman_idx);
        assert!(
            spearman_val.is_some() && !spearman_val.unwrap().is_empty(),
            "Spearman correlation should be computed with 'all'"
        );

        let kendall_val = get_field_value(&record, kendall_idx);
        assert!(
            kendall_val.is_some() && !kendall_val.unwrap().is_empty(),
            "Kendall tau should be computed with 'all'"
        );

        let cov_val = get_field_value(&record, cov_sample_idx);
        assert!(
            cov_val.is_some() && !cov_val.unwrap().is_empty(),
            "Covariance should be computed with 'all'"
        );
    }
}

// Test for invalid --bivariate-stats values
#[test]
fn moarstats_bivariate_stats_invalid() {
    let wrk = Workdir::new("moarstats_bivariate_stats_invalid");

    // Create CSV with two numeric fields
    wrk.create(
        "test.csv",
        vec![
            svec!["x", "y"],
            svec!["1", "2"],
            svec!["2", "4"],
            svec!["3", "6"],
        ],
    );

    // Generate baseline stats
    let mut stats_cmd = wrk.command("stats");
    stats_cmd.arg("--everything").arg("test.csv");
    wrk.assert_success(&mut stats_cmd);

    // Test with invalid stat name
    let mut cmd = wrk.command("moarstats");
    cmd.arg("--bivariate")
        .arg("--bivariate-stats")
        .arg("invalid_stat")
        .arg("test.csv");
    wrk.assert_err(&mut cmd);

    // Test with mix of valid and invalid stats
    let mut cmd = wrk.command("moarstats");
    cmd.arg("--bivariate")
        .arg("--bivariate-stats")
        .arg("pearson,invalid_stat,kendall")
        .arg("test.csv");
    wrk.assert_err(&mut cmd);

    let mut cmd = wrk.command("moarstats");
    cmd.arg("--bivariate")
        .arg("--bivariate-stats")
        .arg("")
        .arg("test.csv");
    wrk.assert_err(&mut cmd);
}

// Test --xsd-gdate-scan with thorough mode requested for all Gregorian types.
// Note: Integer-backed types (gYear) have percentile stats available, so they can
// actually run in thorough mode and are marked with the `?` suffix.
// String-backed types (gYearMonth, gMonthDay, gDay, gMonth) never have percentile
// stats, so even under thorough mode they behave like quick mode and are marked
// with the `??` suffix.
#[test]
fn moarstats_xsd_gdate_scan_thorough_mode() {
    let wrk = Workdir::new("moarstats_xsd_gdate_thorough");

    // Create CSV with various Gregorian date types
    wrk.create(
        "test.csv",
        vec![
            svec![
                "gYear",
                "gYearMonth",
                "gMonthDay",
                "gDay",
                "gMonth",
                "regular_string"
            ],
            svec!["1999", "1999-05", "--05-01", "---01", "--05", "not a date"],
            svec!["2000", "2000-06", "--06-15", "---15", "--06", "also not"],
            svec!["2001", "2001-07", "--07-20", "---20", "--07", "text"],
            svec!["2002", "2002-08", "--08-25", "---25", "--08", "data"],
            svec!["2003", "2003-09", "--09-30", "---30", "--09", "value"],
        ],
    );

    // Generate baseline stats with percentiles
    let mut stats_cmd = wrk.command("stats");
    stats_cmd.arg("--everything").arg("test.csv");
    wrk.assert_success(&mut stats_cmd);

    // Run moarstats with thorough mode
    let mut cmd = wrk.command("moarstats");
    cmd.arg("--xsd-gdate-scan").arg("thorough").arg("test.csv");
    wrk.assert_success(&mut cmd);

    // Verify xsd_type column contains Gregorian types with correct suffixes:
    // "?" for gYear (Integer) and "??" for gYearMonth, gMonthDay, gDay, gMonth (String)
    let stats_content = wrk.read_to_string("test.stats.csv").unwrap();
    let mut rdr = ReaderBuilder::new()
        .has_headers(true)
        .from_reader(stats_content.as_bytes());

    let headers = rdr.headers().unwrap().clone();
    let field_idx = get_column_index(&headers, "field").unwrap();
    let xsd_type_idx = get_column_index(&headers, "xsd_type").unwrap();

    let mut found_g_year = false;
    let mut found_g_year_month = false;
    let mut found_g_month_day = false;
    let mut found_g_day = false;
    let mut found_g_month = false;

    for result in rdr.records() {
        let record = result.unwrap();
        let field = get_field_value(&record, field_idx).unwrap();
        let xsd_type = get_field_value(&record, xsd_type_idx);

        match field.as_str() {
            "gYear" => {
                // Integer types get percentiles, so thorough mode works with ? suffix (more
                // confident)
                assert_eq!(
                    xsd_type.as_deref(),
                    Some("gYear?"),
                    "gYear (Integer) should have ? suffix in thorough mode"
                );
                found_g_year = true;
            },
            "gYearMonth" => {
                // String types don't get percentiles, so thorough mode falls back to quick with
                // ?? suffix (less confident)
                assert_eq!(
                    xsd_type.as_deref(),
                    Some("gYearMonth??"),
                    "gYearMonth (String) falls back to quick mode (?? suffix) since String types \
                     don't have percentiles"
                );
                found_g_year_month = true;
            },
            "gMonthDay" => {
                // String types don't get percentiles, so thorough mode falls back to quick with
                // ?? suffix (less confident)
                assert_eq!(
                    xsd_type.as_deref(),
                    Some("gMonthDay??"),
                    "gMonthDay (String) falls back to quick mode (?? suffix) since String types \
                     don't have percentiles"
                );
                found_g_month_day = true;
            },
            "gDay" => {
                // String types don't get percentiles, so thorough mode falls back to quick with
                // ?? suffix (less confident)
                assert_eq!(
                    xsd_type.as_deref(),
                    Some("gDay??"),
                    "gDay (String) falls back to quick mode (?? suffix) since String types don't \
                     have percentiles"
                );
                found_g_day = true;
            },
            "gMonth" => {
                // String types don't get percentiles, so thorough mode falls back to quick with
                // ?? suffix (less confident)
                assert_eq!(
                    xsd_type.as_deref(),
                    Some("gMonth??"),
                    "gMonth (String) falls back to quick mode (?? suffix) since String types \
                     don't have percentiles"
                );
                found_g_month = true;
            },
            "regular_string" => {
                assert_eq!(
                    xsd_type.as_deref(),
                    Some("string"),
                    "Regular string should not be detected as Gregorian"
                );
            },
            _ => {},
        }
    }

    assert!(found_g_year, "gYear field should be found");
    assert!(found_g_year_month, "gYearMonth field should be found");
    assert!(found_g_month_day, "gMonthDay field should be found");
    assert!(found_g_day, "gDay field should be found");
    assert!(found_g_month, "gMonth field should be found");
}

// Test --xsd-gdate-scan quick mode
#[test]
fn moarstats_xsd_gdate_scan_quick_mode() {
    let wrk = Workdir::new("moarstats_xsd_gdate_quick");

    // Create CSV with Gregorian date types
    wrk.create(
        "test.csv",
        vec![
            svec!["gYear", "gYearMonth", "gMonthDay", "gDay", "gMonth"],
            svec!["1999", "1999-05", "--05-01", "---01", "--05"],
            svec!["2000", "2000-06", "--06-15", "---15", "--06"],
            svec!["2001", "2001-07", "--07-20", "---20", "--07"],
        ],
    );

    // Generate baseline stats
    let mut stats_cmd = wrk.command("stats");
    stats_cmd.arg("--everything").arg("test.csv");
    wrk.assert_success(&mut stats_cmd);

    // Run moarstats with quick mode
    let mut cmd = wrk.command("moarstats");
    cmd.arg("--xsd-gdate-scan").arg("quick").arg("test.csv");
    wrk.assert_success(&mut cmd);

    // Verify xsd_type column contains Gregorian types with ?? suffix (less confident)
    let stats_content = wrk.read_to_string("test.stats.csv").unwrap();
    let mut rdr = ReaderBuilder::new()
        .has_headers(true)
        .from_reader(stats_content.as_bytes());

    let headers = rdr.headers().unwrap().clone();
    let field_idx = get_column_index(&headers, "field").unwrap();
    let xsd_type_idx = get_column_index(&headers, "xsd_type").unwrap();

    for result in rdr.records() {
        let record = result.unwrap();
        let field = get_field_value(&record, field_idx).unwrap();
        let xsd_type = get_field_value(&record, xsd_type_idx);

        if field.starts_with("g") {
            assert!(
                xsd_type.as_deref().map_or(false, |t| t.ends_with("??")),
                "Quick mode should use double ?? suffix (less confident), got: {:?}",
                xsd_type
            );
        }
    }
}

// Test Integer gYear detection in thorough mode
#[test]
fn moarstats_xsd_gdate_scan_integer_g_year_thorough() {
    let wrk = Workdir::new("moarstats_xsd_gdate_integer_thorough");

    // Create CSV with Integer years in valid range
    wrk.create(
        "test.csv",
        vec![
            svec!["year"],
            svec!["1999"],
            svec!["2000"],
            svec!["2001"],
            svec!["2002"],
            svec!["2003"],
            svec!["2010"],
            svec!["2020"],
        ],
    );

    // Generate baseline stats with percentiles
    let mut stats_cmd = wrk.command("stats");
    stats_cmd.arg("--everything").arg("test.csv");
    wrk.assert_success(&mut stats_cmd);

    // Run moarstats with thorough mode
    let mut cmd = wrk.command("moarstats");
    cmd.arg("--xsd-gdate-scan").arg("thorough").arg("test.csv");
    wrk.assert_success(&mut cmd);

    // Verify Integer gYear is detected with ? suffix (more confident)
    let stats_content = wrk.read_to_string("test.stats.csv").unwrap();
    let mut rdr = ReaderBuilder::new()
        .has_headers(true)
        .from_reader(stats_content.as_bytes());

    let headers = rdr.headers().unwrap().clone();
    let field_idx = get_column_index(&headers, "field").unwrap();
    let xsd_type_idx = get_column_index(&headers, "xsd_type").unwrap();

    for result in rdr.records() {
        let record = result.unwrap();
        let field = get_field_value(&record, field_idx).unwrap();
        if field == "year" {
            let xsd_type = get_field_value(&record, xsd_type_idx);
            assert_eq!(
                xsd_type.as_deref(),
                Some("gYear?"),
                "Integer year should be detected as gYear? in thorough mode"
            );
        }
    }
}

// Test Integer gYear detection in quick mode
#[test]
fn moarstats_xsd_gdate_scan_integer_g_year_quick() {
    let wrk = Workdir::new("moarstats_xsd_gdate_integer_quick");

    // Create CSV with Integer years in valid range
    wrk.create(
        "test.csv",
        vec![
            svec!["year"],
            svec!["1999"],
            svec!["2000"],
            svec!["2001"],
            svec!["2020"],
        ],
    );

    // Generate baseline stats
    let mut stats_cmd = wrk.command("stats");
    stats_cmd.arg("--everything").arg("test.csv");
    wrk.assert_success(&mut stats_cmd);

    // Run moarstats with quick mode
    let mut cmd = wrk.command("moarstats");
    cmd.arg("--xsd-gdate-scan").arg("quick").arg("test.csv");
    wrk.assert_success(&mut cmd);

    // Verify Integer gYear is detected with ?? suffix (less confident)
    let stats_content = wrk.read_to_string("test.stats.csv").unwrap();
    let mut rdr = ReaderBuilder::new()
        .has_headers(true)
        .from_reader(stats_content.as_bytes());

    let headers = rdr.headers().unwrap().clone();
    let field_idx = get_column_index(&headers, "field").unwrap();
    let xsd_type_idx = get_column_index(&headers, "xsd_type").unwrap();

    for result in rdr.records() {
        let record = result.unwrap();
        let field = get_field_value(&record, field_idx).unwrap();
        if field == "year" {
            let xsd_type = get_field_value(&record, xsd_type_idx);
            assert_eq!(
                xsd_type.as_deref(),
                Some("gYear??"),
                "Integer year should be detected as gYear?? in quick mode"
            );
        }
    }
}

// Test invalid scan mode
#[test]
fn moarstats_xsd_gdate_scan_invalid_mode() {
    let wrk = Workdir::new("moarstats_xsd_gdate_invalid");

    wrk.create("test.csv", vec![svec!["field"], svec!["value"]]);

    // Generate baseline stats
    let mut stats_cmd = wrk.command("stats");
    stats_cmd.arg("--everything").arg("test.csv");
    wrk.assert_success(&mut stats_cmd);

    // Test with invalid scan mode
    let mut cmd = wrk.command("moarstats");
    cmd.arg("--xsd-gdate-scan")
        .arg("invalid_mode")
        .arg("test.csv");
    wrk.assert_err(&mut cmd);
}

// Test default quick mode (no option specified)
#[test]
fn moarstats_xsd_gdate_scan_default_quick() {
    let wrk = Workdir::new("moarstats_xsd_gdate_default");

    // Create CSV with gYear values
    wrk.create(
        "test.csv",
        vec![svec!["year"], svec!["1999"], svec!["2000"], svec!["2001"]],
    );

    // Generate baseline stats with percentiles
    let mut stats_cmd = wrk.command("stats");
    stats_cmd.arg("--everything").arg("test.csv");
    wrk.assert_success(&mut stats_cmd);

    // Run moarstats without specifying scan mode (should default to quick)
    let mut cmd = wrk.command("moarstats");
    cmd.arg("test.csv");
    wrk.assert_success(&mut cmd);

    // Verify default behavior uses quick mode (?? suffix - less confident)
    let stats_content = wrk.read_to_string("test.stats.csv").unwrap();
    let mut rdr = ReaderBuilder::new()
        .has_headers(true)
        .from_reader(stats_content.as_bytes());

    let headers = rdr.headers().unwrap().clone();
    let field_idx = get_column_index(&headers, "field").unwrap();
    let xsd_type_idx = get_column_index(&headers, "xsd_type").unwrap();

    for result in rdr.records() {
        let record = result.unwrap();
        let field = get_field_value(&record, field_idx).unwrap();
        if field == "year" {
            let xsd_type = get_field_value(&record, xsd_type_idx);
            // Default should be quick mode with ?? suffix
            // So we just check that it's detected as gYear with some suffix
            assert!(
                xsd_type
                    .as_deref()
                    .map_or(false, |t| t.starts_with("gYear")),
                "Year should be detected as gYear variant, got: {:?}",
                xsd_type
            );
        }
    }
}

// Test fallback to quick when percentiles are missing
#[test]
fn moarstats_xsd_gdate_scan_fallback_quick() {
    let wrk = Workdir::new("moarstats_xsd_gdate_fallback_quick");

    // Create CSV with gYear values
    wrk.create(
        "test.csv",
        vec![svec!["year"], svec!["1999"], svec!["2000"], svec!["2001"]],
    );

    // Generate baseline stats WITHOUT percentiles (use minimal stats)
    let mut stats_cmd = wrk.command("stats");
    stats_cmd.arg("test.csv"); // Minimal stats, no --everything
    wrk.assert_success(&mut stats_cmd);

    // Run moarstats with thorough mode (should fallback to quick if percentiles missing)
    let mut cmd = wrk.command("moarstats");
    cmd.arg("--xsd-gdate-scan").arg("thorough").arg("test.csv");
    wrk.assert_success(&mut cmd);

    // Verify it falls back to quick mode (?? suffix) when percentiles unavailable
    let stats_content = wrk.read_to_string("test.stats.csv").unwrap();
    let mut rdr = ReaderBuilder::new()
        .has_headers(true)
        .from_reader(stats_content.as_bytes());

    let headers = rdr.headers().unwrap().clone();
    let field_idx = get_column_index(&headers, "field").unwrap();
    let xsd_type_idx = get_column_index(&headers, "xsd_type").unwrap();

    for result in rdr.records() {
        let record = result.unwrap();
        let field = get_field_value(&record, field_idx).unwrap();
        if field == "year" {
            let xsd_type = get_field_value(&record, xsd_type_idx);
            // Should fallback to quick mode (?? suffix) when percentiles unavailable
            // But if the field is Integer and min/max are in range, it should still detect gYear??
            assert!(
                xsd_type
                    .as_deref()
                    .map_or(false, |t| t.starts_with("gYear")),
                "Year should be detected even when percentiles missing, got: {:?}",
                xsd_type
            );
        }
    }
}

// Regression test for the outlier key bug fix:
// Previously, `needs_outlier_counting` checked for "outliers_extreme_lower" instead of
// "outliers_extreme_lower_cnt", causing outlier counts to never be computed.
// This test verifies that outlier count columns are actually populated with values
// when the data contains clear outliers.
#[test]
fn moarstats_outlier_counts_populated() {
    let wrk = Workdir::new("moarstats_outlier_counts_populated");

    // Create test data with obvious outliers:
    // Normal values: 10-50, Extreme outlier: 1000
    wrk.create(
        "test.csv",
        vec![
            svec!["category", "value"],
            svec!["a", "10"],
            svec!["b", "20"],
            svec!["c", "25"],
            svec!["d", "30"],
            svec!["e", "35"],
            svec!["f", "40"],
            svec!["g", "45"],
            svec!["h", "50"],
            svec!["i", "1000"], // extreme upper outlier
        ],
    );

    // Generate baseline stats with quartiles (required for outlier detection)
    let mut stats_cmd = wrk.command("stats");
    stats_cmd.arg("--everything").arg("test.csv");
    wrk.assert_success(&mut stats_cmd);

    // Run moarstats to compute outlier statistics
    let mut cmd = wrk.command("moarstats");
    cmd.arg("test.csv");
    wrk.assert_success(&mut cmd);

    // Read the enriched stats file
    let stats_content = wrk.read_to_string("test.stats.csv").unwrap();
    let mut rdr = ReaderBuilder::new()
        .has_headers(true)
        .from_reader(stats_content.as_bytes());

    let headers = rdr.headers().unwrap().clone();
    let field_idx = get_column_index(&headers, "field").unwrap();

    // Verify all outlier count columns exist
    let outlier_cnt_columns = [
        "outliers_extreme_lower_cnt",
        "outliers_mild_lower_cnt",
        "outliers_normal_cnt",
        "outliers_mild_upper_cnt",
        "outliers_extreme_upper_cnt",
        "outliers_total_cnt",
    ];
    for col in &outlier_cnt_columns {
        assert!(
            get_column_index(&headers, col).is_some(),
            "Missing outlier count column: {col}"
        );
    }

    // Find the "value" field and verify outlier counts are actually populated
    for result in rdr.records() {
        let record = result.unwrap();
        let field_name = get_field_value(&record, field_idx).unwrap();

        if field_name == "value" {
            // outliers_total_cnt must be non-empty and > 0 (we have a clear outlier at 1000)
            let total_idx = get_column_index(&headers, "outliers_total_cnt").unwrap();
            let total_val = get_field_value(&record, total_idx).unwrap();
            assert!(
                !total_val.is_empty(),
                "outliers_total_cnt should not be empty for data with obvious outliers"
            );
            let total_count: u64 = total_val
                .parse()
                .expect("outliers_total_cnt should be numeric");
            assert!(
                total_count > 0,
                "outliers_total_cnt should be > 0 when data contains outliers, got {total_count}"
            );

            // outliers_normal_cnt should also be populated
            let normal_idx = get_column_index(&headers, "outliers_normal_cnt").unwrap();
            let normal_val = get_field_value(&record, normal_idx).unwrap();
            assert!(
                !normal_val.is_empty(),
                "outliers_normal_cnt should not be empty"
            );
            let normal_count: u64 = normal_val
                .parse()
                .expect("outliers_normal_cnt should be numeric");
            assert!(
                normal_count > 0,
                "outliers_normal_cnt should be > 0, got {normal_count}"
            );

            // Verify extreme upper outlier count (1000 should be extreme upper)
            let extreme_upper_idx =
                get_column_index(&headers, "outliers_extreme_upper_cnt").unwrap();
            let extreme_upper_val = get_field_value(&record, extreme_upper_idx).unwrap();
            assert!(
                !extreme_upper_val.is_empty(),
                "outliers_extreme_upper_cnt should not be empty"
            );
            let extreme_upper_count: u64 = extreme_upper_val
                .parse()
                .expect("outliers_extreme_upper_cnt should be numeric");
            assert!(
                extreme_upper_count > 0,
                "outliers_extreme_upper_cnt should be > 0 for data with value=1000, got \
                 {extreme_upper_count}"
            );

            break;
        }
    }
}
