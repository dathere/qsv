---
type: "CSV Table"
title: "NYC 311 SR 2010 2020 sample 1M"
description: "**Description** The dataset contains one million New York City 311 complaint records spanning the period from January 1 2010 to December 23 2020. Each record is uniquely identified by an integer surrogate key that increases monotonically with time, and includes timestamps for when the complaint was created, closed (if applicable), and when resolution actions were last updated. The data capture a wide range of fields describing the incident location (ZIP code, street names, latitude/longitude, borough, community board, etc.), the nature of the complaint (type, descriptor, agency responsible, status, and resolution narrative), as well as administrative metadata such as the submission channel and due dates. The majority of complaints are closed (≈ 95 %) and most records lack a due date or a closed‑date value. While many categorical fields exhibit heavy skew—most notably “Complaint Type” and “Descriptor,” where an “Other” bucket accounts for more than half of the entries—the dataset also includes high‑cardinality numeric identifiers such as borough–block–lot (BBL) values and geospatial coordinates. **Notable Characteristics** - **Central tendency & spread** – Created dates cluster in 2013‑2015, with a median around late 2014; closed dates are similarly concentrated but have a broader range (up to ~73049 days). - **Distribution shape** – Categorical fields such as “Complaint Type” and “Descriptor” show extreme right‑skewness: the “Other” category captures >56 % and >67 % of observations, respectively. The “Status” field is highly imbalanced with “Closed” dominating (~95 %). - **Missing values** – Substantial nulls exist in fields like `Due Date` (≈ 65 % missing), `Incident Zip`, `City`, `Landmark`, and the geospatial coordinates (`Latitude`, `Longitude`). The presence of a large “Other (N)” bucket in many frequency distributions indicates many unique or low‑frequency values. - **Outliers & data quality** – Although most latitude/longitude pairs fall within NYC bounds, a few entries have extreme coordinate values that likely correspond to placeholder defaults (e.g., `-77.5195844` longitude). The dataset has no duplicate surrogate keys, ensuring record uniqueness. - **Potential privacy concerns** – While the data are not personally identifiable on their own, combining detailed address fields with precise geospatial coordinates can potentially reveal individual locations and should be handled with care in downstream analyses. - **Data consistency** – Certain textual fields contain inconsistent formatting (e.g., `Agency` codes vs. full names), mixed case, or non‑standard delimiters (`Cross Street 1`, `Intersection Street 2`). These inconsistencies may affect join operations and require standardization."
resource: "https://data.cityofnewyork.us/Social-Services/311-Service-Requests"
timestamp: "2020-12-31"
tags:
  - nyc_311_complaints
  - city_government_service_requests
  - geocoded_location_data
  - borough_zip_code_analysis
  - noise_and_light_abuse
  - infrastructure_and_transportation
  - parks_and_recreation_complaints
  - health_environment_issues
  - public_safety_incidents
  - municipal_agencies_involvement
---

# NYC 311 SR 2010 2020 sample 1M

**Description**

The dataset contains one million New York City 311 complaint records spanning the period from January 1 2010 to December 23 2020.  Each record is uniquely identified by an integer surrogate key that increases monotonically with time, and includes timestamps for when the complaint was created, closed (if applicable), and when resolution actions were last updated.  The data capture a wide range of fields describing the incident location (ZIP code, street names, latitude/longitude, borough, community board, etc.), the nature of the complaint (type, descriptor, agency responsible, status, and resolution narrative), as well as administrative metadata such as the submission channel and due dates.  The majority of complaints are closed (≈ 95 %) and most records lack a due date or a closed‑date value.  While many categorical fields exhibit heavy skew—most notably “Complaint Type” and “Descriptor,” where an “Other” bucket accounts for more than half of the entries—the dataset also includes high‑cardinality numeric identifiers such as borough–block–lot (BBL) values and geospatial coordinates.

**Notable Characteristics**

- **Central tendency & spread** – Created dates cluster in 2013‑2015, with a median around late 2014; closed dates are similarly concentrated but have a broader range (up to ~73049 days).  
- **Distribution shape** – Categorical fields such as “Complaint Type” and “Descriptor” show extreme right‑skewness: the “Other” category captures >56 % and >67 % of observations, respectively.  The “Status” field is highly imbalanced with “Closed” dominating (~95 %).  
- **Missing values** – Substantial nulls exist in fields like `Due Date` (≈ 65 % missing), `Incident Zip`, `City`, `Landmark`, and the geospatial coordinates (`Latitude`, `Longitude`).  The presence of a large “Other (N)” bucket in many frequency distributions indicates many unique or low‑frequency values.  
- **Outliers & data quality** – Although most latitude/longitude pairs fall within NYC bounds, a few entries have extreme coordinate values that likely correspond to placeholder defaults (e.g., `-77.5195844` longitude).  The dataset has no duplicate surrogate keys, ensuring record uniqueness.  
- **Potential privacy concerns** – While the data are not personally identifiable on their own, combining detailed address fields with precise geospatial coordinates can potentially reveal individual locations and should be handled with care in downstream analyses.  
- **Data consistency** – Certain textual fields contain inconsistent formatting (e.g., `Agency` codes vs. full names), mixed case, or non‑standard delimiters (`Cross Street 1`, `Intersection Street 2`).  These inconsistencies may affect join operations and require standardization.

