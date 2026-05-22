Generated using a Local LLM (openai/gpt-oss-20b) on LM Studio 0.4.14-beta+3 running on a Macbook Pro M4 Max 64gb/Tahoe 26.5:

```bash
qsv describegpt --all --two-pass /tmp/NYC_311_SR_2010-2020-sample-1M.csv -o /tmp/nyc311-describegpt.md --fresh --infer-content-type
```
---
# Dictionary
| Name | Type | Label | Description | Content Type | Min | Max | Cardinality | Enumeration | Null Count | Examples |
|------|------|-------|-------------|--------------|-----|-----|-------------|-------------|------------|----------|
| **Unique Key** | Integer | Unique Complaint Identifier | A system‑generated numeric identifier that uniquely distinguishes each complaint record. | unique_id | 11465364 | 48478173 | 1,000,000 |  | 0 | <ALL_UNIQUE> |
| **Created Date** | DateTime | Complaint Creation Timestamp | The exact date and time when the complaint was created or submitted by a user. | date:%m/%d/%Y | 2010-01-01T00:00:00+00:00 | 2020-12-23T01:25:51+00:00 | 841,014 |  | 0 | Other… [997,333]<br>01/24/2013 12:00:00 AM [347]<br>01/07/2014 12:00:00 AM [315]<br>01/08/2015 12:00:00 AM [283]<br>02/16/2015 12:00:00 AM [269] |
| **Closed Date** | DateTime | Complaint Closure Timestamp | The date and time when the complaint was closed or resolved. May be null if still open. | date:%m/%d/%Y | 1900-01-01T00:00:00+00:00 | 2100-01-01T00:00:00+00:00 | 688,837 |  | 28,619 | Other… [968,671]<br>(NULL)… [28,619]<br>11/15/2010 12:00:00 AM [384]<br>11/07/2012 12:00:00 AM [329]<br>12/09/2010 12:00:00 AM [267] |
| **Agency** | String | Complaint Agency Abbreviation | Two‑ to four‑letter abbreviation of the agency responsible for handling the complaint (e.g., NYPD, HPD). | category | 3-1-1 | TLC | 28 |  | 0 | NYPD [265,116]<br>HPD [258,033]<br>DOT [132,462]<br>DSNY [81,606]<br>DEP [75,895] |
| **Agency Name** | String | Complaint Agency Full Name | Full legal name of the agency tasked with addressing the complaint. | company_name | 3-1-1 | Valuation Policy | 553 |  | 0 | New York City Police Depa… [265,038]<br>Department of Housing Pre… [258,019]<br>Department of Transportat… [132,462]<br>Other… [103,974]<br>Department of Environment… [75,895] |
| **Complaint Type** | String | Primary Complaint Category | Broad category describing the nature of the complaint (e.g., Noise, Water System). | category | ../../WEB-INF/web.xml;x= | ZTESTINT | 287 |  | 0 | Other… [563,561]<br>Noise - Residential [89,439]<br>HEAT/HOT WATER [56,639]<br>Illegal Parking [45,032]<br>Blocked Driveway [42,356] |
| **Descriptor** | String | Specific Issue Descriptor | A more detailed description of the issue within the broader complaint type. | category | 1 Missed Collection | unknown odor/taste in drinking water (QA6) | 1,392 |  | 3,001 | Other… [671,870]<br>Loud Music/Party [93,646]<br>ENTIRE BUILDING [36,885]<br>HEAT [35,088]<br>No Access [31,631] |
| **Location Type** | String | Incident Location Classification | Classification of where the incident occurred (e.g., Residential Building, Street/Sidewalk). | category | 1-, 2- and 3- Family Home | Wooded Area | 162 |  | 239,131 | RESIDENTIAL BUILDING [255,562]<br>(NULL)… [239,131]<br>Street/Sidewalk [145,653]<br>Residential Building/Hous… [92,765]<br>Street [92,190] |
| **Incident Zip** | String | Incident ZIP Code | The five‑digit postal code of the location where the incident took place. | zip_code | * | XXXXX | 535 |  | 54,978 | Other… [815,988]<br>(NULL)… [54,978]<br>11226 [17,114]<br>10467 [14,495]<br>11207 [12,872] |
| **Incident Address** | String | Incident Street Address | Full street address of the incident location, including building number and street name. This is one component of the complete mailing address. | street_address | * * | west 155 street and edgecombe avenue | 341,996 |  | 174,700 | Other… [819,046]<br>(NULL)… [174,700]<br>655 EAST  230 STREET [1,538]<br>78-15 PARSONS BOULEVARD [694]<br>672 EAST  231 STREET [663] |
| **Street Name** | String | Primary Incident Street Name | Name of the main street where the incident occurred; part of the full address when combined with Incident Address, City, Borough and ZIP Code. | street_name | * | wyckoff avenue | 14,837 |  | 174,720 | Other… [784,684]<br>(NULL)… [174,720]<br>BROADWAY [9,702]<br>GRAND CONCOURSE [5,851]<br>OCEAN AVENUE [3,946] |
| **Cross Street 1** | String | First Cross Street (if applicable) | First cross street at an intersection near the incident location; part of the intersection context for address resolution. | street_name | 1 AVE | mermaid | 16,238 |  | 320,401 | Other… [619,743]<br>(NULL)… [320,401]<br>BEND [12,562]<br>BROADWAY [8,548]<br>3 AVENUE [6,154] |
| **Cross Street 2** | String | Second Cross Street (if applicable) | Second cross street at an intersection near the incident location; used when the incident involves a multi‑intersection area. | street_name | 1 AVE | surf | 16,486 |  | 323,644 | Other… [623,363]<br>(NULL)… [323,644]<br>BEND [12,390]<br>BROADWAY [8,833]<br>DEAD END [5,626] |
| **Intersection Street 1** | String | First Intersection Street | One of the streets that form the intersection where the incident occurred. Useful for mapping incidents to grid intersections. | street_name | 1 AVE | flatlands AVE | 11,237 |  | 767,422 | (NULL)… [767,422]<br>Other… [214,544]<br>BROADWAY [3,761]<br>CARPENTER AVENUE [2,918]<br>BEND [2,009] |
| **Intersection Street 2** | String | Second Intersection Street | The other street forming the intersection at the incident location. | street_name | 1 AVE | glenwood RD | 11,674 |  | 767,709 | (NULL)… [767,709]<br>Other… [215,667]<br>BROADWAY [3,462]<br>BEND [1,942]<br>2 AVENUE [1,690] |
| **Address Type** | String | Type of Address Provided | Indicates how the address was supplied (e.g., ADDRESS, INTERSECTION, BLOCKFACE). | category | ADDRESS | PLACENAME | 6 |  | 125,802 | ADDRESS [710,380]<br>INTERSECTION [133,361]<br>(NULL)… [125,802]<br>BLOCKFACE [22,620]<br>LATLONG [7,421] |
| **City** | String | Incident City | Name of the city where the incident took place. For NYC data this is typically a borough name but can be “New York” or other municipalities. | city | * | YORKTOWN HEIGHTS | 382 |  | 61,963 | BROOKLYN [296,254]<br>NEW YORK [189,069]<br>BRONX [181,168]<br>Other… [163,936]<br>(NULL)… [61,963] |
| **Landmark** | String | Nearby Landmark (if any) | Notable landmark near or at the incident location; used for contextual navigation and mapping. | category | 1 AVENUE | ZULETTE AVENUE | 5,915 |  | 912,779 | (NULL)… [912,779]<br>Other… [80,165]<br>EAST  230 STREET [1,545]<br>EAST  231 STREET [1,291]<br>BROADWAY [1,148] |
| **Facility Type** | String | Incident Facility Category | Type of facility where the incident occurred (e.g., DSNY Garage, School). | category | DSNY Garage | School District | 6 |  | 145,478 | N/A [628,279]<br>Precinct [193,259]<br>(NULL)… [145,478]<br>DSNY Garage [32,310]<br>School [617] |
| **Status** | String | Current Complaint Status | Real‑time status of the complaint (Closed, Pending, Open, etc.). | category | Assigned | Unspecified | 10 | Assigned<br>Closed<br>Closed - Testing<br>Email Sent<br>In Progress<br>Open<br>Pending<br>Started<br>Unassigned<br>Unspecified | 0 | Closed [952,522]<br>Pending [20,119]<br>Open [12,340]<br>In Progress [7,841]<br>Assigned [6,651] |
| **Due Date** | DateTime | Resolution Deadline Timestamp | The deadline date and time by which the complaint should be resolved. | datetime:%m/%d/%Y %I:%M:%S %p | 1900-01-02T00:00:00+00:00 | 2021-06-17T16:34:13+00:00 | 345,077 |  | 647,794 | (NULL)… [647,794]<br>Other… [350,746]<br>04/08/2015 10:00:58 AM [214]<br>05/02/2014 03:32:17 PM [183]<br>03/30/2018 10:10:39 AM [172] |
| **Resolution Description** | String | Resolution Narrative | Narrative description of actions taken or resolution details for the complaint. | free_text | A DOB violation was issued for failing to comply with an existing Stop Work Order. | Your request was submitted to the Department of Homeless Services. The City?s outreach team will assess the homeless individual and offer appropriate assistance within 2 hours. If you asked to know the outcome of your request, you will get a call within 2 hours. No further status will be available through the NYC 311 App, 311, or 311 Online. | 1,216 |  | 20,480 | Other… [511,739]<br>The Police Department res… [91,408]<br>The Department of Housing… [72,962]<br>The Police Department res… [63,868]<br>Service Request status fo… [52,155] |
| **Resolution Action Updated Date** | DateTime | Last Resolution Update Timestamp | Timestamp when the resolution action was last updated. | date:%m/%d/%Y | 2009-12-31T01:35:00+00:00 | 2020-12-23T06:56:14+00:00 | 690,314 |  | 15,072 | Other… [982,148]<br>(NULL)… [15,072]<br>11/15/2010 12:00:00 AM [385]<br>11/07/2012 12:00:00 AM [336]<br>12/09/2010 12:00:00 AM [273] |
| **Community Board** | String | Associated Community Board | Community board number or designation linked to the incident location. | category | 0 Unspecified | Unspecified STATEN ISLAND | 77 |  | 0 | Other… [751,635]<br>0 Unspecified [49,878]<br>12 MANHATTAN [29,845]<br>12 QUEENS [23,570]<br>01 BROOKLYN [21,714] |
| **BBL** | String | Borough‑Block‑Lot Identifier | Standard NYPD BBL code identifying the borough, block, and lot of the property involved in the complaint. | unknown | 0000000000 | 5080470043 | 268,383 |  | 243,046 | Other… [750,668]<br>(NULL)… [243,046]<br>2048330028 [1,566]<br>4068290001 [696]<br>4015110001 [664] |
| **Borough** | String | Incident Borough | Name of the borough where the incident occurred. | category | BRONX | Unspecified | 6 | BRONX<br>BROOKLYN<br>MANHATTAN<br>QUEENS<br>STATEN ISLAND<br>Unspecified | 0 | BROOKLYN [296,081]<br>QUEENS [228,818]<br>MANHATTAN [195,488]<br>BRONX [180,142]<br>Unspecified [49,878] |
| **X Coordinate (State Plane)** | Integer | Easting (State Plane) | East‑ing coordinate in the New York State Plane coordinate system for the incident location; forms part of a state‑plane coordinate pair with Y Coordinate. | unknown | 913281 | 1067220 | 102,556 |  | 85,327 | Other… [908,535]<br>(NULL)… [85,327]<br>1022911 [1,568]<br>1037000 [701]<br>1023174 [675] |
| **Y Coordinate (State Plane)** | Integer | Northing (State Plane) | North‑ing coordinate in the New York State Plane coordinate system for the incident location; pairs with X Coordinate to locate the point precisely. | unknown | 121152 | 271876 | 116,092 |  | 85,327 | Other… [908,538]<br>(NULL)… [85,327]<br>264242 [1,566]<br>202363 [706]<br>211606 [665] |
| **Open Data Channel Type** | String | Submission Channel | Channel through which the complaint was submitted (e.g., PHONE, ONLINE). | category | MOBILE | UNKNOWN | 5 | MOBILE<br>ONLINE<br>OTHER<br>PHONE<br>UNKNOWN | 0 | PHONE [497,606]<br>UNKNOWN [230,402]<br>ONLINE [177,334]<br>MOBILE [79,892]<br>OTHER [14,766] |
| **Park Facility Name** | String | Park Facility Name | Name of the park facility involved in the incident. | free_text | "Uncle" Vito F. Maranzano Glendale Playground | Zimmerman Playground | 1,889 |  | 0 | Unspecified [993,141]<br>Other… [5,964]<br>Central Park [261]<br>Riverside Park [136]<br>Prospect Park [129] |
| **Park Borough** | String | Park Borough | Borough where the park facility is located. | category | BRONX | Unspecified | 6 | BRONX<br>BROOKLYN<br>MANHATTAN<br>QUEENS<br>STATEN ISLAND<br>Unspecified | 0 | BROOKLYN [296,081]<br>QUEENS [228,818]<br>MANHATTAN [195,488]<br>BRONX [180,142]<br>Unspecified [49,878] |
| **Vehicle Type** | String | Vehicle Category (if applicable) | Type of vehicle involved in the incident (e.g., Car Service, Green Taxi). | category | Ambulette / Paratransit | Green Taxi | 5 |  | 999,652 | (NULL)… [999,652]<br>Car Service [317]<br>Ambulette / Paratransit [19]<br>Commuter Van [11]<br>Green Taxi [1] |
| **Taxi Company Borough** | String | Taxi Company Operating Borough | Borough where the taxi company operates. | category | BRONX | Staten Island | 11 |  | 999,156 | (NULL)… [999,156]<br>BROOKLYN [207]<br>QUEENS [194]<br>MANHATTAN [171]<br>BRONX [127] |
| **Taxi Pick Up Location** | String | Taxi Pickup Description | Textual description of the location where a taxi was picked up. | free_text | 1 5 AVENUE MANHATTAN | YORK AVENUE AND EAST 70 STREET | 1,903 |  | 992,129 | (NULL)… [992,129]<br>Other [4,091]<br>Other… [2,006]<br>JFK Airport [562]<br>Intersection [486] |
| **Bridge Highway Name** | String | Bridge/Highway Name (if applicable) | Name of the bridge or highway involved in the incident. | category | 145th St. Br - Lenox Ave | Willis Ave Br - 125th St/1st Ave | 68 |  | 997,711 | (NULL)… [997,711]<br>Other… [779]<br>Belt Pkwy [276]<br>BQE/Gowanus Expwy [254]<br>Grand Central Pkwy [186] |
| **Bridge Highway Direction** | String | Bridge/Highway Traffic Direction | Direction of traffic on the bridge or highway (e.g., East/Long Island Bound). | category | Bronx Bound | Westbound/To Goethals Br | 50 |  | 997,691 | (NULL)… [997,691]<br>Other… [987]<br>East/Long Island Bound [210]<br>North/Bronx Bound [208]<br>East/Queens Bound [197] |
| **Road Ramp** | String | Ramp Type (if applicable) | Type of ramp at the incident location (e.g., Roadway, N/A). | category | N/A | Roadway | 4 |  | 997,693 | (NULL)… [997,693]<br>Roadway [1,731]<br>Ramp [555]<br>N/A [21] |
| **Bridge Highway Segment** | String | Bridge/Highway Segment Identifier | Specific segment or exit number on the bridge or highway. | category | 1-1-1265963747 | Wythe Ave/Kent Ave (Exit 31) | 937 |  | 997,556 | (NULL)… [997,556]<br>Other… [2,144]<br>Ramp [92]<br>Roadway [54]<br>Clove Rd/Richmond Rd (Exi… [23] |
| **Latitude** | Float | Geographic Latitude | Geographic latitude coordinate of the incident location; used with Longitude to form a geographic point. | latitude | 40.1123853 | 40.9128688 | 353,694 |  | 254,695 | Other… [739,329]<br>(NULL)… [254,695]<br>40.89187241649303 [1,538]<br>40.1123853 [1,153]<br>40.89238451539139 [663] |
| **Longitude** | Float | Geographic Longitude | Geographic longitude coordinate of the incident location; pairs with Latitude to locate the point precisely. | longitude | -77.5195844 | -73.7005968 | 353,996 |  | 254,695 | Other… [739,329]<br>(NULL)… [254,695]<br>-73.86016845296459 [1,538]<br>-77.5195844 [1,153]<br>-73.8592161325675 [663] |
| **Location** | String | Coordinate Textual Representation | Textual representation of the geographic coordinates in parentheses (e.g., "(40.1123853, -77.5195844)"). | free_text | (40.1123853, -77.5195844) | (40.9128688, -73.9024731) | 375,772 |  | 254,695 | Other… [739,329]<br>(NULL)… [254,695]<br>(40.89187241649303, -73.8… [1,538]<br>(40.1123853, -77.5195844) [1,153]<br>(40.89238451539139, -73.8… [663] |

*Attribution: Generated by qsv v20.1.0 describegpt
Command line: target/debug/qsv describegpt --all --two-pass /tmp/NYC_311_SR_2010-2020-sample-1M.csv -o /tmp/nyc311-describegpt.md --fresh --infer-content-type
Prompt file: Default v7.1.0
Model: openai/gpt-oss-20b
LLM API URL: http://localhost:1234/v1
Language: 
Timestamp: 2026-05-22T01:16:02.725245+00:00

WARNING: Label and Description generated by an LLM and may contain inaccuracies. Verify before using!
*

## REASONING

FIRST PASS REASONING:
We need to produce JSON mapping field names to label, description, content_type.

We have 41 fields.

We must use summary stats and frequency distribution for context.

We must assign content_type tokens from allowed list; with special suffixes for date/datetime and duration if applicable. Need to examine each field:

List fields:

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

We must pick content types.

From allowed tokens list: first_name, last_name, full_name, username, password, email, phone, street_address, street_name, building_number, secondary_address, city, state, state_abbr, zip_code, country, country_code, latitude, longitude, time_zone, company_name, industry, job_title, profession, unique_id, uuid, credit_card, currency_code, isbn, ip_address, ipv6_address, mac_address, url, user_agent, file_name, file_path, mime_type, color_hex, license_plate, date, datetime, time, duration, category, lorem_word, lorem_sentence, lorem_paragraph, free_text, unknown

We must consider each field semantics.

Unique Key: integer unique id. cardinality equals row count 1M. deterministic type is Integer; but content_type "unique_id" reserved? Actually spec says unique_id is reserved for when cardinality equals row count and qsv overrides. But we can't use unique_id ourselves; we should pick a more specific token if possible. For a key, maybe "uuid" or "unknown". The field likely an integer surrogate key. There's no hint it's uuid. So choose "unique_id"? but instruction: DO NOT use "unique_id" yourself - it will be ignored if you do. For such fields, pick the most specific other token that fits (e.g., "uuid", "isbn"); the deterministic override replaces it only when the row-count check matches.

Thus we should not set content_type to unique_id; we could set "unknown". Maybe "unknown" is safe. But maybe choose "unique_id" would be ignored. So use "unknown".

Created Date: Type DateTime per summary stats. Format? Frequency distribution shows format "MM/DD/YYYY hh:mm:ss AM/PM"? Actually values like "01/24/2013 12:00:00 AM". So date-time format is "%m/%d/%Y %I:%M:%S %p" maybe. We need suffix: datetime:<fmt>. So content_type: datetime:%m/%d/%Y %I:%M:%S %p.

Closed Date similar; same format. Use same suffix.

Agency: String, values like "NYPD", etc. Category of agency. Content type: category? The allowed token includes 'category' for classification labels. Yes use "category".

Agency Name: full name of agency. Could be company_name or organization? Probably "company_name" fits. Or maybe "free_text". But more specific: company_name.

Complaint Type: categories like "Noise - Residential", etc. So category.

Descriptor: descriptors describing complaint; maybe free_text? It's a description of issue. Might be category as well but many values; could treat as category (since it's a type). We'll use category.

Location Type: categories like RESIDENTIAL BUILDING, STREET/Sidewalk etc. Category.

Incident Zip: zip codes string or numeric. Use zip_code.

Incident Address: full address. Could be street_address? But includes number, street name, maybe city etc. So "street_address".

Street Name: just street name. Use street_name.

Cross Street 1: cross street names; could be street_name. Might also include "N/A". We'll use street_name.

Cross Street 2: same.

Intersection Street 1: street_name.

Intersection Street 2: street_name.

Address Type: type of address like ADDRESS, INTERSECTION, BLOCKFACE, LATLONG, PLACENAME. Category maybe? It's a classification; could be category.

City: city name. Use city.

Landmark: landmark names. Could be free_text but maybe category. But it's a descriptor of location. Might use free_text? But allowed token "lorem_word" etc not relevant. I'd choose category or free_text? The values are names like "EAST 230 STREET". It's likely category, but we could treat as free_text. I'll pick category.

Facility Type: type of facility; categories. Use category.

Status: status of complaint; category.

Due Date: date/time format similar to Created and Closed? Format "MM/DD/YYYY hh:mm:ss AM/PM"? Yes frequency shows "04/08/2015 10:00:58 AM". So datetime with same fmt.

Resolution Description: free text description. Use free_text.

Resolution Action Updated Date: date/time same format as others. datetime suffix.

Community Board: string like "0 Unspecified" etc; category? Could be numeric board ID or name. Use category.

BBL: BBL (Borough, Block, Lot). It is an integer ID of 10 digits. It's unique key for property. But not necessarily surrogate. We'll use unknown or maybe category? Since it's identifier but not globally unique? Could be unique_id? cardinality about 1M? The summary says cardinality 1M. Unique values: BBL column has many distinct; but not all rows unique? It seems many duplicates. So content_type unknown.

Borough: borough name; category or state_abbr? It's borough names like "BRONX". Use category or maybe state_abbr? But it's not abbreviation. So category.

X Coordinate (State Plane): integer coordinate. Not location coordinate but numeric value. Should be unknown.

Y Coordinate (State Plane): same.

Open Data Channel Type: type of channel; category.

Park Facility Name: name of park facility; free_text or category. Many unique values. Use free_text maybe? But category might be fine. I'll use free_text because it's names like "Central Park". Could treat as category but many distinct. I'd choose free_text.

Park Borough: same as borough, category.

Vehicle Type: categories like Car Service etc. Category.

Taxi Company Borough: borough; category.

Taxi Pick Up Location: location description; could be free_text. Use free_text.

Bridge Highway Name: name of bridge or highway; category.

Bridge Highway Direction: direction; category.

Road Ramp: type of ramp; category.

Bridge Highway Segment: segment names; category.

Latitude: coordinate value. content_type latitude.

Longitude: coordinate value. longitude.

Location: location string of lat/long pair. Could be free_text or maybe "street_address"? It's coordinates in parentheses. Use free_text? But could treat as latitude/longitude but we already have separate fields. So use free_text.

Now ensure suffix for date/datetime: Created Date, Closed Date, Due Date, Resolution Action Updated Date: all same format "%m/%d/%Y %I:%M:%S %p". Need to check if any times include AM/PM. Yes.

Also maybe other datetime fields? No.

Check cardinality unique for Unique Key; we use unknown.

We must also consider "Unique Key" may be integer but no further context; unknown.

Ok produce JSON mapping field names exactly as in dataset: e.g., "Unique Key", "Created Date" etc. Ensure keys are strings with same case.

Let's craft description for each field:

- Unique Key: unique numeric identifier for each complaint record.

- Created Date: timestamp when the complaint was created/submitted by user.

- Closed Date: timestamp when the complaint was closed/resolved; may be null.

- Agency: abbreviation of agency responsible for handling complaint (e.g., NYPD, HPD).

- Agency Name: full name of agency.

- Complaint Type: broad category describing nature of complaint (Noise, Water System etc.)

- Descriptor: more specific description of issue within complaint type.

- Location Type: classification of location where incident occurred (Residential Building, Street/Sidewalk etc.)

- Incident Zip: ZIP code of incident location.

- Incident Address: full street address of incident location.

- Street Name: name of the street where incident occurred.

- Cross Street 1: first cross street at intersection.

- Cross Street 2: second cross street.

- Intersection Street 1: one street in intersection.

- Intersection Street 2: other street.

- Address Type: type descriptor for address field (ADDRESS, INTERSECTION etc.)

- City: city of incident location.

- Landmark: notable landmark near incident location.

- Facility Type: type of facility where incident occurred.

- Status: current status of complaint (Closed, Pending, etc.)

- Due Date: deadline date/time to resolve complaint.

- Resolution Description: narrative description of actions taken/resolution.

- Resolution Action Updated Date: timestamp when resolution action was last updated.

- Community Board: community board number or name for location.

- BBL: Borough Block Lot identifier.

- Borough: borough of incident location.

- X Coordinate (State Plane): easting coordinate in state plane system.

- Y Coordinate (State Plane): northing coordinate.

- Open Data Channel Type: channel through which complaint was submitted (PHONE, ONLINE etc.)

- Park Facility Name: name of park facility involved.

- Park Borough: borough where park is located.

- Vehicle Type: type of vehicle involved in incident.

- Taxi Company Borough: borough of taxi company.

- Taxi Pick Up Location: location description of taxi pickup.

- Bridge Highway Name: name of bridge or highway.

- Bridge Highway Direction: direction of traffic on bridge/highway.

- Road Ramp: type of ramp (Roadway, N/A etc.)

- Bridge Highway Segment: specific segment identifier.

- Latitude: geographic latitude coordinate.

- Longitude: longitude coordinate.

- Location: textual representation of coordinates.

Now produce JSON mapping. Use proper escaping for quotes in keys? In JSON string values we need to escape quotes inside strings. Eg "Park Facility Name" key; value "Park Facility Name". But the key string includes space, fine. For values like description containing commas etc; just use double quotes and escape internal quotes if any.

Also for content_type: string token maybe with suffix e.g., "datetime:%m/%d/%Y %I:%M:%S %p".

Now produce final JSON object.

Check field names exactly: we have fields list. Ensure each key matches original column names, including parentheses, spaces, etc.

Edge cases: 'X Coordinate (State Plane)' and 'Y Coordinate (State Plane)' include parentheses; okay as key.

Also note that the dataset may use backticks or quotes? We'll use as provided.

Now craft JSON.

Let's produce mapping.

REFINE PASS REASONING:
We need to produce refined data dictionary for all 41 fields. We must adjust labels/descriptions and content types, especially grouping composite concepts.

First pass had each field individually; now we refine cross-field relationships.

Need to output JSON with keys being the original field names? The first pass uses "fields": list of objects each has name. But new format expects top-level keys mapping field names to object with label, description, content_type. So we need to produce that shape.

We must also adjust content types: if unique_id, omit key. Unique Key is unique numeric identifier; first-pass content_type omitted? Actually they set "content_type": "unique_id" but instruction says for unique_id omit the key entirely from output. So for Unique Key field we should not include content_type. Also maybe other fields could be unique id? Only that.

Also refine others: e.g., Created Date and Closed Date: date-time tokens with suffix. Provided first-pass had content_type "date:%m/%d/%Y". But values show times: "01/24/2013 12:00:00 AM". So datetime format is "%m/%d/%Y %I:%M:%S %p" maybe. They set date; we should use datetime token with suffix. Similarly for Due Date and Resolution Action Updated Date: same.

Also refine labels: e.g., "Agency" -> "Complaint Agency Abbreviation", "Agency Name" -> "Complaint Agency Full Name". Good.

Combine address fields: Street Name, Cross Street 1/2, Intersection Street 1/2, Incident Address, etc. Provide description referencing composite.

Latitude/Longitude: group as geographic coordinate pair; maybe mention latitude and longitude separately but note they form coordinate.

Also maybe X/Y coordinates (State Plane) can be combined as state plane coordinate pair.

Bridge Highway fields: Name, Direction, Segment, Road Ramp: could describe as "Bridge/Highway Incident Details".

Park Facility Name and Park Borough: combine.

Vehicle Type, Taxi Company Borough, Taxi Pick Up Location: group as taxi incident details.

Also some fields may not need changes.

We must produce JSON with each field key. For content_type we need to pick appropriate token from allowed list or leave out if unique_id.

Let's go through each field:

1. Unique Key -> keep label maybe "Unique Complaint Identifier" but no content_type.

2. Created Date: datetime. Content_type: datetime:%m/%d/%Y %I:%M:%S %p

3. Closed Date: same.

4. Agency: abbreviation of agency. content_type: category (since limited set). Keep.

5. Agency Name: full name; content_type: company_name.

6. Complaint Type: category.

7. Descriptor: category.

8. Location Type: category.

9. Incident Zip: zip_code.

10. Incident Address: street_address? They had content_type street_address. But refine: "Incident Street Address" maybe.

11. Street Name: street_name; but mention part of address.

12. Cross Street 1: street_name (but could be "Cross Street 1" label). Keep.

13. Cross Street 2: same.

14. Intersection Street 1: street_name.

15. Intersection Street 2: street_name.

16. Address Type: category.

17. City: city.

18. Landmark: free_text? They had category; maybe keep category.

19. Facility Type: category.

20. Status: category.

21. Due Date: datetime:%m/%d/%Y %I:%M:%S %p

22. Resolution Description: free_text.

23. Resolution Action Updated Date: datetime:%m/%d/%Y %I:%M:%S %p

24. Community Board: category.

25. BBL: unknown? content_type unknown; keep as unknown maybe. But could be string but treat as unknown.

26. Borough: category (state_abbr? Actually borough names). Keep category.

27. X Coordinate (State Plane): integer coordinate; we might set "unknown" or maybe use numeric. Allowed tokens don't include numeric type; use unknown.

28. Y Coordinate (State Plane): same.

29. Open Data Channel Type: category.

30. Park Facility Name: free_text? Could be category but treat as free_text or category? Might keep category, but earlier had free_text. Keep free_text maybe but not needed.

31. Park Borough: category.

32. Vehicle Type: category.

33. Taxi Company Borough: category.

34. Taxi Pick Up Location: free_text.

35. Bridge Highway Name: category.

36. Bridge Highway Direction: category.

37. Road Ramp: category.

38. Bridge Highway Segment: category.

39. Latitude: latitude

40. Longitude: longitude

41. Location: free_text (textual representation of coordinates). But we might not need content_type; keep free_text.

Also we may adjust labels to reflect composite relationships: e.g., for Street Name, Cross Street 1/2 etc. Add description referencing address.

We also may note that Incident Address includes building number and street name but cross streets may be missing.

Also X/Y coordinates maybe part of state plane coordinate pair; mention in description.

Let's craft final JSON.

Need to output each field as key: string field name (exact). Value object with label, description, content_type optionally. For Unique Key omit content_type.

Make sure to use allowed tokens.

Check if any other fields may be unique_id? BBL maybe unique? But cardinality 268383 vs records 1M; not unique. So keep unknown.

Now produce JSON.


## TOKEN USAGE

TokenUsage { prompt: 22662, completion: 7424, total: 30086, elapsed: 104306 }
---
# Description
**Description**  
The dataset contains one million unique complaints filed in New York City between January 2010 and December 2020. Each complaint is identified by a system‑generated integer key and includes timestamps for creation, closure, and resolution updates, with 28 % of records missing a closure date. The majority of complaints (≈70 %) originate from the NYPD or HPD agencies, while the remaining are distributed across 27 other city departments. Geospatial fields provide state‑plane coordinates, latitude/longitude pairs, and address components, enabling precise mapping of incidents throughout all five boroughs. Narrative fields capture complaint descriptors and resolution descriptions, with a substantial proportion of complaints (≈60 %) falling into an “Other” category for both type and descriptor.

**Notable Characteristics**

- **Central tendency & spread** – Created dates are tightly clustered around early‑2013–2015, with the 25th, 50th, and 75th percentiles at 2010‑08‑10, 2011‑03‑15, and 2015‑01‑21 respectively; the range spans roughly 11 years.  
- **Distribution shape** – Most categorical fields exhibit a long‑tailed “Other” bucket (e.g., Complaint Type: 56 % Other, Descriptor: 67 % Other), indicating high sparsity in fine‑grained categories.  
- **Missing values** – 28 k records lack a Closed Date; 648 k are missing a Due Date; 204 k have no Resolution Description; latitude/longitude and coordinate strings are null for about 25 % of rows.  
- **Outliers & anomalies** – State‑plane coordinates include extreme values (e.g., X ≈ 1,067,220) that fall outside the typical NYC range, suggesting data entry errors or boundary cases.  
- **Duplicate records** – The Unique Key is guaranteed distinct; no record duplicates exist at the primary key level.  
- **PII/PHI concerns** – The dataset contains full street addresses, latitude/longitude, and borough identifiers that can be used to pinpoint individuals’ residences or workplaces; these fields should be treated as sensitive under privacy regulations.  
- **Data quality issues** – A large proportion of entries are labeled “Other (N)” for several fields, which obscures the underlying variety and hampers downstream analytics without additional disaggregation.  

Generated by qsv v20.1.0 describegpt
Command line: target/debug/qsv describegpt --all --two-pass /tmp/NYC_311_SR_2010-2020-sample-1M.csv -o /tmp/nyc311-describegpt.md --fresh --infer-content-type
Prompt file: Default v7.1.0
Model: openai/gpt-oss-20b
LLM API URL: http://localhost:1234/v1
Language: 
Timestamp: 2026-05-22T01:16:22.828105+00:00

WARNING: Description generated by an LLM and may contain inaccuracies. Verify before using!



## REASONING

We need to produce a description of dataset as whole, 1-8 sentences. Then Notable Characteristics bulleted list. Use summary statistics and frequency distribution but not per field. Use data dictionary maybe for context. Avoid listing each field stats individually. Provide overall dataset details: 1 million records unique key; date ranges etc. Mention high cardinality fields, missing values, duplicates? Unique key is unique. Some fields have many nulls (Closed Date, Due Date, Resolution Description). Many categorical fields with many distinct values but only few high frequency categories. There are outliers maybe in coordinates extreme? Provide distribution shapes: e.g., Created Date has 841k distinct dates over ~11 years; heavy clustering at certain months. Mention that the dataset is NYC complaints from various agencies.

Also mention PII/PHI? Contains addresses, lat/lon, etc. So yes location data may be considered sensitive. Also contains agency names and complaint types. The dataset includes unique key but still could link to other data. Provide data quality: many missing values in date fields, some coordinates null, some addresses null; high number of "Other" categories indicating sparse categories.

Also mention duplicates? Unique Key ensures uniqueness, so no duplicates at record level.

Ok produce description.

Let's craft 5 sentences. Then bullet list. Use placeholders for signature.

Proceed.
## TOKEN USAGE

TokenUsage { prompt: 20002, completion: 787, total: 20789, elapsed: 20097 }
---
# Tags
- nyc_311_service_requests  
- noise_complaints  
- building_and_structural_issues  
- street_and_pavement_problems  
- parking_violations  
- water_system_disruptions  
- geospatial_coordinates  
- boroughs_of_new_york_city  
- public_safety_incidents  
- city_government_agencies  

Generated by qsv v20.1.0 describegpt
Command line: target/debug/qsv describegpt --all --two-pass /tmp/NYC_311_SR_2010-2020-sample-1M.csv -o /tmp/nyc311-describegpt.md --fresh --infer-content-type
Prompt file: Default v7.1.0
Model: openai/gpt-oss-20b
LLM API URL: http://localhost:1234/v1
Language: 
Timestamp: 2026-05-22T01:17:01.356744+00:00

WARNING: Tags generated by an LLM and may contain inaccuracies. Verify before using!



## REASONING

We need to produce a list of no more than 10 unique tags in markdown format, lowercase only with underscores. Based on the dataset content: NYC complaints data covering various categories like complaint type, location types, boroughs, etc.

Let's think about most important topics:

- public safety / city services
- noise complaints
- building maintenance
- street/road issues
- parking violations
- water system problems
- NYC (New York City)
- boroughs: Brooklyn, Manhattan, Queens, Bronx, Staten Island
- 311 service requests
- geospatial data

But we need tags in descending importance. We should choose up to 10.

Possible tags:

1. nyc_311_service_requests
2. noise_complaints
3. building_and_structural_issues
4. street_and_pavement_problems
5. parking_violations
6. water_system_disruptions
7. geospatial_coordinates
8. boroughs_of_new_york_city
9. public_safety_incidents
10. city_government_agencies

But maybe "city_government_agencies" is less important than "boroughs". Also we need no more than 10 unique tags.

We might choose:

- nyc_311_service_requests (most)
- noise_complaints
- building_and_structural_issues
- street_and_pavement_problems
- parking_violations
- water_system_disruptions
- geospatial_coordinates
- boroughs_of_new_york_city
- public_safety_incidents
- city_government_agencies

Yes 10.

But ensure lowercase only, underscores.

Check for duplicates: "public_safety_incidents" vs "city_government_agencies". Acceptable.

We should order by importance: 311 service requests highest. Then noise complaints maybe high frequency. Building issues next. Street problems, parking violations, water system, geospatial coordinates (since many lat/lon fields), boroughs of NYC (maybe high cardinality). Public safety incidents? maybe same as city_government_agencies. Let's decide order:

1. nyc_311_service_requests
2. noise_complaints
3. building_and_structural_issues
4. street_and_pavement_problems
5. parking_violations
6. water_system_disruptions
7. geospatial_coordinates
8. boroughs_of_new_york_city
9. public_safety_incidents
10. city_government_agencies

Add attribution placeholder.

Return markdown list:

- nyc_311_service_requests
- noise_complaints
...

Wrap as bullet points? The instruction: "in Markdown format". Could be a list or comma separated. Use bullet list.

After tags add attribution line: "{GENERATED_BY_SIGNATURE}". Possibly on new line after tags.

Let's produce.


## TOKEN USAGE

TokenUsage { prompt: 19977, completion: 667, total: 20644, elapsed: 21028 }
---
