## Generated using the command:
```bash
$ QSV_LLM_BASE_URL=https://api.together.xyz/v1 QSV_LLM_APIKEY=THEKEY qsv describegpt \
     /tmp/NYC_311_SR_2010-2020-sample-1M.csv --all \
     --model openai/gpt-oss-120b
     --output nyc311-describegpt-openai-oss-120b.md
```

## Output (nyc311-describegpt-openai-oss-120b.md)

```json
[
  {
    "Name": "Unique Key",
    "Type": "Integer",
    "Label": "Unique Record Identifier",
    "Description": "A unique integer identifier for each record (1,000,000 distinct values). No missing values."
  },
  {
    "Name": "Created Date",
    "Type": "DateTime",
    "Label": "Record Creation Timestamp",
    "Description": "Date and time when the service request was created. Most common dates are in 2013‑2015; 99.5 % of values are covered by the top 20 dates, with the remaining 0.5 % listed as “Other”."
  },
  {
    "Name": "Closed Date",
    "Type": "DateTime",
    "Label": "Record Closure Timestamp",
    "Description": "Date and time when the request was closed. About 2.86 % of rows are null (still open). The top 20 closure dates account for ~96 % of non‑null values."
  },
  {
    "Name": "Agency",
    "Type": "String",
    "Label": "Agency Code",
    "Description": "Short code of the city agency handling the request. The field is categorical with 28 distinct values; NYPD (26.5 %) and HPD (25.8 %) dominate, while an “Other” bucket captures 0.014 % of records."
  },
  {
    "Name": "Agency Name",
    "Type": "String",
    "Label": "Agency Full Name",
    "Description": "Full name of the agency. 553 distinct values; the most common are "New York City Police Department" (26.5 %) and "Department of Housing Preservation and Development" (25.8 %)."
  },
  {
    "Name": "Complaint Type",
    "Type": "String",
    "Label": "Type of Complaint",
    "Description": "Categorical description of the complaint. 287 distinct categories; the most frequent is "Noise - Residential" (8.94 %). An "Other" bucket contains 38.47 % of records."
  },
  {
    "Name": "Descriptor",
    "Type": "String",
    "Label": "Complaint Descriptor",
    "Description": "More specific descriptor for the complaint. 1392 distinct values; the most common is "Loud Music/Party" (9.36 %). An "Other" bucket covers 56.54 % of rows."
  },
  {
    "Name": "Location Type",
    "Type": "String",
    "Label": "Location Category",
    "Description": "General classification of the location (e.g., RESIDENTIAL BUILDING, STREET). 28 distinct categories; the top two categories cover ~51 % of records, and an "Other" bucket accounts for 3.22 %."
  },
  {
    "Name": "Incident Zip",
    "Type": "String",
    "Label": "ZIP Code of Incident",
    "Description": "5‑digit ZIP code (or placeholder). 535 distinct values; 5.50 % of rows are null. The most common ZIPs are 11226 (1.71 %) and 10467 (1.45 %). An "Other" bucket captures 72.24 %."
  },
  {
    "Name": "Incident Address",
    "Type": "String",
    "Label": "Incident Street Address",
    "Description": "Street address where the incident occurred. 341,996 distinct values; 17.47 % are null. The most frequent specific address appears 0.154 % of the time; the rest are covered by an "Other" bucket."
  },
  {
    "Name": "Street Name",
    "Type": "String",
    "Label": "Street Name",
    "Description": "Name of the street. 14,817 distinct values; 17.47 % null. The most common street is "BROADWAY" (0.97 %). An "Other" bucket holds 76.45 % of values."
  },
  {
    "Name": "Cross Street 1",
    "Type": "String",
    "Label": "First Cross Street",
    "Description": "First intersecting street, if applicable. 16,218 distinct values; 32.04 % null. The most frequent cross‑street is "BEND" (1.26 %). An "Other" bucket covers 59.78 %."
  },
  {
    "Name": "Cross Street 2",
    "Type": "String",
    "Label": "Second Cross Street",
    "Description": "Second intersecting street, if applicable. 16,364 distinct values; 32.36 % null. "BEND" is again the most common (1.24 %). An "Other" bucket accounts for 60.28 %."
  },
  {
    "Name": "Intersection Street 1",
    "Type": "String",
    "Label": "Primary Intersection Street",
    "Description": "Primary street of an intersection. 76,742 distinct values; 76.74 % null. "BROADWAY" appears in 0.38 % of rows; an "Other" bucket captures 20.78 %."
  },
  {
    "Name": "Intersection Street 2",
    "Type": "String",
    "Label": "Secondary Intersection Street",
    "Description": "Secondary street of an intersection. 76,771 distinct values; 76.77 % null. "BROADWAY" is the top value (0.35 %). An "Other" bucket covers 20.78 %."
  },
  {
    "Name": "Address Type",
    "Type": "String",
    "Label": "Address Classification",
    "Description": "How the address is expressed (ADDRESS, INTERSECTION, etc.). 6 distinct values; the majority (71.04 %) are "ADDRESS"."
  },
  {
    "Name": "City",
    "Type": "String",
    "Label": "Borough‑Level City Name",
    "Description": "City or borough name. 382 distinct values; 6.20 % null. The most frequent city is "BROOKLYN" (29.63 %). An "Other" bucket accounts for 12.33 %."
  },
  {
    "Name": "Landmark",
    "Type": "String",
    "Label": "Nearby Landmark",
    "Description": "Landmark near the incident location. 5,915 distinct values; 91.28 % null. The most common landmark is "EAST  230 STREET" (0.154 %). An "Other" bucket captures 7.77 %."
  },
  {
    "Name": "Facility Type",
    "Type": "String",
    "Label": "Facility Category",
    "Description": "Type of public facility involved. 6 distinct values; 62.83 % are "N/A" and 19.33 % are "Precinct"."
  },
  {
    "Name": "Status",
    "Type": "String",
    "Label": "Request Status",
    "Description": "Current processing status. 10 distinct values; 95.25 % of records are "Closed"."
  },
  {
    "Name": "Due Date",
    "Type": "DateTime",
    "Label": "Resolution Due Date",
    "Description": "Target date for completing the request. 64.78 % null. The most common specific due dates appear only a few hundred times; the remaining 34.99 % are in an "Other" bucket."
  },
  {
    "Name": "Resolution Description",
    "Type": "String",
    "Label": "Resolution Narrative",
    "Description": "Textual description of how the request was resolved. 1,216 distinct values; the most common phrase (9.14 %) states that the police responded with no evidence of violation. An "Other" bucket captures 38.12 %."
  },
  {
    "Name": "Resolution Action Updated Date",
    "Type": "DateTime",
    "Label": "Resolution Update Timestamp",
    "Description": "Date when the resolution action was last updated. 1.51 % null. The remaining dates are spread across many specific values, with an "Other" bucket covering 98.02 %."
  },
  {
    "Name": "Community Board",
    "Type": "String",
    "Label": "Community Board Identifier",
    "Description": "Numeric code and borough for the community board. 77 distinct values; 4.99 % of rows are "0 Unspecified". An "Other" bucket contains 56.53 %."
  },
  {
    "Name": "BBL",
    "Type": "String",
    "Label": "Borough‑Block‑Lot Identifier",
    "Description": "Unique property identifier used by NYC. 268,363 distinct values; 24.30 % null. The most common BBL appears 0.16 % of the time; an "Other" bucket captures 74.80 %."
  },
  {
    "Name": "Borough",
    "Type": "String",
    "Label": "Borough Name",
    "Description": "One of NYC's five boroughs. 6 distinct values; the largest share is "BROOKLYN" (29.61 %)."
  },
  {
    "Name": "X Coordinate (State Plane)",
    "Type": "Integer",
    "Label": "State Plane X Coordinate",
    "Description": "Projected X coordinate in the New York State Plane coordinate system. 85,327 distinct values; 8.53 % null. The most common coordinate appears 0.16 % of the time; the rest are in an "Other" bucket."
  },
  {
    "Name": "Y Coordinate (State Plane)",
    "Type": "Integer",
    "Label": "State Plane Y Coordinate",
    "Description": "Projected Y coordinate in the New York State Plane coordinate system. Same cardinality pattern as the X coordinate."
  },
  {
    "Name": "Open Data Channel Type",
    "Type": "String",
    "Label": "Data Submission Channel",
    "Description": "Method used to submit the request (PHONE, UNKNOWN, ONLINE, MOBILE, OTHER). Five categories; PHONE accounts for 49.76 %."
  },
  {
    "Name": "Park Facility Name",
    "Type": "String",
    "Label": "Park Facility",
    "Description": "Name of a park or related facility. 1,889 distinct values; 99.31 % are "Unspecified". The most common named parks each appear less than 0.03 % of rows."
  },
  {
    "Name": "Park Borough",
    "Type": "String",
    "Label": "Borough of Park Facility",
    "Description": "Borough where the park facility is located. Same distribution as the general "Borough" field."
  },
  {
    "Name": "Vehicle Type",
    "Type": "String",
    "Label": "Vehicle Category",
    "Description": "Type of vehicle used for the request (e.g., Car Service, Ambulette). 5 distinct values; 99.97 % are null, indicating most requests are not vehicle‑specific."
  },
  {
    "Name": "Taxi Company Borough",
    "Type": "String",
    "Label": "Taxi Company Borough",
    "Description": "Borough of the taxi company handling the request. 11 distinct values; 99.92 % null."
  },
  {
    "Name": "Taxi Pick Up Location",
    "Type": "String",
    "Label": "Taxi Pickup Location",
    "Description": "Location where a taxi was requested. 12 distinct values; 99.21 % null. The most frequent non‑null values are generic (e.g., "Other", "JFK Airport")."
  },
  {
    "Name": "Bridge Highway Name",
    "Type": "String",
    "Label": "Bridge/Highway Name",
    "Description": "Name of the bridge or highway involved. 48 distinct values; 99.77 % null. The top named bridges each appear less than 0.03 % of rows."
  },
  {
    "Name": "Bridge Highway Direction",
    "Type": "String",
    "Label": "Bridge/Highway Travel Direction",
    "Description": "Direction of travel on the bridge/highway (e.g., Bronx Bound). 30 distinct values; 99.77 % null."
  },
  {
    "Name": "Road Ramp",
    "Type": "String",
    "Label": "Road Ramp Indicator",
    "Description": "Indicates whether the location is a ramp, roadway, or N/A. 4 distinct values; 99.77 % null."
  },
  {
    "Name": "Bridge Highway Segment",
    "Type": "String",
    "Label": "Bridge/Highway Segment",
    "Description": "Specific segment of a bridge or highway. 20 distinct values; 99.76 % null."
  },
  {
    "Name": "Latitude",
    "Type": "Float",
    "Label": "Geographic Latitude",
    "Description": "Latitude in decimal degrees. 353,674 distinct values; 25.47 % null. The most frequent latitude appears 0.154 % of the time; the rest are in an "Other" bucket."
  },
  {
    "Name": "Longitude",
    "Type": "Float",
    "Label": "Geographic Longitude",
    "Description": "Longitude in decimal degrees. Same distribution as Latitude."
  },
  {
    "Name": "Location",
    "Type": "String",
    "Label": "Geographic Coordinate Pair",
    "Description": "Combined latitude/longitude string in the format "(lat, lon)". 375,752 distinct values; 25.47 % null. The most common coordinate pair appears 0.154 % of the time."
  }
]
```


