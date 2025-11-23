Generated using the command:
```bash
$ QSV_LLM_BASE_URL=https://api.together.xyz/v1 QSV_LLM_APIKEY=THEKEY qsv describegpt \
     NYC_311_SR_2010-2020-sample-1M.csv --all \
     --output nyc311-describegpt.md
```
---
# Data Dictionary

| Name | Type | Label | Description | Cardinality | Enumeration | Null Count | Examples |
|------|------|-------|-------------|------------|--------------|-----------|----------|
| Unique Key | Integer | Unique Key | A unique identifier for each record in the dataset, generated as a 64‑bit integer. | 1000000 |  | 0 | <ALL_UNIQUE> |
| Created Date | DateTime | Created Date | The date and time when the record was created in the system, recorded to the nearest minute. | 32687965 |  | 0 | 01/24/2013 12:00:00 AM (347), 01/07/2014 12:00:00 AM (315), 01/08/2015 12:00:00 AM (283), 02/16/2015 12:00:00 AM (269), 11/25/2013 12:00:00 AM (259) |
| Closed Date | DateTime | Closed Date | The date and time when the record was closed or resolved, if applicable. | 73049 |  | 28619 | (NULL) (28619), 11/15/2010 12:00:00 AM (384), 11/07/2012 12:00:00 AM (329), 12/09/2010 12:00:00 AM (267), 01/28/2013 12:00:00 AM (259) |
| Agency | String | Agency | The agency that submitted or is responsible for the record. | 28 |  | 0 | NYPD (265116), HPD (258033), DOT (132462), DSNY (81606), DEP (75895) |
| Agency Name | String | Agency Name | The full name of the agency responsible for the record. | 553 |  | 0 | New York City Police Department (265038), Department of Housing Preservation and Development (258019), Department of Transportation (132462), Department of Environmental Protection (75895), Department of Buildings (50649) |
| Complaint Type | String | Complaint Type | The category of the complaint or issue reported. | 287 |  | 0 | Noise - Residential (89439), HEAT/HOT WATER (56639), Illegal Parking (45032), Blocked Driveway (42356), Street Condition (41469) |
| Descriptor | String | Descriptor | A more detailed description of the complaint, often specifying the specific problem or symptom. | 1392 |  | 0 | Loud Music/Party (93646), ENTIRE BUILDING (36885), HEAT (35088), No Access (31631), Street Light Out (29659) |
| Location Type | String | Location Type | The type of location where the complaint was reported, such as residential building or street. | 162 |  | 0 | RESIDENTIAL BUILDING (255562), (NULL) (239131), Street/Sidewalk (145653), Residential Building/House (92765), Street (92190) |
| Incident Zip | String | Incident Zip | The ZIP code of the incident location, used for geographic grouping. | 535 |  | 54978 | (NULL) (54978), 11226 (17114), 10467 (14495), 11207 (12872), 11385 (12469) |
| Incident Address | String | Incident Address | The street address of the incident location. | 341996 |  | 174700 | (NULL) (174700), 655 EAST  230 STREET (1538), 78-15 PARSONS BOULEVARD (694), 672 EAST  231 STREET (663), 89-21 ELMHURST AVENUE (660) |
| Street Name | String | Street Name | The name of the street on which the incident occurred. | 14837 |  | 174720 | (NULL) (174720), BROADWAY (9702), GRAND CONCOURSE (5851), OCEAN AVENUE (3946), 5 AVENUE (3694) |
| Cross Street 1 | String | Cross Street 1 | The first cross street intersecting at the incident location, if applicable. | 16238 |  | 320401 | (NULL) (320401), BEND (12562), BROADWAY (8548), 3 AVENUE (6154), 5 AVENUE (6138) |
| Cross Street 2 | String | Cross Street 2 | The second cross street intersecting at the incident location, if applicable. | 16486 |  | 323644 | (NULL) (323644), BEND (12390), BROADWAY (8833), DEAD END (5626), 8 AVENUE (4309) |
| Intersection Street 1 | String | Intersection Street 1 | The first street in the intersection where the incident was reported. | 11237 |  | 767422 | (NULL) (767422), BROADWAY (3761), CARPENTER AVENUE (2918), BEND (2009), 3 AVENUE (1957) |
| Intersection Street 2 | String | Intersection Street 2 | The second street in the intersection where the incident was reported. | 11674 |  | 767709 | (NULL) (767709), BROADWAY (3462), BEND (1942), 2 AVENUE (1690), 3 AVENUE (1630) |
| Address Type | String | Address Type | The classification of the address type used for the incident location. | 6 |  | 125802 | ADDRESS (710380), INTERSECTION (133361), (NULL) (125802), BLOCKFACE (22620), LATLONG (7421) |
| City | String | City | The city in which the incident location falls. | 382 |  | 61963 | BROOKLYN (296254), NEW YORK (189069), BRONX (181168), (NULL) (61963), STATEN ISLAND (49045) |
| Landmark | String | Landmark | A notable landmark near the incident location, if any. | 5915 |  | 912779 | (NULL) (912779), EAST 230 STREET (1545), EAST 231 STREET (1291), BROADWAY (1148), 5 AVENUE (584) |
| Facility Type | String | Facility Type | The type of facility involved in the incident, if applicable. | 6 |  | 145478 | N/A (628279), Precinct (193259), (NULL) (145478), DSNY Garage (32310), School (617) |
| Status | String | Status | The current status of the record, indicating whether it is open, closed, pending, etc. | 10 | Closed, Pending, Open, In Progress, Assigned, Started, Email Sent, Closed - Testing, Unassigned, Unspecified | 0 | Closed (952522), Pending (20119), Open (12340), In Progress (7841), Assigned (6651) |
| Due Date | DateTime | Due Date | The scheduled due date for the record to be resolved or acted upon. | 345077 |  | 647794 | (NULL) (647794), 04/08/2015 10:00:58 AM (214), 05/02/2014 03:32:17 PM (183), 03/30/2018 10:10:39 AM (172), 06/27/2015 10:29:08 AM (157) |
| Resolution Description | String | Resolution Description | A narrative description of the actions taken to resolve the complaint. | 1216 |  | 20480 | The Police Department responded to the complaint and with the information available observed no evidence of the violation at that time. (91408), The Department of Housing Preservation and Development inspected the following conditions. No violations were issued. The complaint has been closed. (72962), The Police Department responded to the complaint and took action to fix the condition. (63868), Service Request status for this request is available on the Department of Transportation? website. Please click the ?Learn More? link below. (52155), The Department of Housing Preservation and Development inspected the following conditions. Violations were issued. Information about specific violations is available at www.nyc.gov/hpd. (43722) |
| Resolution Action Updated Date | DateTime | Resolution Action Updated Date | The most recent date and time when the resolution action was updated. | 690314 |  | 15072 | (NULL) (15072), 11/15/2010 12:00:00 AM (385), 11/07/2012 12:00:00 AM (336), 12/09/2010 12:00:00 AM (273), 02/28/2011 12:00:00 AM (268) |
| Community Board | String | Community Board | The community board number associated with the incident location. | 77 |  | 49878 | 0 Unspecified (49878), 12 MANHATTAN (29845), 12 QUEENS (23570), 01 BROOKLYN (21714), 03 BROOKLYN (21292) |
| BBL | String | BBL | Borough Block and Lot identifier for the location, used in New York City property records. | 268373 |  | 243046 | (NULL) (243046), 2048330028 (1566), 4068290001 (696), 4015110001 (664), 2048330080 (663) |
| Borough | String | Borough | The borough in which the incident location lies. | 6 | BROOKLYN, QUEENS, MANHATTAN, BRONX, Unspecified | 0 | BROOKLYN (296081), QUEENS (228818), MANHATTAN (195488), BRONX (180142), Unspecified (49878) |
| X Coordinate (State Plane) | Integer | X Coordinate (State Plane) | The X coordinate of the incident location in State Plane coordinates. | 102546 |  | 85327 | (NULL) (85327), 1022911 (1568), 1037000 (701), 1023174 (675), 1018372 (669) |
| Y Coordinate (State Plane) | Integer | Y Coordinate (State Plane) | The Y coordinate of the incident location in State Plane coordinates. | 116092 |  | 85327 | (NULL) (85327), 264242 (1566), 202363 (706), 211606 (665), 264429 (663) |
| Open Data Channel Type | String | Open Data Channel Type | The channel through which the data was accessed or submitted. | 5 | PHONE, UNKNOWN, ONLINE, MOBILE, OTHER | 0 | PHONE (497606), UNKNOWN (230402), ONLINE (177334), MOBILE (79892), OTHER (14766) |
| Park Facility Name | String | Park Facility Name | The name of the park facility associated with the incident location. | 1889 |  | 993141 | Unspecified (993141), Central Park (261), Riverside Park (136), Prospect Park (129), N/A (84) |
| Park Borough | String | Park Borough | The borough where the park facility is located. | 6 | BROOKLYN, QUEENS, MANHATTAN, BRONX, Unspecified | 0 | BROOKLYN (296081), QUEENS (228818), MANHATTAN (195488), BRONX (180142), Unspecified (49878) |
| Vehicle Type | String | Vehicle Type | The type of vehicle involved in the incident, if applicable. | 999652 |  | 999652 | (NULL) (999652), Car Service (317), Ambulette / Paratransit (19), Commuter Van (11), Green Taxi (1) |
| Taxi Company Borough | String | Taxi Company Borough | The borough in which the taxi company operates. | 11 |  | 999156 | (NULL) (999156), Other (1) (1) |
| Taxi Pick Up Location | String | Taxi Pick Up Location | The location where a taxi was picked up. | 1903 |  | 992129 | (NULL) (992129), Other (4091), JFK Airport (562), Intersection (486), La Guardia Airport (387) |
| Bridge Highway Name | String | Bridge Highway Name | The name of the bridge or highway associated with the incident location. | 68 |  | 997711 | (NULL) (997711), Belt Pkwy (276), BQE/Gowanus Expwy (254), Grand Central Pkwy (186), Long Island Expwy (173) |
| Bridge Highway Direction | String | Bridge Highway Direction | The direction of the bridge or highway at the incident location. | 50 |  | 997691 | (NULL) (997691), East/Long Island Bound (210), North/Bronx Bound (208), East/Queens Bound (197), West/Staten Island Bound (175) |
| Road Ramp | String | Road Ramp | The type of ramp or road segment involved in the incident. | 4 | (NULL), ROADWAY, RAMP, N/A | 997693 | (NULL) (997693), ROADWAY (1731), RAMP (555), N/A (21) |
| Bridge Highway Segment | String | Bridge Highway Segment | The specific segment of a bridge or highway where the incident occurred. | 937 |  | 997556 | (NULL) (997556), RAMP (92), Roadway (54), Clove Rd/Richmond Rd (Exit 13) - Lily Pond Ave/Bay St (Exit 15 S-N) (23), N/A (23) |
| Latitude | Float | Latitude | The latitude coordinate of the incident location in decimal degrees. | 353684 |  | 254695 | (NULL) (254695), 40.89187241649303 (1538), 40.1123853 (1153), 40.89238451539139 (663), 40.7474203 (552) |
| Longitude | Float | Longitude | The longitude coordinate of the incident location in decimal degrees. | 353986 |  | 254695 | (NULL) (254695), -73.86016845296459 (1538), -77.5195844 (1153), -73.8592161325675 (663), -73.876853 (552) |
| Location | String | Location | The combined latitude and longitude coordinate of the incident location. | 375762 |  | 254695 | (NULL) (254695), "(40.89187241649303, -73.86016845296459)" (1538), "(40.1123853, -77.5195844)" (1153), "(40.89238451539139, -73.8592161325675)" (663), "(40.7474203, -73.876853)" (552) |

