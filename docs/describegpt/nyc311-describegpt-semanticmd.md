---
semantic-md: datadict.yaml
dataset:
  id: NYC_311_SR_2010-2020-sample-1M
  title: NYC 311 SR 2010 2020 sample 1M
  row_count: 1000000
  grain: "one row = one NYC 311 complaint record"
  source: https://data.cityofnewyork.us/Social-Services/311-Service-Requests-from-2010-to-Present/erm2-nwe9
  updated: 2020-12-23
  license: NYC Open Data Terms of Use
  temporal_coverage: { column: "Created Date", start: "2010-01-01T00:00:00+00:00", end: "2020-12-23T01:25:51+00:00" }
  spatial: { crs: "EPSG:4326", lat: "Latitude", lon: "Longitude" }
concepts:
  - category.channel
  - category.status
  - category.type
  - geo.city
  - geo.coordinate_pair
  - geo.crs_stateplane_x
  - geo.crs_stateplane_y
  - geo.latitude
  - geo.longitude
  - geo.zip_code
  - id.surrogate_key
  - nyc.borough
  - nyc.community_board
  - nyc.complaint_type
  - org.agency
  - time.event_timestamp
tags:
  - nyc_311_complaints
  - noise
  - heat_and_hot_water
  - illegal_parking
  - street_condition
  - residential_building
  - parks_and_rec
  - transportation_incidents
  - environmental_health
---

# Dataset NYC_311_SR_2010-2020-sample-1M

**Description**

This dataset contains 1 000 000 NYC 311 complaint records collected between January 2010 and December 2020. Each record is uniquely identified by an integer key, and the timestamps for when complaints were created span a full decade with a concentration of activity in late‑2015. The majority of complaints are filed through phone, and most involve the NYPD or HPD agencies. Complaint categories are highly imbalanced: noise – residential (≈9 %) and heat/hot water (≈6 %) are common, but an “Other” category accounts for more than half of all records. Location information covers all five boroughs; about one‑third of incidents occur in residential buildings and roughly a fifth on streets or sidewalks, while many address components are missing.

---

### Notable Characteristics

- **Unique Identifier** – The `Unique Key` field is strictly unique (uniqueness ratio = 1) with no duplicate records.  
- **Temporal Distribution** – Created dates cluster around 2015–2016; the median falls on 12‑Feb‑2016 and skewness is mildly negative (≈ −0.086). Closed dates contain ~28 000 nulls (~2.8 %) and span an extreme range from 1900 to 2100, reflecting placeholder or missing values.  
- **Agency Distribution** – NYPD and HPD together account for about 52 % of complaints; the distribution is heavily skewed toward these two agencies.  
- **Complaint Type & Descriptor** – “Other” dominates both fields (>50 % each). Many specific categories have very low frequencies, creating a long‑tailed distribution.  
- **Location Data Quality** – Latitude and longitude are missing for 25 % of records; city is missing in 6 %, landmark in 91 %. Coordinates cluster around Manhattan and Brooklyn with a slight right‑skewness.  
- **Outliers & Extremes** – The `Unique Key` range (≈11 M–48 M) contains no duplicates but represents the full spectrum of IDs; date fields occasionally contain extreme placeholder values (1900, 2100).  
- **Missing Values** – Several fields have high sparsity: Landmark (~91 % null), Address Type (~12.6 %), City (~6 %).  
- **PII/PHI Considerations** – Street addresses and geographic coordinates can be considered location‑based personal information; the unique key may link to other datasets, potentially revealing additional sensitive details.  
- **Data Quality & Aggregation** – High cardinality in address fields leads to many unique values, complicating aggregation and analysis; “Other” categories obscure underlying diversity.

## Grain

one row = one NYC 311 complaint record

| Resource | Schema | Title | Rows |
| --- | --- | --- | ---: |
| `NYC_311_SR_2010-2020-sample-1M.csv` | `nyc311sr20102020sample1m` | NYC 311 SR 2010 2020 sample 1M | 1000000 |

