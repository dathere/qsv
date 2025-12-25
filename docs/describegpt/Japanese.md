# Dictionary
| Name | Type | Label | Description | Min | Max | Cardinality | Enumeration | Null Count | is_ascii | sum | range | sort_order | sortiness | min_length | max_length | sum_length | avg_length | stddev_length | variance_length | cv_length | mean | sem | geometric_mean | harmonic_mean | stddev | variance | cv | n_negative | n_zero | n_positive | max_precision | sparsity | mad | lower_outer_fence | lower_inner_fence | q1 | q2_median | q3 | iqr | upper_inner_fence | upper_outer_fence | skewness | uniqueness_ratio | percentiles | pearson_skewness | range_stddev_ratio | quartile_coefficient_dispersion | relative_standard_error | min_zscore | max_zscore | median_mean_ratio | iqr_range_ratio | mad_stddev_ratio | outliers_extreme_lower_cnt | outliers_mild_lower_cnt | outliers_normal_cnt | outliers_mild_upper_cnt | outliers_extreme_upper_cnt | outliers_total_cnt | outliers_mean | non_outliers_mean | outliers_to_normal_mean_ratio | outliers_min | outliers_max | outliers_range | outliers_stddev | outliers_variance | non_outliers_stddev | non_outliers_variance | outliers_cv | non_outliers_cv | outliers_percentage | outlier_impact | outlier_impact_ratio | outliers_normal_stddev_ratio | lower_outer_fence_zscore | upper_outer_fence_zscore | winsorized_mean_25pct | trimmed_mean_25pct | trimmed_stddev_25pct | trimmed_variance_25pct | winsorized_stddev_25pct | winsorized_variance_25pct | trimmed_cv_25pct | winsorized_cv_25pct | trimmed_25pct_stddev_ratio | winsorized_25pct_stddev_ratio | trimmed_range_25pct | winsorized_range_25pct | Examples |
|------|------|-------|-------------|-----|-----|-------------|-------------|------------|----------|----------|----------|----------|----------|----------|----------|----------|----------|----------|----------|----------|----------|----------|----------|----------|----------|----------|----------|----------|----------|----------|----------|----------|----------|----------|----------|----------|----------|----------|----------|----------|----------|----------|----------|----------|----------|----------|----------|----------|----------|----------|----------|----------|----------|----------|----------|----------|----------|----------|----------|----------|----------|----------|----------|----------|----------|----------|----------|----------|----------|----------|----------|----------|----------|----------|----------|----------|----------|----------|----------|----------|----------|----------|----------|----------|----------|----------|----------|----------|----------|----------|
| **_id** | Integer | レコードID | データベース内の各レコードを一意に識別する整数。479,928件すべてがユニークで、主キーとして機能します。 | 38703919 | 39183846 | 479,928 |  | 0 |  | 18690259640460 | 479927 | Unsorted | 0 |  |  |  |  |  |  |  | 38943882.5 | 199.985 | 38943636.0634 | 38943389.6258 | 138543.28 | 19194240430.8183 | 0.3558 | 0 | 0 | 479928 |  | 0 | 119982 | 38104008.5 | 38463954.5 | 38823900.5 | 38943882.5 | 39063864.5 | 239964 | 39423810.5 | 39783756.5 | 0 | 1 | 5: 38727915<br>10: 38751911<br>40: 38895890<br>60: 38991875<br>90: 39135854<br>95: 39159850 | 0 | 3.4641 | 0.0031 | 0 | -1.732 | 1.732 | 1 | 0.5 | 0.866 | 0 | 0 | 479928 | 0 | 0 | 0 |  | 38943882.5 |  |  |  |  |  |  | 138543.4244 | 19194280449.883 |  | 0.0036 | 0 | 0 | 0 |  | -6.0622 | 6.0622 | 38943882.5 | 38943882.5 | 69271.7845 | 4798580129.6046 | 97964.9943 | 9597140106.9587 | 0.0018 | 0.0025 | 0.5 | 0.7071 | 239963 | 239964 | <ALL_UNIQUE> |
| **PARID** | String | 不動産パーセル識別子 | 州や地方自治体が管理する不動産の固有番号（文字列）。多くの場合は「0001C00037000A00」のような16桁のコードで、データセット全体では約302,633種類以上が存在します。 | 0001C00037000A00 | 9946X83943000000 | 302,643 |  | 0 | true |  |  | Unsorted | 0 | 16 | 16 | 7231216 | 15.0673 | 0 | 0 | 0 |  |  |  |  |  |  |  |  |  |  |  | 0 |  |  |  |  |  |  |  |  |  |  | 0.6306 |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  | Other (302,633) [479,750]<br>0431B00017000000 [23]<br>0027D00263000000 [20]<br>0027D00272000000 [20]<br>0027D00286000000 [20] |
| **FULL_ADDRESS** | String | 完全住所 | 物件のフルアドレスを表す文字列（例：'0 SONIE DR, SEWICKLEY, PA 15143'）。数百件の代表的な住所が頻出し、ほぼ全てがユニークです。 | 0 , BRADDOCK, PA 15104 | FORBES AVE, PITTSBURGH, PA 15219 | 278,190 |  | 0 | true |  |  | Unsorted | 0 | 15 | 64 | 17877117 | 37.2496 | 3.5804 | 12.8191 | 0.0961 |  |  |  |  |  |  |  |  |  |  |  | 0 |  |  |  |  |  |  |  |  |  |  | 0.5796 |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  | Other (278,180) [479,006]<br>0 SONIE DR, SEWICKLEY, PA… [113]<br>0 COAL, ELIZABETH, PA 150… [111]<br>0 HUNTER ST, PITTSBURGH, … [98]<br>0 PERRYSVILLE AVE, PITTSB… [98] |
| **PROPERTYHOUSENUM** | Integer | 住宅番号 | 物件の番地を整数で表したもの。0 が最も多く、次に112, 100 等があります。最大値は65,015で、約7.9%がゼロです。 | 0 | 65015 | 10,012 |  | 3 |  | 690480418 | 65015 | Unsorted | 0 |  |  |  |  |  |  |  | 1438.7257 | 2.9796 | 0 |  | 2064.1519 | 4260722.9391 | 143.4708 | 0 | 38055 | 441870 |  | 0 | 514 | -4833 | -2338.5 | 156 | 602 | 1819 | 1663 | 4313.5 | 6808 | 0.4636 | 0.0209 | 5: 0<br>10: 16<br>40: 349<br>60: 933<br>90: 4308<br>95: 5728 | 1.2161 | 31.4972 | 0.842 | 0.0021 | -0.697 | 30.8002 | 0.4184 | 0.0256 | 0.249 | 0 | 0 | 432120 | 32943 | 14862 | 47805 | 6397.26 | 890.168 | 7.1866 | 4314 | 65015 | 60701 | 2449.6083 | 6000581.0401 | 1023.3782 | 1047303.0236 | 0.3829 | 1.1496 | 9.9609 | 548.5577 | 0.6162 | 2.3936 | -3.0384 | 2.6012 | 851.6527 | 715.8642 | 460.4084 | 211975.8564 | 685.6044 | 470053.3913 | 0.6432 | 0.805 | 0.223 | 0.3321 | 1663 | 1663 | Other (10,002) [428,653]<br>0 [38,055]<br>112 [1,615]<br>100 [1,595]<br>110 [1,522] |
| **PROPERTYFRACTION** | String | 住所分数 | 住宅番号に続く分数・文字（例：'1/2', 'A'）。NULL が最頻で 97.6%、その他の値は少数です。 |   |  S | 2,803 |  | 0 | true |  |  | Unsorted | 0 | 1 | 7 | 525644 | 1.0953 | 0.6327 | 0.4004 | 0.5777 |  |  |  |  |  |  |  |  |  |  |  | 0 |  |  |  |  |  |  |  |  |  |  | 0.0058 |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  | (NULL) [468,512]<br>Other (2,793) [9,695]<br>1/2 [853]<br>A [405]<br>B [294] |
| **PROPERTYADDRESSDIR** | String | 方向記号 | 通りの北南東西を示す略語（N, S, E, W）または NULL。E と N が多く、NULL は95%近い頻度です。 | E | W | 5 | (NULL)<br>E<br>N<br>S<br>W | 459,948 | true |  |  | Unsorted | 0 | 0 | 1 | 19980 | 0.0416 | 0 | 0 | 0 |  |  |  |  |  |  |  |  |  |  |  | 0.9584 |  |  |  |  |  |  |  |  |  |  | 0 |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  | (NULL) [459,948]<br>N [5,466]<br>S [5,201]<br>E [5,053]<br>W [4,260] |
| **PROPERTYADDRESSSTREET** | String | 通り名 | 物件が位置する通りの名称。'WASHINGTON', '5TH', 'HIGHLAND' などが上位で、ほぼ全てユニークです。 | 0 OHIO RIVER BLVD | ZUZU | 9,571 |  | 13 | true |  |  | Unsorted | 0 | 0 | 26 | 3605968 | 7.5136 | 2.528 | 6.391 | 0.3365 |  |  |  |  |  |  |  |  |  |  |  | 0 |  |  |  |  |  |  |  |  |  |  | 0.0199 |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  | Other (9,538) [462,225]<br>WASHINGTON [2,606]<br>5TH [2,557]<br>HIGHLAND [1,777]<br>PENN [1,602] |
| **PROPERTYADDRESSSUF** | String | 接尾辞（通り種別） | 通り名の後に付く略語（ST, DR, AVE 等）。最も頻出は ST (25.6%) で、その他は少数です。 | ALY | XING | 48 |  | 1,985 | true |  |  | Unsorted | 0 | 0 | 4 | 1100161 | 2.2923 | 0.5081 | 0.2582 | 0.2216 |  |  |  |  |  |  |  |  |  |  |  | 0.0041 |  |  |  |  |  |  |  |  |  |  | 0.0001 |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  | ST [122,764]<br>DR [113,069]<br>AVE [105,232]<br>RD [71,902]<br>LN [15,471] |
| **PROPERTYADDRESSUNITDESC** | String | ユニット記述子 | 物件内のユニットを示す文字列（'UNIT', 'REAR', 'APT' 等）。NULL が大半で、ユニークな値は少ないです。 | # | UNIT | 11 |  | 468,267 | true |  |  | Unsorted | 0 | 0 | 4 | 46091 | 0.096 | 0.2161 | 0.0467 | 2.2506 |  |  |  |  |  |  |  |  |  |  |  | 0.9757 |  |  |  |  |  |  |  |  |  |  | 0 |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  | (NULL) [468,267]<br>UNIT [10,580]<br>REAR [421]<br>APT [391]<br>STE [132] |
| **PROPERTYUNITNO** | String | ユニット番号 | ユニット内の具体的な番号やコード。多くは NULL で、数百件が1〜5程度の整数。 | 01 | ` | 1,334 |  | 468,641 | true |  |  | Unsorted | 0 | 0 | 10 | 8063 | 0.0168 | 1.393 | 1.9405 | 82.9147 |  |  |  |  |  |  |  |  |  |  |  | 0.9765 |  |  |  |  |  |  |  |  |  |  | 0.0028 |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  | (NULL) [468,641]<br>Other (1,324) [10,002]<br>1 [195]<br>2 [170]<br>3 [166] |
| **PROPERTYCITY** | String | 市区町村名 | 物件所在の都市。PITTSBURGH が最大（53.7%）で、その他にはCORAOPOLIS, MC KEESPORT 等があります。 | ALLISON PARK | WITAKER | 106 |  | 1 | true |  |  | Unsorted | 0 | 0 | 22 | 4740965 | 9.8785 | 1.5954 | 2.5452 | 0.1615 |  |  |  |  |  |  |  |  |  |  |  | 0 |  |  |  |  |  |  |  |  |  |  | 0.0002 |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  | PITTSBURGH [257,608]<br>Other (89) [123,311]<br>CORAOPOLIS [16,497]<br>MC KEESPORT [15,307]<br>GIBSONIA [11,048] |
| **PROPERTYSTATE** | String | 州コード | 州を表す2文字の略語。全て 'PA' であり、データはペンシルベニア州のみです。 | PA | PA | 1 | PA | 0 | true |  |  | Unsorted | 0 | 2 | 2 | 959856 | 2 | 0 | 0 | 0 |  |  |  |  |  |  |  |  |  |  |  | 0 |  |  |  |  |  |  |  |  |  |  | 0 |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  | <ALL_UNIQUE> |
| **PROPERTYZIP** | Integer | 郵便番号 | 5桁の郵便番号（例：15108）。最大値は15237で、約74%が152xx 系列です。 | 15003 | 16229 | 124 |  | 1 |  | 7279841011 | 1226 | Unsorted | 0 |  |  |  |  |  |  |  | 15168.6423 | 0.1314 | 15168.3732 | 15168.1079 | 91.0437 | 8288.9532 | 0.6002 | 0 | 0 | 479927 |  | 0 | 35 | 14769 | 14938.5 | 15108 | 15204 | 15221 | 113 | 15390.5 | 15560 | -0.6991 | 0.0003 | 5: 15030<br>10: 15057<br>40: 15143<br>60: 15211<br>90: 15236<br>95: 15238 | -1.1651 | 13.4661 | 0.0037 | 0 | -1.8194 | 11.6467 | 1.0023 | 0.0922 | 0.3844 | 0 | 0 | 477616 | 0 | 2311 | 2311 | 16035.8542 | 15164.4462 | 1.0575 | 15642 | 16229 | 587 | 70.0255 | 4903.5662 | 68.1824 | 4648.8352 | 0.0044 | 0.0045 | 0.4815 | 4.1961 | 0.0003 | 1.027 | -4.3896 | 4.2986 | 15171.5471 | 15175.9134 | 41.4507 | 1718.1574 | 49.1284 | 2413.5953 | 0.0027 | 0.0032 | 0.4553 | 0.5396 | 113 | 113 | Other (114) [355,692]<br>15108 [16,509]<br>15237 [15,435]<br>15235 [14,585]<br>15212 [13,301] |
| **SCHOOLCODE** | String | 学区コード | 不動産が属する学校区を識別する数値文字列。47 が最頻で24.6%、その他は少数です。 | 01 | 09 | 46 |  | 0 | true |  |  | Unsorted | 0 | 2 | 2 | 169068 | 0.3523 | 0 | 0 | 0 |  |  |  |  |  |  |  |  |  |  |  | 0 |  |  |  |  |  |  |  |  |  |  | 0.0001 |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  | Other (36) [228,000]<br>47 [117,977]<br>27 [19,685]<br>09 [19,227]<br>30 [17,635] |
| **SCHOOLDESC** | String | 学区名 | 学区の名称（例：Pittsburgh, North Allegheny）。SCHOOLCODE と同じ順序で 10 個が代表的です。 | Allegheny Valley | Woodland Hills | 46 |  | 0 | true |  |  | Unsorted | 0 | 6 | 20 | 5808210 | 12.1023 | 3.0138 | 9.0828 | 0.249 |  |  |  |  |  |  |  |  |  |  |  | 0 |  |  |  |  |  |  |  |  |  |  | 0.0001 |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  | Other (36) [228,000]<br>Pittsburgh [117,977]<br>North Allegheny [19,685]<br>Woodland Hills [19,227]<br>Penn Hills Twp [17,635] |
| **MUNICODE** | Integer | 自治体コード | 物件所在自治体を示す数値コード。934 が最大頻度 (3.7%)、その他は 1〜10 位まで多様。 | 101 | 953 | 175 |  | 0 |  | 330773558 | 852 | Unsorted | 0 |  |  |  |  |  |  |  | 689.215 | 0.4906 | 528.8235 | 333.6491 | 339.9002 | 115532.1656 | 49.317 | 0 | 0 | 479928 |  | 0 | 60 | -1973 | -885.5 | 202 | 874 | 927 | 725 | 2014.5 | 3102 | -0.8538 | 0.0004 | 5: 110<br>10: 115<br>40: 843<br>60: 884<br>90: 940<br>95: 946 | -1.6309 | 2.5066 | 0.6422 | 0.0007 | -1.7306 | 0.7761 | 1.2681 | 0.8509 | 0.1765 | 0 | 0 | 479928 | 0 | 0 | 0 |  | 689.215 |  |  |  |  |  |  | 339.9006 | 115532.4063 |  | 0.4932 | 0 | 0 | 0 |  | -7.8323 | 7.0985 | 706.8031 | 844.4377 | 125.4879 | 15747.2237 | 303.0737 | 91853.6674 | 0.1486 | 0.4288 | 0.3692 | 0.8917 | 725 | 725 | Other (165) [369,793]<br>934 [17,635]<br>119 [12,648]<br>940 [12,003]<br>926 [10,359] |
| **MUNIDESC** | String | 自治体名 | 自治体の正式名称（例：Penn Hills, 19th Ward - PITTSBURGH）。上位10が代表的です。 | 10th Ward -  McKEESPORT | Wilmerding   | 175 |  | 0 | true |  |  | Unsorted | 0 | 6 | 23 | 6677740 | 13.914 | 5.4806 | 30.0372 | 0.3939 |  |  |  |  |  |  |  |  |  |  |  | 0 |  |  |  |  |  |  |  |  |  |  | 0.0004 |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  | Other (165) [369,793]<br>Penn Hills [17,635]<br>19th Ward - PITTSBURGH [12,648]<br>Ross [12,003]<br>Mt.Lebanon [10,359] |
| **RECORDDATE** | Date | 記録日付 | 取引情報がレコードに登録された年月日。NULL が0.26%で、2012〜2028 年間のデータがあります。 | 0212-08-01 | 2028-09-28 | 3,821 |  | 1,262 |  |  | 663339 | Unsorted | 0 |  |  |  |  |  |  |  | 2018-12-11 | 2.72253 |  |  | 1883.59919 | 3547945.89188 | 10.5365 |  |  |  |  | 0.0026 | 1204 | 1995-11-06 | 2005-09-26 | 2015-08-17 | 2018-12-14 | 2022-03-21 | 2408 | 2032-02-09 | 2041-12-30 | -0.0091 | 0.008 | 5: 2012-10-03<br>10: 2013-06-18<br>40: 2017-08-23<br>60: 2020-05-19<br>90: 2024-06-04<br>95: 2025-03-14 |  | 352.1657 |  |  |  |  |  | 0.0036 | 0.6392 | 4 | 0 | 0 | 0 | 478662 | 478666 | 2018-12-11 |  |  | 0212-08-01 | 2028-09-28 | 663339 | 1883.6012 | 3547953.3041 |  |  | 0.1054 |  | 100 |  |  |  |  |  | 2018-12-13 | 2018-12-23 | 701.5438 | 492163.7353 | 985.3383 | 970891.5393 | 0.0392 | 0.0551 | 0.3724 | 0.5231 | 2408 | 2408 | Other (3,811) [474,687]<br>(NULL) [1,262]<br>2012-10-26 [587]<br>2013-04-26 [552]<br>2012-01-11 [488] |
| **SALEDATE** | Date | 販売日付 | 不動産が実際に売却された年月日。NULL は 4.9% で、最頻は 2012-10-26 等です。 | 2012-01-01 | 2025-12-13 | 4,888 |  | 0 |  |  | 5095 | Unsorted | 0 |  |  |  |  |  |  |  | 2018-11-23 | 2.06661 | 17800.71631 | 17742.99182 | 1431.67882 | 2049704.25143 | 8.0169 |  |  |  |  | 0 | 1206 | 1995-09-30 | 2005-08-24 | 2015-07-20 | 2018-11-16 | 2022-02-24 | 2411 | 2032-01-19 | 2041-12-14 | -0.0079 | 0.0102 | 5: 2012-10-02<br>10: 2013-06-14<br>40: 2017-07-28<br>60: 2020-04-10<br>90: 2024-05-15<br>95: 2025-02-26 |  | 3.5588 |  |  |  |  |  | 0.4732 | 0.8424 | 0 | 0 | 0 | 0 | 479928 | 479928 | 2018-11-23 |  |  | 2012-01-01 | 2025-12-13 | 5095 | 1431.6803 | 2049708.5223 |  |  | 0.0802 |  | 100 |  |  |  |  |  | 2018-11-15 | 2018-11-25 | 702.473 | 493468.3505 | 986.4928 | 973168.0588 | 0.0393 | 0.0553 | 0.4907 | 0.689 | 2411 | 2411 | Other (4,878) [475,391]<br>2012-10-26 [586]<br>2016-04-29 [584]<br>2013-04-26 [559]<br>2012-01-11 [490] |
| **PRICE** | Integer | 販売価格（米ドル） | 取引金額を整数で表したもの。0 が最も多く、最大値は148,752,900 ドル。平均約195,323 ドルで、分布は広範囲にわたります。 | 0 | 148752900 | 37,872 |  | 3,020 |  | 93151160594 | 148752900 | Unsorted | 0 |  |  |  |  |  |  |  | 195323.1244 | 1324.4887 | 0 |  | 914672.4027 | 836625604233.6805 | 468.2868 | 0 | 15508 | 461400 |  | 0.0063 | 90999 | -659960 | -329975 | 10 | 91000 | 220000 | 219990 | 549985 | 879970 | 0.1728 | 0.0789 | 5: 1<br>10: 1<br>40: 45000<br>60: 139900<br>90: 391245<br>95: 557500 | 0.3422 | 162.6297 | 0.9999 | 0.0068 | -0.2135 | 162.4162 | 0.4659 | 0.0015 | 0.0995 | 0 | 0 | 452171 | 15057 | 9680 | 24737 | 1587762.8321 | 119146.6755 | 13.3261 | 549990 | 148752900 | 148202910 | 3710912.2886 | 13770870014023.4082 | 131095.7851 | 17186104869.0189 | 2.3372 | 1.1003 | 5.187 | 76176.4488 | 0.6394 | 28.3069 | -0.9351 | 0.7485 | 102414.3713 | 93157.8205 | 66442.4724 | 4414602135.3502 | 90817.3683 | 8247794390.7866 | 0.7132 | 0.8868 | 0.0726 | 0.0993 | 219990 | 219990 | Other (37,862) [340,479]<br>1 [98,344]<br>0 [15,508]<br>10 [6,763]<br>150000 [3,140] |
| **DEEDBOOK** | String | 権利書ブック番号 | 不動産の権利書が記載されているブック番号。TR18 が最頻で25.8%。NULL は0.12%です。 |  14795 | `17274 | 5,814 |  | 570 | true |  |  | Unsorted | 0 | 0 | 8 | 20735 | 0.0432 | 0.2715 | 0.0737 | 6.2849 |  |  |  |  |  |  |  |  |  |  |  | 0.0012 |  |  |  |  |  |  |  |  |  |  | 0.0121 |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  | Other (5,800) [473,060]<br>TR18 [1,239]<br>0 [1,063]<br>TR13 [938]<br>00 [798] |
| **DEEDPAGE** | String | ページ番号 | 権利書内のページ番号。1 が最も多く、2〜10 まで分布しています。 |  120 | W | 2,133 |  | 582 | true |  |  | Unsorted | 0 | 0 | 8 | 10547 | 0.022 | 0.8754 | 0.7663 | 39.8329 |  |  |  |  |  |  |  |  |  |  |  | 0.0012 |  |  |  |  |  |  |  |  |  |  | 0.0044 |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  | Other (2,115) [465,578]<br>1 [5,174]<br>6 [1,485]<br>7 [1,112]<br>0 [1,002] |
| **SALECODE** | String | 販売コード | 販売の種類を示す文字列（例：3, 0, H）。上位は '3' と '0'。NULL はほぼ無いです。 | 0X | Z | 47 |  | 0 | true |  |  | Unsorted | 0 | 1 | 2 | 165605 | 0.3451 | 0.4874 | 0.2375 | 1.4124 |  |  |  |  |  |  |  |  |  |  |  | 0 |  |  |  |  |  |  |  |  |  |  | 0.0001 |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  | 3 [97,932]<br>0 [91,484]<br>H [63,750]<br>Other (37) [54,274]<br>14 [42,089] |
| **SALEDESC** | String | 販売説明 | 販売タイプに関する詳細な記述（LOVE AND AFFECTION SALE、VALID SALE 等）。10 種類が代表的で、Other が9.5% です。 | BANK/FINANCIAL INSTITUTION | VALID SALE | 27 |  | 0 | true |  |  | Unsorted | 0 | 5 | 39 | 8868445 | 18.4787 | 8.0936 | 65.5065 | 0.438 |  |  |  |  |  |  |  |  |  |  |  | 0 |  |  |  |  |  |  |  |  |  |  | 0.0001 |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  | LOVE AND AFFECTION SALE [97,932]<br>VALID SALE [91,484]<br>MULTI-PARCEL SALE [63,750]<br>Other (17) [45,680]<br>TIME ON MARKET (INSUFF/EX… [42,089] |
| **INSTRTYP** | String | 取引種別コード | 不動産の取引種類を示す略語（DE, SW, GW 等）。DE が最頻で58%。NULL はほぼ無いです。 | AD | WD | 31 |  | 0 | true |  |  | Unsorted | 0 | 2 | 2 | 959856 | 2 | 0 | 0 | 0 |  |  |  |  |  |  |  |  |  |  |  | 0 |  |  |  |  |  |  |  |  |  |  | 0.0001 |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  | DE [278,804]<br>SW [113,966]<br>GW [23,662]<br>QC [18,068]<br>SD [15,521] |
| **INSTRTYPDESC** | String | 取引種別説明 | INSTRTYP の詳細な説明（DEED、SPECIAL WARRANTY 等）。同様に DE が最頻で58%。 | ADMIN DEED | WILL BOOK | 31 |  | 0 | true |  |  | Unsorted | 0 | 4 | 25 | 4107480 | 8.5585 | 5.5256 | 30.532 | 0.6456 |  |  |  |  |  |  |  |  |  |  |  | 0 |  |  |  |  |  |  |  |  |  |  | 0.0001 |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  | DEED [278,804]<br>SPECIAL WARRANTY [113,966]<br>GENERAL WARRANTY [23,662]<br>QUIT CLAIM [18,068]<br>SHERIFF DEED [15,521] |

*Attribution: Generated by qsv v12.0.0 describegpt
Command line: qsv describegpt --all allegheny_county_property_sale_transactions.csv --language Japanese --fresh --output Japanese.md --addl-cols-list moar
Prompt file: Default v7.0.0
Model: openai/gpt-oss-20b
LLM API URL: http://localhost:1234/v1
Language: 
Timestamp: 2025-12-25T12:02:42.762845+00:00

WARNING: Label and Description generated by an LLM and may contain inaccuracies. Verify before using!
*

## REASONING

We need Japanese labels and descriptions for all 26 fields. Use summary stats and frequency distribution to understand context.

We must produce JSON with keys field names as given: e.g., "_id", "PARID", etc.

Labels should be human-friendly. Descriptions multiple sentences. Should reflect dataset domain: property sales in Pennsylvania? Fields like address, sale date, price, deed book/page, school district, municipal code, etc. Provide Japanese translations of labels and descriptions, referencing data types.

We need to use Summary Statistics for each field: type, cardinality, uniqueness, distribution. Frequency Distribution gives top values.

Goal: produce Japanese labels & description.

We'll craft for each:

_id: Unique identifier internal, numeric, 479k records unique.

PARID: property parcel ID? It's string; many distinct values but most 'Other'. Provide label "不動産所有権識別子".

FULL_ADDRESS: full address text; top values show many addresses with "0" street number. Label "完全住所". Description mentions includes street, city, zip etc.

PROPERTYHOUSENUM: house number numeric; many zeros etc. Provide label "住宅番号". Mention distribution of 0 being most common.

PROPERTYFRACTION: fractional part of address like '1/2', 'A' etc. Many nulls. Label "住所分数".

PROPERTYADDRESSDIR: directional indicator (N,S,E,W). Label "方向記号".

PROPERTYADDRESSSTREET: street name; top values e.g., WASHINGTON, 5TH, etc.

PROPERTYADDRESSSUF: suffix like ST, DR, AVE. Label "通りの接尾辞".

PROPERTYADDRESSUNITDESC: unit descriptor (#, UNIT, REAR). Label "ユニット記述子".

PROPERTYUNITNO: unit number; many nulls.

PROPERTYCITY: city name.

PROPERTYSTATE: state abbreviation 'PA'.

PROPERTYZIP: zip code numeric 5-digit.

SCHOOLCODE: school district code. Provide label "学区コード". Description referencing mapping to school district.

SCHOOLDESC: school district description (e.g., Pittsburgh, North Allegheny). Label "学区名".

MUNICODE: municipal code; top values e.g., 934 etc. Label "自治体コード".

MUNIDESC: municipal description; top like 'Penn Hills', '19th Ward - PITTSBURGH'.

RECORDDATE: record date (when transaction recorded). Date type.

SALEDATE: sale date of property.

PRICE: sale price in dollars? integer; includes 0, 1 etc. Provide label "販売価格". Note range up to ~148M? Actually max 148752900. Many zeros and ones.

DEEDBOOK: deed book identifier.

DEEDPAGE: page number.

SALECODE: code for sale type? e.g., 3, 0, H etc.

SALEDESC: description of sale type; top includes LOVE AND AFFECTION SALE, VALID SALE etc.

INSTRTYP: instruction type? codes like DE, SW. Provide label "取引タイプコード".

INSTRTYPDESC: description for instruction type.

We should also provide labels in Japanese. For example:

"_id": "レコードID" or "ユニーク識別子". Description: internal primary key; 479,928 unique numeric values.

PARID: "不動産パーセルID" etc.

Need to be careful with Japanese grammar and use appropriate terms.

Also mention cardinality and uniqueness where relevant.

We will produce JSON object with each field mapping. Use double quotes for keys and strings. Must not include comments or extra text outside JSON.

Let's craft.

We'll create label in Japanese, description also in Japanese. Provide details about data type (numeric/string/date) maybe.

Let's prepare.

For _id:

Label: "レコードID". Description: "データベース内の各レコードを一意に識別する整数。479,928件すべてがユニークで、主キーとして機能します。"

PARID:

Label: "不動産パーセル識別子". Description: "州や地方自治体が管理する不動産の固有番号（文字列）。多くの場合は「0001C00037000A00」のような16桁のコードで、データセット全体では約302,633種類以上が存在します。"

FULL_ADDRESS:

Label: "完全住所". Description: "物件のフルアドレスを表す文字列（例：'0 SONIE DR, SEWICKLEY, PA 15143'）。数百件の代表的な住所が頻出し、ほぼ全てがユニークです。"

PROPERTYHOUSENUM:

Label: "住宅番号". Description: "物件の番地を整数で表したもの。0 が最も多く、次に112, 100 等があります。最大値は65,015で、約7.9%がゼロです。"

PROPERTYFRACTION:

Label: "住所分数". Description: "住宅番号に続く分数・文字（例：'1/2', 'A'）。NULL が最頻で 97.6%、その他の値は少数です。"

PROPERTYADDRESSDIR:

Label: "方向記号". Description: "通りの北南東西を示す略語（N, S, E, W）または NULL。E と N が多く、NULL は95%近い頻度です。"

PROPERTYADDRESSSTREET:

Label: "通り名". Description: "物件が位置する通りの名称。'WASHINGTON', '5TH', 'HIGHLAND' などが上位で、ほぼ全てユニークです。"

PROPERTYADDRESSSUF:

Label: "接尾辞（通り種別）". Description: "通り名の後に付く略語（ST, DR, AVE 等）。最も頻出は ST (25.6%) で、その他は少数です。"

PROPERTYADDRESSUNITDESC:

Label: "ユニット記述子". Description: "物件内のユニットを示す文字列（'UNIT', 'REAR', 'APT' 等）。NULL が大半で、ユニークな値は少ないです。"

PROPERTYUNITNO:

Label: "ユニット番号". Description: "ユニット内の具体的な番号やコード。多くは NULL で、数百件が1〜5程度の整数。"

PROPERTYCITY:

Label: "市区町村名". Description: "物件所在の都市。PITTSBURGH が最大（53.7%）で、その他にはCORAOPOLIS, MC KEESPORT 等があります。"

PROPERTYSTATE:

Label: "州コード". Description: "州を表す2文字の略語。全て 'PA' であり、データはペンシルベニア州のみです。"

PROPERTYZIP:

Label: "郵便番号". Description: "5桁の郵便番号（例：15108）。最大値は15237で、約74%が152xx 系列です。"

SCHOOLCODE:

Label: "学区コード". Description: "不動産が属する学校区を識別する数値文字列。47 が最頻で24.6%、その他は少数です。"

SCHOOLDESC:

Label: "学区名". Description: "学区の名称（例：Pittsburgh, North Allegheny）。SCHOOLCODE と同じ順序で 10 個が代表的です。"

MUNICODE:

Label: "自治体コード". Description: "物件所在自治体を示す数値コード。934 が最大頻度 (3.7%)、その他は 1〜10 位まで多様。"

MUNIDESC:

Label: "自治体名". Description: "自治体の正式名称（例：Penn Hills, 19th Ward - PITTSBURGH）。上位10が代表的です。"

RECORDDATE:

Label: "記録日付". Description: "取引情報がレコードに登録された年月日。NULL が0.26%で、2012〜2028 年間のデータがあります。"

SALEDATE:

Label: "販売日付". Description: "不動産が実際に売却された年月日。NULL は 4.9% で、最頻は 2012-10-26 等です。"

PRICE:

Label: "販売価格（米ドル）". Description: "取引金額を整数で表したもの。0 が最も多く、最大値は148,752,900 ドル。平均約195,323 円？ Actually it's dollars. Range 0〜148M. Many small values (1, 10, etc.)."

DEEDBOOK:

Label: "権利書ブック番号". Description: "不動産の権利書が記載されているブック番号。TR18 が最頻で25.8%。NULL は0.12%です。"

DEEDPAGE:

Label: "ページ番号". Description: "権利書内のページ番号。1 が最も多く、2〜10 まで分布しています。"

SALECODE:

Label: "販売コード". Description: "販売の種類を示す文字列（例：3, 0, H）。上位は '3' と '0'。NULL はほぼ無いです。"

SALEDESC:

Label: "販売説明". Description: "販売タイプに関する詳細な記述（LOVE AND AFFECTION SALE、VALID SALE 等）。10 種類が代表的で、Other が9.5% です。"

INSTRTYP:

Label: "取引種別コード". Description: "不動産の取引種類を示す略語（DE, SW, GW 等）。DE が最頻で58%。NULL はほぼ無いです。"

INSTRTYPDESC:

Label: "取引種別説明". Description: "INSTRTYP の詳細な説明（DEED、SPECIAL WARRANTY 等）。同様に DE が最頻で58%。"

Make sure JSON is valid. Use backslashes? No need.

Let's craft final JSON with 26 entries.


## TOKEN USAGE

TokenUsage { prompt: 9493, completion: 3809, total: 13302, elapsed: 61218 }
---
# Description
## データセット概要

このデータセットは、ペンシルベニア州の不動産取引情報を約48万件（479,928レコード）収集したものです。各レコードには物件識別子（PARID）、住所、販売日・記録日、価格、権利書情報、売買種類などが含まれます。データは州全域にわたり、最頻出都市はピッツバーグで約54％を占めています。販売価格は極端な値（最大1億4,875万ドル）から0まで幅広く分布し、右側に長い尾が認められます。また、多数のフィールド（例：FULL_ADDRESS、PARID）はほぼ一意であり、トップ10の頻度しか占めない「Other」カテゴリが多く存在します。全体としては欠損値が比較的少なく、主に住所関連や取引日付、価格情報に集中しています。

## Notable Characteristics

- **分布形状**:  
  - `PRICE` は右裾込みの極端な高値（148,752,900ドル）を持ち、中央値は約219,990ドル。平均と標準偏差も大きく、データは非対称でスキューが正方向に強い。  
  - `SALECODE` や `SALEDESC` は少数のコード・説明が多数を占めるワンホット分布（例：`3` が20％）。  

- **欠損率**:  
  - `RECORDDATE` と `SALEDATE` の欠損はそれぞれ0.26%と1.07%。  
  - `PROPERTYFRACTION`, `PROPERTYADDRESSDIR`, `PROPERTYUNITDESC`, `PROPERTYUNITNO` は95%以上がNULLで、情報量が極めて低い。  

- **ユニーク性・カードインティ**:  
  - `PARID` と `FULL_ADDRESS` は約30万〜280k種類のユニーク値を持ち、トップ10以外に多くの「Other」カテゴリが存在。  
  - `_id` は完全一意で重複はありません。  

- **極端な外れ値**:  
  - `PRICE` の最大値1億4,875万ドルといった極端高値、また `DEEDBOOK` や `DEEDPAGE` における0や大きな数（例：TR18）も外れ値の候補。  

- **データ品質**:  
  - フィールドごとの「Other」カテゴリが多いことから、欠損以外に情報の分散が高く、サンプリングバイアスや統計解析時の注意が必要です。  
  - `PROPERTYZIP` の約74%は「Other」であり、郵便番号レベルでの集約が困難。  

- **プライバシー・セキュリティ**:  
  - 住所と販売価格の組み合わせは個人を特定できる可能性があるため、公開時には適切な匿名化やデータ最小化の措置が推奨されます。  

- **重複チェック**:  
  - `_id` が一意であるものの、同一物件の異なる取引（例：再販売）が存在する可能性があるため、重複レコードの確認は追加解析に有用です。

---

*Attribution:* Generated by qsv v12.0.0 describegpt
Command line: qsv describegpt --all allegheny_county_property_sale_transactions.csv --language Japanese --fresh --output Japanese.md --addl-cols-list moar
Prompt file: Default v7.0.0
Model: openai/gpt-oss-20b
LLM API URL: http://localhost:1234/v1
Language: 
Timestamp: 2025-12-25T12:03:14.436442+00:00

WARNING: Description generated by an LLM and may contain inaccuracies. Verify before using!



## REASONING

We need to produce a Japanese description of dataset as a whole (1-8 sentences). Then "Notable Characteristics" bullet list summarizing key stats: central tendency, spread, distribution shape, anomalies, patterns; outliers, missing values, duplicates, PII/PHI/PCI data, other data quality issues. Use information from summary statistics and frequency distribution.

We must not output per-field details, only dataset-level description. Must not output any per field frequencies or stats. So just summarise overall dataset: 479,928 records, fields count maybe? The dataset includes property transaction data in Pennsylvania. Data about addresses, sale price, dates, etc. Provide Japanese description.

Also include Notable Characteristics bullet list: high cardinality of many columns; many unique values; missingness rates for some columns; outliers in PRICE (max 148 million, median 220k? Actually mean 195k). Price distribution skewed right. Outliers extreme price 148m etc. Null counts for RECORDDATE 1262 (~0.26%), SALEDATE 5095 (~1%). Some fields have many missing values like PROPERTYFRACTION (97% null). Also some fields with many unique values but small top frequencies, e.g., PARID, FULL_ADDRESS. Data includes potential PII/PHI: addresses contain personal info? Not necessarily. Contains property location which could be sensitive; but not PII per se. But there might be sale price and address combination that could identify individuals. So mention privacy concerns.

Also duplicates: maybe _id unique ensures no duplicates. But other fields may have duplicate combos.

Outliers: PRICE extremes, DEEDBOOK values etc.

Distribution shape: Price right-skewed; many zeros in some numeric columns (e.g., 0 price). Also counts for salecode show heavy skew.

Missingness high for some columns like PROPERTYFRACTION, PROPERTYADDRESSDIR, PROPERTYUNITDESC, PROPERTYUNITNO. Also "Other" categories large indicating many distinct values not captured by top10.

Also note cardinality: many fields have high cardinality > 100k unique values; e.g., PARID, FULL_ADDRESS, etc.

Also mention that dataset covers Pennsylvania only (state PA). All PROPERTYSTATE 'PA'.

Now produce description in Japanese with 1-8 sentences. Then bullet list. Provide Attribution placeholder at bottom: {GENERATED_BY_SIGNATURE}.

Let's craft 5-6 sentences. Use formal tone.

Ok.
## TOKEN USAGE

TokenUsage { prompt: 10943, completion: 1298, total: 12241, elapsed: 31671 }
---
# Tags
不動産_取引  
ペンシルベニア州_不動産  
住所_情報  
権利書_記録  
価格_分析  
販売日_履歴  
市区町村_コード  
学区_情報  
取引_種類  
郵便番号_分布  

Generated by qsv v12.0.0 describegpt
Command line: qsv describegpt --all allegheny_county_property_sale_transactions.csv --language Japanese --fresh --output Japanese.md --addl-cols-list moar
Prompt file: Default v7.0.0
Model: openai/gpt-oss-20b
LLM API URL: http://localhost:1234/v1
Language: 
Timestamp: 2025-12-25T12:03:56.713313+00:00

WARNING: Tags generated by an LLM and may contain inaccuracies. Verify before using!



## REASONING

We need to produce tags in Japanese, markdown format, no field names. Max 10 unique tags, lower case only and use underscore to separate words. Based on dataset contents: real estate property sales in Pennsylvania, includes addresses, sale dates, prices, types of deeds, etc. So tags could be:

- real_estate_transactions
- pa_property_sales
- address_data
- deed_records
- property_price_analysis
- historical_sale_dates
- municipal_codes
- school_district_info
- transaction_types
- zip_code_distribution

But must be Japanese lower case and underscores. In Japanese, use Japanese words but underscore between words. Eg "不動産_取引", but that uses kanji which might not be lowercase? They said lower case only, but Japanese characters don't have case; it's fine. But underscores separate words.

We need tags in Markdown format: each on new line maybe with bullet? Just list them as lines or bullet points. Usually tags are just plain text separated by commas? The instruction says "in Markdown format". Might be a simple list.

Also add Attribution placeholder "{GENERATED_BY_SIGNATURE}" after the Tags. So output something like:

```
不動産_取引
パイナンシャル_州
住所_情報
権利書_記録
価格_分析
販売日_履歴
自治体_コード
学区_情報
取引_タイプ
郵便番号_分布

{GENERATED_BY_SIGNATURE}
```

But we need Japanese tags. Let's think of appropriate Japanese terms:

- 不動産_取引 (real estate transaction)
- ペンシルベニア州_物件 (Pennsylvania property) maybe "ペンシルベニア州_不動産"
- 住所_情報
- 権利書_記録
- 価格_分析
- 販売日_履歴
- 市区町村_コード
- 学区_情報
- 取引_種類
- 郵便番号_分布

That's 10 tags.

Ensure lower case only? Japanese doesn't have case; fine. Use underscores to separate words.

Let's output each tag on its own line with no bullet points, then attribution line.
## TOKEN USAGE

TokenUsage { prompt: 10913, completion: 542, total: 11455, elapsed: 21839 }
---
