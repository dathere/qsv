# Dictionary
| Name | Type | Label | Description | Content Type | Min | Max | Cardinality | Enumeration | Null Count | is_ascii | sum | range | sort_order | sortiness | min_length | max_length | sum_length | avg_length | stddev_length | variance_length | cv_length | mean | sem | geometric_mean | harmonic_mean | stddev | variance | cv | n_negative | n_zero | n_positive | max_precision | sparsity | mad | lower_outer_fence | lower_inner_fence | q1 | q2_median | q3 | iqr | upper_inner_fence | upper_outer_fence | skewness | uniqueness_ratio | percentiles | Examples |
|------|------|-------|-------------|--------------|-----|-----|-------------|-------------|------------|----------|----------|----------|----------|----------|----------|----------|----------|----------|----------|----------|----------|----------|----------|----------|----------|----------|----------|----------|----------|----------|----------|----------|----------|----------|----------|----------|----------|----------|----------|----------|----------|----------|----------|----------|----------|----------|
| **Unique Key** | Integer | Record ID | A surrogate numeric identifier that uniquely distinguishes each complaint record in the dataset. | unique_id | 11465364 | 48478173 | 1,000,000 |  | 0 |  | 32687965858032 | 37012809 | Unsorted | 0.0018 |  |  |  |  |  |  |  | 32687965.858 | 9013.8953 | 31351729.249 | 29944311.4641 | 9013895.3358 | 81250309125279.2656 | 27.5756 | 0 | 0 | 1000000 |  | 0 | 7477037 | -19639208.5 | 2803282.25 | 25245773 | 32853358.5 | 40207433.5 | 14961660.5 | 62649924.25 | 85092415 | -0.0169 | 1 | 5: 18453724<br>10: 20062969<br>40: 29913180<br>60: 35829112<br>90: 45355115<br>95: 46937288 | <ALL_UNIQUE> |
| **Created Date** | DateTime | Complaint Creation Timestamp | The date and time when a complaint was originally filed, recorded with millisecond precision. | date:%m/%d/%Y | 01/01/2010 | 12/23/2020 | 841,014 |  | 0 |  |  | 4009.05962 | Unsorted | 0.0008 |  |  |  |  |  |  |  | 2015-11-10T18:05:22.615+00:00 | 1.15502 | 16709.46856 | 16668.78207 | 1155.01606 | 1334062.09198 | 6.8957 |  |  |  |  | 0 | 965.6694 | 1997-01-08T17:56:34.500+00:00 | 2005-02-08T08:58:19.500+00:00 | 2013-03-11T00:00:04.500+00:00 | 2016-02-12T13:16:49+00:00 | 2018-07-31T10:01:14.500+00:00 | 1968.41748 | 2026-08-31T01:02:59.500+00:00 | 2034-09-30T16:04:44.500+00:00 | -0.0857 | 0.841 | 5: 2010-08-10T00:00:00+00:00<br>10: 2011-03-15T11:35:08+00:00<br>40: 2015-01-21T10:24:00+00:00<br>60: 2017-02-25T20:10:00+00:00<br>90: 2020-01-10T08:26:00+00:00<br>95: 2020-07-21T18:32:11+00:00 | Other… [997,333]<br>01/24/2013 [347]<br>01/07/2014 [315]<br>01/08/2015 [283]<br>02/16/2015 [269] |
| **Closed Date** | DateTime | Complaint Closure Timestamp | The date and time when the complaint was officially closed or resolved. May be null if still open. | date:%m/%d/%Y | 01/01/1900 | 01/01/2100 | 688,837 |  | 28,619 |  |  | 73049 | Unsorted | 0.001 |  |  |  |  |  |  |  | 2015-11-14T10:16:16.743+00:00 | 1.33393 |  |  | 1314.70016 | 1728436.50813 | 7.8474 |  |  |  |  | 0.0286 | 954.61806 | 1997-04-12T11:33:24.500+00:00 | 2005-04-09T10:53:21.500+00:00 | 2013-04-06T10:13:18.500+00:00 | 2016-02-26T01:40:00+00:00 | 2018-08-04T09:46:36.500+00:00 | 1945.98146 | 2026-08-01T09:06:33.500+00:00 | 2034-07-29T08:26:30.500+00:00 | -0.0849 | 0.6888 | 5: 2010-08-23T11:35:00+00:00<br>10: 2011-04-01T00:00:00+00:00<br>40: 2015-02-09T00:00:00+00:00<br>60: 2017-03-08T14:06:03+00:00<br>90: 2020-01-07T00:02:00+00:00<br>95: 2020-07-20T12:00:00+00:00 | Other… [968,671]<br>(NULL)… [28,619]<br>11/15/2010 [384]<br>11/07/2012 [329]<br>12/09/2010 [267] |
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
| **Resolution Action Updated Date** | DateTime | Last Resolution Update Timestamp | The date and time when the resolution details were last updated. | date:%m/%d/%Y | 12/31/2009 | 12/23/2020 | 690,314 |  | 15,072 |  |  | 4010.22308 | Unsorted | 0.001 |  |  |  |  |  |  |  | 2015-11-19T19:44:34.889+00:00 | 1.16204 | 16718.67594 | 16678.12298 | 1153.24922 | 1329983.75668 | 6.8814 |  |  |  |  | 0.0151 | 966.48803 | 1997-01-12T11:30:24+00:00 | 2005-02-14T12:31:15.750+00:00 | 2013-03-19T13:32:07.500+00:00 | 2016-02-22T22:38:30+00:00 | 2018-08-10T14:12:42+00:00 | 1970.02818 | 2026-09-12T15:13:33.750+00:00 | 2034-10-15T16:14:25.500+00:00 | -0.0867 | 0.6903 | 5: 2010-08-22T00:00:00+00:00<br>10: 2011-03-25T11:49:23+00:00<br>40: 2015-02-01T00:00:00+00:00<br>60: 2017-03-10T02:24:33+00:00<br>90: 2020-01-12T09:08:00+00:00<br>95: 2020-07-22T01:06:53+00:00 | Other… [982,148]<br>(NULL)… [15,072]<br>11/15/2010 [385]<br>11/07/2012 [336]<br>12/09/2010 [273] |
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