# Schema `nyc311sr20102020sample1m`

| Column | Type | Role | Concept | Join? | Null | Label |
| --- | --- | --- | --- | :---: | ---: | --- |
| `Unique Key` | required integer | identifier | `id.surrogate_key` | PK | 0 | Unique Key |
| `Created Date` | required timestamp | timestamp | `time.event_timestamp` | FK? | 0 | Created Date |
| `Closed Date` | timestamp | timestamp | `time.event_timestamp` | FK? | 28619 | Closed Date |
| `Agency` | required text | dimension | `org.agency` | FK? | 0 | Agency Code |
| `Agency Name` | required text | dimension | `org.agency` | FK? | 0 | Agency Name |
| `Complaint Type` | required text | dimension | `nyc.complaint_type` | FK? | 0 | Complaint Category |
| `Descriptor` | text | dimension | `nyc.complaint_type` | FK? | 3001 | Complaint Sub‑Category |
| `Location Type` | text | dimension | `category.type` |  | 239131 | Location Classification |
| `Incident Zip` | text | dimension | `geo.zip_code` | FK? | 54978 | Incident ZIP Code |
| `Incident Address` | text | dimension | `unknown` |  | 174700 | Incident Street Address |
| `Street Name` | text | dimension | `unknown` |  | 174720 | Primary Street Name |
| `Cross Street 1` | text | dimension | `unknown` |  | 320401 | First Cross Street |
| `Cross Street 2` | text | dimension | `unknown` |  | 323644 | Second Cross Street |
| `Intersection Street 1` | text | dimension | `unknown` |  | 767422 | First Intersection Street |
| `Intersection Street 2` | text | dimension | `unknown` |  | 767709 | Second Intersection Street |
| `Address Type` | text | dimension | `category.type` |  | 125802 | Address Component Type |
| `City` | text | dimension | `geo.city` | FK? | 61963 | City / Borough |
| `Landmark` | text | dimension | `unknown` |  | 912779 | Nearby Landmark |
| `Facility Type` | text | dimension | `unknown` |  | 145478 | Facility Category |
| `Status` | required text | dimension | `category.status` |  | 0 | Complaint Status |
| `Due Date` | timestamp | timestamp | `time.event_timestamp` | FK? | 647794 | Resolution Due Date |
| `Resolution Description` | text | dimension | `unknown` |  | 20480 | Resolution Narrative |
| `Resolution Action Updated Date` | timestamp | timestamp | `time.event_timestamp` | FK? | 15072 | Last Resolution Update |
| `Community Board` | required text | dimension | `nyc.community_board` | FK? | 0 | Community Board |
| `BBL` | text | dimension | `unknown` |  | 243046 | Brooklyn Borough Lot ID |
| `Borough` | required text | dimension | `nyc.borough` | FK? | 0 | NYC Borough |
| `X Coordinate (State Plane)` | integer | dimension | `geo.crs_stateplane_x` | FK? | 85327 | State Plane X Coordinate |
| `Y Coordinate (State Plane)` | integer | dimension | `geo.crs_stateplane_y` | FK? | 85327 | State Plane Y Coordinate |
| `Open Data Channel Type` | required text | dimension | `category.channel` |  | 0 | Submission Channel |
| `Park Facility Name` | required text | dimension | `unknown` |  | 0 | Park Facility |
| `Park Borough` | required text | dimension | `nyc.borough` | FK? | 0 | Park Borough |
| `Vehicle Type` | text | dimension | `unknown` |  | 999652 | Vehicle Category |
| `Taxi Company Borough` | text | dimension | `nyc.borough` | FK? | 999156 | Taxi Company Borough |
| `Taxi Pick Up Location` | text | dimension | `unknown` |  | 992129 | Taxi Pickup Point |
| `Bridge Highway Name` | text | dimension | `unknown` |  | 997711 | Bridge/Highway Name |
| `Bridge Highway Direction` | text | dimension | `unknown` |  | 997691 | Bridge/Highway Direction |
| `Road Ramp` | text | dimension | `unknown` |  | 997693 | Ramp Type |
| `Bridge Highway Segment` | text | dimension | `unknown` |  | 997556 | Bridge/Highway Segment |
| `Latitude` | number | dimension | `geo.latitude` | FK? | 254695 | Latitude |
| `Longitude` | number | dimension | `geo.longitude` | FK? | 254695 | Longitude |
| `Location` | text | dimension | `geo.coordinate_pair` | FK? | 254695 | Latitude/Longitude Pair |

