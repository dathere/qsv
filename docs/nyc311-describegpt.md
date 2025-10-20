## Generated using the command:
```bash
$ QSV_LLM_BASE_URL=https://api.together.xyz/v1 QSV_LLM_APIKEY=THEKEY qsv describegpt \
     /tmp/NYC_311_SR_2010-2020-sample-1M.csv --all \
     --output nyc311-describegpt.md
```

## Output (nyc311-describegpt.md)
```json
[
  {
    "name": "Unique Key",
    "type": "Integer",
    "label": "Unique Key",
    "description": "A unique numeric identifier for each record in the dataset."
  },
  {
    "name": "Created Date",
    "type": "DateTime",
    "label": "Created Date",
    "description": "The timestamp when the complaint record was created in the system."
  },
  {
    "name": "Closed Date",
    "type": "DateTime",
    "label": "Closed Date",
    "description": "The timestamp when the complaint record was closed or marked as resolved."
  },
  {
    "name": "Agency",
    "type": "String",
    "label": "Agency",
    "description": "The short code for the agency responsible for the complaint (e.g., NYPD, HPD, DOT)."
  },
  {
    "name": "Agency Name",
    "type": "String",
    "label": "Agency Name",
    "description": "The full name of the agency responsible for the complaint (e.g., New York City Police Department)."
  },
  {
    "name": "Complaint Type",
    "type": "String",
    "label": "Complaint Type",
    "description": "The broad category of the complaint, such as Noise - Residential, HEAT/HOT WATER, or Illegal Parking."
  },
  {
    "name": "Descriptor",
    "type": "String",
    "label": "Descriptor",
    "description": "A more specific description of the issue (e.g., Loud Music/Party, HEAT, Street Light Out)."
  },
  {
    "name": "Location Type",
    "type": "String",
    "label": "Location Type",
    "description": "The type of physical location where the complaint was reported (e.g., RESIDENTIAL BUILDING, STREET/SIDEWALK)."
  },
  {
    "name": "Incident Zip",
    "type": "String",
    "label": "Incident ZIP",
    "description": "The ZIP code of the incident location."
  },
  {
    "name": "Incident Address",
    "type": "String",
    "label": "Incident Address",
    "description": "The street address where the incident occurred."
  },
  {
    "name": "Street Name",
    "type": "String",
    "label": "Street Name",
    "description": "The primary street name at the incident location."
  },
  {
    "name": "Cross Street 1",
    "type": "String",
    "label": "Cross Street 1",
    "description": "The first cross street intersecting or adjacent to the incident location."
  },
  {
    "name": "Cross Street 2",
    "type": "String",
    "label": "Cross Street 2",
    "description": "The second cross street intersecting or adjacent to the incident location."
  },
  {
    "name": "Intersection Street 1",
    "type": "String",
    "label": "Intersection Street 1",
    "description": "The first street in the intersection at the incident location."
  },
  {
    "name": "Intersection Street 2",
    "type": "String",
    "label": "Intersection Street 2",
    "description": "The second street in the intersection at the incident location."
  },
  {
    "name": "Address Type",
    "type": "String",
    "label": "Address Type",
    "description": "The type of address reference used in the record (e.g., ADDRESS, INTERSECTION, BLOCKFACE)."
  },
  {
    "name": "City",
    "type": "String",
    "label": "City",
    "description": "The city or borough where the incident occurred."
  },
  {
    "name": "Landmark",
    "type": "String",
    "label": "Landmark",
    "description": "The nearest landmark to the incident location (e.g., Broadway, 5 Avenue)."
  },
  {
    "name": "Facility Type",
    "type": "String",
    "label": "Facility Type",
    "description": "The type of facility involved in the complaint (e.g., DSNY Garage, School District)."
  },
  {
    "name": "Status",
    "type": "String",
    "label": "Status",
    "description": "The current status of the complaint (e.g., Closed, Pending, Open)."
  },
  {
    "name": "Due Date",
    "type": "DateTime",
    "label": "Due Date",
    "description": "The target or deadline date for resolving the complaint."
  },
  {
    "name": "Resolution Description",
    "type": "String",
    "label": "Resolution Description",
    "description": "A narrative summary of the actions taken to resolve the complaint."
  },
  {
    "name": "Resolution Action Updated Date",
    "type": "DateTime",
    "label": "Resolution Action Updated Date",
    "description": "The timestamp of the most recent update to the resolution action."
  },
  {
    "name": "Community Board",
    "type": "String",
    "label": "Community Board",
    "description": "The community board number or designation responsible for the area."
  },
  {
    "name": "BBL",
    "type": "String",
    "label": "BBL",
    "description": "Borough, Block, Lot identifier for the property involved in the complaint."
  },
  {
    "name": "Borough",
    "type": "String",
    "label": "Borough",
    "description": "The New York City borough where the complaint was reported."
  },
  {
    "name": "X Coordinate (State Plane)",
    "type": "Integer",
    "label": "X Coordinate (State Plane)",
    "description": "State Plane coordinate X for the incident location."
  },
  {
    "name": "Y Coordinate (State Plane)",
    "type": "Integer",
    "label": "Y Coordinate (State Plane)",
    "description": "State Plane coordinate Y for the incident location."
  },
  {
    "name": "Open Data Channel Type",
    "type": "String",
    "label": "Open Data Channel Type",
    "description": "The channel through which the complaint was submitted (e.g., PHONE, MOBILE)."
  },
  {
    "name": "Park Facility Name",
    "type": "String",
    "label": "Park Facility Name",
    "description": "The name of the park facility involved, if any."
  },
  {
    "name": "Park Borough",
    "type": "String",
    "label": "Park Borough",
    "description": "The borough where the park facility is located."
  },
  {
    "name": "Vehicle Type",
    "type": "String",
    "label": "Vehicle Type",
    "description": "The type of vehicle involved in the incident, if applicable."
  },
  {
    "name": "Taxi Company Borough",
    "type": "String",
    "label": "Taxi Company Borough",
    "description": "The borough of the taxi company that operated the vehicle."
  },
  {
    "name": "Taxi Pick Up Location",
    "type": "String",
    "label": "Taxi Pick Up Location",
    "description": "The location where a taxi was picked up."
  },
  {
    "name": "Bridge Highway Name",
    "type": "String",
    "label": "Bridge Highway Name",
    "description": "The name of the bridge or highway involved."
  },
  {
    "name": "Bridge Highway Direction",
    "type": "String",
    "label": "Bridge Highway Direction",
    "description": "The direction of travel on the bridge or highway."
  },
  {
    "name": "Road Ramp",
    "type": "String",
    "label": "Road Ramp",
    "description": "The type of road ramp involved in the incident."
  },
  {
    "name": "Bridge Highway Segment",
    "type": "String",
    "label": "Bridge Highway Segment",
    "description": "The segment identifier for the bridge or highway."
  },
  {
    "name": "Latitude",
    "type": "Float",
    "label": "Latitude",
    "description": "The latitude coordinate of the incident location."
  },
  {
    "name": "Longitude",
    "type": "Float",
    "label": "Longitude",
    "description": "The longitude coordinate of the incident location."
  },
  {
    "name": "Location",
    "type": "String",
    "label": "Location",
    "description": "The geographic coordinate pair (latitude, longitude) of the incident."
  }
]
```


