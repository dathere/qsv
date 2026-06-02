---
semantic-md: datadict.yaml
tags:
  - nyc_311
  - public_complaints
  - noise_and_safety
  - building_and_housing
  - transportation_and_parking
  - geospatial_data
  - boroughs
  - open_data
  - agency_response
---

# Dataset NYC_311_SR_2010-2020-sample-1M

**Description**

This NYC 311 complaint dataset contains one million incident records recorded between January 2010 and December 2020. Each record is identified by a surrogate `Unique Key` that guarantees no duplicate complaints, while the majority of other fields exhibit high cardinality—over 300 000 unique latitude‑longitude pairs, 340 000 distinct street addresses, and more than 500 000 distinct ZIP codes. The temporal distribution shows a concentration of complaints around the middle of the decade (median created date ≈ 2015), with a broad range of roughly 11 years for both creation and closure dates. Most complaints are categorized as “Other” in the `Complaint Type` and `Descriptor` fields, reflecting a large proportion of unstructured or rarely used categories. The dataset also contains substantial missingness: nearly 28 000 closed dates are null, over 647 000 due dates are missing, and many address‑type and landmark fields lack values.

**Notable Characteristics**

- **Scale & uniqueness** – 1 M records with a unique surrogate key; no duplicate complaint identifiers.
- **Temporal spread** – Created dates span 4009 days (≈ 11 years), median ≈ 2015; Closed dates cover a similar period but contain many nulls (≈ 2.8 %).
- **High cardinality & sparsity** – > 300 k unique latitude/longitude pairs, > 340 k street addresses, > 500 k ZIP codes; 74 % of incident ZIPs fall under an “Other” bucket.
- **Skewed categorical distributions** – `Complaint Type` and `Descriptor` are dominated by a single “Other” category (≈ 56 % & 67 % respectively); many other fields similarly have large “Other” proportions, indicating unstructured or infrequent values.
- **Missingness** – Significant nulls in Closed Date, Due Date, Address Type, Landmark, and several coordinate-related fields; these gaps could bias analyses that rely on closure status or precise location.
- **Outliers & data quality** – Created dates include extreme past/future placeholders (e.g., 2010‑01‑01 and 2020‑12‑23); Closed Date has boundary values from 1900‑01‑01 to 2100‑01‑01 used as defaults. Inconsistent formatting of addresses (e.g., varying capitalization, punctuation) may hinder spatial joins.
- **PII/PHI considerations** – The dataset contains street addresses and exact latitude/longitude points; while it does not include personal names or identifiers, the precision of location data can pose privacy concerns if combined with other datasets.

| Resource | Schema | Title |
| --- | --- | --- |
| `NYC_311_SR_2010-2020-sample-1M.csv` | `nyc311sr20102020sample1m` | NYC 311 SR 2010 2020 sample 1M |

# Schema `nyc311sr20102020sample1m`

| Column | Type | Label |
| --- | --- | --- |
| `Unique Key` | required integer | Unique Key |
| `Created Date` | required timestamp | Created Date |
| `Closed Date` | timestamp | Closed Date |
| `Agency` | required text | Agency Code |
| `Agency Name` | required text | Agency Name |
| `Complaint Type` | required text | Complaint Category |
| `Descriptor` | text | Descriptor |
| `Location Type` | text | Location Category |
| `Incident Zip` | text | Incident ZIP Code |
| `Incident Address` | text | Incident Street Address |
| `Street Name` | text | Primary Street Name |
| `Cross Street 1` | text | First Cross Street |
| `Cross Street 2` | text | Second Cross Street |
| `Intersection Street 1` | text | First Intersection Street |
| `Intersection Street 2` | text | Second Intersection Street |
| `Address Type` | text | Address Type |
| `City` | text | City / Borough |
| `Landmark` | text | Nearby Landmark |
| `Facility Type` | text | Facility Category |
| `Status` | required text | Complaint Status |
| `Due Date` | timestamp | Resolution Deadline |
| `Resolution Description` | text | Resolution Narrative |
| `Resolution Action Updated Date` | timestamp | Last Resolution Update |
| `Community Board` | required text | Community Board District |
| `BBL` | text | Building Block Lot Number |
| `Borough` | required text | NYC Borough |
| `X Coordinate (State Plane)` | integer | X Coordinate (State Plane) |
| `Y Coordinate (State Plane)` | integer | Y Coordinate (State Plane) |
| `Open Data Channel Type` | required text | Data Entry Channel |
| `Park Facility Name` | required text | Park Facility |
| `Park Borough` | required text | Park Borough |
| `Vehicle Type` | text | Vehicle Category |
| `Taxi Company Borough` | text | Taxi Company Borough |
| `Taxi Pick Up Location` | text | Taxi Pickup Point |
| `Bridge Highway Name` | text | Bridge/Highway Name |
| `Bridge Highway Direction` | text | Bridge/Highway Direction |
| `Road Ramp` | text | Ramp Type |
| `Bridge Highway Segment` | text | Highway Segment / Exit |
| `Latitude` | number | Latitude |
| `Longitude` | number | Longitude |
| `Location` | text | Coordinate Pair |