## REASONING

We need to generate a Data Dictionary JSON with each field: Name, Type, Label, Description. Use Summary Statistics "type" column for Type. Need to craft human-friendly labels and descriptions using frequency distribution insights.

We have many fields. Probably include all fields listed in summary stats. That's many (~50). Provide reasonable label and description.

We'll iterate fields: Unique Key, Created Date, Closed Date, Agency, Agency Name, Complaint Type, Descriptor, Location Type, Incident Zip, Incident Address, Street Name, Cross Street 1, Cross Street 2, Intersection Street 1, Intersection Street 2, Address Type, City, Landmark, Facility Type, Status, Due Date, Resolution Description, Resolution Action Updated Date, Community Board, BBL, Borough, X Coordinate (State Plane), Y Coordinate (State Plane), Open Data Channel Type, Park Facility Name, Park Borough, Vehicle Type, Taxi Company Borough, Taxi Pick Up Location, Bridge Highway Name, Bridge Highway Direction, Road Ramp, Bridge Highway Segment, Latitude, Longitude, Location.

We need to produce JSON array of objects.

Label: human-friendly, maybe separate words.

Description: combine type, typical values, cardinality, most frequent values.

We have cardinality from summary stats: cardinality column. Use that.

Will generate concise descriptions.

Let's craft.


