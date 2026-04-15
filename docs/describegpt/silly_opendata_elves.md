Generated using openai/gpt-oss-120b model against https://api.together/xyz/v1 running on a Macbook Pro M4 Max 64gb/Tahoe 26.3:

```bash
$ QSV_LLM_BASE_URL=http://localhost:1234/v1 qsv describegpt --all /tmp/NYC_311_SR_2010-2020-sample-1M.csv \
   --language "Silly Santa's Elves preparing the dataset for open data publishing on Christmas Day" \
   --model openai/gpt-oss-120b -o silly_opendata_elves.md
```
---
# Dictionary
| Name | Type | Label | Description | Min | Max | Cardinality | Enumeration | Null Count | Examples |
|------|------|-------|-------------|-----|-----|-------------|-------------|------------|----------|
| **Unique Key** | Integer | Unique Record Identifier | A system‑generated integer that uniquely identifies every 311 service request in the dataset. All one‑million rows have a distinct value (cardinality = 100 %). | 11465364 | 48478173 | 1,000,000 |  | 0 | <ALL_UNIQUE> |
| **Created Date** | DateTime | Request Creation Date‑Time | Timestamp when the 311 request was originally entered into the system. Values range from 2010‑01‑01 to 2020‑12‑23. The most common single‑day values each appear only a few hundred times; 99.73 % of records fall into an “Other” bucket, indicating a highly granular distribution of dates. | 2010-01-01T00:00:00+00:00 | 2020-12-23T01:25:51+00:00 | 841,014 |  | 0 | Other (841,004) [997,333]<br>01/24/2013 12:00:00 AM [347]<br>01/07/2014 12:00:00 AM [315]<br>01/08/2015 12:00:00 AM [283]<br>02/16/2015 12:00:00 AM [269] |
| **Closed Date** | DateTime | Request Closure Date‑Time | Timestamp indicating when the request was marked closed. Approximately 2.86 % of rows have a null value (still open). When present, dates span a wide historical range (1900‑2100) with many distinct values; the top ten dates each cover less than 0.04 % of records. | 1900-01-01T00:00:00+00:00 | 2100-01-01T00:00:00+00:00 | 688,837 |  | 28,619 | Other (688,827) [968,897]<br>(NULL) [28,619]<br>11/15/2010 12:00:00 AM [384]<br>11/07/2012 12:00:00 AM [329]<br>12/09/2010 12:00:00 AM [267] |
| **Agency** | String | Responsible Agency Code | Short code for the city agency handling the request (e.g., NYPD, HPD, DOT). The ten most frequent agencies account for roughly 64 % of records; an “Other (18)” category captures all remaining agency codes. | 3-1-1 | TLC | 28 |  | 0 | NYPD [265,116]<br>HPD [258,033]<br>DOT [132,462]<br>DSNY [81,606]<br>DEP [75,895] |
| **Agency Name** | String | Responsible Agency Full Name | Human‑readable name of the agency responsible for the request. The three largest agencies (NYC Police Department, Department of Housing Preservation & Development, Department of Transportation) together cover about 65 % of the dataset; an “Other (543)” bucket holds the rest. | 3-1-1 | Valuation Policy | 553 |  | 0 | New York City Police Depa… [265,038]<br>Department of Housing Pre… [258,019]<br>Department of Transportat… [132,462]<br>Other (543) [103,974]<br>Department of Environment… [75,895] |
| **Complaint Type** | String | Primary Complaint Category | Broad classification of the issue reported (e.g., Noise – Residential, Heat/Hot Water, Illegal Parking). The ten listed categories represent roughly 43 % of complaints; a large “Other (277)” group (56 %) contains many less common types. | ../../WEB-INF/web.xml;x= | ZTESTINT | 287 |  | 0 | Other (277) [563,561]<br>Noise - Residential [89,439]<br>HEAT/HOT WATER [56,639]<br>Illegal Parking [45,032]<br>Blocked Driveway [42,356] |
| **Descriptor** | String | Complaint Sub‑type (Descriptor) | More specific description of the complaint within the chosen type (e.g., Loud Music/Party, ENTIRE BUILDING, HEAT). The top ten descriptors cover about 33 % of rows; the remaining 67 % fall under “Other (1,382)”. | 1 Missed Collection | unknown odor/taste in drinking water (QA6) | 1,392 |  | 3,001 | Other (1,382) [674,871]<br>Loud Music/Party [93,646]<br>ENTIRE BUILDING [36,885]<br>HEAT [35,088]<br>No Access [31,631] |
| **Location Type** | String | Nature of the Reported Location | Category indicating the setting of the issue (e.g., RESIDENTIAL BUILDING, Street/Sidewalk, Store/Commercial). Residential Building‑type records dominate (≈25 %). A notable 23 % of rows have a null value, meaning the location type was not supplied. | 1-, 2- and 3- Family Home | Wooded Area | 162 |  | 239,131 | RESIDENTIAL BUILDING [255,562]<br>(NULL) [239,131]<br>Street/Sidewalk [145,653]<br>Residential Building/Hous… [92,765]<br>Street [92,190] |
| **Incident Zip** | String | ZIP Code of the Incident | Five‑digit postal code where the problem occurred. The most frequent ZIPs each appear in roughly 1 % of records; 82 % of rows are captured by an “Other (525)” bucket, and 5.5 % are null. | * | XXXXX | 535 |  | 54,978 | Other (525) [827,654]<br>(NULL) [54,978]<br>11226 [17,114]<br>10467 [14,495]<br>11207 [12,872] |
| **Incident Address** | String | Street Address of the Incident | Full street address reported for the incident. Only a handful of specific addresses appear repeatedly; 81.94 % of records are grouped under “Other (341,986)” and 17.47 % are null. | * * | west 155 street and edgecombe avenue | 341,996 |  | 174,700 | Other (341,986) [819,378]<br>(NULL) [174,700]<br>655 EAST  230 STREET [1,538]<br>78-15 PARSONS BOULEVARD [694]<br>672 EAST  231 STREET [663] |
| **Street Name** | String | Street Name | Name of the street on which the incident occurred. The most common street (BROADWAY) appears in under 1 % of rows; 78.72 % of records fall into the “Other (14,827)” category. | * | wyckoff avenue | 14,837 |  | 174,720 | Other (14,827) [787,222]<br>(NULL) [174,720]<br>BROADWAY [9,702]<br>GRAND CONCOURSE [5,851]<br>OCEAN AVENUE [3,946] |
| **Cross Street 1** | String | First Cross‑Street | The first intersecting street listed for the incident. Approximately 32 % of records are null; the top ten cross‑streets each constitute less than 1 % of the dataset, with “Other (16,228)” covering the vast majority. | 1 AVE | mermaid | 16,238 |  | 320,401 | Other (16,228) [623,317]<br>(NULL) [320,401]<br>BEND [12,562]<br>BROADWAY [8,548]<br>3 AVENUE [6,154] |
| **Cross Street 2** | String | Second Cross‑Street | The second intersecting street, if provided. Like Cross Street 1, about 32 % are null and the remaining values are highly dispersed, with an “Other (16,476)” bucket for most records. | 1 AVE | surf | 16,486 |  | 323,644 | Other (16,476) [626,168]<br>(NULL) [323,644]<br>BEND [12,390]<br>BROADWAY [8,833]<br>DEAD END [5,626] |
| **Intersection Street 1** | String | Primary Intersection Street | Street name for the primary intersection point of the incident. Nearly 77 % of rows have a null value; the top ten intersections together represent only about 2 % of the data, the rest being “Other (11,227)”. | 1 AVE | flatlands AVE | 11,237 |  | 767,422 | (NULL) [767,422]<br>Other (11,227) [215,482]<br>BROADWAY [3,761]<br>CARPENTER AVENUE [2,918]<br>BEND [2,009] |
| **Intersection Street 2** | String | Secondary Intersection Street | Street name for the secondary intersection point, if any. Again, about 77 % are null and the remaining values are sparsely distributed across an “Other (11,664)” bucket. | 1 AVE | glenwood RD | 11,674 |  | 767,709 | (NULL) [767,709]<br>Other (11,664) [216,748]<br>BROADWAY [3,462]<br>BEND [1,942]<br>2 AVENUE [1,690] |
| **Address Type** | String | Address Classification | Method used to describe the location (e.g., ADDRESS, INTERSECTION, BLOCKFACE, LATLONG). The majority (71 %) are plain addresses; 13 % are intersections; 12 % are null. | ADDRESS | PLACENAME | 6 | (NULL)<br>ADDRESS<br>BLOCKFACE<br>INTERSECTION<br>LATLONG<br>PLACENAME | 125,802 | ADDRESS [710,380]<br>INTERSECTION [133,361]<br>(NULL) [125,802]<br>BLOCKFACE [22,620]<br>LATLONG [7,421] |
| **City** | String | City Borough | Borough name (Brooklyn, New York, Bronx, etc.) where the incident took place. Brooklyn and New York together account for roughly 48 % of records; 6 % are null and another 17 % fall under “Other (372)”. | * | YORKTOWN HEIGHTS | 382 |  | 61,963 | BROOKLYN [296,254]<br>NEW YORK [189,069]<br>BRONX [181,168]<br>Other (372) [171,028]<br>(NULL) [61,963] |
| **Landmark** | String | Nearby Landmark | A known landmark near the incident location. Over 91 % of rows have no landmark; the most frequent listed landmark (EAST 230 STREET) appears in only 0.15 % of cases. | 1 AVENUE | ZULETTE AVENUE | 5,915 |  | 912,779 | (NULL) [912,779]<br>Other (5,905) [80,508]<br>EAST  230 STREET [1,545]<br>EAST  231 STREET [1,291]<br>BROADWAY [1,148] |
| **Facility Type** | String | Facility Classification | Type of public facility associated with the request (e.g., DSNY Garage, School, Precinct). The dominant value is “N/A” (62.8 %); “Precinct” covers 19.3 %; a small 14.5 % are null. | DSNY Garage | School District | 6 | (NULL)<br>DSNY Garage<br>N/A<br>Precinct<br>School<br>School District | 145,478 | N/A [628,279]<br>Precinct [193,259]<br>(NULL) [145,478]<br>DSNY Garage [32,310]<br>School [617] |
| **Status** | String | Current Request Status | Lifecycle stage of the request (Closed, Pending, Open, etc.). Closed requests dominate (95 %); the remaining statuses (Pending, Open, In Progress, Assigned, etc.) each represent less than 2 % of records. | Assigned | Unspecified | 10 | Assigned<br>Closed<br>Closed - Testing<br>Email Sent<br>In Progress<br>Open<br>Pending<br>Started<br>Unassigned<br>Unspecified | 0 | Closed [952,522]<br>Pending [20,119]<br>Open [12,340]<br>In Progress [7,841]<br>Assigned [6,651] |
| **Due Date** | DateTime | Target Resolution Due Date‑Time | The deadline by which the agency aims to resolve the request. Approximately 64.8 % of rows are null (no due date assigned). When populated, dates are spread across many years, with the top ten specific timestamps each covering less than 0.03 % of the data. | 1900-01-02T00:00:00+00:00 | 2021-06-17T16:34:13+00:00 | 345,077 |  | 647,794 | (NULL) [647,794]<br>Other (345,067) [350,849]<br>04/08/2015 10:00:58 AM [214]<br>05/02/2014 03:32:17 PM [183]<br>03/30/2018 10:10:39 AM [172] |
| **Resolution Description** | String | Outcome Narrative | Textual description of how the request was resolved. The ten most common narratives together account for about 35 % of rows; the overwhelming majority (53 %) are captured under “Other (1,206)”. | A DOB violation was issued for failing to comply with an existing Stop Work Order. | Your request was submitted to the Department of Homeless Services. The City?s outreach team will assess the homeless individual and offer appropriate assistance within 2 hours. If you asked to know the outcome of your request, you will get a call within 2 hours. No further status will be available through the NYC 311 App, 311, or 311 Online. | 1,216 |  | 20,480 | Other (1,206) [532,002]<br>The Police Department res… [91,408]<br>The Department of Housing… [72,962]<br>The Police Department res… [63,868]<br>Service Request status fo… [52,155] |
| **Resolution Action Updated Date** | DateTime | Resolution Update Timestamp | Date‑time when the resolution description was last modified. Only 1.5 % of records have a null value; the rest are widely distributed across many dates, with an “Other (690,304)” bucket for the bulk of entries. | 2009-12-31T01:35:00+00:00 | 2020-12-23T06:56:14+00:00 | 690,314 |  | 15,072 | Other (690,304) [982,378]<br>(NULL) [15,072]<br>11/15/2010 12:00:00 AM [385]<br>11/07/2012 12:00:00 AM [336]<br>12/09/2010 12:00:00 AM [273] |
| **Community Board** | String | Community Board District | Identifier of the NYC Community Board responsible for the area (e.g., 12 MANHATTAN). The ten most frequent boards cover roughly 22 % of records; 75 % fall under “Other (67)”. | 0 Unspecified | Unspecified STATEN ISLAND | 77 |  | 0 | Other (67) [751,635]<br>0 Unspecified [49,878]<br>12 MANHATTAN [29,845]<br>12 QUEENS [23,570]<br>01 BROOKLYN [21,714] |
| **BBL** | String | Borough‑Block‑Lot Identifier | Unique NYC parcel identifier. About 24 % of rows are null; the most common individual BBL values each appear in under 0.2 % of rows, with the remaining 75 % grouped under “Other (268,373)”. | 0000000000 | 5080470043 | 268,383 |  | 243,046 | Other (268,373) [751,031]<br>(NULL) [243,046]<br>2048330028 [1,566]<br>4068290001 [696]<br>4015110001 [664] |
| **Borough** | String | Borough | NYC borough where the incident occurred (Brooklyn, Queens, Manhattan, Bronx, Staten Island, Unspecified). The five named boroughs together represent 95 % of records; 5 % are marked “Unspecified”. | BRONX | Unspecified | 6 | BRONX<br>BROOKLYN<br>MANHATTAN<br>QUEENS<br>STATEN ISLAND<br>Unspecified | 0 | BROOKLYN [296,081]<br>QUEENS [228,818]<br>MANHATTAN [195,488]<br>BRONX [180,142]<br>Unspecified [49,878] |
| **X Coordinate (State Plane)** | Integer | State Plane X‑Coordinate | Projected X coordinate (in feet) using the New York State Plane coordinate system. Approximately 8.5 % of rows are null; the most common coordinate values each appear in less than 0.2 % of records, with 90.9 % falling into an “Other (102,546)” bucket. | 913281 | 1067220 | 102,556 |  | 85,327 | Other (102,546) [908,877]<br>(NULL) [85,327]<br>1022911 [1,568]<br>1037000 [701]<br>1023174 [675] |
| **Y Coordinate (State Plane)** | Integer | State Plane Y‑Coordinate | Projected Y coordinate (in feet) using the New York State Plane coordinate system. Like the X‑coordinate, about 8.5 % are null and the bulk (90.9 %) are classified as “Other (116,082)”. | 121152 | 271876 | 116,092 |  | 85,327 | Other (116,082) [908,868]<br>(NULL) [85,327]<br>264242 [1,566]<br>202363 [706]<br>211606 [665] |
| **Open Data Channel Type** | String | Submission Channel | Method by which the request was submitted (PHONE, ONLINE, MOBILE, UNKNOWN, OTHER). Phone calls account for half of all submissions; the rest are split among online, mobile, and unknown channels. | MOBILE | UNKNOWN | 5 | MOBILE<br>ONLINE<br>OTHER<br>PHONE<br>UNKNOWN | 0 | PHONE [497,606]<br>UNKNOWN [230,402]<br>ONLINE [177,334]<br>MOBILE [79,892]<br>OTHER [14,766] |
| **Park Facility Name** | String | Park Facility Name | Name of the park facility referenced, if applicable. Almost all records (99.3 %) are “Unspecified”; the few named parks (Central Park, Riverside Park, etc.) together cover less than 0.1 % of the data. | "Uncle" Vito F. Maranzano Glendale Playground | Zimmerman Playground | 1,889 |  | 0 | Unspecified [993,141]<br>Other (1,879) [5,964]<br>Central Park [261]<br>Riverside Park [136]<br>Prospect Park [129] |
| **Park Borough** | String | Park Borough | Borough containing the referenced park facility. The distribution mirrors the overall borough distribution, with Brooklyn, Queens, Manhattan, and Bronx each representing roughly 20–30 % of the few specified parks. | BRONX | Unspecified | 6 | BRONX<br>BROOKLYN<br>MANHATTAN<br>QUEENS<br>STATEN ISLAND<br>Unspecified | 0 | BROOKLYN [296,081]<br>QUEENS [228,818]<br>MANHATTAN [195,488]<br>BRONX [180,142]<br>Unspecified [49,878] |
| **Vehicle Type** | String | Vehicle Category | Category of vehicle associated with the request (e.g., Car Service, Ambulette / Paratransit, Green Taxi). Almost all rows (99.97 %) are null, indicating that vehicle type is rarely recorded. | Ambulette / Paratransit | Green Taxi | 5 | (NULL)<br>Ambulette / Paratransit<br>Car Service<br>Commuter Van<br>Green Taxi | 999,652 | (NULL) [999,652]<br>Car Service [317]<br>Ambulette / Paratransit [19]<br>Commuter Van [11]<br>Green Taxi [1] |
| **Taxi Company Borough** | String | Taxi Company Borough | Borough where the taxi company operating the request is based. Similar to Vehicle Type, 99.9 % are null; the few non‑null entries are spread across the five boroughs. | BRONX | Staten Island | 11 |  | 999,156 | (NULL) [999,156]<br>BROOKLYN [207]<br>QUEENS [194]<br>MANHATTAN [171]<br>BRONX [127] |
| **Taxi Pick Up Location** | String | Taxi Pick‑Up Location | Specific location where the taxi was requested to pick up the caller. The vast majority (99.2 %) are null; the remaining entries include airports, major stations, and an “Other” bucket. | 1 5 AVENUE MANHATTAN | YORK AVENUE AND EAST 70 STREET | 1,903 |  | 992,129 | (NULL) [992,129]<br>Other [4,091]<br>Other (1,893) [2,021]<br>JFK Airport [562]<br>Intersection [486] |
| **Bridge Highway Name** | String | Bridge/Highway Name | Name of the bridge or highway related to the incident. Over 99 % of rows are null; the most common named highways (Belt Pkwy, BQE/Gowanus Expwy, etc.) each constitute less than 0.03 % of records. | 145th St. Br - Lenox Ave | Willis Ave Br - 125th St/1st Ave | 68 |  | 997,711 | (NULL) [997,711]<br>Other (58) [851]<br>Belt Pkwy [276]<br>BQE/Gowanus Expwy [254]<br>Grand Central Pkwy [186] |
| **Bridge Highway Direction** | String | Bridge/Highway Direction | Directional qualifier for the bridge/highway (e.g., East/Long Island Bound). Nearly all rows (99.77 %) are null; the few populated values are uniformly distributed across a small set of directions. | Bronx Bound | Westbound/To Goethals Br | 50 |  | 997,691 | (NULL) [997,691]<br>Other (40) [1,064]<br>East/Long Island Bound [210]<br>North/Bronx Bound [208]<br>East/Queens Bound [197] |
| **Road Ramp** | String | Road Ramp Indicator | Indicates whether the location involves a ramp, roadway, or is unspecified. Approximately 99.77 % are null; when present, “Roadway” accounts for the majority of the small non‑null subset. | N/A | Roadway | 4 | (NULL)<br>N/A<br>Ramp<br>Roadway | 997,693 | (NULL) [997,693]<br>Roadway [1,731]<br>Ramp [555]<br>N/A [21] |
| **Bridge Highway Segment** | String | Bridge/Highway Segment | Specific segment identifier of a bridge or highway (e.g., Ramp, Roadway). Almost all rows are null; a tiny fraction (≈0.2 %) list a concrete segment name, with the remainder in an “Other (927)” bucket. | 1-1-1265963747 | Wythe Ave/Kent Ave (Exit 31) | 937 |  | 997,556 | (NULL) [997,556]<br>Other (927) [2,159]<br>Ramp [92]<br>Roadway [54]<br>Clove Rd/Richmond Rd (Exi… [23] |
| **Latitude** | Float | Latitude (Decimal Degrees) | Geographic latitude of the incident location in decimal degrees. About 25 % of rows are null; the most common specific latitudes each appear in less than 0.2 % of records, and 73.96 % fall under an “Other (353,684)” bucket. | 40.1123853 | 40.9128688 | 353,694 |  | 254,695 | Other (353,684) [739,574]<br>(NULL) [254,695]<br>40.89187241649303 [1,538]<br>40.1123853 [1,153]<br>40.89238451539139 [663] |
| **Longitude** | Float | Longitude (Decimal Degrees) | Geographic longitude of the incident location in decimal degrees. Mirrors the Latitude field: ~25 % null and 73.96 % grouped in an “Other (353,986)” category. | -77.5195844 | -73.7005968 | 353,996 |  | 254,695 | Other (353,986) [739,574]<br>(NULL) [254,695]<br>-73.86016845296459 [1,538]<br>-77.5195844 [1,153]<br>-73.8592161325675 [663] |
| **Location** | String | Location (Latitude, Longitude Tuple) | Combined latitude/longitude representation of the incident as a string tuple. About 25 % are null; the most frequent coordinate pairs each cover less than 0.2 % of rows, with 73.96 % categorized as “Other (375,762)”. | (40.1123853, -77.5195844) | (40.9128688, -73.9024731) | 375,772 |  | 254,695 | Other (375,762) [739,574]<br>(NULL) [254,695]<br>(40.89187241649303, -73.8… [1,538]<br>(40.1123853, -77.5195844) [1,153]<br>(40.89238451539139, -73.8… [663] |

*Attribution: Generated by qsv v12.0.0 describegpt
Command line: qsv describegpt --all /tmp/NYC_311_SR_2010-2020-sample-1M.csv --language Silly Santa's Elves preparing the dataset for open data publishing on Christmas Day --model openai/gpt-oss-120b -o silly_opendata_elves.md --fresh
Prompt file: Default v7.0.0
Model: openai/gpt-oss-120b
LLM API URL: https://api.together.xyz/v1
Language: 
Timestamp: 2025-12-24T16:19:08.658315+00:00

WARNING: Label and Description generated by an LLM and may contain inaccuracies. Verify before using!
*

## REASONING

We need to generate labels and descriptions for each of 41 fields, using context from summary stats and frequency distribution.

We must produce JSON where keys are field names exactly as given (e.g., "Unique Key") with label and description.

We need to be creative "Silly Santa'S Elves Preparing The Dataset For Open Data Publishing On Christmas Day Labels and Descriptions". But description should be informative.

We'll go through each field, use stats: e.g., Unique Key is unique integer, all unique. So description: unique identifier for each report.

Created Date: date of creation, range etc. Frequent dates list; many dates have few occurrences; many others.

Closed Date: date closed; many null (open). Provide description.

Agency: many agencies like NYPD, HPD etc. Provide description.

Agency Name: full name.

Complaint Type: categories (Noise Residential etc). Provide description.

Descriptor: subcategory.

Location Type: type of location (RESIDENTIAL BUILDING, etc) with null.

Incident Zip: zip code, many null and top values.

Incident Address: address string, many null.

Street Name: street name.

Cross Street 1: first cross street.

Cross Street 2.

Intersection Street 1.

Intersection Street 2.

Address Type: address classification.

City.

Landmark.

Facility Type: DSNY Garage etc, N/A.

Status: Closed, Pending etc.

Due Date: due date for resolution; many null.

Resolution Description: textual description of outcome; many "Other".

Resolution Action Updated Date: timestamp of last update; many null.

Community Board.

BBL: borough-block-lot number.

Borough.

X Coordinate (State Plane): coordinate possibly with null.

Y Coordinate.

Open Data Channel Type: how request submitted (PHONE, etc).

Park Facility Name: many unspecified.

Park Borough.

Vehicle Type: vehicle used (Car Service, Ambulette...), many null.

Taxi Company Borough.

Taxi Pick Up Location.

Bridge Highway Name.

Bridge Highway Direction.

Road Ramp.

Bridge Highway Segment.

Latitude, Longitude.

Location.

Make sure JSON valid, strings escaped as needed.

We'll generate label same as field but maybe with spaces and caps. Could also make more friendly: "Unique Key" -> "Unique Record Identifier".

Provide description sentences explaining meaning and notable distribution.

We'll include mention of cardinality, null percentage, top values.

We'll produce JSON with each field name as key.

Ok craft now.
## TOKEN USAGE

TokenUsage { prompt: 13458, completion: 3518, total: 16976, elapsed: 20964 }
---
# Description
**Silly Santa’s Elves Preparing The Dataset For Open Data Publishing On Christmas Day**

The North‑Pole data‑reindeer team has wrangled a jolly‑size trove of **1 000 000** 311‑style requests, each stamped with a glittering unique key.  Most columns sparkle with a mix of dazzling diversity and frosty blanks – roughly **60 % – 70 %** of records sit in “Other” buckets or are null, especially in date‑time fields and location descriptors.  A handful of agencies (NYPD, HPD, DOT) hog the spotlight, together accounting for about **66 %** of the merriment, while the rest are scattered across a long tail of tiny‑sized agency codes.  Complaint types are similarly frosted, with the top ten covering only **≈ 43 %** and the remaining **≈ 57 %** cloaked in an “Other (277)” veil.  Geographic fields (ZIP, street, latitude/longitude) are peppered with many unique values, yet the majority of coordinates hide in a massive “Other” bucket, mirroring the dataset’s high cardinality and low sparsity.  Dates of creation and closure are wildly spread across a decade‑plus range, yielding near‑neutral skewness and a fairly flat distribution.  The elves note that personally‑identifiable information is largely absent, but the sheer volume of nulls and “Other” categories suggests a need for careful cleaning before the sleigh‑ride to open‑data fame.

### Notable Characteristics
- 🎁 **Uniqueness:** Every record has a distinct primary key; no duplicate tickets to untangle.  
- ❄️ **Heavy “Other” & Null Presence:**  ≈ 70 % of values in many fields fall into “Other (N)” buckets or are null, especially for dates, ZIP codes, and geographic coordinates.  
- 🎄 **Skewed Agency & Complaint Distribution:** A few agencies and complaint types dominate (≈ 66 % & ≈ 43 % respectively), leaving long tails of rare categories.  
- ⛄ **Wide Date Range & Flat Shape:** Creation dates span 2010‑2020, closure dates even broader (1900‑2100), with near‑zero skew (‑0.08) indicating a relatively uniform spread.  
- 🌟 **High Cardinality Fields:** Street names, incident addresses, and lat/long have millions of distinct values, making indexing and summarisation a challenge.  
- 🧝 **Missing Data:** Several fields (e.g., `Closed Date`, `Due Date`, `Location Type`, `Landmark`) contain > 20 % nulls, potentially affecting downstream analyses.  
- 🎁 **No Sensitive PII/PHI/PCI:** The dataset lacks explicit personal identifiers, but careful handling of location data is still advisable.  
- 🎅 **Data Quality Alerts:** “Other (N)” categories hide many low‑frequency values; consider collapsing or recoding for robust modeling.

*Attribution:* Generated by qsv v12.0.0 describegpt
Command line: qsv describegpt --all /tmp/NYC_311_SR_2010-2020-sample-1M.csv --language Silly Santa's Elves preparing the dataset for open data publishing on Christmas Day --model openai/gpt-oss-120b -o silly_opendata_elves.md --fresh
Prompt file: Default v7.0.0
Model: openai/gpt-oss-120b
LLM API URL: https://api.together.xyz/v1
Language: 
Timestamp: 2025-12-24T16:19:14.020139+00:00

WARNING: Description generated by an LLM and may contain inaccuracies. Verify before using!



## REASONING

We need to generate a description in a whimsical "Silly Santa's Elves Preparing The Dataset For Open Data Publishing On Christmas Day". Use the summary stats to note overall aspects: 1M records, unique key all unique, many columns have high sparsity, top values dominate some fields, many nulls, etc. Then bullet list notable characteristics: high cardinality, many nulls, "Other" buckets dominating, main agencies distribution, etc. Provide attribution placeholder.

We must not output per-field stats, just overall description (1-8 sentences). After description, add "Notable Characteristics" bullet list. Use the whimsical language.

Let's craft.


## TOKEN USAGE

TokenUsage { prompt: 16218, completion: 783, total: 17001, elapsed: 5360 }
---
# Tags
- nyc_311_requests  
- city_issue_reports  
- municipal_response_analysis  
- borough_location_data  
- issue_category_breakdown  
- submission_channel_stats  
- resolution_text_summary  
- geographic_zip_distribution  
- time_series_2010_2020  
- holiday_open_data_release  

**Attribution:** Generated by qsv v12.0.0 describegpt
Command line: qsv describegpt --all /tmp/NYC_311_SR_2010-2020-sample-1M.csv --language Silly Santa's Elves preparing the dataset for open data publishing on Christmas Day --model openai/gpt-oss-120b -o silly_opendata_elves.md --fresh
Prompt file: Default v7.0.0
Model: openai/gpt-oss-120b
LLM API URL: https://api.together.xyz/v1
Language: 
Timestamp: 2025-12-24T16:19:25.945215+00:00

WARNING: Tags generated by an LLM and may contain inaccuracies. Verify before using!



## REASONING

We need to produce up to 10 unique tags, in Markdown format, descending order of importance (lowercase, underscores). Not use field names. Must be in language: "Silly Santa'S Elves Preparing The Dataset For Open Data Publishing On Christmas Day". That likely means we should phrase tags in that whimsical style? Possibly the tags should be in that language (maybe include whimsical words). But tags must be lowercase and use underscores, no spaces. The phrase instruction: "Make sure your tag choices are in this language: Silly Santa'S Elves preparing The Dataset For Open Data Publishing On Christmas Day". I think they want tags to reflect the whimsical style: maybe include "silly_santa_elves", "christmas_day_open_data", etc. Must be relevant to dataset contents. Choose up to 10 tags.

We need to think about importance based on summary stats and frequency distribution. Identify top themes: 311 service requests, NYC, complaints, agencies, location (boroughs), complaint types, resolution, open data channel.

Potential tags:
- nyc_311_service_requests
- city_complaint_data
- public_agency_responses
- borough_geolocation
- noise_heat_legal_parking (maybe combine)
- open_data_channel_phone_online
- resolution_descriptions
- incident_zip_codes
- temporal_trends_2010_2020
- christmas_dataset_release (since Santa theme)

But must be lowercase and underscores.

We need max 10, no more.

Order descending importance: most important first.

Probably:
1. nyc_311_service_requests
2. city_complaint_data
3. public_agency_responses
4. borough_geolocation
5. complaint_type_distribution
6. open_data_channel_usage
7. resolution_outcome_texts
8. incident_zip_codes
9. temporal_trends_2010_2020
10. christmas_dataset_release

Make sure no field names (like "agency", "complaint_type") appear? "agency" is a field name; they said not to use field names as tags. "public_agency_responses" includes "agency". Might be considered a field name. Better avoid. Could use "public_response_data". "complaint_type_distribution" includes "complaint". That's a field. Avoid. Use more generic: "service_issue_categories". That's okay.

Let's craft tags without exact field names: avoid "agency", "complaint", "status", "date", "zip", "borough". Could use synonyms.

Potential tags:
- nyc_311_requests
- city_issue_reports
- municipal_response_analysis
- borough_location_data
- issue_category_breakdown
- submission_channel_stats
- resolution_text_summary
- geographic_zip_distribution
- time_series_2010_2020
- holiday_open_data_release

All lowercase underscores. No field names directly (though "municipal" is okay, "issue" is generic). Good.

Now add attribution placeholder after tags. In markdown format, after list, add line: **Attribution:** {GENERATED_BY_SIGNATURE} maybe. They said add attribution after the Tags. So after list we can add a line.

Produce output: bullet list (Markdown). At most 10 tags.

Let's answer.
## TOKEN USAGE

TokenUsage { prompt: 16173, completion: 743, total: 16916, elapsed: 4594 }
---