| Primary key |
| --- |
| `Unique Key` |

## Column `Unique Key`

A surrogate numeric identifier that uniquely distinguishes each complaint record.

### Validation

Values >= 11465364

## Column `Created Date`

The date and time when the complaint was entered into the system, indicating the creation moment of the incident report.

## Column `Closed Date`

The date and time when the complaint was officially closed or resolved in the system, marking the completion of investigation or action.

## Column `Agency`

Short code identifying the agency that handled the complaint (e.g., NYPD, HPD).

## Column `Agency Name`

Full name of the agency that processed the complaint, providing descriptive context beyond the short code.

## Column `Complaint Type`

Primary category of the complaint (e.g., Noise, Illegal Parking) grouping incidents into high‑level topics.

## Column `Descriptor`

A more specific descriptor within the complaint type that explains the nature or detail of the issue.

## Column `Location Type`

Classification of where the incident occurred (e.g., RESIDENTIAL BUILDING, STREET).

## Column `Incident Zip`

Five‑digit postal code for the location of the incident, used to locate the area within New York City.

## Column `Incident Address`

Full street address where the complaint was reported, including building number and street name; combine with city, zip code, borough to form a complete mailing address.

## Column `Street Name`

Name of the primary street involved in the incident (e.g., BROADWAY); part of the full incident address.

## Column `Cross Street 1`

Name of the first cross street intersecting at or near the incident location; component of the geographic context.

## Column `Cross Street 2`

Name of the second cross street, if applicable, intersecting at the incident location; further refines the location context.

## Column `Intersection Street 1`

First street name in an intersection related to the complaint; part of the broader intersection description.

## Column `Intersection Street 2`

Second street name in an intersection related to the complaint; completes the intersection pair.

## Column `Address Type`

The type of address provided (e.g., ADDRESS, INTERSECTION, BLOCKFACE) indicating how the location is described.

## Column `City`

Name of the city or borough in which the incident occurred; for NYC this corresponds to one of the five boroughs.

## Column `Landmark`

Notable landmark near the incident location (e.g., EAST 230 STREET) providing additional spatial context.

## Column `Facility Type`

Type of facility where the complaint was filed (e.g., DSNY Garage, School District).

## Column `Status`

Current status of the complaint record (e.g., Closed, Pending, Open), indicating progress in handling the issue.

### Choices

- Assigned
- Closed
- Closed - Testing
- Email Sent
- In Progress
- Open
- Pending
- Started
- Unassigned
- Unspecified

## Column `Due Date`

The deadline by which a response or action is expected for the complaint.

## Column `Resolution Description`

Free‑form narrative describing how the complaint was resolved, including actions taken or reasons for closure.

## Column `Resolution Action Updated Date`

Timestamp of the most recent update to the resolution action, tracking when the status was last modified.

## Column `Community Board`

The community board district associated with the incident location (e.g., 12 MANHATTAN).

## Column `BBL`

Unique identifier for NYC parcels used in municipal land records.

## Column `Borough`

The borough of New York City where the incident occurred (e.g., BROOKLYN).