| Primary key |
| --- |
| `Unique Key` |

## Column `Unique Key`

A unique integer identifier for the complaint record.

- **Concept:** `id.surrogate_key`
- **Role:** identifier
- **Join:** primary key; cardinality 1:1; not nullable

### Validation

- Values >= 11465364
- Values <= 48478173

### Statistics

| Mean | Median | StdDev | Q1 | Q3 | Skew | Lower fence | Upper fence | Sparsity |
| ---: | ---: | ---: | ---: | ---: | ---: | ---: | ---: | ---: |
| 32687965.858 | 32853358.5 | 9013895.3358 | 25245773 | 40207433.5 | -0.0169 | 2803282.25 | 62649924.25 | 0 |

## Column `Created Date`

Timestamp indicating when the complaint was created in the system.

- **Concept:** `time.event_timestamp`
- **Role:** timestamp
- **Join:** candidate (concept `time.event_timestamp`); cardinality N:1; not nullable

## Column `Closed Date`

Timestamp indicating when the complaint was closed or resolved.

- **Concept:** `time.event_timestamp`
- **Role:** timestamp
- **Join:** candidate (concept `time.event_timestamp`); cardinality N:1; nullable
- **Quality:** placeholder-dates

## Column `Agency`

Abbreviated code for the agency responsible for addressing the complaint.

- **Concept:** `org.agency`
- **Role:** dimension
- **Join:** candidate (concept `org.agency`); cardinality N:1; not nullable

### Validation

- Length 3–42

## Column `Agency Name`

Full name of the agency handling the complaint.

- **Concept:** `org.agency`
- **Role:** dimension
- **Join:** candidate (concept `org.agency`); cardinality N:1; not nullable

### Validation

- Length 3–82

## Column `Complaint Type`

Primary category describing the nature of the complaint (e.g., noise, illegal parking).

- **Concept:** `nyc.complaint_type`
- **Role:** dimension
- **Join:** candidate (concept `nyc.complaint_type`); cardinality N:1; not nullable

### Validation

- Length 3–41

## Column `Descriptor`

Specific subcategory or detail within the complaint type.

- **Concept:** `nyc.complaint_type`
- **Role:** dimension
- **Join:** candidate (concept `nyc.complaint_type`); cardinality N:1; nullable

### Validation

- Length 0–80

## Column `Location Type`

General classification of where the incident occurred (e.g., residential building, street).

- **Concept:** `category.type`
- **Role:** dimension

### Validation

- Length 0–36

## Column `Incident Zip`

5‑digit ZIP code for the location of the incident.

- **Concept:** `geo.zip_code`
- **Role:** dimension
- **Join:** candidate (concept `geo.zip_code`); cardinality N:1; nullable

### Validation

- Length 0–10

## Column `Incident Address`

Full street address where the incident took place; includes building number, street name, and directional components. Combine with City, Borough, and ZIP Code to form a complete mailing address.

- **Concept:** `unknown`
- **Role:** dimension

### Validation

- Length 0–55

## Column `Street Name`

Name of the main street involved in the incident; part of the full mailing address.

- **Concept:** `unknown`
- **Role:** dimension

### Validation

- Length 0–55

## Column `Cross Street 1`

First cross street intersecting at the incident location; used when describing an intersection or blockface.