Generated by qsv's describegpt command with prompt file v4.0.0 using openai/gpt-oss-20b on 2025-11-23T18:42:04.657580+00:00


## REASONING

We need to produce Data Dictionary table with columns: Name, Type, Label, Description, Cardinality, Enumeration, Null Count, Examples.

Need to derive label and description from field name maybe descriptive. Use summary stats for type, cardinality, nullcount. Enumeration: only if cardinality <=10 and no rank 0 in frequency distribution (i.e., no "Other" category). For many fields, cardinality high >10 so leave empty. If cardinality <=10 but there is an Other rank? Actually spec says: If cardinality>10 leave empty. Else if none of the corresponding unique values have rank=0, enumerate unique values from Frequency Distribution.

So for fields with cardinality <=10 and no "Other" entry, we need to list enumeration (unique values). Let's check some fields:

- Unique Key has cardinality 1000000 >10 -> enumeration empty.
- Created Date: cardinality 32687965? Wait summary stats shows cardinality=32687965. That's huge >10 => enumeration empty.

We might not have any field with cardinality <=10 except maybe Status (cardinality 10). Let's check Status: cardinality=10, and in frequency distribution there is no rank 0? The freq list for Status includes values Closed, Pending, Open, In Progress, Assigned, Started, Email Sent, Closed - Testing, Unassigned, Unspecified. No "Other". So enumeration should list those 10 values.