**Description**  
The dataset comprises one million NYC 311 service requests recorded between 2010‑01‑01 and 2020‑12‑23, each identified by a unique integer key.  Records are timestamped with creation dates that cluster around the early‑to‑mid‑2010s, while roughly 2.9 % of rows lack a closure date, indicating still‑open cases.  The two dominant agencies are NYPD (≈26 %) and HPD (≈26 %), and the most frequent complaint types are *Noise – Residential* (≈9 %) and *Heat/Hot Water* (≈5 %).  Geographic information is extensive but highly sparse: over 25 % of rows miss latitude/longitude, and address‑related fields (e.g., Incident Address, Street Name) contain many nulls and an overwhelming number of unique values.  Categorical fields often contain an “Other” bucket that captures the majority of low‑frequency categories (e.g., 38 % of Complaint Types, 56 % of Descriptors).  The data include potentially sensitive location details (street addresses and coordinates), which should be handled as personally identifiable information.  

**Notable Characteristics**
- **Size & Scope:** 1,000,000 records spanning a 10‑year period, with a single‑column primary key (`Unique Key`).  
- **Temporal Coverage:** Creation dates are concentrated in 2013‑2015; ~99.5 % of dates fall within the top 20 values, while the rest are grouped under “Other”.  
- **Agency & Complaint Distribution:** NYPD and HPD together account for > 50 % of records; noise‑related complaints dominate the top categories.  
- **Missing Data:** Closed Date (2.86 % null), Due Date (64.78 % null), Incident Address (17.47 % null), Street Name (17.47 % null), and location coordinates (≈25 % null).  
- **High Cardinality & “Other” Buckets:** Fields such as `Incident Address`, `BBL`, `Descriptor`, and many street‑level attributes have > 70 % of their values captured only by an “Other” category, indicating a long tail of rare values.  
- **Skewed Distributions:** Most categorical fields are heavily right‑skewed, with a few values accounting for the majority of rows and a long tail of rare entries.  
- **Potential PII:** Exact street addresses and latitude/longitude pairs could identify individuals or households; appropriate anonymization or access controls are recommended.  
- **Data Quality Concerns:** High sparsity, numerous nulls, and extensive “Other” categories may affect modeling and require careful preprocessing (e.g., imputation, grouping of rare categories).  