# Schema

| Column | Type | Description |
| --- | --- | --- |
| `Unique Key` | integer | A surrogate numeric identifier that uniquely distinguishes each record in the dataset. It is an integer that increases with time but contains no inherent meaning beyond uniqueness. |
| `Created Date` | timestamp | The timestamp when the complaint record was created in the system. Values are stored as date‑time strings in the format "MM/DD/YYYY hh:mm:ss AM/PM". |
| `Closed Date` | timestamp | The timestamp when the complaint was closed or resolved. The format matches that of Created Date: "MM/DD/YYYY hh:mm:ss AM/PM". |
| `Agency` | text | A short code identifying the New York City agency responsible for handling the complaint (e.g., NYPD, HPD). |
| `Agency Name` | text | The full name of the agency that received or processed the complaint. |
| `Complaint Type` | text | High‑level category of the complaint (e.g., Noise, Illegal Parking). The most common types account for about half of all records. |
| `Descriptor` | text | A more detailed free‑text description that further specifies the nature of the complaint (e.g., Loud Music/Party, Street Light Out). |
| `Location Type` | text | The type of location where the complaint occurred (e.g., RESIDENTIAL BUILDING, STREET/SIDEWALK). |
| `Incident Zip` | text | The five‑digit ZIP code of the incident location. |
| `Incident Address` | text | A street address string indicating where the complaint was reported (often includes building number and street). |
| `Street Name` | text | The name of the main street on which the incident occurred. |
| `Cross Street 1` | text | The first cross‑street name at an intersection, if applicable. |
| `Cross Street 2` | text | The second cross‑street name at an intersection, if applicable. |
| `Intersection Street 1` | text | One street involved in the intersection where the incident occurred. |
| `Intersection Street 2` | text | The second street involved in the intersection where the incident occurred. |
| `Address Type` | text | Classification of how the address was recorded (e.g., ADDRESS, INTERSECTION). |
| `City` | text | The city or borough name in which the incident took place (e.g., BROOKLYN, NEW YORK). |
| `Landmark` | text | A notable nearby landmark or point of interest mentioned in the complaint. |
| `Facility Type` | text | The type of facility involved, such as DSNY Garage or School District. |
| `Status` | text | Current status of the complaint record (e.g., Closed, Pending). |
| `Due Date` | timestamp | The deadline date for resolving the complaint. Stored as a timestamp in "MM/DD/YYYY hh:mm:ss AM/PM" format. |
| `Resolution Description` | text | A narrative description of the actions taken to resolve the complaint. |
| `Resolution Action Updated Date` | timestamp | The most recent timestamp when the resolution action was updated. Format matches other date‑time fields. |
| `Community Board` | text | The NYC community board number or "Unspecified" if not applicable. |
| `BBL` | text | A numeric identifier representing the borough, block, and lot of a property in NYC. |
| `Borough` | text | The borough (e.g., BROOKLYN, QUEENS) where the incident occurred. |
| `X Coordinate (State Plane)` | integer | The easting coordinate in the New York State Plane coordinate system. |
| `Y Coordinate (State Plane)` | integer | The northing coordinate in the New York State Plane coordinate system. |
| `Open Data Channel Type` | text | The channel through which the complaint was submitted (e.g., PHONE, ONLINE). |
| `Park Facility Name` | text | Name of a park facility involved in the complaint, if any. |
| `Park Borough` | text | The borough where the park facility is located. |
| `Vehicle Type` | text | Type of vehicle involved, such as Car Service or Green Taxi. |
| `Taxi Company Borough` | text | The borough in which the taxi company operates. |
| `Taxi Pick Up Location` | text | Text description of where a taxi was picked up (e.g., JFK Airport, Intersection). |
| `Bridge Highway Name` | text | Name or designation of the bridge or highway involved in the complaint. |
| `Bridge Highway Direction` | text | The directionality of traffic on the bridge/highway (e.g., East/Long Island Bound). |
| `Road Ramp` | text | Indicates whether a ramp or roadway was involved. |
| `Bridge Highway Segment` | text | Specific segment of the bridge or highway (e.g., Exit 13). |
| `Latitude` | number | Geographic latitude coordinate in decimal degrees. |
| `Longitude` | number | Geographic longitude coordinate in decimal degrees. |
| `Location` | text | String representation of the latitude and longitude pair in the format "(lat, lon)". |

*Attribution: Generated by qsv v21.1.0 describegpt
Command line: target/debug/qsv describegpt NYC_311_SR_2010-2020-sample-1M.csv --all --format okf --ds-source https://data.cityofnewyork.us/Social-Services/311-Service-Requests --ds-updated 2020-12-31 --no-cache -o docs/describegpt/nyc311-describegpt-okf.md
Prompt file: Default v7.2.0
Model: openai/gpt-oss-20b
LLM API URL: http://localhost:1234/v1
Language: 
Timestamp: 2026-06-21T05:01:14.708504+00:00

WARNING: Label, Description and Content Type generated by an LLM and may contain inaccuracies. Verify before using!
*
