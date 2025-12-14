Generated using a Local LLM (openai/gpt-oss-20b) on LM Studio 0.3.34 Build 1 running on a Macbook Pro M4 Max 64gb/Tahoe 26.2:

```bash
$ QSV_LLM_BASE_URL=http://localhost:1234/v1 qsv describegpt NYC_311_SR_2010-2020-sample-1M.csv --all \
     --addl-cols \
     --addl-cols-list everything \
     --output nyc311-describegpt-everything.md
```
---
# Dictionary
| Name | Type | Label | Description | Min | Max | Cardinality | Enumeration | Null Count | is_ascii | sum | range | sort_order | sortiness | min_length | max_length | sum_length | avg_length | stddev_length | variance_length | cv_length | mean | sem | geometric_mean | harmonic_mean | stddev | variance | cv | n_negative | n_zero | n_positive | max_precision | sparsity | mad | lower_outer_fence | lower_inner_fence | q1 | q2_median | q3 | iqr | upper_inner_fence | upper_outer_fence | skewness | uniqueness_ratio | percentiles | Examples |
|------|------|-------|-------------|-----|-----|-------------|-------------|------------|----------|----------|----------|----------|----------|----------|----------|----------|----------|----------|----------|----------|----------|----------|----------|----------|----------|----------|----------|----------|----------|----------|----------|----------|----------|----------|----------|----------|----------|----------|----------|----------|----------|----------|----------|----------|----------|
| **Unique Key** | Integer | Record Identifier | A unique integer assigned to each complaint record in the dataset. With a cardinality of 1,000,000 and a uniqueness ratio of 1.0, this field guarantees that every row can be referenced unambiguously. | 11465364 | 48478173 | 1,000,000 |  | 0 |  | 32687965858032 | 37012809 | Unsorted | 0.0018 |  |  |  |  |  |  |  | 32687965.858 | 9013.8953 | 31351729.2491 | 29944311.4641 | 9013895.3358 | 81250309125281.5938 | 27.5756 | 0 | 0 | 1000000 |  | 0 | 7477037 | -19639208.5 | 2803282.25 | 25245773 | 32853358.5 | 40207433.5 | 14961660.5 | 62649924.25 | 85092415 | -0.0169 | 1 | 5: 18453724<br>10: 20062969<br>40: 29913180<br>60: 35829112<br>90: 45355115<br>95: 46937288 | <ALL_UNIQUE> |
| **Created Date** | DateTime | Complaint Creation Timestamp | The date and time when the complaint was entered into the system. The data span from 2010‑01‑01 to 2020‑12‑23, with a mean of 2015‑11‑10. The most frequent dates are clustered in early 2013–2014, indicating periods of higher reporting activity. | 2010-01-01T00:00:00+00:00 | 2020-12-23T01:25:51+00:00 | 841,014 |  | 0 |  |  | 4009.05962 | Unsorted | 0.0008 |  |  |  |  |  |  |  | 2015-11-10T18:05:22.615+00:00 | 1.15502 | 16709.46856 | 16668.78207 | 1155.01606 | 1334062.09198 | 6.8957 |  |  |  |  | 0 | 965.6694 | 1997-01-08T17:56:34.500+00:00 | 2005-02-08T08:58:19.500+00:00 | 2013-03-11T00:00:04.500+00:00 | 2016-02-12T13:16:49+00:00 | 2018-07-31T10:01:14.500+00:00 | 1968.41748 | 2026-08-31T01:02:59.500+00:00 | 2034-09-30T16:04:44.500+00:00 | -0.0857 | 0.841 | 5: 2010-08-10T00:00:00+00:00<br>10: 2011-03-15T11:35:08+00:00<br>40: 2015-01-21T10:24:00+00:00<br>60: 2017-02-25T20:10:00+00:00<br>90: 2020-01-10T08:26:00+00:00<br>95: 2020-07-21T18:32:11+00:00 | Other (841,004) [997,333]<br>01/24/2013 12:00:00 AM [347]<br>01/07/2014 12:00:00 AM [315]<br>01/08/2015 12:00:00 AM [283]<br>02/16/2015 12:00:00 AM [269] |
| **Closed Date** | DateTime | Complaint Closure Timestamp | The date and time when the complaint was officially closed or resolved. Null values (2.86 % of records) indicate complaints that remain open. The dates range from 1900‑01‑01 to 2100‑01‑01, with a mean close date of 2015‑11‑14. | 1900-01-01T00:00:00+00:00 | 2100-01-01T00:00:00+00:00 | 688,837 |  | 28,619 |  |  | 73049 | Unsorted | 0.001 |  |  |  |  |  |  |  | 2015-11-14T10:16:16.743+00:00 | 1.33393 |  |  | 1314.70016 | 1728436.50813 | 7.8474 |  |  |  |  | 0.0286 | 954.61806 | 1997-04-12T11:33:24.500+00:00 | 2005-04-09T10:53:21.500+00:00 | 2013-04-06T10:13:18.500+00:00 | 2016-02-26T01:40:00+00:00 | 2018-08-04T09:46:36.500+00:00 | 1945.98146 | 2026-08-01T09:06:33.500+00:00 | 2034-07-29T08:26:30.500+00:00 | -0.0849 | 0.6888 | 5: 2010-08-23T11:35:00+00:00<br>10: 2011-04-01T00:00:00+00:00<br>40: 2015-02-09T00:00:00+00:00<br>60: 2017-03-08T14:06:03+00:00<br>90: 2020-01-07T00:02:00+00:00<br>95: 2020-07-20T12:00:00+00:00 | Other (688,827) [968,897]<br>(NULL) [28,619]<br>11/15/2010 12:00:00 AM [384]<br>11/07/2012 12:00:00 AM [329]<br>12/09/2010 12:00:00 AM [267] |
| **Agency** | String | Reporting Agency Code | An abbreviated code for the agency that received or handled the complaint. The most common codes are NYPD (26.5 %) and HPD (25.8 %). This field maps to the full agency name in the "Agency Name" column. | 3-1-1 | TLC | 28 |  | 0 | false |  |  | Unsorted | 0.1729 | 3 | 42 | 3490582 | 3.4906 | 1.8975 | 3.6005 | 0.5436 |  |  |  |  |  |  |  |  |  |  |  | 0 |  |  |  |  |  |  |  |  |  |  | 0 |  | NYPD [265,116]<br>HPD [258,033]<br>DOT [132,462]<br>DSNY [81,606]<br>DEP [75,895] |
| **Agency Name** | String | Reporting Agency Full Name | The official, human‑readable name of the agency that processed the complaint. For example, NYPD corresponds to "New York City Police Department", HPD to "Department of Housing Preservation and Development", etc. | 3-1-1 | Valuation Policy | 553 |  | 0 | false |  |  | Unsorted | 0.1671 | 3 | 82 | 34840715 | 34.8407 | 10.5137 | 110.5379 | 0.3018 |  |  |  |  |  |  |  |  |  |  |  | 0 |  |  |  |  |  |  |  |  |  |  | 0.0006 |  | New York City Police Depa… [265,038]<br>Department of Housing Pre… [258,019]<br>Department of Transportat… [132,462]<br>Other (543) [103,974]<br>Department of Environment… [75,895] |
| **Complaint Type** | String | Complaint Category | High‑level classification of the issue reported. The dominant categories are Noise (Residential) at 8.94 % and HEAT/HOT WATER at 5.66 %. Other categories include Illegal Parking, Blocked Driveway, and Street Condition. | ../../WEB-INF/web.xml;x= | ZTESTINT | 287 |  | 0 | true |  |  | Unsorted | 0.0284 | 3 | 41 | 16475270 | 16.4753 | 6.8221 | 46.5406 | 0.4141 |  |  |  |  |  |  |  |  |  |  |  | 0 |  |  |  |  |  |  |  |  |  |  | 0.0003 |  | Other (277) [563,561]<br>Noise - Residential [89,439]<br>HEAT/HOT WATER [56,639]<br>Illegal Parking [45,032]<br>Blocked Driveway [42,356] |
| **Descriptor** | String | Detailed Complaint Descriptor | A more specific description within the complaint category, such as "Loud Music/Party" or "HEAT". The most frequent descriptor is Loud Music/Party (9.36 %). | 1 Missed Collection | unknown odor/taste in drinking water (QA6) | 1,392 |  | 3,001 | true |  |  | Unsorted | 0.0186 | 0 | 80 | 17426583 | 17.4266 | 10.4342 | 108.8723 | 0.5988 |  |  |  |  |  |  |  |  |  |  |  | 0.003 |  |  |  |  |  |  |  |  |  |  | 0.0014 |  | Other (1,382) [674,871]<br>Loud Music/Party [93,646]<br>ENTIRE BUILDING [36,885]<br>HEAT [35,088]<br>No Access [31,631] |
| **Location Type** | String | Physical Location Category | The type of place where the issue occurred: residential building, street/sidewalk, parking lot, etc. Residential buildings account for 25.6 % of records; null values (23.9 %) represent missing or ambiguous locations. | 1-, 2- and 3- Family Home | Wooded Area | 162 |  | 239,131 | true |  |  | Unsorted | 0.187 | 0 | 36 | 12417750 | 12.4177 | 8.976 | 80.5677 | 0.7228 |  |  |  |  |  |  |  |  |  |  |  | 0.2391 |  |  |  |  |  |  |  |  |  |  | 0.0002 |  | RESIDENTIAL BUILDING [255,562]<br>(NULL) [239,131]<br>Street/Sidewalk [145,653]<br>Residential Building/Hous… [92,765]<br>Street [92,190] |
| **Incident Zip** | String | Incident ZIP Code | The five‑digit postal code where the complaint was filed. Nulls constitute 5.50 % of entries, and the majority of records cluster around NYC ZIP codes such as 11226 and 10467. | * | XXXXX | 535 |  | 54,978 | true |  |  | Unsorted | 0.0084 | 0 | 10 | 4713122 | 4.7131 | 1.1405 | 1.3007 | 0.242 |  |  |  |  |  |  |  |  |  |  |  | 0.055 |  |  |  |  |  |  |  |  |  |  | 0.0005 |  | Other (525) [827,654]<br>(NULL) [54,978]<br>11226 [17,114]<br>10467 [14,495]<br>11207 [12,872] |
| **Incident Address** | String | Full Incident Address | The street address associated with the incident. About 17.47 % of records have a null value; the most common addresses are "655 EAST 230 STREET" and "78‑15 PARSONS BOULEVARD". | * * | west 155 street and edgecombe avenue | 341,996 |  | 174,700 | true |  |  | Unsorted | -0.0005 | 0 | 55 | 14591947 | 14.5919 | 7.3321 | 53.7594 | 0.5025 |  |  |  |  |  |  |  |  |  |  |  | 0.1747 |  |  |  |  |  |  |  |  |  |  | 0.342 |  | Other (341,986) [819,378]<br>(NULL) [174,700]<br>655 EAST  230 STREET [1,538]<br>78-15 PARSONS BOULEVARD [694]<br>672 EAST  231 STREET [663] |
| **Street Name** | String | Primary Street Name | The main street on which the incident occurred. The most frequent streets include Broadway, Grand Concourse, and Ocean Avenue; about 17.47 % of records have a null value. | * | wyckoff avenue | 14,837 |  | 174,720 | true |  |  | Unsorted | 0.0001 | 0 | 55 | 10888475 | 10.8885 | 5.7969 | 33.6035 | 0.5324 |  |  |  |  |  |  |  |  |  |  |  | 0.1747 |  |  |  |  |  |  |  |  |  |  | 0.0148 |  | Other (14,827) [787,222]<br>(NULL) [174,720]<br>BROADWAY [9,702]<br>GRAND CONCOURSE [5,851]<br>OCEAN AVENUE [3,946] |
| **Cross Street 1** | String | First Cross Street | One of the cross streets at an intersection involved in the incident. Null values account for 32.04 %; the most common cross street is "BEND". | 1 AVE | mermaid | 16,238 |  | 320,401 | true |  |  | Unsorted | 0.0009 | 0 | 32 | 8355458 | 8.3555 | 6.6045 | 43.6194 | 0.7904 |  |  |  |  |  |  |  |  |  |  |  | 0.3204 |  |  |  |  |  |  |  |  |  |  | 0.0162 |  | Other (16,228) [623,317]<br>(NULL) [320,401]<br>BEND [12,562]<br>BROADWAY [8,548]<br>3 AVENUE [6,154] |
| **Cross Street 2** | String | Second Cross Street | The second cross street, if applicable. Approximately 32.36 % of records have a null value; frequent values include "BEND" and "BROADWAY". | 1 AVE | surf | 16,486 |  | 323,644 | true |  |  | Unsorted | 0.0016 | 0 | 35 | 8363431 | 8.3634 | 6.645 | 44.1555 | 0.7945 |  |  |  |  |  |  |  |  |  |  |  | 0.3236 |  |  |  |  |  |  |  |  |  |  | 0.0165 |  | Other (16,476) [626,168]<br>(NULL) [323,644]<br>BEND [12,390]<br>BROADWAY [8,833]<br>DEAD END [5,626] |
| **Intersection Street 1** | String | First Intersection Street | One of the streets forming an intersection at the incident location. Nulls occur in 76.74 % of records, with the most common being "BROADWAY". | 1 AVE | flatlands AVE | 11,237 |  | 767,422 | true |  |  | Unsorted | -0.0009 | 0 | 35 | 2949273 | 2.9493 | 5.6792 | 32.2534 | 1.9256 |  |  |  |  |  |  |  |  |  |  |  | 0.7674 |  |  |  |  |  |  |  |  |  |  | 0.0112 |  | (NULL) [767,422]<br>Other (11,227) [215,482]<br>BROADWAY [3,761]<br>CARPENTER AVENUE [2,918]<br>BEND [2,009] |
| **Intersection Street 2** | String | Second Intersection Street | The second street in an intersection. Around 76.77 % of records are null; frequent values include "BROADWAY" and "BEND". | 1 AVE | glenwood RD | 11,674 |  | 767,709 | true |  |  | Unsorted | 0.003 | 0 | 33 | 2917798 | 2.9178 | 5.6362 | 31.767 | 1.9317 |  |  |  |  |  |  |  |  |  |  |  | 0.7677 |  |  |  |  |  |  |  |  |  |  | 0.0117 |  | (NULL) [767,709]<br>Other (11,664) [216,748]<br>BROADWAY [3,462]<br>BEND [1,942]<br>2 AVENUE [1,690] |
| **Address Type** | String | Address Classification | Indicates the nature of the address field: ADDRESS, INTERSECTION, BLOCKFACE, LATLONG, etc. The most common type is ADDRESS (71.04 %). | ADDRESS | PLACENAME | 6 | (NULL)<br>ADDRESS<br>BLOCKFACE<br>INTERSECTION<br>LATLONG<br>PLACENAME | 125,802 | true |  |  | Unsorted | 0.6845 | 0 | 12 | 6832263 | 6.8323 | 3.0923 | 9.5623 | 0.4526 |  |  |  |  |  |  |  |  |  |  |  | 0.1258 |  |  |  |  |  |  |  |  |  |  | 0 |  | ADDRESS [710,380]<br>INTERSECTION [133,361]<br>(NULL) [125,802]<br>BLOCKFACE [22,620]<br>LATLONG [7,421] |
| **City** | String | Incident City | The city or borough where the incident took place. The majority are in Brooklyn (29.63 %) and New York (18.91 %); null values account for 6.20 %. | * | YORKTOWN HEIGHTS | 382 |  | 61,963 | true |  |  | Unsorted | 0.1811 | 0 | 22 | 7721241 | 7.7212 | 3.2635 | 10.6505 | 0.4227 |  |  |  |  |  |  |  |  |  |  |  | 0.062 |  |  |  |  |  |  |  |  |  |  | 0.0004 |  | BROOKLYN [296,254]<br>NEW YORK [189,069]<br>BRONX [181,168]<br>Other (372) [171,028]<br>(NULL) [61,963] |
| **Landmark** | String | Nearby Landmark | A notable landmark or building near the incident location, such as a street name or public structure. Most records have a null value (91.28 %). | 1 AVENUE | ZULETTE AVENUE | 5,915 |  | 912,779 | true |  |  | Unsorted | 0.001 | 0 | 32 | 1165773 | 1.1658 | 3.8975 | 15.1908 | 3.3433 |  |  |  |  |  |  |  |  |  |  |  | 0.9128 |  |  |  |  |  |  |  |  |  |  | 0.0059 |  | (NULL) [912,779]<br>Other (5,905) [80,508]<br>EAST  230 STREET [1,545]<br>EAST  231 STREET [1,291]<br>BROADWAY [1,148] |
| **Facility Type** | String | Related Facility Category | Type of facility involved in the complaint, e.g., DSNY Garage or Precinct. The most frequent is "N/A" at 62.83 %; null values represent missing information. | DSNY Garage | School District | 6 | (NULL)<br>DSNY Garage<br>N/A<br>Precinct<br>School<br>School District | 145,478 | true |  |  | Unsorted | 0.5941 | 0 | 15 | 3790876 | 3.7909 | 2.7562 | 7.5969 | 0.7271 |  |  |  |  |  |  |  |  |  |  |  | 0.1455 |  |  |  |  |  |  |  |  |  |  | 0 |  | N/A [628,279]<br>Precinct [193,259]<br>(NULL) [145,478]<br>DSNY Garage [32,310]<br>School [617] |
| **Status** | String | Complaint Status | Current status of the complaint: Closed (95.25 %), Pending, Open, etc. This field tracks whether a resolution has been issued. | Assigned | Unspecified | 10 | Assigned<br>Closed<br>Closed - Testing<br>Email Sent<br>In Progress<br>Open<br>Pending<br>Started<br>Unassigned<br>Unspecified | 0 | true |  |  | Unsorted | 0.9079 | 4 | 16 | 6048943 | 6.0489 | 0.5411 | 0.2928 | 0.0894 |  |  |  |  |  |  |  |  |  |  |  | 0 |  |  |  |  |  |  |  |  |  |  | 0 |  | Closed [952,522]<br>Pending [20,119]<br>Open [12,340]<br>In Progress [7,841]<br>Assigned [6,651] |
| **Due Date** | DateTime | Resolution Due Timestamp | The scheduled date and time by which an action should be completed. Nulls dominate this column (64.78 %). The earliest due dates cluster in early 2015, with later dates extending to 2024. | 1900-01-02T00:00:00+00:00 | 2021-06-17T16:34:13+00:00 | 345,077 |  | 647,794 |  |  | 44361.69043 | Unsorted | 0.0011 |  |  |  |  |  |  |  | 2015-05-30T02:54:49.998+00:00 | 1.74433 |  |  | 1035.204 | 1071647.31812 | 6.2418 |  |  |  |  | 0.6478 | 800.07713 | 1999-09-09T02:53:37+00:00 | 2006-06-14T19:09:32.500+00:00 | 2013-03-20T11:25:28+00:00 | 2015-10-03T01:27:48+00:00 | 2017-09-22T14:16:05+00:00 | 1647.11848 | 2024-06-28T06:32:00.500+00:00 | 2031-04-03T22:47:56+00:00 | -0.1251 | 0.3451 | 5: 2010-08-25T10:54:44+00:00<br>10: 2011-04-14T19:02:13+00:00<br>40: 2014-11-06T10:17:31+00:00<br>60: 2016-07-31T14:14:53+00:00<br>90: 2018-10-25T07:47:20+00:00<br>95: 2019-03-24T08:03:29+00:00 | (NULL) [647,794]<br>Other (345,067) [350,849]<br>04/08/2015 10:00:58 AM [214]<br>05/02/2014 03:32:17 PM [183]<br>03/30/2018 10:10:39 AM [172] |
| **Resolution Description** | String | Resolution Narrative | Free‑text description of the outcome or actions taken for a complaint. The most frequent narrative is an explanatory statement from the Police Department. Null values constitute 2.05 %. | A DOB violation was issued for failing to comply with an existing Stop Work Order. | Your request was submitted to the Department of Homeless Services. The City?s outreach team will assess the homeless individual and offer appropriate assistance within 2 hours. If you asked to know the outcome of your request, you will get a call within 2 hours. No further status will be available through the NYC 311 App, 311, or 311 Online. | 1,216 |  | 20,480 | false |  |  | Unsorted | 0.0319 | 0 | 934 | 153148305 | 153.1483 | 82.149 | 6748.4538 | 0.5364 |  |  |  |  |  |  |  |  |  |  |  | 0.0205 |  |  |  |  |  |  |  |  |  |  | 0.0012 |  | Other (1,206) [532,002]<br>The Police Department res… [91,408]<br>The Department of Housing… [72,962]<br>The Police Department res… [63,868]<br>Service Request status fo… [52,155] |
| **Resolution Action Updated Date** | DateTime | Last Resolution Update Timestamp | The date and time when the resolution details were last modified. Nulls are rare (1.51 %). | 2009-12-31T01:35:00+00:00 | 2020-12-23T06:56:14+00:00 | 690,314 |  | 15,072 |  |  | 4010.22308 | Unsorted | 0.001 |  |  |  |  |  |  |  | 2015-11-19T19:44:34.889+00:00 | 1.16204 | 16718.67594 | 16678.12298 | 1153.24922 | 1329983.75668 | 6.8814 |  |  |  |  | 0.0151 | 966.48803 | 1997-01-12T11:30:24+00:00 | 2005-02-14T12:31:15.750+00:00 | 2013-03-19T13:32:07.500+00:00 | 2016-02-22T22:38:30+00:00 | 2018-08-10T14:12:42+00:00 | 1970.02818 | 2026-09-12T15:13:33.750+00:00 | 2034-10-15T16:14:25.500+00:00 | -0.0867 | 0.6903 | 5: 2010-08-22T00:00:00+00:00<br>10: 2011-03-25T11:49:23+00:00<br>40: 2015-02-01T00:00:00+00:00<br>60: 2017-03-10T02:24:33+00:00<br>90: 2020-01-12T09:08:00+00:00<br>95: 2020-07-22T01:06:53+00:00 | Other (690,304) [982,378]<br>(NULL) [15,072]<br>11/15/2010 12:00:00 AM [385]<br>11/07/2012 12:00:00 AM [336]<br>12/09/2010 12:00:00 AM [273] |
| **Community Board** | String | Community Board Number | The NYC community board that has jurisdiction over the incident location, expressed as a number or "Unspecified". The majority of records are unspecified (75.16 %). | 0 Unspecified | Unspecified STATEN ISLAND | 77 |  | 0 | true |  |  | Unsorted | 0.0193 | 8 | 25 | 11142863 | 11.1429 | 2.971 | 8.8269 | 0.2666 |  |  |  |  |  |  |  |  |  |  |  | 0 |  |  |  |  |  |  |  |  |  |  | 0.0001 |  | Other (67) [751,635]<br>0 Unspecified [49,878]<br>12 MANHATTAN [29,845]<br>12 QUEENS [23,570]<br>01 BROOKLYN [21,714] |
| **BBL** | String | Borough‑Block‑Lot Identifier | A unique property identifier used in New York City’s land record system. Null values appear in 24.30 % of records; the most common BBLs are small, low‑value numbers. | 0000000000 | 5270000501 | 268,383 |  | 243,046 | true |  |  | Unsorted | -0.0007 | 0 | 10 | 2821442 | 2.8214 | 4.2791 | 18.3108 | 1.5166 |  |  |  |  |  |  |  |  |  |  |  | 0.243 |  |  |  |  |  |  |  |  |  |  | 0.2684 |  | Other (268,373) [751,031]<br>(NULL) [243,046]<br>2048330028 [1,566]<br>4068290001 [696]<br>4015110001 [664] |
| **Borough** | String | Incident Borough | The borough where the incident occurred: Brooklyn (29.61 %), Queens (22.88 %), Manhattan (19.55 %) or Bronx (18.01 %). "Unspecified" accounts for 4.99 %. | BRONX | Unspecified | 6 | BRONX<br>BROOKLYN<br>MANHATTAN<br>QUEENS<br>STATEN ISLAND<br>Unspecified | 0 | true |  |  | Unsorted | 0.2155 | 5 | 13 | 7595025 | 7.595 | 2.0632 | 4.2568 | 0.2717 |  |  |  |  |  |  |  |  |  |  |  | 0 |  |  |  |  |  |  |  |  |  |  | 0 |  | BROOKLYN [296,081]<br>QUEENS [228,818]<br>MANHATTAN [195,488]<br>BRONX [180,142]<br>Unspecified [49,878] |
| **X Coordinate (State Plane)** | Integer | State‑Plane X Coordinate | The eastward coordinate in the New York State Plane system, expressed in feet. Nulls occur in 8.53 % of records; the most common value is 1,022,911. | 913281 | 1067220 | 102,556 |  | 85,327 |  | 919555108413 | 153939 | Unsorted | -0.0004 |  |  |  |  |  |  |  | 1005337.5451 | 23.5391 | 1005083.7023 | 1004827.9356 | 22512.4528 | 506810531.5324 | 2.2393 | 0 | 0 | 914673 |  | 0.0853 | 12292 | 919661 | 956616.5 | 993572 | 1004546 | 1018209 | 24637 | 1055164.5 | 1092120 | 0.1091 | 0.1026 | 5: 964313<br>10: 984035<br>40: 999859<br>60: 1009147<br>90: 1034015<br>95: 1043903 | Other (102,546) [908,877]<br>(NULL) [85,327]<br>1022911 [1,568]<br>1037000 [701]<br>1023174 [675] |
| **Y Coordinate (State Plane)** | Integer | State‑Plane Y Coordinate | The northward coordinate in the New York State Plane system, expressed in feet. Nulls also occur in 8.53 % of records; the most common value is 264,242. | 121152 | 271876 | 116,092 |  | 85,327 |  | 188099299101 | 150724 | Unsorted | 0 |  |  |  |  |  |  |  | 205646.4978 | 33.1699 | 203166.0871 | 200659.7012 | 31723.1985 | 1006361322.6747 | 15.4261 | 0 | 0 | 914673 |  | 0.0853 | 24236 | 24257 | 103334 | 182411 | 202514 | 235129 | 52718 | 314206 | 393283 | 0.2373 | 0.1161 | 5: 156639<br>10: 164744<br>40: 193463<br>60: 212470<br>90: 250365<br>95: 256054 | Other (116,082) [908,868]<br>(NULL) [85,327]<br>264242 [1,566]<br>202363 [706]<br>211606 [665] |
| **Open Data Channel Type** | String | Submission Channel | Method by which the complaint was submitted: PHONE (49.76 %), ONLINE (17.73 %) or MOBILE (7.99 %). "UNKNOWN" accounts for 23.04 %. | MOBILE | UNKNOWN | 5 | MOBILE<br>ONLINE<br>OTHER<br>PHONE<br>UNKNOWN | 0 | true |  |  | Unsorted | 0.3379 | 5 | 7 | 5718030 | 5.718 | 0.8144 | 0.6633 | 0.1424 |  |  |  |  |  |  |  |  |  |  |  | 0 |  |  |  |  |  |  |  |  |  |  | 0 |  | PHONE [497,606]<br>UNKNOWN [230,402]<br>ONLINE [177,334]<br>MOBILE [79,892]<br>OTHER [14,766] |
| **Park Facility Name** | String | Park Facility Name | Name of a park facility involved in the incident, such as playgrounds or recreation centers. The majority of records are unspecified (99.31 %). | "Uncle" Vito F. Maranzano Glendale Playground | Zimmerman Playground | 1,889 |  | 0 | true |  |  | Unsorted | 0.9863 | 3 | 82 | 11072428 | 11.0724 | 1.2391 | 1.5353 | 0.1119 |  |  |  |  |  |  |  |  |  |  |  | 0 |  |  |  |  |  |  |  |  |  |  | 0.0019 |  | Unspecified [993,141]<br>Other (1,879) [5,964]<br>Central Park [261]<br>Riverside Park [136]<br>Prospect Park [129] |
| **Park Borough** | String | Park Borough | Borough where the park facility is located; distribution mirrors overall borough percentages. | BRONX | Unspecified | 6 | BRONX<br>BROOKLYN<br>MANHATTAN<br>QUEENS<br>STATEN ISLAND<br>Unspecified | 0 | true |  |  | Unsorted | 0.2155 | 5 | 13 | 7595025 | 7.595 | 2.0632 | 4.2568 | 0.2717 |  |  |  |  |  |  |  |  |  |  |  | 0 |  |  |  |  |  |  |  |  |  |  | 0 |  | BROOKLYN [296,081]<br>QUEENS [228,818]<br>MANHATTAN [195,488]<br>BRONX [180,142]<br>Unspecified [49,878] |
| **Vehicle Type** | String | Associated Vehicle Type | Type of vehicle referenced in the complaint, e.g., Car Service or Ambulette. Nulls are very common (99.97 %). | Ambulette / Paratransit | Green Taxi | 5 | (NULL)<br>Ambulette / Paratransit<br>Car Service<br>Commuter Van<br>Green Taxi | 999,652 | true |  |  | Unsorted | 0.8386 | 0 | 23 | 4066 | 0.0041 | 0.2239 | 0.0501 | 55.0744 |  |  |  |  |  |  |  |  |  |  |  | 0.9997 |  |  |  |  |  |  |  |  |  |  | 0 |  | (NULL) [999,652]<br>Car Service [317]<br>Ambulette / Paratransit [19]<br>Commuter Van [11]<br>Green Taxi [1] |
| **Taxi Company Borough** | String | Taxi Company Operating Borough | Borough where the taxi company operates or is based; nearly all records have null values (99.92 %). | BRONX | Staten Island | 11 |  | 999,156 | true |  |  | Unsorted | 0.1815 | 0 | 13 | 6313 | 0.0063 | 0.2246 | 0.0504 | 35.5769 |  |  |  |  |  |  |  |  |  |  |  | 0.9992 |  |  |  |  |  |  |  |  |  |  | 0 |  | (NULL) [999,156]<br>BROOKLYN [207]<br>QUEENS [194]<br>MANHATTAN [171]<br>BRONX [127] |
| **Taxi Pick Up Location** | String | Taxi Pick‑Up Site | Location where a taxi was requested or picked up, such as an intersection or airport. Nulls are rare (0.79 %); the most frequent entry is "ADDRESS". | 1 5 AVENUE MANHATTAN | YORK AVENUE AND EAST 70 STREET | 1,903 |  | 992,129 | true |  |  | Unsorted | 0.2851 | 0 | 60 | 135661 | 0.1357 | 2.15 | 4.6227 | 15.8487 |  |  |  |  |  |  |  |  |  |  |  | 0.9921 |  |  |  |  |  |  |  |  |  |  | 0.0019 |  | (NULL) [992,129]<br>Other [4,091]<br>Other (1,893) [2,021]<br>JFK Airport [562]<br>Intersection [486] |
| **Bridge Highway Name** | String | Bridge/Highway Name | Name of the bridge or highway involved in a traffic‑related complaint. The column is mostly null (99.77 %). | 145th St. Br - Lenox Ave | Willis Ave Br - 125th St/1st Ave | 68 |  | 997,711 | true |  |  | Unsorted | 0.0201 | 0 | 42 | 36974 | 0.037 | 0.8203 | 0.6729 | 22.1852 |  |  |  |  |  |  |  |  |  |  |  | 0.9977 |  |  |  |  |  |  |  |  |  |  | 0.0001 |  | (NULL) [997,711]<br>Other (58) [851]<br>Belt Pkwy [276]<br>BQE/Gowanus Expwy [254]<br>Grand Central Pkwy [186] |
| **Bridge Highway Direction** | String | Bridge/Highway Direction | Direction of travel on the bridge or highway, e.g., East/Long Island Bound. Nulls dominate (99.77 %). | Bronx Bound | Westbound/To Goethals Br | 50 |  | 997,691 | true |  |  | Unsorted | 0.0329 | 0 | 33 | 44089 | 0.0441 | 0.9512 | 0.9048 | 21.5746 |  |  |  |  |  |  |  |  |  |  |  | 0.9977 |  |  |  |  |  |  |  |  |  |  | 0.0001 |  | (NULL) [997,691]<br>Other (40) [1,064]<br>East/Long Island Bound [210]<br>North/Bronx Bound [208]<br>East/Queens Bound [197] |
| **Road Ramp** | String | Ramp Type | Indicates whether a ramp was involved; "ROADWAY" is most common when present. Null values are the majority. | N/A | Roadway | 4 | (NULL)<br>N/A<br>Ramp<br>Roadway | 997,693 | true |  |  | Unsorted | 0.6227 | 0 | 7 | 14400 | 0.0144 | 0.3062 | 0.0938 | 21.2639 |  |  |  |  |  |  |  |  |  |  |  | 0.9977 |  |  |  |  |  |  |  |  |  |  | 0 |  | (NULL) [997,693]<br>Roadway [1,731]<br>Ramp [555]<br>N/A [21] |
| **Bridge Highway Segment** | String | Highway/Bridge Segment Identifier | Identifier for a specific segment of a bridge or highway. The column is almost entirely null (99.76 %). | 1-1-1265963747 | Wythe Ave/Kent Ave (Exit 31) | 937 |  | 997,556 | true |  |  | Unsorted | -0.0037 | 0 | 100 | 110781 | 0.1108 | 2.511 | 6.3049 | 22.666 |  |  |  |  |  |  |  |  |  |  |  | 0.9976 |  |  |  |  |  |  |  |  |  |  | 0.0009 |  | (NULL) [997,556]<br>Other (927) [2,159]<br>Ramp [92]<br>Roadway [54]<br>Clove Rd/Richmond Rd (Exi… [23] |
| **Latitude** | Float | Geographic Latitude | The latitude coordinate in decimal degrees, ranging from 40.112 to 40.913. Nulls are present in 25.47 % of records; the most common value is 40.89187241649303. | 40.1123853 | 40.9128688 | 353,694 |  | 254,695 |  | 30355391.7604 | 0.8005 | Unsorted | -0.001 |  |  |  |  |  |  |  | 40.7288 | 0.0001 | 40.7287 | 40.7286 | 0.0893 | 0.008 | 0.2193 | 0 | 0 | 745305 | 15 | 0.2547 | 0.0632 | 40.2615 | 40.4646 | 40.6677 | 40.7222 | 40.8031 | 0.1354 | 41.0062 | 41.2094 | 0.1957 | 0.3537 | 5: 40.5955<br>10: 40.6175<br>40: 40.6986<br>60: 40.748<br>90: 40.8521<br>95: 40.8684 | Other (353,684) [739,574]<br>(NULL) [254,695]<br>40.89187241649303 [1,538]<br>40.1123853 [1,153]<br>40.89238451539139 [663] |
| **Longitude** | Float | Geographic Longitude | The longitude coordinate in decimal degrees, ranging from -77.520 to -73.701. Nulls are present in 25.47 % of records; the most common value is -73.86016845296459. | -77.5195844 | -73.7005968 | 353,996 |  | 254,695 |  | -55100392.9499 | 3.819 | Unsorted | -0.0008 |  |  |  |  |  |  |  | -73.93 | 0.0002 |  |  | 0.1635 | 0.0267 | -0.2212 | 745305 | 0 | 0 | 14 | 0.2547 | 0.0468 | -74.2533 | -74.1119 | -73.9705 | -73.9279 | -73.8763 | 0.0943 | -73.7349 | -73.5935 | 0.0964 | 0.354 | 5: -74.0787<br>10: -74.0022<br>40: -73.9454<br>60: -73.9106<br>90: -73.8191<br>95: -73.7839 | Other (353,986) [739,574]<br>(NULL) [254,695]<br>-73.86016845296459 [1,538]<br>-77.5195844 [1,153]<br>-73.8592161325675 [663] |
| **Location** | String | Geographic Point | String representation of latitude and longitude in the form "(lat, lon)". Nulls occur in 25.47 % of records; the most frequent point is (40.89187241649303, -73.86016845296459). | (40.1123853, -77.5195844) | (40.9128688, -73.9024731) | 375,772 |  | 254,695 | true |  |  | Unsorted | -0.001 | 0 | 40 | 19709659 | 19.7097 | 12.1918 | 148.6406 | 0.6186 |  |  |  |  |  |  |  |  |  |  |  | 0.2547 |  |  |  |  |  |  |  |  |  |  | 0.3758 |  | Other (375,762) [739,574]<br>(NULL) [254,695]<br>(40.89187241649303, -73.8… [1,538]<br>(40.1123853, -77.5195844) [1,153]<br>(40.89238451539139, -73.8… [663] |

*Attribution: Generated by qsv v11.0.2 describegpt
Command line: target/debug/qsv describegpt --all NYC_311_SR_2010-2020-sample-1M.csv --addl-cols --addl-cols-list everything --output nyc311-describegpt-everything.md
Prompt file: Default v6.3.0
Model: openai/gpt-oss-20b
LLM API URL: http://localhost:1234/v1
Timestamp: 2025-12-13T23:06:32.670825+00:00

WARNING: Label and Description generated by an LLM and may contain inaccuracies. Verify before using!*

## REASONING

We need to produce JSON mapping field names to label and description. Use summary statistics and frequency distribution to understand context.

Dataset fields: Let's list all unique field names from summary stats and freq. From summary stats: Unique Key, Created Date, Closed Date, Agency, Agency Name, Complaint Type, Descriptor, Location Type, Incident Zip, Incident Address, Street Name, Cross Street 1, Cross Street 2, Intersection Street 1, Intersection Street 2, Address Type, City, Landmark, Facility Type, Status, Due Date, Resolution Description, Resolution Action Updated Date, Community Board, BBL, Borough, X Coordinate (State Plane), Y Coordinate (State Plane), Open Data Channel Type, Park Facility Name, Park Borough, Vehicle Type, Taxi Company Borough, Taxi Pick Up Location, Bridge Highway Name, Bridge Highway Direction, Road Ramp, Bridge Highway Segment, Latitude, Longitude, Location.

Also maybe 'Open Data Channel Type' appears earlier. Yes.

We need to provide human-friendly label and description for each field, using summary stats and freq data to infer meaning.

We can create concise but descriptive labels: e.g., "Unique Key" label: "Record Identifier". Description: unique integer key identifying each complaint record; non-duplicate across dataset; high cardinality etc. Use summary stats: cardinality 1 million, uniqueness ratio 1. Good.

Similarly for Created Date: date/time when complaint was created in system. Observed many unique values but top 10 frequent show certain dates. Provide description accordingly.

Closed Date: date/time complaint closed or null if open; distribution shows some nulls etc.

Agency: code of agency responsible? Frequency shows NYPD, HPD etc. So label: "Reporting Agency" maybe "Primary Agency".

Agency Name: full name of agency. Description includes mapping from code to name.

Complaint Type: type of complaint (Noise, HEAT/HOT WATER...). Label: "Complaint Category". Provide description referencing NYC 311 categories.

Descriptor: more specific descriptor within complaint type. Many values like Loud Music/Party etc. Label: "Detailed Descriptor".

Location Type: type of location where issue occurs; e.g., residential building, street/sidewalk etc. Provide description.

Incident Zip: ZIP code of incident location. Note distribution includes nulls and many others.

Incident Address: full address string.

Street Name: primary street name; many unique values.

Cross Street 1 & Cross Street 2: cross streets at intersection; can be null.

Intersection Street 1/2: other streets intersecting? Might represent intersection details.

Address Type: type of address: ADDRESS, INTERSECTION etc.

City: city where incident located. In NYC boroughs but also other.

Landmark: nearby landmark or building name.

Facility Type: facility involved (e.g., DSNY Garage, Precinct). Provide description.

Status: status of complaint; values closed, pending, open etc.

Due Date: date/time by which action due? Many nulls.

Resolution Description: narrative description of resolution. Provide description.

Resolution Action Updated Date: last updated timestamp for resolution actions.

Community Board: community board number or unspecified.

BBL: Borough Block Lot identifier (NYC unique property ID). Provide description.

Borough: borough name where incident occurred; values BRONX, QUEENS etc.

X Coordinate (State Plane), Y Coordinate (State Plane): coordinates in State Plane coordinate system (probably NYC state plane NAD83?). Provide description.

Open Data Channel Type: how data submitted (PHONE, ONLINE etc).

Park Facility Name: name of park facility involved. Provide description.

Park Borough: borough of the park.

Vehicle Type: type of vehicle associated with incident.

Taxi Company Borough: borough where taxi company operates? maybe location of pick-up or base.

Taxi Pick Up Location: location where taxi was picked up.

Bridge Highway Name: name of bridge/highway involved in incident.

Bridge Highway Direction: direction of traffic at bridge.

Road Ramp: ramp type.

Bridge Highway Segment: segment identifier for highway/bridge.

Latitude, Longitude: geographic coordinates.

Location: string representation of lat/lon.

We also might need to mention null handling; cardinality etc. Provide description referencing frequency distribution.

Let's produce JSON. Ensure proper escaping. Use double quotes inside strings escaped. Provide each field object with label and description.

We'll generate for all fields.

Also consider 'Open Data Channel Type' appears twice? Already included.

Let's craft descriptions referencing summary stats: e.g., "Unique Key" - cardinality 1 million, uniqueness ratio 1; indicates unique record identifier.

For dates: mention range and mean date etc.

We must be concise but full description can have multiple sentences. Use the data to support.

Let's produce JSON with keys sorted maybe alphabetically or as appear? No requirement. Just mapping.

Ok let's craft.


## TOKEN USAGE

TokenUsage { prompt: 13155, completion: 3519, total: 16674, elapsed: 55790 }
---
# Description
**Description**

The dataset comprises one million distinct 311 complaint records collected from New York City between January 2010 and December 2020.  Each record is uniquely identified by an integer key, ensuring no duplicates within the file.  The majority of complaints were reported to police (≈26 %) or housing agencies (≈26 %), with noise‑related issues representing roughly nine percent of all reports.  Temporal analysis shows a peak in late‑2013 and early‑2014, while most complaint closures lag only a few days after creation.  Geographic coordinates are available for about three‑quarters of the records; the remaining quarter lacks latitude/longitude or address information.  The dataset contains several high‑cardinality categorical fields (e.g., street names, boroughs) that are dominated by an “Other” bucket, indicating many unique or rare values.

**Notable Characteristics**

- **Central tendency & spread:** Created and closed dates cluster around mid‑2015 with a slight left skew (negative skewness), while latitude/longitude exhibit wide geographic dispersion across all five boroughs.  
- **Distribution shape:** Most categorical fields show highly right‑skewed distributions, where the top 10 values account for less than half of the observations and an “Other” bucket captures the bulk of the remainder.  
- **Missingness:** Approximately 3 % of records have null closed dates; about 25 % lack geographic coordinates or address details; several fields (e.g., `Location Type`, `Community Board`) exhibit over 70 % missing values.  
- **Outliers & anomalies:** Extreme coordinate values exist outside the normal NYC bounds, and some timestamp fields contain out‑of‑range dates (1900–2100).  
- **Duplicates:** The unique key guarantees no row duplication; however, many records share identical complaint details (e.g., same descriptor and location), suggesting clustered reporting behavior.  
- **PII/PHI concerns:** Address strings, street names, and coordinates can be linked to specific locations and may indirectly identify individuals or businesses, so the dataset should be handled with privacy‑preserving measures.  
- **Data quality issues:** The prevalence of “Other” categories indicates many low‑frequency values that are difficult to analyze; non‑ASCII characters appear in some string fields; inconsistent naming conventions (e.g., `Borough` vs. `City`) may require harmonization before downstream use.


Generated by qsv v11.0.2 describegpt
Command line: target/debug/qsv describegpt -all NYC_311_SR_2010-2020-sample-1M.csv --addl-cols --addl-cols-list everything --output nyc311-describegpt-everything.md
Prompt file: Default v6.3.0
Model: openai/gpt-oss-20b
LLM API URL: http://localhost:1234/v1
Timestamp: 2025-12-13T23:06:58.006330+00:00

WARNING: Generated by an LLM and may contain inaccuracies. Verify before using!



## REASONING

We need to produce a dataset description based on summary statistics and frequency distribution, but not list per field. Provide 1-8 sentences overall description. Then Notable Characteristics bullet points: central tendency, spread, distribution shape, anomalies, patterns; outliers, missing values, duplicates, PII/PHI/PCI data; other data quality issues.

We must use the stats to talk about dataset as a whole. We can refer to unique key count 1M rows, created dates range 2010-2020, etc. Provide mention of distribution of complaint types, top categories. Mention high missingness in some fields (Closed Date ~2.86% nulls; Created Date mostly non-null but other many). The frequency shows majority 'Other' for many categorical fields. Provide mention of skewed distributions, heavy tails. Mention duplicates? Unique key ensures uniqueness.

Also note presence of PII: addresses, street names, coordinates etc - could be considered PII/PHI? Not exactly PHI. But may contain personal info. So note potential privacy concerns.

Also note high cardinality for some fields like 'Unique Key', 'Location' etc.

Outliers: Created date distribution mean 2015-11-10 but skewness negative -0.0857 indicates slight left skew; but maybe heavy tail? Also Closed Date similar.

Geographic coordinates spread across NYC, with large portion missing (25% missing). Outliers in X and Y coordinates also present.

Also mention data quality: many 'Other' categories, missing values, some fields have non-ASCII entries etc. Provide bullet list.

Add Attribution placeholder at bottom.

Make sure not to output field-level summaries or frequencies. So keep description generic.

Let's craft 6 sentences. Then bullet list.

Ok.
## TOKEN USAGE

TokenUsage { prompt: 15688, completion: 871, total: 16559, elapsed: 25326 }
---
# Tags
nyc_311  
noise_complaint  
heat_and_hot_water  
parking_violation  
street_condition  
building_plumbing_issue  
agency_response  
borough  
geospatial_location  
open_data  


Generated by qsv v11.0.2 describegpt
Command line: target/debug/qsv describegpt --all NYC_311_SR_2010-2020-sample-1M.csv --addl-cols --addl-cols-list everything --output nyc311-describegpt-everything.md
Prompt file: Default v6.3.0
Model: openai/gpt-oss-20b
LLM API URL: http://localhost:1234/v1
Timestamp: 2025-12-13T23:07:19.600359+00:00

WARNING: Generated by an LLM and may contain inaccuracies. Verify before using!



## REASONING

We need to produce tags. Use summary statistics and freq distribution. Determine top topics: NYC 311 complaints dataset; categories like noise, heat, parking, street conditions; agencies NYPD, HPD, DOT etc; boroughs; incidents; addresses; etc.

Tags should be lowercase with underscores. Max 10 tags in descending importance.

Consider the dataset as "NYC 311 complaint data". Tags: new york_city_311, public_safety_complaints, city_services, noise_complaints, building_conditions, transportation_incidents, boroughs, street_address, geospatial_data, agency_responses. But need to reflect content and frequency. The most frequent categories are "Other" for complaint type (56%) but we don't tag that. Top real categories: Noise - Residential, HEAT/HOT WATER etc.

Also high cardinality fields like location coordinates etc.

Thus tags: new_york_city_311, noise_complaints, heat_and_water, building_conditions, transportation_incidents, city_agencies, boroughs, geospatial_data, data_viz (maybe not). Let's pick 10:

1. new_york_city_311
2. noise_complaints
3. heat_and_hot_water
4. building_and_plumbing_issues
5. transportation_and_street_conditions
6. city_agencies_response
7. boroughs_and_locations
8. geospatial_coordinates
9. public_safety_complaints
10. open_data_platform

But maybe "open data platform" less relevant? Might use "data_set_of_public_services". Let's refine.

Important: Tags should be descriptive and helpful for search. Use domain terms: "311_service_requests", "nyc_311", "noise_complaint", "heat_issue", "parking_violation", "street_condition", "agency_response", "borough", "geospatial_location".

But limit to 10 tags, descending importance.

Let's decide order:

1. nyc_311
2. noise_complaint
3. heat_and_hot_water
4. parking_violation
5. street_condition
6. building_plumbing_issue
7. agency_response
8. borough
9. geospatial_location
10. open_data

But we might prefer "public_safety" over parking violation? Which is more important? The dataset has many complaint types, but noise and heat are top. Parking is third but still significant.

Let's produce tags: nyc_311, noise_complaint, heat_and_hot_water, parking_violation, street_condition, building_plumbing_issue, agency_response, borough, geospatial_location, open_data.

Now include attribution placeholder.

Make sure tags lower case underscore.

Add them as list lines? The instructions: "Add an Attribution with the placeholder "{GENERATED_BY_SIGNATURE}" after the Tags. If generating JSON format, add the Attribution as a separate key at the top level of the JSON object, after the Tags, otherwise add it at the bottom of the Tags in Markdown format."

We will output plain text tags each on new line? Probably just list with bullet or comma separated.

We'll produce:

```
nyc_311
noise_complaint
heat_and_hot_water
parking_violation
street_condition
building_plumbing_issue
agency_response
borough
geospatial_location
open_data

{GENERATED_BY_SIGNATURE}
```

But ensure no extra formatting. Let's provide as plain text lines.

Let's do that.
## TOKEN USAGE

TokenUsage { prompt: 15660, completion: 755, total: 16415, elapsed: 21592 }
---
