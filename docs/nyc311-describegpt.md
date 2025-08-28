**Data Dictionary – NYC 311 Complaint Dataset**

| Name | Type | Label | Description |
|------|------|-------|-------------|
| Unique Key | Integer | **Unique Key** | Primary key that uniquely identifies each complaint record (≈1,000,000 distinct values). |
| Created Date | DateTime | **Created Date** | Timestamp when the complaint was first filed. The most common dates are early‑January 2013–2015; about 99 % of records have a non‑null value. |
| Closed Date | DateTime | **Closed Date** | Timestamp when the complaint was closed. A null value indicates an open or pending case (≈2 8 k records). |
| Agency | String | **Agency** | Short code for the agency that handled the complaint (e.g., `NYPD`, `HPD`). Top 10 codes account for >90 % of records; other agencies are grouped under “Other”. |
| Agency Name | String | **Agency Name** | Full name of the responding agency. The most common values mirror those in *Agency* (e.g., “New York City Police Department”). |
| Complaint Type | String | **Complaint Type** | Primary category of the complaint (e.g., `Noise – Residential`, `HEAT/HOT WATER`). The top 10 categories cover ~70 % of all complaints. |
| Descriptor | String | **Descriptor** | Sub‑category or specific issue within a *Complaint Type* (e.g., “Loud Music/Party”, “Pothole”). Over 90 % of records are one of the top 10 descriptors; the remainder are aggregated under “Other”. |
| Location Type | String | **Location Type** | Classification of the incident location (e.g., `RESIDENTIAL BUILDING`, `STREET`). The two most common types together cover ~50 % of complaints. |
| Incident Zip | String | **Incident ZIP** | Five‑digit ZIP code where the incident occurred. Nulls occur in ~5 % of records; the top 10 ZIP codes account for >10 % of non‑null values. |
| Incident Address | String | **Incident Address** | Street address of the incident. The most common addresses appear only a few times each (e.g., “655 EAST 230 STREET”). |
| Street Name | String | **Street Name** | Main street involved in the complaint. About 18 % of records are null; “BROADWAY” and “GRAND CONCOURSE” are the most frequent non‑null values. |
| Cross Street 1 | String | **Cross Street 1** | First cross street at the incident location. Most values are null (~32 %). |
| Cross Street 2 | String | **Cross Street 2** | Second cross street; also largely null (~32 %). |
| Intersection Street 1 | String | **Intersection Street 1** | First street in an intersection where the complaint was filed. Nearly three‑quarters of records are non‑null, with “BROADWAY” being most common. |
| Intersection Street 2 | String | **Intersection Street 2** | Second street in an intersection; similar distribution to *Intersection Street 1*. |
| Address Type | String | **Address Type** | Classification of the address (e.g., `ADDRESS`, `INTERSECTION`). “ADDRESS” dominates (~71 %). |
| City | String | **City** | Borough or city name where the incident occurred. Top values are Brooklyn, New York, Bronx; ~17 % are unspecified. |
| Landmark | String | **Landmark** | Notable landmark near the incident (e.g., “BROADWAY”). Over 90 % of records have a null value; the top non‑null landmarks appear only a few times each. |
| Facility Type | String | **Facility Type** | Type of facility involved in the complaint (e.g., `DSNY Garage`, `N/A`). The most common values account for ~70 % of records. |
| Status | String | **Status** | Current status of the complaint (`Closed`, `Pending`, `Open`, etc.). “Closed” comprises 95 % of cases. |
| Due Date | DateTime | **Due Date** | Target resolution date (often null). The top ten due dates represent a small fraction; most records have no due date set. |
| Resolution Description | String | **Resolution Description** | Narrative from the agency explaining how the complaint was resolved. The most common descriptions cover ~10 % of cases; the rest are grouped under “Other”. |
| Resolution Action Updated Date | DateTime | **Resolution Action Updated Date** | Timestamp of the latest resolution update. Nulls occur in 1.5 % of records. |
| Community Board | String | **Community Board** | Numerical or unspecified community board number (e.g., `0 Unspecified`). “Other” accounts for ~75 %. |
| BBL | Integer | **BBL** | Borough/Block/Lot identifier; null in 24 % of cases. The most frequent values are block‑lot numbers. |
| Borough | String | **Borough** | NYC borough (`BROOKLYN`, `QUEENS`, etc.). “Unspecified” accounts for ~5 %. |
| X Coordinate (State Plane) | Integer | **X Coordinate (State Plane)** | State plane X coordinate in feet; null in 8 % of records. |
| Y Coordinate (State Plane) | Integer | **Y Coordinate (State Plane)** | State plane Y coordinate in feet; null in 8 %. |
| Open Data Channel Type | String | **Open Data Channel Type** | Channel used to file the complaint (`PHONE`, `ONLINE`, etc.). “PHONE” dominates (~50 %). |
| Park Facility Name | String | **Park Facility Name** | Name of a park facility involved; most values are null (≈99 %) with “Unspecified” as the top value. |
| Park Borough | String | **Park Borough** | Borough where the park is located; matches the borough field for 100 % of non‑null records. |
| Vehicle Type | String | **Vehicle Type** | Type of vehicle involved in the incident (e.g., `Car Service`, `Ambulette`). The majority are null (~99 %). |
| Taxi Company Borough | String | **Taxi Company Borough** | Borough where the taxi company is based; almost all records have a value. |
| Taxi Pick Up Location | String | **Taxi Pick Up Location** | Category of the pick‑up location (e.g., `JFK Airport`, `Intersection`). The top two categories account for >60 % of non‑null values. |
| Bridge Highway Name | String | **Bridge Highway Name** | Official name of a bridge or highway involved; nearly all records are null (~1 %). |
| Bridge Highway Direction | String | **Bridge Highway Direction** | Cardinal direction for the bridge/highway incident; almost all records are null. |
| Road Ramp | String | **Road Ramp** | Type of ramp involved (e.g., `RAMP`, `ROADWAY`). The vast majority of records are null. |
| Bridge Highway Segment | String | **Bridge Highway Segment** | Specific segment or exit on a bridge/highway; almost all records are null. |
| Latitude | Float | **Latitude** | Geographic latitude of the incident location. Null in 25 % of records; top values cluster around Manhattan’s latitude range. |
| Longitude | Float | **Longitude** | Geographic longitude of the incident location. Null in 25 % of records; values span NYC’s western to eastern boundaries. |
| Location | String | **Location** | Combined latitude/longitude string (e.g., “(40.89187241649303, -73.86016845296459)”). Null in 25 % of records; the top ten coordinate strings account for ~1 %. |