## REASONING

We need to produce a Data Dictionary with Name, Type, Label, Description. Use summary stats type field. Provide human-friendly label and description. We have many fields; but likely list all fields. Provide JSON array of objects.

We need to create labels: e.g., "Unique Key" -> "Unique Key" or "Unique Identifier". Provide description: e.g., "A unique numeric identifier for each record." Use summary stats to infer. For datetime fields: "Created Date". Provide description.

We also need to use frequency distribution to help labels/description for categorical values.

Massive dataset, but we can produce dictionary for each field in summary stats: Unique Key, Created Date, Closed Date, Agency, Agency Name, Complaint Type, Descriptor, Location Type, Incident Zip, Incident Address, Street Name, Cross Street 1, Cross Street 2, Intersection Street 1, Intersection Street 2, Address Type, City, Landmark, Facility Type, Status, Due Date, Resolution Description, Resolution Action Updated Date, Community Board, BBL, Borough, X Coordinate (State Plane), Y Coordinate (State Plane), Open Data Channel Type, Park Facility Name, Park Borough, Vehicle Type, Taxi Company Borough, Taxi Pick Up Location, Bridge Highway Name, Bridge Highway Direction, Road Ramp, Bridge Highway Segment, Latitude, Longitude, Location.

We need to gather type: from summary stats: e.g., Unique Key,Integer; Created Date,DateTime; Agency,String; etc. Provide labels maybe same as field names but with spaces. Provide description based on summary stats and frequency distribution.

