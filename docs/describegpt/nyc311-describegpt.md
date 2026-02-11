Generated using a Local LLM (openai/gpt-oss-20b) on LM Studio 0.4.2+2 running on a Macbook Pro M4 Max 64gb/Tahoe 26.3:

```bash
$ QSV_LLM_BASE_URL=http://localhost:1234/v1 qsv describegpt NYC_311_SR_2010-2020-sample-1M.csv --all \
     --output nyc311-describegpt.md
```
---
# Dictionary
| Name | Type | Label | Description | Min | Max | Cardinality | Enumeration | Null Count | Examples |
|------|------|-------|-------------|-----|-----|-------------|-------------|------------|----------|
| **Unique Key** | Integer | Record Identifier | A unique numeric identifier for each complaint record. It is the primary key in the dataset and has 1,000,000 distinct values (100% uniqueness). | 11465364 | 48478173 | 1,000,000 |  | 0 | <ALL_UNIQUE> |
| **Created Date** | DateTime | Complaint Creation Timestamp | UTC timestamp indicating when a 311 service request was logged. The dates span from January 1 2010 to December 23 2020 with a mean around November 10 2015. Approximately 84% of records have missing values. | 2010-01-01T00:00:00+00:00 | 2020-12-23T01:25:51+00:00 | 841,014 |  | 0 | Other [997,333]<br>01/24/2013 12:00:00 AM [347]<br>01/07/2014 12:00:00 AM [315]<br>01/08/2015 12:00:00 AM [283]<br>02/16/2015 12:00:00 AM [269] |
| **Closed Date** | DateTime | Complaint Closure Timestamp | UTC timestamp marking the final resolution or closure of the complaint. Valid dates range from January 1 1900 to January 1 2100, but most entries are null (≈99% missing). The top recorded dates show that complaints were frequently closed in 2010–2020. | 1900-01-01T00:00:00+00:00 | 2100-01-01T00:00:00+00:00 | 688,837 |  | 28,619 | Other [968,671]<br>(NULL) [28,619]<br>11/15/2010 12:00:00 AM [384]<br>11/07/2012 12:00:00 AM [329]<br>12/09/2010 12:00:00 AM [267] |
| **Agency** | String | Agency Code | Two‑ or three‑character code identifying the city agency responsible for handling the complaint. The most common codes are NYPD (26.5 %), HPD (25.8 %), DOT (13.2 %) and DSNY (8.2 %). A small proportion of records have an "Other" code. | 3-1-1 | TLC | 28 |  | 0 | NYPD [265,116]<br>HPD [258,033]<br>DOT [132,462]<br>DSNY [81,606]<br>DEP [75,895] |
| **Agency Name** | String | Agency Full Name | Full name of the agency that processed or responded to the complaint. The top agencies include New York City Police Department, Department of Housing Preservation and Development, Department of Transportation, etc., together accounting for about 70 % of records. | 3-1-1 | Valuation Policy | 553 |  | 0 | New York City Police Depa… [265,038]<br>Department of Housing Pre… [258,019]<br>Department of Transportat… [132,462]<br>Other [103,974]<br>Department of Environment… [75,895] |
| **Complaint Type** | String | Complaint Category | High‑level classification of the reported issue, such as Noise - Residential, HEAT/HOT WATER, Illegal Parking, Blocked Driveway, Street Condition, etc. The majority of complaints (≈56 %) fall under an "Other" category, while the top ten categories represent roughly 43 % of all records. | ../../WEB-INF/web.xml;x= | ZTESTINT | 287 |  | 0 | Other [563,561]<br>Noise - Residential [89,439]<br>HEAT/HOT WATER [56,639]<br>Illegal Parking [45,032]<br>Blocked Driveway [42,356] |
| **Descriptor** | String | Complaint Detail | Fine‑grained description within a complaint type (e.g., Loud Music/Party, Heat, No Access). The most frequent descriptors include Loud Music/Party (~9.4 %) and other subcategories, with about 67 % of entries labeled as "Other". | 1 Missed Collection | unknown odor/taste in drinking water (QA6) | 1,392 |  | 3,001 | Other [671,870]<br>Loud Music/Party [93,646]<br>ENTIRE BUILDING [36,885]<br>HEAT [35,088]<br>No Access [31,631] |
| **Location Type** | String | Location Category | General classification of where the incident occurred—Residential Building, Street/Sidewalk, etc. Residential Building accounts for ~34 %, Street/Sidewalk ~19 %, and many records are missing. | 1-, 2- and 3- Family Home | Wooded Area | 162 |  | 239,131 | RESIDENTIAL BUILDING [255,562]<br>(NULL) [239,131]<br>Street/Sidewalk [145,653]<br>Residential Building/Hous… [92,765]<br>Street [92,190] |
| **Incident Zip** | String | Incident ZIP Code | Five‑digit ZIP code indicating the area of the complaint. The most common ZIP codes include 11226 (~1.8 %), 10467 (~1.5 %), etc., while 86 % of records have a valid ZIP; about 6 % are missing. | * | XXXXX | 535 |  | 54,978 | Other [815,988]<br>(NULL) [54,978]<br>11226 [17,114]<br>10467 [14,495]<br>11207 [12,872] |
| **Incident Address** | String | Incident Address | Full street address or intersection where the issue was reported. The dataset contains many distinct addresses, with the top ten appearing in ~0.2 % each; nearly all records fall into the "Other" bucket indicating a high level of uniqueness. | * * | west 155 street and edgecombe avenue | 341,996 |  | 174,700 | Other [819,046]<br>(NULL) [174,700]<br>655 EAST  230 STREET [1,538]<br>78-15 PARSONS BOULEVARD [694]<br>672 EAST  231 STREET [663] |
| **Street Name** | String | Street Name | Name of the primary street involved in the incident. Broadway is the most common (≈1.2 %) followed by other major streets; about 95 % of values are unique or appear rarely. | * | wyckoff avenue | 14,837 |  | 174,720 | Other [784,684]<br>(NULL) [174,720]<br>BROADWAY [9,702]<br>GRAND CONCOURSE [5,851]<br>OCEAN AVENUE [3,946] |
| **Cross Street 1** | String | Cross Street 1 | First cross street at an intersection complaint location, e.g., BEND or BROADWAY. The top values account for roughly 18 % of entries; the field is null in many records that are not intersections. | 1 AVE | mermaid | 16,238 |  | 320,401 | Other [619,743]<br>(NULL) [320,401]<br>BEND [12,562]<br>BROADWAY [8,548]<br>3 AVENUE [6,154] |
| **Cross Street 2** | String | Cross Street 2 | Second cross street at an intersection. Distribution mirrors Cross Street 1, with BEND and BROADWAY again among the most frequent values. | 1 AVE | surf | 16,486 |  | 323,644 | Other [623,363]<br>(NULL) [323,644]<br>BEND [12,390]<br>BROADWAY [8,833]<br>DEAD END [5,626] |
| **Intersection Street 1** | String | Intersection Street 1 | First street forming an intersection where a complaint was reported; common values include BROADWAY (~3.6 %) and other major streets. The field is often null for non‑intersection complaints. | 1 AVE | flatlands AVE | 11,237 |  | 767,422 | (NULL) [767,422]<br>Other [214,544]<br>BROADWAY [3,761]<br>CARPENTER AVENUE [2,918]<br>BEND [2,009] |
| **Intersection Street 2** | String | Intersection Street 2 | Second street of the intersection, with similar distribution to Intersection Street 1. | 1 AVE | glenwood RD | 11,674 |  | 767,709 | (NULL) [767,709]<br>Other [215,667]<br>BROADWAY [3,462]<br>BEND [1,942]<br>2 AVENUE [1,690] |
| **Address Type** | String | Address Format | Specifies how the incident location was recorded—ADDRESS, INTERSECTION, BLOCKFACE, LATLONG, or PLACENAME. ADDRESS dominates at ~81 %, followed by INTERSECTION (~15 %). | ADDRESS | PLACENAME | 6 |  | 125,802 | ADDRESS [710,380]<br>INTERSECTION [133,361]<br>(NULL) [125,802]<br>BLOCKFACE [22,620]<br>LATLONG [7,421] |
| **City** | String | City/Borough | Name of the borough where the complaint occurred (Brooklyn, Queens, Manhattan, Bronx, Staten Island). Brooklyn is the most frequent borough (~32 %), with many records missing. | * | YORKTOWN HEIGHTS | 382 |  | 61,963 | BROOKLYN [296,254]<br>NEW YORK [189,069]<br>BRONX [181,168]<br>Other [163,936]<br>(NULL) [61,963] |
| **Landmark** | String | Nearby Landmark | Notable landmark or street close to the incident location, e.g., EAST 230 STREET, BROADWAY. The field is null for ~91 % of records; the top landmarks represent a small portion of the data. | 1 AVENUE | ZULETTE AVENUE | 5,915 |  | 912,779 | (NULL) [912,779]<br>Other [80,165]<br>EAST  230 STREET [1,545]<br>EAST  231 STREET [1,291]<br>BROADWAY [1,148] |
| **Facility Type** | String | Facility Category | Type of facility involved in the complaint (e.g., DSNY Garage, Precinct). N/A accounts for 73 % of entries; Precinct appears in about 23 %; other categories are rare. | DSNY Garage | School District | 6 |  | 145,478 | N/A [628,279]<br>Precinct [193,259]<br>(NULL) [145,478]<br>DSNY Garage [32,310]<br>School [617] |
| **Status** | String | Complaint Status | Current processing status—Closed, Pending, Open, etc. Closed complaints dominate (~95 %), with small percentages remaining pending or open. | Assigned | Unspecified | 10 | Assigned<br>Closed<br>Closed - Testing<br>Email Sent<br>In Progress<br>Open<br>Pending<br>Started<br>Unassigned<br>Unspecified | 0 | Closed [952,522]<br>Pending [20,119]<br>Open [12,340]<br>In Progress [7,841]<br>Assigned [6,651] |
| **Due Date** | DateTime | Resolution Deadline | Target date set by the agency to resolve the complaint, ranging from 2010 to 2021. Many records have missing values (≈65 %); among those present, the most common dates cluster in mid‑2015. | 1900-01-02T00:00:00+00:00 | 2021-06-17T16:34:13+00:00 | 345,077 |  | 647,794 | (NULL) [647,794]<br>Other [350,746]<br>04/08/2015 10:00:58 AM [214]<br>05/02/2014 03:32:17 PM [183]<br>03/30/2018 10:10:39 AM [172] |
| **Resolution Description** | String | Resolution Narrative | Textual account of how the complaint was resolved—actions taken or outcome noted. The top narratives involve police response or HPD inspections; about 52 % of entries fall under an "Other" category. | A DOB violation was issued for failing to comply with an existing Stop Work Order. | Your request was submitted to the Department of Homeless Services. The City?s outreach team will assess the homeless individual and offer appropriate assistance within 2 hours. If you asked to know the outcome of your request, you will get a call within 2 hours. No further status will be available through the NYC 311 App, 311, or 311 Online. | 1,216 |  | 20,480 | Other [511,739]<br>The Police Department res… [91,408]<br>The Department of Housing… [72,962]<br>The Police Department res… [63,868]<br>Service Request status fo… [52,155] |
| **Resolution Action Updated Date** | DateTime | Last Update Timestamp | UTC timestamp indicating when the resolution action was last updated. The field has a high proportion of nulls (~99 %) and, where present, dates span from 2010 to 2020. | 2009-12-31T01:35:00+00:00 | 2020-12-23T06:56:14+00:00 | 690,314 |  | 15,072 | Other [982,148]<br>(NULL) [15,072]<br>11/15/2010 12:00:00 AM [385]<br>11/07/2012 12:00:00 AM [336]<br>12/09/2010 12:00:00 AM [273] |
| **Community Board** | String | Community Board | Borough‑level community board responsible for the area (e.g., 12 MANHATTAN). Most records are unspecified; when specified, they range across all boroughs with varying frequencies. | 0 Unspecified | Unspecified STATEN ISLAND | 77 |  | 0 | Other [751,635]<br>0 Unspecified [49,878]<br>12 MANHATTAN [29,845]<br>12 QUEENS [23,570]<br>01 BROOKLYN [21,714] |
| **BBL** | String | BBL (Borough‑Block‑Lot) | Numeric identifier unique to NYC property parcels. The dataset contains many distinct values; about 24 % of entries are missing. The top ten BBLs account for roughly 0.2 % each. | 0140694020 | 0140694020 | 268,383 |  | 243,046 | Other [750,668]<br>(NULL) [243,046]<br>2048330028 [1,566]<br>4068290001 [696]<br>4015110001 [664] |
| **Borough** | String | Borough | One of the five NYC boroughs or Unspecified. Brooklyn is the most common (≈30 %), followed by Queens, Manhattan, Bronx, and Staten Island. | BRONX | Unspecified | 6 | BRONX<br>BROOKLYN<br>MANHATTAN<br>QUEENS<br>STATEN ISLAND<br>Unspecified | 0 | BROOKLYN [296,081]<br>QUEENS [228,818]<br>MANHATTAN [195,488]<br>BRONX [180,142]<br>Unspecified [49,878] |
| **X Coordinate (State Plane)** | Integer | X Coordinate (State Plane) | Easting coordinate in the New York State Plane coordinate system, used for precise mapping of complaint locations. Most values cluster around 1,020,000 m; about 10 % of records are missing. | 913281 | 1067220 | 102,556 |  | 85,327 | Other [908,535]<br>(NULL) [85,327]<br>1022911 [1,568]<br>1037000 [701]<br>1023174 [675] |
| **Y Coordinate (State Plane)** | Integer | Y Coordinate (State Plane) | Northing coordinate in the State Plane system. Values typically fall near 200,000 m; a small portion of entries are null. | 121152 | 271876 | 116,092 |  | 85,327 | Other [908,538]<br>(NULL) [85,327]<br>264242 [1,566]<br>202363 [706]<br>211606 [665] |
| **Open Data Channel Type** | String | Submission Channel | Method used to submit the complaint—PHONE, UNKNOWN, ONLINE, MOBILE, or OTHER. PHONE is by far the most common channel (~50 %), followed by UNKNOWN (~23 %) and ONLINE (~18 %). | MOBILE | UNKNOWN | 5 | MOBILE<br>ONLINE<br>OTHER<br>PHONE<br>UNKNOWN | 0 | PHONE [497,606]<br>UNKNOWN [230,402]<br>ONLINE [177,334]<br>MOBILE [79,892]<br>OTHER [14,766] |
| **Park Facility Name** | String | Park Facility Name | Name of a park or playground if the incident occurred within a park; the majority of records are Unspecified, with Central Park, Riverside Park, etc., appearing in the minority. | "Uncle" Vito F. Maranzano Glendale Playground | Zimmerman Playground | 1,889 |  | 0 | Unspecified [993,141]<br>Other [5,964]<br>Central Park [261]<br>Riverside Park [136]<br>Prospect Park [129] |
| **Park Borough** | String | Park Borough | Borough where the park is located; distribution mirrors that of Borough (Brooklyn ~30 %, Queens ~23 %, Manhattan ~20 %). | BRONX | Unspecified | 6 | BRONX<br>BROOKLYN<br>MANHATTAN<br>QUEENS<br>STATEN ISLAND<br>Unspecified | 0 | BROOKLYN [296,081]<br>QUEENS [228,818]<br>MANHATTAN [195,488]<br>BRONX [180,142]<br>Unspecified [49,878] |
| **Vehicle Type** | String | Vehicle Type | Type of vehicle involved in the complaint, such as Car Service or Green Taxi. The field is largely null (~99.9 %); among non‑null values, Car Service dominates (~91 %). | Ambulette / Paratransit | Green Taxi | 5 |  | 999,652 | (NULL) [999,652]<br>Car Service [317]<br>Ambulette / Paratransit [19]<br>Commuter Van [11]<br>Green Taxi [1] |
| **Taxi Company Borough** | String | Taxi Company Borough | Borough affiliation of the taxi company that issued the ticket or was involved. Most records are unspecified; when specified, they map to the five boroughs with varying frequencies. | BRONX | Staten Island | 11 |  | 999,156 | (NULL) [999,156]<br>BROOKLYN [207]<br>QUEENS [194]<br>MANHATTAN [171]<br>BRONX [127] |
| **Taxi Pick Up Location** | String | Taxi Pickup Location | Origin point for a taxi pickup—common values include JFK Airport, intersection, etc.; the field is null in many cases and "Other" represents the majority of non‑null entries. | 1 5 AVENUE MANHATTAN | YORK AVENUE AND EAST 70 STREET | 1,903 |  | 992,129 | (NULL) [992,129]<br>Other [4,091]<br>Other [2,006]<br>JFK Airport [562]<br>Intersection [486] |
| **Bridge Highway Name** | String | Bridge/Highway Name | Name of bridge or highway associated with the complaint (e.g., Belt Pkwy, BQE/Gowanus Expwy). A handful of names cover most records; a large proportion are labeled as "Other". | 145th St. Br - Lenox Ave | Willis Ave Br - 125th St/1st Ave | 68 |  | 997,711 | (NULL) [997,711]<br>Other [779]<br>Belt Pkwy [276]<br>BQE/Gowanus Expwy [254]<br>Grand Central Pkwy [186] |
| **Bridge Highway Direction** | String | Bridge/Highway Direction | Direction of travel on the bridge or highway—East/Long Island Bound, North/Bronx Bound, etc.; the field has a high prevalence of nulls. | Bronx Bound | Westbound/To Goethals Br | 50 |  | 997,691 | (NULL) [997,691]<br>Other [987]<br>East/Long Island Bound [210]<br>North/Bronx Bound [208]<br>East/Queens Bound [197] |
| **Road Ramp** | String | Road Ramp Type | Indicates whether a ramp is present at the complaint location (Roadway, Ramp, N/A). Most records have "Roadway" (~75 %), followed by "Ramp" (~24 %). | N/A | Roadway | 4 |  | 997,693 | (NULL) [997,693]<br>Roadway [1,731]<br>Ramp [555]<br>N/A [21] |
| **Bridge Highway Segment** | String | Bridge/Highway Segment | Specific segment identifier of the bridge or highway involved; most entries are labeled as "Other". | 1-1-1265963747 | Wythe Ave/Kent Ave (Exit 31) | 937 |  | 997,556 | (NULL) [997,556]<br>Other [2,144]<br>Ramp [92]<br>Roadway [54]<br>Clove Rd/Richmond Rd (Exi… [23] |
| **Latitude** | Float | Latitude | Decimal degree latitude coordinate for the complaint location, ranging from about 40.1° to 41.0°. The majority of points lie between 40.1 and 40.9. | 40.1123853 | 40.9128688 | 353,694 |  | 254,695 | Other [739,329]<br>(NULL) [254,695]<br>40.89187241649303 [1,538]<br>40.1123853 [1,153]<br>40.89238451539139 [663] |
| **Longitude** | Float | Longitude | Decimal degree longitude coordinate, typically between -77.5° and -73.7°. Most locations are in the range -74 to -73.8. | -77.5195844 | -73.7005968 | 353,996 |  | 254,695 | Other [739,329]<br>(NULL) [254,695]<br>-73.86016845296459 [1,538]<br>-77.5195844 [1,153]<br>-73.8592161325675 [663] |
| **Location** | String | Location (LatLon) | Textual representation of latitude and longitude as a comma‑separated pair enclosed in parentheses, e.g., "(40.89187241649303, -73.86016845296459)". The field is null for about 25 % of records. | (40.1123853, -77.5195844) | (40.9128688, -73.9024731) | 375,772 |  | 254,695 | Other [739,329]<br>(NULL) [254,695]<br>(40.89187241649303, -73.8… [1,538]<br>(40.1123853, -77.5195844) [1,153]<br>(40.89238451539139, -73.8… [663] |

*Attribution: Generated by qsv v16.0.0 describegpt
Command line: target/debug/qsv describegpt /tmp/NYC_311_SR_2010-2020-sample-1M.csv --all --output /tmp/nyc311-describegpt.md
Prompt file: Default v7.0.0
Model: openai/gpt-oss-20b
LLM API URL: http://localhost:1234/v1
Language: 
Timestamp: 2026-02-10T21:48:35.892529+00:00

WARNING: Label and Description generated by an LLM and may contain inaccuracies. Verify before using!
*

## REASONING

We need to generate Labels and Descriptions for all 41 fields using summary stats and freq distribution.

Goal: Provide JSON with each field name as key; value has "label" and "description".

Need human-friendly labels and descriptions. Use domain knowledge. Provide full description referencing dataset context, meaning, typical values etc. Should use data from frequency distributions to add details about common values or categories.

We need to think step by step: For each field, craft label same as name but maybe better: e.g., "Unique Key" -> "Record Identifier". Provide description: unique integer identifier for complaint record; primary key; non-null; cardinality 1 million; etc. Use summary stats like uniqueness ratio (100%) and cardinality.

We have many fields with null counts etc. We can mention typical values, data types, usage.

Need to produce JSON object with 41 entries. Provide label and description for each.

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

We need to use summary stats and frequency distribution for context.

Also note that dataset appears to be NYC 311 complaints dataset. Provide description accordingly.

We'll produce JSON with keys exactly as field names: "Unique Key" etc.

Let's craft each:

1. Unique Key: integer unique ID. Label: "Record Identifier". Description: unique numeric key assigned to each complaint; primary key; cardinality equal to number of records; all values unique; used for linking and referencing.

2. Created Date: datetime when complaint was created. Label: "Complaint Creation Timestamp". Description: timestamp (UTC) indicating when 311 service request was logged; covers dates from 2010-01-01 to 2020-12-23; majority around 2015; high uniqueness ratio? Not fully unique.

3. Closed Date: datetime when complaint closed. Label: "Complaint Closure Timestamp". Description: UTC timestamp of final resolution or closure; ranges 1900-2100 due to placeholder values; most records have null or placeholder; 99% missing; typical distribution shows top dates etc.

4. Agency: short code for agency responsible. Label: "Agency Code". Description: two- or three-character code representing the city agency that handled the complaint (e.g., NYPD, HPD). Most common codes: NYPD, HPD, DOT. Many nulls? Not indicated; but frequency shows top 10.

5. Agency Name: full name of responsible agency. Label: "Agency Name". Description: full descriptive name for the agency that responded to or processed complaint; most frequent are NYC Police Department, Dept of Housing Preservation and Development etc.

6. Complaint Type: category of issue reported (e.g., Noise - Residential). Label: "Complaint Category". Description: primary classification of problem; top categories include noise, heat/hot water, illegal parking, blocked driveway, street condition, etc.; majority classified as other.

7. Descriptor: more detailed subcategory within complaint type (e.g., Loud Music/Party). Label: "Detailed Complaint Descriptor". Description: finer-grained description of issue; common descriptors include Loud Music/Party, Heat, No Access, Street Light Out, Pothole, etc.; majority labeled as other.

8. Location Type: classification of the location where complaint occurred (e.g., Residential Building, Street). Label: "Location Category". Description: indicates general type of place for incident; most frequent are residential building, street/sidewalk, etc.; many nulls.

9. Incident Zip: ZIP code of incident. Label: "Incident ZIP Code". Description: 5-digit ZIP code where complaint was reported; top zip codes include 11226, 10467, etc.; majority present but some missing.

10. Incident Address: full address string. Label: "Incident Address". Description: textual street address or intersection where issue occurred; includes many addresses such as "655 EAST 230 STREET" etc.; high proportion of 'Other'.

11. Street Name: name of the street involved in incident. Label: "Street Name". Description: common street names like Broadway, Grand Concourse; majority of records have a valid street.

12. Cross Street 1: first cross street at intersection. Label: "Cross Street 1". Description: if complaint occurred at an intersection, this field lists the first cross street; top values include BEND, BROADWAY etc.; some nulls.

13. Cross Street 2: second cross street. Label: "Cross Street 2". Description: second cross street in intersection; similar distribution to Cross Street 1.

14. Intersection Street 1: one of streets forming the intersection. Label: "Intersection Street 1". Description: first street in an intersection complaint; many values, common ones like BROADWAY etc.

15. Intersection Street 2: second street in intersection. Label: "Intersection Street 2". Description: second street of intersection.

16. Address Type: type of address used (e.g., ADDRESS, INTERSECTION). Label: "Address Type". Description: indicates format of incident address; most common are ADDRESS and INTERSECTION; small proportion BLOCKFACE etc.; some nulls.

17. City: city or borough where complaint occurred. Label: "City/Borough". Description: name of NYC borough (Brooklyn, Queens, Manhattan, Bronx, Staten Island) or other cities; majority in Brooklyn.

18. Landmark: notable landmark near incident. Label: "Nearby Landmark". Description: common landmarks such as EAST 230 STREET, BROADWAY, etc.; many nulls.

19. Facility Type: type of facility involved (e.g., DSNY Garage). Label: "Facility Type". Description: indicates building or structure type; majority are N/A; some are Precinct, DSNY Garage, School.

20. Status: current status of complaint (Closed, Pending, Open, etc.). Label: "Complaint Status". Description: indicates processing stage; 95% closed, rest pending/open/in progress.

21. Due Date: due date for resolution. Label: "Due Date". Description: target resolution date set by agency; ranges across years; many missing.

22. Resolution Description: narrative description of resolution. Label: "Resolution Narrative". Description: textual account of actions taken or outcome; common phrases include police responded, HPD inspected etc.; majority 'Other'.

23. Resolution Action Updated Date: timestamp of last update to resolution action. Label: "Last Resolution Update Timestamp". Description: UTC time when resolution was last updated; high frequency of nulls.

24. Community Board: community board number or Unspecified. Label: "Community Board". Description: borough-level community board responsible for area (e.g., 12 MANHATTAN); majority unspecified.

25. BBL: Borough, Block, Lot number unique to NYC property parcels. Label: "BBL (Borough-Block-Lot)". Description: numeric identifier representing parcel; many nulls; some top values.

26. Borough: borough where incident occurred. Label: "Borough". Description: one of Brooklyn, Queens, Manhattan, Bronx, Staten Island, Unspecified; majority Brooklyn.

27. X Coordinate (State Plane): X coordinate in State Plane metric system. Label: "X Coordinate (State Plane)". Description: numeric Easting coordinate in US state plane units; used for mapping.

28. Y Coordinate (State Plane): Y coordinate in State Plane. Label: "Y Coordinate (State Plane)". Description: numeric Northing coordinate.

29. Open Data Channel Type: channel used to submit complaint. Label: "Open Data Channel". Description: indicates method of submission: PHONE, UNKNOWN, ONLINE, MOBILE, OTHER; majority phone.

30. Park Facility Name: name of park facility if incident in a park. Label: "Park Facility Name". Description: name of park or playground; majority unspecified; some parks like Central Park, Riverside Park etc.

31. Park Borough: borough where park is located. Label: "Park Borough". Description: borough of the park; same distribution as Borough.

32. Vehicle Type: type of vehicle in complaint (e.g., Car Service). Label: "Vehicle Type". Description: indicates vehicle involved if relevant; majority null, few Car Service etc.

33. Taxi Company Borough: borough where taxi company operates. Label: "Taxi Company Borough". Description: borough association of taxi company; mostly unspecified.

34. Taxi Pick Up Location: pickup location for taxis. Label: "Taxi Pickup Location". Description: indicates origin point such as JFK Airport, intersection, etc.; majority 'Other'.

35. Bridge Highway Name: name of bridge or highway involved. Label: "Bridge/Highway Name". Description: name of bridge or highway; many values, e.g., Belt Pkwy, BQE/Gowanus Expwy.

36. Bridge Highway Direction: direction of travel on bridge/highway. Label: "Bridge/Highway Direction". Description: indicates cardinal direction of traffic (East/Long Island Bound etc.).

37. Road Ramp: type of road ramp. Label: "Road Ramp Type". Description: indicates presence of ramp; values like Roadway, Ramp, N/A.

38. Bridge Highway Segment: specific segment of bridge or highway. Label: "Bridge/Highway Segment". Description: identifier for segment (e.g., 1-1-1265963747).

39. Latitude: geographic latitude coordinate. Label: "Latitude". Description: decimal degrees; distribution across NYC; majority between 40.1 and 41.0.

40. Longitude: geographic longitude coordinate. Label: "Longitude". Description: decimal degrees; majority -73 to -77.

41. Location: combined lat/long string. Label: "Location (LatLon)". Description: textual representation of coordinates, e.g., "(40.89187241649303, -73.86016845296459)".

Need to incorporate frequency distribution context where relevant: mention top values and percentages; e.g., for Agency code top 10 codes with percentages; for Complaint Type etc.

Also include null count and uniqueness ratio from summary stats where possible.

Let's check summary stats for some fields: e.g., Unique Key has sum, min, max, cardinality 1000000, uniqueness_ratio 1. Created Date: nullcount 841014 (approx 84% missing). For others, we can mention approximate null counts etc.

But the instruction says "Use the Summary Statistics and Frequency Distribution to understand context". So we should incorporate numeric details like percentages of top values maybe.

Ok produce JSON with each field mapping.

We need to ensure output is valid JSON. The keys are strings; but JSON can't have spaces? Actually keys can be strings with spaces, as long as quoted. We'll use the field names exactly as given: "Unique Key", etc. Provide label and description. Ensure no trailing commas.

Let's craft.

Also we might want to escape quotes inside description properly for JSON. Use double quotes and escape internal double quotes with \".

We must produce final answer only with JSON, not any other commentary.

Let's build the JSON object gradually.

We'll include each field as key: e.g., "Unique Key": { ... }.

Will we need to use backticks? No.

Make sure no trailing comma after last entry.

Ok.

I'll craft descriptions now.

1. Unique Key: label "Record Identifier". Description: "A unique numeric identifier for each complaint record. It is the primary key in the dataset and has 1,000,000 distinct values (100% uniqueness)."

2. Created Date: label "Complaint Creation Timestamp". Description: "UTC timestamp indicating when a 311 service request was logged. The dates span from January 1 2010 to December 23 2020 with a mean around November 10 2015. Approximately 84% of records have missing values."

3. Closed Date: label "Complaint Closure Timestamp". Description: "UTC timestamp marking the final resolution or closure of the complaint. Valid dates range from January 1 1900 to January 1 2100, but most entries are null (≈99% missing). The top recorded dates show that complaints were frequently closed in 2010–2020."

4. Agency: label "Agency Code". Description: "Two- or three-character code identifying the city agency responsible for handling the complaint. The most common codes are NYPD (26.5 %), HPD (25.8 %), DOT (13.2 %) and DSNY (8.2 %). A small proportion of records have an 'Other' code."

5. Agency Name: label "Agency Full Name". Description: "Full name of the agency that processed or responded to the complaint. The top agencies include New York City Police Department, Department of Housing Preservation and Development, Department of Transportation, etc., together accounting for about 70 % of records."

6. Complaint Type: label "Complaint Category". Description: "High-level classification of the reported issue, such as Noise - Residential, HEAT/HOT WATER, Illegal Parking, Blocked Driveway, Street Condition, etc. The majority of complaints (≈56 %) fall under an 'Other' category, while the top ten categories represent roughly 43 % of all records."

7. Descriptor: label "Complaint Detail". Description: "Fine-grained description within a complaint type (e.g., Loud Music/Party, Heat, No Access). The most frequent descriptors include Loud Music/Party (~9.4 %) and other subcategories, with about 67 % of entries labeled as 'Other'."

8. Location Type: label "Location Category". Description: "General classification of where the incident occurred—Residential Building, Street/Sidewalk, etc. Residential Building accounts for ~34 %, Street/Sidewalk ~19 %, and many records are missing."

9. Incident Zip: label "Incident ZIP Code". Description: "Five-digit ZIP code indicating the area of the complaint. The most common ZIP codes include 11226 (~1.8 %), 10467 (~1.5 %), etc., while 86 % of records have a valid ZIP; about 6 % are missing."

10. Incident Address: label "Incident Address". Description: "Full street address or intersection where the issue was reported. The dataset contains many distinct addresses, with the top ten appearing in ~0.2 % each; nearly all records fall into the 'Other' bucket indicating a high level of uniqueness."

11. Street Name: label "Street Name". Description: "Name of the primary street involved in the incident. Broadway is the most common (≈1.2 %) followed by other major streets; about 95 % of values are unique or appear rarely."

12. Cross Street 1: label "Cross Street 1". Description: "First cross street at an intersection complaint location, e.g., BEND or BROADWAY. The top values account for roughly 18 % of entries; the field is null in many records that are not intersections."

13. Cross Street 2: label "Cross Street 2". Description: "Second cross street at an intersection. Distribution mirrors Cross Street 1, with BEND and BROADWAY again among the most frequent values."

14. Intersection Street 1: label "Intersection Street 1". Description: "First street forming an intersection where a complaint was reported; common values include BROADWAY (~3.6 %) and other major streets. The field is often null for non-intersection complaints."

15. Intersection Street 2: label "Intersection Street 2". Description: "Second street of the intersection, with similar distribution to Intersection Street 1."

16. Address Type: label "Address Format". Description: "Specifies how the incident location was recorded—ADDRESS, INTERSECTION, BLOCKFACE, LATLONG, or PLACENAME. ADDRESS dominates at ~81 %, followed by INTERSECTION (~15 %)."

17. City: label "City/Borough". Description: "Name of the borough where the complaint occurred (Brooklyn, Queens, Manhattan, Bronx, Staten Island). Brooklyn is the most frequent borough (~32 %), with many records missing."

18. Landmark: label "Nearby Landmark". Description: "Notable landmark or street close to the incident location, e.g., EAST 230 STREET, BROADWAY. The field is null for ~91 % of records; the top landmarks represent a small portion of the data."

19. Facility Type: label "Facility Category". Description: "Type of facility involved in the complaint (e.g., DSNY Garage, Precinct). N/A accounts for 73 % of entries; Precinct appears in about 23 %; other categories are rare."

20. Status: label "Complaint Status". Description: "Current processing status—Closed, Pending, Open, etc. Closed complaints dominate (~95 %), with small percentages remaining pending or open."

21. Due Date: label "Resolution Deadline". Description: "Target date set by the agency to resolve the complaint, ranging from 2010 to 2021. Many records have missing values (≈65 %); among those present, the most common dates cluster in mid‑2015."

22. Resolution Description: label "Resolution Narrative". Description: "Textual account of how the complaint was resolved—actions taken or outcome noted. The top narratives involve police response or HPD inspections; about 52 % of entries fall under an 'Other' category."

23. Resolution Action Updated Date: label "Last Update Timestamp". Description: "UTC timestamp indicating when the resolution action was last updated. The field has a high proportion of nulls (~99 %) and, where present, dates span from 2010 to 2020."

24. Community Board: label "Community Board". Description: "Borough-level community board responsible for the area (e.g., 12 MANHATTAN). Most records are unspecified; when specified, they range across all boroughs with varying frequencies."

25. BBL: label "BBL (Borough‑Block‑Lot)". Description: "Numeric identifier unique to NYC property parcels. The dataset contains many distinct values; about 24 % of entries are missing. The top ten BBLs account for roughly 0.2 % each."

26. Borough: label "Borough". Description: "One of the five NYC boroughs or Unspecified. Brooklyn is the most common (≈30 %), followed by Queens, Manhattan, Bronx, and Staten Island."

27. X Coordinate (State Plane): label "X Coordinate (State Plane)". Description: "Easting coordinate in the New York State Plane coordinate system, used for precise mapping of complaint locations. Most values cluster around 1,020,000 m; about 10 % of records are missing."

28. Y Coordinate (State Plane): label "Y Coordinate (State Plane)". Description: "Northing coordinate in the State Plane system. Values typically fall near 200,000 m; a small portion of entries are null."

29. Open Data Channel Type: label "Submission Channel". Description: "Method used to submit the complaint—PHONE, UNKNOWN, ONLINE, MOBILE, or OTHER. PHONE is by far the most common channel (~50 %), followed by UNKNOWN (~23 %) and ONLINE (~18 %)."

30. Park Facility Name: label "Park Facility Name". Description: "Name of a park or playground if the incident occurred within a park; the majority of records are Unspecified, with Central Park, Riverside Park, etc., appearing in the minority."

31. Park Borough: label "Park Borough". Description: "Borough where the park is located; distribution mirrors that of Borough (Brooklyn ~30 %, Queens ~23 %, Manhattan ~20 %)."

32. Vehicle Type: label "Vehicle Type". Description: "Type of vehicle involved in the complaint, such as Car Service or Green Taxi. The field is largely null (~99.9 %); among non‑null values, Car Service dominates (~91 %)."

33. Taxi Company Borough: label "Taxi Company Borough". Description: "Borough affiliation of the taxi company that issued the ticket or was involved. Most records are unspecified; when specified, they map to the five boroughs with varying frequencies."

34. Taxi Pick Up Location: label "Taxi Pickup Location". Description: "Origin point for a taxi pickup—common values include JFK Airport, intersection, etc.; the field is null in many cases and 'Other' represents the majority of non‑null entries."

35. Bridge Highway Name: label "Bridge/Highway Name". Description: "Name of bridge or highway associated with the complaint (e.g., Belt Pkwy, BQE/Gowanus Expwy). A handful of names cover most records; a large proportion are labeled as 'Other'."

36. Bridge Highway Direction: label "Bridge/Highway Direction". Description: "Direction of travel on the bridge or highway—East/Long Island Bound, North/Bronx Bound, etc.; the field has a high prevalence of nulls."

37. Road Ramp: label "Road Ramp Type". Description: "Indicates whether a ramp is present at the complaint location (Roadway, Ramp, N/A). Most records have 'Roadway' (~75 %), followed by 'Ramp' (~24 %)."

38. Bridge Highway Segment: label "Bridge/Highway Segment". Description: "Specific segment identifier of the bridge or highway involved; most entries are labeled as 'Other'."

39. Latitude: label "Latitude". Description: "Decimal degree latitude coordinate for the complaint location, ranging from about 40.1 ° to 41.0 °. The majority of points lie between 40.1 and 40.9."

40. Longitude: label "Longitude". Description: "Decimal degree longitude coordinate, typically between -77.5 ° and -73.7 °. Most locations are in the range -74 to -73.8."

41. Location: label "Location (LatLon)". Description: "Textual representation of latitude and longitude as a comma‑separated pair enclosed in parentheses, e.g., '(40.89187241649303, -73.86016845296459)'. The field is null for about 25 % of records."

Need to ensure JSON syntax: double quotes around keys; description string may contain apostrophes etc; we need escape double quotes inside. Use \".

Let's craft final JSON.
## TOKEN USAGE

TokenUsage { prompt: 13666, completion: 7403, total: 21069, elapsed: 97959 }
---
# Description
**Dataset Description**

This NYC 311 service‑request dataset contains one million unique complaint records spanning from January 2010 to December 2020. Each record is identified by a globally unique integer key and includes timestamps for when the request was created, closed, or last updated (though most of these dates are missing). The majority of complaints fall under broad “Other” categories in both *Complaint Type* and *Descriptor*, with Noise‑Residential and HEAT/HOT WATER representing only a small fraction of the total. Geographic information is provided through ZIP codes, street names, intersection data, and State Plane coordinates that cluster around New York City’s five boroughs.

---

### Notable Characteristics

- **High cardinality**: The *Unique Key* field is 100% unique (no duplicates).  
- **Missingness**: Created Date, Closed Date, and many location fields have very high null rates (≈84–99%).  
- **Skewed categorical distributions**: Over half of complaints are labeled “Other” in both *Complaint Type* (56%) and *Descriptor* (67%), indicating a large number of rare or unclassified issues.  
- **Spatial concentration**: Latitude/Longitude values cluster around major NYC boroughs, with the Bronx and Manhattan having the highest densities.  
- **Outliers & range**: Created Date spans 4,009 days (~11 years) while Closed Date covers 73,049 days (~200 years), though most dates fall between 2010–2020. State Plane coordinates lie within expected ranges for NYC.  
- **Data quality issues**: Numerous “Other (N)” categories in categorical fields mask underlying heterogeneity; top ten values cover a small percentage of records.  
- **Potential PII**: Address, ZIP code, and coordinate fields can be used to triangulate individual locations, so care should be taken when sharing the data publicly.  
- **No direct personal identifiers (PII/PHI/PCI)** are present beyond location information.

---

**Attribution**

Generated by qsv v16.0.0 describegpt
Command line: target/debug/qsv describegpt /tmp/NYC_311_SR_2010-2020-sample-1M.csv --all --output /tmp/nyc311-describegpt.md
Prompt file: Default v7.0.0
Model: openai/gpt-oss-20b
LLM API URL: http://localhost:1234/v1
Language: 
Timestamp: 2026-02-10T21:49:01.086845+00:00

WARNING: Description generated by an LLM and may contain inaccuracies. Verify before using!



## REASONING

We need to produce a dataset description based on summary statistics and frequency distribution, but not per field. Provide 1-8 sentence description overall. Then Notable Characteristics bullet list. Avoid field-level data. Use aggregated insights: high missingness for dates etc; majority of complaints 'Other'; unique key all unique; distribution skewness in complaint type, descriptor etc.

Also note that many fields have "Other" categories covering large proportions (e.g., Complaint Type 56%, Descriptor 67%). Many null values for Created Date (~84% missing). Unique Key is unique. There are duplicates? Uniqueness ratio =1 so no duplicates. PII/PHI: likely contains personal addresses, but maybe not personally identifying; still location data.

Also mention spatial coordinates distribution across NYC with many points in Bronx etc.

Add attribution placeholder. Provide Markdown.

Let's craft description 3-4 sentences.

Notable characteristics bullet list:

- Unique key ensures one-to-one record.
- Created Date missing ~84% ; Closed Date missing ~99.7%; majority of dates in 2010-2020.
- Complaint Type skewed heavily to 'Other' (56%) and Noise Residential (9%).
- Descriptor also dominated by Other (~67%).
- Location fields: Incident Zip with 86% other; many addresses unique but top 10 cover ~1.8%.
- Spatial coordinates concentrated in Bronx/Manhattan, etc.
- Missing values high in Address Type, City, Landmarks.
- 'Other' categories capture majority of categorical data indicating low granularity or many rare categories.
- Outliers: Created Date has 4009 days range; closed date 73049 days; coordinate ranges typical for NYC State Plane.
- No duplicates (unique key). Data includes addresses and coordinates that could be PII but no direct personal identifiers.

Let's produce.
## TOKEN USAGE

TokenUsage { prompt: 15990, completion: 809, total: 16799, elapsed: 25192 }
---
# Tags
- nyc_311_complaints  
- city_government_data  
- public_service_requests  
- urban_infrastructure  
- transportation_incidents  
- housing_and_building_inspections  
- police_and_public_safety  
- noise_reports  
- geospatial_data  
- citizen_engagement  

Generated by qsv v16.0.0 describegpt
Command line: target/debug/qsv describegpt /tmp/NYC_311_SR_2010-2020-sample-1M.csv --all --output /tmp/nyc311-describegpt.md
Prompt file: Default v7.0.0
Model: openai/gpt-oss-20b
LLM API URL: http://localhost:1234/v1
Language: 
Timestamp: 2026-02-10T21:49:20.370763+00:00

WARNING: Tags generated by an LLM and may contain inaccuracies. Verify before using!



## REASONING

We need to output tags in markdown format, no field names. We have dataset about NYC 311 complaints. Should choose up to 10 unique tags, lowercase and underscores.

Need descending importance based on summary stats/frequency distribution. So pick most prominent aspects: city data, complaints, NYC, 311, public service requests, infrastructure issues, transportation, noise, housing, police, emergency services? Also include geospatial coordinates. Maybe "nyc_311_service_requests" but tag must be lowercase with underscore. We need up to 10 tags.

Consider importance: dataset is about NYC 311 complaints; high cardinality and many categories. So top tags: nyc_311_complaints, public_safety_incidents? Actually includes police (NYPD), housing, transportation, etc. Let's think of general tags that help find similar datasets: city_government_data, civic_engagement, public_service_requests, urban_planning, infrastructure_management, transport_services, noise_reports, building_inspections, emergency_response.

But we need to pick up to 10 unique tags in descending importance. Use underscores.

Possible tags:

1. nyc_311_complaints
2. city_government_data
3. public_service_requests
4. urban_infrastructure
5. transportation_incidents
6. housing_and_building_inspections
7. police_and_public_safety
8. noise_reports
9. geospatial_data
10. citizen_engagement

Make sure no field names used; these are generic.

Check if "city_government_data" maybe too generic? But okay.

Return as markdown list with bullet points? The instruction: "Add an Attribution with the placeholder "{GENERATED_BY_SIGNATURE}" after the Tags." It doesn't say format, but says tags in Markdown format. So we can produce a bullet list or just comma separated? Probably bullet list is fine.

Thus output:

- nyc_311_complaints
- city_government_data
...

At bottom add attribution line: "{GENERATED_BY_SIGNATURE}"

Let's do that.
## TOKEN USAGE

TokenUsage { prompt: 15965, completion: 489, total: 16454, elapsed: 19276 }
---