- **Concept:** `unknown`
- **Role:** dimension

### Validation

- Length 0–32

## Column `Cross Street 2`

Second cross street intersecting at the incident location, if applicable.

- **Concept:** `unknown`
- **Role:** dimension

### Validation

- Length 0–35

## Column `Intersection Street 1`

First street in an intersection where the incident occurred; part of the intersection description.

- **Concept:** `unknown`
- **Role:** dimension
- **Quality:** sparse

### Validation

- Length 0–35

## Column `Intersection Street 2`

Second street in an intersection where the incident occurred, if applicable.

- **Concept:** `unknown`
- **Role:** dimension
- **Quality:** sparse

### Validation

- Length 0–33

## Column `Address Type`

Classification of how the location is represented (e.g., ADDRESS, INTERSECTION, BLOCKFACE).

- **Concept:** `category.type`
- **Role:** dimension

### Validation

- Length 0–12

## Column `City`

Name of the city or borough where the incident occurred; used with ZIP code and state to form a complete mailing address.

- **Concept:** `geo.city`
- **Role:** dimension
- **Join:** candidate (concept `geo.city`); cardinality N:1; nullable

### Validation

- Length 0–22

## Column `Landmark`

Notable landmark or point of interest near the incident location, often used as a reference when locating the site.

- **Concept:** `unknown`
- **Role:** dimension
- **Quality:** sparse

### Validation

- Length 0–32

## Column `Facility Type`

Type of facility involved in the complaint, if any.

- **Concept:** `unknown`
- **Role:** dimension

### Validation

- Length 0–15

## Column `Status`

Current status of the complaint (e.g., Open, Closed).

- **Concept:** `category.status`
- **Role:** dimension

### Validation

- Length 4–16

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

Date and time by which the complaint was expected to be resolved.

- **Concept:** `time.event_timestamp`
- **Role:** timestamp
- **Join:** candidate (concept `time.event_timestamp`); cardinality N:1; nullable
- **Quality:** sparse

## Column `Resolution Description`

Narrative detailing how the complaint was resolved or why it remained open.

- **Concept:** `unknown`
- **Role:** dimension

### Validation

- Length 0–934

## Column `Resolution Action Updated Date`

Timestamp of the most recent update to the resolution action.

- **Concept:** `time.event_timestamp`
- **Role:** timestamp
- **Join:** candidate (concept `time.event_timestamp`); cardinality N:1; nullable

## Column `Community Board`

Council community board number or designation associated with the incident location.

- **Concept:** `nyc.community_board`
- **Role:** dimension
- **Join:** candidate (concept `nyc.community_board`); cardinality N:1; not nullable

### Validation

- Length 8–25

## Column `BBL`

Brooklyn Borough Lot identifier representing a specific parcel of land.

- **Concept:** `unknown`
- **Role:** dimension

### Validation

- Length 0–10

## Column `Borough`

New York City borough where the incident occurred.

- **Concept:** `nyc.borough`
- **Role:** dimension
- **Join:** candidate (concept `nyc.borough`); cardinality N:1; not nullable

### Validation

- Length 5–13

### Choices

- BRONX
- BROOKLYN
- MANHATTAN
- QUEENS
- STATEN ISLAND
- Unspecified

## Column `X Coordinate (State Plane)`

East–west coordinate in the State Plane coordinate system for the incident location.

- **Concept:** `geo.crs_stateplane_x`
- **Role:** dimension
- **Join:** candidate (concept `geo.crs_stateplane_x`); cardinality N:1; nullable

### Validation

- Values >= 913281
- Values <= 1067220

### Statistics

| Mean | Median | StdDev | Q1 | Q3 | Skew | Lower fence | Upper fence | Sparsity |
| ---: | ---: | ---: | ---: | ---: | ---: | ---: | ---: | ---: |
| 1005337.5451 | 1004546 | 22512.4528 | 993572 | 1018209 | 0.1091 | 956616.5 | 1055164.5 | 0.0853 |