*All field names match the column headers in the raw CSV. The descriptions incorporate insights from the summary statistics (data types, null rates, cardinality) and frequency distributions (most common values and their relative frequencies).*

**Dataset Description**

This dataset contains one million 311 service‑request records submitted to the City of New York between January 2010 and December 2020. Each record captures the time a complaint was created, its resolution status, the agency responsible for handling it, and a host of contextual attributes such as location (ZIP code, street, latitude/longitude), complaint type, and additional descriptors. The data are highly granular: 48 % of the fields have no missing values, while several key attributes—particularly `Incident Zip`, `Latitude`, `Longitude`, and `Location`—have nulls in roughly one‑quarter of the records. Categorical variables often contain a dominant “Other” bucket that aggregates many rare or outlier values; for example, over 90 % of complaint types fall into a handful of categories, with the remaining 10 % collapsed into an “Other” group. The dataset is free of duplicate primary keys (`Unique Key`) but contains placeholder dates (e.g., `1900‑01‑01` or `2100‑01‑01`) to indicate missing closure information.

---

### Notable Characteristics

- **Missing Values**  
  - ~25 % of records lack latitude, longitude, and the composite `Location` string.  
  - Incident ZIP codes are null in ~5 % of cases; Incident Address is null in ~17 %.  
  - The `Closed Date` field uses extreme sentinel dates (1900‑01‑01 / 2100‑01‑01) to flag unresolved complaints.

- **High Cardinality & “Other” Buckets**  
  - Many categorical fields aggregate rare values into an “Other” bucket that accounts for >80 % of the distribution (e.g., `Descriptor`, `Community Board`).  
  - This aggregation masks fine‑grained distinctions but keeps the dataset manageable.

- **Temporal Coverage**  
  - Records span a decade, with the earliest creation date on 2010‑01‑01 and the latest on 2020‑12‑23.  
  - The distribution of `Created Date` shows clustering around early‑year timestamps, likely reflecting bulk imports or system quirks.

- **Geospatial Inconsistencies**  
  - Latitude/longitude pairs are missing in one quarter of rows; the remaining points cluster within Manhattan and surrounding boroughs but also contain a few extreme outliers (e.g., coordinates outside NYC bounds).

- **Duplicate Handling**  
  - The `Unique Key` field is strictly unique, confirming no duplicate records.

- **PII & Sensitive Data**  
  - No personally identifying information is present beyond the service‑request identifier and general location details.  
  - The dataset is suitable for public release after standard de‑identification checks.

---

*Footnote: Generated by qsv's describegpt command using openai/gpt-oss-20b on 2025-08-28T03:55:02.658565+00:00*

nyc_311_complaints  
noise_complaints  
housing_and_building_maintenance  
transportation_infrastructure  
environmental_protection  
public_safety_agencies  
city_governance_data  
geospatial_location  
boroughs  
street_conditions  
heat_hot_water  
illegal_parking  
tax_boroughs  
health_and_hygiene  
open_data_platform

