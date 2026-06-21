---
type: "CSV Table"
title: "NYC 311 SR 2010 2020 sample 1M"
description: "This NYC 311 complaint dataset contains 1 000 000 records, each uniquely identified by a system‑generated integer key."
resource: "https://data.cityofnewyork.us/Social-Services/311-Service-Requests"
timestamp: "2020-12-31"
tags:
  - nyc_311_complaints
  - public_service_complaint
  - geospatial_data
  - borough_and_zip_code
  - agency_responsibility
  - status_tracking
  - date_time_records
  - open_data_submission
  - noise_plumbing_heating
  - complaint_types
---

# NYC 311 SR 2010 2020 sample 1M

**Description**

This NYC 311 complaint dataset contains 1 000 000 records, each uniquely identified by a system‑generated integer key. The complaints were logged between January 2010 and December 2020, with most incident dates concentrated in the first half of that decade. About 12 % of the records lack a closure date, while the remaining 88 % span from 1900 to 2100. Agency responsibility is heavily skewed toward NYPD (≈26 %) and HPD (≈26 %), with the rest distributed among DOT, DSNY, DEP, and other city agencies. Complaint types are dominated by a single “Other” category that accounts for more than half of all entries; the next most common categories—Noise‑Residential, Heat/Hot Water, Illegal Parking—collectively represent roughly 20 %. Geographic coordinates are provided in both decimal degrees and New York State Plane units, with latitude values between 40.11° and 40.91° and longitude values from –77.52° to –73.70°. Many address‑related fields (e.g., Incident Address, Street Name) contain a high proportion of unique or “Other” entries, reflecting the free‑text nature of the source data.

---

### Notable Characteristics

- **Cardinality & Sparsity**  
  *Unique Key* is truly unique (1 000 000 distinct values).  
  Several fields exhibit extreme sparsity: *Closed Date* has ~2.9 % null, *Resolution Description* and *Vehicle Type* are almost entirely missing (~20 % and >99 % null respectively).  
  *Incident Zip*, *City*, and *Borough* show moderate cardinality (535, 382, 6), but most values cluster in a few ZIPs (e.g., 11226) and boroughs (Brooklyn, Queens).

- **Distribution Shape**  
  - Created/Closed dates are heavily right‑skewed; the top ten dates account for ~10 % of all complaints.  
  - *Complaint Type* and *Descriptor* have long tails with a dominant “Other” bucket (>50 %).  
  - *Agency* distribution is bimodal, split roughly between police (NYPD/HPD) and transportation/environmental agencies.

- **Outliers & Extremes**  
  Date fields contain sentinel extremes: *Closed Date* ranges from 01‑01‑1900 to 01‑01‑2100.  
  Coordinate fields have a few outlier values outside the expected NYC bounds (e.g., latitude > 41°, longitude < –78°), likely due to data entry errors.

- **Missing Values & Data Quality**  
  - High missingness in *Resolution Description* (~2 % null) and *Vehicle Type* (>99 %) limits analytical depth for those dimensions.  
  - The free‑text nature of *Incident Address*, *Landmark*, and *Taxi Pick Up Location* leads to a large “Other” category, hindering reliable categorization without NLP preprocessing.

- **Privacy & PII**  
  While no explicit personal identifiers are present, addresses and geographic coordinates can potentially be used to re‑identify individuals or sensitive locations. Users should apply masking or aggregation when publishing derived insights.

- **Duplicates & Uniqueness**  
  No duplicate *Unique Key* values exist; however, many records share identical timestamps or address fields, indicating possible multiple complaints for the same incident.

# Schema