## Column `Y Coordinate (State Plane)`

North–south coordinate in the State Plane coordinate system for the incident location.

- **Concept:** `geo.crs_stateplane_y`
- **Role:** dimension
- **Join:** candidate (concept `geo.crs_stateplane_y`); cardinality N:1; nullable

### Validation

- Values >= 121152
- Values <= 271876

### Statistics

| Mean | Median | StdDev | Q1 | Q3 | Skew | Lower fence | Upper fence | Sparsity |
| ---: | ---: | ---: | ---: | ---: | ---: | ---: | ---: | ---: |
| 205646.4978 | 202514 | 31723.1985 | 182411 | 235129 | 0.2373 | 103334 | 314206 | 0.0853 |

## Column `Open Data Channel Type`

Medium through which the complaint was submitted (e.g., PHONE, ONLINE).

- **Concept:** `category.channel`
- **Role:** dimension

### Validation

- Length 5–7

### Choices

- MOBILE
- ONLINE
- OTHER
- PHONE
- UNKNOWN

## Column `Park Facility Name`

Name of the park facility involved in the complaint.

- **Concept:** `unknown`
- **Role:** dimension

### Validation

- Length 3–82

## Column `Park Borough`

NYC borough where the park facility is located.

- **Concept:** `nyc.borough`
- **Role:** dimension
- **Join:** candidate (concept `nyc.borough`); cardinality N:1; not nullable

### Validation

- Length 5–13

### Choices

- BRONX
- BROOKLYN
- MANHATTAN
- QUEENS
- STATEN ISLAND
- Unspecified

## Column `Vehicle Type`

Type of vehicle involved in the incident, if any.

- **Concept:** `unknown`
- **Role:** dimension
- **Quality:** sparse

### Validation

- Length 0–23

## Column `Taxi Company Borough`

NYC borough where the taxi company is registered.

- **Concept:** `nyc.borough`
- **Role:** dimension
- **Join:** candidate (concept `nyc.borough`); cardinality N:1; nullable
- **Quality:** sparse

### Validation

- Length 0–13

## Column `Taxi Pick Up Location`

Location from which a taxi was picked up.

- **Concept:** `unknown`
- **Role:** dimension
- **Quality:** sparse

### Validation

- Length 0–60

## Column `Bridge Highway Name`

Name of the bridge or highway involved in the incident.

- **Concept:** `unknown`
- **Role:** dimension
- **Quality:** sparse

### Validation

- Length 0–42

## Column `Bridge Highway Direction`

Direction of travel on the bridge or highway (e.g., East/Long Island Bound).

- **Concept:** `unknown`
- **Role:** dimension
- **Quality:** sparse

### Validation

- Length 0–33

## Column `Road Ramp`

Type of ramp involved in the incident, if applicable.

- **Concept:** `unknown`
- **Role:** dimension
- **Quality:** sparse

### Validation

- Length 0–7

## Column `Bridge Highway Segment`

Specific segment or exit number on a bridge or highway.

- **Concept:** `unknown`
- **Role:** dimension
- **Quality:** sparse

### Validation

- Length 0–100

## Column `Latitude`

Geographic latitude coordinate of the incident location.

- **Concept:** `geo.latitude`
- **Role:** dimension
- **Join:** candidate (concept `geo.latitude`); cardinality N:1; nullable
- **Quality:** PII-location

### Validation

- Values >= 40.1123853
- Values <= 40.9128688

### Statistics

| Mean | Median | StdDev | Q1 | Q3 | Skew | Lower fence | Upper fence | Sparsity |
| ---: | ---: | ---: | ---: | ---: | ---: | ---: | ---: | ---: |
| 40.7288 | 40.7222 | 0.0893 | 40.6677 | 40.8031 | 0.1957 | 40.4646 | 41.0062 | 0.2547 |

## Column `Longitude`

Geographic longitude coordinate of the incident location.

