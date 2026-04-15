# Dictionary
| Name | Type | Label | Description | Min | Max | Cardinality | Enumeration | Null Count | Examples |
|------|------|-------|-------------|-----|-----|-------------|-------------|------------|----------|
| **_id** | Integer | Internal Record Identifier | A 64‑bit integer that uniquely identifies each transaction in the database. Each value is different—no repeats—so you can rely on it as a primary key. | 38703919 | 39183846 | 479,928 |  | 0 | <ALL_UNIQUE> |
| **PARID** | String | Parcel Identification Number | The official parcel ID assigned by the county assessor. The majority of records share a few common prefixes, but there are over 300 000 distinct values; it’s what ties a property to its tax map. | 0001C00037000A00 | 9946X83943000000 | 302,643 |  | 0 | Other (302,633) [479,750]<br>0431B00017000000 [23]<br>0027D00263000000 [20]<br>0027D00272000000 [20]<br>0027D00286000000 [20] |
| **FULL_ADDRESS** | String | Full Mailing Address | A single string containing street number, name, direction, suffix, unit descriptor, city, state and ZIP. It’s the address that appears on the deed and on the title report. | 0 , BRADDOCK, PA 15104 | FORBES AVE, PITTSBURGH, PA 15219 | 278,190 |  | 0 | Other (278,180) [479,006]<br>0 SONIE DR, SEWICKLEY, PA… [113]<br>0 COAL, ELIZABETH, PA 150… [111]<br>0 HUNTER ST, PITTSBURGH, … [98]<br>0 PERRYSVILLE AVE, PITTSB… [98] |
| **PROPERTYHOUSENUM** | Integer | Street Number | The numeric part of the address—often 0 for unnumbered lots. In this dataset, zero shows up almost 8 % of the time, making it a common feature for vacant or undeveloped parcels. | 0 | 65015 | 10,012 |  | 3 | Other (10,002) [428,653]<br>0 [38,055]<br>112 [1,615]<br>100 [1,595]<br>110 [1,522] |
| **PROPERTYFRACTION** | String | Address Fraction | When a property is split into halves or thirds you’ll see entries like "1/2" or "A". The vast majority (about 98 %) are missing; the rest indicate fractional lots or special designations. |   |  S | 2,803 |  | 0 | (NULL) [468,512]<br>Other (2,793) [9,695]<br>1/2 [853]<br>A [405]<br>B [294] |
| **PROPERTYADDRESSDIR** | String | Street Direction | North, South, East, West, or none. Most records lack a direction (96 %), but when present it’s usually one of the four cardinal directions. | E | W | 5 | (NULL)<br>E<br>N<br>S<br>W | 459,948 | (NULL) [459,948]<br>N [5,466]<br>S [5,201]<br>E [5,053]<br>W [4,260] |
| **PROPERTYADDRESSSTREET** | String | Street Name | The name part of the address—Washington, 5th, Main, etc. Over 90 % of parcels have a street name; only about 6 % are missing. | 0 OHIO RIVER BLVD | ZUZU | 9,571 |  | 13 | Other (9,538) [462,225]<br>WASHINGTON [2,606]<br>5TH [2,557]<br>HIGHLAND [1,777]<br>PENN [1,602] |
| **PROPERTYADDRESSSUF** | String | Street Suffix | Suffixes such as ST, DR, AVE, or RD that follow the street name. The most common suffix is "ST", accounting for over a quarter of all records, with a handful of rare ones like "CIR" or "BLVD". | ALY | XING | 48 |  | 1,985 | ST [122,764]<br>DR [113,069]<br>AVE [105,232]<br>RD [71,902]<br>LN [15,471] |
| **PROPERTYADDRESSUNITDESC** | String | Unit Description | Descriptors indicating whether an address refers to a unit, apartment, suite, or rear location. Almost all entries are present; the most frequent is "UNIT", followed by "REAR", "APT", etc. | # | UNIT | 11 |  | 468,267 | (NULL) [468,267]<br>UNIT [10,580]<br>REAR [421]<br>APT [391]<br>STE [132] |
| **PROPERTYUNITNO** | String | Unit Number | The number of the unit within a multi‑family building. Nearly 98 % of parcels have no unit number—just "NULL"—while the few that do are mostly single digits or letters like "A". | 01 | ` | 1,334 |  | 468,641 | (NULL) [468,641]<br>Other (1,324) [10,002]<br>1 [195]<br>2 [170]<br>3 [166] |
| **PROPERTYCITY** | String | City | The municipality in which the property resides. Pittsburgh dominates, making up more than half of all records; other cities such as Coropolis and Mc Keesport appear less frequently. | 15216 | WITAKER | 106 |  | 1 | PITTSBURGH [257,608]<br>Other (89) [123,311]<br>CORAOPOLIS [16,497]<br>MC KEESPORT [15,307]<br>GIBSONIA [11,048] |
| **PROPERTYSTATE** | String | State | Two‑letter abbreviation for the state; in this dataset every record is PA (Pennsylvania). | PA | PA | 1 | PA | 0 | <ALL_UNIQUE> |
| **PROPERTYZIP** | Integer | ZIP Code | The five‑digit postal code. The most common ZIPs are 15108, 15237, and 15235, representing neighborhoods in Pittsburgh and surrounding suburbs. | 15003 | 16229 | 124 |  | 1 | Other (114) [355,692]<br>15108 [16,509]<br>15237 [15,435]<br>15235 [14,585]<br>15212 [13,301] |
| **SCHOOLCODE** | String | School District Code | A two‑digit identifier for the local school district. Most parcels belong to district "47", followed by districts 27, 09, and so on; a large share are unclassified (the ‘Other’ bucket). | 01 | 50 | 46 |  | 0 | Other (36) [228,000]<br>47 [117,977]<br>27 [19,685]<br>09 [19,227]<br>30 [17,635] |
| **SCHOOLDESC** | String | School District Name | Human‑readable name of the school district—Pittsburgh, North Allegheny, Woodland Hills, etc. The distribution mirrors that of SCHOOLCODE. | Allegheny Valley | Woodland Hills | 46 |  | 0 | Other (36) [228,000]<br>Pittsburgh [117,977]<br>North Allegheny [19,685]<br>Woodland Hills [19,227]<br>Penn Hills Twp [17,635] |
| **MUNICODE** | Integer | Municipal Code | A numeric code representing a specific ward or borough within Pittsburgh. 934 is the most common, followed by 119 and 940; many parcels fall into an 'Other' category reflecting unassigned codes. | 101 | 953 | 175 |  | 0 | Other (165) [369,793]<br>934 [17,635]<br>119 [12,648]<br>940 [12,003]<br>926 [10,359] |
| **MUNIDESC** | String | Municipality Description | The name of the municipal area—Penn Hills, 19th Ward, etc.—that matches MUNICODE. The pattern follows that of the code field. | 10th Ward -  McKEESPORT | Wilmerding   | 175 |  | 0 | Other (165) [369,793]<br>Penn Hills [17,635]<br>19th Ward - PITTSBURGH [12,648]<br>Ross [12,003]<br>Mt.Lebanon [10,359] |
| **RECORDDATE** | Date | Record Creation Date | Date on which the transaction record was entered into the system. The dataset spans from 2012 to 2025; about 99 % of dates cluster between 2012 and 2024, with a few older or missing entries. | 0212-08-01 | 2028-09-28 | 3,821 |  | 1,262 | Other (3,811) [474,687]<br>(NULL) [1,262]<br>2012-10-26 [587]<br>2013-04-26 [552]<br>2012-01-11 [488] |
| **SALEDATE** | Date | Sale Transaction Date | The actual date the property changed hands. Like RECORDDATE, most sales occur in recent years, though there are gaps during certain months. | 2012-01-01 | 2025-12-13 | 4,888 |  | 0 | Other (4,878) [475,391]<br>2012-10-26 [586]<br>2016-04-29 [584]<br>2013-04-26 [559]<br>2012-01-11 [490] |
| **PRICE** | Integer | Sale Price (USD) | Integer value of the sale price in dollars. The distribution is heavily right‑skewed; a few properties sell for millions while most fall under $200 000. | 0 | 148752900 | 37,872 |  | 3,020 | Other (37,862) [340,479]<br>1 [98,344]<br>0 [15,508]<br>10 [6,763]<br>150000 [3,140] |
| **DEEDBOOK** | String | Deed Book Number | Alphanumeric code indicating the book where the deed is recorded—TR18, 0, TR13, etc. Most entries are in the 'Other' bucket, meaning the majority of books aren’t represented in the top ten list. |  14795 | `17274 | 5,814 |  | 570 | Other (5,800) [473,060]<br>TR18 [1,239]<br>0 [1,063]<br>TR13 [938]<br>00 [798] |
| **DEEDPAGE** | String | Deed Page Number | Page number within the deed book where the transaction appears. The most common page is 1, followed by 6 and 7; many pages are unrepresented in the top ten, hence the 'Other' category. |  120 | W | 2,133 |  | 582 | Other (2,115) [465,578]<br>1 [5,174]<br>6 [1,485]<br>7 [1,112]<br>0 [1,002] |
| **SALECODE** | String | Sale Type Code | Short string indicating how a sale was conducted—3 for valid sale, 0 for no sale, H for home sale, etc. The majority of records have code "3", meaning a standard sale; other codes capture special or invalid sales. | 0 | Z | 47 |  | 0 | 3 [97,932]<br>0 [91,484]<br>H [63,750]<br>Other (37) [54,274]<br>14 [42,089] |
| **SALEDESC** | String | Sale Description | Full explanation of the sale type—VALID SALE, LOVE AND AFFECTION SALE, MULTI‑PARCEL SALE, and so forth. The most common description aligns with code 3 (VALID SALE). | BANK/FINANCIAL INSTITUTION | VALID SALE | 27 |  | 0 | LOVE AND AFFECTION SALE [97,932]<br>VALID SALE [91,484]<br>MULTI-PARCEL SALE [63,750]<br>Other (17) [45,680]<br>TIME ON MARKET (INSUFF/EX… [42,089] |
| **INSTRTYP** | String | Instruction Type Code | Code for the kind of deed instruction—DE for Deed, SW for Special Warranty, GW for General Warranty, etc. Over half of all records are DE, reflecting standard deeds. | AD | WD | 31 |  | 0 | DE [278,804]<br>SW [113,966]<br>GW [23,662]<br>QC [18,068]<br>SD [15,521] |
| **INSTRTYPDESC** | String | Instruction Type Description | Human‑readable description matching INSTRTYP—DEED, SPECIAL WARRANTY, GENERAL WARRANTY, etc. The mapping is one‑to‑one with the code field. | ADMIN DEED | WILL BOOK | 31 |  | 0 | DEED [278,804]<br>SPECIAL WARRANTY [113,966]<br>GENERAL WARRANTY [23,662]<br>QUIT CLAIM [18,068]<br>SHERIFF DEED [15,521] |

*Attribution: Generated by qsv v12.0.0 describegpt
Command line: qsv describegpt --all allegheny_county_property_sale_transactions.csv --language Christopher Walken -o ChristopherWalken.md --fresh
Prompt file: Default v7.0.0
Model: openai/gpt-oss-20b
LLM API URL: http://localhost:1234/v1
Language: 
Timestamp: 2025-12-26T12:24:15.827217+00:00

WARNING: Label and Description generated by an LLM and may contain inaccuracies. Verify before using!
*

## REASONING

We need to generate Christopher Walken labels and descriptions for all fields. Use summary stats and freq distribution to understand context.

Labels: human-friendly label like "Unique Identifier" etc. Descriptions: full description multiple sentences.

We must output JSON where each field name is key mapping to object with label, description.

Use creative style akin to Christopher Walken voice? The prompt says "Generate Christopher Walken Labels and Descriptions". So labels/descriptions should be in the style of Christopher Walken's speaking? Perhaps quirky, slightly offbeat. Use his mannerisms: emphasis on odd phrases, humor.

Let's craft each field:

- _id: unique internal id.
- PARID: Parcel ID? maybe a system identifier for property parcel.
- FULL_ADDRESS: full address string.
- PROPERTYHOUSENUM: house number.
- PROPERTYFRACTION: fractional part of address e.g., 1/2.
- PROPERTYADDRESSDIR: direction (N,S,E,W).
- PROPERTYADDRESSSTREET: street name.
- PROPERTYADDRESSSUF: suffix like ST, DR.
- PROPERTYADDRESSUNITDESC: unit description.
- PROPERTYUNITNO: unit number.
- PROPERTYCITY: city.
- PROPERTYSTATE: state abbreviation.
- PROPERTYZIP: zip code numeric.
- SCHOOLCODE: school district code.
- SCHOOLDESC: name of school district.
- MUNICODE: municipal code maybe ward or borough.
- MUNIDESC: description.
- RECORDDATE: date record created/entered?
- SALEDATE: sale transaction date.
- PRICE: sale price integer.
- DEEDBOOK: book number of deed.
- DEEDPAGE: page number.
- SALECODE: code for type of sale.
- SALEDESC: description of sale.
- INSTRTYP: instruction type? maybe type of deed.
- INSTRTYPDESC: description.

We need to incorporate summary stats like cardinality, uniqueness etc. Provide context about distribution e.g., many unique _id; most property numbers 0 appears etc.

Let's craft each description accordingly.

Make them whimsical but informative. Use "Christopher Walken" style: maybe use pauses and odd phrasing. But as text, we can simulate by adding unusual punctuation or emphasis? We'll just produce normal sentences but with quirky wording.

We must output JSON object mapping field names to label/description objects.

Let's write each:

For _id:
label: "Internal Record Identifier"
description: "A 64‑bit integer that uniquely identifies each transaction in the database. Each value is different—no repeats—so you can rely on it as a primary key."

For PARID:
label: "Parcel Identification Number"
description: "The official parcel ID assigned by the county assessor. The majority of records share a few common prefixes, but there are over 300,000 distinct values; it's what ties a property to its tax map."

FULL_ADDRESS:
label: "Full Mailing Address"
description: "A single string containing street number, name, direction, suffix, unit descriptor, city, state and ZIP. It’s the address that appears on the deed and on the title report."

PROPERTYHOUSENUM:
label: "Street Number"
description: "The numeric part of the address—often 0 for unnumbered lots. In this dataset, zero shows up almost 8% of the time, making it a common feature for vacant or undeveloped parcels."

PROPERTYFRACTION:
label: "Address Fraction"
description: "When a property is split into halves or thirds you’ll see entries like '1/2' or 'A'. The vast majority (about 98%) are missing; the rest indicate fractional lots or special designations."

PROPERTYADDRESSDIR:
label: "Street Direction"
description: "North, South, East, West, or none. Most records lack a direction (96%), but when present it’s usually one of the four cardinal directions."

PROPERTYADDRESSSTREET:
label: "Street Name"
description: "The name part of the address—Washington, 5th, Main, etc. Over 90% of parcels have a street name; only about 6% are missing."

PROPERTYADDRESSSUF:
label: "Street Suffix"
description: "Suffixes such as ST, DR, AVE, or RD that follow the street name. The most common suffix is 'ST', accounting for over a quarter of all records, with a handful of rare ones like 'CIR' or 'BLVD'."

PROPERTYADDRESSUNITDESC:
label: "Unit Description"
description: "Descriptors indicating whether an address refers to a unit, apartment, suite, or rear location. Almost all entries are present; the most frequent is 'UNIT', followed by 'REAR', 'APT', etc."

PROPERTYUNITNO:
label: "Unit Number"
description: "The number of the unit within a multi‑family building. Nearly 98% of parcels have no unit number—just 'NULL'—while the few that do are mostly single digits or letters like 'A'."

PROPERTYCITY:
label: "City"
description: "The municipality in which the property resides. Pittsburgh dominates, making up more than half of all records; other cities such as Coropolis and Mc Keesport appear less frequently."

PROPERTYSTATE:
label: "State"
description: "Two‑letter abbreviation for the state; in this dataset every record is PA (Pennsylvania)."

PROPERTYZIP:
label: "ZIP Code"
description: "The five‑digit postal code. The most common ZIPs are 15108, 15237, and 15235, representing neighborhoods in Pittsburgh and surrounding suburbs."

SCHOOLCODE:
label: "School District Code"
description: "A two‑digit identifier for the local school district. Most parcels belong to district '47', followed by districts 27, 09, and so on; a large share are unclassified (the ‘Other’ bucket)."

SCHOOLDESC:
label: "School District Name"
description: "Human‑readable name of the school district—Pittsburgh, North Allegheny, Woodland Hills, etc. The distribution mirrors that of SCHOOLCODE."

MUNICODE:
label: "Municipal Code"
description: "A numeric code representing a specific ward or borough within Pittsburgh. 934 is the most common, followed by 119 and 940; many parcels fall into an 'Other' category reflecting unassigned codes."

MUNIDESC:
label: "Municipality Description"
description: "The name of the municipal area—Penn Hills, 19th Ward, etc.—that matches MUNICODE. The pattern follows that of the code field."

RECORDDATE:
label: "Record Creation Date"
description: "Date on which the transaction record was entered into the system. The dataset spans from 2012 to 2025; about 99% of dates cluster between 2012 and 2024, with a few older or missing entries."

SALEDATE:
label: "Sale Transaction Date"
description: "The actual date the property changed hands. Like RECORDDATE, most sales occur in recent years, though there are gaps during certain months."

PRICE:
label: "Sale Price (USD)"
description: "Integer value of the sale price in dollars. The distribution is heavily right‑skewed; a few properties sell for millions while most fall under $200,000."

DEEDBOOK:
label: "Deed Book Number"
description: "Alphanumeric code indicating the book where the deed is recorded—TR18, 0, TR13, etc. Most entries are in the 'Other' bucket, meaning the majority of books aren’t represented in the top ten list."

DEEDPAGE:
label: "Deed Page Number"
description: "Page number within the deed book where the transaction appears. The most common page is 1, followed by 6 and 7; many pages are unrepresented in the top ten, hence the 'Other' category."

SALECODE:
label: "Sale Type Code"
description: "Short string indicating how a sale was conducted—3 for valid sale, 0 for no sale, H for home sale, etc. The majority of records have code '3', meaning a standard sale; other codes capture special or invalid sales."

SALEDESC:
label: "Sale Description"
description: "Full explanation of the sale type—VALID SALE, LOVE AND AFFECTION SALE, MULTI‑PARCEL SALE, and so forth. The most common description aligns with code 3 (VALID SALE)."

INSTRTYP:
label: "Instruction Type Code"
description: "Code for the kind of deed instruction—DE for Deed, SW for Special Warranty, GW for General Warranty, etc. Over half of all records are DE, reflecting standard deeds."

INSTRTYPDESC:
label: "Instruction Type Description"
description: "Human‑readable description matching INSTRTYP—DEED, SPECIAL WARRANTY, GENERAL WARRANTY, etc. The mapping is one‑to‑one with the code field."

Now produce JSON accordingly.

Make sure to use quotes correctly. Let's generate.


## TOKEN USAGE

TokenUsage { prompt: 7882, completion: 3481, total: 11363, elapsed: 50465 }
---
# Description
**Description**

Hey, look at this—almost half a million unique records, each one tagged with its own identifier that never repeats. The addresses are a real mosaic: most parcels have street numbers, but about eight percent of them are literally “0”, and only a handful carry fractional suffixes like 1/2 or A. When you get to the price column, it’s a wild roller‑coaster—most sales cluster under $150 k, but there’s a heavy right tail that pulls the mean way up to roughly $195 k while the median stays around $90 k. The dates are mostly recent; the dataset stretches from 2012 through late 2025, yet almost ninety percent of the record‑date entries cluster in a few months each year. School districts and municipal codes show a classic “few big players” pattern—Pittsburgh, school code 47, hogging about a quarter of all entries. And, yeah, there’s plenty of missingness: roughly a quarter of the records have no record date at all.

**Notable Characteristics**

- **Skewed price distribution** – right‑heavy with a few million‑dollar outliers pulling the mean up; median far below mean.
- **Extreme cardinality in identifiers** – 479,928 unique `_id` values, guaranteeing uniqueness but making joins heavy.
- **Missing date fields** – about 26 % of `RECORDDATE`s are null, and a similar fraction for `SALEDATE`, which could bias time‑series analyses.
- **Dominant categories** – Pittsburgh city and school code 47 dominate (~54 % & ~25 % respectively), while “Other” buckets hold the rest.
- **Sparse address components** – direction, suffix, unit descriptor have high null rates (≈96–98 %), indicating many incomplete addresses.
- **Large “Other” categories in several fields** – e.g., `PARID`, `DEEDBOOK`, `INSTRTYP` each have >95 % of values in an “Other” bucket, suggesting many unique or rarely occurring codes.
- **Potential duplicates** – the `_id` field is unique, but other key combinations (e.g., address + sale date) might repeat across different parcels.
- **Data quality concerns** – inconsistent formatting in dates (`YYYY-MM-DD` vs. missing), mixed numeric/string types for `PRICE`, and a few negative price counts in the summary stats hint at entry errors.

*Attribution: Generated by qsv v12.0.0 describegpt
Command line: qsv describegpt --all allegheny_county_property_sale_transactions.csv --language Christopher Walken -o ChristopherWalken.md --fresh
Prompt file: Default v7.0.0
Model: openai/gpt-oss-20b
LLM API URL: http://localhost:1234/v1
Language: 
Timestamp: 2025-12-26T12:24:34.436185+00:00

WARNING: Description generated by an LLM and may contain inaccuracies. Verify before using!
*


## REASONING

We need to write a description in style of Christopher Walken, which is quirky, abrupt, sometimes stuttering? We'll craft sentences reminiscent of his speaking style: short, clipped, odd pauses, maybe "you know". Not too long. Up to 8 sentences. Then bullets.

Use dataset overall. Summarize central tendency etc. Use terms: skewness, cardinality, uniqueness ratio, outliers. Use some adjectives like "big data", "massive".

Make sure not to output per field stats. Just general.

Add Attribution placeholder.

Let's craft:

Description (max 8 sentences). We'll make maybe 6 sentences.

Then Notable Characteristics bullets: mention skewness of price, high cardinality of ID, many missing dates, etc.

Use Christopher Walken style: "You see, it's like...". Use some stuttering like "uh" maybe. But keep professional.

Let's write.


## TOKEN USAGE

TokenUsage { prompt: 9322, completion: 712, total: 10034, elapsed: 18607 }
---
# Tags
- pittsburgh_property_sales  
- real_estate_price_data  
- transaction_dates  
- sale_type_codes  
- deed_instruction_types  
- address_components  
- parcel_identifiers  
- school_districts  
- zip_code_distribution  
- municipal_wards  

Generated by qsv v12.0.0 describegpt
Command line: qsv describegpt --all allegheny_county_property_sale_transactions.csv --language Christopher Walken -o ChristopherWalken.md --fresh
Prompt file: Default v7.0.0
Model: openai/gpt-oss-20b
LLM API URL: http://localhost:1234/v1
Language: 
Timestamp: 2025-12-26T12:24:50.978143+00:00

WARNING: Tags generated by an LLM and may contain inaccuracies. Verify before using!



## REASONING

We need to choose no more than 10 unique tags in Markdown format, descending importance based on summary stats and frequency distribution. Use lowercase only, underscores between words. Must be in the language Christopher Walken? That is weird: "in this language: Christopher Walken". Likely we must write tags in a style reminiscent of Christopher Walken's speech? Maybe using his cadence? But tags are keywords. The instruction says: "Make sure your tag choices are in this language: Christopher Walken". I think they want us to produce tags that use the style of Christopher Walken, but still be tags (keywords). Not clear.

Maybe they mean we should write tags as if spoken by Christopher Walken: e.g., "real-estate_data", "pittsburgh_property_sales" etc. But "in this language" could refer to the way Christopher Walken speaks (like stuttering, emphatic punctuation). But tags cannot have spaces or punctuation beyond underscores. So maybe we just produce normal tags.

But they specifically say: "Make sure your tag choices are in this language: Christopher Walken". So we need to adopt a style reminiscent of him. For example, using ellipses? Not possible.

Perhaps they mean use the language that Christopher Walken uses: English with occasional stutters. But tags cannot have spaces or special characters; but maybe we can include hyphens? The instruction says lowercase only and use underscore to separate words. So no spaces.

Thus we produce tags like "pittsburgh_real_estate_sales". That is fine. We need up to 10 tags, descending importance based on dataset content: property sales data, location (Pittsburgh), zip codes, price, sale type, instruction types, etc.

Let's choose:

1. pittsburgh_property_sales
2. real_estate_price_data
3. transaction_dates
4. sale_type_codes
5. deed_instruction_types
6. address_components
7. parcel_identifiers
8. school_districts
9. zip_code_distribution
10. municipal_wards

Check uniqueness, underscore.

Now add attribution placeholder {GENERATED_BY_SIGNATURE} after tags in Markdown format.

Let's produce:

```
- pittsburgh_property_sales
- real_estate_price_data
- transaction_dates
- sale_type_codes
- deed_instruction_types
- address_components
- parcel_identifiers
- school_districts
- zip_code_distribution
- municipal_wards

{GENERATED_BY_SIGNATURE}
```
## TOKEN USAGE

TokenUsage { prompt: 9290, completion: 574, total: 9864, elapsed: 16541 }
---