### Choices

- BRONX
- BROOKLYN
- MANHATTAN
- QUEENS
- STATEN ISLAND
- Unspecified

## Column `X Coordinate (State Plane)`

East‑ing coordinate in the state plane projection system for precise mapping within NYC.

### Validation

Values >= 913281

## Column `Y Coordinate (State Plane)`

North‑ing coordinate in the state plane projection system, complementing the X coordinate for location mapping.

### Validation

Values >= 121152

## Column `Open Data Channel Type`

Channel through which the complaint was submitted (e.g., PHONE, ONLINE).

### Choices

- MOBILE
- ONLINE
- OTHER
- PHONE
- UNKNOWN

## Column `Park Facility Name`

Name of a park facility involved in or near the incident (e.g., CENTRAL PARK).

## Column `Park Borough`

Borough where the park facility is located.

### Choices

- BRONX
- BROOKLYN
- MANHATTAN
- QUEENS
- STATEN ISLAND
- Unspecified

## Column `Vehicle Type`

Type of vehicle associated with the incident (e.g., Car Service, Green Taxi).

## Column `Taxi Company Borough`

Borough where the taxi company operates or was registered.

## Column `Taxi Pick Up Location`

Descriptive location of a taxi pick‑up point, often including airport names or intersections.

## Column `Bridge Highway Name`

Name of the bridge or highway involved in the incident (e.g., Belt Pkwy).

## Column `Bridge Highway Direction`

Direction of travel on the bridge or highway (e.g., East/Long Island Bound).

## Column `Road Ramp`

Type of ramp associated with the incident (e.g., Roadway, Ramp).

## Column `Bridge Highway Segment`

Specific segment or exit number on a bridge or highway.

## Column `Latitude`

Geographic latitude coordinate of the incident location in decimal degrees.

### Validation

Values >= 40.1123853

## Column `Longitude`

Geographic longitude coordinate of the incident location in decimal degrees.

### Validation

Values >= -77.5195844

## Column `Location`

Combined latitude and longitude pair formatted as a string, providing a precise point reference for mapping.

# Resource `NYC_311_SR_2010-2020-sample-1M.csv`

## Statistics