Similarly maybe Borough cardinality=6 <=10 and no Other? Frequency distribution shows 5 unique (BRONX, QUEENS, MANHATTAN, BRONX again? Actually duplicates). Let's check: Borough freq list includes BROOKLYN, QUEENS, MANHATTAN, BRONX, Unspecified. That's 5 distinct; cardinality 6 >10? Wait summary stats cardinality for Borough is 6? Yes cardinality=6 <=10 and no "Other" category. So enumeration should list the five values.

Similarly Open Data Channel Type cardinality 5 <=10 and freq list has 5 unique (PHONE, UNKNOWN, ONLINE, MOBILE, OTHER). No Other. So enumeration list those.

Park Facility Name cardinality 1889 >10 => empty.

Vehicle Type cardinality 999652 >10 -> empty? Actually Vehicle Type summary says cardinality=999652; so no enumeration.

Taxi Company Borough cardinality 11 >10? Wait summary shows cardinality 11 (since uniqueness ratio 0.0001). But freq list includes NULL and many values including Other (1) as rank 0? Let's check: Taxi Company Borough frequency list has values: (NULL), BROOKLYN, QUEENS, MANHATTAN, BRONX, STATEN ISLAND, Manhattan, Brooklyn, Queens, Bronx, Other (1). The "Other" is rank 0. Since there's a rank 0 entry, enumeration should be empty even though cardinality <=10? The rule says: If none of the unique values have rank=0, enumerate. Here there is rank 0 -> so leave enumeration empty.

Similarly Taxi Pick Up Location cardinality huge >10; ignore.

Bridge Highway Name cardinality 68 >10.

Bridge Highway Direction cardinality 50 >10.

Road Ramp cardinality 4? Wait summary says cardinality 4? Actually Road Ramp summary shows cardinality=4. freq list has values: (NULL), ROADWAY, RAMP, N/A. No Other. So enumeration should list those four.

Bridge Highway Segment cardinality 937? >10 no enumeration.

Latitude cardinality 353684? huge >10.

Longitude cardinality 353986? >10.

Location cardinality 375762? >10.

Community Board cardinality 67? >10? Actually summary shows cardinality=77? Wait summary for Community Board: cardinality 77. freq list has 11 values plus Other (67). So enumeration empty.

BBL cardinality 268373? huge.

Borough cardinality 6, enumeration as earlier.

X Coordinate (State Plane) cardinality 102546? >10.

Y coordinate similar.

Open Data Channel Type already enumerated.

Park Borough cardinality 6? Actually summary shows cardinality=6? Wait Park Borough summary: cardinality 6. freq list has 5 values: BROOKLYN, QUEENS, MANHATTAN, BRONX, Unspecified. No Other. So enumeration should list those five.

Vehicle Type already big.

Ok. Also there is "Status" with cardinality=10; enumeration will include all 10 status strings.

Also maybe "Resolution Action Updated Date" cardinality? It shows 690304? >10.

Now need to create table. For each field: Name, Type, Label (human-friendly). We can use PascalCase or capitalize words: e.g., Unique Key -> "Unique Key", Created Date -> "Created Date". Maybe we can add spaces as is.

Description: provide full description for field; we can write general explanation of what the field contains. Use multiple sentences.

Examples: top 5 values descending count, from frequency distribution. For fields with less than 5 unique values, list all? The spec says at least 5 top values based on Frequency Distribution, in count descending order. If fewer than 5, we can list all available? Should still provide at least 5? But some have only 4 or 5. We can include the ones present.

Also set to "<ALL_UNIQUE>" if field has frequency distribution percentage=100 i.e., <ALL_UNIQUE> indicates all unique. That's for Unique Key and maybe other fields with all unique? Let's see: Unique Key freq shows <ALL_UNIQUE>. So examples should be "<ALL_UNIQUE>".

Now produce table in markdown.

Need footnote placeholder {GENERATED_BY_SIGNATURE} at bottom.

Let's craft data dictionary entries for each field present in summary stats. That's many fields; need to include all.

We can generate rows but due to length, maybe summarizing? The instruction: "Generate a Data Dictionary as aforementioned (in Markdown format) for ALL fields in the Dataset". So we need to produce row per field.

Ok let's list all fields from summary stats:

Unique Key
Created Date
Closed Date
Agency
Agency Name
Complaint Type
Descriptor
Location Type
Incident Zip
Incident Address
Street Name
Cross Street 1
Cross Street 2
Intersection Street 1
Intersection Street 2
Address Type
City
Landmark
Facility Type
Status
Due Date
Resolution Description
Resolution Action Updated Date
Community Board
BBL
Borough
X Coordinate (State Plane)
Y Coordinate (State Plane)
Open Data Channel Type
Park Facility Name
Park Borough
Vehicle Type
Taxi Company Borough
Taxi Pick Up Location
Bridge Highway Name
Bridge Highway Direction
Road Ramp
Bridge Highway Segment
Latitude
Longitude
Location

Total 42 fields.

