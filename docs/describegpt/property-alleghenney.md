# Dictionary
| Name | Type | Label | Description | Content Type | Min | Max | Cardinality | Enumeration | Null Count | Examples |
|------|------|-------|-------------|--------------|-----|-----|-------------|-------------|------------|----------|
| **_id** | Integer | Record Identifier | A system‑generated integer that uniquely identifies each transaction record in the dataset. | unique_id | 97067785 | 97558047 | 490,263 |  | 0 | <ALL_UNIQUE> |
| **PARID** | String | Parcel Identifier | An alphanumeric code used by the county to reference a specific property parcel. It is shared across multiple records for properties that have more than one transaction. | unknown | 0001C00037000A00 | 9946X83943000000 | 307,189 |  | 0 | Other… [490,085]<br>0431B00017000000 [23]<br>0027D00263000000 [20]<br>0027D00272000000 [20]<br>0027D00286000000 [20] |
| **FULL_ADDRESS** | String | Full Property Address | The complete street address of the property, including house number, fraction/letter, direction, street name, suffix, unit descriptor and number, city, state abbreviation and ZIP code. | street_address | 0 , BRADDOCK, PA 15104 | MOON CLINTON RD, CORAOPOLIS, PA 15108 | 282,224 |  | 0 | Other… [489,345]<br>0 SONIE DR, SEWICKLEY, PA… [115]<br>0 COAL, ELIZABETH, PA 150… [111]<br>0 KELLY ST, PITTSBURGH, P… [100]<br>0 PERRYSVILLE AVE, PITTSB… [100] |
| **PROPERTYHOUSENUM** | Integer | House Number | The primary numeric component of the property address; part of the mailing address. Combine with PROPERTYFRACTION, PROPERTYADDRESSDIR, PROPERTYADDRESSSTREET, PROPERTYADDRESSSUF, PROPERTYUNITNO, PROPERTYCITY, PROPERTYSTATE and PROPERTYZIP to form the full address. | building_number | 0 | 39391 | 10,042 |  | 6 | Other… [437,741]<br>0 [39,001]<br>112 [1,657]<br>100 [1,640]<br>110 [1,555] |
| **PROPERTYFRACTION** | String | Address Fraction / Letter | An optional fraction or letter used to distinguish units on a property (e.g., "1/2", "A"); part of the mailing address. | secondary_address |   |  S | 2,826 |  | 0 | (NULL)… [478,523]<br>Other… [9,947]<br>1/2 [880]<br>A [412]<br>B [301] |
| **PROPERTYADDRESSDIR** | String | Street Direction Abbreviation | The cardinal direction abbreviation that precedes the street name (e.g., N, S, E, W); part of the mailing address. | free_text | E | W | 5 |  | 469,921 | (NULL)… [469,921]<br>N [5,577]<br>S [5,304]<br>E [5,168]<br>W [4,293] |
| **PROPERTYADDRESSSTREET** | String | Street Name | The name of the street on which the property is located; part of the mailing address. | street_name | 0 OHIO RIVER BLVD | ZUZU | 9,695 |  | 16 | Other… [472,322]<br>WASHINGTON [2,650]<br>5TH [2,581]<br>HIGHLAND [1,804]<br>LINCOLN [1,613] |
| **PROPERTYADDRESSSUF** | String | Street Suffix | The suffix that follows a street name (e.g., "ST", "AVE", "RD"); part of the mailing address. | free_text | ALY | XING | 47 |  | 2,032 | ST [125,045]<br>DR [115,422]<br>AVE [107,058]<br>RD [73,055]<br>LN [15,818] |
| **PROPERTYADDRESSUNITDESC** | String | Unit Description | A descriptor indicating the type of unit (e.g., "APT", "UNIT", "STE"); part of the mailing address. | secondary_address | APT | UNIT | 10 |  | 478,384 | (NULL)… [478,384]<br>UNIT [10,278]<br>APT [773]<br>REAR [428]<br>STE [210] |
| **PROPERTYUNITNO** | String | Unit Number | The numeric or alphanumeric identifier of a specific unit within the property; part of the mailing address. | building_number | 01 | ` | 1,342 |  | 478,756 | (NULL)… [478,756]<br>Other… [10,076]<br>1 [196]<br>2 [172]<br>3 [169] |
| **PROPERTYCITY** | String | City | The city in which the property is located; part of the mailing address. | city | 15216 | WITAKER | 106 |  | 2 | PITTSBURGH [262,846]<br>Other… [126,038]<br>CORAOPOLIS [17,051]<br>MC KEESPORT [15,638]<br>GIBSONIA [11,346] |
| **PROPERTYSTATE** | String | State Abbreviation | The two‑letter abbreviation for the state where the property resides. | state_abbr | PA | PA | 1 | PA | 0 | <ALL_UNIQUE> |
| **PROPERTYZIP** | Integer | Zip Code | The five‑digit ZIP code of the property's location; part of the mailing address. | zip_code | 15003 | 16229 | 124 |  | 1 | Other… [363,146]<br>15108 [17,065]<br>15237 [15,716]<br>15235 [14,917]<br>15212 [13,571] |
| **SCHOOLCODE** | String | School District Code | An internal numeric identifier for the school district serving the property. | unknown | 01 | 50 | 46 |  | 0 | Other… [233,052]<br>47 [120,421]<br>27 [20,142]<br>09 [19,600]<br>30 [18,034] |
| **SCHOOLDESC** | String | School District Name | The human‑readable name of the school district corresponding to SCHOOLCODE. | free_text | Allegheny Valley | Woodland Hills | 46 |  | 0 | Other… [233,052]<br>Pittsburgh [120,421]<br>North Allegheny [20,142]<br>Woodland Hills [19,600]<br>Penn Hills Twp [18,034] |
| **MUNICODE** | Integer | Municipality Code | An internal numeric code identifying the municipality (e.g., city, borough) of the property. | unknown | 101 | 953 | 175 |  | 0 | Other… [377,756]<br>934 [18,034]<br>119 [12,889]<br>940 [12,229]<br>926 [10,572] |
| **MUNIDESC** | String | Municipality Description | The name or description of the municipality corresponding to MUNICODE. | free_text | 10th Ward -  McKEESPORT | Wilmerding   | 175 |  | 0 | Other… [377,756]<br>Penn Hills [18,034]<br>19th Ward - PITTSBURGH [12,889]<br>Ross [12,229]<br>Mt.Lebanon [10,572] |
| **RECORDDATE** | Date | Record Date | The date on which the transaction record was entered into the system. Format is YYYY‑MM‑DD. | date:%Y-%m-%d | 0212-08-01 | 2026-04-28 | 3,886 |  | 1,252 | Other… [484,695]<br>(NULL)… [1,252]<br>2012-10-26 [587]<br>2013-04-26 [552]<br>2012-01-11 [488] |
| **SALEDATE** | Date | Sale Date | The date on which the property sale actually occurred. Format is YYYY‑MM‑DD. | date:%Y-%m-%d | 2012-01-01 | 2026-04-27 | 5,021 |  | 0 | Other… [485,732]<br>2012-10-26 [586]<br>2016-04-29 [583]<br>2013-04-26 [559]<br>2012-01-11 [490] |
| **PRICE** | Integer | Sale Price | The monetary amount (in dollars) received for the property at the time of sale. | unknown | 0 | 148752900 | 38,596 |  | 3,028 | Other… [344,667]<br>1 [101,391]<br>0 [15,560]<br>10 [7,046]<br>150000 [3,229] |
| **DEEDBOOK** | String | Deed Book Identifier | An alphanumeric code that identifies the book in which the deed record is filed. | free_text |  14795 | `17274 | 5,951 |  | 567 | Other… [483,122]<br>TR18 [1,233]<br>0 [1,078]<br>TR13 [938]<br>00 [797] |
| **DEEDPAGE** | String | Deed Page Number | The page number within the deed book where the transaction record appears. | free_text |  120 | W | 2,135 |  | 579 | Other… [475,030]<br>1 [5,273]<br>6 [1,511]<br>7 [1,142]<br>0 [1,016] |
| **SALECODE** | String | Sale Type Code | A short code indicating the nature of the sale (e.g., "3" for standard sale, "H" for homestead). | free_text | 0 | Z | 47 |  | 0 | 3 [100,702]<br>0 [94,707]<br>H [65,379]<br>Other… [55,622]<br>14 [43,389] |
| **SALEDESC** | String | Sale Type Description | Human‑readable description of the sale type corresponding to SALECODE. | free_text | BANK/FINANCIAL INSTITUTION | VALID SALE | 27 |  | 0 | LOVE AND AFFECTION SALE [100,702]<br>VALID SALE [94,707]<br>MULTI-PARCEL SALE [65,379]<br>Other… [47,027]<br>TIME ON MARKET (INSUFF/EX… [43,389] |
| **INSTRTYP** | String | Instruction Type Code | A code that indicates how the deed was recorded (e.g., "DE" for standard, "SW" for special warranty). | free_text | AD | WD | 31 |  | 0 | DE [284,729]<br>SW [116,588]<br>GW [23,966]<br>QC [18,681]<br>SD [15,783] |
| **INSTRTYPDESC** | String | Instruction Type Description | Human‑readable description of the instruction type corresponding to INSTRTYP. | free_text | ADMIN DEED | WILL BOOK | 31 |  | 0 | DEED [284,729]<br>SPECIAL WARRANTY [116,588]<br>GENERAL WARRANTY [23,966]<br>QUIT CLAIM [18,681]<br>SHERIFF DEED [15,783] |

*Attribution: Generated by qsv v21.1.0 describegpt
Command line: /Users/joelnatividad/GitHub/qsv/target/debug/qsv describegpt --dictionary --description property-alleghenney.csv --fresh -o property-alleghenney2.md --infer-content-type --two-pass
Prompt file: Default v7.2.0
Model: openai/gpt-oss-20b
LLM API URL: http://localhost:1234/v1
Language: 
Timestamp: 2026-07-06T13:45:10.530739+00:00

WARNING: Label, Description and Content Type generated by an LLM and may contain inaccuracies. Verify before using!
*

## REASONING

FIRST PASS REASONING:
We need to generate JSON with each field name key mapping to object: label, description, content_type, role, concept.

We must use summary statistics and frequency distribution for context.

Also identify relationships among fields: joint, ordered, correlated.

We also need grain string: one row = ??? probably "one row = one property sale transaction" or record of property sale.

Let's analyze each field:

1. _id: unique integer surrogate key; cardinality 490263 rows? Actually summary shows min 97067785 max 97558047 range 490262, sort_order Ascending. So unique. It's likely a primary key. Label: "Record ID" or "Unique Identifier". Content type: unknown because it's numeric but unique id; we could use uuid? But it's integer surrogate. The allowed content types include "unique_id" reserved for unique-key columns. But spec says do not use "unique_id" yourself; deterministic override will replace it if row count matches cardinality ratio 1. So we should not set content_type to unique_id. Instead choose most specific token: maybe "unknown". Role: identifier. Concept: id.surrogate_key.

2. PARID: String, likely property address ID? It's a string of digits and letters. Frequency shows many duplicates like 0431B00017000000 etc. Not unique. Content type: unknown (string). Could be "free_text" but maybe it's a parcel identifier. There is concept id.foreign_key perhaps referencing some other table; we can set content_type "unknown", role dimension, concept id.foreign_key.

3. FULL_ADDRESS: String containing full address including street number etc. Content type: street_address? Actually format like "0 SONIE DR, SEWICKLEY, PA 15143". That's a complete street address. So content_type: street_address. Role: dimension (categorical). Concept: geo.street_address.

4-10: components of address:
PROPERTYHOUSENUM: integer house number; but also includes zeros etc. Many zeros? The summary shows min 0 max 39391, many zeros. Content type building_number. Role dimension. Concept: geo.building_number.

PROPERTYFRACTION: string fraction like "1/2", "A". Could be part of address (apartment). Content type secondary_address? Actually it's a fractional or letter. Could use secondary_address. Role dimension. Concept: unknown maybe.

PROPERTYADDRESSDIR: direction abbreviation; content_type street_name? No, that's direction of street, so could be free_text but concept unknown. Might be "free_text" but we can set as street_name? Wait street_name is for name only like "Main". Direction is separate token not listed. So use free_text.

PROPERTYADDRESSSTREET: string like WASHINGTON etc; content_type street_name. Role dimension. Concept geo.street_name.

PROPERTYADDRESSSUF: suffix like ST, DR, AVE; content_type street_suffix? Not available; maybe free_text. Could use "street_address"? But that's for full address. So free_text.

PROPERTYADDRESSUNITDESC: unit description like APT, UNIT; could be secondary_address. Role dimension. Concept unknown.

PROPERTYUNITNO: unit number string or numeric; content_type building_number? Actually unit number separate from house number. Use building_number? Or maybe free_text. But we can use "building_number". Role dimension.

11-13: city, state, zip. Content types city, state_abbr, zip_code respectively. Roles dimension. Concepts geo.city, geo.state, geo.zip_code.

14-16: school code and desc, municipal code and desc. Schoolcode: numeric string; content_type unknown? Could be "free_text" but maybe treat as category. Role dimension. Concept unknown or category.type? But maybe use id.foreign_key referencing schools table. So concept id.foreign_key.

SCHOOLCODE: content_type free_text or unknown. Role dimension.

SCHOOLDESC: description of school; content_type free_text; role dimension.

MUNICODE: integer code for municipality; content_type unknown, role dimension, concept id.foreign_key.

MUNIDESC: description; free_text.

17-19: dates: RECORDDATE and SALEDATE; type Date. Frequency shows format "YYYY-MM-DD". So date:%Y-%m-%d.

Content types: date:%Y-%m-%d. Role timestamp (since date). Concept time.event_timestamp? Could be time.date. Since both are event timestamps but generic, use time.event_timestamp maybe.

20: PRICE: integer; content_type unknown? It's monetary value in cents? Actually price likely in dollars; range 0 to 148752900; typical property sale price up to $148M. So numeric measure. Content type unknown or maybe "unknown" because it's currency but not specified as currency_code. Use unknown.

Role: measure. Concept measure.amount.

21-22: DEEDBOOK and DEEDPAGE: strings with book/page numbers. Content_type free_text? Could be "file_name"? No, these are identifiers of deed location. So content_type free_text or unknown. Role dimension. Concept unknown maybe id.foreign_key referencing deeds.

23-24: SALECODE and SALEDESC: code and description; similar to school codes. content_type free_text. role dimension. concept category.type? Use category.type maybe for sale type.

25-26: INSTRTYP, INSTRTYPDESC: instruction type code and description. Content type free_text. Role dimension. Concept category.type or id.foreign_key.

Now relationships:

Joint: city+state+zip forms address; propertystreet + direction etc but we can treat as joint of FULL_ADDRESS? But FULL_ADDRESS already includes all parts. We could have joint group: PROPERTYHOUSENUM, PROPERTYFRACTION, PROPERTYADDRESSDIR, PROPERTYADDRESSSTREET, PROPERTYADDRESSSUF, PROPERTYUNITNO, PROPERTYCITY, PROPERTYSTATE, PROPERTYZIP to form full address components. That's a big group; but maybe simpler: FULL_ADDRESS is derived from those fields; but relationship? Could define joint among the components that combine into FULL_ADDRESS. However spec says relationships between independent fields not derived? But it's fine.

Also joint: SCHOOLCODE + SCHOOLDESC (and maybe MUNIDESC). Might be joint.

Ordered: RECORDDATE <= SALEDATE? Not necessarily; sale date might be after record date? Typically record date earlier or equal to sale date. So we can set ordered relationship between RECORDDATE and SALEDATE, anchor RECORDDATE.

Also maybe DEEDBOOK/DEEDPAGE ordering? Not monotonic.

Correlated: PRICE vs SALEDESC? Maybe not.

We could identify correlated numeric fields like PRICE and maybe nothing else.

Thus relationships: joint of address components; joint of schoolcode+desc; joint of muni code+desc. And ordered between RECORDDATE and SALEDATE.

Also perhaps correlation between DEEDBOOK/DEEDPAGE? Not numeric.

Let's craft JSON accordingly.

Need to list each field with label, description, content_type, role, concept.

We must ensure content_type token suffix for dates: date:%Y-%m-%d

Now produce output.

REFINE PASS REASONING:
We need to refine labels/descriptions and content types with cross-field context.

The dataset has fields: 26.

Goal: group components into composite concepts like mailing address, person name etc. For this dataset, we have property address components; sale dates; price; codes etc. We must produce for each field a refined label, description referencing other fields if part of composite concept, and content_type token (except unique_id). For _id field first-pass had content_type unique_id but rule says omit content_type key entirely from output. So we don't include content_type in _id.

Also we may adjust content types: e.g., PROPERTYHOUSENUM is building_number; ok. PROPERTYFRACTION maybe secondary_address or building_number? Already set as secondary_address. But maybe better as "secondary_address" fine. Others: PROPERTYADDRESSDIR (free_text) maybe address_direction? Not in vocabulary; but can keep free_text.

PROPERTYADDRESSSTREET: street_name.

PROPERTYADDRESSSUF: street_suffix but not token; free_text ok.

PROPERTYUNITNO: building_number? Actually unit number could be building_number or secondary_address. It's numeric/alphanumeric. Could remain building_number.

But composite concept "Mailing Address" includes full address and components. We can adjust labels to reflect that each is part of Mailing Address. For description we mention combine with others.

Also for date fields: RECORDDATE, SALEDATE content_type should be date (with format). Format: first-pass had date:%Y-%m-%d; we keep same token "date:%Y-%m-%d". But rule says we may add suffix; but we keep as is. So include content_type "date:%Y-%m-%d".

For PRICE: numeric amount in dollars. Could set content_type "unknown" or maybe "currency_code"? Actually it's a numeric price value, not currency code. There's no token for monetary amount. So unknown.

DEEDBOOK/DEEDPAGE free_text fine.

SALECODE etc. free_text ok.

INSTRTYP etc. free_text.

Also we might adjust PROPERTYFRACTION: could be secondary_address or building_number? But it's fraction like "1/2" or letter; maybe secondary_address is okay.

We may rename labels for clarity e.g., FULL_ADDRESS: "Full Property Address". Good.

Other fields: SCHOOLCODE, SCHOOLDESC etc. Keep.

MUNICODE, MUNIDESC.

Let's craft output JSON with keys same as field names? The example says:
{
  "field_name_1": { ... },
...
}

So we need each field name string key mapping to object with label, description, content_type (except _id). Provide only those fields. Use cross-field context.

Also ensure to not include cardinality etc. Just label, description, content_type.

We also need to consider that first-pass had "content_type" for many; we might keep or change. For unique_id: omit content_type entirely.

Let's produce.

Check each field:

1. _id: Omit content_type. Label: "Record Identifier". Description same as before but maybe mention system-generated integer that uniquely identifies transaction record. Good.

2. PARID: label stays Parcel Identifier; description fine; content_type unknown? Keep unknown.

3. FULL_ADDRESS: label refine to "Full Property Address"; description mentions full street address including house number, street name, direction, suffix, unit descriptor, city/state/zip. Content_type street_address (already). Good.

4. PROPERTYHOUSENUM: label maybe "House Number" or "Property House Number". Already fine; content_type building_number.

5. PROPERTYFRACTION: keep "Address Fraction / Letter", content_type secondary_address.

6. PROPERTYADDRESSDIR: "Street Direction Abbreviation"; description mention cardinal direction preceding street name; content_type free_text (fine).

7. PROPERTYADDRESSSTREET: label "Street Name" or "Property Street Name". Keep street_name.

8. PROPERTYADDRESSSUF: "Street Suffix"; keep free_text.

9. PROPERTYADDRESSUNITDESC: "Unit Description"; keep secondary_address.

10. PROPERTYUNITNO: "Unit Number"; content_type building_number maybe ok. Keep.

11. PROPERTYCITY: "City"; content_type city.

12. PROPERTYSTATE: "State Abbreviation"; content_type state_abbr.

13. PROPERTYZIP: "Zip Code"; content_type zip_code.

14. SCHOOLCODE: label "School District Code" fine; content_type unknown.

15. SCHOOLDESC: "School District Name"; content_type free_text.

16. MUNICODE: "Municipality Code"; content_type unknown.

17. MUNIDESC: "Municipality Description"; content_type free_text.

18. RECORDDATE: label "Record Date", description same; content_type date:%Y-%m-%d.

19. SALEDATE: "Sale Date" same; content_type date:%Y-%m-%d.

20. PRICE: "Sale Price" description fine; content_type unknown.

21. DEEDBOOK: "Deed Book Identifier"; content_type free_text.

22. DEEDPAGE: "Deed Page Number"; content_type free_text.

23. SALECODE: "Sale Type Code"; content_type free_text.

24. SALEDESC: "Sale Type Description"; content_type free_text.

25. INSTRTYP: "Instruction Type Code" ; content_type free_text.

26. INSTRTYPDESC: "Instruction Type Description"; content_type free_text.

Also cross-field relationships: In descriptions for components we can mention combine with others to form mailing address; e.g., PROPERTYHOUSENUM is part of mailing address; etc.

Let's craft each description accordingly.

Make sure not to include any extraneous keys. Provide JSON object mapping field names to objects.

Edge: For content_type, we need to output token exactly as allowed. Eg "street_address", "building_number" etc. Ensure no colon unless date/datetime. Good.

Ok produce final answer.
## TOKEN USAGE

TokenUsage { prompt: 15043, completion: 5959, total: 21002, elapsed: 81062, completion_tps: 73.51 }
---
# Description
**Description**

The dataset contains 490 263 property‑sale transaction records for properties located in Pennsylvania, collected from county records between August 2012 and April 2026. Each row is uniquely identified by an auto‑generated `_id`, while the same parcel (PARID) can appear multiple times, resulting in 307 189 distinct parcels. The address components together form a full street address with 282 224 unique values, but most sub‑address fields such as direction and unit number contain large “Other” or null groups, indicating inconsistent or missing data. Sale prices range from $0 to nearly $149 million; the average price is about $196 k, the distribution is strongly right‑skewed with the 90th percentile at $395 k and the top 5 % exceeding $560 k, while roughly 3 % of transactions report a zero price. Record dates span from August 2012 to April 2026, with about 0.25 % missing, whereas sale dates are complete for every row. Instruction and sale type codes show a dominant share of standard sales (code “3”) and standard deed instructions (“DE”), but many rare or other categories make up the remaining transactions.

**Notable Characteristics**

- **High cardinality & data quality issues**: 282 224 unique full addresses, yet 96 %+ of `PROPERTYADDRESSDIR` and 98 %+ of `PROPERTYUNITNO` are missing or grouped under “Other”, reflecting inconsistent sub‑address entries.  
- **Skewed price distribution**: The mean sale price (~$196k) is inflated by a small number of very high values; the median lies well below the mean, and 3 % of transactions have a zero price.  
- **Missing values**: 3028 rows (≈0.6 %) lack a `PRICE`; 1252 rows (≈0.25 %) miss a `RECORDDATE`. All `SALEDATE` entries are present.  
- **Duplicate parcel records**: The same PARID can appear many times, indicating multiple sales or events for a single property; the `_id` field guarantees row uniqueness.  
- **PII/PHI considerations**: Full street addresses and ZIP codes are included, so masking is required before public sharing to comply with privacy regulations.  
- **Dominant categorical values**: `INSTRTYP` (DE 58 %) and `SALECODE` (3 20 %) dominate; the “Other” categories in several fields represent a large share of records, suggesting possible classification gaps or data entry errors.  
- **Single state coverage**: All properties are located in PA, limiting geographic diversity.

Generated by qsv v21.1.0 describegpt
Command line: /Users/joelnatividad/GitHub/qsv/target/debug/qsv describegpt --dictionary --description property-alleghenney.csv --fresh -o property-alleghenney2.md --infer-content-type --two-pass
Prompt file: Default v7.2.0
Model: openai/gpt-oss-20b
LLM API URL: http://localhost:1234/v1
Language: 
Timestamp: 2026-07-06T13:45:43.070285+00:00

WARNING: Description generated by an LLM and may contain inaccuracies. Verify before using!



## REASONING

We need to produce a description of dataset based on summary stats, freq distribution, and data dictionary. The user wants a 1-8 sentence description (so between 1 and 8 sentences). Then "Notable Characteristics" bullet list. Use the information from stats.

Dataset: property sale transactions for Pennsylvania? It's a dataset of real estate sales with unique ID per record; multiple parcels can have many records; includes addresses, price, dates, etc. The cardinality: about 490k rows. Many fields have high uniqueness or low cardinality.

We need to highlight central tendency and spread: For _id sum min max range etc. But we should not output per field stats. Instead mention overall dataset features: 490k unique records; 307k distinct parcels, etc. Provide distribution insights: e.g., price distribution heavy skewed right (skewness high), mean ~911k? Actually from summary for PRICE: sum 95666916462, min 0, max 148752900, range 148752900. avg maybe 196346.56? Wait average of PRICE is 196346.56; median maybe? percentiles show 5th percentile 1, 10th 10, 40th 45000, 60th 140000, 90th 395000, 95th 561000. So distribution skewed right with many low values and some high ones. Also large number of zero prices (count 15560). Many missing price values (3028). Many records have nulls in address components etc.

Also note that there are many "Other" categories for categorical fields, meaning majority unique or unknown. For example, property address has 282k distinct addresses; but many 'Other' counts high. There is a high percentage of NULLs in some fields: e.g., PROPERTYADDRESSDIR null count 469921 (~96%); PROPERTYUNITNO null count 478756 (~98%); PROPERTYFRACTION no null? 0; etc.

Also mention that PROPERTYSTATE is always PA, so constant. The dataset includes PII like addresses, but not personally identifying beyond property location.

Also duplicates: _id unique; but PARID can have multiple records per parcel (duplicate across transactions). Also many 'Other' categories indicate data quality issues: e.g., property address field contains some weird formatting or incomplete addresses.

Also mention date fields: record date has 1252 nulls, sale date all present. Dates range 2012-2026; distribution concentrated around certain dates with peaks.

Also mention instruction types: majority DE (58%), SW 24%, etc.

Also note that there are outliers in price and other numeric fields like _id? Actually _id is unique integer, but min ~97M, max ~97558047. Not relevant.

Also missing values in some fields: e.g., PROPERTYADDRESSDIR, UNITNO, FRACTION has no nulls? Actually PARID none; FULL_ADDRESS none; etc.

Now produce 1-8 sentences description.

We must avoid per-field details, but can mention counts.

Let's craft:

Sentence 1: The dataset comprises 490,263 real‑estate transaction records for properties located in Pennsylvania collected from county records between 2012 and 2026. Sentence 2: Each record is uniquely identified by an auto‑generated _id while the same parcel (PARID) can appear multiple times, yielding 307,189 distinct parcels. Sentence 3: The address components together form a full street address with 282,224 unique values, but most fields such as direction and unit number contain large “Other” or null groups, indicating inconsistent or missing sub‑address data. Sentence 4: Sale prices range from $0 to nearly $149 million, with an average of about $196k and a right‑skewed distribution where the 90th percentile is $395k and the top 5 % exceed $560k; zero price transactions account for roughly 3 %. Sentence 5: Date fields span from August 2012 to April 2026, with record dates missing in about 0.25 % of rows while sale dates are complete. Sentence 6: Instruction and sale type codes show a dominant share of standard sales (code “3”) and standard deed instructions (“DE”), but there are many rare or other categories that account for the remaining transactions.

Maybe that's 6 sentences.

Now bullet list Notable Characteristics:

- High cardinality in address fields with many "Other" entries and missing sub‑address components.
- Strong right skewness in sale price distribution; a small number of very high values drive the mean above median.
- Presence of zero-price sales (~3 %) and 3028 missing prices (~0.6 %).
- Record date has 1252 nulls (~0.25 %); sale date fully populated.
- The dataset contains PII: full street addresses, ZIP codes; these should be masked if shared publicly.
- Duplicate records for the same parcel (PARID) reflect multiple transaction events; unique _id ensures row uniqueness.
- Instruction type and sale code distributions show dominant categories but many rare values; “Other” categories comprise large shares in some fields indicating possible data quality or missing classification.
- The dataset includes a single state (PA) so no cross‑state variation.

Add attribution placeholder.

Ensure markdown formatting: maybe use headings.

Let's produce final answer.
## TOKEN USAGE

TokenUsage { prompt: 11760, completion: 1694, total: 13454, elapsed: 32531, completion_tps: 52.07 }
---
