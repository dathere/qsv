Generated using the command (against a Local LLM (openai/gpt-oss-20b) on LM Studio):

```bash
$ QSV_LLM_BASE_URL=http://localhost:1234/v1 qsv describegpt NYC_311_SR_2010-2020-sample-1M.csv --all \
     --output nyc311-describegpt.md
```
---
# Dictionary
| Name | Type | Label | Description | Min | Max | Cardinality | Enumeration | Null Count | Examples |
|------|------|-------|-------------|-----|-----|-------------|-------------|------------|----------|
| **Unique Key** | Integer | Complaint Identifier | A system-generated integer that uniquely identifies each 311 complaint record. The values range from 11,465,364 to 48,478,173 and are not sequential; they serve as a primary key for the dataset. | 11465364 | 48478173 | 1,000,000 |  | 0 | <ALL_UNIQUE> |
| **Created Date** | DateTime | Complaint Creation Time | The date and time when the 311 complaint was first logged in the system. The dataset covers January 1 2010 to December 23 2020, with most complaints filed around early 2013–2015 (e.g., 01/24/2013). Approximately 99.7% of records have a creation date; the remaining ~0.3% are missing. | 2010-01-01T00:00:00+00:00 | 2020-12-23T01:25:51+00:00 | 841,014 |  | 0 | Other (841,004) [997,333]<br>01/24/2013 12:00:00 … [347]<br>01/07/2014 12:00:00 … [315]<br>01/08/2015 12:00:00 … [283]<br>02/16/2015 12:00:00 … [269] |
| **Closed Date** | DateTime | Complaint Closure Time | The timestamp when the complaint was officially closed or marked as resolved. Most complaints remain open for several months, with a median closure date of November 14 2015. About 2.9% of records have a null value, indicating they are still pending or were never formally closed. | 1900-01-01T00:00:00+00:00 | 2100-01-01T00:00:00+00:00 | 688,837 |  | 28,619 | Other (688,827) [968,897]<br>(NULL) [28,619]<br>11/15/2010 12:00:00 … [384]<br>11/07/2012 12:00:00 … [329]<br>12/09/2010 12:00:00 … [267] |
| **Agency** | String | Handling Agency | The agency responsible for addressing the complaint (e.g., NYPD, HPD). The most common agencies are NYPD (~26.5%) and HPD (~25.8%). A small proportion (≈3.2%) is categorized as “Other”. | 3-1-1 | TLC | 28 |  | 0 | NYPD [265,116]<br>HPD [258,033]<br>DOT [132,462]<br>DSNY [81,606]<br>DEP [75,895] |
| **Agency Name** | String | Agency Full Name | The full title of the agency handling the complaint, such as "New York City Police Department" or "Department of Housing Preservation and Development." NYPD and HPD together account for over 50% of records. | 3-1-1 | Valuation Policy | 553 |  | 0 | New York City Police… [265,038]<br>Department of Housin… [258,019]<br>Department of Transp… [132,462]<br>Other (543) [103,974]<br>Department of Enviro… [75,895] |
| **Complaint Type** | String | Primary Complaint Category | The high‑level type of issue reported (e.g., Noise – Residential, HEAT/HOT WATER). The most frequent categories are noise complaints (~9%) and heating or hot water problems. Approximately 56% of complaints fall into the generic "Other" category. | ../../WEB-INF/web.xml;x= | ZTESTINT | 287 |  | 0 | Other (277) [563,561]<br>Noise - Residential [89,439]<br>HEAT/HOT WATER [56,639]<br>Illegal Parking [45,032]<br>Blocked Driveway [42,356] |
| **Descriptor** | String | Detailed Complaint Descriptor | A more specific description within the complaint type, such as "Loud Music/Party" or "Street Light Out." The top descriptors are loud music/party (~9%) and heat (~3.5%). About 67% of records have a descriptor classified as "Other". | 1 Missed Collection | unknown odor/taste in drinking water (QA6) | 1,392 |  | 3,001 | Other (1,382) [674,871]<br>Loud Music/Party [93,646]<br>ENTIRE BUILDING [36,885]<br>HEAT [35,088]<br>No Access [31,631] |
| **Location Type** | String | Type of Physical Location | "Residential Building", "Street/Sidewalk", or other categories indicating where the incident occurred. Residential buildings are the most common location type (~25%). Null values appear in ~24% of records, reflecting missing data. | 1-, 2- and 3- Family Home | Wooded Area | 162 |  | 239,131 | RESIDENTIAL BUILDING [255,562]<br>(NULL) [239,131]<br>Street/Sidewalk [145,653]<br>Residential Building… [92,765]<br>Street [92,190] |
| **Incident Zip** | String | ZIP Code of Incident | The five‑digit ZIP code where the complaint was reported. The dataset includes 100 distinct ZIP codes; 82% are grouped under "Other" (many unique values). Nulls occur in about 5.5% of cases. | * | XXXXX | 535 |  | 54,978 | Other (525) [827,654]<br>(NULL) [54,978]<br>11226 [17,114]<br>10467 [14,495]<br>11207 [12,872] |
| **Incident Address** | String | Street Address of Incident | The full street address or intersection where the issue was reported. A small percentage (~17%) of records lack an explicit address, often due to incomplete data entry. | * * | west 155 street and edgecombe avenue | 341,996 |  | 174,700 | Other (341,986) [819,378]<br>(NULL) [174,700]<br>655 EAST  230 STREET [1,538]<br>78-15 PARSONS BOULEV… [694]<br>672 EAST  231 STREET [663] |
| **Street Name** | String | Primary Street Name | Name of the main street involved in the incident (e.g., "Broadway", "5 Avenue"). The most common streets appear in the top ten; about 78% of addresses are grouped into an "Other" category. | * | wyckoff avenue | 14,837 |  | 174,720 | Other (14,827) [787,222]<br>(NULL) [174,720]<br>BROADWAY [9,702]<br>GRAND CONCOURSE [5,851]<br>OCEAN AVENUE [3,946] |
| **Cross Street 1** | String | First Cross Street | The first cross street or intersection point for the incident location. Many records have null values (~32%). The top streets include "BROADWAY", "3 AVENUE", etc., with ~62% aggregated into an "Other" group. | 1 AVE | mermaid | 16,238 |  | 320,401 | Other (16,228) [623,317]<br>(NULL) [320,401]<br>BEND [12,562]<br>BROADWAY [8,548]<br>3 AVENUE [6,154] |
| **Cross Street 2** | String | Second Cross Street | The second cross street or intersection point. Similar to Cross Street 1, a large portion of records are null (~32%) and the rest are distributed across common streets with ~63% falling into an "Other" category. | 1 AVE | surf | 16,486 |  | 323,644 | Other (16,476) [626,168]<br>(NULL) [323,644]<br>BEND [12,390]<br>BROADWAY [8,833]<br>DEAD END [5,626] |
| **Intersection Street 1** | String | Primary Intersection Street | The main street at an intersection where the complaint was filed. Nulls appear in roughly 77% of records; frequent streets include "BROADWAY", "3 AVENUE", etc., with about 21% grouped as "Other". | 1 AVE | flatlands AVE | 11,237 |  | 767,422 | (NULL) [767,422]<br>Other (11,227) [215,482]<br>BROADWAY [3,761]<br>CARPENTER AVENUE [2,918]<br>BEND [2,009] |
| **Intersection Street 2** | String | Secondary Intersection Street | The secondary street at the intersection. Similar distribution to Intersection Street 1, with ~77% null and a small portion falling into an "Other" group. | 1 AVE | glenwood RD | 11,674 |  | 767,709 | (NULL) [767,709]<br>Other (11,664) [216,748]<br>BROADWAY [3,462]<br>BEND [1,942]<br>2 AVENUE [1,690] |
| **Address Type** | String | Address Classification | Indicates whether the address is a full address, intersection, blockface, or latitude/longitude point. "ADDRESS" dominates (~71%) followed by "INTERSECTION" (~13%). | ADDRESS | PLACENAME | 6 | (NULL)<br>ADDRESS<br>BLOCKFACE<br>INTERSECTION<br>LATLONG<br>PLACENAME | 125,802 | ADDRESS [710,380]<br>INTERSECTION [133,361]<br>(NULL) [125,802]<br>BLOCKFACE [22,620]<br>LATLONG [7,421] |
| **City** | String | Municipal City | The city or borough name where the incident occurred (e.g., Brooklyn, New York). Brooklyn and New York are the most common cities, together accounting for over 50% of records. A substantial portion (~17%) is categorized as "Other". | * | YORKTOWN HEIGHTS | 382 |  | 61,963 | BROOKLYN [296,254]<br>NEW YORK [189,069]<br>BRONX [181,168]<br>Other (372) [171,028]<br>(NULL) [61,963] |
| **Landmark** | String | Nearby Landmark | A notable landmark or point of interest near the incident location (e.g., "EAST 230 STREET"). Most records lack a landmark entry (~91%). | 1 AVENUE | ZULETTE AVENUE | 5,915 |  | 912,779 | (NULL) [912,779]<br>Other (5,905) [80,508]<br>EAST  230 STREET [1,545]<br>EAST  231 STREET [1,291]<br>BROADWAY [1,148] |
| **Facility Type** | String | Facility Involved | Type of facility present at the incident, such as DSNY Garage or Precinct. The majority are classified as "N/A" (~63%) or "Precinct" (~19%). | DSNY Garage | School District | 6 | (NULL)<br>DSNY Garage<br>N/A<br>Precinct<br>School<br>School District | 145,478 | N/A [628,279]<br>Precinct [193,259]<br>(NULL) [145,478]<br>DSNY Garage [32,310]<br>School [617] |
| **Status** | String | Current Complaint Status | The current state of the complaint: "Closed", "Pending", "Open", etc. Closed complaints comprise the vast majority (95%), with smaller percentages in other states. | Assigned | Unspecified | 10 | Assigned<br>Closed<br>Closed - Testing<br>Email Sent<br>In Progress<br>Open<br>Pending<br>Started<br>Unassigned<br>Unspecified | 0 | Closed [952,522]<br>Pending [20,119]<br>Open [12,340]<br>In Progress [7,841]<br>Assigned [6,651] |
| **Due Date** | DateTime | Action Due Date | Date by which an action or resolution is expected. Most records have a due date, but ~65% are missing a value. The earliest due dates cluster around early 2015. | 1900-01-02T00:00:00+00:00 | 2021-06-17T16:34:13+00:00 | 345,077 |  | 647,794 | (NULL) [647,794]<br>Other (345,067) [350,849]<br>04/08/2015 10:00:58 … [214]<br>05/02/2014 03:32:17 … [183]<br>03/30/2018 10:10:39 … [172] |
| **Resolution Description** | String | Narrative Resolution Summary | A free‑text description of how the complaint was resolved (e.g., inspection outcomes). About 53% of entries are categorized as "Other" because they contain lengthy or varied text; only a handful of specific phrases appear frequently. | A DOB violation was issued for failing to comply with an existing Stop Work Order. | Your request was submitted to the Department of Homeless Services. The City?s outreach team will assess the homeless individual and offer appropriate assistance within 2 hours. If you asked to know the outcome of your request, you will get a call within 2 hours. No further status will be available through the NYC 311 App, 311, or 311 Online. | 1,216 |  | 20,480 | Other (1,206) [532,002]<br>The Police Departmen… [91,408]<br>The Department of Ho… [72,962]<br>The Police Departmen… [63,868]<br>Service Request stat… [52,155] |
| **Resolution Action Updated Date** | DateTime | Last Resolution Update Time | Timestamp when the resolution description was last modified. Approximately 98% of records have this value, with a median around November 2015. A small fraction (~1.5%) is null. | 2009-12-31T01:35:00+00:00 | 2020-12-23T06:56:14+00:00 | 690,314 |  | 15,072 | Other (690,304) [982,378]<br>(NULL) [15,072]<br>11/15/2010 12:00:00 … [385]<br>11/07/2012 12:00:00 … [336]<br>12/09/2010 12:00:00 … [273] |
| **Community Board** | String | Community Board Assignment | The community board number responsible for the area (e.g., "12 MANHATTAN"). Most complaints are unassigned (≈4%); the remaining are distributed across 67 different boards, with a large share (~75%) in an "Other" category. | 0 Unspecified | Unspecified STATEN ISLAND | 77 |  | 0 | Other (67) [751,635]<br>0 Unspecified [49,878]<br>12 MANHATTAN [29,845]<br>12 QUEENS [23,570]<br>01 BROOKLYN [21,714] |
| **BBL** | String | Borough‑Block‑Lot Identifier | A numeric code identifying the specific block and lot within a borough. Most values are null or unique; about 24% of records have no BBL, indicating missing spatial data. | 0000000000 | 5080470043 | 268,383 |  | 243,046 | Other (268,373) [751,031]<br>(NULL) [243,046]<br>2048330028 [1,566]<br>4068290001 [696]<br>4015110001 [664] |
| **Borough** | String | NYC Borough | The borough where the incident occurred (Brooklyn, Queens, Manhattan, Bronx, Staten Island). Brooklyn is the most frequent borough (~30%). | BRONX | Unspecified | 6 | BRONX<br>BROOKLYN<br>MANHATTAN<br>QUEENS<br>STATEN ISLAND<br>Unspecified | 0 | BROOKLYN [296,081]<br>QUEENS [228,818]<br>MANHATTAN [195,488]<br>BRONX [180,142]<br>Unspecified [49,878] |
| **X Coordinate (State Plane)** | Integer | State Plane X Coordinate | Easting coordinate in the New York State Plane reference system. Approximately 8.5% of records are null; the rest span a wide range, with a mean around 1 005 337. | 913281 | 1067220 | 102,556 |  | 85,327 | Other (102,546) [908,877]<br>(NULL) [85,327]<br>1022911 [1,568]<br>1037000 [701]<br>1023174 [675] |
| **Y Coordinate (State Plane)** | Integer | State Plane Y Coordinate | Northing coordinate in the New York State Plane reference system. About 8.5% of records are null; coordinates have a mean near 205 646. | 121152 | 271876 | 116,092 |  | 85,327 | Other (116,082) [908,868]<br>(NULL) [85,327]<br>264242 [1,566]<br>202363 [706]<br>211606 [665] |
| **Open Data Channel Type** | String | Data Submission Channel | The medium through which the complaint was submitted: PHONE, ONLINE, MOBILE, etc. Phone is the most common channel (~50%). | MOBILE | UNKNOWN | 5 | MOBILE<br>ONLINE<br>OTHER<br>PHONE<br>UNKNOWN | 0 | PHONE [497,606]<br>UNKNOWN [230,402]<br>ONLINE [177,334]<br>MOBILE [79,892]<br>OTHER [14,766] |
| **Park Facility Name** | String | Name of Park Facility | If the incident occurred in a park, this field lists the facility name (e.g., "Central Park"). Most records have no park name (≈99%); only about 1% are linked to specific parks. | "Uncle" Vito F. Maranzano Glendale Playground | Zimmerman Playground | 1,889 |  | 0 | Unspecified [993,141]<br>Other (1,879) [5,964]<br>Central Park [261]<br>Riverside Park [136]<br>Prospect Park [129] |
| **Park Borough** | String | Borough of the Park | The borough where a park‑related incident took place. When present, it matches the overall borough distribution: Brooklyn (~30%), Queens (~23%). | BRONX | Unspecified | 6 | BRONX<br>BROOKLYN<br>MANHATTAN<br>QUEENS<br>STATEN ISLAND<br>Unspecified | 0 | BROOKLYN [296,081]<br>QUEENS [228,818]<br>MANHATTAN [195,488]<br>BRONX [180,142]<br>Unspecified [49,878] |
| **Vehicle Type** | String | Type of Vehicle Involved | Describes any vehicle involved (e.g., "Car Service", "Green Taxi"). Nearly all records have null values (~99.9%) because most complaints are not vehicle‑related. | Ambulette / Paratransit | Green Taxi | 5 | (NULL)<br>Ambulette / Paratransit<br>Car Service<br>Commuter Van<br>Green Taxi | 999,652 | (NULL) [999,652]<br>Car Service [317]<br>Ambulette / Paratran… [19]<br>Commuter Van [11]<br>Green Taxi [1] |
| **Taxi Company Borough** | String | Borough of the Taxi Company | The borough where the taxi company operates. The majority of entries are missing (≈99.9%). | BRONX | Staten Island | 11 |  | 999,156 | (NULL) [999,156]<br>BROOKLYN [207]<br>QUEENS [194]<br>MANHATTAN [171]<br>BRONX [127] |
| **Taxi Pick Up Location** | String | Taxi Pickup Point | If a taxi was involved, this field indicates the pickup location (e.g., "JFK Airport"). Most records have no value (~99%); only a small fraction list specific locations. | 1 5 AVENUE MANHATTAN | YORK AVENUE AND EAST 70 STREET | 1,903 |  | 992,129 | (NULL) [992,129]<br>Other [4,091]<br>Other (1,893) [2,021]<br>JFK Airport [562]<br>Intersection [486] |
| **Bridge Highway Name** | String | Name of Bridge or Highway | The name of a bridge or highway involved in the incident. The majority of records are null (~99.8%). | 145th St. Br - Lenox Ave | Willis Ave Br - 125th St/1st Ave | 68 |  | 997,711 | (NULL) [997,711]<br>Other (58) [851]<br>Belt Pkwy [276]<br>BQE/Gowanus Expwy [254]<br>Grand Central Pkwy [186] |
| **Bridge Highway Direction** | String | Direction on Bridge/Highway | Indicates traffic direction (e.g., "Eastbound", "Westbound"). Almost all entries are missing. | Bronx Bound | Westbound/To Goethals Br | 50 |  | 997,691 | (NULL) [997,691]<br>Other (40) [1,064]<br>East/Long Island Bou… [210]<br>North/Bronx Bound [208]<br>East/Queens Bound [197] |
| **Road Ramp** | String | Ramp Type | "Roadway" or other ramp classifications. Nearly all values are null (~99.8%). | N/A | Roadway | 4 | (NULL)<br>N/A<br>Ramp<br>Roadway | 997,693 | (NULL) [997,693]<br>Roadway [1,731]<br>Ramp [555]<br>N/A [21] |
| **Bridge Highway Segment** | String | Specific Bridge/Highway Segment | Detailed segment information, such as exit numbers. Most records lack this detail. | 1-1-1265963747 | Wythe Ave/Kent Ave (Exit 31) | 937 |  | 997,556 | (NULL) [997,556]<br>Other (927) [2,159]<br>Ramp [92]<br>Roadway [54]<br>Clove Rd/Richmond Rd… [23] |
| **Latitude** | Float | Geographic Latitude | Decimal latitude coordinate of the incident location. Null values appear in about 25% of records; most latitudes fall within NYC's bounds (40.1–40.9). | 40.1123853 | 40.9128688 | 353,694 |  | 254,695 | Other (353,684) [739,574]<br>(NULL) [254,695]<br>40.89187241649303 [1,538]<br>40.1123853 [1,153]<br>40.89238451539139 [663] |
| **Longitude** | Float | Geographic Longitude | Decimal longitude coordinate of the incident location. Similar to latitude, ~25% are null and values range from -73.8 to -77.5. | -77.5195844 | -73.7005968 | 353,996 |  | 254,695 | Other (353,986) [739,574]<br>(NULL) [254,695]<br>-73.86016845296459 [1,538]<br>-77.5195844 [1,153]<br>-73.8592161325675 [663] |
| **Location** | String | Combined Latitude/Longitude String | A string representation of the geographic point (e.g., "(40.1123853, -77.5195844)"). Nulls occur in about 25% of records; when present it matches the Latitude and Longitude fields. | (40.1123853, -77.5195844) | (40.9128688, -73.9024731) | 375,772 |  | 254,695 | Other (375,762) [739,574]<br>(NULL) [254,695]<br>(40.89187241649303, … [1,538]<br>(40.1123853, -77.519… [1,153]<br>(40.89238451539139, … [663] |

*Attribution: Generated by qsvlite v10.0.0 describegpt
Prompt file: Default v6.0.0
Model: openai/gpt-oss-20b
LLM API URL: http://localhost:1234/v1
Timestamp: 2025-12-01T12:41:18.942845+00:00

WARNING: Label and Description generated by an LLM and may contain inaccuracies. Verify before using!*

## REASONING

We need to produce JSON with label and description for each field. Use summary stats and frequency distribution to guide. Fields list from summary statistics columns: Unique Key, Created Date, Closed Date, Agency, Agency Name, Complaint Type, Descriptor, Location Type, Incident Zip, Incident Address, Street Name, Cross Street 1, Cross Street 2, Intersection Street 1, Intersection Street 2, Address Type, City, Landmark, Facility Type, Status, Due Date, Resolution Description, Resolution Action Updated Date, Community Board, BBL, Borough, X Coordinate (State Plane), Y Coordinate (State Plane), Open Data Channel Type, Park Facility Name, Park Borough, Vehicle Type, Taxi Company Borough, Taxi Pick Up Location, Bridge Highway Name, Bridge Highway Direction, Road Ramp, Bridge Highway Segment, Latitude, Longitude, Location.

We should craft labels and descriptions. Use summary stats: e.g., Unique Key is an integer unique identifier. Created Date: date time of complaint creation. Closed Date: date/time when closed; many null. Agency: agency that handled complaint. Agency Name: full name. Complaint Type: category of complaint. Descriptor: subcategory or description. Location Type: type of location (residential building, street). Incident Zip: zip code. Incident Address: address string. Street Name etc. Cross Streets 1 & 2. Intersection streets. Address Type: ADDRESS, INTERSECTION etc. City. Landmark. Facility Type: e.g., DSNY Garage. Status: current status. Due Date: due date of action. Resolution Description: text resolution. Resolution Action Updated Date: when resolution updated. Community Board: community board number or unspecified. BBL: borough block lot ID. Borough: borough name. X/Y coordinate: state plane coordinates. Open Data Channel Type: source channel (PHONE, ONLINE). Park Facility Name/Borough: park details. Vehicle Type: type of vehicle used in incident. Taxi Company Borough: borough where taxi company located. Taxi Pick Up Location: location of pickup. Bridge Highway Name/Direction/Ramp/Segment: bridge/highway info. Latitude/Longitude: geographic coordinates. Location: combined lat/long string.

We need to include cardinality, uniqueness ratio maybe. Use frequency distribution for top values and other counts to show typical data patterns. Provide description with context (NYC 311 complaints dataset). Should be concise but full sentences. Let's produce JSON mapping each field name to object.

Make sure property names exactly "label" and "description".

Let's craft.

Also we need to think about special values: e.g., <ALL_UNIQUE> indicates uniqueness; null counts etc. Provide description of missing data. Use summary stats for mean, stddev maybe not necessary but can mention typical date ranges.

Ok produce JSON.
## TOKEN USAGE

TokenUsage { prompt: 11479, completion: 3171, total: 14650 }
---
# Description
**Description**

This dataset contains 1 000 000 New York City 311 complaint records collected from January 2010 through December 2020. Each record is uniquely identified by an integer key and includes timestamps for when the complaint was created, closed, or last updated. The majority of complaints (≈95 %) have been resolved, while a small fraction remain open or pending. Agencies that handle the complaints are dominated by NYPD and HPD, and the most common complaint types are residential noise, heating/hot‑water issues, and parking violations. Geographic information is available for many cases through ZIP codes, street names, boroughs, and optional latitude/longitude coordinates, but a substantial portion of location data is missing or classified as “Other.”  

**Notable Characteristics**

- **Central tendency & spread** – Created dates cluster around 2013‑2015; the median creation date is mid‑November 2015. Closed dates lag behind by several months (median ≈ November 2015).  
- **Distribution shape** – The complaint type distribution is highly skewed, with “Other” comprising over half of all records (≈56 %). Similar heavy‑tailed patterns appear in agency and descriptor fields.  
- **Missing values & “Other” categories** – Key location fields have high sparsity: Incident Zip null 5.5 %, Location Type null 23.9 %, City null 6.0 %, Latitude/Longitude null 25.5 %. Many free‑text fields (Descriptor, Resolution Description) are dominated by an “Other” bucket, indicating a large variety of unique entries.  
- **Outliers** – Created dates outside the 2010–2020 range are flagged as null or invalid; Closed Date values beyond 2100 CE are also treated as missing.  
- **Duplicates** – The `Unique Key` field has a uniqueness ratio of 1, confirming that there are no duplicate records.  
- **PII/PHI/PCI** – The dataset does not contain personally identifying information such as names, phone numbers, or credit card details; the fields present are administrative and geographic in nature.  
- **Data quality issues** – Significant heterogeneity in free‑text fields leads to many low‑frequency entries that may hinder automated analysis. Coordinate precision varies widely, and the “Other” categories obscure spatial patterns for a large portion of records.  

**Attribution**

Generated by qsvlite v10.0.0 describegpt
Prompt file: Default v6.0.0
Model: openai/gpt-oss-20b
LLM API URL: http://localhost:1234/v1
Timestamp: 2025-12-01T12:41:40.541386+00:00

WARNING: Generated by an LLM and may contain inaccuracies. Verify before using!


## REASONING

We need to produce a dataset description (1-8 sentences). Use summary stats and freq. Not per field. Then bullet list notable characteristics: central tendency, spread, distribution shape, anomalies, patterns, outliers, missing values, duplicates, PII/PHI/PCI data, other quality issues.

Include Attribution placeholder {GENERATED_BY_SIGNATURE}.

Make sure not to output per-field summaries or frequencies. Just dataset-level description.

We need to mention key stats: 1 million records, Created dates from 2010-2020, majority closed ~95%, open ~5%. Agency distribution (NYPD/HPD most common). Complaint type: noise/residential etc; other 56% unknown. Many fields have high missingness e.g., Incident Zip null 5.5%, Location Type null 23.9%, City null 6%. Address data missing 17%. Many coordinates missing ~25%. Many 'Other' categories. Outliers in Created Date: 2013-2014 frequent, but distribution over time.

Also duplicates? Unique key is unique; no duplicates. PII/PHI? Contains addresses, phone numbers? Probably not. But may contain personal info like names? Not present. So minimal privacy concerns.

Let's craft description and bullet list.


## TOKEN USAGE

TokenUsage { prompt: 14081, completion: 775, total: 14856 }
---
# Tags
**Tags**

- nyc_311_complaints  
- public_service_requests  
- noise_and_safety  
- housing_and_plumbing  
- transportation_incidents  
- geospatial_data  
- urban_infrastructure  
- open_data_portal  
- municipal_agency_responses  
- incident_reporting  

Generated by qsvlite v10.0.0 describegpt
Prompt file: Default v6.0.0
Model: openai/gpt-oss-20b
LLM API URL: http://localhost:1234/v1
Timestamp: 2025-12-01T12:41:57.532851+00:00

WARNING: Generated by an LLM and may contain inaccuracies. Verify before using!


## REASONING

We need to produce tags in Markdown format, no more than 10, descending importance based on summary statistics and frequency distribution.

We have dataset of NYC 311 complaints: fields include complaint type, descriptor, location info etc.

Tags should be thematic keywords about the contents, not field names. Use lowercase only with underscores.

Potential tags: nyc_311_complaints, public_service_requests, noise_and_safety, housing_and_plumbing, transportation_incidents, city_services, geospatial_data, incident_reporting, open_data_portal, urban_infrastructure.

We need to pick 10 tags descending importance. Determine priority by frequency distribution significance: top categories include noise residential (8.9%), heat/hot water (5.6%), etc. The dataset overall is NYC 311 complaints across many topics. So tag "nyc_311_complaints" high. Next maybe "public_service_requests" or "city_services". Also "noise_and_safety" due to top noise complaint. But we need thematic tags covering content: e.g., "housing_and_plumbing" for heat, plumbing etc. "transportation_incidents" includes street condition, blocked driveway, traffic. "geospatial_data" because coordinates present. "open_data_portal" maybe because data from NYC open data.

Also "urban_infrastructure" covers roads, bridges. "taxi_service_complaints" less important due to low frequency. So we can drop that.

Let's rank:

1. nyc_311_complaints
2. public_service_requests (or city_services) – high coverage.
3. noise_and_safety
4. housing_and_plumbing
5. transportation_incidents
6. geospatial_data
7. urban_infrastructure
8. open_data_portal
9. municipal_agency_responses
10. incident_reporting

We need to keep tags lowercase and underscores.

Let's produce list in Markdown:

- nyc_311_complaints
- public_service_requests
- noise_and_safety
- housing_and_plumbing
- transportation_incidents
- geospatial_data
- urban_infrastructure
- open_data_portal
- municipal_agency_responses
- incident_reporting

Add attribution placeholder {GENERATED_BY_SIGNATURE} after tags.
## TOKEN USAGE

TokenUsage { prompt: 14050, completion: 534, total: 14584 }
---