| Column | Min | Max | Cardinality | Null Count |
| --- | ---: | ---: | ---: | ---: |
| `Unique Key` | 11465364 | 48478173 | 1000000 | 0 |
| `Created Date` | 2010-01-01T00:00:00+00:00 | 2020-12-23T01:25:51+00:00 | 841014 | 0 |
| `Closed Date` | 1900-01-01T00:00:00+00:00 | 2100-01-01T00:00:00+00:00 | 688837 | 28619 |
| `Agency` | 3-1-1 | TLC | 28 | 0 |
| `Agency Name` | 3-1-1 | Valuation Policy | 553 | 0 |
| `Complaint Type` | ../../WEB-INF/web.xml;x= | ZTESTINT | 287 | 0 |
| `Descriptor` | 1 Missed Collection | unknown odor/taste in drinking water (QA6) | 1392 | 3001 |
| `Location Type` | 1-, 2- and 3- Family Home | Wooded Area | 162 | 239131 |
| `Incident Zip` | * | XXXXX | 535 | 54978 |
| `Incident Address` | * * | west 155 street and edgecombe avenue | 341996 | 174700 |
| `Street Name` | * | wyckoff avenue | 14837 | 174720 |
| `Cross Street 1` | 1 AVE | mermaid | 16238 | 320401 |
| `Cross Street 2` | 1 AVE | surf | 16486 | 323644 |
| `Intersection Street 1` | 1 AVE | flatlands AVE | 11237 | 767422 |
| `Intersection Street 2` | 1 AVE | glenwood RD | 11674 | 767709 |
| `Address Type` | ADDRESS | PLACENAME | 6 | 125802 |
| `City` | * | YORKTOWN HEIGHTS | 382 | 61963 |
| `Landmark` | 1 AVENUE | ZULETTE AVENUE | 5915 | 912779 |
| `Facility Type` | DSNY Garage | School District | 6 | 145478 |
| `Status` | Assigned | Unspecified | 10 | 0 |
| `Due Date` | 1900-01-02T00:00:00+00:00 | 2021-06-17T16:34:13+00:00 | 345077 | 647794 |
| `Resolution Description` | A DOB violation was issued for failing to comply with an existing Stop Work Order. | Your request was submitted to the Department of Homeless Services. The City?s outreach team will assess the homeless individual and offer appropriate assistance within 2 hours. If you asked to know the outcome of your request, you will get a call within 2 hours. No further status will be available through the NYC 311 App, 311, or 311 Online. | 1216 | 20480 |
| `Resolution Action Updated Date` | 2009-12-31T01:35:00+00:00 | 2020-12-23T06:56:14+00:00 | 690314 | 15072 |
| `Community Board` | 0 Unspecified | Unspecified STATEN ISLAND | 77 | 0 |
| `BBL` | 0000000000 | 5080470043 | 268383 | 243046 |
| `Borough` | BRONX | Unspecified | 6 | 0 |
| `X Coordinate (State Plane)` | 913281 | 1067220 | 102556 | 85327 |
| `Y Coordinate (State Plane)` | 121152 | 271876 | 116092 | 85327 |
| `Open Data Channel Type` | MOBILE | UNKNOWN | 5 | 0 |
| `Park Facility Name` | "Uncle" Vito F. Maranzano Glendale Playground | Zimmerman Playground | 1889 | 0 |
| `Park Borough` | BRONX | Unspecified | 6 | 0 |
| `Vehicle Type` | Ambulette / Paratransit | Green Taxi | 5 | 999652 |
| `Taxi Company Borough` | BRONX | Staten Island | 11 | 999156 |
| `Taxi Pick Up Location` | 1 5 AVENUE MANHATTAN | YORK AVENUE AND EAST 70 STREET | 1903 | 992129 |
| `Bridge Highway Name` | 145th St. Br - Lenox Ave | Willis Ave Br - 125th St/1st Ave | 68 | 997711 |
| `Bridge Highway Direction` | Bronx Bound | Westbound/To Goethals Br | 50 | 997691 |
| `Road Ramp` | N/A | Roadway | 4 | 997693 |
| `Bridge Highway Segment` | 1-1-1265963747 | Wythe Ave/Kent Ave (Exit 31) | 937 | 997556 |
| `Latitude` | 40.1123853 | 40.9128688 | 353694 | 254695 |
| `Longitude` | -77.5195844 | -73.7005968 | 353996 | 254695 |
| `Location` | (40.1123853, -77.5195844) | (40.9128688, -73.9024731) | 375772 | 254695 |

### Frequency for `Created Date`

| Choice | Frequency | Percentage | Rank |
| :--- | ---: | ---: | ---: |
| Other… | 997333 | 99.73% |  |
| 01/24/2013 12:00:00 AM | 347 | 0.03% | 1 |
| 01/07/2014 12:00:00 AM | 315 | 0.03% | 2 |
| 01/08/2015 12:00:00 AM | 283 | 0.03% | 3 |
| 02/16/2015 12:00:00 AM | 269 | 0.03% | 4 |

### Frequency for `Closed Date`

| Choice | Frequency | Percentage | Rank |
| :--- | ---: | ---: | ---: |
| Other… | 968671 | 99.72% |  |
| (NULL)… | 28619 | 0.00% |  |
| 11/15/2010 12:00:00 AM | 384 | 0.04% | 1 |
| 11/07/2012 12:00:00 AM | 329 | 0.03% | 2 |
| 12/09/2010 12:00:00 AM | 267 | 0.03% | 3 |

### Frequency for `Agency`

| Choice | Frequency | Percentage | Rank |
| :--- | ---: | ---: | ---: |
| NYPD | 265116 | 26.51% | 1 |
| HPD | 258033 | 25.80% | 2 |
| DOT | 132462 | 13.25% | 3 |
| DSNY | 81606 | 8.16% | 4 |
| DEP | 75895 | 7.59% | 5 |

### Frequency for `Agency Name`

