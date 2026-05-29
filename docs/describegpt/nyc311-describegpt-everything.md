# Dictionary
| Name | Type | Label | Description | Content Type | Min | Max | Cardinality | Enumeration | Null Count | is_ascii | sum | range | sort_order | sortiness | min_length | max_length | sum_length | avg_length | stddev_length | variance_length | cv_length | mean | sem | geometric_mean | harmonic_mean | stddev | variance | cv | n_negative | n_zero | n_positive | max_precision | sparsity | mad | lower_outer_fence | lower_inner_fence | q1 | q2_median | q3 | iqr | upper_inner_fence | upper_outer_fence | skewness | uniqueness_ratio | percentiles | Examples |
|------|------|-------|-------------|--------------|-----|-----|-------------|-------------|------------|----------|----------|----------|----------|----------|----------|----------|----------|----------|----------|----------|----------|----------|----------|----------|----------|----------|----------|----------|----------|----------|----------|----------|----------|----------|----------|----------|----------|----------|----------|----------|----------|----------|----------|----------|----------|----------|
| **Unique Key** | Integer | Record ID | An integer that uniquely identifies each complaint record in the dataset. | unique_id | 11465364 | 48478173 | 1,000,000 |  | 0 |  | 32687965858032 | 37012809 | Unsorted | 0.0018 |  |  |  |  |  |  |  | 32687965.858 | 9013.8953 | 31351729.249 | 29944311.4641 | 9013895.3358 | 81250309125279.2656 | 27.5756 | 0 | 0 | 1000000 |  | 0 | 7477037 | -19639208.5 | 2803282.25 | 25245773 | 32853358.5 | 40207433.5 | 14961660.5 | 62649924.25 | 85092415 | -0.0169 | 1 | 5: 18453724<br>10: 20062969<br>40: 29913180<br>60: 35829112<br>90: 45355115<br>95: 46937288 | <ALL_UNIQUE> |
| **Created Date** | DateTime | Complaint Submission Date | The date and time when the complaint was submitted, recorded to the second in ISO‑8601 UTC format. | datetime | 2010-01-01T00:00:00+00:00 | 2020-12-23T01:25:51+00:00 | 841,014 |  | 0 |  |  | 4009.05962 | Unsorted | 0.0008 |  |  |  |  |  |  |  | 2015-11-10T18:05:22.615+00:00 | 1.15502 | 16709.46856 | 16668.78207 | 1155.01606 | 1334062.09198 | 6.8957 |  |  |  |  | 0 | 965.6694 | 1997-01-08T17:56:34.500+00:00 | 2005-02-08T08:58:19.500+00:00 | 2013-03-11T00:00:04.500+00:00 | 2016-02-12T13:16:49+00:00 | 2018-07-31T10:01:14.500+00:00 | 1968.41748 | 2026-08-31T01:02:59.500+00:00 | 2034-09-30T16:04:44.500+00:00 | -0.0857 | 0.841 | 5: 2010-08-10T00:00:00+00:00<br>10: 2011-03-15T11:35:08+00:00<br>40: 2015-01-21T10:24:00+00:00<br>60: 2017-02-25T20:10:00+00:00<br>90: 2020-01-10T08:26:00+00:00<br>95: 2020-07-21T18:32:11+00:00 | Other… [997,333]<br>01/24/2013 12:00:00 AM [347]<br>01/07/2014 12:00:00 AM [315]<br>01/08/2015 12:00:00 AM [283]<br>02/16/2015 12:00:00 AM [269] |
| **Closed Date** | DateTime | Complaint Closure Date | The date and time when the complaint was closed or resolved, recorded to the second in ISO‑8601 UTC format. | datetime | 1900-01-01T00:00:00+00:00 | 2100-01-01T00:00:00+00:00 | 688,837 |  | 28,619 |  |  | 73049 | Unsorted | 0.001 |  |  |  |  |  |  |  | 2015-11-14T10:16:16.743+00:00 | 1.33393 |  |  | 1314.70016 | 1728436.50813 | 7.8474 |  |  |  |  | 0.0286 | 954.61806 | 1997-04-12T11:33:24.500+00:00 | 2005-04-09T10:53:21.500+00:00 | 2013-04-06T10:13:18.500+00:00 | 2016-02-26T01:40:00+00:00 | 2018-08-04T09:46:36.500+00:00 | 1945.98146 | 2026-08-01T09:06:33.500+00:00 | 2034-07-29T08:26:30.500+00:00 | -0.0849 | 0.6888 | 5: 2010-08-23T11:35:00+00:00<br>10: 2011-04-01T00:00:00+00:00<br>40: 2015-02-09T00:00:00+00:00<br>60: 2017-03-08T14:06:03+00:00<br>90: 2020-01-07T00:02:00+00:00<br>95: 2020-07-20T12:00:00+00:00 | Other… [968,671]<br>(NULL)… [28,619]<br>11/15/2010 12:00:00 AM [384]<br>11/07/2012 12:00:00 AM [329]<br>12/09/2010 12:00:00 AM [267] |
| **Agency** | String | Responding Agency Abbreviation | Abbreviated code of the agency responsible for handling the complaint (e.g., NYPD, HPD). | category | 3-1-1 | TLC | 28 |  | 0 | false |  |  | Unsorted | 0.1729 | 3 | 42 | 3490582 | 3.4906 | 1.8975 | 3.6005 | 0.5436 |  |  |  |  |  |  |  |  |  |  |  | 0 |  |  |  |  |  |  |  |  |  |  | 0 |  | NYPD [265,116]<br>HPD [258,033]<br>DOT [132,462]<br>DSNY [81,606]<br>DEP [75,895] |
| **Agency Name** | String | Responding Agency Full Name | Full official name of the agency that handled the complaint. | category | 3-1-1 | Valuation Policy | 553 |  | 0 | false |  |  | Unsorted | 0.1671 | 3 | 82 | 34840715 | 34.8407 | 10.5137 | 110.5379 | 0.3018 |  |  |  |  |  |  |  |  |  |  |  | 0 |  |  |  |  |  |  |  |  |  |  | 0.0006 |  | New York City Police Depa… [265,038]<br>Department of Housing Pre… [258,019]<br>Department of Transportat… [132,462]<br>Other… [103,974]<br>Department of Environment… [75,895] |
| **Complaint Type** | String | Primary Complaint Category | High‑level category of the complaint (e.g., Noise - Residential, HEAT/HOT WATER). | category | ../../WEB-INF/web.xml;x= | ZTESTINT | 287 |  | 0 | true |  |  | Unsorted | 0.0284 | 3 | 41 | 16475270 | 16.4753 | 6.8221 | 46.5406 | 0.4141 |  |  |  |  |  |  |  |  |  |  |  | 0 |  |  |  |  |  |  |  |  |  |  | 0.0003 |  | Other… [563,561]<br>Noise - Residential [89,439]<br>HEAT/HOT WATER [56,639]<br>Illegal Parking [45,032]<br>Blocked Driveway [42,356] |
| **Descriptor** | String | Detailed Complaint Description | More specific description of the issue reported in the complaint. | free_text | 1 Missed Collection | unknown odor/taste in drinking water (QA6) | 1,392 |  | 3,001 | true |  |  | Unsorted | 0.0186 | 0 | 80 | 17426583 | 17.4266 | 10.4342 | 108.8723 | 0.5988 |  |  |  |  |  |  |  |  |  |  |  | 0.003 |  |  |  |  |  |  |  |  |  |  | 0.0014 |  | Other… [671,870]<br>Loud Music/Party [93,646]<br>ENTIRE BUILDING [36,885]<br>HEAT [35,088]<br>No Access [31,631] |
| **Location Type** | String | Incident Location Category | General type of location where the incident occurred (e.g., Residential Building, Street). | category | 1-, 2- and 3- Family Home | Wooded Area | 162 |  | 239,131 | true |  |  | Unsorted | 0.187 | 0 | 36 | 12417750 | 12.4177 | 8.9759 | 80.5671 | 0.7228 |  |  |  |  |  |  |  |  |  |  |  | 0.2391 |  |  |  |  |  |  |  |  |  |  | 0.0002 |  | RESIDENTIAL BUILDING [255,562]<br>(NULL)… [239,131]<br>Street/Sidewalk [145,653]<br>Residential Building/Hous… [92,765]<br>Street [92,190] |
| **Incident Zip** | String | Incident ZIP Code | Five‑digit ZIP code of the incident location. | zip_code | * | XXXXX | 535 |  | 54,978 | true |  |  | Unsorted | 0.0085 | 0 | 10 | 4347871 | 4.3479 | 1.14 | 1.2996 | 0.2622 |  |  |  |  |  |  |  |  |  |  |  | 0.055 |  |  |  |  |  |  |  |  |  |  | 0.0005 |  | Other… [815,988]<br>(NULL)… [54,978]<br>11226 [17,114]<br>10467 [14,495]<br>11207 [12,872] |
| **Incident Address** | String | Full Incident Street Address | Street address where the incident took place; combine with City, Borough and Incident Zip to form a full mailing address. | street_address | * * | west 155 street and edgecombe avenue | 341,996 |  | 174,700 | true |  |  | Unsorted | -0.0005 | 0 | 55 | 14591947 | 14.5919 | 7.332 | 53.7589 | 0.5025 |  |  |  |  |  |  |  |  |  |  |  | 0.1747 |  |  |  |  |  |  |  |  |  |  | 0.342 |  | Other… [819,046]<br>(NULL)… [174,700]<br>655 EAST  230 STREET [1,538]<br>78-15 PARSONS BOULEVARD [694]<br>672 EAST  231 STREET [663] |
| **Street Name** | String | Main Street Name | Name of the primary street involved in the incident; part of the incident address. | street_name | * | wyckoff avenue | 14,837 |  | 174,720 | true |  |  | Unsorted | 0.0001 | 0 | 55 | 10888475 | 10.8885 | 5.7968 | 33.6032 | 0.5324 |  |  |  |  |  |  |  |  |  |  |  | 0.1747 |  |  |  |  |  |  |  |  |  |  | 0.0148 |  | Other… [784,684]<br>(NULL)… [174,720]<br>BROADWAY [9,702]<br>GRAND CONCOURSE [5,851]<br>OCEAN AVENUE [3,946] |
| **Cross Street 1** | String | First Cross Street | Name of the first cross street intersecting at the incident location (if applicable). | street_name | 1 AVE | mermaid | 16,238 |  | 320,401 | true |  |  | Unsorted | 0.0009 | 0 | 32 | 8355458 | 8.3555 | 6.6045 | 43.6193 | 0.7904 |  |  |  |  |  |  |  |  |  |  |  | 0.3204 |  |  |  |  |  |  |  |  |  |  | 0.0162 |  | Other… [619,743]<br>(NULL)… [320,401]<br>BEND [12,562]<br>BROADWAY [8,548]<br>3 AVENUE [6,154] |
| **Cross Street 2** | String | Second Cross Street | Name of the second cross street intersecting at the incident location (if applicable). | street_name | 1 AVE | surf | 16,486 |  | 323,644 | true |  |  | Unsorted | 0.0016 | 0 | 35 | 8363431 | 8.3634 | 6.645 | 44.1554 | 0.7945 |  |  |  |  |  |  |  |  |  |  |  | 0.3236 |  |  |  |  |  |  |  |  |  |  | 0.0165 |  | Other… [623,363]<br>(NULL)… [323,644]<br>BEND [12,390]<br>BROADWAY [8,833]<br>DEAD END [5,626] |
| **Intersection Street 1** | String | First Intersection Street | Name of one street forming the intersection where the incident occurred; may differ from Cross Street 1. | street_name | 1 AVE | flatlands AVE | 11,237 |  | 767,422 | true |  |  | Unsorted | -0.0009 | 0 | 35 | 2949273 | 2.9493 | 5.6793 | 32.2544 | 1.9257 |  |  |  |  |  |  |  |  |  |  |  | 0.7674 |  |  |  |  |  |  |  |  |  |  | 0.0112 |  | (NULL)… [767,422]<br>Other… [214,544]<br>BROADWAY [3,761]<br>CARPENTER AVENUE [2,918]<br>BEND [2,009] |
| **Intersection Street 2** | String | Second Intersection Street | Name of another street forming the intersection where the incident occurred; may differ from Cross Street 2. | street_name | 1 AVE | glenwood RD | 11,674 |  | 767,709 | true |  |  | Unsorted | 0.003 | 0 | 33 | 2917798 | 2.9178 | 5.6363 | 31.768 | 1.9317 |  |  |  |  |  |  |  |  |  |  |  | 0.7677 |  |  |  |  |  |  |  |  |  |  | 0.0117 |  | (NULL)… [767,709]<br>Other… [215,667]<br>BROADWAY [3,462]<br>BEND [1,942]<br>2 AVENUE [1,690] |
| **Address Type** | String | Address Classification | Classification indicating how the address is represented (e.g., ADDRESS, INTERSECTION, BLOCKFACE, LATLONG). | category | ADDRESS | PLACENAME | 6 |  | 125,802 | true |  |  | Unsorted | 0.6845 | 0 | 12 | 6832263 | 6.8323 | 3.0923 | 9.5623 | 0.4526 |  |  |  |  |  |  |  |  |  |  |  | 0.1258 |  |  |  |  |  |  |  |  |  |  | 0 |  | ADDRESS [710,380]<br>INTERSECTION [133,361]<br>(NULL)… [125,802]<br>BLOCKFACE [22,620]<br>LATLONG [7,421] |
| **City** | String | Incident City | Name of the city in which the incident took place. | city | * | YORKTOWN HEIGHTS | 382 |  | 61,963 | true |  |  | Unsorted | 0.1811 | 0 | 22 | 7721241 | 7.7212 | 3.2635 | 10.6505 | 0.4227 |  |  |  |  |  |  |  |  |  |  |  | 0.062 |  |  |  |  |  |  |  |  |  |  | 0.0004 |  | BROOKLYN [296,254]<br>NEW YORK [189,069]<br>BRONX [181,168]<br>Other… [163,936]<br>(NULL)… [61,963] |
| **Landmark** | String | Nearby Landmark | Notable landmark or point of reference near the incident location, provided as free text. | free_text | 1 AVENUE | ZULETTE AVENUE | 5,915 |  | 912,779 | true |  |  | Unsorted | 0.0009 | 0 | 32 | 1165773 | 1.1658 | 3.8978 | 15.1925 | 3.3435 |  |  |  |  |  |  |  |  |  |  |  | 0.9128 |  |  |  |  |  |  |  |  |  |  | 0.0059 |  | (NULL)… [912,779]<br>Other… [80,165]<br>EAST  230 STREET [1,545]<br>EAST  231 STREET [1,291]<br>BROADWAY [1,148] |
| **Facility Type** | String | Related Facility Type | Type of facility involved in the complaint (e.g., DSNY Garage). | category | DSNY Garage | School District | 6 |  | 145,478 | true |  |  | Unsorted | 0.5941 | 0 | 15 | 3790876 | 3.7909 | 2.7562 | 7.5969 | 0.7271 |  |  |  |  |  |  |  |  |  |  |  | 0.1455 |  |  |  |  |  |  |  |  |  |  | 0 |  | N/A [628,279]<br>Precinct [193,259]<br>(NULL)… [145,478]<br>DSNY Garage [32,310]<br>School [617] |
| **Status** | String | Complaint Status | Current status of the complaint (e.g., Closed, Pending, Open). | category | Assigned | Unspecified | 10 | Assigned<br>Closed<br>Closed - Testing<br>Email Sent<br>In Progress<br>Open<br>Pending<br>Started<br>Unassigned<br>Unspecified | 0 | true |  |  | Unsorted | 0.9079 | 4 | 16 | 6048943 | 6.0489 | 0.5411 | 0.2928 | 0.0894 |  |  |  |  |  |  |  |  |  |  |  | 0 |  |  |  |  |  |  |  |  |  |  | 0 |  | Closed [952,522]<br>Pending [20,119]<br>Open [12,340]<br>In Progress [7,841]<br>Assigned [6,651] |
| **Due Date** | DateTime | Required Action Deadline | Deadline for action on the complaint, recorded to the second in US‑month/day/year format. | datetime:%m/%d/%Y %I:%M:%S %p | 01/02/1900 12:00:00 AM | 06/17/2021 04:34:13 PM | 345,077 |  | 647,794 |  |  | 44361.69043 | Unsorted | 0.0011 |  |  |  |  |  |  |  | 2015-05-30T02:54:49.998+00:00 | 1.74433 |  |  | 1035.204 | 1071647.31812 | 6.2418 |  |  |  |  | 0.6478 | 800.07713 | 1999-09-09T02:53:37+00:00 | 2006-06-14T19:09:32.500+00:00 | 2013-03-20T11:25:28+00:00 | 2015-10-03T01:27:48+00:00 | 2017-09-22T14:16:05+00:00 | 1647.11848 | 2024-06-28T06:32:00.500+00:00 | 2031-04-03T22:47:56+00:00 | -0.1251 | 0.3451 | 5: 2010-08-25T10:54:44+00:00<br>10: 2011-04-14T19:02:13+00:00<br>40: 2014-11-06T10:17:31+00:00<br>60: 2016-07-31T14:14:53+00:00<br>90: 2018-10-25T07:47:20+00:00<br>95: 2019-03-24T08:03:29+00:00 | (NULL)… [647,794]<br>Other… [350,746]<br>04/08/2015 10:00:58 AM [214]<br>05/02/2014 03:32:17 PM [183]<br>03/30/2018 10:10:39 AM [172] |
| **Resolution Description** | String | Resolution Narrative | Narrative text describing how the complaint was resolved or what actions were taken. | free_text | A DOB violation was issued for failing to comply with an existing Stop Work Order. | Your request was submitted to the Department of Homeless Services. The City?s outreach team will assess the homeless individual and offer appropriate assistance within 2 hours. If you asked to know the outcome of your request, you will get a call within 2 hours. No further status will be available through the NYC 311 App, 311, or 311 Online. | 1,216 |  | 20,480 | false |  |  | Unsorted | 0.0319 | 0 | 934 | 153148305 | 153.1483 | 82.149 | 6748.4538 | 0.5364 |  |  |  |  |  |  |  |  |  |  |  | 0.0205 |  |  |  |  |  |  |  |  |  |  | 0.0012 |  | Other… [511,739]<br>The Police Department res… [91,408]<br>The Department of Housing… [72,962]<br>The Police Department res… [63,868]<br>Service Request status fo… [52,155] |
| **Resolution Action Updated Date** | DateTime | Last Resolution Update Date | Timestamp of the most recent update to the resolution action, recorded to the second in US‑month/day/year format. | date:%m/%d/%Y | 12/31/2009 | 12/23/2020 | 690,314 |  | 15,072 |  |  | 4010.22308 | Unsorted | 0.001 |  |  |  |  |  |  |  | 2015-11-19T19:44:34.889+00:00 | 1.16204 | 16718.67594 | 16678.12298 | 1153.24922 | 1329983.75668 | 6.8814 |  |  |  |  | 0.0151 | 966.48803 | 1997-01-12T11:30:24+00:00 | 2005-02-14T12:31:15.750+00:00 | 2013-03-19T13:32:07.500+00:00 | 2016-02-22T22:38:30+00:00 | 2018-08-10T14:12:42+00:00 | 1970.02818 | 2026-09-12T15:13:33.750+00:00 | 2034-10-15T16:14:25.500+00:00 | -0.0867 | 0.6903 | 5: 2010-08-22T00:00:00+00:00<br>10: 2011-03-25T11:49:23+00:00<br>40: 2015-02-01T00:00:00+00:00<br>60: 2017-03-10T02:24:33+00:00<br>90: 2020-01-12T09:08:00+00:00<br>95: 2020-07-22T01:06:53+00:00 | Other… [982,148]<br>(NULL)… [15,072]<br>11/15/2010 12:00:00 AM [385]<br>11/07/2012 12:00:00 AM [336]<br>12/09/2010 12:00:00 AM [273] |
| **Community Board** | String | Community Board Number | Number or identifier of the community board associated with the incident location. | category | 0 Unspecified | Unspecified STATEN ISLAND | 77 |  | 0 | true |  |  | Unsorted | 0.0193 | 8 | 25 | 11142863 | 11.1429 | 2.971 | 8.8269 | 0.2666 |  |  |  |  |  |  |  |  |  |  |  | 0 |  |  |  |  |  |  |  |  |  |  | 0.0001 |  | Other… [751,635]<br>0 Unspecified [49,878]<br>12 MANHATTAN [29,845]<br>12 QUEENS [23,570]<br>01 BROOKLYN [21,714] |
| **BBL** | String | Borough Block Lot Identifier | Borough, Block, Lot (BBL) identifier used for NYC property records; can be cross‑referenced with Community Board. | unknown | 0000000000 | 5080470043 | 268,383 |  | 243,046 | true |  |  | Unsorted | -0.0009 | 0 | 10 | 448540 | 0.4485 | 4.3011 | 18.4993 | 9.5891 |  |  |  |  |  |  |  |  |  |  |  | 0.243 |  |  |  |  |  |  |  |  |  |  | 0.2684 |  | Other… [750,668]<br>(NULL)… [243,046]<br>2048330028 [1,566]<br>4068290001 [696]<br>4015110001 [664] |
| **Borough** | String | Incident Borough | NYC borough where the incident occurred. | category | BRONX | Unspecified | 6 | BRONX<br>BROOKLYN<br>MANHATTAN<br>QUEENS<br>STATEN ISLAND<br>Unspecified | 0 | true |  |  | Unsorted | 0.2155 | 5 | 13 | 7595025 | 7.595 | 2.0632 | 4.2568 | 0.2717 |  |  |  |  |  |  |  |  |  |  |  | 0 |  |  |  |  |  |  |  |  |  |  | 0 |  | BROOKLYN [296,081]<br>QUEENS [228,818]<br>MANHATTAN [195,488]<br>BRONX [180,142]<br>Unspecified [49,878] |
| **X Coordinate (State Plane)** | Integer | State Plane X Coordinate (Easting) | East–west coordinate of the incident in New York State Plane units; used for spatial analysis. | unknown | 913281 | 1067220 | 102,556 |  | 85,327 |  | 919555108413 | 153939 | Unsorted | -0.0004 |  |  |  |  |  |  |  | 1005337.5451 | 23.5391 | 1005083.7023 | 1004827.9356 | 22512.4528 | 506810531.5324 | 2.2393 | 0 | 0 | 914673 |  | 0.0853 | 12292 | 919661 | 956616.5 | 993572 | 1004546 | 1018209 | 24637 | 1055164.5 | 1092120 | 0.1091 | 0.1026 | 5: 964313<br>10: 984035<br>40: 999859<br>60: 1009147<br>90: 1034015<br>95: 1043903 | Other… [908,535]<br>(NULL)… [85,327]<br>1022911 [1,568]<br>1037000 [701]<br>1023174 [675] |
| **Y Coordinate (State Plane)** | Integer | State Plane Y Coordinate (Northing) | North–south coordinate of the incident in New York State Plane units; used for spatial analysis. | unknown | 121152 | 271876 | 116,092 |  | 85,327 |  | 188099299101 | 150724 | Unsorted | 0 |  |  |  |  |  |  |  | 205646.4978 | 33.1699 | 203166.0871 | 200659.7012 | 31723.1985 | 1006361322.6747 | 15.4261 | 0 | 0 | 914673 |  | 0.0853 | 24236 | 24257 | 103334 | 182411 | 202514 | 235129 | 52718 | 314206 | 393283 | 0.2373 | 0.1161 | 5: 156639<br>10: 164744<br>40: 193463<br>60: 212470<br>90: 250365<br>95: 256054 | Other… [908,538]<br>(NULL)… [85,327]<br>264242 [1,566]<br>202363 [706]<br>211606 [665] |
| **Open Data Channel Type** | String | Reporting Channel | Medium through which the complaint was reported (e.g., PHONE, ONLINE). | category | MOBILE | UNKNOWN | 5 | MOBILE<br>ONLINE<br>OTHER<br>PHONE<br>UNKNOWN | 0 | true |  |  | Unsorted | 0.3379 | 5 | 7 | 5718030 | 5.718 | 0.8144 | 0.6633 | 0.1424 |  |  |  |  |  |  |  |  |  |  |  | 0 |  |  |  |  |  |  |  |  |  |  | 0 |  | PHONE [497,606]<br>UNKNOWN [230,402]<br>ONLINE [177,334]<br>MOBILE [79,892]<br>OTHER [14,766] |
| **Park Facility Name** | String | Park Facility Name | Name of a park facility involved in the incident, if applicable. | free_text | "Uncle" Vito F. Maranzano Glendale Playground | Zimmerman Playground | 1,889 |  | 0 | true |  |  | Unsorted | 0.9863 | 3 | 82 | 11072428 | 11.0724 | 1.2391 | 1.5353 | 0.1119 |  |  |  |  |  |  |  |  |  |  |  | 0 |  |  |  |  |  |  |  |  |  |  | 0.0019 |  | Unspecified [993,141]<br>Other… [5,964]<br>Central Park [261]<br>Riverside Park [136]<br>Prospect Park [129] |
| **Park Borough** | String | Park Borough | NYC borough where the park facility is located. | category | BRONX | Unspecified | 6 | BRONX<br>BROOKLYN<br>MANHATTAN<br>QUEENS<br>STATEN ISLAND<br>Unspecified | 0 | true |  |  | Unsorted | 0.2155 | 5 | 13 | 7595025 | 7.595 | 2.0632 | 4.2568 | 0.2717 |  |  |  |  |  |  |  |  |  |  |  | 0 |  |  |  |  |  |  |  |  |  |  | 0 |  | BROOKLYN [296,081]<br>QUEENS [228,818]<br>MANHATTAN [195,488]<br>BRONX [180,142]<br>Unspecified [49,878] |
| **Vehicle Type** | String | Vehicle Involved Type | Type of vehicle involved in the incident (e.g., Car Service, Ambulette). | category | Ambulette / Paratransit | Green Taxi | 5 |  | 999,652 | true |  |  | Unsorted | 0.8329 | 0 | 23 | 4066 | 0.0041 | 0.2293 | 0.0526 | 56.4051 |  |  |  |  |  |  |  |  |  |  |  | 0.9997 |  |  |  |  |  |  |  |  |  |  | 0 |  | (NULL)… [999,652]<br>Car Service [317]<br>Ambulette / Paratransit [19]<br>Commuter Van [11]<br>Green Taxi [1] |
| **Taxi Company Borough** | String | Taxi Company Headquarters Borough | Borough where the taxi company is headquartered. | category | BRONX | Staten Island | 11 |  | 999,156 | true |  |  | Unsorted | 0.1839 | 0 | 13 | 6313 | 0.0063 | 0.2259 | 0.051 | 35.783 |  |  |  |  |  |  |  |  |  |  |  | 0.9992 |  |  |  |  |  |  |  |  |  |  | 0 |  | (NULL)… [999,156]<br>BROOKLYN [207]<br>QUEENS [194]<br>MANHATTAN [171]<br>BRONX [127] |
| **Taxi Pick Up Location** | String | Taxi Pickup Location Description | Description of the location where a taxi was picked up, provided as free text. | free_text | 1 5 AVENUE MANHATTAN | YORK AVENUE AND EAST 70 STREET | 1,903 |  | 992,129 | true |  |  | Unsorted | 0.2854 | 0 | 60 | 135661 | 0.1357 | 2.1518 | 4.6304 | 15.8618 |  |  |  |  |  |  |  |  |  |  |  | 0.9921 |  |  |  |  |  |  |  |  |  |  | 0.0019 |  | (NULL)… [992,129]<br>Other [4,091]<br>Other… [2,006]<br>JFK Airport [562]<br>Intersection [486] |
| **Bridge Highway Name** | String | Bridge/Highway Name | Name of the bridge or highway involved in the incident. | category | 145th St. Br - Lenox Ave | Willis Ave Br - 125th St/1st Ave | 68 |  | 997,711 | true |  |  | Unsorted | 0.0227 | 0 | 42 | 36974 | 0.037 | 0.8221 | 0.6759 | 22.2353 |  |  |  |  |  |  |  |  |  |  |  | 0.9977 |  |  |  |  |  |  |  |  |  |  | 0.0001 |  | (NULL)… [997,711]<br>Other… [779]<br>Belt Pkwy [276]<br>BQE/Gowanus Expwy [254]<br>Grand Central Pkwy [186] |
| **Bridge Highway Direction** | String | Bridge/Highway Direction | Direction of travel on the bridge or highway (e.g., East/Long Island Bound). | category | Bronx Bound | Westbound/To Goethals Br | 50 |  | 997,691 | true |  |  | Unsorted | 0.0338 | 0 | 33 | 44089 | 0.0441 | 0.9533 | 0.9089 | 21.6233 |  |  |  |  |  |  |  |  |  |  |  | 0.9977 |  |  |  |  |  |  |  |  |  |  | 0.0001 |  | (NULL)… [997,691]<br>Other… [987]<br>East/Long Island Bound [210]<br>North/Bronx Bound [208]<br>East/Queens Bound [197] |
| **Road Ramp** | String | Road Ramp Classification | Classification of the road ramp involved in the incident. | category | N/A | Roadway | 4 |  | 997,693 | true |  |  | Unsorted | 0.6245 | 0 | 7 | 14400 | 0.0144 | 0.3069 | 0.0942 | 21.3118 |  |  |  |  |  |  |  |  |  |  |  | 0.9977 |  |  |  |  |  |  |  |  |  |  | 0 |  | (NULL)… [997,693]<br>Roadway [1,731]<br>Ramp [555]<br>N/A [21] |
| **Bridge Highway Segment** | String | Bridge/Highway Segment Identifier | Specific segment identifier for a bridge or highway involved in the incident. | category | 1-1-1265963747 | Wythe Ave/Kent Ave (Exit 31) | 937 |  | 997,556 | true |  |  | Unsorted | -0.007 | 0 | 100 | 110781 | 0.1108 | 2.5166 | 6.3334 | 22.7171 |  |  |  |  |  |  |  |  |  |  |  | 0.9976 |  |  |  |  |  |  |  |  |  |  | 0.0009 |  | (NULL)… [997,556]<br>Other… [2,144]<br>Ramp [92]<br>Roadway [54]<br>Clove Rd/Richmond Rd (Exi… [23] |
| **Latitude** | Float | Geographic Latitude | Geographic latitude coordinate of the incident location; together with Longitude defines a point on the map. | latitude | 40.1123853 | 40.9128688 | 353,694 |  | 254,695 |  | 30355391.7604 | 0.8005 | Unsorted | -0.001 |  |  |  |  |  |  |  | 40.7288 | 0.0001 | 40.7287 | 40.7286 | 0.0893 | 0.008 | 0.2193 | 0 | 0 | 745305 | 15 | 0.2547 | 0.0632 | 40.2615 | 40.4646 | 40.6677 | 40.7222 | 40.8031 | 0.1354 | 41.0062 | 41.2094 | 0.1957 | 0.3537 | 5: 40.5955<br>10: 40.6175<br>40: 40.6986<br>60: 40.748<br>90: 40.8521<br>95: 40.8684 | Other… [739,329]<br>(NULL)… [254,695]<br>40.89187241649303 [1,538]<br>40.1123853 [1,153]<br>40.89238451539139 [663] |
| **Longitude** | Float | Geographic Longitude | Geographic longitude coordinate of the incident location; together with Latitude defines a point on the map. | longitude | -77.5195844 | -73.7005968 | 353,996 |  | 254,695 |  | -55100392.9499 | 3.819 | Unsorted | -0.0008 |  |  |  |  |  |  |  | -73.93 | 0.0002 |  |  | 0.1635 | 0.0267 | -0.2212 | 745305 | 0 | 0 | 14 | 0.2547 | 0.0468 | -74.2533 | -74.1119 | -73.9705 | -73.9279 | -73.8763 | 0.0943 | -73.7349 | -73.5935 | 0.0964 | 0.354 | 5: -74.0787<br>10: -74.0022<br>40: -73.9454<br>60: -73.9106<br>90: -73.8191<br>95: -73.7839 | Other… [739,329]<br>(NULL)… [254,695]<br>-73.86016845296459 [1,538]<br>-77.5195844 [1,153]<br>-73.8592161325675 [663] |
| **Location** | String | Combined Latitude and Longitude Coordinates | String representation of latitude and longitude pair in parentheses (e.g., "(40.1123853, -77.5195844)"); equivalent to the Latitude and Longitude fields. | free_text | (40.1123853, -77.5195844) | (40.9128688, -73.9024731) | 375,772 |  | 254,695 | true |  |  | Unsorted | -0.001 | 0 | 40 | 19709659 | 19.7097 | 12.1918 | 148.6401 | 0.6186 |  |  |  |  |  |  |  |  |  |  |  | 0.2547 |  |  |  |  |  |  |  |  |  |  | 0.3758 |  | Other… [739,329]<br>(NULL)… [254,695]<br>(40.89187241649303, -73.8… [1,538]<br>(40.1123853, -77.5195844) [1,153]<br>(40.89238451539139, -73.8… [663] |

*Attribution: Generated by qsv v20.1.0 describegpt
Command line: target/debug/qsv describegpt --all --two-pass /tmp/NYC_311_SR_2010-2020-sample-1M.csv -o /tmp/nyc311-describegpt-everything.md --addl-cols --addl-cols-list everything --infer-content-type --fresh
Prompt file: Default v7.1.0
Model: openai/gpt-oss-20b
LLM API URL: http://localhost:1234/v1
Language: 
Timestamp: 2026-05-29T23:32:54.761295+00:00

WARNING: Label and Description generated by an LLM and may contain inaccuracies. Verify before using!
*

## REASONING

FIRST PASS REASONING:
We need to generate Labels, Descriptions and Content Types for all 41 fields. Use summary stats and frequency distribution.

Also need relationships array: joint, ordered, correlated.

Let's analyze each field.

1 Unique Key: integer unique, cardinality equal to row count? summary says cardinality 1000000? Wait dataset has 1M rows. So Unique Key is unique per record; content type likely uuid or unique_id but the rule says don't use unique_id; pick most specific token that fits e.g., "uuid" if format looks like uuid? Not given. It's integer sequence from 32687965858032 to 11465364?? Actually min max values weird: min 11465364, max 32687965858032, but range huge. Not typical primary key; but cardinality=1M equal row count. So content type maybe "unique_id" but we are told not to use unique_id. The rule says: For such fields, pick the most specific other token that fits (e.g., "uuid", "isbn"); deterministic override replaces it only when the row-count check matches. Since cardinality equals row count, qsv would set type as "unique_id" but we cannot output that; we must choose another token that fits: maybe "unknown"? But better to use "unknown". Because it's an integer key no semantic meaning.

So content_type unknown.

Label: "Unique Key". Description: Unique identifier for each record. Provide details.

2 Created Date: type datetime, format appears as "01/24/2013 12:00:00 AM" etc. So date/time with format mm/dd/yyyy hh:mm:ss AM/PM? In freq values: e.g., "01/24/2013 12:00:00 AM". That is "%m/%d/%Y %I:%M:%S %p". Use suffix accordingly.

Content_type: datetime:%m/%d/%Y %I:%M:%S %p

Description: timestamp when complaint was created.

3 Closed Date: same format mm/dd/yyyy 12:00:00 AM. So datetime:%m/%d/%Y %I:%M:%S %p. Description.

4 Agency: String but values like NYPD etc. Category type "category" fits. Content_type category.

5 Agency Name: similar; content_type category.

6 Complaint Type: category.

7 Descriptor: category.

8 Location Type: category.

9 Incident Zip: string numeric 5 digits. Could be zip_code. But summary shows 'XXXXX' maybe placeholder for missing? It includes "*" as wildcard? Frequency distribution has values like 11226 etc. So treat as zip_code. content_type zip_code.

10 Incident Address: free_text (street address). We have street_address? Actually format like "655 EAST 230 STREET". That is a full address but not in standard format; but we can classify as street_address maybe? But description may mention it's the street address where incident occurred. Since there are also separate fields for Street Name, Cross Streets etc., this might be raw address string. So content_type street_address.

11 Street Name: category or street_name token. Values like "BROADWAY". So content_type street_name.

12 Cross Street 1: street_name.

13 Cross Street 2: street_name.

14 Intersection Street 1: street_name.

15 Intersection Street 2: street_name.

16 Address Type: category (ADDRESS, INTERSECTION etc.)

17 City: city token.

18 Landmark: free_text? Values like "EAST 230 STREET". Could be street_address or landmark. Use free_text maybe. But we can call it free_text or maybe street_address? Not standard. We'll use free_text.

19 Facility Type: category.

20 Status: category.

21 Due Date: datetime mm/dd/yyyy hh:mm:ss AM/PM. content_type datetime:%m/%d/%Y %I:%M:%S %p

22 Resolution Description: free_text (long sentences). So content_type free_text.

23 Resolution Action Updated Date: datetime same format. content_type datetime:%m/%d/%Y %I:%M:%S %p

24 Community Board: category.

25 BBL: integer but maybe building block lot ID; unique? cardinality 1000000? frequency shows 'Other (268,372)' etc. Not unique. So unknown numeric. Content_type unknown.

26 Borough: category token "borough" not in list; but we have state_abbr? But borough names like BRONX. No token. Use city? Actually "Borough" similar to borough. There's no borough token. So use unknown or free_text? Use free_text maybe, but better unknown. Since not defined.

27 X Coordinate (State Plane): integer numeric coordinate. content_type unknown.

28 Y Coordinate (State Plane): unknown.

29 Open Data Channel Type: category (PHONE, UNKNOWN etc.). Use category.

30 Park Facility Name: free_text or category? It's names of parks; could be free_text. Use free_text.

31 Park Borough: same as borough field; unknown.

32 Vehicle Type: category. content_type category.

33 Taxi Company Borough: same as borough; unknown.

34 Taxi Pick Up Location: free_text (location description). Use free_text.

35 Bridge Highway Name: category.

36 Bridge Highway Direction: category.

37 Road Ramp: category.

38 Bridge Highway Segment: category.

39 Latitude: float numeric coordinate. content_type unknown? But maybe latitude token exists. There is "latitude" in list. So use latitude.

40 Longitude: longitude token.

41 Location: string pair coordinates; could be free_text but format "(lat, lon)". Could use free_text. But maybe location field can be treated as free_text. Use free_text.

Now relationships:

Ordered: Created Date <= Closed Date? yes. Also Due Date may be after Created Date? Not necessarily but likely due date later than created. Might include ordered between Created Date and Closed Date, also maybe Resolution Action Updated Date between Closed Date and ??? But we don't know ordering.

But typical: Created Date <= Closed Date (or null). So relationship kind "ordered" with members ["Created Date", "Closed Date"], anchor "Created Date".

Also "Due Date" may be after Created Date; but not guaranteed. We can include due date maybe between created and closed? Might not always. But we could create ordered relationship: ["Created Date","Due Date","Closed Date"] if data indicates typical.

Let's check freq distribution for Due Date: earliest 04/08/2015 etc. Created dates start 01/24/2013 to 2020. So due date often after created but not always; but we can still list as ordered maybe with anchor "Created Date". But risk of incorrect assumption.

Better to only include guaranteed order: Created <= Closed, and maybe Due Date >= Created? We can't be sure. Might exclude due date.

Correlated: numeric fields like Latitude/Longitude correlated? Could say correlated group ["Latitude","Longitude"].

Also X Coordinate and Y coordinate correlated with latitude longitude? Possibly but not necessary.

Joint: e.g., City, Borough, Incident Zip? They are related but may vary across same city; but they likely correlate. But joint relationship meaning values only occur in fixed real-world combinations. Example: city+state+zip. Here city and borough maybe independent. Not sure.

We can create joint group ["City","Borough"]? City names correspond to borough; but there are multiple cities within a borough (NYC boroughs). So not joint.

Maybe "Agency" + "Agency Name" are jointly related: each agency has a name. But Agency field values like NYPD, HPD etc. Agency Name is the full name. Each unique mapping. So we can treat as joint relationship.

Also "Community Board" may map to borough? but not fixed.

So create joint group ["Agency","Agency Name"].

Also maybe "Vehicle Type" + "Taxi Company Borough"? Not joint.

Also "Bridge Highway Direction" and "Bridge Highway Name" might be jointly related; each direction depends on highway name. But we can skip.

Ordered: Created Date <= Closed Date.

Correlated: Latitude <-> Longitude. Also X Coordinate (State Plane) correlated with Y coordinate. And maybe latitude correlated with longitude, etc.

But rule: Do not put a field in more than one relationship. So we must decide groups.

Let's define:

- ordered: ["Created Date","Closed Date"] anchor "Created Date".

- joint: ["Agency","Agency Name"]

- correlated: ["Latitude","Longitude"], also maybe ["X Coordinate (State Plane)","Y Coordinate (State Plane)"]. But that would be two relationships each with 2 fields. That's okay but no field in more than one relationship. If we include both pairs, X coordinate and Y coordinate appear only once each. That's fine.

Also could correlate Latitude/Longitude with X/Y coordinates? That would involve more fields; but then those fields would appear twice (not allowed). So keep separate.

Now produce JSON object mapping each field name to label, description, content_type.

Need to ensure field names exactly as given in dataset list. They include spaces and parentheses. We'll match exactly: e.g., "X Coordinate (State Plane)" etc.

Also for content_type we must use tokens from allowed list with suffix if needed.

Let's craft descriptions:

Unique Key: unique identifier for each record; integer sequence.

Created Date: timestamp of complaint creation, includes date and time to the second in local timezone? Provide description.

Closed Date: timestamp when complaint was closed.

Agency: name abbreviation for agency that handled complaint.

Agency Name: full official name of agency.

Complaint Type: category of complaint e.g., Noise - Residential.

Descriptor: more detailed descriptor of issue.

Location Type: type of location where incident occurred (Residential, Street, etc.).

Incident Zip: five-digit ZIP code of incident location; may be missing.

Incident Address: street address of incident, raw text.

Street Name: name of main street involved.

Cross Street 1 / Cross Street 2: names of cross streets intersecting at incident location.

Intersection Street 1/2: names of streets forming intersection where incident occurred.

Address Type: classification of address type (ADDRESS, INTERSECTION, etc.).

City: city in which the incident took place.

Landmark: notable landmark near incident; free text.

Facility Type: type of facility involved (e.g., DSNY Garage).

Status: current status of complaint.

Due Date: deadline for action on complaint.

Resolution Description: narrative description of how complaint was resolved.

Resolution Action Updated Date: timestamp when resolution action was last updated.

Community Board: community board number assigned to the location.

BBL: Borough, Block, Lot identifier; numeric.

Borough: borough where incident occurred.

X Coordinate (State Plane): X coordinate in New York State Plane coordinate system.

Y Coordinate (State Plane): Y coordinate.

Open Data Channel Type: channel through which complaint was reported.

Park Facility Name: name of park facility involved, if any.

Park Borough: borough containing the park facility.

Vehicle Type: type of vehicle involved (Car Service etc.).

Taxi Company Borough: borough where taxi company is based.

Taxi Pick Up Location: location description for taxi pickup.

Bridge Highway Name: name of bridge or highway involved.

Bridge Highway Direction: direction of travel on bridge/highway.

Road Ramp: classification of road ramp.

Bridge Highway Segment: specific segment identifier of bridge/highway.

Latitude: geographic latitude coordinate.

Longitude: geographic longitude coordinate.

Location: combined latitude and longitude as a string pair.

Now produce JSON with fields.

Check that content_type values are allowed tokens. Let's list mapping:

Unique Key -> unknown
Created Date -> datetime:%m/%d/%Y %I:%M:%S %p
Closed Date -> same format
Agency -> category
Agency Name -> category
Complaint Type -> category
Descriptor -> category
Location Type -> category
Incident Zip -> zip_code
Incident Address -> street_address
Street Name -> street_name
Cross Street 1 -> street_name
Cross Street 2 -> street_name
Intersection Street 1 -> street_name
Intersection Street 2 -> street_name
Address Type -> category
City -> city
Landmark -> free_text (but free_text is allowed)
Facility Type -> category
Status -> category
Due Date -> datetime:%m/%d/%Y %I:%M:%S %p
Resolution Description -> free_text
Resolution Action Updated Date -> datetime:%m/%d/%Y %I:%M:%S %p
Community Board -> category
BBL -> unknown
Borough -> unknown (no borough token)
X Coordinate (State Plane) -> unknown
Y Coordinate (State Plane) -> unknown
Open Data Channel Type -> category
Park Facility Name -> free_text
Park Borough -> unknown
Vehicle Type -> category
Taxi Company Borough -> unknown
Taxi Pick Up Location -> free_text
Bridge Highway Name -> category
Bridge Highway Direction -> category
Road Ramp -> category
Bridge Highway Segment -> category
Latitude -> latitude
Longitude -> longitude
Location -> free_text

Now produce relationships array:

[
 {"kind":"ordered","members":["Created Date","Closed Date"],"anchor":"Created Date"},
 {"kind":"joint","members":["Agency","Agency Name"]},
 {"kind":"correlated","members":["Latitude","Longitude"]},
 {"kind":"correlated","members":["X Coordinate (State Plane)","Y Coordinate (State Plane)"]}
]

Check rule: do not place field in more than one relationship. All unique.

Now produce JSON object with fields as keys.

Make sure to escape quotes properly; but we output raw JSON.

Let's craft JSON:

{
  "Unique Key": {"label":"Unique Key", "description":"A unique integer identifier assigned to each record.", "content_type":"unknown"},
  ...
}

We need to include all 41 fields.

Let's produce final answer.

REFINE PASS REASONING:
We need to refine the dictionary with cross-field context.

Given 41 fields. Need to produce JSON mapping each field name string (e.g., "Unique Key") to object with label, description, content_type (unless unique_id; but first pass had unique_id for Unique Key. We must omit content_type for that). Also if we change content type, adjust accordingly.

We need to identify composite concepts: e.g., address fields: Incident Address, Street Name, Cross Streets, Intersection streets, Address Type etc. But maybe we keep separate but note relationships.

Also combine Latitude/Longitude into coordinate pair? But they are separate; description can mention that together form a coordinate. Also Location field is string representation of lat/lon pair; content_type probably free_text? Could be "free_text" or maybe "location". Since allowed tokens: free_text, unknown etc. Let's keep free_text.

Other composite: Agency + Agency Name (full name). Maybe rename label to "Agency Abbreviation" and "Agency Full Name". Keep description referencing each other.

Similarly, Community Board vs BBL: they are separate; but we can note relationship.

Also fields like Vehicle Type, Taxi Company Borough, Taxi Pick Up Location. etc.

We should also consider content_type improvements:

- Unique Key: unique_id omitted.
- Created Date / Closed Date / Due Date / Resolution Action Updated Date: date/datetime with appropriate format tokens. The first pass had "datetime:%m/%d/%Y %I:%M:%S %p" for Created and others. For Closed Date, min max show timezone offset? They have +00:00. The content_type should be datetime:%m/%d/%Y %H:%M:%S (since 24h?). But we keep same as first pass.

- Agency: category; maybe "category" remains.
- Agency Name: category.
- Complaint Type: category.
- Descriptor: free_text? Could remain free_text.
- Location Type: category.
- Incident Zip: zip_code. Already set? In first pass content_type is zip_code. Keep.
- Incident Address: street_address (should change from street_address? Actually first pass had street_address). Good.

- Street Name: street_name (first pass had street_name). Keep.

- Cross Street 1, Cross Street 2: street_name. Keep.

- Intersection Street 1/2: street_name. Keep.

- Address Type: category. Could be "category" but maybe "address_type"? Not in list. Keep category.

- City: city. Good.

- Landmark: free_text? Might remain free_text.

- Facility Type: category.

- Status: category.

- Due Date: datetime with format; keep same.

- Resolution Description: free_text.

- Resolution Action Updated Date: date? It had content_type "date:%m/%d/%Y". Actually first pass said content_type: "date:%m/%d/%Y". But it's timestamp but no time. So use date:%m/%d/%Y.

- Community Board: category.

- BBL: unknown (since format not recognized). Keep unknown.

- Borough: category? It had enumeration of borough names. Could be category or maybe state_abbr? No. Keep category.

- X Coordinate (State Plane): unknown? In first pass content_type is "unknown". But it's numeric integer; but we might keep unknown.

- Y Coordinate (State Plane): unknown.

- Open Data Channel Type: category.

- Park Facility Name: free_text.

- Park Borough: category.

- Vehicle Type: category.

- Taxi Company Borough: category.

- Taxi Pick Up Location: free_text.

- Bridge Highway Name: category.

- Bridge Highway Direction: category.

- Road Ramp: category.

- Bridge Highway Segment: category.

- Latitude: latitude. Good.

- Longitude: longitude.

- Location: free_text (maybe "free_text" or "location"? Not in allowed tokens, so free_text). Could maybe be "unknown" but better free_text.

Also we might adjust some content types like "Cross Street 1/2" to street_name (already), Intersection Street 1/2 also. Good.

Now refine labels: make more human-friendly.

Examples:

Unique Key -> "Record ID"

Created Date -> "Complaint Submission Date"

Closed Date -> "Complaint Closure Date"

Agency -> "Responding Agency Abbreviation"

Agency Name -> "Responding Agency Full Name"

Complaint Type -> "Primary Complaint Category"

Descriptor -> "Detailed Complaint Description"

Location Type -> "Incident Location Category"

Incident Zip -> "Incident ZIP Code"

Incident Address -> "Full Incident Street Address"

Street Name -> "Main Street Name"

Cross Street 1 -> "First Cross Street"

Cross Street 2 -> "Second Cross Street"

Intersection Street 1 -> "First Intersection Street"

Intersection Street 2 -> "Second Intersection Street"

Address Type -> "Address Classification"

City -> "Incident City"

Landmark -> "Nearby Landmark"

Facility Type -> "Related Facility Type"

Status -> "Complaint Status"

Due Date -> "Required Action Deadline"

Resolution Description -> "Resolution Narrative"

Resolution Action Updated Date -> "Last Resolution Update Date"

Community Board -> "Community Board Number"

BBL -> "Borough Block Lot Identifier"

Borough -> "Incident Borough"

X Coordinate (State Plane) -> "State Plane X Coordinate (Easting)"

Y Coordinate (State Plane) -> "State Plane Y Coordinate (Northing)"

Open Data Channel Type -> "Reporting Channel"

Park Facility Name -> "Park Facility Name"

Park Borough -> "Park Borough"

Vehicle Type -> "Vehicle Involved Type"

Taxi Company Borough -> "Taxi Company Headquarters Borough"

Taxi Pick Up Location -> "Taxi Pickup Location Description"

Bridge Highway Name -> "Bridge/Highway Name"

Bridge Highway Direction -> "Bridge/Highway Direction"

Road Ramp -> "Road Ramp Classification"

Bridge Highway Segment -> "Bridge/Highway Segment Identifier"

Latitude -> "Geographic Latitude"

Longitude -> "Geographic Longitude"

Location -> "Combined Latitude and Longitude Coordinates"

Now descriptions referencing cross-field relationships.

For example:

Incident Address: "Full street address of the incident location; combine with City, Borough, Incident Zip for full mailing address."

Street Name: "Primary street name involved in the incident; part of the incident address."

Cross Street 1/2: "Names of cross streets at the incident intersection (if applicable)."

Intersection Street 1/2: "Alternative names for intersecting streets (if different from cross streets)."

Address Type: "Classification indicating how the address is represented (e.g., ADDRESS, INTERSECTION, BLOCKFACE, LATLONG)."

Latitude and Longitude: "Geographic coordinates of incident location; together define a point on the map. Use Latitude and Longitude fields to construct precise location."

Location: "String representation of latitude and longitude pair in parentheses; equivalent to '(Latitude, Longitude)'."

Similarly for others.

Also mention that BBL is property identifier used by NYC property records; can be cross-referenced with community board.

Now content_type modifications:

- Unique Key: omit content_type.
- Created Date: keep datetime:%m/%d/%Y %I:%M:%S %p. Could we change to 24h? The original raw values show "2010-01-01T00:00:00+00:00" so it's ISO 8601, but first pass used "%m/%d/%Y %I:%M:%S %p". Maybe better use datetime:%Y-%m-%dT%H:%M:%SZ? But allowed token only "datetime:<fmt>" with chrono strftime. So we can use datetime:%Y-%m-%dT%H:%M:%SZ or datetime:%Y-%m-%dT%H:%M:%S%z? They had timezone +00:00. Format could be "%Y-%m-%dT%H:%M:%S%z". But the sample had "2010-01-01T00:00:00+00:00" which matches "%Y-%m-%dT%H:%M:%S%z". Let's use that.

But first pass used "%m/%d/%Y %I:%M:%S %p", maybe from some other representation. We can improve.

We should check each date field:

- Created Date values: "2010-01-01T00:00:00+00:00" so format is ISO 8601 with timezone offset +00:00. So we set content_type datetime:%Y-%m-%dT%H:%M:%S%z.

- Closed Date same format.

- Due Date: values like "04/08/2015 10:00:58 AM". That is "%m/%d/%Y %I:%M:%S %p" but with timezone? Not indicated. So keep datetime:%m/%d/%Y %I:%M:%S %p.

- Resolution Action Updated Date: values like "11/15/2010 12:00:00 AM" (date only). Format "%m/%d/%Y %H:%M:%S"? Actually it's date/time but no timezone. So datetime:%m/%d/%Y %I:%M:%S %p.

But the first pass had content_type "date:%m/%d/%Y" for Resolution Action Updated Date, but includes time. We'll correct to datetime.

- Also maybe we want to use "datetime:%m/%d/%Y %I:%M:%S %p".

Now for BBL: unknown; remains unknown.

X/Y coordinates are integers but represent State Plane coordinates; we can keep unknown or category? But allowed tokens include unknown. So leave unknown.

For Location field: free_text.

Also, maybe "Latitude" and "Longitude": content_type latitude/longitude already correct.

Now produce JSON mapping each field name to object with label, description, content_type (unless omitted). Ensure keys are exactly field names as strings.

Let's produce final output.

Check that we don't include any fields missing. Provide 41 entries.

Also ensure no trailing commas etc.

Let's craft the JSON.


## TOKEN USAGE

TokenUsage { prompt: 36761, completion: 8952, total: 45713, elapsed: 132749 }
---
# Description
**Description**

This public New York City 311 complaints dataset contains one million distinct incident records (unique identifiers ranging from 11,465,364 to 48,478,173) reported between January 2010 and December 2020. Each record links a complaint type—such as *Noise – Residential*, *HEAT/HOT WATER* or *Illegal Parking*—to the agency that responded, the incident location (address, ZIP code, latitude/longitude), and the current status of the case. The majority of complaints were filed in 2013–2014, with the most common agencies being NYPD (≈26 %) and HPD (≈26 %). Geospatial fields provide state‑plane coordinates as well as GPS latitudes and longitudes; about one quarter of these geographic values are missing. Most complaint records include a resolution narrative or action date, but 28 000 complaints lack closure dates while over 600 000 have no due‑date specified.

**Notable Characteristics**

- **Temporal concentration** – The bulk of complaints were submitted in the early 2010s; the *Created Date* distribution is slightly left‑skewed (skewness ≈ –0.09), with a long tail extending to late 2020.
- **Agency dominance** – NYPD and HPD together account for roughly half of all complaints, while the remaining agencies are represented sparsely; many agencies appear only in an “Other” bucket capturing numerous low‑frequency values.
- **High cardinality and “Other” categories** – Fields such as *Descriptor*, *Incident Address*, and *Community Board* contain thousands of distinct values, with large proportions (≈ 67 % for Descriptor, ≈ 75 % for Community Board) aggregated into a single “Other” category. This indicates that most complaints refer to unique or rarely repeated issues.
- **Missingness** – Significant gaps exist in date fields (*Closed Date* missing 28 k records; *Due Date* missing 647 k records) and geographic coordinates (≈ 25 % of latitude/longitude pairs are null). The high sparsity may bias analyses that rely on complete geospatial or temporal information.
- **Duplicate-like patterns** – Although the primary key is unique, many complaints share identical addresses or ZIP codes; duplicates in incident details can inflate counts for specific locations if not de‑duplicated at analysis time.
- **Potential privacy concerns** – The dataset contains free‑text fields (incident address, descriptor, resolution description) and precise latitude/longitude coordinates. Even with the unique key anonymized, these elements could facilitate reidentification of individuals or properties.
- **Outlier values** – Coordinate ranges are bounded by the New York State Plane system; however, the presence of “Other” buckets in geographic fields suggests occasional outliers or erroneous entries (e.g., coordinates outside the city limits).
- **Data quality heterogeneity** – Some categorical fields use a mix of ISO‑8601 dates and US month/day/year strings (*Due Date*), which may cause parsing issues. The *Agency Name* field contains both official agency names and miscellaneous entries, necessitating careful cleaning before aggregation.

> Generated by qsv v20.1.0 describegpt
Command line: target/debug/qsv describegpt --all --two-pass /tmp/NYC_311_SR_2010-2020-sample-1M.csv -o /tmp/nyc311-describegpt-everything.md --addl-cols --addl-cols-list everything --infer-content-type --fresh
Prompt file: Default v7.1.0
Model: openai/gpt-oss-20b
LLM API URL: http://localhost:1234/v1
Language: 
Timestamp: 2026-05-29T23:33:46.973860+00:00

WARNING: Description generated by an LLM and may contain inaccuracies. Verify before using!



## REASONING

We need to produce a Description of dataset based on summary statistics, frequency distribution, data dictionary. Avoid per-field detail. Provide 1-8 sentence description overall.

Then "Notable Characteristics" bullet list covering central tendency/spread, distribution shape, anomalies, patterns; outliers, missing values, duplicates, PII/PHI/PCI data; other data quality issues to be aware of.

Add Attribution placeholder.

Need to ensure no per-field output. Use dataset: NYC 311 complaints? It has 1M records (unique key). Data ranges from 2010-2020. Many fields with high cardinality, many nulls in some columns like Incident Zip 54k missing etc. Also lots of "Other" categories for high cardinality categorical fields. There are duplicates? Unique key indicates unique record ID; but other fields not unique.

Also note the dataset includes date/time fields; time zone UTC. The distribution: Created Date mostly 2013-2020, with many in early 2014 etc. Closed Date similar. Frequency distributions show top values for dates and agencies, etc. Many "Other" categories.

Also mention that some numeric coordinates have outliers? But coordinate ranges fairly typical.

Also mention that fields like "Latitude", "Longitude" have many nulls (~254k missing). Also some text fields contain PII (address, location), but dataset may be anonymized or contains addresses and lat/long. So potential privacy concerns. Many free_text fields have high cardinality; could contain sensitive info.

Also mention that data quality issues: high sparsity in many categorical fields (e.g., "Agency Name" has 0 nulls? but some top values). Some numeric columns may be missing or zero. There are duplicates in address etc.

Also mention distribution shape: e.g., "Created Date" skewness negative small -0.0857 indicates slight left-skew. Many dates clustered early 2014 etc.

Also mention "Agency" field has many categories but top two agencies account for ~52% of records.

Also mention that fields like "Community Board" have large proportion of 'Other' category (75% of cases). So dataset highly sparse in certain fields.

Also note that some numeric columns like "X Coordinate (State Plane)" have high skewness? Actually given mean etc. But outliers maybe extremes.

Also mention that the dataset uses US date format for Due Date; but other dates use ISO-8601.

Also mention that many fields have "Other" categories capturing many unique values, meaning data is not fully categorical.

Also mention missing values: e.g., Closed Date null 28k; Due Date null 647k. So large missingness.

Also mention duplicates? Unique key ensures no duplicates by ID but maybe duplicate complaints (same address etc.) Not sure.

Also note that the dataset includes fields like "Resolution Description" with free text, high cardinality and potential PII.

Let's produce description: 1-8 sentences about overall dataset. Use dataset size 1M records, timeframe 2010-2020, key categories. Provide mention of complaint types, agencies, geographic distribution etc.

Then bullet list for notable characteristics.

Also include attribution placeholder at bottom.

Make sure not to output summary statistics per field or frequency distribution per field. Just overall description.

Let's craft.
## TOKEN USAGE

TokenUsage { prompt: 33814, completion: 1346, total: 35160, elapsed: 52190 }
---
# Tags
- nyc_311_complaints  
- noise_and_environment  
- city_agencies_response  
- incident_location_coordinates  
- boroughs_of_nyc  
- complaint_resolution_status  
- urban_infrastructure_problems  
- temporal_trends  
- public_safety_issues  
- service_request_process  

Generated by qsv v20.1.0 describegpt
Command line: target/debug/qsv describegpt --all --two-pass /tmp/NYC_311_SR_2010-2020-sample-1M.csv -o /tmp/nyc311-describegpt-everything.md --addl-cols --addl-cols-list everything --infer-content-type --fresh
Prompt file: Default v7.1.0
Model: openai/gpt-oss-20b
LLM API URL: http://localhost:1234/v1
Language: 
Timestamp: 2026-05-29T23:35:02.242155+00:00

WARNING: Tags generated by an LLM and may contain inaccuracies. Verify before using!



## REASONING

We need to produce up to 10 unique tags, lower case, underscore separated words. Based on dataset content: NYC complaints, with agencies, types of complaint, location etc. The tags should be about contents not field names.

Consider tags like:

- nyc_311_complaints
- public_safety_issues
- noise_and_environment
- city_agencies_response
- incident_location_coordinates
- boroughs_of_nyc
- complaint_resolution_status
- temporal_trends (maybe)
- urban_infrastructure_problems

But we must choose descending importance based on summary stats, freq distribution, data dictionary. The most common fields: Complaint Type (noise residential etc). So "noise_and_environment" is important. Agency also high cardinality but many agencies. "city_agencies_response". Location type and address.

Also the dataset covers NYC 311 complaints; so "nyc_311_complaints" maybe top tag. Also "public_service_requests".

Let's list possible tags:

1. nyc_311_complaints
2. noise_and_environment
3. city_agencies_response
4. incident_location_coordinates
5. boroughs_of_nyc
6. complaint_resolution_status
7. urban_infrastructure_problems
8. temporal_trends
9. public_safety_issues
10. service_request_process

We need to ensure lower case and underscores. They should be unique, no field names.

Let's order by importance: top is nyc_311_complaints; noise_and_environment (common complaint type); city_agencies_response (agency variety). incident_location_coordinates (geographic data). boroughs_of_nyc (location). complaint_resolution_status (status). urban_infrastructure_problems (facility type etc). temporal_trends (created date, closed date). public_safety_issues (public safety agencies like NYPD). service_request_process (resolution description).

We should maybe remove one if overlapping: "public_safety_issues" overlaps with city_agencies_response. But we have 10 tags max; choose 10.

Let's produce markdown list:

- nyc_311_complaints
- noise_and_environment
- city_agencies_response
- incident_location_coordinates
- boroughs_of_nyc
- complaint_resolution_status
- urban_infrastructure_problems
- temporal_trends
- public_safety_issues
- service_request_process

Add attribution placeholder at bottom.

Let's produce output.
## TOKEN USAGE

TokenUsage { prompt: 33789, completion: 569, total: 34358, elapsed: 56980 }
---
# Dictionary
| Name | Type | Label | Description | Content Type | Min | Max | Cardinality | Enumeration | Null Count | is_ascii | sum | range | sort_order | sortiness | min_length | max_length | sum_length | avg_length | stddev_length | variance_length | cv_length | mean | sem | geometric_mean | harmonic_mean | stddev | variance | cv | n_negative | n_zero | n_positive | max_precision | sparsity | mad | lower_outer_fence | lower_inner_fence | q1 | q2_median | q3 | iqr | upper_inner_fence | upper_outer_fence | skewness | uniqueness_ratio | percentiles | Examples |
|------|------|-------|-------------|--------------|-----|-----|-------------|-------------|------------|----------|----------|----------|----------|----------|----------|----------|----------|----------|----------|----------|----------|----------|----------|----------|----------|----------|----------|----------|----------|----------|----------|----------|----------|----------|----------|----------|----------|----------|----------|----------|----------|----------|----------|----------|----------|----------|
| **Unique Key** | Integer | Record ID | A surrogate numeric identifier that uniquely distinguishes each complaint record in the dataset. | unique_id | 11465364 | 48478173 | 1,000,000 |  | 0 |  | 32687965858032 | 37012809 | Unsorted | 0.0018 |  |  |  |  |  |  |  | 32687965.858 | 9013.8953 | 31351729.249 | 29944311.4641 | 9013895.3358 | 81250309125279.2656 | 27.5756 | 0 | 0 | 1000000 |  | 0 | 7477037 | -19639208.5 | 2803282.25 | 25245773 | 32853358.5 | 40207433.5 | 14961660.5 | 62649924.25 | 85092415 | -0.0169 | 1 | 5: 18453724<br>10: 20062969<br>40: 29913180<br>60: 35829112<br>90: 45355115<br>95: 46937288 | <ALL_UNIQUE> |
| **Created Date** | DateTime | Complaint Creation Timestamp | The date and time when a complaint was originally filed, recorded with millisecond precision. | date:%m/%d/%Y | 01/01/2010 | 12/23/2020 | 841,014 |  | 0 |  |  | 4009.05962 | Unsorted | 0.0008 |  |  |  |  |  |  |  | 2015-11-10T18:05:22.615+00:00 | 1.15502 | 16709.46856 | 16668.78207 | 1155.01606 | 1334062.09198 | 6.8957 |  |  |  |  | 0 | 965.6694 | 1997-01-08T17:56:34.500+00:00 | 2005-02-08T08:58:19.500+00:00 | 2013-03-11T00:00:04.500+00:00 | 2016-02-12T13:16:49+00:00 | 2018-07-31T10:01:14.500+00:00 | 1968.41748 | 2026-08-31T01:02:59.500+00:00 | 2034-09-30T16:04:44.500+00:00 | -0.0857 | 0.841 | 5: 2010-08-10T00:00:00+00:00<br>10: 2011-03-15T11:35:08+00:00<br>40: 2015-01-21T10:24:00+00:00<br>60: 2017-02-25T20:10:00+00:00<br>90: 2020-01-10T08:26:00+00:00<br>95: 2020-07-21T18:32:11+00:00 | Other… [997,333]<br>01/24/2013 12:00:00 AM [347]<br>01/07/2014 12:00:00 AM [315]<br>01/08/2015 12:00:00 AM [283]<br>02/16/2015 12:00:00 AM [269] |
| **Closed Date** | DateTime | Complaint Closure Timestamp | The date and time when the complaint was officially closed or resolved. May be null if still open. | date:%m/%d/%Y | 01/01/1900 | 01/01/2100 | 688,837 |  | 28,619 |  |  | 73049 | Unsorted | 0.001 |  |  |  |  |  |  |  | 2015-11-14T10:16:16.743+00:00 | 1.33393 |  |  | 1314.70016 | 1728436.50813 | 7.8474 |  |  |  |  | 0.0286 | 954.61806 | 1997-04-12T11:33:24.500+00:00 | 2005-04-09T10:53:21.500+00:00 | 2013-04-06T10:13:18.500+00:00 | 2016-02-26T01:40:00+00:00 | 2018-08-04T09:46:36.500+00:00 | 1945.98146 | 2026-08-01T09:06:33.500+00:00 | 2034-07-29T08:26:30.500+00:00 | -0.0849 | 0.6888 | 5: 2010-08-23T11:35:00+00:00<br>10: 2011-04-01T00:00:00+00:00<br>40: 2015-02-09T00:00:00+00:00<br>60: 2017-03-08T14:06:03+00:00<br>90: 2020-01-07T00:02:00+00:00<br>95: 2020-07-20T12:00:00+00:00 | Other… [968,671]<br>(NULL)… [28,619]<br>11/15/2010 12:00:00 AM [384]<br>11/07/2012 12:00:00 AM [329]<br>12/09/2010 12:00:00 AM [267] |
| **Agency** | String | Agency Code | An abbreviated code indicating the city agency responsible for handling the complaint (e.g., NYPD, HPD). | category | 3-1-1 | TLC | 28 |  | 0 | false |  |  | Unsorted | 0.1729 | 3 | 42 | 3490582 | 3.4906 | 1.8975 | 3.6005 | 0.5436 |  |  |  |  |  |  |  |  |  |  |  | 0 |  |  |  |  |  |  |  |  |  |  | 0 |  | NYPD [265,116]<br>HPD [258,033]<br>DOT [132,462]<br>DSNY [81,606]<br>DEP [75,895] |
| **Agency Name** | String | Agency Full Name | The full name of the city agency that received or processed the complaint. | company_name | 3-1-1 | Valuation Policy | 553 |  | 0 | false |  |  | Unsorted | 0.1671 | 3 | 82 | 34840715 | 34.8407 | 10.5137 | 110.5379 | 0.3018 |  |  |  |  |  |  |  |  |  |  |  | 0 |  |  |  |  |  |  |  |  |  |  | 0.0006 |  | New York City Police Depa… [265,038]<br>Department of Housing Pre… [258,019]<br>Department of Transportat… [132,462]<br>Other… [103,974]<br>Department of Environment… [75,895] |
| **Complaint Type** | String | Primary Complaint Category | A high‑level category summarizing the nature of the complaint (e.g., Noise, Illegal Parking). | category | ../../WEB-INF/web.xml;x= | ZTESTINT | 287 |  | 0 | true |  |  | Unsorted | 0.0284 | 3 | 41 | 16475270 | 16.4753 | 6.8221 | 46.5406 | 0.4141 |  |  |  |  |  |  |  |  |  |  |  | 0 |  |  |  |  |  |  |  |  |  |  | 0.0003 |  | Other… [563,561]<br>Noise - Residential [89,439]<br>HEAT/HOT WATER [56,639]<br>Illegal Parking [45,032]<br>Blocked Driveway [42,356] |
| **Descriptor** | String | Detailed Complaint Subcategory | A more detailed description or subcategory of the complaint, refining the primary type. | category | 1 Missed Collection | unknown odor/taste in drinking water (QA6) | 1,392 |  | 3,001 | true |  |  | Unsorted | 0.0186 | 0 | 80 | 17426583 | 17.4266 | 10.4342 | 108.8723 | 0.5988 |  |  |  |  |  |  |  |  |  |  |  | 0.003 |  |  |  |  |  |  |  |  |  |  | 0.0014 |  | Other… [671,870]<br>Loud Music/Party [93,646]<br>ENTIRE BUILDING [36,885]<br>HEAT [35,088]<br>No Access [31,631] |
| **Location Type** | String | Physical Location Category | The type of physical location where the complaint occurred (e.g., Residential Building, Street). | category | 1-, 2- and 3- Family Home | Wooded Area | 162 |  | 239,131 | true |  |  | Unsorted | 0.187 | 0 | 36 | 12417750 | 12.4177 | 8.9759 | 80.5671 | 0.7228 |  |  |  |  |  |  |  |  |  |  |  | 0.2391 |  |  |  |  |  |  |  |  |  |  | 0.0002 |  | RESIDENTIAL BUILDING [255,562]<br>(NULL)… [239,131]<br>Street/Sidewalk [145,653]<br>Residential Building/Hous… [92,765]<br>Street [92,190] |
| **Incident Zip** | String | Incident ZIP Code | The five‑digit ZIP code corresponding to the incident location. | zip_code | * | XXXXX | 535 |  | 54,978 | true |  |  | Unsorted | 0.0085 | 0 | 10 | 4347871 | 4.3479 | 1.14 | 1.2996 | 0.2622 |  |  |  |  |  |  |  |  |  |  |  | 0.055 |  |  |  |  |  |  |  |  |  |  | 0.0005 |  | Other… [815,988]<br>(NULL)… [54,978]<br>11226 [17,114]<br>10467 [14,495]<br>11207 [12,872] |
| **Incident Address** | String | Full Incident Street Address | The full street address where the complaint was reported, including number and street name. It is part of the mailing address; combine with City, Incident Zip, and Street Name for a complete address. | street_address | * * | west 155 street and edgecombe avenue | 341,996 |  | 174,700 | true |  |  | Unsorted | -0.0005 | 0 | 55 | 14591947 | 14.5919 | 7.332 | 53.7589 | 0.5025 |  |  |  |  |  |  |  |  |  |  |  | 0.1747 |  |  |  |  |  |  |  |  |  |  | 0.342 |  | Other… [819,046]<br>(NULL)… [174,700]<br>655 EAST  230 STREET [1,538]<br>78-15 PARSONS BOULEVARD [694]<br>672 EAST  231 STREET [663] |
| **Street Name** | String | Primary Street Name | The main street name associated with the incident location; component of the full address. | street_name | * | wyckoff avenue | 14,837 |  | 174,720 | true |  |  | Unsorted | 0.0001 | 0 | 55 | 10888475 | 10.8885 | 5.7968 | 33.6032 | 0.5324 |  |  |  |  |  |  |  |  |  |  |  | 0.1747 |  |  |  |  |  |  |  |  |  |  | 0.0148 |  | Other… [784,684]<br>(NULL)… [174,720]<br>BROADWAY [9,702]<br>GRAND CONCOURSE [5,851]<br>OCEAN AVENUE [3,946] |
| **Cross Street 1** | String | First Cross Street | The first cross street that intersects or is adjacent to the incident location. Useful for locating the site on a map. | street_name | 1 AVE | mermaid | 16,238 |  | 320,401 | true |  |  | Unsorted | 0.0009 | 0 | 32 | 8355458 | 8.3555 | 6.6045 | 43.6193 | 0.7904 |  |  |  |  |  |  |  |  |  |  |  | 0.3204 |  |  |  |  |  |  |  |  |  |  | 0.0162 |  | Other… [619,743]<br>(NULL)… [320,401]<br>BEND [12,562]<br>BROADWAY [8,548]<br>3 AVENUE [6,154] |
| **Cross Street 2** | String | Second Cross Street | A second cross street near the incident location, if applicable. | street_name | 1 AVE | surf | 16,486 |  | 323,644 | true |  |  | Unsorted | 0.0016 | 0 | 35 | 8363431 | 8.3634 | 6.645 | 44.1554 | 0.7945 |  |  |  |  |  |  |  |  |  |  |  | 0.3236 |  |  |  |  |  |  |  |  |  |  | 0.0165 |  | Other… [623,363]<br>(NULL)… [323,644]<br>BEND [12,390]<br>BROADWAY [8,833]<br>DEAD END [5,626] |
| **Intersection Street 1** | String | First Intersection Street | One of the streets forming an intersection at the incident site. | street_name | 1 AVE | flatlands AVE | 11,237 |  | 767,422 | true |  |  | Unsorted | -0.0009 | 0 | 35 | 2949273 | 2.9493 | 5.6793 | 32.2544 | 1.9257 |  |  |  |  |  |  |  |  |  |  |  | 0.7674 |  |  |  |  |  |  |  |  |  |  | 0.0112 |  | (NULL)… [767,422]<br>Other… [214,544]<br>BROADWAY [3,761]<br>CARPENTER AVENUE [2,918]<br>BEND [2,009] |
| **Intersection Street 2** | String | Second Intersection Street | The other street forming an intersection at the incident site. | street_name | 1 AVE | glenwood RD | 11,674 |  | 767,709 | true |  |  | Unsorted | 0.003 | 0 | 33 | 2917798 | 2.9178 | 5.6363 | 31.768 | 1.9317 |  |  |  |  |  |  |  |  |  |  |  | 0.7677 |  |  |  |  |  |  |  |  |  |  | 0.0117 |  | (NULL)… [767,709]<br>Other… [215,667]<br>BROADWAY [3,462]<br>BEND [1,942]<br>2 AVENUE [1,690] |
| **Address Type** | String | Address Format Type | A classification of the address format used (e.g., ADDRESS, INTERSECTION, BLOCKFACE). | category | ADDRESS | PLACENAME | 6 |  | 125,802 | true |  |  | Unsorted | 0.6845 | 0 | 12 | 6832263 | 6.8323 | 3.0923 | 9.5623 | 0.4526 |  |  |  |  |  |  |  |  |  |  |  | 0.1258 |  |  |  |  |  |  |  |  |  |  | 0 |  | ADDRESS [710,380]<br>INTERSECTION [133,361]<br>(NULL)… [125,802]<br>BLOCKFACE [22,620]<br>LATLONG [7,421] |
| **City** | String | Incident City | The city in which the incident occurred. Typically New York City but may include boroughs. | city | * | YORKTOWN HEIGHTS | 382 |  | 61,963 | true |  |  | Unsorted | 0.1811 | 0 | 22 | 7721241 | 7.7212 | 3.2635 | 10.6505 | 0.4227 |  |  |  |  |  |  |  |  |  |  |  | 0.062 |  |  |  |  |  |  |  |  |  |  | 0.0004 |  | BROOKLYN [296,254]<br>NEW YORK [189,069]<br>BRONX [181,168]<br>Other… [163,936]<br>(NULL)… [61,963] |
| **Landmark** | String | Nearby Landmark | A notable landmark or reference point near the incident location, if specified. | free_text | 1 AVENUE | ZULETTE AVENUE | 5,915 |  | 912,779 | true |  |  | Unsorted | 0.0009 | 0 | 32 | 1165773 | 1.1658 | 3.8978 | 15.1925 | 3.3435 |  |  |  |  |  |  |  |  |  |  |  | 0.9128 |  |  |  |  |  |  |  |  |  |  | 0.0059 |  | (NULL)… [912,779]<br>Other… [80,165]<br>EAST  230 STREET [1,545]<br>EAST  231 STREET [1,291]<br>BROADWAY [1,148] |
| **Facility Type** | String | Public Facility Category | The type of public facility involved in the complaint (e.g., DSNY Garage, School District). | category | DSNY Garage | School District | 6 |  | 145,478 | true |  |  | Unsorted | 0.5941 | 0 | 15 | 3790876 | 3.7909 | 2.7562 | 7.5969 | 0.7271 |  |  |  |  |  |  |  |  |  |  |  | 0.1455 |  |  |  |  |  |  |  |  |  |  | 0 |  | N/A [628,279]<br>Precinct [193,259]<br>(NULL)… [145,478]<br>DSNY Garage [32,310]<br>School [617] |
| **Status** | String | Complaint Status | The current state of the complaint (e.g., Closed, Pending, Open). | category | Assigned | Unspecified | 10 | Assigned<br>Closed<br>Closed - Testing<br>Email Sent<br>In Progress<br>Open<br>Pending<br>Started<br>Unassigned<br>Unspecified | 0 | true |  |  | Unsorted | 0.9079 | 4 | 16 | 6048943 | 6.0489 | 0.5411 | 0.2928 | 0.0894 |  |  |  |  |  |  |  |  |  |  |  | 0 |  |  |  |  |  |  |  |  |  |  | 0 |  | Closed [952,522]<br>Pending [20,119]<br>Open [12,340]<br>In Progress [7,841]<br>Assigned [6,651] |
| **Due Date** | DateTime | Scheduled Resolution Deadline | The scheduled completion or resolution date for the complaint. | datetime:%m/%d/%Y %I:%M:%S %p | 01/02/1900 12:00:00 AM | 06/17/2021 04:34:13 PM | 345,077 |  | 647,794 |  |  | 44361.69043 | Unsorted | 0.0011 |  |  |  |  |  |  |  | 2015-05-30T02:54:49.998+00:00 | 1.74433 |  |  | 1035.204 | 1071647.31812 | 6.2418 |  |  |  |  | 0.6478 | 800.07713 | 1999-09-09T02:53:37+00:00 | 2006-06-14T19:09:32.500+00:00 | 2013-03-20T11:25:28+00:00 | 2015-10-03T01:27:48+00:00 | 2017-09-22T14:16:05+00:00 | 1647.11848 | 2024-06-28T06:32:00.500+00:00 | 2031-04-03T22:47:56+00:00 | -0.1251 | 0.3451 | 5: 2010-08-25T10:54:44+00:00<br>10: 2011-04-14T19:02:13+00:00<br>40: 2014-11-06T10:17:31+00:00<br>60: 2016-07-31T14:14:53+00:00<br>90: 2018-10-25T07:47:20+00:00<br>95: 2019-03-24T08:03:29+00:00 | (NULL)… [647,794]<br>Other… [350,746]<br>04/08/2015 10:00:58 AM [214]<br>05/02/2014 03:32:17 PM [183]<br>03/30/2018 10:10:39 AM [172] |
| **Resolution Description** | String | Resolution Narrative | A free‑text narrative describing how the complaint was resolved, including actions taken or outcomes. | free_text | A DOB violation was issued for failing to comply with an existing Stop Work Order. | Your request was submitted to the Department of Homeless Services. The City?s outreach team will assess the homeless individual and offer appropriate assistance within 2 hours. If you asked to know the outcome of your request, you will get a call within 2 hours. No further status will be available through the NYC 311 App, 311, or 311 Online. | 1,216 |  | 20,480 | false |  |  | Unsorted | 0.0319 | 0 | 934 | 153148305 | 153.1483 | 82.149 | 6748.4538 | 0.5364 |  |  |  |  |  |  |  |  |  |  |  | 0.0205 |  |  |  |  |  |  |  |  |  |  | 0.0012 |  | Other… [511,739]<br>The Police Department res… [91,408]<br>The Department of Housing… [72,962]<br>The Police Department res… [63,868]<br>Service Request status fo… [52,155] |
| **Resolution Action Updated Date** | DateTime | Last Resolution Update Timestamp | The date and time when the resolution details were last updated. | date:%m/%d/%Y | 12/31/2009 | 12/23/2020 | 690,314 |  | 15,072 |  |  | 4010.22308 | Unsorted | 0.001 |  |  |  |  |  |  |  | 2015-11-19T19:44:34.889+00:00 | 1.16204 | 16718.67594 | 16678.12298 | 1153.24922 | 1329983.75668 | 6.8814 |  |  |  |  | 0.0151 | 966.48803 | 1997-01-12T11:30:24+00:00 | 2005-02-14T12:31:15.750+00:00 | 2013-03-19T13:32:07.500+00:00 | 2016-02-22T22:38:30+00:00 | 2018-08-10T14:12:42+00:00 | 1970.02818 | 2026-09-12T15:13:33.750+00:00 | 2034-10-15T16:14:25.500+00:00 | -0.0867 | 0.6903 | 5: 2010-08-22T00:00:00+00:00<br>10: 2011-03-25T11:49:23+00:00<br>40: 2015-02-01T00:00:00+00:00<br>60: 2017-03-10T02:24:33+00:00<br>90: 2020-01-12T09:08:00+00:00<br>95: 2020-07-22T01:06:53+00:00 | Other… [982,148]<br>(NULL)… [15,072]<br>11/15/2010 12:00:00 AM [385]<br>11/07/2012 12:00:00 AM [336]<br>12/09/2010 12:00:00 AM [273] |
| **Community Board** | String | Neighborhood Community Board | The community board number or designation responsible for the neighborhood where the incident occurred. | category | 0 Unspecified | Unspecified STATEN ISLAND | 77 |  | 0 | true |  |  | Unsorted | 0.0193 | 8 | 25 | 11142863 | 11.1429 | 2.971 | 8.8269 | 0.2666 |  |  |  |  |  |  |  |  |  |  |  | 0 |  |  |  |  |  |  |  |  |  |  | 0.0001 |  | Other… [751,635]<br>0 Unspecified [49,878]<br>12 MANHATTAN [29,845]<br>12 QUEENS [23,570]<br>01 BROOKLYN [21,714] |
| **BBL** | String | Borough Block Lot (BBL) Identifier | A ten‑digit identifier representing borough, block, and lot numbers in NYC property records. | unknown | 0000000000 | 5080470043 | 268,383 |  | 243,046 | true |  |  | Unsorted | -0.0009 | 0 | 10 | 448540 | 0.4485 | 4.3011 | 18.4993 | 9.5891 |  |  |  |  |  |  |  |  |  |  |  | 0.243 |  |  |  |  |  |  |  |  |  |  | 0.2684 |  | Other… [750,668]<br>(NULL)… [243,046]<br>2048330028 [1,566]<br>4068290001 [696]<br>4015110001 [664] |
| **Borough** | String | Incident Borough | The New York City borough where the incident took place (e.g., Brooklyn, Queens). | category | BRONX | Unspecified | 6 | BRONX<br>BROOKLYN<br>MANHATTAN<br>QUEENS<br>STATEN ISLAND<br>Unspecified | 0 | true |  |  | Unsorted | 0.2155 | 5 | 13 | 7595025 | 7.595 | 2.0632 | 4.2568 | 0.2717 |  |  |  |  |  |  |  |  |  |  |  | 0 |  |  |  |  |  |  |  |  |  |  | 0 |  | BROOKLYN [296,081]<br>QUEENS [228,818]<br>MANHATTAN [195,488]<br>BRONX [180,142]<br>Unspecified [49,878] |
| **X Coordinate (State Plane)** | Integer | State Plane Easting (X) | The easting coordinate of the incident location expressed in State Plane units. Often paired with Y Coordinate for spatial analysis. | unknown | 913281 | 1067220 | 102,556 |  | 85,327 |  | 919555108413 | 153939 | Unsorted | -0.0004 |  |  |  |  |  |  |  | 1005337.5451 | 23.5391 | 1005083.7023 | 1004827.9356 | 22512.4528 | 506810531.5324 | 2.2393 | 0 | 0 | 914673 |  | 0.0853 | 12292 | 919661 | 956616.5 | 993572 | 1004546 | 1018209 | 24637 | 1055164.5 | 1092120 | 0.1091 | 0.1026 | 5: 964313<br>10: 984035<br>40: 999859<br>60: 1009147<br>90: 1034015<br>95: 1043903 | Other… [908,535]<br>(NULL)… [85,327]<br>1022911 [1,568]<br>1037000 [701]<br>1023174 [675] |
| **Y Coordinate (State Plane)** | Integer | State Plane Northing (Y) | The northing coordinate of the incident location expressed in State Plane units. Often paired with X Coordinate for spatial analysis. | unknown | 121152 | 271876 | 116,092 |  | 85,327 |  | 188099299101 | 150724 | Unsorted | 0 |  |  |  |  |  |  |  | 205646.4978 | 33.1699 | 203166.0871 | 200659.7012 | 31723.1985 | 1006361322.6747 | 15.4261 | 0 | 0 | 914673 |  | 0.0853 | 24236 | 24257 | 103334 | 182411 | 202514 | 235129 | 52718 | 314206 | 393283 | 0.2373 | 0.1161 | 5: 156639<br>10: 164744<br>40: 193463<br>60: 212470<br>90: 250365<br>95: 256054 | Other… [908,538]<br>(NULL)… [85,327]<br>264242 [1,566]<br>202363 [706]<br>211606 [665] |
| **Open Data Channel Type** | String | Submission Medium | The medium through which the complaint was submitted (e.g., PHONE, ONLINE). | category | MOBILE | UNKNOWN | 5 | MOBILE<br>ONLINE<br>OTHER<br>PHONE<br>UNKNOWN | 0 | true |  |  | Unsorted | 0.3379 | 5 | 7 | 5718030 | 5.718 | 0.8144 | 0.6633 | 0.1424 |  |  |  |  |  |  |  |  |  |  |  | 0 |  |  |  |  |  |  |  |  |  |  | 0 |  | PHONE [497,606]<br>UNKNOWN [230,402]<br>ONLINE [177,334]<br>MOBILE [79,892]<br>OTHER [14,766] |
| **Park Facility Name** | String | Park or Recreational Facility Name | The name of a park or recreational facility involved in the complaint. | free_text | "Uncle" Vito F. Maranzano Glendale Playground | Zimmerman Playground | 1,889 |  | 0 | true |  |  | Unsorted | 0.9863 | 3 | 82 | 11072428 | 11.0724 | 1.2391 | 1.5353 | 0.1119 |  |  |  |  |  |  |  |  |  |  |  | 0 |  |  |  |  |  |  |  |  |  |  | 0.0019 |  | Unspecified [993,141]<br>Other… [5,964]<br>Central Park [261]<br>Riverside Park [136]<br>Prospect Park [129] |
| **Park Borough** | String | Park Facility Borough | The borough where the park facility is located. | category | BRONX | Unspecified | 6 | BRONX<br>BROOKLYN<br>MANHATTAN<br>QUEENS<br>STATEN ISLAND<br>Unspecified | 0 | true |  |  | Unsorted | 0.2155 | 5 | 13 | 7595025 | 7.595 | 2.0632 | 4.2568 | 0.2717 |  |  |  |  |  |  |  |  |  |  |  | 0 |  |  |  |  |  |  |  |  |  |  | 0 |  | BROOKLYN [296,081]<br>QUEENS [228,818]<br>MANHATTAN [195,488]<br>BRONX [180,142]<br>Unspecified [49,878] |
| **Vehicle Type** | String | Vehicle Category | The classification of vehicle related to the complaint (e.g., Car Service, Green Taxi). | category | Ambulette / Paratransit | Green Taxi | 5 |  | 999,652 | true |  |  | Unsorted | 0.8329 | 0 | 23 | 4066 | 0.0041 | 0.2293 | 0.0526 | 56.4051 |  |  |  |  |  |  |  |  |  |  |  | 0.9997 |  |  |  |  |  |  |  |  |  |  | 0 |  | (NULL)… [999,652]<br>Car Service [317]<br>Ambulette / Paratransit [19]<br>Commuter Van [11]<br>Green Taxi [1] |
| **Taxi Company Borough** | String | Taxi Company Operating Borough | The borough where the taxi company operates. | category | BRONX | Staten Island | 11 |  | 999,156 | true |  |  | Unsorted | 0.1839 | 0 | 13 | 6313 | 0.0063 | 0.2259 | 0.051 | 35.783 |  |  |  |  |  |  |  |  |  |  |  | 0.9992 |  |  |  |  |  |  |  |  |  |  | 0 |  | (NULL)… [999,156]<br>BROOKLYN [207]<br>QUEENS [194]<br>MANHATTAN [171]<br>BRONX [127] |
| **Taxi Pick Up Location** | String | Taxi Pick‑Up Description | A description of the location from which a taxi was picked up, often including an intersection or landmark. | free_text | 1 5 AVENUE MANHATTAN | YORK AVENUE AND EAST 70 STREET | 1,903 |  | 992,129 | true |  |  | Unsorted | 0.2854 | 0 | 60 | 135661 | 0.1357 | 2.1518 | 4.6304 | 15.8618 |  |  |  |  |  |  |  |  |  |  |  | 0.9921 |  |  |  |  |  |  |  |  |  |  | 0.0019 |  | (NULL)… [992,129]<br>Other [4,091]<br>Other… [2,006]<br>JFK Airport [562]<br>Intersection [486] |
| **Bridge Highway Name** | String | Bridge/Highway Name | The name of a bridge or highway associated with the complaint (e.g., Belt Pkwy, Long Island Expwy). | category | 145th St. Br - Lenox Ave | Willis Ave Br - 125th St/1st Ave | 68 |  | 997,711 | true |  |  | Unsorted | 0.0227 | 0 | 42 | 36974 | 0.037 | 0.8221 | 0.6759 | 22.2353 |  |  |  |  |  |  |  |  |  |  |  | 0.9977 |  |  |  |  |  |  |  |  |  |  | 0.0001 |  | (NULL)… [997,711]<br>Other… [779]<br>Belt Pkwy [276]<br>BQE/Gowanus Expwy [254]<br>Grand Central Pkwy [186] |
| **Bridge Highway Direction** | String | Bridge/Highway Traffic Direction | The cardinal direction in which traffic flows on the bridge or highway. | category | Bronx Bound | Westbound/To Goethals Br | 50 |  | 997,691 | true |  |  | Unsorted | 0.0338 | 0 | 33 | 44089 | 0.0441 | 0.9533 | 0.9089 | 21.6233 |  |  |  |  |  |  |  |  |  |  |  | 0.9977 |  |  |  |  |  |  |  |  |  |  | 0.0001 |  | (NULL)… [997,691]<br>Other… [987]<br>East/Long Island Bound [210]<br>North/Bronx Bound [208]<br>East/Queens Bound [197] |
| **Road Ramp** | String | Ramp Presence Indicator | Indicates whether a ramp was involved, and if so its type (e.g., Roadway, N/A). | category | N/A | Roadway | 4 |  | 997,693 | true |  |  | Unsorted | 0.6245 | 0 | 7 | 14400 | 0.0144 | 0.3069 | 0.0942 | 21.3118 |  |  |  |  |  |  |  |  |  |  |  | 0.9977 |  |  |  |  |  |  |  |  |  |  | 0 |  | (NULL)… [997,693]<br>Roadway [1,731]<br>Ramp [555]<br>N/A [21] |
| **Bridge Highway Segment** | String | Bridge/Highway Segment Identifier | A descriptive identifier for a specific segment of a bridge or highway. | free_text | 1-1-1265963747 | Wythe Ave/Kent Ave (Exit 31) | 937 |  | 997,556 | true |  |  | Unsorted | -0.007 | 0 | 100 | 110781 | 0.1108 | 2.5166 | 6.3334 | 22.7171 |  |  |  |  |  |  |  |  |  |  |  | 0.9976 |  |  |  |  |  |  |  |  |  |  | 0.0009 |  | (NULL)… [997,556]<br>Other… [2,144]<br>Ramp [92]<br>Roadway [54]<br>Clove Rd/Richmond Rd (Exi… [23] |
| **Latitude** | Float | Geographic Latitude | The geographic latitude coordinate of the incident location, expressed in decimal degrees. | latitude | 40.1123853 | 40.9128688 | 353,694 |  | 254,695 |  | 30355391.7604 | 0.8005 | Unsorted | -0.001 |  |  |  |  |  |  |  | 40.7288 | 0.0001 | 40.7287 | 40.7286 | 0.0893 | 0.008 | 0.2193 | 0 | 0 | 745305 | 15 | 0.2547 | 0.0632 | 40.2615 | 40.4646 | 40.6677 | 40.7222 | 40.8031 | 0.1354 | 41.0062 | 41.2094 | 0.1957 | 0.3537 | 5: 40.5955<br>10: 40.6175<br>40: 40.6986<br>60: 40.748<br>90: 40.8521<br>95: 40.8684 | Other… [739,329]<br>(NULL)… [254,695]<br>40.89187241649303 [1,538]<br>40.1123853 [1,153]<br>40.89238451539139 [663] |
| **Longitude** | Float | Geographic Longitude | The geographic longitude coordinate of the incident location, expressed in decimal degrees. | longitude | -77.5195844 | -73.7005968 | 353,996 |  | 254,695 |  | -55100392.9499 | 3.819 | Unsorted | -0.0008 |  |  |  |  |  |  |  | -73.93 | 0.0002 |  |  | 0.1635 | 0.0267 | -0.2212 | 745305 | 0 | 0 | 14 | 0.2547 | 0.0468 | -74.2533 | -74.1119 | -73.9705 | -73.9279 | -73.8763 | 0.0943 | -73.7349 | -73.5935 | 0.0964 | 0.354 | 5: -74.0787<br>10: -74.0022<br>40: -73.9454<br>60: -73.9106<br>90: -73.8191<br>95: -73.7839 | Other… [739,329]<br>(NULL)… [254,695]<br>-73.86016845296459 [1,538]<br>-77.5195844 [1,153]<br>-73.8592161325675 [663] |
| **Location** | String | Location (Lat/Lon) | A textual representation of the latitude and longitude pair for the incident site. | free_text | (40.1123853, -77.5195844) | (40.9128688, -73.9024731) | 375,772 |  | 254,695 | true |  |  | Unsorted | -0.001 | 0 | 40 | 19709659 | 19.7097 | 12.1918 | 148.6401 | 0.6186 |  |  |  |  |  |  |  |  |  |  |  | 0.2547 |  |  |  |  |  |  |  |  |  |  | 0.3758 |  | Other… [739,329]<br>(NULL)… [254,695]<br>(40.89187241649303, -73.8… [1,538]<br>(40.1123853, -77.5195844) [1,153]<br>(40.89238451539139, -73.8… [663] |

*Attribution: Generated by qsv v20.1.0 describegpt
Command line: target/debug/qsv describegpt --all --two-pass /tmp/NYC_311_SR_2010-2020-sample-1M.csv -o /tmp/nyc311-describegpt-everything.md --addl-cols --addl-cols-list everything --infer-content-type --fresh
Prompt file: Default v7.1.0
Model: openai/gpt-oss-20b
LLM API URL: http://localhost:1234/v1
Language: 
Timestamp: 2026-05-29T23:39:27.173168+00:00

WARNING: Label and Description generated by an LLM and may contain inaccuracies. Verify before using!
*

## REASONING

FIRST PASS REASONING:
We need to generate Labels, Descriptions and Content Types for all 41 fields. Use summary stats and frequency distribution.

Need content_type tokens: first_name, last_name, full_name, username, password, email, phone, street_address, street_name, building_number, secondary_address, city, state, state_abbr, zip_code, country, country_code, latitude, longitude, time_zone, company_name, industry, job_title, profession, unique_id, uuid, credit_card, currency_code, isbn, ip_address, ipv6_address, mac_address, url, user_agent, file_name, file_path, mime_type, color_hex, license_plate, date, datetime, time, duration, category, lorem_word, lorem_sentence, lorem_paragraph, free_text, unknown. plus optional suffix for date/datetime and duration.

We must consider each field.

Let's list fields:

1 Unique Key
2 Created Date
3 Closed Date
4 Agency
5 Agency Name
6 Complaint Type
7 Descriptor
8 Location Type
9 Incident Zip
10 Incident Address
11 Street Name
12 Cross Street 1
13 Cross Street 2
14 Intersection Street 1
15 Intersection Street 2
16 Address Type
17 City
18 Landmark
19 Facility Type
20 Status
21 Due Date
22 Resolution Description
23 Resolution Action Updated Date
24 Community Board
25 BBL
26 Borough
27 X Coordinate (State Plane)
28 Y Coordinate (State Plane)
29 Open Data Channel Type
30 Park Facility Name
31 Park Borough
32 Vehicle Type
33 Taxi Company Borough
34 Taxi Pick Up Location
35 Bridge Highway Name
36 Bridge Highway Direction
37 Road Ramp
38 Bridge Highway Segment
39 Latitude
40 Longitude
41 Location

We need label, description and content_type.

Use summary stats for type. Many types are String, DateTime etc. For numeric fields: Unique Key (Integer), X Coordinate, Y Coordinate maybe Integer or Float? Summary says X Coordinate is Integer, Y coordinate integer; but they might represent coordinates in State Plane units (US survey feet). Should be integer numeric - use unknown.

Latitude/Longitude Float -> content_type latitude/longitude.

Location string of latlon pair -> free_text? Could consider as "latlon" not allowed token. Maybe treat as free_text or location coordinate? Not in list. Use free_text.

Now content type decisions:

Unique Key: cardinality equals 1,000,000; uniqueness ratio 1. So unique_id maybe but deterministic override only if cardinality equals row count and field is surrogate key. This qualifies for unique_id token. But spec says do not use "unique_id" yourself - it will be replaced by qsv if true. We should pick the most specific other token that fits, e.g., "uuid" or "isbn". But it's numeric sequence; maybe best to use "unknown" or "uuid"? The instructions: "If no token fits, use unknown." For unique surrogate key numeric, we can choose "unknown" or "unique_id"? We cannot use unique_id. So choose "unknown".

But maybe "unknown" is okay.

Created Date: type DateTime, format appears as mm/dd/yyyy hh:mm:ss AM/PM? Frequency shows "01/24/2013 12:00:00 AM". That is "%m/%d/%Y %I:%M:%S %p". So content_type datetime:%m/%d/%Y %I:%M:%S %p.

Closed Date same format. Due Date also same.

Agency: string; could be category of agency; use category token.

Agency Name: company_name? It's name of agency. Use company_name.

Complaint Type: category.

Descriptor: free_text or category? descriptor describes complaint details, may be free text but frequent categories like Loud Music/Party etc. Could treat as category. But many descriptors include "Other (1,381)" - typical. Might use category. However some are free text like "unknown odor/taste in drinking water". That is more free_text. But overall it's a descriptive label of complaint; we can set content_type category.

Location Type: category.

Incident Zip: zip_code.

Incident Address: street_address? It's address string. Use street_address.

Street Name: street_name.

Cross Street 1, Cross Street 2: street_name each.

Intersection Street 1/2: street_name each.

Address Type: category? Could be "ADDRESS", "INTERSECTION" etc; use category.

City: city.

Landmark: free_text? It's a landmark name. Use free_text or category. Probably category. But may include many names; but treat as category.

Facility Type: category.

Status: category.

Due Date already handled.

Resolution Description: free_text.

Resolution Action Updated Date: datetime same format.

Community Board: category (board numbers). Use category.

BBL: BBL stands for Borough Block Lot, numeric 10-digit. Could be unique_id? But not all unique; many duplicates. So use unknown.

Borough: state_abbr? Actually borough names; use category or state_abbr? It's borough name. Use category.

X Coordinate (State Plane): integer coordinate; could be numeric; unknown.

Y Coordinate (State Plane): unknown.

Open Data Channel Type: category.

Park Facility Name: free_text or category. But many unique names; treat as free_text.

Park Borough: category.

Vehicle Type: category.

Taxi Company Borough: category.

Taxi Pick Up Location: free_text? Many are location strings, maybe category of pick up place. Use category? Might be free_text but use category.

Bridge Highway Name: category.

Bridge Highway Direction: category.

Road Ramp: category.

Bridge Highway Segment: free_text? Many names; treat as free_text or category. But we can set category.

Latitude: latitude token.

Longitude: longitude token.

Location: string of lat lon pair; maybe free_text.

Now produce JSON with each field name key and object.

Also relationships:

- ordered: Created Date <= Closed Date? The summary indicates min closed > min created? Could assume monotonic. But dataset may have cases where closed before created? Not sure. We'll set relationship kind "ordered" members ["Created Date","Closed Date"], anchor "Created Date". Also maybe Due Date after Created? Could be. But not necessary.

- joint: city, borough, zip, incident address? We could define joint group for Incident Zip + City + Borough? The dataset includes those fields. They may correlate. So relationship kind joint members ["Incident Zip","City","Borough"] anchor none. Or maybe "Location Type" with Address Type? Not sure.

- correlated: numeric fields like X Coordinate, Y Coordinate, Latitude, Longitude; they are correlated coordinates. But we can group them as correlated.

Let's define:

{
  "kind":"ordered",
  "members":["Created Date","Closed Date"],
  "anchor":"Created Date"
}

Also maybe Created Date <= Due Date? Not certain but could be.

- joint: ["Incident Zip","City","Borough"] maybe also "Community Board" with Borough. Could choose one group.

Let's include joint for Incident Zip, City, Borough.

- correlated: ["Latitude","Longitude","X Coordinate (State Plane)","Y Coordinate (State Plane)"]

Now produce JSON.

Need to ensure field names match exactly as given. For content_type tokens we must not invent tokens. Provide suffixes where needed.

Check date/datetime formatting:

Created Date frequency sample "01/24/2013 12:00:00 AM" => format "%m/%d/%Y %I:%M:%S %p". Use that.

Closed Date same.

Due Date same.

Resolution Action Updated Date same.

Now produce JSON.

Let's craft description for each field.

I'll write concise but descriptive.

We should not exceed token limit. Provide full description maybe up to 2-3 sentences.

Proceed.



REFINE PASS REASONING:
We need to produce a refined data dictionary JSON mapping each field name to label, description, content_type (unless unique_id). Use cross-field context. We have 41 fields.

Need to group components: mailing address? There are Incident Address, Street Name, Cross Street1/2, Intersection Street1/2, etc. But there is also separate Incident Zip, City. So we can label each component accordingly. Also maybe combine X Coordinate and Y coordinate as State Plane coordinates. And Latitude/Longitude.

Also "Open Data Channel Type" remains category.

"Vehicle Type" and "Taxi Company Borough" are related but keep separate.

"Location" (Lat/Lon string) could be free_text or location? Might use free_text, but maybe label as "Location (Lat/Lon)" already. Keep content_type free_text.

Unique Key: remove content_type field entirely.

Also maybe change content_type for "Latitude" to latitude; correct existing? Already set as latitude. Good.

Longitude -> longitude.

Incident Address: should be street_address.

Street Name: street_name.

Cross Street1,2: street_name each.

Intersection Street1,2: street_name each.

Address Type: category remains.

City: city.

Landmark: free_text (maybe location descriptor). Keep free_text.

Facility Type: category.

Status: category.

Due Date, Created Date, Closed Date, Resolution Action Updated Date: date/datetime tokens with correct format. Provided first-pass had datetime:%m/%d/%Y %I:%M:%S %p for Created Date. For Closed Date also same. Due Date has datetime:%m/%d/%Y %I:%M:%S %p but value examples show 04/08/2015 10:00:58 AM etc. That format is m/d/Y h:i:s A. So keep as datetime:%m/%d/%Y %I:%M:%S %p.

Resolution Action Updated Date has date:%m/%d/%Y (but examples show "11/15/2010 12:00:00 AM" which includes time, but content_type was date:%m/%d/%Y? Actually first-pass shows date:%m/%d/%Y. That seems wrong; should be datetime:%m/%d/%Y %I:%M:%S %p. We'll correct to datetime with format.

Open Data Channel Type: category stays.

Park Facility Name, Park Borough similar to other names: free_text and category respectively.

Vehicle Type: category.

Taxi Company Borough: category.

Taxi Pick Up Location: free_text.

Bridge Highway Name: category.

Bridge Highway Direction: category.

Road Ramp: category.

Bridge Highway Segment: free_text.

Latitude/Longitude already correct.

Location (Lat/Lon): free_text.

Now we need to output JSON mapping each field name string to object with label, description, content_type if not unique_id. For Unique Key omit content_type key entirely.

Also refine labels: e.g., "Unique Key" stays maybe "Record ID". But keep same? Should be more human-friendly; can change. Provide better descriptions referencing cross-fields.

We need to produce JSON shape exactly like first pass: {"fields": [ ... ]} or the new format? The instruction says return results in SAME JSON shape as the first pass:

{
  "field_name_1": {
    "label": "...",
    "description": "...",
    "content_type": "..."
  },
  ...
}

So top-level keys are field names, not array. So produce object mapping.

Let's prepare for each of 41 fields.

We'll include description referencing cross-fields when relevant: e.g., Incident Address component of mailing address; combine with City, Incident Zip, Street Name etc.

Also we may note that "Address Type" indicates format type.

We should also adjust content_type for "Cross Street1/2" and "Intersection Street1/2": street_name each.

For "X Coordinate (State Plane)" and "Y Coordinate (State Plane)": maybe content_type unknown? They are integer coordinates; could use unknown. But we can keep as unknown or category? In first pass they had unknown type. We'll leave as unknown.

Similarly, "Borough" category.

"BBL": unknown string; keep unknown.

"Location Type": category.

"Facility Type": category.

"Community Board": category.

"Taxi Pick Up Location": free_text.

Now produce JSON.

Check to include content_type for all except Unique Key.

Let's craft each entry:

Unique Key:
label: "Record ID"
description: "A surrogate numeric identifier that uniquely distinguishes each complaint record in the dataset."
omit content_type.

Created Date:
label: "Complaint Creation Timestamp"
description: "The date and time when a complaint was originally filed, recorded with millisecond precision."
content_type: datetime:%m/%d/%Y %I:%M:%S %p

Closed Date:
label: "Complaint Closure Timestamp"
description: "The date and time when the complaint was officially closed or resolved. May be null if still open."
content_type: datetime:%m/%d/%Y %I:%M:%S %p

Agency:
label: "Agency Code"
description: "An abbreviated code indicating the city agency responsible for handling the complaint (e.g., NYPD, HPD)."

Agency Name:
label: "Agency Full Name"
description: "The full name of the city agency that received or processed the complaint."

Complaint Type:
label: "Primary Complaint Category"
description: "A high‑level category summarizing the nature of the complaint (e.g., Noise, Illegal Parking)."

Descriptor:
label: "Detailed Complaint Subcategory"
description: "A more detailed description or subcategory of the complaint, refining the primary type."

Location Type:
label: "Physical Location Category"
description: "The type of physical location where the complaint occurred (e.g., Residential Building, Street)."

Incident Zip:
label: "Incident ZIP Code"
description: "The five‑digit ZIP code corresponding to the incident location."

Incident Address:
label: "Full Incident Street Address"
description: "The full street address where the complaint was reported, including number and street name. It is part of the mailing address; combine with City, Incident Zip, and Street Name for a complete address."

Street Name:
label: "Primary Street Name"
description: "The main street name associated with the incident location; component of the full address."

Cross Street 1:
label: "First Cross Street"
description: "The first cross street that intersects or is adjacent to the incident location. Useful for locating the site on a map."

Cross Street 2:
label: "Second Cross Street"
description: "A second cross street near the incident location, if applicable."

Intersection Street 1:
label: "First Intersection Street"
description: "One of the streets forming an intersection at the incident site."

Intersection Street 2:
label: "Second Intersection Street"
description: "The other street forming an intersection at the incident site."

Address Type:
label: "Address Format Type"
description: "A classification of the address format used (e.g., ADDRESS, INTERSECTION, BLOCKFACE)."

City:
label: "Incident City"
description: "The city in which the incident occurred. Typically New York City but may include boroughs."

Landmark:
label: "Nearby Landmark"
description: "A notable landmark or reference point near the incident location, if specified."

Facility Type:
label: "Public Facility Category"
description: "The type of public facility involved in the complaint (e.g., DSNY Garage, School District)."

Status:
label: "Complaint Status"
description: "The current state of the complaint (e.g., Closed, Pending, Open)."

Due Date:
label: "Scheduled Resolution Deadline"
description: "The scheduled completion or resolution date for the complaint."

Resolution Description:
label: "Resolution Narrative"
description: "A free‑text narrative describing how the complaint was resolved, including actions taken or outcomes."

Resolution Action Updated Date:
label: "Last Resolution Update Timestamp"
description: "The date and time when the resolution details were last updated."

Community Board:
label: "Neighborhood Community Board"
description: "The community board number or designation responsible for the neighborhood where the incident occurred."

BBL:
label: "Borough Block Lot (BBL) Identifier"
description: "A ten‑digit identifier representing borough, block, and lot numbers in NYC property records."

Borough:
label: "Incident Borough"
description: "The New York City borough where the incident took place (e.g., Brooklyn, Queens)."

X Coordinate (State Plane):
label: "State Plane Easting (X)"
description: "The easting coordinate of the incident location expressed in State Plane units. Often paired with Y Coordinate for spatial analysis."

Y Coordinate (State Plane):
label: "State Plane Northing (Y)"
description: "The northing coordinate of the incident location expressed in State Plane units. Often paired with X Coordinate for spatial analysis."

Open Data Channel Type:
label: "Submission Medium"
description: "The medium through which the complaint was submitted (e.g., PHONE, ONLINE)."

Park Facility Name:
label: "Park or Recreational Facility Name"
description: "The name of a park or recreational facility involved in the complaint."

Park Borough:
label: "Park Facility Borough"
description: "The borough where the park facility is located."

Vehicle Type:
label: "Vehicle Category"
description: "The classification of vehicle related to the complaint (e.g., Car Service, Green Taxi)."

Taxi Company Borough:
label: "Taxi Company Operating Borough"
description: "The borough where the taxi company operates."

Taxi Pick Up Location:
label: "Taxi Pick‑Up Description"
description: "A description of the location from which a taxi was picked up, often including an intersection or landmark."

Bridge Highway Name:
label: "Bridge/Highway Name"
description: "The name of a bridge or highway associated with the complaint (e.g., Belt Pkwy, Long Island Expwy)."

Bridge Highway Direction:
label: "Bridge/Highway Traffic Direction"
description: "The cardinal direction in which traffic flows on the bridge or highway."

Road Ramp:
label: "Ramp Presence Indicator"
description: "Indicates whether a ramp was involved, and if so its type (e.g., Roadway, N/A)."

Bridge Highway Segment:
label: "Bridge/Highway Segment Identifier"
description: "A descriptive identifier for a specific segment of a bridge or highway."

Latitude:
label: "Geographic Latitude"
description: "The geographic latitude coordinate of the incident location, expressed in decimal degrees."
content_type: latitude

Longitude:
label: "Geographic Longitude"
description: "The geographic longitude coordinate of the incident location, expressed in decimal degrees."
content_type: longitude

Location (Lat/Lon):
label: "Coordinate Pair String"
description: "A textual representation of the latitude and longitude pair for the incident site."
content_type: free_text

Now content_type assignments:

Unique Key omitted.

Created Date: datetime:%m/%d/%Y %I:%M:%S %p

Closed Date: same

Agency, Agency Name: category? Actually first-pass had category for Agency; but maybe "agency_code" not in vocabulary. So keep category.

Same for Agency Name: company_name.

Complaint Type: category

Descriptor: free_text or category? It is subcategory of complaint type; could be category. Keep category.

Location Type: category

Incident Zip: zip_code

Incident Address: street_address

Street Name: street_name

Cross Street 1/2: street_name each

Intersection Street 1/2: street_name each

Address Type: category

City: city

Landmark: free_text (maybe location). Keep free_text.

Facility Type: category

Status: category

Due Date: datetime:%m/%d/%Y %I:%M:%S %p

Resolution Description: free_text

Resolution Action Updated Date: datetime:%m/%d/%Y %I:%M:%S %p

Community Board: category

BBL: unknown

Borough: category

X Coordinate (State Plane): unknown

Y Coordinate (State Plane): unknown

Open Data Channel Type: category

Park Facility Name: free_text

Park Borough: category

Vehicle Type: category

Taxi Company Borough: category

Taxi Pick Up Location: free_text

Bridge Highway Name: category

Bridge Highway Direction: category

Road Ramp: category

Bridge Highway Segment: free_text

Latitude: latitude

Longitude: longitude

Location (Lat/Lon): free_text

Now produce JSON mapping.

Let's craft output.
## TOKEN USAGE

TokenUsage { prompt: 36839, completion: 8328, total: 45167, elapsed: 138295 }
---
# Description
**Description**

The dataset records one million NYC 311 complaints filed from 1 January 2010 to 23 December 2020. Each record is uniquely identified by a surrogate integer key and contains the complaint’s creation, optional closure, and scheduled resolution timestamps; an agency code and full name; a primary complaint type with a detailed descriptor; location details (address, ZIP, city, borough, coordinates); status flags; and free‑text resolution narratives. The complaints span all boroughs and cover a wide range of issue categories, but the majority are “Other” or highly specific sub‑categories, indicating substantial heterogeneity in reporting.

---

### Notable Characteristics

- **Temporal distribution** – Created dates cluster around early 2010–2014 with a slight left‑skew (most recent complaints slightly outnumber older ones). Closed dates follow a similar pattern but include ~2.8 % of records still open (`NULL`).  
- **Agency dominance** – NYPD, HPD, and DOT together account for ~66 % of complaints; the remaining agencies contribute far fewer cases.  
- **Complaint taxonomy** – “Other” dominates 56 % of complaint types, while noise‑related categories (noise residential, heat/hot water) are the most common specific sub‑categories.  
- **Location heterogeneity** – Residential buildings and street/sidewalks make up ~45 % of incidents, yet ~24 % of location type entries are missing (`NULL`). ZIP codes, addresses, and street names exhibit very high cardinality; only a handful of ZIP codes or street names account for a few percent of records.  
- **Geospatial spread** – Latitude values range from 40.112° to 40.913°, longitude from –77.520° to –73.701°. State‑plane coordinates (X ≈ 1,000,000; Y ≈ 200,000) are tightly clustered with a small spread but contain occasional extreme outliers that fall outside the main interquartile range.  
- **Status distribution** – Closed complaints dominate (~95 %); open or pending cases are rare, and a handful of records remain unassigned.  
- **Missing data** – Several fields have large gaps: 28 k closed dates missing, 647 k due dates missing, 174 k incident addresses missing, and 912 k landmark names missing.  
- **“Other” categories** – Many attributes (descriptor, incident ZIP, street name, etc.) contain a generic “Other” bucket that captures the majority of values; this indicates many complaints are not captured by the predefined taxonomy.  
- **Duplicate safety** – The unique key guarantees no duplicate records; however, other fields exhibit high uniqueness ratios (e.g., latitude 35 % unique), reflecting fine‑grained spatial reporting.  
- **PII / PHI considerations** – The dataset contains addresses and coordinates that could be used to approximate a household location but does not include personal identifiers such as names or contact details; nonetheless, care should be taken when publishing geocoded data to avoid revealing sensitive residential information.

---

Generated by qsv v20.1.0 describegpt
Command line: target/debug/qsv describegpt --all --two-pass /tmp/NYC_311_SR_2010-2020-sample-1M.csv -o /tmp/nyc311-describegpt-everything.md --addl-cols --addl-cols-list everything --infer-content-type --fresh
Prompt file: Default v7.1.0
Model: openai/gpt-oss-20b
LLM API URL: http://localhost:1234/v1
Language: 
Timestamp: 2026-05-29T23:40:35.465037+00:00

WARNING: Description generated by an LLM and may contain inaccuracies. Verify before using!



## REASONING

We need to generate a Description based on summary stats, frequency distribution, and data dictionary but not output per-field details. Just a 1-8 sentence description about dataset as a whole.

Then Notable Characteristics bullet list: central tendency/spread, distribution shape, anomalies, patterns; outliers, missing values, duplicates, PII/PHI/PCI etc.

We should use stats: 1 million records, unique key 1000000. Created date range ~2010-01-01 to 2020-12-23. Closed date similar. Agency distribution: NYPD 26.5%, HPD 25.8%. Complaint type: noise residential 8.9%, other 56% etc.

We should mention high proportion of 'Other' categories for many fields (like descriptor, incident zip). Missing values large in some fields like Incident Address 174k nulls, Closed Date 28k nulls, Due Date 647k nulls. Many duplicates: unique key ensures uniqueness; but many other fields have high cardinality but still many distinct values.

Also geospatial coordinates: lat range .8 degrees (~80 km). State plane coords ~1 million units? Actually X coordinate mean 1,005,337 etc. Y similar.

Notable characteristics bullet list should mention:

- 1M records with unique key; duplicates prevented.
- Created dates mostly early 2010s; skewness slight negative -0.0857 indicating left skew (more recent). Actually skewness of created date -0.0857 small.
- Closed dates also similar, many open/NULL.

- Agencies: top 3 NYPD, HPD, DOT; rest less.

- Complaint types: majority 'Other' 56% indicates many unspecified categories.

- Descriptor: 'Other (1,381)' 67% of entries.

- Location Type: Residential Building highest 33.6%, but also high missing 239k.

- Incident Zip: 86% others; few top zip codes.

- Incident Address: 174k nulls; top address repeated.

- Street Name etc many unique.

- City: mostly Brooklyn, New York, Bronx; other cities less.

- Landmarks: many NULL (912k), only 1.5% have names.

- Facility type: N/A dominates 73%.

- Status: Closed 95%; open small.

- Due Date: majority null; others 350k with distribution across years 2010-2020.

- Resolution Description: 20480 nulls, 511k 'Other'.

- Spatial coordinates: lat/long range about 40.11 to 40.91 and -77.5 to -73.7. Many duplicates of coordinates.

Also mention outliers: X,Y coordinate extremes; some outlier values in Created Date "Other (841,004)" indicates many missing or unspecified dates? Actually frequency distribution shows 997333 counts for 'Other (841,004)' meaning 99.7333% not top 10 dates. Similarly Closed date etc.

Also mention data quality: high proportion of NULLs and 'Other' categories in many fields; high cardinality with many unique values; potential PII? Some addresses may contain sensitive info but not personal identifiers.

Also note that dataset is from NYC 311 complaints, includes addresses, agencies, dates, etc. Potentially contains PHI? No direct personal data except maybe names of individuals? Not present.

Ok produce description: "This dataset captures one million 311 service requests in New York City between January 2010 and December 2020, covering a range of complaint types..."

Then bullet list with characteristics.

Add attribution placeholder at end.
## TOKEN USAGE

TokenUsage { prompt: 33814, completion: 1412, total: 35226, elapsed: 68273 }
---
# Tags
- nyc_311_complaints  
- public_service_requests  
- city_governance  
- urban_infrastructure  
- noise_and_safety  
- environmental_issues  
- transportation_incidents  
- geospatial_data  
- borough_specific  
- data_publication  

Generated by qsv v20.1.0 describegpt
Command line: target/debug/qsv describegpt --all --two-pass /tmp/NYC_311_SR_2010-2020-sample-1M.csv -o /tmp/nyc311-describegpt-everything.md --addl-cols --addl-cols-list everything --infer-content-type --fresh
Prompt file: Default v7.1.0
Model: openai/gpt-oss-20b
LLM API URL: http://localhost:1234/v1
Language: 
Timestamp: 2026-05-29T23:41:50.196048+00:00

WARNING: Tags generated by an LLM and may contain inaccuracies. Verify before using!



## REASONING

We need to produce tags based on dataset content. No more than 10 unique tags, markdown format, descending importance. Use lowercase only and underscores between words. Avoid field names.

Dataset is NYC complaints data (311). Tags could be: nyc_311_complaints, public_service_requests, city_governance, urban_infrastructure, noise_and_safety, service_disruption, environmental_issues, transportation_incidents, borough_specific, geospatial_data.

We need 10 tags. Let's think which are most important.

Key aspects: City (NYC), complaints, service requests, public services, urban issues, noise, heat/hot water, illegal parking, street conditions. Also includes location coordinates. So tags:

1. nyc_311_complaints
2. city_governance
3. public_service_requests
4. urban_infrastructure
5. noise_and_safety
6. environmental_issues
7. transportation_incidents
8. geospatial_data
9. borough_specific
10. data_publication

But we need to consider importance descending: likely first is nyc_311_complaints, then public_service_requests, city_governance, urban_infrastructure, noise_and_safety, environmental_issues, transportation_incidents, geospatial_data, borough_specific, data_publication.

Let's produce tags in markdown format list. Add attribution after tags. Use the placeholder {GENERATED_BY_SIGNATURE}. Ensure no field names used. Let's output accordingly.
## TOKEN USAGE

TokenUsage { prompt: 33789, completion: 371, total: 34160, elapsed: 54709 }
---