- **Concept:** `geo.longitude`
- **Role:** dimension
- **Join:** candidate (concept `geo.longitude`); cardinality N:1; nullable
- **Quality:** PII-location

### Validation

- Values >= -77.5195844
- Values <= -73.7005968

### Statistics

| Mean | Median | StdDev | Q1 | Q3 | Skew | Lower fence | Upper fence | Sparsity |
| ---: | ---: | ---: | ---: | ---: | ---: | ---: | ---: | ---: |
| -73.93 | -73.9279 | 0.1635 | -73.9705 | -73.8763 | 0.0964 | -74.1119 | -73.7349 | 0.2547 |

## Column `Location`

Textual representation of latitude and longitude as a point pair; derived from Latitude and Longitude fields.

- **Concept:** `geo.coordinate_pair`
- **Role:** dimension
- **Join:** candidate (concept `geo.coordinate_pair`); cardinality N:1; nullable
- **Quality:** PII-location

### Validation

- Length 0–40

# Example Queries

Load the resource once, then run any query below. SQL targets DuckDB (qsv's `sqlp` engine); timestamps may need an explicit cast.

```python
import pandas as pd
df = pd.read_csv("NYC_311_SR_2010-2020-sample-1M.csv")
```

### Count by Agency

```sql
SELECT "Agency", count(*) AS n FROM 'NYC_311_SR_2010-2020-sample-1M.csv' GROUP BY 1 ORDER BY n DESC LIMIT 20;
```

```python
df.groupby("Agency").size().sort_values(ascending=False).head(20)
```

### Count by Agency Name

```sql
SELECT "Agency Name", count(*) AS n FROM 'NYC_311_SR_2010-2020-sample-1M.csv' GROUP BY 1 ORDER BY n DESC LIMIT 20;
```

```python
df.groupby("Agency Name").size().sort_values(ascending=False).head(20)
```

### Count by Complaint Type

```sql
SELECT "Complaint Type", count(*) AS n FROM 'NYC_311_SR_2010-2020-sample-1M.csv' GROUP BY 1 ORDER BY n DESC LIMIT 20;
```

```python
df.groupby("Complaint Type").size().sort_values(ascending=False).head(20)
```

### Monthly volume by Created Date

```sql
SELECT date_trunc('month', try_cast("Created Date" AS TIMESTAMP)) AS month, count(*) AS n FROM 'NYC_311_SR_2010-2020-sample-1M.csv' GROUP BY 1 ORDER BY 1;
```

```python
df.assign(month=pd.to_datetime(df["Created Date"], errors="coerce").dt.to_period("M")).groupby("month").size()
```

### Join a catalog dataset sharing concept `id.surrogate_key`

```sql
-- Any catalog dataset whose column carries concept 'id.surrogate_key' joins here.
-- SELECT a.*, b.* FROM 'NYC_311_SR_2010-2020-sample-1M.csv' a JOIN 'other.csv' b ON a."Unique Key" = b.<col with concept id.surrogate_key>;
```

```python
# merged = a.merge(b, left_on="Unique Key", right_on=<b col with concept 'id.surrogate_key'>)
```

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
Command line: ./target/debug/qsv describegpt NYC_311_SR_2010-2020-sample-1M.csv --all --two-pass --fresh --format semanticmd --base-url http://localhost:1234/v1 --model openai/gpt-oss-20b --ds-source https://data.cityofnewyork.us/Social-Services/311-Service-Requests-from-2010-to-Present/erm2-nwe9 --ds-license NYC Open Data Terms of Use --ds-updated 2020-12-23 --output docs/describegpt/nyc311-describegpt-semanticmd.md
Prompt file: Default v7.1.0
Model: openai/gpt-oss-20b
LLM API URL: http://localhost:1234/v1
Language: 
Timestamp: 2026-06-02T16:27:09.206095+00:00

WARNING: Label and Description generated by an LLM and may contain inaccuracies. Verify before using!
*