| Choice | Frequency | Percentage | Rank |
| :--- | ---: | ---: | ---: |
| New York City Police Depa… | 265038 | 26.50% | 1 |
| Department of Housing Pre… | 258019 | 25.80% | 2 |
| Department of Transportat… | 132462 | 13.25% | 3 |
| Other… | 103974 | 10.40% |  |
| Department of Environment… | 75895 | 7.59% | 4 |

### Frequency for `Complaint Type`

| Choice | Frequency | Percentage | Rank |
| :--- | ---: | ---: | ---: |
| Other… | 563561 | 56.36% |  |
| Noise - Residential | 89439 | 8.94% | 1 |
| HEAT/HOT WATER | 56639 | 5.66% | 2 |
| Illegal Parking | 45032 | 4.50% | 3 |
| Blocked Driveway | 42356 | 4.24% | 4 |

### Frequency for `Descriptor`

| Choice | Frequency | Percentage | Rank |
| :--- | ---: | ---: | ---: |
| Other… | 671870 | 67.39% |  |
| Loud Music/Party | 93646 | 9.39% | 1 |
| ENTIRE BUILDING | 36885 | 3.70% | 2 |
| HEAT | 35088 | 3.52% | 3 |
| No Access | 31631 | 3.17% | 4 |

### Frequency for `Location Type`

| Choice | Frequency | Percentage | Rank |
| :--- | ---: | ---: | ---: |
| RESIDENTIAL BUILDING | 255562 | 33.59% | 1 |
| (NULL)… | 239131 | 0.00% |  |
| Street/Sidewalk | 145653 | 19.14% | 2 |
| Residential Building/Hous… | 92765 | 12.19% | 3 |
| Street | 92190 | 12.12% | 4 |

### Frequency for `Incident Zip`

| Choice | Frequency | Percentage | Rank |
| :--- | ---: | ---: | ---: |
| Other… | 815988 | 86.35% |  |
| (NULL)… | 54978 | 0.00% |  |
| 11226 | 17114 | 1.81% | 1 |
| 10467 | 14495 | 1.53% | 2 |
| 11207 | 12872 | 1.36% | 3 |

### Frequency for `Incident Address`

| Choice | Frequency | Percentage | Rank |
| :--- | ---: | ---: | ---: |
| Other… | 819046 | 99.24% |  |
| (NULL)… | 174700 | 0.00% |  |
| 655 EAST  230 STREET | 1538 | 0.19% | 1 |
| 78-15 PARSONS BOULEVARD | 694 | 0.08% | 2 |
| 672 EAST  231 STREET | 663 | 0.08% | 3 |

### Frequency for `Street Name`

| Choice | Frequency | Percentage | Rank |
| :--- | ---: | ---: | ---: |
| Other… | 784684 | 95.08% |  |
| (NULL)… | 174720 | 0.00% |  |
| BROADWAY | 9702 | 1.18% | 1 |
| GRAND CONCOURSE | 5851 | 0.71% | 2 |
| OCEAN AVENUE | 3946 | 0.48% | 3 |

### Frequency for `Cross Street 1`

| Choice | Frequency | Percentage | Rank |
| :--- | ---: | ---: | ---: |
| Other… | 619743 | 91.19% |  |
| (NULL)… | 320401 | 0.00% |  |
| BEND | 12562 | 1.85% | 1 |
| BROADWAY | 8548 | 1.26% | 2 |
| 3 AVENUE | 6154 | 0.91% | 3 |

### Frequency for `Cross Street 2`

| Choice | Frequency | Percentage | Rank |
| :--- | ---: | ---: | ---: |
| Other… | 623363 | 92.16% |  |
| (NULL)… | 323644 | 0.00% |  |
| BEND | 12390 | 1.83% | 1 |
| BROADWAY | 8833 | 1.31% | 2 |
| DEAD END | 5626 | 0.83% | 3 |

### Frequency for `Intersection Street 1`

| Choice | Frequency | Percentage | Rank |
| :--- | ---: | ---: | ---: |
| (NULL)… | 767422 | 0.00% |  |
| Other… | 214544 | 92.25% |  |
| BROADWAY | 3761 | 1.62% | 1 |
| CARPENTER AVENUE | 2918 | 1.25% | 2 |
| BEND | 2009 | 0.86% | 3 |