We'll need to produce 42 rows. Provide enumeration for Status, Borough, Open Data Channel Type, Road Ramp, Park Borough (maybe also Park Facility? cardinality high). Also maybe "Community Board" cardinality 77 >10 no.

Also "Vehicle Type" cardinality huge.

Ok.

Now craft each row with appropriate examples.

We'll need to extract top 5 values from frequency distribution for each field. For fields with <ALL_UNIQUE> or less than 5 unique? Use those.

Let's parse some:

Unique Key: Example "<ALL_UNIQUE>".

Created Date: top 5 counts:
01/24/2013 12:00:00 AM (347)
01/07/2014 12:00:00 AM (315)
01/08/2015 12:00:00 AM (283)
02/16/2015 12:00:00 AM (269)
11/25/2013 12:00:00 AM (259)

Closed Date:
(Null) 28619
11/15/2010 12:00:00 AM 384
11/07/2012 12:00:00 AM 329
12/09/2010 12:00:00 AM 267
01/28/2013 12:00:00 AM 259

Agency:
NYPD 265116
HPD 258033
DOT 132462
DSNY 81606
DEP 75895

Agency Name:
New York City Police Department 265038
Department of Housing Preservation and Development 258019
Department of Transportation 132462
Department of Environmental Protection 75895
Department of Buildings 50649

Complaint Type:
Noise - Residential 89439
HEAT/HOT WATER 56639
Illegal Parking 45032
Blocked Driveway 42356
Street Condition 41469

Descriptor:
Loud Music/Party 93646
ENTIRE BUILDING 36885
HEAT 35088
No Access 31631
Street Light Out 29659

Location Type:
RESIDENTIAL BUILDING 255562
(Null) 239131
Street/Sidewalk 145653
Residential Building/House 92765
Street 92190

Incident Zip: (Null) 54978; 11226 17114; 10467 14495; 11207 12872; 11385 12469

Incident Address: (Null) 174700; 655 EAST 230 STREET 1538; 78-15 PARSONS BOULEVARD 694; 672 EAST 231 STREET 663; 89-21 ELMHURST AVENUE 660

Street Name: (NULL) 174720; BROADWAY 9702; GRAND CONCOURSE 5851; OCEAN AVENUE 3946; 5 AVENUE 3694

Cross Street 1: (NULL) 320401; BEND 12562; BROADWAY 8548; 3 AVENUE 6154; 5 AVENUE 6138

Cross Street 2: (NULL) 323644; BEND 12390; BROADWAY 8833; DEAD END 5626; 8 AVENUE 4309

Intersection Street 1: (NULL) 767422; BROADWAY 3761; CARPENTER AVENUE 2918; BEND 2009; 3 AVENUE 1957

Intersection Street 2: (NULL) 767709; BROADWAY 3462; BEND 1942; 2 AVENUE 1690; 3 AVENUE 1630

Address Type: ADDRESS 710380; INTERSECTION 133361; (NULL)125802; BLOCKFACE 22620; LATLONG 7421

City: BROOKLYN 296254; NEW YORK 189069; BRONX 181168; (NULL)61963; STATEN ISLAND 49045

Landmark: (NULL)912779; EAST 230 STREET 1545; EAST 231 STREET 1291; BROADWAY 1148; 5 AVENUE 584

Facility Type: N/A 628279; Precinct 193259; (NULL)145478; DSNY Garage 32310; School 617

Status enumeration already known.

Due Date: (NULL)647794; 04/08/2015 10:00:58 AM 214; 05/02/2014 03:32:17 PM 183; 03/30/2018 10:10:39 AM 172; 06/27/2015 10:29:08 AM 157

Resolution Description top 5:
The Police Department responded to the complaint and with the information available observed no evidence of the violation at that time. 91408
The Department of Housing Preservation and Development inspected the following conditions. No violations were issued. The complaint has been closed. 72962
The Police Department responded to the complaint and took action to fix the condition. 63868
Service Request status for this request is available on the Department of Transportation? website. Please click the ?Learn More? link below. 52155
The Department of Housing Preservation and Development inspected the following conditions. Violations were issued. Information about specific violations is available at www.nyc.gov/hpd. 43722

Resolution Action Updated Date: (NULL)15072; 11/15/2010 12:00:00 AM 385; 11/07/2012 12:00:00 AM 336; 12/09/2010 12:00:00 AM 273; 02/28/2011 12:00:00 AM 268

Community Board top 5:
0 Unspecified 49878
12 MANHATTAN 29845
12 QUEENS 23570
01 BROOKLYN 21714
03 BROOKLYN 21292

BBL top 5:
(NULL)243046; 2048330028 1566; 4068290001 696; 4015110001 664; 2048330080 663

Borough enumeration: BROOKLYN, QUEENS, MANHATTAN, BRONX, Unspecified

X Coordinate (State Plane) top 5:
(NULL)85327; 1022911 1568; 1037000 701; 1023174 675; 1018372 669

Y Coordinate (State Plane) top 5: (NULL)85327; 264242 1566; 202363 706; 211606 665; 264429 663

Open Data Channel Type enumeration: PHONE, UNKNOWN, ONLINE, MOBILE, OTHER

Park Facility Name examples: Unspecified 993141; Central Park 261; Riverside Park 136; Prospect Park 129; N/A 84

Park Borough enumeration: BROOKLYN, QUEENS, MANHATTAN, BRONX, Unspecified

Vehicle Type top 5: (NULL)999652; Car Service 317; Ambulette / Paratransit 19; Commuter Van 11; Green Taxi 1

Taxi Company Borough enumeration? cardinality >10? Actually cardinality 11 but has rank 0 so no enumeration.

Taxi Pick Up Location top 5: (NULL)992129; Other 4091; JFK Airport 562; Intersection 486; La Guardia Airport 387

Bridge Highway Name top 5: (NULL)997711; Belt Pkwy 276; BQE/Gowanus Expwy 254; Grand Central Pkwy 186; Long Island Expwy 173

Bridge Highway Direction top 5: (NULL)997691; East/Long Island Bound 210; North/Bronx Bound 208; East/Queens Bound 197; West/Staten Island Bound 175

Road Ramp enumeration: (NULL)997693; ROADWAY 1731; RAMP 555; N/A 21

