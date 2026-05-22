# Dictionary
| Name | Type | Label | Description | Content Type | Min | Max | Cardinality | Enumeration | Null Count | is_ascii | sum | range | sort_order | sortiness | min_length | max_length | sum_length | avg_length | stddev_length | variance_length | cv_length | mean | sem | geometric_mean | harmonic_mean | stddev | variance | cv | n_negative | n_zero | n_positive | max_precision | sparsity | mad | lower_outer_fence | lower_inner_fence | q1 | q2_median | q3 | iqr | upper_inner_fence | upper_outer_fence | skewness | uniqueness_ratio | percentiles | Examples |
|------|------|-------|-------------|--------------|-----|-----|-------------|-------------|------------|----------|----------|----------|----------|----------|----------|----------|----------|----------|----------|----------|----------|----------|----------|----------|----------|----------|----------|----------|----------|----------|----------|----------|----------|----------|----------|----------|----------|----------|----------|----------|----------|----------|----------|----------|----------|----------|
| **Unique Key** | Integer | Record ID | A surrogate integer identifier that uniquely distinguishes each complaint record in the dataset. | unique_id | 11465364 | 48478173 | 1,000,000 |  | 0 |  | 32687965858032 | 37012809 | Unsorted | 0.0018 |  |  |  |  |  |  |  | 32687965.858 | 9013.8953 | 31351729.249 | 29944311.4641 | 9013895.3358 | 81250309125279.2656 | 27.5756 | 0 | 0 | 1000000 |  | 0 | 7477037 | -19639208.5 | 2803282.25 | 25245773 | 32853358.5 | 40207433.5 | 14961660.5 | 62649924.25 | 85092415 | -0.0169 | 1 | 5: 18453724<br>10: 20062969<br>40: 29913180<br>60: 35829112<br>90: 45355115<br>95: 46937288 | <ALL_UNIQUE> |
| **Created Date** | DateTime | Complaint Creation Timestamp | The exact timestamp when the complaint was first logged into the system. | date:%m/%d/%Y | 2010-01-01T00:00:00+00:00 | 2020-12-23T01:25:51+00:00 | 841,014 |  | 0 |  |  | 4009.05962 | Unsorted | 0.0008 |  |  |  |  |  |  |  | 2015-11-10T18:05:22.615+00:00 | 1.15502 | 16709.46856 | 16668.78207 | 1155.01606 | 1334062.09198 | 6.8957 |  |  |  |  | 0 | 965.6694 | 1997-01-08T17:56:34.500+00:00 | 2005-02-08T08:58:19.500+00:00 | 2013-03-11T00:00:04.500+00:00 | 2016-02-12T13:16:49+00:00 | 2018-07-31T10:01:14.500+00:00 | 1968.41748 | 2026-08-31T01:02:59.500+00:00 | 2034-09-30T16:04:44.500+00:00 | -0.0857 | 0.841 | 5: 2010-08-10T00:00:00+00:00<br>10: 2011-03-15T11:35:08+00:00<br>40: 2015-01-21T10:24:00+00:00<br>60: 2017-02-25T20:10:00+00:00<br>90: 2020-01-10T08:26:00+00:00<br>95: 2020-07-21T18:32:11+00:00 | Other… [997,333]<br>01/24/2013 12:00:00 AM [347]<br>01/07/2014 12:00:00 AM [315]<br>01/08/2015 12:00:00 AM [283]<br>02/16/2015 12:00:00 AM [269] |
| **Closed Date** | DateTime | Complaint Closure Timestamp | The timestamp indicating when the complaint was officially closed or resolved. | date:%m/%d/%Y | 1900-01-01T00:00:00+00:00 | 2100-01-01T00:00:00+00:00 | 688,837 |  | 28,619 |  |  | 73049 | Unsorted | 0.001 |  |  |  |  |  |  |  | 2015-11-14T10:16:16.743+00:00 | 1.33393 |  |  | 1314.70016 | 1728436.50813 | 7.8474 |  |  |  |  | 0.0286 | 954.61806 | 1997-04-12T11:33:24.500+00:00 | 2005-04-09T10:53:21.500+00:00 | 2013-04-06T10:13:18.500+00:00 | 2016-02-26T01:40:00+00:00 | 2018-08-04T09:46:36.500+00:00 | 1945.98146 | 2026-08-01T09:06:33.500+00:00 | 2034-07-29T08:26:30.500+00:00 | -0.0849 | 0.6888 | 5: 2010-08-23T11:35:00+00:00<br>10: 2011-04-01T00:00:00+00:00<br>40: 2015-02-09T00:00:00+00:00<br>60: 2017-03-08T14:06:03+00:00<br>90: 2020-01-07T00:02:00+00:00<br>95: 2020-07-20T12:00:00+00:00 | Other… [968,671]<br>(NULL)… [28,619]<br>11/15/2010 12:00:00 AM [384]<br>11/07/2012 12:00:00 AM [329]<br>12/09/2010 12:00:00 AM [267] |
| **Agency** | String | Agency Code | A short code identifying the agency responsible for handling the complaint (e.g., NYPD, HPD). | category | 3-1-1 | TLC | 28 |  | 0 | false |  |  | Unsorted | 0.1729 | 3 | 42 | 3490582 | 3.4906 | 1.8975 | 3.6005 | 0.5436 |  |  |  |  |  |  |  |  |  |  |  | 0 |  |  |  |  |  |  |  |  |  |  | 0 |  | NYPD [265,116]<br>HPD [258,033]<br>DOT [132,462]<br>DSNY [81,606]<br>DEP [75,895] |
| **Agency Name** | String | Agency Full Name | The full name of the agency that processed or investigated the complaint. | company_name | 3-1-1 | Valuation Policy | 553 |  | 0 | false |  |  | Unsorted | 0.1671 | 3 | 82 | 34840715 | 34.8407 | 10.5137 | 110.5379 | 0.3018 |  |  |  |  |  |  |  |  |  |  |  | 0 |  |  |  |  |  |  |  |  |  |  | 0.0006 |  | New York City Police Depa… [265,038]<br>Department of Housing Pre… [258,019]<br>Department of Transportat… [132,462]<br>Other… [103,974]<br>Department of Environment… [75,895] |
| **Complaint Type** | String | Complaint Category | A high‑level category describing the nature of the complaint (e.g., Noise, Heat/HOT WATER). | category | ../../WEB-INF/web.xml;x= | ZTESTINT | 287 |  | 0 | true |  |  | Unsorted | 0.0284 | 3 | 41 | 16475270 | 16.4753 | 6.8221 | 46.5406 | 0.4141 |  |  |  |  |  |  |  |  |  |  |  | 0 |  |  |  |  |  |  |  |  |  |  | 0.0003 |  | Other… [563,561]<br>Noise - Residential [89,439]<br>HEAT/HOT WATER [56,639]<br>Illegal Parking [45,032]<br>Blocked Driveway [42,356] |
| **Descriptor** | String | Complaint Sub‑category | A more specific sub‑category or description that further clarifies the complaint type. | category | 1 Missed Collection | unknown odor/taste in drinking water (QA6) | 1,392 |  | 3,001 | true |  |  | Unsorted | 0.0186 | 0 | 80 | 17426583 | 17.4266 | 10.4342 | 108.8723 | 0.5988 |  |  |  |  |  |  |  |  |  |  |  | 0.003 |  |  |  |  |  |  |  |  |  |  | 0.0014 |  | Other… [671,870]<br>Loud Music/Party [93,646]<br>ENTIRE BUILDING [36,885]<br>HEAT [35,088]<br>No Access [31,631] |
| **Location Type** | String | Location Setting | Classification of the physical setting where the incident occurred (e.g., RESIDENTIAL BUILDING, STREET). | category | 1-, 2- and 3- Family Home | Wooded Area | 162 |  | 239,131 | true |  |  | Unsorted | 0.187 | 0 | 36 | 12417750 | 12.4177 | 8.9759 | 80.5671 | 0.7228 |  |  |  |  |  |  |  |  |  |  |  | 0.2391 |  |  |  |  |  |  |  |  |  |  | 0.0002 |  | RESIDENTIAL BUILDING [255,562]<br>(NULL)… [239,131]<br>Street/Sidewalk [145,653]<br>Residential Building/Hous… [92,765]<br>Street [92,190] |
| **Incident Zip** | String | Incident ZIP Code | The five‑digit ZIP code corresponding to the location of the incident. | zip_code | * | XXXXX | 535 |  | 54,978 | true |  |  | Unsorted | 0.0085 | 0 | 10 | 4347871 | 4.3479 | 1.14 | 1.2996 | 0.2622 |  |  |  |  |  |  |  |  |  |  |  | 0.055 |  |  |  |  |  |  |  |  |  |  | 0.0005 |  | Other… [815,988]<br>(NULL)… [54,978]<br>11226 [17,114]<br>10467 [14,495]<br>11207 [12,872] |
| **Incident Address** | String | Full Incident Address | Full street address where the incident took place, including building number and street name. | street_address | * * | west 155 street and edgecombe avenue | 341,996 |  | 174,700 | true |  |  | Unsorted | -0.0005 | 0 | 55 | 14591947 | 14.5919 | 7.332 | 53.7589 | 0.5025 |  |  |  |  |  |  |  |  |  |  |  | 0.1747 |  |  |  |  |  |  |  |  |  |  | 0.342 |  | Other… [819,046]<br>(NULL)… [174,700]<br>655 EAST  230 STREET [1,538]<br>78-15 PARSONS BOULEVARD [694]<br>672 EAST  231 STREET [663] |
| **Street Name** | String | Street Name (Primary) | The name of the primary street involved in the incident; combine with Incident Address, City, Borough, and ZIP to form the full address. | street_name | * | wyckoff avenue | 14,837 |  | 174,720 | true |  |  | Unsorted | 0.0001 | 0 | 55 | 10888475 | 10.8885 | 5.7968 | 33.6032 | 0.5324 |  |  |  |  |  |  |  |  |  |  |  | 0.1747 |  |  |  |  |  |  |  |  |  |  | 0.0148 |  | Other… [784,684]<br>(NULL)… [174,720]<br>BROADWAY [9,702]<br>GRAND CONCOURSE [5,851]<br>OCEAN AVENUE [3,946] |
| **Cross Street 1** | String | Cross Street 1 | The first cross street at an intersection near the incident location. | street_name | 1 AVE | mermaid | 16,238 |  | 320,401 | true |  |  | Unsorted | 0.0009 | 0 | 32 | 8355458 | 8.3555 | 6.6045 | 43.6193 | 0.7904 |  |  |  |  |  |  |  |  |  |  |  | 0.3204 |  |  |  |  |  |  |  |  |  |  | 0.0162 |  | Other… [619,743]<br>(NULL)… [320,401]<br>BEND [12,562]<br>BROADWAY [8,548]<br>3 AVENUE [6,154] |
| **Cross Street 2** | String | Cross Street 2 | A secondary or alternative cross street related to the incident location. | street_name | 1 AVE | surf | 16,486 |  | 323,644 | true |  |  | Unsorted | 0.0016 | 0 | 35 | 8363431 | 8.3634 | 6.645 | 44.1554 | 0.7945 |  |  |  |  |  |  |  |  |  |  |  | 0.3236 |  |  |  |  |  |  |  |  |  |  | 0.0165 |  | Other… [623,363]<br>(NULL)… [323,644]<br>BEND [12,390]<br>BROADWAY [8,833]<br>DEAD END [5,626] |
| **Intersection Street 1** | String | Intersection Street 1 | The first street forming an intersection near the incident site. | street_name | 1 AVE | flatlands AVE | 11,237 |  | 767,422 | true |  |  | Unsorted | -0.0009 | 0 | 35 | 2949273 | 2.9493 | 5.6793 | 32.2544 | 1.9257 |  |  |  |  |  |  |  |  |  |  |  | 0.7674 |  |  |  |  |  |  |  |  |  |  | 0.0112 |  | (NULL)… [767,422]<br>Other… [214,544]<br>BROADWAY [3,761]<br>CARPENTER AVENUE [2,918]<br>BEND [2,009] |
| **Intersection Street 2** | String | Intersection Street 2 | The second street forming that same intersection. | street_name | 1 AVE | glenwood RD | 11,674 |  | 767,709 | true |  |  | Unsorted | 0.003 | 0 | 33 | 2917798 | 2.9178 | 5.6363 | 31.768 | 1.9317 |  |  |  |  |  |  |  |  |  |  |  | 0.7677 |  |  |  |  |  |  |  |  |  |  | 0.0117 |  | (NULL)… [767,709]<br>Other… [215,667]<br>BROADWAY [3,462]<br>BEND [1,942]<br>2 AVENUE [1,690] |
| **Address Type** | String | Address Format | A descriptor of the address format (e.g., ADDRESS, INTERSECTION, BLOCKFACE). | category | ADDRESS | PLACENAME | 6 |  | 125,802 | true |  |  | Unsorted | 0.6845 | 0 | 12 | 6832263 | 6.8323 | 3.0923 | 9.5623 | 0.4526 |  |  |  |  |  |  |  |  |  |  |  | 0.1258 |  |  |  |  |  |  |  |  |  |  | 0 |  | ADDRESS [710,380]<br>INTERSECTION [133,361]<br>(NULL)… [125,802]<br>BLOCKFACE [22,620]<br>LATLONG [7,421] |
| **City** | String | City | The city in which the incident occurred. | city | * | YORKTOWN HEIGHTS | 382 |  | 61,963 | true |  |  | Unsorted | 0.1811 | 0 | 22 | 7721241 | 7.7212 | 3.2635 | 10.6505 | 0.4227 |  |  |  |  |  |  |  |  |  |  |  | 0.062 |  |  |  |  |  |  |  |  |  |  | 0.0004 |  | BROOKLYN [296,254]<br>NEW YORK [189,069]<br>BRONX [181,168]<br>Other… [163,936]<br>(NULL)… [61,963] |
| **Landmark** | String | Nearby Landmark | A notable nearby landmark or point of reference for the incident location. | free_text | 1 AVENUE | ZULETTE AVENUE | 5,915 |  | 912,779 | true |  |  | Unsorted | 0.0009 | 0 | 32 | 1165773 | 1.1658 | 3.8978 | 15.1925 | 3.3435 |  |  |  |  |  |  |  |  |  |  |  | 0.9128 |  |  |  |  |  |  |  |  |  |  | 0.0059 |  | (NULL)… [912,779]<br>Other… [80,165]<br>EAST  230 STREET [1,545]<br>EAST  231 STREET [1,291]<br>BROADWAY [1,148] |
| **Facility Type** | String | Facility Category | The type of facility involved in the complaint (e.g., DSNY Garage, School District). | category | DSNY Garage | School District | 6 |  | 145,478 | true |  |  | Unsorted | 0.5941 | 0 | 15 | 3790876 | 3.7909 | 2.7562 | 7.5969 | 0.7271 |  |  |  |  |  |  |  |  |  |  |  | 0.1455 |  |  |  |  |  |  |  |  |  |  | 0 |  | N/A [628,279]<br>Precinct [193,259]<br>(NULL)… [145,478]<br>DSNY Garage [32,310]<br>School [617] |
| **Status** | String | Complaint Status | Current processing status of the complaint. | category | Assigned | Unspecified | 10 | Assigned<br>Closed<br>Closed - Testing<br>Email Sent<br>In Progress<br>Open<br>Pending<br>Started<br>Unassigned<br>Unspecified | 0 | true |  |  | Unsorted | 0.9079 | 4 | 16 | 6048943 | 6.0489 | 0.5411 | 0.2928 | 0.0894 |  |  |  |  |  |  |  |  |  |  |  | 0 |  |  |  |  |  |  |  |  |  |  | 0 |  | Closed [952,522]<br>Pending [20,119]<br>Open [12,340]<br>In Progress [7,841]<br>Assigned [6,651] |
| **Due Date** | DateTime | Resolution Deadline Timestamp | The deadline by which a resolution or response is expected. | datetime:%m/%d/%Y %I:%M:%S %p | 1900-01-02T00:00:00+00:00 | 2021-06-17T16:34:13+00:00 | 345,077 |  | 647,794 |  |  | 44361.69043 | Unsorted | 0.0011 |  |  |  |  |  |  |  | 2015-05-30T02:54:49.998+00:00 | 1.74433 |  |  | 1035.204 | 1071647.31812 | 6.2418 |  |  |  |  | 0.6478 | 800.07713 | 1999-09-09T02:53:37+00:00 | 2006-06-14T19:09:32.500+00:00 | 2013-03-20T11:25:28+00:00 | 2015-10-03T01:27:48+00:00 | 2017-09-22T14:16:05+00:00 | 1647.11848 | 2024-06-28T06:32:00.500+00:00 | 2031-04-03T22:47:56+00:00 | -0.1251 | 0.3451 | 5: 2010-08-25T10:54:44+00:00<br>10: 2011-04-14T19:02:13+00:00<br>40: 2014-11-06T10:17:31+00:00<br>60: 2016-07-31T14:14:53+00:00<br>90: 2018-10-25T07:47:20+00:00<br>95: 2019-03-24T08:03:29+00:00 | (NULL)… [647,794]<br>Other… [350,746]<br>04/08/2015 10:00:58 AM [214]<br>05/02/2014 03:32:17 PM [183]<br>03/30/2018 10:10:39 AM [172] |
| **Resolution Description** | String | Resolution Narrative | A detailed narrative of the actions taken, findings, or outcomes associated with the complaint. | free_text | A DOB violation was issued for failing to comply with an existing Stop Work Order. | Your request was submitted to the Department of Homeless Services. The City?s outreach team will assess the homeless individual and offer appropriate assistance within 2 hours. If you asked to know the outcome of your request, you will get a call within 2 hours. No further status will be available through the NYC 311 App, 311, or 311 Online. | 1,216 |  | 20,480 | false |  |  | Unsorted | 0.0319 | 0 | 934 | 153148305 | 153.1483 | 82.149 | 6748.4538 | 0.5364 |  |  |  |  |  |  |  |  |  |  |  | 0.0205 |  |  |  |  |  |  |  |  |  |  | 0.0012 |  | Other… [511,739]<br>The Police Department res… [91,408]<br>The Department of Housing… [72,962]<br>The Police Department res… [63,868]<br>Service Request status fo… [52,155] |
| **Resolution Action Updated Date** | DateTime | Last Update to Resolution Timestamp | Timestamp indicating when the resolution action was last updated. | date:%m/%d/%Y | 2009-12-31T01:35:00+00:00 | 2020-12-23T06:56:14+00:00 | 690,314 |  | 15,072 |  |  | 4010.22308 | Unsorted | 0.001 |  |  |  |  |  |  |  | 2015-11-19T19:44:34.889+00:00 | 1.16204 | 16718.67594 | 16678.12298 | 1153.24922 | 1329983.75668 | 6.8814 |  |  |  |  | 0.0151 | 966.48803 | 1997-01-12T11:30:24+00:00 | 2005-02-14T12:31:15.750+00:00 | 2013-03-19T13:32:07.500+00:00 | 2016-02-22T22:38:30+00:00 | 2018-08-10T14:12:42+00:00 | 1970.02818 | 2026-09-12T15:13:33.750+00:00 | 2034-10-15T16:14:25.500+00:00 | -0.0867 | 0.6903 | 5: 2010-08-22T00:00:00+00:00<br>10: 2011-03-25T11:49:23+00:00<br>40: 2015-02-01T00:00:00+00:00<br>60: 2017-03-10T02:24:33+00:00<br>90: 2020-01-12T09:08:00+00:00<br>95: 2020-07-22T01:06:53+00:00 | Other… [982,148]<br>(NULL)… [15,072]<br>11/15/2010 12:00:00 AM [385]<br>11/07/2012 12:00:00 AM [336]<br>12/09/2010 12:00:00 AM [273] |
| **Community Board** | String | Community Board Identifier | Identifier for the community board area relevant to the incident. | category | 0 Unspecified | Unspecified STATEN ISLAND | 77 |  | 0 | true |  |  | Unsorted | 0.0193 | 8 | 25 | 11142863 | 11.1429 | 2.971 | 8.8269 | 0.2666 |  |  |  |  |  |  |  |  |  |  |  | 0 |  |  |  |  |  |  |  |  |  |  | 0.0001 |  | Other… [751,635]<br>0 Unspecified [49,878]<br>12 MANHATTAN [29,845]<br>12 QUEENS [23,570]<br>01 BROOKLYN [21,714] |
| **BBL** | String | Borough‑Block‑Lot (BBL) | Numeric identifier used in NYC property records. | unknown | 0000000000 | 5080470043 | 268,383 |  | 243,046 | true |  |  | Unsorted | -0.0009 | 0 | 10 | 448540 | 0.4485 | 4.3011 | 18.4993 | 9.5891 |  |  |  |  |  |  |  |  |  |  |  | 0.243 |  |  |  |  |  |  |  |  |  |  | 0.2684 |  | Other… [750,668]<br>(NULL)… [243,046]<br>2048330028 [1,566]<br>4068290001 [696]<br>4015110001 [664] |
| **Borough** | String | Borough | The borough of New York City where the incident occurred. | category | BRONX | Unspecified | 6 | BRONX<br>BROOKLYN<br>MANHATTAN<br>QUEENS<br>STATEN ISLAND<br>Unspecified | 0 | true |  |  | Unsorted | 0.2155 | 5 | 13 | 7595025 | 7.595 | 2.0632 | 4.2568 | 0.2717 |  |  |  |  |  |  |  |  |  |  |  | 0 |  |  |  |  |  |  |  |  |  |  | 0 |  | BROOKLYN [296,081]<br>QUEENS [228,818]<br>MANHATTAN [195,488]<br>BRONX [180,142]<br>Unspecified [49,878] |
| **X Coordinate (State Plane)** | Integer | X Coordinate (State Plane) | Easting coordinate in the State Plane coordinate system for the incident location. | unknown | 913281 | 1067220 | 102,556 |  | 85,327 |  | 919555108413 | 153939 | Unsorted | -0.0004 |  |  |  |  |  |  |  | 1005337.5451 | 23.5391 | 1005083.7023 | 1004827.9356 | 22512.4528 | 506810531.5324 | 2.2393 | 0 | 0 | 914673 |  | 0.0853 | 12292 | 919661 | 956616.5 | 993572 | 1004546 | 1018209 | 24637 | 1055164.5 | 1092120 | 0.1091 | 0.1026 | 5: 964313<br>10: 984035<br>40: 999859<br>60: 1009147<br>90: 1034015<br>95: 1043903 | Other… [908,535]<br>(NULL)… [85,327]<br>1022911 [1,568]<br>1037000 [701]<br>1023174 [675] |
| **Y Coordinate (State Plane)** | Integer | Y Coordinate (State Plane) | Northing coordinate in the State Plane coordinate system for the incident location. | unknown | 121152 | 271876 | 116,092 |  | 85,327 |  | 188099299101 | 150724 | Unsorted | 0 |  |  |  |  |  |  |  | 205646.4978 | 33.1699 | 203166.0871 | 200659.7012 | 31723.1985 | 1006361322.6747 | 15.4261 | 0 | 0 | 914673 |  | 0.0853 | 24236 | 24257 | 103334 | 182411 | 202514 | 235129 | 52718 | 314206 | 393283 | 0.2373 | 0.1161 | 5: 156639<br>10: 164744<br>40: 193463<br>60: 212470<br>90: 250365<br>95: 256054 | Other… [908,538]<br>(NULL)… [85,327]<br>264242 [1,566]<br>202363 [706]<br>211606 [665] |
| **Open Data Channel Type** | String | Submission Channel | The medium through which the complaint was submitted to the open‑data platform. | category | MOBILE | UNKNOWN | 5 | MOBILE<br>ONLINE<br>OTHER<br>PHONE<br>UNKNOWN | 0 | true |  |  | Unsorted | 0.3379 | 5 | 7 | 5718030 | 5.718 | 0.8144 | 0.6633 | 0.1424 |  |  |  |  |  |  |  |  |  |  |  | 0 |  |  |  |  |  |  |  |  |  |  | 0 |  | PHONE [497,606]<br>UNKNOWN [230,402]<br>ONLINE [177,334]<br>MOBILE [79,892]<br>OTHER [14,766] |
| **Park Facility Name** | String | Park Facility Name | Name of the park or recreational facility involved in the incident. | free_text | "Uncle" Vito F. Maranzano Glendale Playground | Zimmerman Playground | 1,889 |  | 0 | true |  |  | Unsorted | 0.9863 | 3 | 82 | 11072428 | 11.0724 | 1.2391 | 1.5353 | 0.1119 |  |  |  |  |  |  |  |  |  |  |  | 0 |  |  |  |  |  |  |  |  |  |  | 0.0019 |  | Unspecified [993,141]<br>Other… [5,964]<br>Central Park [261]<br>Riverside Park [136]<br>Prospect Park [129] |
| **Park Borough** | String | Park Borough | The borough where the referenced park facility is located. | category | BRONX | Unspecified | 6 | BRONX<br>BROOKLYN<br>MANHATTAN<br>QUEENS<br>STATEN ISLAND<br>Unspecified | 0 | true |  |  | Unsorted | 0.2155 | 5 | 13 | 7595025 | 7.595 | 2.0632 | 4.2568 | 0.2717 |  |  |  |  |  |  |  |  |  |  |  | 0 |  |  |  |  |  |  |  |  |  |  | 0 |  | BROOKLYN [296,081]<br>QUEENS [228,818]<br>MANHATTAN [195,488]<br>BRONX [180,142]<br>Unspecified [49,878] |
| **Vehicle Type** | String | Vehicle Category | Classification of vehicle related to the incident. | category | Ambulette / Paratransit | Green Taxi | 5 |  | 999,652 | true |  |  | Unsorted | 0.8329 | 0 | 23 | 4066 | 0.0041 | 0.2293 | 0.0526 | 56.4051 |  |  |  |  |  |  |  |  |  |  |  | 0.9997 |  |  |  |  |  |  |  |  |  |  | 0 |  | (NULL)… [999,652]<br>Car Service [317]<br>Ambulette / Paratransit [19]<br>Commuter Van [11]<br>Green Taxi [1] |
| **Taxi Company Borough** | String | Taxi Company Borough | The borough in which the taxi company operates or is registered. | category | BRONX | Staten Island | 11 |  | 999,156 | true |  |  | Unsorted | 0.1839 | 0 | 13 | 6313 | 0.0063 | 0.2259 | 0.051 | 35.783 |  |  |  |  |  |  |  |  |  |  |  | 0.9992 |  |  |  |  |  |  |  |  |  |  | 0 |  | (NULL)… [999,156]<br>BROOKLYN [207]<br>QUEENS [194]<br>MANHATTAN [171]<br>BRONX [127] |
| **Taxi Pick Up Location** | String | Taxi Pick‑Up Location | Description of the location where a taxi was picked up. | free_text | 1 5 AVENUE MANHATTAN | YORK AVENUE AND EAST 70 STREET | 1,903 |  | 992,129 | true |  |  | Unsorted | 0.2854 | 0 | 60 | 135661 | 0.1357 | 2.1518 | 4.6304 | 15.8618 |  |  |  |  |  |  |  |  |  |  |  | 0.9921 |  |  |  |  |  |  |  |  |  |  | 0.0019 |  | (NULL)… [992,129]<br>Other [4,091]<br>Other… [2,006]<br>JFK Airport [562]<br>Intersection [486] |
| **Bridge Highway Name** | String | Bridge/Highway Name | Name or designation of the bridge or highway involved in the incident. | category | 145th St. Br - Lenox Ave | Willis Ave Br - 125th St/1st Ave | 68 |  | 997,711 | true |  |  | Unsorted | 0.0227 | 0 | 42 | 36974 | 0.037 | 0.8221 | 0.6759 | 22.2353 |  |  |  |  |  |  |  |  |  |  |  | 0.9977 |  |  |  |  |  |  |  |  |  |  | 0.0001 |  | (NULL)… [997,711]<br>Other… [779]<br>Belt Pkwy [276]<br>BQE/Gowanus Expwy [254]<br>Grand Central Pkwy [186] |
| **Bridge Highway Direction** | String | Bridge/Highway Direction | Direction of travel on the bridge or highway. | category | Bronx Bound | Westbound/To Goethals Br | 50 |  | 997,691 | true |  |  | Unsorted | 0.0338 | 0 | 33 | 44089 | 0.0441 | 0.9533 | 0.9089 | 21.6233 |  |  |  |  |  |  |  |  |  |  |  | 0.9977 |  |  |  |  |  |  |  |  |  |  | 0.0001 |  | (NULL)… [997,691]<br>Other… [987]<br>East/Long Island Bound [210]<br>North/Bronx Bound [208]<br>East/Queens Bound [197] |
| **Road Ramp** | String | Road Ramp Type | Type of road ramp associated with the incident. | category | N/A | Roadway | 4 |  | 997,693 | true |  |  | Unsorted | 0.6245 | 0 | 7 | 14400 | 0.0144 | 0.3069 | 0.0942 | 21.3118 |  |  |  |  |  |  |  |  |  |  |  | 0.9977 |  |  |  |  |  |  |  |  |  |  | 0 |  | (NULL)… [997,693]<br>Roadway [1,731]<br>Ramp [555]<br>N/A [21] |
| **Bridge Highway Segment** | String | Bridge/Highway Segment | Specific segment or exit identifier for a bridge or highway. | category | 1-1-1265963747 | Wythe Ave/Kent Ave (Exit 31) | 937 |  | 997,556 | true |  |  | Unsorted | -0.007 | 0 | 100 | 110781 | 0.1108 | 2.5166 | 6.3334 | 22.7171 |  |  |  |  |  |  |  |  |  |  |  | 0.9976 |  |  |  |  |  |  |  |  |  |  | 0.0009 |  | (NULL)… [997,556]<br>Other… [2,144]<br>Ramp [92]<br>Roadway [54]<br>Clove Rd/Richmond Rd (Exi… [23] |
| **Latitude** | Float | Latitude (decimal degrees) | Decimal‑degree latitude coordinate of the incident location. | latitude | 40.1123853 | 40.9128688 | 353,694 |  | 254,695 |  | 30355391.7604 | 0.8005 | Unsorted | -0.001 |  |  |  |  |  |  |  | 40.7288 | 0.0001 | 40.7287 | 40.7286 | 0.0893 | 0.008 | 0.2193 | 0 | 0 | 745305 | 15 | 0.2547 | 0.0632 | 40.2615 | 40.4646 | 40.6677 | 40.7222 | 40.8031 | 0.1354 | 41.0062 | 41.2094 | 0.1957 | 0.3537 | 5: 40.5955<br>10: 40.6175<br>40: 40.6986<br>60: 40.748<br>90: 40.8521<br>95: 40.8684 | Other… [739,329]<br>(NULL)… [254,695]<br>40.89187241649303 [1,538]<br>40.1123853 [1,153]<br>40.89238451539139 [663] |
| **Longitude** | Float | Longitude (decimal degrees) | Decimal‑degree longitude coordinate of the incident location. | longitude | -77.5195844 | -73.7005968 | 353,996 |  | 254,695 |  | -55100392.9499 | 3.819 | Unsorted | -0.0008 |  |  |  |  |  |  |  | -73.93 | 0.0002 |  |  | 0.1635 | 0.0267 | -0.2212 | 745305 | 0 | 0 | 14 | 0.2547 | 0.0468 | -74.2533 | -74.1119 | -73.9705 | -73.9279 | -73.8763 | 0.0943 | -73.7349 | -73.5935 | 0.0964 | 0.354 | 5: -74.0787<br>10: -74.0022<br>40: -73.9454<br>60: -73.9106<br>90: -73.8191<br>95: -73.7839 | Other… [739,329]<br>(NULL)… [254,695]<br>-73.86016845296459 [1,538]<br>-77.5195844 [1,153]<br>-73.8592161325675 [663] |
| **Location** | String | Coordinate String | Textual representation of the latitude and longitude pair for the incident. | free_text | (40.1123853, -77.5195844) | (40.9128688, -73.9024731) | 375,772 |  | 254,695 | true |  |  | Unsorted | -0.001 | 0 | 40 | 19709659 | 19.7097 | 12.1918 | 148.6401 | 0.6186 |  |  |  |  |  |  |  |  |  |  |  | 0.2547 |  |  |  |  |  |  |  |  |  |  | 0.3758 |  | Other… [739,329]<br>(NULL)… [254,695]<br>(40.89187241649303, -73.8… [1,538]<br>(40.1123853, -77.5195844) [1,153]<br>(40.89238451539139, -73.8… [663] |

*Attribution: Generated by qsv v20.1.0 describegpt
Command line: target/debug/qsv describegpt --all --two-pass /tmp/NYC_311_SR_2010-2020-sample-1M.csv -o /tmp/nyc311-describegpt-everything.md --addl-cols --addl-cols-list everything --infer-content-type --fresh
Prompt file: Default v7.1.0
Model: openai/gpt-oss-20b
LLM API URL: http://localhost:1234/v1
Language: 
Timestamp: 2026-05-22T02:39:46.173166+00:00

WARNING: Label and Description generated by an LLM and may contain inaccuracies. Verify before using!
*

## REASONING

FIRST PASS REASONING:
We need to generate JSON with each field name mapping to label, description, content_type. Use summary stats and freq distribution.

Need to interpret each field semantics: e.g., Unique Key is unique identifier; but we can't use "unique_id" token because it's reserved? Actually they said do not invent tokens. "unique_id" is reserved but set deterministically by qsv when cardinality equals row count. We shouldn't use it ourselves; instead pick most specific other token. For Unique Key, integer but likely a surrogate key. Could be "uuid"? But values are integers 32687965858032 etc. It's numeric ID. So content_type maybe "unknown" or "unique_id"? They say not to use unique_id. Use the most specific token that fits: it's an integer ID; we can label as "unknown". But maybe "uuid" but these are not UUIDs. So unknown.

Created Date, type DateTime. Format from freq distribution sample values like "01/24/2013 12:00:00 AM". That is mm/dd/yyyy hh:mm:ss AM/PM? Actually format: MM/DD/YYYY HH:MM:SS AM/PM? It uses '12:00:00 AM' so time part includes seconds. So strftime format: "%m/%d/%Y %I:%M:%S %p" (month/day/year hour 12-hour, minute second am/pm). Good.

Closed Date similar format.

Agency: string; values are codes like "NYPD". Could be an agency code token? None matches. So unknown.

Agency Name: full name of agency; token could be organization_name? Not in list. So unknown.

Complaint Type: category of complaint; maybe 'category' token.

Descriptor: description of issue, more detailed. Might be free_text but also some categories like "Loud Music/Party". Could use category? But descriptor may have many values; but still categorical. However token 'category' fits. If not, unknown. We'll choose category.

Location Type: type of location e.g., RESIDENTIAL BUILDING etc. So category.

Incident Zip: zip code numeric string maybe "zip_code".

Incident Address: street address text; could be street_address? The values include building number and street name; yes use street_address.

Street Name: street_name.

Cross Street 1, Cross Street 2: also street_name.

Intersection Street 1, Intersection Street 2: street_name.

Address Type: classification of address type e.g., ADDRESS, INTERSECTION. Might be category or free_text? Use category.

City: city; token city.

Landmark: name of landmark; maybe free_text but could be category; use category? But there are many unique values; but still category. We'll use category.

Facility Type: type of facility like DSNY Garage etc. Category.

Status: status of complaint. Category.

Due Date: datetime format similar to Created/Closed. Format mm/dd/yyyy hh:mm:ss AM/PM. So date/time.

Resolution Description: free_text (long description). Use free_text.

Resolution Action Updated Date: datetime same as others. Format mm/dd/yyyy hh:mm:ss AM/PM? Wait freq shows "11/15/2010 12:00:00 AM". Yes same format.

Community Board: category? Many values like '0 Unspecified', numbers. So category.

BBL: borough lot number; numeric string but likely an identifier; unknown.

Borough: city borough names; token state_abbr? Not exactly. Borough is a region, maybe "state_abbr" not right. Use category or maybe "city"? But distinct from City field. We'll use category.

X Coordinate (State Plane): integer coordinate; numeric. Could be latitude/longitude but it's state plane coordinates; unknown.

Y Coordinate (State Plane): unknown.

Open Data Channel Type: type of channel e.g., PHONE, ONLINE. Category.

Park Facility Name: name of park facility; free_text? But many values. Use category maybe but could be free_text. Since many unique names, use category or free_text. I'd pick category since it's a named entity.

Park Borough: borough of park; category.

Vehicle Type: type of vehicle; category.

Taxi Company Borough: borough of taxi company; category.

Taxi Pick Up Location: location description; category? Many values including "Other" etc. Could be category.

Bridge Highway Name: name of bridge or highway; category.

Bridge Highway Direction: direction string; category.

Road Ramp: type; category.

Bridge Highway Segment: description; category.

Latitude: float coordinate 40.xx; content_type latitude.

Longitude: longitude.

Location: composite lat/long string; maybe free_text? But could be pair. Use free_text or maybe a location string. We'll use free_text (since format not simple). But we might choose "free_text".

Now produce JSON mapping field names to objects with label, description, content_type.

We need to craft descriptions for each field referencing context.

Let's go through each of 41 fields and produce:

1 Unique Key: label same; description: unique integer identifier for each complaint record. Content_type unknown.

2 Created Date: label "Created Date". Description: timestamp when the complaint was created in the system. Format mm/dd/yyyy hh:mm:ss AM/PM. content_type datetime:%m/%d/%Y %I:%M:%S %p

3 Closed Date: similar.

4 Agency: short code for agency handling complaint. Content_type unknown.

5 Agency Name: full name of agency. unknown.

6 Complaint Type: category indicating general type of complaint (e.g., Noise, Heat). content_type category.

7 Descriptor: more detailed description or subcategory; e.g., Loud Music/Party. Category.

8 Location Type: classification of location where incident occurred. Category.

9 Incident Zip: ZIP code of incident location. zip_code.

10 Incident Address: full street address of incident. street_address.

11 Street Name: name of street where incident occurred. street_name.

12 Cross Street 1: primary cross street at intersection. street_name.

13 Cross Street 2: secondary cross street. street_name.

14 Intersection Street 1: first street in intersection. street_name.

15 Intersection Street 2: second street. street_name.

16 Address Type: type of address record (ADDRESS, INTERSECTION). category.

17 City: city where incident occurred. city.

18 Landmark: nearby landmark name. category.

19 Facility Type: type of facility involved (e.g., DSNY Garage). category.

20 Status: current status of complaint. category.

21 Due Date: deadline for resolution. datetime format same as Created/Closed. content_type datetime:%m/%d/%Y %I:%M:%S %p

22 Resolution Description: detailed narrative of action taken or outcome. free_text.

23 Resolution Action Updated Date: timestamp when resolution action was last updated. datetime:%m/%d/%Y %I:%M:%S %p

24 Community Board: community board number/identifier for location. category.

25 BBL: Borough‑Block‑Lot identifier, a unique numeric code. unknown.

26 Borough: borough where incident occurred. category.

27 X Coordinate (State Plane): easting coordinate in state plane system. unknown.

28 Y Coordinate (State Plane): northing coordinate. unknown.

29 Open Data Channel Type: medium through which complaint was submitted. category.

30 Park Facility Name: name of park facility involved. category or free_text; choose category.

31 Park Borough: borough where the park is located. category.

32 Vehicle Type: type of vehicle involved in incident. category.

33 Taxi Company Borough: borough of taxi company. category.

34 Taxi Pick Up Location: location description for pickup. category.

35 Bridge Highway Name: name of bridge or highway. category.

36 Bridge Highway Direction: direction of travel on the bridge/highway. category.

37 Road Ramp: type of road ramp (Roadway, Ramp). category.

38 Bridge Highway Segment: specific segment identifier. category.

39 Latitude: latitude coordinate in decimal degrees. latitude.

40 Longitude: longitude coordinate. longitude.

41 Location: textual representation of latitude/longitude pair. free_text.

Now produce JSON accordingly.

Make sure content_type tokens are from allowed list; for datetime we add suffix.

Also for unknown fields, use "unknown". For unique key maybe unknown too. For numeric coordinates unknown.

Ok generate JSON.



REFINE PASS REASONING:
We need to refine labels, descriptions and content_type for all fields.

We have 41 fields. We must consider composite concepts:

- Mailing address: Incident Address, Street Name, Cross Street 1/2, Intersection Street 1/2, Address Type maybe part of location type? Also Incident Zip, City, Borough, maybe.

But Incident Address likely includes building number and street name; but we have separate fields for Street Name etc. But dataset seems to store address components separately: Incident Address full string, Street Name primary, Cross Streets (maybe intersection or cross streets). Intersection Street 1/2 are the two intersecting streets? Actually "Intersection Street 1" and "Intersection Street 2" might be the two street names at intersection; but we also have Cross Street 1 & 2 maybe alternative cross streets? Not sure. But we can define components: Incident Address (full address), Street Name, Cross Street(s) etc.

Also location type: classification of physical setting.

We have coordinates: X and Y State Plane, Latitude, Longitude. So coordinate pair is a composite concept: "Geographic Coordinate" maybe with separate fields for latitude/longitude but also state plane coordinates. We can note that Latitude and Longitude are components of geographic coordinate; X/Y State Plane are also coordinate representation.

Also BBL (borough-block-lot) numeric identifier, Borough, Community Board: group into property or administrative region concept: "Administrative Region" maybe.

Also Agency + Agency Name: composite: agency code and full name. So we can keep both but mention relationship.

Also Complaint Type + Descriptor: combine as complaint category. Maybe "Complaint Category" is combination of high-level type and sub-type.

Open Data Channel Type separate.

Park Facility Name + Park Borough: maybe part of park info.

Vehicle Type, Taxi Company Borough, Taxi Pick Up Location: group taxi incident info.

Bridge Highway Name/Direction/Ramp/Segment: group as bridge/highway info.

Location field is textual representation of lat/lon pair; maybe redundant with Latitude/Longitude but still present. We can note that it's a textual representation of coordinates.

Also Due Date, Created Date, Closed Date, Resolution Action Updated Date: date/time fields.

Status separate.

Resolution Description free text.

Now adjust content types:

Let's list each field and propose new label, description, content_type.

We need to remove content_type for unique_id fields. Unique Key is unique id; so omit content_type. All other fields keep or adjust.

Also "Agency" likely category code: use category token.

"Agency Name": company_name maybe? But it's organization name: could be "company_name" or "organization". Allowed tokens include company_name, industry, job_title etc. Use company_name.

But we can also consider "agency" as "category" (since it's a short code). So keep category.

Incident Zip: zip_code token.

Incident Address: street_address.

Street Name: street_name.

Cross Street 1/2: street_name? They are cross streets; maybe still street_name. Could use street_name too.

Intersection Street 1/2: street_name.

Address Type: category (since values like ADDRESS, INTERSECTION, etc.)

City: city token.

Landmark: free_text or category? Many unique names; but could be "free_text". Keep free_text.

Facility Type: category.

Status: category.

Due Date: datetime with format. Use datetime:%m/%d/%Y %I:%M:%S %p maybe same as Created/Closed. But due date appears to have only date, not time? The examples show date/time? Example "04/08/2015 10:00:58 AM" yes includes time. So keep datetime with that format.

Resolution Description: free_text.

Resolution Action Updated Date: datetime:%m/%d/%Y %I:%M:%S %p or maybe only date? Example shows "11/15/2010 12:00:00 AM". Use same as others.

Community Board: category.

BBL: unknown (numeric). We can use unknown. But maybe numeric string; but not standard token. Keep unknown.

Borough: state_abbr? Actually borough names are full, like BROOKLYN. Could be "state" or "city"? But borough is a specific category. Use category.

X Coordinate (State Plane): integer coordinate in meters. Could be "unknown". Keep unknown.

Y Coordinate (State Plane): same; unknown.

Open Data Channel Type: category.

Park Facility Name: company_name? Actually it's facility name; maybe free_text or company_name. Use free_text.

Park Borough: same as borough, but category.

Vehicle Type: category.

Taxi Company Borough: category.

Taxi Pick Up Location: free_text.

Bridge Highway Name: category.

Bridge Highway Direction: category.

Road Ramp: category.

Bridge Highway Segment: category.

Latitude: latitude token.

Longitude: longitude token.

Location: free_text (textual representation of lat/lon pair). Keep free_text.

Now refine descriptions referencing cross-field relationships. Provide label maybe more explicit like "Mailing Address – Street Name" etc. But we can keep original labels but refined to show role. Eg for Street Name: "Street name component of the incident address; combine with Incident Address, City, Borough, and ZIP to form full address." That is okay.

Similarly for Cross Streets: mention as cross street components at intersection near incident location.

Intersection Streets: maybe "Intersection street components forming an intersection near the incident site".

Latitude/Longitude: part of geographic coordinate pair. X/Y State Plane also coordinate representation.

BBL + Borough + Community Board can be described together.

Agency + Agency Name: separate fields but mention relationship.

Now we need to produce JSON with each field name as key and sub-object.

Let's create mapping:

"Unique Key": omit content_type; keep label same? Could refine to "Record Identifier". Description unchanged maybe. Let's adjust.

We must output for every field: label, description, content_type (except unique id). So include all 41 keys.

Now produce final JSON.

Let's craft each entry:

1. Unique Key:
- label: "Record ID"
- description: "A surrogate integer identifier that uniquely distinguishes each complaint record in the dataset."
- no content_type key

2. Created Date:
- label: "Complaint Creation Timestamp"
- description: "The exact timestamp when the complaint was first logged into the system."
- content_type: datetime:%m/%d/%Y %I:%M:%S %p

3. Closed Date:
- label: "Complaint Closure Timestamp"
- description: "The timestamp indicating when the complaint was officially closed or resolved."
- content_type: datetime:%m/%d/%Y %I:%M:%S %p

4. Agency:
- label: "Agency Code"
- description: "A short code identifying the agency responsible for handling the complaint (e.g., NYPD, HPD)."
- content_type: category

5. Agency Name:
- label: "Agency Full Name"
- description: "The full name of the agency that processed or investigated the complaint."
- content_type: company_name

6. Complaint Type:
- label: "Complaint Category"
- description: "A high‑level category describing the nature of the complaint (e.g., Noise, Heat/HOT WATER)."
- content_type: category

7. Descriptor:
- label: "Complaint Sub‑category"
- description: "A more specific sub‑category or description that further clarifies the complaint type."
- content_type: category

8. Location Type:
- label: "Location Setting"
- description: "Classification of the physical setting where the incident occurred (e.g., RESIDENTIAL BUILDING, STREET)."
- content_type: category

9. Incident Zip:
- label: "Incident ZIP Code"
- description: "The five‑digit ZIP code corresponding to the location of the incident."
- content_type: zip_code

10. Incident Address:
- label: "Full Incident Address"
- description: "Full street address where the incident took place, including building number and street name."
- content_type: street_address

11. Street Name:
- label: "Street Name (Primary)"
- description: "The name of the primary street involved in the incident; combine with Incident Address, City, Borough, and ZIP to form the full address."
- content_type: street_name

12. Cross Street 1:
- label: "Cross Street 1"
- description: "The first cross street at an intersection near the incident location."
- content_type: street_name

13. Cross Street 2:
- label: "Cross Street 2"
- description: "A secondary or alternative cross street related to the incident location."
- content_type: street_name

14. Intersection Street 1:
- label: "Intersection Street 1"
- description: "The first street forming an intersection near the incident site."
- content_type: street_name

15. Intersection Street 2:
- label: "Intersection Street 2"
- description: "The second street forming that same intersection."
- content_type: street_name

16. Address Type:
- label: "Address Format"
- description: "A descriptor of the address format (e.g., ADDRESS, INTERSECTION, BLOCKFACE)."
- content_type: category

17. City:
- label: "City"
- description: "The city in which the incident occurred."
- content_type: city

18. Landmark:
- label: "Nearby Landmark"
- description: "A notable nearby landmark or point of reference for the incident location."
- content_type: free_text

19. Facility Type:
- label: "Facility Category"
- description: "The type of facility involved in the complaint (e.g., DSNY Garage, School District)."
- content_type: category

20. Status:
- label: "Complaint Status"
- description: "Current processing status of the complaint."
- content_type: category

21. Due Date:
- label: "Resolution Deadline Timestamp"
- description: "The deadline by which a resolution or response is expected."
- content_type: datetime:%m/%d/%Y %I:%M:%S %p

22. Resolution Description:
- label: "Resolution Narrative"
- description: "A detailed narrative of the actions taken, findings, or outcomes associated with the complaint."
- content_type: free_text

23. Resolution Action Updated Date:
- label: "Last Update to Resolution Timestamp"
- description: "Timestamp indicating when the resolution action was last updated."
- content_type: datetime:%m/%d/%Y %I:%M:%S %p

24. Community Board:
- label: "Community Board Identifier"
- description: "Identifier for the community board area relevant to the incident."
- content_type: category

25. BBL:
- label: "Borough‑Block‑Lot (BBL)"
- description: "Numeric identifier used in NYC property records."
- content_type: unknown

26. Borough:
- label: "Borough"
- description: "The borough of New York City where the incident occurred."
- content_type: category

27. X Coordinate (State Plane):
- label: "X Coordinate (State Plane)"
- description: "Easting coordinate in the State Plane coordinate system for the incident location."
- content_type: unknown

28. Y Coordinate (State Plane):
- label: "Y Coordinate (State Plane)"
- description: "Northing coordinate in the State Plane coordinate system for the incident location."
- content_type: unknown

29. Open Data Channel Type:
- label: "Submission Channel"
- description: "The medium through which the complaint was submitted to the open‑data platform."
- content_type: category

30. Park Facility Name:
- label: "Park Facility Name"
- description: "Name of the park or recreational facility involved in the incident."
- content_type: free_text (or company_name). But use free_text.

31. Park Borough:
- label: "Park Borough"
- description: "The borough where the referenced park facility is located."
- content_type: category

32. Vehicle Type:
- label: "Vehicle Category"
- description: "Classification of vehicle related to the incident."
- content_type: category

33. Taxi Company Borough:
- label: "Taxi Company Borough"
- description: "The borough in which the taxi company operates or is registered."
- content_type: category

34. Taxi Pick Up Location:
- label: "Taxi Pick‑Up Location"
- description: "Description of the location where a taxi was picked up."
- content_type: free_text

35. Bridge Highway Name:
- label: "Bridge/Highway Name"
- description: "Name or designation of the bridge or highway involved in the incident."
- content_type: category

36. Bridge Highway Direction:
- label: "Bridge/Highway Direction"
- description: "Direction of travel on the bridge or highway."
- content_type: category

37. Road Ramp:
- label: "Road Ramp Type"
- description: "Type of road ramp associated with the incident."
- content_type: category

38. Bridge Highway Segment:
- label: "Bridge/Highway Segment"
- description: "Specific segment or exit identifier for a bridge or highway."
- content_type: category

39. Latitude:
- label: "Latitude (decimal degrees)"
- description: "Decimal‑degree latitude coordinate of the incident location."
- content_type: latitude

40. Longitude:
- label: "Longitude (decimal degrees)"
- description: "Decimal‑degree longitude coordinate of the incident location."
- content_type: longitude

41. Location:
- label: "Coordinate String"
- description: "Textual representation of the latitude and longitude pair for the incident."
- content_type: free_text

Double-check unique_id omission: Unique Key only.

Also maybe other fields that are numeric but not unique could be unknown; but we can keep unknown.

Now produce JSON with keys as field names exactly as original? In first pass they used "Unique Key" etc. The output should have same key names. So use the field names as given (string). Provide sub-object.

Let's craft final JSON accordingly.
## TOKEN USAGE

TokenUsage { prompt: 36530, completion: 8530, total: 45060, elapsed: 145778 }
---
# Description
**Description**

This is a one‑million‑record snapshot of New York City’s 311 complaint system spanning from January 2010 to December 2020. Each record represents an individual citizen or agency report, captured with a unique surrogate key and enriched with timestamps for creation, closure, and last resolution update. The data blend categorical descriptors (agency, complaint type, location setting, status, etc.) with structured numeric fields (geospatial coordinates in State Plane and decimal degrees), free‑text narratives (resolution description, incident address, landmark) and administrative metadata such as borough, community board, and vehicle or park facility identifiers. While the majority of complaints are lodged via phone or online channels, a sizeable portion originates from other sources, and many records lack completion dates, closure information, or resolution text. The dataset’s breadth covers a wide range of incident locations—from residential buildings to bridges, highways, parks, and taxi pick‑up points—providing granular insight into citywide service requests.

**Notable Characteristics**

- **Scale & Coverage** – 1 000 000 distinct complaint entries covering all five NYC boroughs over an eleven‑year period.  
- **High Cardinality & “Other” Categories** – Many fields (e.g., Complaint Type, Descriptor, Incident Zip) are dominated by a single “Other (N)” bucket that aggregates the majority of unique values beyond the top ten, indicating heavy right‑skew and difficulty in downstream categorization.  
- **Temporal Distribution** – Created dates cluster around 2015–2020 with a long tail back to 2010; closure dates follow a similar pattern but include some older defaults (1900/2100) as placeholders for missing data.  
- **Missingness & Sparsity** – Roughly 65 % of records lack a Due Date, ~2.8 % have no Closed Date, and about 25 % miss geospatial coordinates or Incident Address; many categorical fields also exhibit high sparsity (e.g., Address Type null in 12 %).  
- **Outliers & Extreme Values** – Geometric coordinate ranges are bounded by the NYC State Plane system but still contain extreme values at the lower/upper edges; some free‑text fields have unusually long strings (up to ~934 characters).  
- **Duplicates & Uniqueness** – The surrogate Unique Key guarantees record uniqueness; however, other attributes frequently repeat across records (e.g., Agency “NYPD” accounts for >26 % of entries).  
- **PII/PHI Concerns** – Address strings, zip codes, latitude/longitude pairs, and taxi pick‑up locations potentially reveal personally identifying information. Resolution narratives may contain sensitive content.  
- **Data Quality Issues** – The heavy use of “Other” buckets, high missingness in key fields (Due Date, Closure), inconsistent free‑text formatting, and a non‑uniform coordinate system can impede automated analysis without extensive cleaning or transformation.

Generated by qsv v20.1.0 describegpt
Command line: target/debug/qsv describegpt --all --two-pass /tmp/NYC_311_SR_2010-2020-sample-1M.csv -o /tmp/nyc311-describegpt-everything.md --addl-cols --addl-cols-list everything --infer-content-type --fresh
Prompt file: Default v7.1.0
Model: openai/gpt-oss-20b
LLM API URL: http://localhost:1234/v1
Language: 
Timestamp: 2026-05-22T02:40:50.383934+00:00

WARNING: Description generated by an LLM and may contain inaccuracies. Verify before using!



## REASONING

We need to generate a description using summary statistics, frequency distribution and data dictionary data about the dataset as a whole, not field-specific details. Must produce 1-8 sentence description. Then "Notable Characteristics" bullet list with points like central tendency/spread, distribution shape, anomalies, patterns, outliers, missing values, duplicates, PII/PHI etc.

Use information from summary statistics: Unique Key is unique for each record; dataset size appears to be 1,000,000 records. Created Date and Closed Date ranges show timeline from 2010-01-01 to 2020-12-23 for creation, closed date 1900-2100 but actual usage around 2010-2020. Many fields have high cardinality with many unique values; many are categorical with large number of categories (Agency, Complaint Type etc). Frequency distribution shows many "Other" categories representing majority of records for some fields: e.g., Complaint Type other 56% and descriptor other 67%; Incident Zip other 86%. That indicates heavy skewness. Many fields have high sparsity or missing values: e.g., Closed Date null 28619, Due Date null 647794 (65% missing), Resolution Description null 20480 (2%), etc. Location Type has null 239131 (~24%). Address type null 125802.

Also unique key uniqueness ratio 1. There are duplicates? Unique key is surrogate so no duplicates at record level; but other fields may have duplicate values across records.

PII/PHI: dataset includes addresses, zip codes, coordinates, phone numbers maybe not; there is "Taxi Pick Up Location" and "Resolution Description" that might contain PII but we don't know. But the data dictionary shows content_type categories, free_text etc; likely contains PII like names, addresses.

We should mention outliers: Some fields have extreme values e.g., X Coordinate range 913281-1067220; Y coordinate 121152-271876; lat/long within NYC boundaries but some missing.

Also there are many "Other" categories capturing many unique values. The dataset includes a mix of categorical, numeric, datetime, geospatial coordinates, free text, etc.

We should note high skewness: e.g., Created Date distribution shows long tail with most records in 2015-2020; Complaint Type heavily skewed to other. Many fields have high cardinality but low uniqueness ratio (like Incident Zip 535 unique values out of 1M, but 86% captured as Other? Actually cardinality is 535 but other category counts for 524k out of 1M -> indicates that many unique zip codes beyond top 10; distribution heavily right-skewed). Similarly Agency has 28 categories with 3-4 categories dominating.

Missing values: Due Date missing 647k records (~65%); Closed Date missing 28k (~2.8%); others moderate.

Outliers: Created Date min 2010, max 2020; but some older/ newer extremes maybe out of scope? There is "Other (N)" categories indicating many unique values beyond top 10.

Duplicate values: Unique Key all unique.

Potential data quality issues: high sparsity, missing values, heavy use of "Other" categories making analysis difficult; free text fields may contain noise or inconsistent formatting; some numeric fields have extremely skewed distribution with outliers (e.g., X coordinate extremes). Coordinates missing for 254k records (~25%).

Also PII concerns: addresses, coordinates, taxi pick up location, resolution description, etc. Provide note.

Now produce output: one paragraph 1-8 sentences describing dataset overall: "The dataset comprises 1 million NYC 311 complaint records from 2010 to 2020..." etc. Then bullet list of notable characteristics.

Make sure no field-level stats displayed.

Add Attribution placeholder at bottom.

Let's craft.


## TOKEN USAGE

TokenUsage { prompt: 33526, completion: 1419, total: 34945, elapsed: 64196 }
---
# Tags
- nyc_311  
- public_complaints  
- municipal_issues  
- noise_problems  
- housing_disputes  
- transportation_incidents  
- environmental_issues  
- service_requests  
- city_services  
- urban_infrastructure  

Generated by qsv v20.1.0 describegpt
Command line: target/debug/qsv describegpt --all --two-pass /tmp/NYC_311_SR_2010-2020-sample-1M.csv -o /tmp/nyc311-describegpt-everything.md --addl-cols --addl-cols-list everything --infer-content-type --fresh
Prompt file: Default v7.1.0
Model: openai/gpt-oss-20b
LLM API URL: http://localhost:1234/v1
Language: 
Timestamp: 2026-05-22T02:42:12.881381+00:00

WARNING: Tags generated by an LLM and may contain inaccuracies. Verify before using!



## REASONING

We need to produce a list of tags (max 10) in markdown format, descending importance, lowercase only underscore separated. Use the data summary and frequency distribution.

Dataset: NYC 311 complaints? Many fields about complaint type, location, agency, etc.

What are important topics? The dataset covers municipal service complaints across NYC boroughs, covering noise, heat/hot water, illegal parking, street conditions, etc. It's likely a public complaints dataset. Tags should be generic but relevant to content: e.g., "nyc311", "public_service_complaints", "municipal_issues", "noise_complaint", "housing_disputes", "transportation_incidents", "weather_related" maybe.

But we need 10 tags, lower case underscore. Based on summary stats: top Complaint Type categories like noise residential (8.9%), heat/hot water (5.6%), etc. Also Agency types: NYPD, HPD, DOT, DSNY. So topics: "nyc", "311_service_requests" but not field names.

Consider tags: nyc_public_complaints, municipal_issues, noise_problems, housing_disputes, transportation_incidents, environmental_issues, service_request, complaint_data, urban_infrastructure, city_services.

Also we could use "new_york_city" as a tag. But maybe too generic? It's fine.

But need 10 tags maximum; choose the most important descending importance based on data: The largest cardinality fields with high uniqueness ratio? Actually top categories: Complaint Type noise residential (8.9%), heat/hot water (5.6%). So "noise_complaint" and "heat_hydrant_issue"? Not necessary.

Maybe priority tags reflect dataset's primary use: It's a dataset of NYC 311 complaints; so "nyc_311", "public_service_requests".

Also include "complaint_data", "municipal_services". Also "city_governance".

But we should not use field names, but can refer to content like "noise" etc.

Let's list:

1. nyc_311
2. public_complaints
3. municipal_issues
4. noise_problems
5. housing_disputes
6. transportation_incidents
7. environmental_issues
8. service_requests
9. city_services
10. urban_infrastructure

Check lower case underscore only: "nyc_311", "public_complaints" (two words). "municipal_issues". "noise_problems". "housing_disputes". "transportation_incidents". "environmental_issues". "service_requests". "city_services". "urban_infrastructure".

Make sure no field names. Good.

Add attribution line: "{GENERATED_BY_SIGNATURE}". So final output: list of tags in markdown format. Use bullet points? The instruction says "in Markdown format" but not exactly how. Usually a list with hyphens. Then after the tags add attribution. We'll produce:

- nyc_311
- public_complaints
...

Then {GENERATED_BY_SIGNATURE}.

Let's do that.
## TOKEN USAGE

TokenUsage { prompt: 33501, completion: 701, total: 34202, elapsed: 61490 }
---