### Frequency for `Intersection Street 2`

| Choice | Frequency | Percentage | Rank |
| :--- | ---: | ---: | ---: |
| (NULL)… | 767709 | 0.00% |  |
| Other… | 215667 | 92.84% |  |
| BROADWAY | 3462 | 1.49% | 1 |
| BEND | 1942 | 0.84% | 2 |
| 2 AVENUE | 1690 | 0.73% | 3 |

### Frequency for `Address Type`

| Choice | Frequency | Percentage | Rank |
| :--- | ---: | ---: | ---: |
| ADDRESS | 710380 | 81.26% | 1 |
| INTERSECTION | 133361 | 15.26% | 2 |
| (NULL)… | 125802 | 0.00% |  |
| BLOCKFACE | 22620 | 2.59% | 3 |
| LATLONG | 7421 | 0.85% | 4 |

### Frequency for `City`

| Choice | Frequency | Percentage | Rank |
| :--- | ---: | ---: | ---: |
| BROOKLYN | 296254 | 31.58% | 1 |
| NEW YORK | 189069 | 20.16% | 2 |
| BRONX | 181168 | 19.31% | 3 |
| Other… | 163936 | 17.48% |  |
| (NULL)… | 61963 | 0.00% |  |

### Frequency for `Landmark`

| Choice | Frequency | Percentage | Rank |
| :--- | ---: | ---: | ---: |
| (NULL)… | 912779 | 0.00% |  |
| Other… | 80165 | 91.91% |  |
| EAST  230 STREET | 1545 | 1.77% | 1 |
| EAST  231 STREET | 1291 | 1.48% | 2 |
| BROADWAY | 1148 | 1.32% | 3 |

### Frequency for `Facility Type`

| Choice | Frequency | Percentage | Rank |
| :--- | ---: | ---: | ---: |
| N/A | 628279 | 73.52% | 1 |
| Precinct | 193259 | 22.62% | 2 |
| (NULL)… | 145478 | 0.00% |  |
| DSNY Garage | 32310 | 3.78% | 3 |
| School | 617 | 0.07% | 4 |

### Frequency for `Status`

| Choice | Frequency | Percentage | Rank |
| :--- | ---: | ---: | ---: |
| Closed | 952522 | 95.25% | 1 |
| Pending | 20119 | 2.01% | 2 |
| Open | 12340 | 1.23% | 3 |
| In Progress | 7841 | 0.78% | 4 |
| Assigned | 6651 | 0.67% | 5 |

### Frequency for `Due Date`

| Choice | Frequency | Percentage | Rank |
| :--- | ---: | ---: | ---: |
| (NULL)… | 647794 | 0.00% |  |
| Other… | 350746 | 99.59% |  |
| 04/08/2015 10:00:58 AM | 214 | 0.06% | 1 |
| 05/02/2014 03:32:17 PM | 183 | 0.05% | 2 |
| 03/30/2018 10:10:39 AM | 172 | 0.05% | 3 |

### Frequency for `Resolution Description`

| Choice | Frequency | Percentage | Rank |
| :--- | ---: | ---: | ---: |
| Other… | 511739 | 52.24% |  |
| The Police Department res… | 91408 | 9.33% | 1 |
| The Department of Housing… | 72962 | 7.45% | 2 |
| The Police Department res… | 63868 | 6.52% | 3 |
| Service Request status fo… | 52155 | 5.32% | 4 |

### Frequency for `Resolution Action Updated Date`

| Choice | Frequency | Percentage | Rank |
| :--- | ---: | ---: | ---: |
| Other… | 982148 | 99.72% |  |
| (NULL)… | 15072 | 0.00% |  |
| 11/15/2010 12:00:00 AM | 385 | 0.04% | 1 |
| 11/07/2012 12:00:00 AM | 336 | 0.03% | 2 |
| 12/09/2010 12:00:00 AM | 273 | 0.03% | 3 |

### Frequency for `Community Board`