| Column | Type | Content Type | Description |
| --- | --- | --- | --- |
| `Unique Key` | integer | unique_id | A system-generated unique identifier for each complaint record. |
| `Created Date` | timestamp | date:%m/%d/%Y | The date and time when the complaint was first logged in the system. |
| `Closed Date` | timestamp | date:%m/%d/%Y | The date and time when the complaint was closed or resolved. |
| `Agency` | text | category | The short code identifying the agency that is responsible for handling the complaint. |
| `Agency Name` | text | unknown | The full name of the agency responsible for addressing the complaint. |
| `Complaint Type` | text | category | The primary category describing the nature of the complaint (e.g., Noise, Plumbing). |
| `Descriptor` | text | category | A more detailed description or sub‑type of the complaint within its primary category. |
| `Location Type` | text | category | The type of location where the complaint was reported (e.g., Residential Building, Street). |
| `Incident Zip` | text | zip_code | The five‑digit ZIP code of the incident location. |
| `Incident Address` | text | street_address | A free‑text street address where the complaint was reported. |
| `Street Name` | text | street_name | The primary street name associated with the incident location. |
| `Cross Street 1` | text | street_name | First cross street intersecting at or near the incident location. |
| `Cross Street 2` | text | street_name | Second cross street intersecting at or near the incident location, if applicable. |
| `Intersection Street 1` | text | street_name | One of the streets forming an intersection at the incident location. |
| `Intersection Street 2` | text | street_name | The other street forming an intersection at the incident location, if applicable. |
| `Address Type` | text | category | The type of address used to locate the complaint (e.g., ADDRESS, INTERSECTION). |
| `City` | text | city | The city or borough in which the incident occurred. |
| `Landmark` | text | free_text | A notable nearby landmark referenced in the complaint record. |
| `Facility Type` | text | category | The type of facility involved or affected (e.g., DSNY Garage, School District). |
| `Status` | text | category | Current status of the complaint (e.g., Closed, Pending, Open). |
| `Due Date` | timestamp | datetime:%m/%d/%Y %I:%M:%S %p | The deadline by which the complaint should be resolved. |
| `Resolution Description` | text | free_text | Free‑text narrative describing the actions taken to resolve or investigate the complaint. |
| `Resolution Action Updated Date` | timestamp | date:%m/%d/%Y | The most recent date and time when the resolution action was updated. |
| `Community Board` | text | category | The community board number responsible for the area where the incident occurred. |
| `BBL` | text | unknown | Borough‑Block‑Lot identifier used by NYC planning and tax maps. |
| `Borough` | text | unknown | The borough in which the incident took place (e.g., BROOKLYN, MANHATTAN). |
| `X Coordinate (State Plane)` | integer | unknown | The X coordinate of the incident location in New York State Plane coordinates. |
| `Y Coordinate (State Plane)` | integer | unknown | The Y coordinate of the incident location in New York State Plane coordinates. |
| `Open Data Channel Type` | text | category | The channel through which the complaint was submitted (e.g., PHONE, ONLINE). |
| `Park Facility Name` | text | free_text | Name of a park facility involved in the complaint if applicable. |
| `Park Borough` | text | unknown | The borough where the referenced park facility is located. |
| `Vehicle Type` | text | category | Type of vehicle associated with the complaint (e.g., Car Service, Green Taxi). |
| `Taxi Company Borough` | text | unknown | The borough where the taxi company is registered. |
| `Taxi Pick Up Location` | text | free_text | Free‑text description of the location from which a taxi was picked up. |
| `Bridge Highway Name` | text | category | Name or designation of a bridge or highway involved in the complaint. |
| `Bridge Highway Direction` | text | category | The direction of travel for the bridge or highway (e.g., East/Long Island Bound). |
| `Road Ramp` | text | category | Type of road ramp referenced in the complaint. |
| `Bridge Highway Segment` | text | free_text | Specific segment or exit number on a bridge or highway, if applicable. |
| `Latitude` | number | latitude | Geographic latitude of the incident location in decimal degrees. |
| `Longitude` | number | longitude | Geographic longitude of the incident location in decimal degrees. |
| `Location` | text | unknown | String representation of the geographic coordinate pair for the incident location. |

*Attribution: Generated by qsv v21.1.0 describegpt
Command line: ./target/debug/qsv describegpt NYC_311_SR_2010-2020-sample-1M.csv --all --format okf --model openai/gpt-oss-20b --ds-source https://data.cityofnewyork.us/Social-Services/311-Service-Requests --ds-updated 2020-12-31 -o docs/describegpt/nyc311-describegpt-okf.md
Prompt file: Default v7.2.0
Model: openai/gpt-oss-20b
LLM API URL: http://localhost:1234/v1
Language: 
Timestamp: 2026-06-21T05:51:16.836505+00:00

WARNING: Label, Description and Content Type generated by an LLM and may contain inaccuracies. Verify before using!
*