---  
Generated by qsv's describegpt command using openai/gpt-oss-120b on 2025-10-20T00:35:43.160460+00:00


## REASONING

We need to generate a description of the dataset as a whole, using summary stats and frequency distribution. Should be 1-8 sentences. Then "Notable Characteristics" bullet list. Then footnote with placeholder.

We must not output per-field details, just overall.

We need to infer dataset: appears to be NYC 311 Service Requests (311 complaints) with fields like Unique Key, Created Date, Closed Date, Agency, Complaint Type, etc. 1,000,000 records. Many categorical fields with high cardinality, many nulls, "Other" buckets.

We should mention size, temporal coverage (2010-2020), main agencies (NYPD, HPD), most common complaint types (Noise Residential, Heat/Hot Water), many missing values in address fields, location info, etc. Note high cardinality fields like Unique Key, Incident Address, BBL, etc. Note that many fields have "Other" dominating (e.g., Complaint Type has 38% other). Many nulls: Closed Date 2.86% null (open), Due Date 64.8% null, etc. Also many fields have high "Other" indicating many unique values.

Missing values percentages, skewness. Outliers: unique key values huge range 11M-48M, but that's okay.

Potential PII: address fields contain street addresses, could be considered PII. Also location coordinates.

Data quality issues: high sparsity, many nulls, many categorical fields with many low-frequency categories.

Write description 4-5 sentences.

Bullet list of notable characteristics: size 1M rows, time span 2010-2020, top agencies, top complaint types, large proportion of missing values in date fields, address fields high cardinality and many nulls, coordinates present but many missing, "Other" categories dominate many fields, skewed distributions, possible PII.

Add footnote.

Now produce markdown.
```json
[
  "nyc_311_service_requests",
  "public_agency_complaints",
  "noise_complaints",
  "housing_and_maintenance_issues",
  "transportation_and_road_issues",
  "borough_geolocation",
  "complaint_resolution",
  "request_status",
  "timestamp_analysis",
  "location_type",
  "open_data_submission_channels",
  "zip_code_distribution",
  "street_address_data",
  "incident_coordinates",
  "civic_engagement"
]
```


## REASONING

We need to generate up to 15 tags (keywords) describing the dataset, using lowercase and underscores, no field names. Tags should reflect thematic content: NYC 311 service requests, complaints, agencies, location, etc. We must think step by step.

Identify main themes:

- NYC 311 service requests / complaints
- public agencies (NYPD, HPD, DOT, etc.)
- complaint types (noise, heat, illegal parking)
- geographic info (borough, zip, coordinates)
- status (closed, open)
- resolution descriptions
- data source (311, open data)
- time series (created date)
- location types (residential building, street)
- open data channel (phone, online)

Select up to 15 tags, ordered by importance descending.

Potential tags:
1. nyc_311_service_requests
2. public_agency_complaints
3. noise_complaints
4. housing_and_maintenance_issues
5. transportation_and_road_issues
6. borough_geolocation
7. complaint_resolution
8. request_status
9. timestamps
10. location_type
11. open_data_channels
12. zip_code_analysis
13. street_address_data
14. incident_coordinates
15. civic_engagement

Make sure no field names appear (like "status" maybe okay? It's generic). Use underscores.

Return JSON array.