| Choice | Frequency | Percentage | Rank |
| :--- | ---: | ---: | ---: |
| Other… | 751635 | 75.16% |  |
| 0 Unspecified | 49878 | 4.99% | 1 |
| 12 MANHATTAN | 29845 | 2.98% | 2 |
| 12 QUEENS | 23570 | 2.36% | 3 |
| 01 BROOKLYN | 21714 | 2.17% | 4 |

### Frequency for `BBL`

| Choice | Frequency | Percentage | Rank |
| :--- | ---: | ---: | ---: |
| Other… | 750668 | 99.17% |  |
| (NULL)… | 243046 | 0.00% |  |
| 2048330028 | 1566 | 0.21% | 1 |
| 4068290001 | 696 | 0.09% | 2 |
| 4015110001 | 664 | 0.09% | 3 |

### Frequency for `Borough`

| Choice | Frequency | Percentage | Rank |
| :--- | ---: | ---: | ---: |
| BROOKLYN | 296081 | 29.61% | 1 |
| QUEENS | 228818 | 22.88% | 2 |
| MANHATTAN | 195488 | 19.55% | 3 |
| BRONX | 180142 | 18.01% | 4 |
| Unspecified | 49878 | 4.99% | 5 |

### Frequency for `X Coordinate (State Plane)`

| Choice | Frequency | Percentage | Rank |
| :--- | ---: | ---: | ---: |
| Other… | 908535 | 99.33% |  |
| (NULL)… | 85327 | 0.00% |  |
| 1022911 | 1568 | 0.17% | 1 |
| 1037000 | 701 | 0.08% | 2 |
| 1023174 | 675 | 0.07% | 3 |

### Frequency for `Y Coordinate (State Plane)`

| Choice | Frequency | Percentage | Rank |
| :--- | ---: | ---: | ---: |
| Other… | 908538 | 99.33% |  |
| (NULL)… | 85327 | 0.00% |  |
| 264242 | 1566 | 0.17% | 1 |
| 202363 | 706 | 0.08% | 2 |
| 211606 | 665 | 0.07% | 3 |

### Frequency for `Open Data Channel Type`

| Choice | Frequency | Percentage | Rank |
| :--- | ---: | ---: | ---: |
| PHONE | 497606 | 49.76% | 1 |
| UNKNOWN | 230402 | 23.04% | 2 |
| ONLINE | 177334 | 17.73% | 3 |
| MOBILE | 79892 | 7.99% | 4 |
| OTHER | 14766 | 1.48% | 5 |

### Frequency for `Park Facility Name`

| Choice | Frequency | Percentage | Rank |
| :--- | ---: | ---: | ---: |
| Unspecified | 993141 | 99.31% | 1 |
| Other… | 5964 | 0.60% |  |
| Central Park | 261 | 0.03% | 2 |
| Riverside Park | 136 | 0.01% | 3 |
| Prospect Park | 129 | 0.01% | 4 |

### Frequency for `Park Borough`

| Choice | Frequency | Percentage | Rank |
| :--- | ---: | ---: | ---: |
| BROOKLYN | 296081 | 29.61% | 1 |
| QUEENS | 228818 | 22.88% | 2 |
| MANHATTAN | 195488 | 19.55% | 3 |
| BRONX | 180142 | 18.01% | 4 |
| Unspecified | 49878 | 4.99% | 5 |

### Frequency for `Vehicle Type`

| Choice | Frequency | Percentage | Rank |
| :--- | ---: | ---: | ---: |
| (NULL)… | 999652 | 0.00% |  |
| Car Service | 317 | 91.09% | 1 |
| Ambulette / Paratransit | 19 | 5.46% | 2 |
| Commuter Van | 11 | 3.16% | 3 |
| Green Taxi | 1 | 0.29% | 4 |

### Frequency for `Taxi Company Borough`

| Choice | Frequency | Percentage | Rank |
| :--- | ---: | ---: | ---: |
| (NULL)… | 999156 | 0.00% |  |
| BROOKLYN | 207 | 24.53% | 1 |
| QUEENS | 194 | 22.99% | 2 |
| MANHATTAN | 171 | 20.26% | 3 |
| BRONX | 127 | 15.05% | 4 |