Bridge Highway Segment top 5: (NULL)997556; RAMP 92; Roadway 54; Clove Rd/Richmond Rd... 23; N/A 23

Latitude top 5: (NULL)254695; 40.89187241649303 1538; 40.1123853 1153; 40.89238451539139 663; 40.7474203 552

Longitude top 5: (NULL)254695; -73.86016845296459 1538; -77.5195844 1153; -73.8592161325675 663; -73.876853 552

Location top 5: (NULL)254695; "(40.89187241649303, -73.86016845296459)" 1538; "(40.1123853, -77.5195844)" 1153; "(40.89238451539139, -73.8592161325675)" 663; "(40.7474203, -73.876853)" 552

Now we need to create table with columns:

Name | Type | Label | Description | Cardinality | Enumeration | Null Count | Examples

We should use the field names as Name.

Label: maybe same as Name but human-friendly e.g., "Unique Key" -> "Unique Key". Could just copy name. Or transform underscores etc. We'll keep as is.

Description: Provide a brief description of what the field represents. Use multiple sentences if needed.

Examples: show top 5 values with count in parentheses.

Enumeration: For those fields enumerated, list each value separated by comma? The spec says "enumerate unique values for this field from the Frequency Distribution." Could be as comma-separated string. We'll list like `NYPD, HPD, DOT...`. For Status enumeration also list all 10 statuses separated by commas.

Null Count: use nullcount from summary stats.

Cardinality: from summary.

Type: from summary type.

We must use exact values from summary and freq; no approximations.

Now produce table. We'll need to ensure correct formatting for markdown: header row with | separators, then rows.

Also footnote placeholder {GENERATED_BY_SIGNATURE} at bottom of dictionary.

Let's create the table. We have 42 rows. Let's start building each row.

I'll craft using manual text.

Row1: Unique Key

Name: Unique Key
Type: Integer
Label: Unique Key
Description: A unique identifier for each record in the dataset, generated as a 64‑bit integer.
Cardinality: 1000000
Enumeration: (empty)
Null Count: 0
Examples: <ALL_UNIQUE>

Row2: Created Date

Name: Created Date
Type: DateTime
Label: Created Date
Description: The date and time when the record was created in the system, recorded to the nearest minute.
Cardinality: 32687965
Enumeration:
Null Count: 0
Examples: 01/24/2013 12:00:00 AM (347), 01/07/2014 12:00:00 AM (315), 01/08/2015 12:00:00 AM (283), 02/16/2015 12:00:00 AM (269), 11/25/2013 12:00:00 AM (259)

Row3: Closed Date

Name: Closed Date
Type: DateTime
Label: Closed Date
Description: The date and time when the record was closed or resolved, if applicable.
Cardinality: 73049
Enumeration:
Null Count: 28619
Examples: (NULL) (28619), 11/15/2010 12:00:00 AM (384), 11/07/2012 12:00:00 AM (329), 12/09/2010 12:00:00 AM (267), 01/28/2013 12:00:00 AM (259)

Row4: Agency

Name: Agency
Type: String
Label: Agency
Description: The agency that submitted or is responsible for the record.
Cardinality: 28
Enumeration:
Null Count: 0
Examples: NYPD (265116), HPD (258033), DOT (132462), DSNY (81606), DEP (75895)

Row5: Agency Name

Name: Agency Name
Type: String
Label: Agency Name
Description: The full name of the agency responsible for the record.
Cardinality: 553
Enumeration:
Null Count: 0
Examples: New York City Police Department (265038), Department of Housing Preservation and Development (258019), Department of Transportation (132462), Department of Environmental Protection (75895), Department of Buildings (50649)

Row6: Complaint Type

Name: Complaint Type
Type: String
Label: Complaint Type
Description: The category of the complaint or issue reported.
Cardinality: 287
Enumeration:
Null Count: 0
Examples: Noise - Residential (89439), HEAT/HOT WATER (56639), Illegal Parking (45032), Blocked Driveway (42356), Street Condition (41469)

Row7: Descriptor

Name: Descriptor
Type: String
Label: Descriptor
Description: A more detailed description of the complaint, often specifying the specific problem or symptom.
Cardinality: 1392
Enumeration:
Null Count: 0
Examples: Loud Music/Party (93646), ENTIRE BUILDING (36885), HEAT (35088), No Access (31631), Street Light Out (29659)

Row8: Location Type

Name: Location Type
Type: String
Label: Location Type
Description: The type of location where the complaint was reported, such as residential building or street.
Cardinality: 162
Enumeration:
Null Count: 0
Examples: RESIDENTIAL BUILDING (255562), (NULL) (239131), Street/Sidewalk (145653), Residential Building/House (92765), Street (92190)

Row9: Incident Zip

Name: Incident Zip
Type: String
Label: Incident Zip
Description: The ZIP code of the incident location, used for geographic grouping.
Cardinality: 535
Enumeration:
Null Count: 54978
Examples: (NULL) (54978), 11226 (17114), 10467 (14495), 11207 (12872), 11385 (12469)

Row10: Incident Address

Name: Incident Address
Type: String
Label: Incident Address
Description: The street address of the incident location.
Cardinality: 341996
Enumeration:
Null Count: 174700
Examples: (NULL) (174700), 655 EAST  230 STREET (1538), 78-15 PARSONS BOULEVARD (694), 672 EAST  231 STREET (663), 89-21 ELMHURST AVENUE (660)

Row11: Street Name

Name: Street Name
Type: String
Label: Street Name
Description: The name of the street on which the incident occurred.
Cardinality: 14837
Enumeration:
Null Count: 174720
Examples: (NULL) (174720), BROADWAY (9702), GRAND CONCOURSE (5851), OCEAN AVENUE (3946), 5 AVENUE (3694)

Row12: Cross Street 1

Name: Cross Street 1
Type: String
Label: Cross Street 1
Description: The first cross street intersecting at the incident location, if applicable.
Cardinality: 16238
Enumeration:
Null Count: 320401
Examples: (NULL) (320401), BEND (12562), BROADWAY (8548), 3 AVENUE (6154), 5 AVENUE (6138)

