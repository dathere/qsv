Generated using a Local LLM (openai/gpt-oss-20b) on LM Studio 0.3.33 Build 1 running on a Macbook Pro M4 Max 64gb/Tahoe 26.2:

```bash
$ QSV_LLM_BASE_URL=http://localhost:1234/v1 qsv describegpt NYC_311_SR_2010-2020-sample-1M.csv --all \
     --output nyc311-describegpt.md
```
---
# Dictionary
| Name | Type | Label | Description | Min | Max | Cardinality | Enumeration | Null Count | Examples |
|------|------|-------|-------------|-----|-----|-------------|-------------|------------|----------|
| **Unique Key** | Integer | Unique Identifier | A system‑generated integer that uniquely identifies each 311 complaint record. It is an auto‑incrementing primary key with one million distinct values and no missing entries (null count = 0). The uniqueness ratio is 1, indicating perfect uniqueness across the dataset. | 11465364 | 48478173 | 1,000,000 |  | 0 | <ALL_UNIQUE> |
| **Created Date** | DateTime | Submission Timestamp | The date and time when a complaint was submitted to NYC 311. Stored as UTC datetime, it ranges from January 1, 2010 to December 23, 2020. The most common dates fall in early 2013‑2015; the top ten dates account for only ~0.27 % of all records, while the remaining 99.73 % are distributed across the full range. | 2010-01-01T00:00:00+00:00 | 2020-12-23T01:25:51+00:00 | 841,014 |  | 0 | Other (841,004) [997,333]<br>01/24/2013 12:00:00 AM [347]<br>01/07/2014 12:00:00 AM [315]<br>01/08/2015 12:00:00 AM [283]<br>02/16/2015 12:00:00 AM [269] |
| **Closed Date** | DateTime | Resolution Timestamp | The datetime when a complaint was closed or resolved. About 2.86 % of records have non‑null values; the rest (≈97.14 %) remain open. The dates span from January 1, 1900 to January 1, 2100, with most resolutions occurring within a few days after submission. | 1900-01-01T00:00:00+00:00 | 2100-01-01T00:00:00+00:00 | 688,837 |  | 28,619 | Other (688,827) [968,897]<br>(NULL) [28,619]<br>11/15/2010 12:00:00 AM [384]<br>11/07/2012 12:00:00 AM [329]<br>12/09/2010 12:00:00 AM [267] |
| **Agency** | String | Reporting Agency Code | A two‑letter code identifying the agency that handled the complaint (e.g., NYPD, HPD). The top codes are NYPD (26.5 %) and HPD (25.8 %). There are ten distinct values plus an “Other” category accounting for 3.2 % of records. | 3-1-1 | TLC | 28 |  | 0 | NYPD [265,116]<br>HPD [258,033]<br>DOT [132,462]<br>DSNY [81,606]<br>DEP [75,895] |
| **Agency Name** | String | Agency Full Name | The full name of the agency responsible for processing the complaint. The most frequent agencies are NYPD, Department of Housing Preservation and Development, and Department of Transportation. An “Other” category represents about 10 % of entries. | 3-1-1 | Valuation Policy | 553 |  | 0 | New York City Police Depa… [265,038]<br>Department of Housing Pre… [258,019]<br>Department of Transportat… [132,462]<br>Other (543) [103,974]<br>Department of Environment… [75,895] |
| **Complaint Type** | String | Complaint Category | High‑level classification of the issue (e.g., Noise – Residential, HEAT/HOT WATER). The top ten categories cover only ~44 % of complaints; an “Other” category comprises 56.4 %. This field has 287 distinct values. | ../../WEB-INF/web.xml;x= | ZTESTINT | 287 |  | 0 | Other (277) [563,561]<br>Noise - Residential [89,439]<br>HEAT/HOT WATER [56,639]<br>Illegal Parking [45,032]<br>Blocked Driveway [42,356] |
| **Descriptor** | String | Specific Issue Descriptor | A finer‑grained description within a complaint type (e.g., Loud Music/Party, Pothole). The top ten descriptors cover ~32 % of records; the remaining 67.5 % fall under “Other.” | 1 Missed Collection | unknown odor/taste in drinking water (QA6) | 1,392 |  | 3,001 | Other (1,382) [674,871]<br>Loud Music/Party [93,646]<br>ENTIRE BUILDING [36,885]<br>HEAT [35,088]<br>No Access [31,631] |
| **Location Type** | String | Location Category | The general class of location where the incident occurred (Residential Building, Street/Sidewalk, etc.). Residential Building and NULL are the two most common categories, together accounting for ~49 % of records. | 1-, 2- and 3- Family Home | Wooded Area | 162 |  | 239,131 | RESIDENTIAL BUILDING [255,562]<br>(NULL) [239,131]<br>Street/Sidewalk [145,653]<br>Residential Building/Hous… [92,765]<br>Street [92,190] |
| **Incident Zip** | String | ZIP Code of Incident | 5‑digit ZIP code of the complaint location; NULL values appear in about 5.5 %. The top three ZIPs are 11226, 10467, and 11207. | * | XXXXX | 535 |  | 54,978 | Other (525) [827,654]<br>(NULL) [54,978]<br>11226 [17,114]<br>10467 [14,495]<br>11207 [12,872] |
| **Incident Address** | String | Street Address of Incident | Full street address where the incident was reported. Approximately 17 % of records have a non‑null value; common addresses include “655 EAST 230 STREET” and “78‑15 PARSONS BOULEVARD.” | * * | west 155 street and edgecombe avenue | 341,996 |  | 174,700 | Other (341,986) [819,378]<br>(NULL) [174,700]<br>655 EAST  230 STREET [1,538]<br>78-15 PARSONS BOULEVARD [694]<br>672 EAST  231 STREET [663] |
| **Street Name** | String | Primary Street Name | Name of the main street in the incident location. The most frequent streets are BROADWAY, GRAND CONCOURSE, and OCEAN AVENUE; about 17 % of records contain a non‑null value. | * | wyckoff avenue | 14,837 |  | 174,720 | Other (14,827) [787,222]<br>(NULL) [174,720]<br>BROADWAY [9,702]<br>GRAND CONCOURSE [5,851]<br>OCEAN AVENUE [3,946] |
| **Cross Street 1** | String | Cross Street 1 | Name of the first cross street at the incident location. The field is NULL in ~32 % of records, with BROADWAY and BEND among the top values. | 1 AVE | mermaid | 16,238 |  | 320,401 | Other (16,228) [623,317]<br>(NULL) [320,401]<br>BEND [12,562]<br>BROADWAY [8,548]<br>3 AVENUE [6,154] |
| **Cross Street 2** | String | Cross Street 2 | Name of the second cross street; also NULL in ~32 % of records. The most common entries are BROADWAY and DEAD END. | 1 AVE | surf | 16,486 |  | 323,644 | Other (16,476) [626,168]<br>(NULL) [323,644]<br>BEND [12,390]<br>BROADWAY [8,833]<br>DEAD END [5,626] |
| **Intersection Street 1** | String | Intersection Street 1 | First street in an intersection-based address. This field is NULL in ~76 % of cases; when populated, the top value is BROADWAY. | 1 AVE | flatlands AVE | 11,237 |  | 767,422 | (NULL) [767,422]<br>Other (11,227) [215,482]<br>BROADWAY [3,761]<br>CARPENTER AVENUE [2,918]<br>BEND [2,009] |
| **Intersection Street 2** | String | Intersection Street 2 | Second street in an intersection-based address; NULL in ~77 % of records. The most frequent entry is also BROADWAY. | 1 AVE | glenwood RD | 11,674 |  | 767,709 | (NULL) [767,709]<br>Other (11,664) [216,748]<br>BROADWAY [3,462]<br>BEND [1,942]<br>2 AVENUE [1,690] |
| **Address Type** | String | Address Classification | Indicates the type of address used (ADDRESS, INTERSECTION, BLOCKFACE, LATLONG, PLACENAME). ADDRESS dominates at 71 %, followed by INTERSECTION (13 %). | ADDRESS | PLACENAME | 6 | (NULL)<br>ADDRESS<br>BLOCKFACE<br>INTERSECTION<br>LATLONG<br>PLACENAME | 125,802 | ADDRESS [710,380]<br>INTERSECTION [133,361]<br>(NULL) [125,802]<br>BLOCKFACE [22,620]<br>LATLONG [7,421] |
| **City** | String | City / Borough | The city name within New York State; NULL in ~6.2 % of records. Brooklyn and New York are the most common, each covering about 18–30 %. | * | YORKTOWN HEIGHTS | 382 |  | 61,963 | BROOKLYN [296,254]<br>NEW YORK [189,069]<br>BRONX [181,168]<br>Other (372) [171,028]<br>(NULL) [61,963] |
| **Landmark** | String | Nearby Landmark | A notable landmark close to the incident location. The field is NULL for ~91 %. When populated, common values include “EAST 230 STREET” and “BROADWAY.” | 1 AVENUE | ZULETTE AVENUE | 5,915 |  | 912,779 | (NULL) [912,779]<br>Other (5,905) [80,508]<br>EAST  230 STREET [1,545]<br>EAST  231 STREET [1,291]<br>BROADWAY [1,148] |
| **Facility Type** | String | Facility Category | Indicates if a specific facility was involved (e.g., DSNY Garage, Precinct). The majority of entries are N/A (63 %). Other categories appear rarely. | DSNY Garage | School District | 6 | (NULL)<br>DSNY Garage<br>N/A<br>Precinct<br>School<br>School District | 145,478 | N/A [628,279]<br>Precinct [193,259]<br>(NULL) [145,478]<br>DSNY Garage [32,310]<br>School [617] |
| **Status** | String | Complaint Status | Current status of the complaint: Closed (95.3 %), Pending (2 %), Open (1.2 %) and other minor states. The field has no null values. | Assigned | Unspecified | 10 | Assigned<br>Closed<br>Closed - Testing<br>Email Sent<br>In Progress<br>Open<br>Pending<br>Started<br>Unassigned<br>Unspecified | 0 | Closed [952,522]<br>Pending [20,119]<br>Open [12,340]<br>In Progress [7,841]<br>Assigned [6,651] |
| **Due Date** | DateTime | Resolution Deadline | The date by which a complaint should be resolved. About 65 % of records have NULL; the most frequent dates fall in early 2015–2018. | 1900-01-02T00:00:00+00:00 | 2021-06-17T16:34:13+00:00 | 345,077 |  | 647,794 | (NULL) [647,794]<br>Other (345,067) [350,849]<br>04/08/2015 10:00:58 AM [214]<br>05/02/2014 03:32:17 PM [183]<br>03/30/2018 10:10:39 AM [172] |
| **Resolution Description** | String | Resolution Narrative | Textual description of how the complaint was addressed, often containing agency‑specific language. Only ~2 % of entries are NULL; the top narrative is a police response note. | A DOB violation was issued for failing to comply with an existing Stop Work Order. | Your request was submitted to the Department of Homeless Services. The City?s outreach team will assess the homeless individual and offer appropriate assistance within 2 hours. If you asked to know the outcome of your request, you will get a call within 2 hours. No further status will be available through the NYC 311 App, 311, or 311 Online. | 1,216 |  | 20,480 | Other (1,206) [532,002]<br>The Police Department res… [91,408]<br>The Department of Housing… [72,962]<br>The Police Department res… [63,868]<br>Service Request status fo… [52,155] |
| **Resolution Action Updated Date** | DateTime | Last Update Timestamp | Datetime when the resolution status was last updated. The field has many NULLs (~1.5 %) and the most common dates cluster around early 2010‑2015. | 2009-12-31T01:35:00+00:00 | 2020-12-23T06:56:14+00:00 | 690,314 |  | 15,072 | Other (690,304) [982,378]<br>(NULL) [15,072]<br>11/15/2010 12:00:00 AM [385]<br>11/07/2012 12:00:00 AM [336]<br>12/09/2010 12:00:00 AM [273] |
| **Community Board** | String | Community Board | NYC community board number or Unspecified. Most records are Unspecified (≈75 %). The remaining entries range from 1 to 12 across boroughs. | 0 Unspecified | Unspecified STATEN ISLAND | 77 |  | 0 | Other (67) [751,635]<br>0 Unspecified [49,878]<br>12 MANHATTAN [29,845]<br>12 QUEENS [23,570]<br>01 BROOKLYN [21,714] |
| **BBL** | String | Borough, Block, Lot Number | A ten‑digit parcel identifier used by the NYC Department of Finance. About 24 % of records are NULL; the most common BBL starts with “2048330028.” | 0000000000 | 5270000501 | 268,383 |  | 243,046 | Other (268,373) [751,031]<br>(NULL) [243,046]<br>2048330028 [1,566]<br>4068290001 [696]<br>4015110001 [664] |
| **Borough** | String | New York City Borough | The borough where the incident occurred (Brooklyn, Queens, Manhattan, Bronx, Unspecified). Brooklyn is the most frequent at ~29 %; Unspecified accounts for about 5 %. | BRONX | Unspecified | 6 | BRONX<br>BROOKLYN<br>MANHATTAN<br>QUEENS<br>STATEN ISLAND<br>Unspecified | 0 | BROOKLYN [296,081]<br>QUEENS [228,818]<br>MANHATTAN [195,488]<br>BRONX [180,142]<br>Unspecified [49,878] |
| **X Coordinate (State Plane)** | Integer | X Coordinate (State Plane) | Easting coordinate in the New York State Plane system. Only ~8.5 % of records contain a value; NULL is the most common. | 913281 | 1067220 | 102,556 |  | 85,327 | Other (102,546) [908,877]<br>(NULL) [85,327]<br>1022911 [1,568]<br>1037000 [701]<br>1023174 [675] |
| **Y Coordinate (State Plane)** | Integer | Y Coordinate (State Plane) | Northing coordinate in the New York State Plane system, with similar sparsity to X (≈8.5 % non‑null). | 121152 | 271876 | 116,092 |  | 85,327 | Other (116,082) [908,868]<br>(NULL) [85,327]<br>264242 [1,566]<br>202363 [706]<br>211606 [665] |
| **Open Data Channel Type** | String | Data Source Channel | Method by which the complaint was reported: PHONE (49 %), UNKNOWN (23 %), ONLINE (18 %) and MOBILE (8 %). The field has no nulls. | MOBILE | UNKNOWN | 5 | MOBILE<br>ONLINE<br>OTHER<br>PHONE<br>UNKNOWN | 0 | PHONE [497,606]<br>UNKNOWN [230,402]<br>ONLINE [177,334]<br>MOBILE [79,892]<br>OTHER [14,766] |
| **Park Facility Name** | String | Park Facility Name | Name of a park facility involved in the incident. Nearly all records are Unspecified (99 %); only ~0.6 % contain actual names such as Central Park or Riverside Park. | "Uncle" Vito F. Maranzano Glendale Playground | Zimmerman Playground | 1,889 |  | 0 | Unspecified [993,141]<br>Other (1,879) [5,964]<br>Central Park [261]<br>Riverside Park [136]<br>Prospect Park [129] |
| **Park Borough** | String | Park Borough | Borough where the park facility is located, mirroring the distribution of the main Borough field. | BRONX | Unspecified | 6 | BRONX<br>BROOKLYN<br>MANHATTAN<br>QUEENS<br>STATEN ISLAND<br>Unspecified | 0 | BROOKLYN [296,081]<br>QUEENS [228,818]<br>MANHATTAN [195,488]<br>BRONX [180,142]<br>Unspecified [49,878] |
| **Vehicle Type** | String | Vehicle Type Involved | Type of vehicle mentioned in the complaint (Car Service, Ambulette/Paratransit). The vast majority are NULL (~99.96 %); only a handful of specific types appear. | Ambulette / Paratransit | Green Taxi | 5 | (NULL)<br>Ambulette / Paratransit<br>Car Service<br>Commuter Van<br>Green Taxi | 999,652 | (NULL) [999,652]<br>Car Service [317]<br>Ambulette / Paratransit [19]<br>Commuter Van [11]<br>Green Taxi [1] |
| **Taxi Company Borough** | String | Taxi Company Borough | Borough where the taxi company is registered; almost all records are NULL (~99.9 %). When present, values include Brooklyn and Queens. | BRONX | Staten Island | 11 |  | 999,156 | (NULL) [999,156]<br>BROOKLYN [207]<br>QUEENS [194]<br>MANHATTAN [171]<br>BRONX [127] |
| **Taxi Pick Up Location** | String | Taxi Pick‑Up Location | Location where a taxi was picked up (e.g., JFK Airport). The field is NULL for ~99 % of records; the most common non‑null value is “Other.” | 1 5 AVENUE MANHATTAN | YORK AVENUE AND EAST 70 STREET | 1,903 |  | 992,129 | (NULL) [992,129]<br>Other [4,091]<br>Other (1,893) [2,021]<br>JFK Airport [562]<br>Intersection [486] |
| **Bridge Highway Name** | String | Bridge or Highway Name | Name of a bridge or highway involved in the incident. Most entries are NULL (~99.8 %). The top values include Belt Pkwy and BQE/Gowanus Expwy. | 145th St. Br - Lenox Ave | Willis Ave Br - 125th St/1st Ave | 68 |  | 997,711 | (NULL) [997,711]<br>Other (58) [851]<br>Belt Pkwy [276]<br>BQE/Gowanus Expwy [254]<br>Grand Central Pkwy [186] |
| **Bridge Highway Direction** | String | Bridge/Highway Direction | Travel direction on the bridge/highway (e.g., East/Long Island Bound). Approximately 99.8 % of records are NULL; the most common non‑null values are East/Long Island Bound and North/Bronx Bound. | Bronx Bound | Westbound/To Goethals Br | 50 |  | 997,691 | (NULL) [997,691]<br>Other (40) [1,064]<br>East/Long Island Bound [210]<br>North/Bronx Bound [208]<br>East/Queens Bound [197] |
| **Road Ramp** | String | Road Ramp Type | Indicates whether a ramp is involved (RAMP, ROADWAY). The field is NULL for ~99.8 % of entries; the most frequent non‑null value is ROADWAY. | N/A | Roadway | 4 | (NULL)<br>N/A<br>Ramp<br>Roadway | 997,693 | (NULL) [997,693]<br>Roadway [1,731]<br>Ramp [555]<br>N/A [21] |
| **Bridge Highway Segment** | String | Bridge/Highway Segment | Specific segment description on a bridge or highway. Over 99.7 % are NULL; when present, values include “Ramp” and “Roadway.” | 1-1-1265963747 | Wythe Ave/Kent Ave (Exit 31) | 937 |  | 997,556 | (NULL) [997,556]<br>Other (927) [2,159]<br>Ramp [92]<br>Roadway [54]<br>Clove Rd/Richmond Rd (Exi… [23] |
| **Latitude** | Float | Latitude Coordinate | Geographic latitude in decimal degrees for the incident location. About 73.9 % of records contain a value; NULLs appear in ~25 %. The most common values are around 40.89° and 40.11°. | 40.1123853 | 40.9128688 | 353,694 |  | 254,695 | Other (353,684) [739,574]<br>(NULL) [254,695]<br>40.89187241649303 [1,538]<br>40.1123853 [1,153]<br>40.89238451539139 [663] |
| **Longitude** | Float | Longitude Coordinate | Geographic longitude in decimal degrees for the incident location. Like latitude, ~74 % of records contain a value; NULLs appear in ~25 %. The most frequent values are around –73.86° and –77.52°. | -77.5195844 | -73.7005968 | 353,996 |  | 254,695 | Other (353,986) [739,574]<br>(NULL) [254,695]<br>-73.86016845296459 [1,538]<br>-77.5195844 [1,153]<br>-73.8592161325675 [663] |
| **Location** | String | Geographic Point (Lat/Long) | Combined string representation of latitude and longitude (e.g., “(40.1123853, -77.5195844)”). The field is NULL in ~25 % of cases; the majority of non‑null entries are clustered around 40.89°, –73.86°. | (40.1123853, -77.5195844) | (40.9128688, -73.9024731) | 375,772 |  | 254,695 | Other (375,762) [739,574]<br>(NULL) [254,695]<br>(40.89187241649303, -73.8… [1,538]<br>(40.1123853, -77.5195844) [1,153]<br>(40.89238451539139, -73.8… [663] |

*Attribution: Generated by qsv v11.0.2 describegpt
Prompt file: Default v6.2.1
Model: openai/gpt-oss-20b
LLM API URL: http://localhost:1234/v1
Timestamp: 2025-12-08T12:10:29.494591+00:00

WARNING: Label and Description generated by an LLM and may contain inaccuracies. Verify before using!*

## REASONING

We need to generate labels and descriptions for each field in dataset. The summary stats provide type, cardinality etc. We also have frequency distribution giving top values.

We must produce JSON with key per field: "Unique Key", etc? Actually the field names are exactly as columns given in summary statistics (like Unique Key, Created Date...). Provide label and description.

We should use domain knowledge: seems dataset is NYC 311 complaints maybe? Fields include Agency, Complaint Type, Descriptor, Location Type, Incident Zip/Address, etc. Provide human-friendly labels: e.g., "Unique Identifier", "Submission Timestamp", "Resolution Timestamp", etc.

Use frequency distribution to mention top values and patterns.

We need description for each field: what it represents, data type, source, typical values.

Also incorporate uniqueness ratio and null counts.

Let's list all fields from summary stats. We'll extract unique keys:

- Unique Key
- Created Date
- Closed Date
- Agency
- Agency Name
- Complaint Type
- Descriptor
- Location Type
- Incident Zip
- Incident Address
- Street Name
- Cross Street 1
- Cross Street 2
- Intersection Street 1
- Intersection Street 2
- Address Type
- City
- Landmark
- Facility Type
- Status
- Due Date
- Resolution Description
- Resolution Action Updated Date
- Community Board
- BBL
- Borough
- X Coordinate (State Plane)
- Y Coordinate (State Plane)
- Open Data Channel Type
- Park Facility Name
- Park Borough
- Vehicle Type
- Taxi Company Borough
- Taxi Pick Up Location
- Bridge Highway Name
- Bridge Highway Direction
- Road Ramp
- Bridge Highway Segment
- Latitude
- Longitude
- Location

Also maybe some duplicates: we saw "Latitude" and "Longitude" again.

We need labels for each. Provide description referencing typical values, cardinality, nulls etc.

Let's craft JSON.

Need to think step by step:

For each field:

1. Unique Key:
   Label: "Unique Identifier"
   Description: A system-generated integer that uniquely identifies each 311 complaint record. It's an auto-incrementing primary key with no missing values; one million distinct values across the dataset.

2. Created Date:
   Label: "Submission Timestamp"
   Description: The date and time when a complaint was submitted to NYC 311, stored as datetime in UTC. Most common dates are early 2013-2015; about 1% missing? No null count 0? Actually summary shows nullcount 0 for Created Date.

But we saw frequency distribution includes "Other (841,004)" meaning 99.7333% of records not among top 10. So description should mention majority dates vary across 2010-2020, with a heavy concentration around 2015.

Also maybe mention the range: 2010-01-01 to 2020-12-23.

3. Closed Date:
   Label: "Resolution Timestamp"
   Description: The date and time when the complaint was closed or resolved. Many records have NULL indicating still open; about 2.86% non-null, with most dates between 2010-2021; typical resolution within a few days of submission.

4. Agency:
   Label: "Reporting Agency Code"
   Description: Two-letter code representing the agency that processed the complaint (e.g., NYPD, HPD). Top agencies are NYPD and HPD. Nulls none. Cardinality 10 unique values including 'Other'.

5. Agency Name:
   Label: "Agency Full Name"
   Description: Full name of the agency handling the complaint; top names correspond to NYPD, Department of Housing Preservation & Development etc.

6. Complaint Type:
   Label: "Complaint Category"
   Description: General category of the issue (Noise - Residential, HEAT/HOT WATER, Illegal Parking, etc.). Over 56% of complaints fall under 'Other'.

7. Descriptor:
   Label: "Specific Issue Descriptor"
   Description: More granular description within a complaint type (e.g., Loud Music/Party, Pothole). Top descriptors are loud music, entire building, heat.

8. Location Type:
   Label: "Location Category"
   Description: Classification of where the complaint was made (Residential Building, Street/Sidewalk, etc.). Two most common categories: Residential Building and NULL.

9. Incident Zip:
   Label: "ZIP Code of Incident"
   Description: 5-digit ZIP code or NULL; top ZIPs include 11226,10467,11207.

10. Incident Address:
    Label: "Street Address of Incident"
    Description: Full street address where the complaint occurred. Many are NULL; common addresses include '655 EAST 230 STREET'.

11. Street Name:
    Label: "Primary Street Name"
    Description: Name of the main street in the incident location. Common streets: BROADWAY, GRAND CONCOURSE.

12. Cross Street 1 and Cross Street 2:
    Label: "Cross Street 1" / "Cross Street 2": names of intersecting streets; many NULL.

13. Intersection Street 1/2:
    Label: "Intersection Street 1" etc.

14. Address Type:
    Label: "Address Classification"
    Description: Indicates whether address is ADDRESS, INTERSECTION, BLOCKFACE, LATLONG, or PLACENAME. Most records are ADDRESS.

15. City:
    Label: "City / Borough"
    Description: The city within New York State (Brooklyn, New York etc.) Many NULL; top cities: Brooklyn, New York.

16. Landmark:
    Label: "Nearby Landmark"
    Description: Notable landmark near the incident location; mostly NULL; common landmarks include EAST 230 STREET.

17. Facility Type:
    Label: "Facility Category"
    Description: Indicates if the complaint involves a facility such as DSNY Garage, Precinct etc.; majority 'N/A'.

18. Status:
    Label: "Complaint Status"
    Description: Current status of the case (Closed, Pending, Open). 95% Closed.

19. Due Date:
    Label: "Due Date for Resolution"
    Description: Deadline by which complaint should be resolved; many NULL; top dates early 2015-2018.

20. Resolution Description:
    Label: "Resolution Narrative"
    Description: Textual description of how the complaint was resolved; often contains agency-specific messages; about 2% null.

21. Resolution Action Updated Date:
    Label: "Last Update Timestamp"
    Description: When the resolution status was last updated; many NULL; top dates early 2010-2015.

22. Community Board:
    Label: "Community Board"
    Description: NYC community board number or Unspecified; most records unspecified.

23. BBL:
    Label: "Borough, Block, Lot (BBL) Number"
    Description: Unique 10-digit identifier for property parcels; many NULL; top values include '2048330028'.

24. Borough:
    Label: "New York City Borough"
    Description: The borough where the incident occurred (Brooklyn, Queens, Manhattan, Bronx, Unspecified). Most are Brooklyn.

25. X Coordinate (State Plane):
    Label: "X Coordinate (State Plane)"
    Description: Easting coordinate in New York State Plane coordinates; mostly NULL; 8.5% non-null.

26. Y Coordinate (State Plane):
    Label: "Y Coordinate (State Plane)"
    Description: Northing coordinate; same as above.

27. Open Data Channel Type:
    Label: "Data Source Channel"
    Description: How the complaint was reported (Phone, Online, Mobile, etc.). Majority Phone.

28. Park Facility Name:
    Label: "Park Facility Name"
    Description: If the incident involves a park facility; most are Unspecified.

29. Park Borough:
    Label: "Park Borough"
    Description: Borough of the park facility; similar distribution to Borough field.

30. Vehicle Type:
    Label: "Vehicle Type Involved"
    Description: Type of vehicle in the complaint (Car Service, Ambulette/Paratransit, etc.). Majority NULL.

31. Taxi Company Borough:
    Label: "Taxi Company Borough"
    Description: Borough where the taxi company is registered; majority NULL.

32. Taxi Pick Up Location:
    Label: "Taxi Pick-Up Location"
    Description: Where the taxi was picked up (e.g., JFK Airport). Most records NULL.

33. Bridge Highway Name:
    Label: "Bridge or Highway Name"
    Description: Name of bridge or highway involved in the incident; majority NULL.

34. Bridge Highway Direction:
    Label: "Bridge/Highway Direction"
    Description: Direction of travel on bridge/highway; most are NULL.

35. Road Ramp:
    Label: "Road Ramp Type"
    Description: Indicates whether a ramp is involved; mostly NULL.

36. Bridge Highway Segment:
    Label: "Bridge/Highway Segment"
    Description: Specific segment description; majority NULL.

37. Latitude:
    Label: "Latitude Coordinate"
    Description: Geographical latitude of the incident location in decimal degrees. Majority are other values.

38. Longitude:
    Label: "Longitude Coordinate"
    Description: Geographical longitude.

39. Location:
    Label: "Geographic Point (Lat/Long)"
    Description: Combined coordinate string; majority 'Other'.

But note some fields like Latitude and Longitude appear duplicates with <ALL_UNIQUE> etc.

Need to produce JSON with all these fields.

We need to ensure the JSON is properly formatted.

Let's craft it. Use consistent field names as keys exactly same as dataset columns.

We'll produce description referencing summary stats: e.g., cardinality, nulls, uniqueness ratio, typical values from frequency distribution.

Make sure not too long but detailed.

Ok let's draft.


## TOKEN USAGE

TokenUsage { prompt: 11558, completion: 4672, total: 16230, elapsed: 67035 }
---
# Description
**Description**

The dataset contains one million 311 service‑request records from New York City, covering the period 2010–2020. Each row is uniquely identified by an auto‑incremented integer key and includes timestamps for when a complaint was created (most concentrated between early 2013 and mid‑2015) and, when available, closed or updated dates. The majority of requests are handled by the NYPD or HPD agencies, with “Noise – Residential” and “HEAT/HOT WATER” being the most common complaint types. Geographic information is partially geocoded: about three‑quarters of records include latitude/longitude coordinates, while address components (street name, cross streets, etc.) are present in roughly a third of cases. The dataset also records how each request was submitted—most were filed by phone— and tracks the current status (≈95 % closed).  

**Notable Characteristics**

- **Central tendency & spread** – Created dates cluster around 2013–2015; status is heavily skewed toward “Closed.” Agency distribution shows a long tail of less‑frequent agencies, with an “Other” category accounting for ~3 %.  
- **Distribution shape** – Many categorical fields exhibit high cardinality and a pronounced “Other (N)” bucket that aggregates the remaining unique values. Geographic fields (zip code, street name) show sparse coverage: 25–30 % of records lack address detail.  
- **Missing values & placeholders** – Significant null rates in Closed Date (≈97 %), Incident Zip (≈5 %), Latitude/Longitude (≈25 %), and several address components. Placeholder dates such as “1900‑01‑01” or “2100‑01‑01” appear in Closed Date, indicating unknown or future values.  
- **Outliers & anomalies** – The presence of extreme dates (e.g., 1900/2100) and a few unusually high counts for specific fields (“Other (N)” groups). No numeric outliers are evident beyond the coordinate ranges typical for New York City.  
- **Duplicates** – The `Unique Key` guarantees no duplicate rows; however, other fields exhibit many repeated values (e.g., “BROOKLYN” in Borough) and a high proportion of distinct entries in street names and incident addresses.  
- **PII / Sensitive data** – Full addresses and precise latitude/longitude coordinates may be considered personally identifying or location‑sensitive under privacy regulations; users should handle these fields with appropriate safeguards if sharing the dataset publicly.  
- **Data quality issues** – High “Other” proportions indicate many unique, uncategorized entries that could hinder analytical consistency. The uneven distribution of missingness across fields suggests potential sampling bias in downstream analyses.


Generated by qsv v11.0.2 describegpt
Prompt file: Default v6.2.1
Model: openai/gpt-oss-20b
LLM API URL: http://localhost:1234/v1
Timestamp: 2025-12-08T12:10:55.098557+00:00

WARNING: Generated by an LLM and may contain inaccuracies. Verify before using!



## REASONING

We need to produce a dataset description, 1-8 sentences. Then Notable Characteristics bullet list summarizing key stats: central tendency, spread, distribution shape, anomalies, patterns; outliers, missing values, duplicates, PII/PHI/PCI, other data quality issues.

Use the provided summary statistics and frequency distribution for guidance but not output per field.

Dataset is NYC 311 complaints dataset from 2010-2020 with 1 million records. Unique key unique. Created date mostly early 2013-2015; closed date often null; Agency distribution major: NYPD, HPD, DOT etc. Complaint type heavy noise residential and other. Location types residential building most common, many nulls. Zip codes mostly missing or high variety. Address fields sparse. Street names have many unique values with big "Other". Many geocoded lat/long but ~25% null.

Also open data channel: phone majority. Status closed 95%. PII? The dataset includes addresses, agency, maybe location coordinates; likely no PII except possibly address info may be considered sensitive. Not sure.

We should mention duplicates: Unique key ensures uniqueness so no duplicate rows. But other fields have high cardinality.

Missing values: many nulls across various columns (e.g., Closed Date 2.8% non-null, Incident Zip 5.5% null). Latitude/Longitude missing ~25%. Address components many null. Others like Vehicle Type almost all null.

Outliers? Not obvious but some extreme dates like 1900-2100; but these are placeholders for unknown or erroneous. Also range of coordinates.

Anomalies: Created date distribution with heavy concentration in early 2013-2015, maybe due to data collection change. Closed Date has many NULL and some placeholder values 1900/2100.

Pattern: Many "Other" categories indicating high diversity.

Also note presence of potential PII such as full addresses; but dataset is public so presumably sanitized. But location coordinates may be considered sensitive? Provide caution.

Also note the presence of duplicate "Other (N)" rows summarizing other values; large counts for Other indicate high variety.

Now produce description: 1-8 sentences.

Then bullet list.

Add Attribution placeholder.

Let's craft.
## TOKEN USAGE

TokenUsage { prompt: 14196, completion: 1041, total: 15237, elapsed: 25602 }
---
# Tags
- nyc_311_complaints  
- public_safety  
- noise_complaint  
- transportation_infrastructure  
- housing_and_community  
- geospatial_data  
- open_data_platform  
- municipal_services  
- citizen_engagement  
- big_data_analysis  

Attribution: 
Generated by qsv v11.0.2 describegpt
Prompt file: Default v6.2.1
Model: openai/gpt-oss-20b
LLM API URL: http://localhost:1234/v1
Timestamp: 2025-12-08T12:11:09.258643+00:00

WARNING: Generated by an LLM and may contain inaccuracies. Verify before using!



## REASONING

We need to output tags in Markdown format, no more than 10, descending importance. Use the summary stats and freq distribution data. Tags should be lowercase with underscores.

Consider dataset is NYC 311 complaints. Thematic tags: citizen services, public safety, noise complaints, transportation, housing, city government, geospatial, open data, complaint management, New York City.

Let's pick 10: nyc_311_complaints, public_safety, noise_complaint, transportation_infrastructure, housing_and_community, geospatial_data, open_data_platform, municipal_services, citizen_engagement, big_data_analysis. That is 10 tags.

Ensure lowercase with underscore.

Add attribution placeholder after tags. Format:

```
- nyc_311_complaints
- public_safety
...
Attribution: {GENERATED_BY_SIGNATURE}
```

But instruction says "Add an Attribution with the placeholder "{GENERATED_BY_SIGNATURE}" after the Tags." It doesn't specify format but likely simple text after list.

Let's output markdown bullet list then attribution line.
## TOKEN USAGE

TokenUsage { prompt: 14165, completion: 286, total: 14451, elapsed: 14153 }
---