### Frequency for `Taxi Pick Up Location`

| Choice | Frequency | Percentage | Rank |
| :--- | ---: | ---: | ---: |
| (NULL)… | 992129 | 0.00% |  |
| Other | 4091 | 51.98% | 1 |
| Other… | 2006 | 25.49% |  |
| JFK Airport | 562 | 7.14% | 2 |
| Intersection | 486 | 6.17% | 3 |

### Frequency for `Bridge Highway Name`

| Choice | Frequency | Percentage | Rank |
| :--- | ---: | ---: | ---: |
| (NULL)… | 997711 | 0.00% |  |
| Other… | 779 | 34.03% |  |
| Belt Pkwy | 276 | 12.06% | 1 |
| BQE/Gowanus Expwy | 254 | 11.10% | 2 |
| Grand Central Pkwy | 186 | 8.13% | 3 |

### Frequency for `Bridge Highway Direction`

| Choice | Frequency | Percentage | Rank |
| :--- | ---: | ---: | ---: |
| (NULL)… | 997691 | 0.00% |  |
| Other… | 987 | 42.75% |  |
| East/Long Island Bound | 210 | 9.09% | 1 |
| North/Bronx Bound | 208 | 9.01% | 2 |
| East/Queens Bound | 197 | 8.53% | 3 |

### Frequency for `Road Ramp`

| Choice | Frequency | Percentage | Rank |
| :--- | ---: | ---: | ---: |
| (NULL)… | 997693 | 0.00% |  |
| Roadway | 1731 | 75.03% | 1 |
| Ramp | 555 | 24.06% | 2 |
| N/A | 21 | 0.91% | 3 |

### Frequency for `Bridge Highway Segment`

| Choice | Frequency | Percentage | Rank |
| :--- | ---: | ---: | ---: |
| (NULL)… | 997556 | 0.00% |  |
| Other… | 2144 | 87.73% |  |
| Ramp | 92 | 3.76% | 1 |
| Roadway | 54 | 2.21% | 2 |
| Clove Rd/Richmond Rd (Exi… | 23 | 0.94% | 3 |

### Frequency for `Latitude`

| Choice | Frequency | Percentage | Rank |
| :--- | ---: | ---: | ---: |
| Other… | 739329 | 99.20% |  |
| (NULL)… | 254695 | 0.00% |  |
| 40.89187241649303 | 1538 | 0.21% | 1 |
| 40.1123853 | 1153 | 0.15% | 2 |
| 40.89238451539139 | 663 | 0.09% | 3 |

### Frequency for `Longitude`

| Choice | Frequency | Percentage | Rank |
| :--- | ---: | ---: | ---: |
| Other… | 739329 | 99.20% |  |
| (NULL)… | 254695 | 0.00% |  |
| -73.86016845296459 | 1538 | 0.21% | 1 |
| -77.5195844 | 1153 | 0.15% | 2 |
| -73.8592161325675 | 663 | 0.09% | 3 |

### Frequency for `Location`

| Choice | Frequency | Percentage | Rank |
| :--- | ---: | ---: | ---: |
| Other… | 739329 | 99.20% |  |
| (NULL)… | 254695 | 0.00% |  |
| (40.89187241649303, -73.8… | 1538 | 0.21% | 1 |
| (40.1123853, -77.5195844) | 1153 | 0.15% | 2 |
| (40.89238451539139, -73.8… | 663 | 0.09% | 3 |

*Attribution: Generated by qsv v20.1.0 describegpt
Command line: target/debug/qsv describegpt --all --two-pass /tmp/NYC_311_SR_2010-2020-sample-1M.csv -o /tmp/nyc311-describegpt-semanticmd.md --fresh --infer-content-type --format semanticmd
Prompt file: Default v7.1.0
Model: openai/gpt-oss-20b
LLM API URL: http://localhost:1234/v1
Language: 
Timestamp: 2026-06-02T11:52:17.433475+00:00

WARNING: Label and Description generated by an LLM and may contain inaccuracies. Verify before using!
*