Row13: Cross Street 2

Name: Cross Street 2
Type: String
Label: Cross Street 2
Description: The second cross street intersecting at the incident location, if applicable.
Cardinality: 16486
Enumeration:
Null Count: 323644
Examples: (NULL) (323644), BEND (12390), BROADWAY (8833), DEAD END (5626), 8 AVENUE (4309)

Row14: Intersection Street 1

Name: Intersection Street 1
Type: String
Label: Intersection Street 1
Description: The first street in the intersection where the incident was reported.
Cardinality: 11237
Enumeration:
Null Count: 767422
Examples: (NULL) (767422), BROADWAY (3761), CARPENTER AVENUE (2918), BEND (2009), 3 AVENUE (1957)

Row15: Intersection Street 2

Name: Intersection Street 2
Type: String
Label: Intersection Street 2
Description: The second street in the intersection where the incident was reported.
Cardinality: 11674
Enumeration:
Null Count: 767709
Examples: (NULL) (767709), BROADWAY (3462), BEND (1942), 2 AVENUE (1690), 3 AVENUE (1630)

Row16: Address Type

Name: Address Type
Type: String
Label: Address Type
Description: The classification of the address type used for the incident location.
Cardinality: 6
Enumeration:
Null Count: 125802
Examples: ADDRESS (710380), INTERSECTION (133361), (NULL) (125802), BLOCKFACE (22620), LATLONG (7421)

Row17: City

Name: City
Type: String
Label: City
Description: The city in which the incident location falls.
Cardinality: 382
Enumeration:
Null Count: 61963
Examples: BROOKLYN (296254), NEW YORK (189069), BRONX (181168), (NULL) (61963), STATEN ISLAND (49045)

Row18: Landmark

Name: Landmark
Type: String
Label: Landmark
Description: A notable landmark near the incident location, if any.
Cardinality: 5915
Enumeration:
Null Count: 912779
Examples: (NULL) (912779), EAST  230 STREET (1545), EAST  231 STREET (1291), BROADWAY (1148), 5 AVENUE (584)

Row19: Facility Type

Name: Facility Type
Type: String
Label: Facility Type
Description: The type of facility involved in the incident, if applicable.
Cardinality: 6
Enumeration:
Null Count: 145478
Examples: N/A (628279), Precinct (193259), (NULL) (145478), DSNY Garage (32310), School (617)

Row20: Status

Name: Status
Type: String
Label: Status
Description: The current status of the record, indicating whether it is open, closed, pending, etc.
Cardinality: 10
Enumeration: Closed, Pending, Open, In Progress, Assigned, Started, Email Sent, Closed - Testing, Unassigned, Unspecified
Null Count: 0
Examples: Closed (952522), Pending (20119), Open (12340), In Progress (7841), Assigned (6651)

Row21: Due Date

Name: Due Date
Type: DateTime
Label: Due Date
Description: The scheduled due date for the record to be resolved or acted upon.
Cardinality: 345077
Enumeration:
Null Count: 647794
Examples: (NULL) (647794), 04/08/2015 10:00:58 AM (214), 05/02/2014 03:32:17 PM (183), 03/30/2018 10:10:39 AM (172), 06/27/2015 10:29:08 AM (157)

Row22: Resolution Description

Name: Resolution Description
Type: String
Label: Resolution Description
Description: A narrative description of the actions taken to resolve the complaint.
Cardinality: 1216
Enumeration:
Null Count: 20480
Examples: The Police Department responded to the complaint and with the information available observed no evidence of the violation at that time. (91408), The Department of Housing Preservation and Development inspected the following conditions. No violations were issued. The complaint has been closed. (72962), The Police Department responded to the complaint and took action to fix the condition. (63868), Service Request status for this request is available on the Department of Transportation? website. Please click the ?Learn More? link below. (52155), The Department of Housing Preservation and Development inspected the following conditions. Violations were issued. Information about specific violations is available at www.nyc.gov/hpd. (43722)

Row23: Resolution Action Updated Date

Name: Resolution Action Updated Date
Type: DateTime
Label: Resolution Action Updated Date
Description: The most recent date and time when the resolution action was updated.
Cardinality: 690314
Enumeration:
Null Count: 15072
Examples: (NULL) (15072), 11/15/2010 12:00:00 AM (385), 11/07/2012 12:00:00 AM (336), 12/09/2010 12:00:00 AM (273), 02/28/2011 12:00:00 AM (268)

Row24: Community Board

Name: Community Board
Type: String
Label: Community Board
Description: The community board number associated with the incident location.
Cardinality: 77
Enumeration:
Null Count: 49878
Examples: 0 Unspecified (49878), 12 MANHATTAN (29845), 12 QUEENS (23570), 01 BROOKLYN (21714), 03 BROOKLYN (21292)

Row25: BBL

Name: BBL
Type: String
Label: BBL
Description: Borough Block and Lot identifier for the location, used in New York City property records.
Cardinality: 268373
Enumeration:
Null Count: 243046
Examples: (NULL) (243046), 2048330028 (1566), 4068290001 (696), 4015110001 (664), 2048330080 (663)

Row26: Borough

Name: Borough
Type: String
Label: Borough
Description: The borough in which the incident location lies.
Cardinality: 6
Enumeration: BROOKLYN, QUEENS, MANHATTAN, BRONX, Unspecified
Null Count: 0
Examples: BROOKLYN (296081), QUEENS (228818), MANHATTAN (195488), BRONX (180142), Unspecified (49878)

Row27: X Coordinate (State Plane)

Name: X Coordinate (State Plane)
Type: Integer
Label: X Coordinate (State Plane)
Description: The X coordinate of the incident location in State Plane coordinates.
Cardinality: 102546
Enumeration:
Null Count: 85327
Examples: (NULL) (85327), 1022911 (1568), 1037000 (701), 1023174 (675), 1018372 (669)

Row28: Y Coordinate (State Plane)