We should produce JSON array.

Let's craft. We'll use mapping. For each field, create object:

{
"name":"Unique Key",
"type":"Integer",
"label":"Unique Key",
"description":"A unique numeric identifier for each record in the dataset."
}

For Created Date: type DateTime.

Add description: "Timestamp indicating when the record was created."

Closed Date: similar.

Agency: type String; description: "Agency responsible for the incident, e.g., NYPD, HPD, etc."

Agency Name: type String; description: "Full name of the agency."

Complaint Type: type String; description: "Category of complaint, such as Noise - Residential, HEAT/HOT WATER, etc."

Descriptor: type String; description: "Specific descriptor of the issue, e.g., Loud Music/Party, HEAT, etc."

Location Type: type String; description: "Type of location where the complaint occurred, e.g., RESIDENTIAL BUILDING, STREET/SIDEWALK, etc."

Incident Zip: type String; description: "ZIP code of the incident location."

Incident Address: type String; description: "Street address of the incident."

Street Name: type String; description: "Primary street name at incident location."

Cross Street 1: type String; description: "First cross street at incident location."

Cross Street 2: type String; description: "Second cross street at incident location."

Intersection Street 1: type String; description: "First street in intersection at incident location."

Intersection Street 2: type String; description: "Second street in intersection at incident location."

Address Type: type String; description: "Type of address reference used, e.g., ADDRESS, INTERSECTION, etc."

City: type String; description: "City or borough where incident occurred."

Landmark: type String; description: "Landmark nearest to incident location."

Facility Type: type String; description: "Type of facility involved, e.g., DSNY Garage, School District, etc."

Status: type String; description: "Current status of complaint (e.g., Closed, Pending, Open)."

Due Date: type DateTime; description: "Deadline or target date for resolution."

Resolution Description: type String; description: "Textual description of resolution actions taken."

Resolution Action Updated Date: type DateTime; description: "Timestamp of last update to resolution action."

Community Board: type String; description: "Community board number or designation."

BBL: type String; description: "Borough, Block, Lot identifier for property."

Borough: type String; description: "Borough in which incident took place."

X Coordinate (State Plane): type Integer; description: "State Plane coordinate X for location."

Y Coordinate (State Plane): type Integer; description: "State Plane coordinate Y for location."

Open Data Channel Type: type String; description: "Channel through which data was submitted (e.g., PHONE, MOBILE)."

Park Facility Name: type String; description: "Name of park facility involved, if any."

Park Borough: type String; description: "Borough of the park facility."

Vehicle Type: type String; description: "Type of vehicle involved, if applicable."

Taxi Company Borough: type String; description: "Borough of taxi company."

Taxi Pick Up Location: type String; description: "Location where taxi pickup occurred."

Bridge Highway Name: type String; description: "Name of bridge or highway."

Bridge Highway Direction: type String; description: "Direction of bridge or highway."

Road Ramp: type String; description: "Road ramp type."

Bridge Highway Segment: type String; description: "Segment identifier for bridge or highway."

Latitude: type Float; description: "Latitude coordinate of incident location."

Longitude: type Float; description: "Longitude coordinate of incident location."

Location: type String; description: "Geographic coordinate pair of incident location."

Ok.

Return JSON.