Name: Y Coordinate (State Plane)
Type: Integer
Label: Y Coordinate (State Plane)
Description: The Y coordinate of the incident location in State Plane coordinates.
Cardinality: 116092
Enumeration:
Null Count: 85327
Examples: (NULL) (85327), 264242 (1566), 202363 (706), 211606 (665), 264429 (663)

Row29: Open Data Channel Type

Name: Open Data Channel Type
Type: String
Label: Open Data Channel Type
Description: The channel through which the data was accessed or submitted.
Cardinality: 5
Enumeration: PHONE, UNKNOWN, ONLINE, MOBILE, OTHER
Null Count: 0
Examples: PHONE (497606), UNKNOWN (230402), ONLINE (177334), MOBILE (79892), OTHER (14766)

Row30: Park Facility Name

Name: Park Facility Name
Type: String
Label: Park Facility Name
Description: The name of the park facility associated with the incident location.
Cardinality: 1889
Enumeration:
Null Count: 993141
Examples: Unspecified (993141), Central Park (261), Riverside Park (136), Prospect Park (129), N/A (84)

Row31: Park Borough

Name: Park Borough
Type: String
Label: Park Borough
Description: The borough where the park facility is located.
Cardinality: 6
Enumeration: BROOKLYN, QUEENS, MANHATTAN, BRONX, Unspecified
Null Count: 0
Examples: BROOKLYN (296081), QUEENS (228818), MANHATTAN (195488), BRONX (180142), Unspecified (49878)

Row32: Vehicle Type

Name: Vehicle Type
Type: String
Label: Vehicle Type
Description: The type of vehicle involved in the incident, if applicable.
Cardinality: 999652
Enumeration:
Null Count: 999652
Examples: (NULL) (999652), Car Service (317), Ambulette / Paratransit (19), Commuter Van (11), Green Taxi (1)

Row33: Taxi Company Borough

Name: Taxi Company Borough
Type: String
Label: Taxi Company Borough
Description: The borough in which the taxi company operates.
Cardinality: 11
Enumeration:
Null Count: 999156
Examples: (NULL) (999156), Other (1) (1)

Row34: Taxi Pick Up Location

Name: Taxi Pick Up Location
Type: String
Label: Taxi Pick Up Location
Description: The location where a taxi was picked up.
Cardinality: 1903
Enumeration:
Null Count: 992129
Examples: (NULL) (992129), Other (4091), JFK Airport (562), Intersection (486), La Guardia Airport (387)

Row35: Bridge Highway Name

Name: Bridge Highway Name
Type: String
Label: Bridge Highway Name
Description: The name of the bridge or highway associated with the incident location.
Cardinality: 68
Enumeration:
Null Count: 997711
Examples: (NULL) (997711), Belt Pkwy (276), BQE/Gowanus Expwy (254), Grand Central Pkwy (186), Long Island Expwy (173)

Row36: Bridge Highway Direction

Name: Bridge Highway Direction
Type: String
Label: Bridge Highway Direction
Description: The direction of the bridge or highway at the incident location.
Cardinality: 50
Enumeration:
Null Count: 997691
Examples: (NULL) (997691), East/Long Island Bound (210), North/Bronx Bound (208), East/Queens Bound (197), West/Staten Island Bound (175)

Row37: Road Ramp

Name: Road Ramp
Type: String
Label: Road Ramp
Description: The type of ramp or road segment involved in the incident.
Cardinality: 4
Enumeration: (NULL), ROADWAY, RAMP, N/A
Null Count: 997693
Examples: (NULL) (997693), ROADWAY (1731), RAMP (555), N/A (21)

Row38: Bridge Highway Segment

Name: Bridge Highway Segment
Type: String
Label: Bridge Highway Segment
Description: The specific segment of a bridge or highway where the incident occurred.
Cardinality: 937
Enumeration:
Null Count: 997556
Examples: (NULL) (997556), RAMP (92), Roadway (54), Clove Rd/Richmond Rd (Exit 13) - Lily Pond Ave/Bay St (Exit 15 S-N) (23), N/A (23)

Row39: Latitude

Name: Latitude
Type: Float
Label: Latitude
Description: The latitude coordinate of the incident location in decimal degrees.
Cardinality: 353684
Enumeration:
Null Count: 254695
Examples: (NULL) (254695), 40.89187241649303 (1538), 40.1123853 (1153), 40.89238451539139 (663), 40.7474203 (552)

Row40: Longitude

Name: Longitude
Type: Float
Label: Longitude
Description: The longitude coordinate of the incident location in decimal degrees.
Cardinality: 353986
Enumeration:
Null Count: 254695
Examples: (NULL) (254695), -73.86016845296459 (1538), -77.5195844 (1153), -73.8592161325675 (663), -73.876853 (552)

Row41: Location

Name: Location
Type: String
Label: Location
Description: The combined latitude and longitude coordinate of the incident location.
Cardinality: 375762
Enumeration:
Null Count: 254695
Examples: (NULL) (254695), "(40.89187241649303, -73.86016845296459)" (1538), "(40.1123853, -77.5195844)" (1153), "(40.89238451539139, -73.8592161325675)" (663), "(40.7474203, -73.876853)" (552)

Now produce markdown table with headers.

Need to ensure each cell separated by | and no extra pipes inside values that contain commas? That's fine. But some values have commas like "BROADWAY, 2 AVENUE" etc but they are separate rows; not in a single cell. In enumeration we list comma-separated values: e.g., for Status enumeration list with commas. Ensure not to include pipe characters.

Also for Description may contain apostrophes and quotes; fine.

Now produce final answer.
## TOKEN USAGE

TokenUsage { prompt: 11606, completion: 11016, total: 22622 }
---
# Description
**Description**

This dataset contains one million New York City 311 service‑request records spanning from January 2010 to December 2020. Each record is identified by a unique key and records the timestamps for creation, closure, and due dates; the agency responsible; the complaint type with a detailed descriptor; and extensive geographic information such as address, ZIP code, borough, latitude/longitude, and intersection data. The bulk of complaints were filed between 2013 and 2015, with noise‑related issues accounting for roughly one‑fifth of all reports. A very high proportion (≈95 %) of records remain open or are marked “Closed” only after long periods, while the majority of fields contain a large “Other” category that absorbs remaining diversity.