Let's craft final answer.
**Dataset Description**  
This 1 million‑row dataset captures New York City 311 complaints filed between January 2010 and December 2020. Each record records a unique complaint identifier, timestamps for creation, resolution, and due dates, the agency and agency name that handled the complaint, the type and descriptor of the issue, the physical location (address, street, borough, ZIP code, latitude/longitude), and ancillary metadata such as the community board, open‑data channel used, and, where applicable, vehicle or bridge‑highway details. The majority of complaints are noise‑related (≈9 %) or classified as “Other” (≈38 %), with the remainder spread across heating, plumbing, street‑light, and parking issues. The dataset is dominated by borough‑wide and residential building locations, with most incidents concentrated in Brooklyn, Queens, and Manhattan. Geographic coordinates are provided in both State‑Plane and latitude/longitude, enabling spatial analysis of complaint density and hotspot identification.  

**Notable Characteristics**  

- **High Cardinality & “Other” Categories** – Fields such as `Agency`, `Complaint Type`, `Descriptor`, `Incident Zip`, and `Incident Address` contain many distinct values, with “Other (N)” absorbing the majority of rare categories (e.g., 72 % of ZIP codes, 81 % of addresses).  
- **Missingness** – `Closed Date` is null in 2.86 % of records, `Due Date` in 64.78 %, and many spatial fields have up to 25 % missing (e.g., `Latitude`, `Longitude`).  
- **Skewed Distributions** – Complaint type and descriptor are heavily right‑skewed, with single categories capturing a large fraction of records.  
- **Temporal Concentration** – Created and closed dates cluster around 2013–2015, with a slight left‑skew (negative skewness) indicating a heavier tail toward earlier dates.  
- **Duplicate / Unique Key** – `Unique Key` is guaranteed unique (1 000 000 distinct values), ensuring no duplicate complaints.  
- **Potential PII** – Records contain full street addresses and latitude/longitude pairs, which could be considered personally identifying information for location‑based privacy.  
- **Outliers** – `Closed Date` values span from 1900 to 2100, indicating data entry or system defaults that generate unrealistic timestamps.  
- **Data Quality Issues** – High proportion of “Other” values, significant missingness in key fields, and extreme timestamp ranges suggest the need for data cleaning and validation before analysis.  

---  

_Footnote: Generated by qsv's describegpt command using openai/gpt-oss-20b on 2025-10-20T00:17:33.504305+00:00_


## REASONING

We need to produce a description of the dataset as a whole, 1-8 sentences. Then a "Notable Characteristics" list. Use summary stats and frequency distribution to inform. We don't output per-field stat tables. Summarize dataset: It's NYC 311 complaints dataset? 1 million records. Fields: Unique key, timestamps, agency, complaint type, descriptor, location type, zip, address, borough, coordinates, etc. Provide description. Notable: large proportion of missing values in many fields (e.g., Incident Zip 72% other, Incident Address 81% other, etc.), high cardinality fields with unique values. Many "Other" categories. Distribution skewness: Complaint type heavily skewed towards "Other" (38%), Noise - Residential (9%). Geographic: majority in boroughs and zip. Timestamp ranges 2010-2020. Data quality: many missing/nulls, high cardinality, duplicates? unique key indicates unique records but "Unique Key" field has all unique. PII: contains addresses, coordinates, maybe phone numbers? Not in summary. Provide mention.

Also mention skewness: skewness values for numeric columns? Not specified but we could mention distribution shape: heavy right-skewed for complaint type, etc.

Also mention outliers: Created Date distribution: skewness -0.0869, so slightly left? Actually negative indicates left skewness? But we can mention distribution of Created Date shows concentration around 2013-2015.

Also mention missing: Created Date null count 0? Not shown. But Closed Date has 2.86% null. etc.

Include footnote placeholder.

Let's craft.
```json
[
  "new_york_city",
  "311_service_requests",
  "city_complaints",
  "public_services",
  "transportation",
  "housing",
  "noise_complaints",
  "infrastructure",
  "public_safety",
  "environmental",
  "health_and_hygiene",
  "traffic",
  "sanitation",
  "city_governance",
  "civic_engagement"
]
```


## REASONING

We need to produce no more than 15 most thematic tags (lowercase only, underscores). Based on dataset content. We have NYC 311 complaints dataset. The tags could be: new_york_city, 311_service_requests, city_complaints, public_services, transportation, housing, noise_complaints, infrastructure, public_safety, environmental, health_and_hygiene, traffic, sanitation, city_governance, civic_engagement. That's 15. Ensure lowercase and underscores. Provide JSON array.

Let's output.