**Notable Characteristics**

- **Central tendency & spread** – Created dates cluster around late 2015 (median ≈ 2015‑11‑10), but span a decade; due dates are heavily skewed toward early 2020.  
- **Distribution shape** – Complaint type and descriptor distributions are highly right‑skewed: the top categories (“Noise – Residential”, “HEAT/HOT WATER”) dominate, while an “Other” bucket contains ~56 % of all complaints for these fields.  
- **Missing values** – Significant gaps exist in key geographic fields: Incident Zip null ≈ 5.5 %, Incident Address null ≈ 17 %, Location Type null ≈ 24 %. Vehicle Type is almost entirely null (≈ 99.9 %).  
- **High cardinality** – Address, latitude/longitude, and BBL exhibit very high uniqueness ratios; most records have unique combinations of these fields.  
- **Duplicate keys** – The Unique Key field has a 1:1 mapping to records, confirming no duplicates.  
- **PII considerations** – While the dataset contains street addresses, it does not include personal names or contact information, so it is unlikely to contain personally identifiable data in the strict sense.  
- **Data quality issues** – Inconsistent naming conventions (e.g., “HEAT/HOT WATER” vs. “HEATING”), a large proportion of “Other” categories, and many nulls suggest that downstream processing should include cleaning and standardization steps.  
- **Spatial spread** – Latitude/longitude values cover the full five boroughs, but a substantial fraction (≈ 74 %) fall into an “Other” bucket indicating incomplete or non‑standardized coordinates.

---

Generated by qsv's describegpt command with prompt file v4.0.0 using openai/gpt-oss-20b on 2025-11-23T18:42:30.768418+00:00


## REASONING

We need to generate a description of dataset as a whole, not per field. Use summary statistics and frequency distribution to inform. We have 1 million records (Unique Key). Created date range 2010-2020; majority early 2013-2015. Closed dates mostly null or older 2010-2014. Agency: NYPD, HPD etc. Complaint type major noise residential etc. Many missing values e.g., Incident Zip null 5%? Actually Incident Zip null 5.5%. Location fields many nulls too. High cardinality of unique addresses etc. PII? Might include addresses and agency names; but probably not personal data. Notable characteristics: skewness, top categories dominate, heavy tail with "Other" categories. Missing values in Created Date? No. Closed date missing 2.8%. Some fields have high null counts e.g., Vehicle Type nearly all null. Many other fields have low cardinality.

Also we can mention that dataset is NYC 311 complaints dataset from 2010-2020; contains 1 million service requests.

We need to produce 1-8 sentences description, then bullet list of notable characteristics, then footnote placeholder.

Let's craft description: "This dataset comprises one million New York City 311 service request records collected between January 2010 and December 2020. Each record includes a unique identifier, timestamps for creation, closure, and due dates; information about the agency responsible; complaint type and detailed descriptor; geographic details such as address, ZIP code, borough, latitude/longitude, and intersection data; and additional attributes like vehicle or park facility involved. The majority of complaints were filed between 2013 and 2015, with noise-related issues representing nearly one‑fifth of all reports. A large proportion of records are still open (≈95 % closed status) and many fields contain substantial “Other” categories that absorb the remaining diversity."

Now bullet list: central tendency: Created Date median late 2015; range etc. Distribution shape: skewed, heavy tail in "Other" categories for complaint type and descriptor. Outliers: None numeric but coordinates vary widely across NYC. Missing values: Incident Zip null ~5.5%; Incident Address null ~17%; Location Type null 24%; Vehicle Type null 99.9%. Duplicate unique key no duplicates. PII: addresses could be considered sensitive but not personal names. Data quality issues: high percentage of "Other" categories, inconsistent naming conventions (e.g., "HEAT/HOT WATER" vs "HEATING"). Some fields like Open Data Channel Type have many unknowns.

Also note cardinality: Many unique values in address and latitude/longitude, but limited uniqueness ratio for others. Provide mention of "Other" category absorbing many unique values.

Add footnote placeholder {GENERATED_BY_SIGNATURE}.

Let's produce Markdown.
## TOKEN USAGE

TokenUsage { prompt: 14779, completion: 1111, total: 15890 }
---
# Tags
- new_york_city  
- nyc_311_complaints  
- public_infrastructure  
- noise_disputes  
- heat_and_hot_water_problems  
- illegal_parking  
- street_condition  
- residential_building  
- transportation_services  
- parks_and_rec_facilities  
- geospatial_coordinates  
- boroughs  
- community_boards  
- open_data_access  
- vehicle_types


## REASONING

We need to produce up to 15 tags in markdown format, lowercase with underscores. Based on dataset contents: NYC complaints dataset? It's about NYC 311 service requests. Tags should be thematic.

Consider key aspects: city of New York, public services, complaints, infrastructure, noise, heat/hot water, parking, street condition, building, residential, transportation, parks, open data channel, GIS coordinates, community board, boroughs, vehicle types, taxi pick up location, bridge highway segments, etc.

Generate tags like:

- new_york_city
- 311_complaint_service
- public_infrastructure
- noise_issues
- heat_and_hot_water
- illegal_parking
- street_condition
- residential_building
- transportation_related
- parks_and_rec
- geospatial_data
- boroughs
- community_board
- open_data_access
- vehicle_type

That's 15 tags. Ensure lowercase with underscores.

We can also include "nyc_311" but that's similar to 311_complaint_service. We might keep that too? But limit to 15.

Let's produce markdown list:

```markdown
- new_york_city
- nyc_311_complaints
- public_infrastructure
- noise_disputes
- heat_and_hot_water_problems
- illegal_parking
- street_condition
- residential_building
- transportation_services
- parks_and_rec_facilities
- geospatial_coordinates
- boroughs
- community_boards
- open_data_access
- vehicle_types
```

Check count: 15. Good.

Return as markdown list, no code fences? They said Markdown format, so list can be plain bullet points. Provide tags separated by commas? But "Markdown format" likely bullet list or comma-separated. I'll use unordered list.
## TOKEN USAGE

TokenUsage { prompt: 14699, completion: 460, total: 15159 }
---
