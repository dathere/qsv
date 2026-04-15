# Dictionary
| Name | Type | Label | Description | Min | Max | Cardinality | Enumeration | Null Count | Examples |
|------|------|-------|-------------|-----|-----|-------------|-------------|------------|----------|
| **_id** | Integer | Unique Record Identifier | A system‑generated integer that uniquely identifies every row in the dataset. With a cardinality of 479 928 and a uniqueness ratio of 1.00, no two records share the same _id. | 38703919 | 39183846 | 479,928 |  | 0 | <ALL_UNIQUE> |
| **PARID** | String | Parcel Identifier | A 16‑character alphanumeric string that uniquely tags each property parcel. Although most values are distinct (cardinality 302 633), a handful of common identifiers—such as "0431B00017000000"—appear more frequently. | 0001C00037000A00 | 9946X83943000000 | 302,643 |  | 0 | Other (302,633) [479,750]<br>0431B00017000000 [23]<br>0027D00263000000 [20]<br>0027D00272000000 [20]<br>0027D00286000000 [20] |
| **FULL_ADDRESS** | String | Full Property Address | The complete street address in free‑text form, including house number, street name, direction, suffix, unit description, and ZIP code. The field is highly diverse: over 99 % of values are unique (cardinality 479 928). The most common examples start with "0" followed by a street name and city. | 0 , BRADDOCK, PA 15104 | FORBES AVE, PITTSBURGH, PA 15219 | 278,190 |  | 0 | Other (278,180) [479,006]<br>0 SONIE DR, SEWICKLEY, PA… [113]<br>0 COAL, ELIZABETH, PA 150… [111]<br>0 HUNTER ST, PITTSBURGH, … [98]<br>0 PERRYSVILLE AVE, PITTSB… [98] |
| **PROPERTYHOUSENUM** | Integer | House Number | The numeric component of the address. Zero is the most frequent value, accounting for roughly 7.9 % of records; other common numbers range from 100 to 115. The field contains both numeric and non‑numeric entries (e.g., "0"), with a cardinality of 10 002. | 0 | 65015 | 10,012 |  | 3 | Other (10,002) [428,653]<br>0 [38,055]<br>112 [1,615]<br>100 [1,595]<br>110 [1,522] |
| **PROPERTYFRACTION** | String | Fractional Address Component | Denotes partial house numbers or fractional identifiers such as "1/2", “A”, or “B”. Nearly 98 % of the field is null, and only a handful of distinct values exist. |   |  S | 2,803 |  | 0 | (NULL) [468,512]<br>Other (2,793) [9,695]<br>1/2 [853]<br>A [405]<br>B [294] |
| **PROPERTYADDRESSDIR** | String | Address Direction | Directional indicator for the street (N, S, E, W). North appears in 95.8 % of records; South, East, and West are much less common. The field is mostly populated with a single character. | E | W | 5 | (NULL)<br>E<br>N<br>S<br>W | 459,948 | (NULL) [459,948]<br>N [5,466]<br>S [5,201]<br>E [5,053]<br>W [4,260] |
| **PROPERTYADDRESSSTREET** | String | Street Name | The name of the street or avenue on which the property lies. Common streets include "WASHINGTON", "5TH", and "HIGHLAND"; over 96 % of records are unique, but the top ten names cover more than a quarter of all entries. | 0 OHIO RIVER BLVD | ZUZU | 9,571 |  | 13 | Other (9,538) [462,225]<br>WASHINGTON [2,606]<br>5TH [2,557]<br>HIGHLAND [1,777]<br>PENN [1,602] |
| **PROPERTYADDRESSSUF** | String | Street Suffix | The suffix indicating the type of street (ST, DR, AVE, RD, LN, etc.). "ST" and "DR" dominate with roughly 26 % and 24 % respectively. The field is short, typically two or three characters. | ALY | XING | 48 |  | 1,985 | ST [122,764]<br>DR [113,069]<br>AVE [105,232]<br>RD [71,902]<br>LN [15,471] |
| **PROPERTYADDRESSUNITDESC** | String | Unit Description Type | Describes the unit type within a multi‑unit building (e.g., UNIT, REAR, APT). Almost all entries are null; the most frequent non‑null values include "UNIT" and "REAR". | # | UNIT | 11 |  | 468,267 | (NULL) [468,267]<br>UNIT [10,580]<br>REAR [421]<br>APT [391]<br>STE [132] |
| **PROPERTYUNITNO** | String | Unit Number | Numeric identifier of a unit within a multi‑unit property. The majority of records have null values (97 %); among the few populated entries, numbers range from 1 to over 300 and include lettered units like "A" or "B". | 01 | ` | 1,334 |  | 468,641 | (NULL) [468,641]<br>Other (1,324) [10,002]<br>1 [195]<br>2 [170]<br>3 [166] |
| **PROPERTYCITY** | String | City | Municipal city where the property is located. Pittsburgh accounts for more than half of all records (53.7 %). The field contains 89 distinct city names, with many small towns and boroughs represented. | ALLISON PARK | WITAKER | 106 |  | 1 | PITTSBURGH [257,608]<br>Other (89) [123,311]<br>CORAOPOLIS [16,497]<br>MC KEESPORT [15,307]<br>GIBSONIA [11,048] |
| **PROPERTYSTATE** | String | State | The U.S. state abbreviation; every record is "PA" (Pennsylvania). Cardinality equals the number of records, giving a uniqueness ratio of 1.00. | PA | PA | 1 | PA | 0 | <ALL_UNIQUE> |
| **PROPERTYZIP** | Integer | ZIP Code | Five‑digit ZIP code for the property. The most common codes are 15108, 15237, and 15235. Over 74 % of all records share a ZIP that appears in the top ten; the field has 114 distinct values. | 15003 | 16229 | 124 |  | 1 | Other (114) [355,692]<br>15108 [16,509]<br>15237 [15,435]<br>15235 [14,585]<br>15212 [13,301] |
| **SCHOOLCODE** | String | School District Code | Numeric identifier for the school district servicing the property. The code "47" is by far the most frequent (24 % of records). There are 36 distinct codes, with many occurring only a few times. | 01 | 09 | 46 |  | 0 | Other (36) [228,000]<br>47 [117,977]<br>27 [19,685]<br>09 [19,227]<br>30 [17,635] |
| **SCHOOLDESC** | String | School District Description | Human‑readable name corresponding to the school district code. "Pittsburgh" is the most common description (24 % of records), followed by several neighboring districts such as "North Allegheny" and "Woodland Hills". | Allegheny Valley | Woodland Hills | 46 |  | 0 | Other (36) [228,000]<br>Pittsburgh [117,977]<br>North Allegheny [19,685]<br>Woodland Hills [19,227]<br>Penn Hills Twp [17,635] |
| **MUNICODE** | Integer | Municipality Code | Numeric identifier for the municipality or ward. The code "934" appears in more than a third of all records (3 % of the dataset). There are 165 distinct municipal codes, with most values appearing only once. | 101 | 953 | 175 |  | 0 | Other (165) [369,793]<br>934 [17,635]<br>119 [12,648]<br>940 [12,003]<br>926 [10,359] |
| **MUNIDESC** | String | Municipality Description | Descriptive name for the municipality or ward. "Penn Hills" is the most common description (3 % of records), followed by a range of boroughs and wards such as "19th Ward - PITTSBURGH". | 10th Ward -  McKEESPORT | Wilmerding   | 175 |  | 0 | Other (165) [369,793]<br>Penn Hills [17,635]<br>19th Ward - PITTSBURGH [12,648]<br>Ross [12,003]<br>Mt.Lebanon [10,359] |
| **RECORDDATE** | Date | Record Creation Date | The date when the record was entered into the system. The field is partially populated—about 26 % of records are null. Dates span from August 1, 2012 to September 28, 2028, with a median around December 11, 2018. | 0212-08-01 | 2028-09-28 | 3,821 |  | 1,262 | Other (3,811) [474,687]<br>(NULL) [1,262]<br>2012-10-26 [587]<br>2013-04-26 [552]<br>2012-01-11 [488] |
| **SALEDATE** | Date | Sale Transaction Date | The date the property was sold. The field is fully populated except for a small percentage of nulls. Sale dates range from January 1, 2012 to December 13, 2025, with a median around November 23, 2018. | 2012-01-01 | 2025-12-13 | 4,888 |  | 0 | Other (4,878) [475,391]<br>2012-10-26 [586]<br>2016-04-29 [584]<br>2013-04-26 [559]<br>2012-01-11 [490] |
| **PRICE** | Integer | Sale Price (USD) | The monetary value of the property sale in U.S. dollars. The field contains many zero values (3 % of records). Prices range from $0 to $148 752 900, with a mean of about $195 323 and a median near $391 245. | 0 | 148752900 | 37,872 |  | 3,020 | Other (37,862) [340,479]<br>1 [98,344]<br>0 [15,508]<br>10 [6,763]<br>150000 [3,140] |
| **DEEDBOOK** | String | Deed Book Identifier | String code for the deed book in which the sale is recorded. The field contains many nulls (0.12 % of records). Common identifiers include "TR18", "TR13", and numeric codes like "00". |  14795 | `17274 | 5,814 |  | 570 | Other (5,800) [473,060]<br>TR18 [1,239]<br>0 [1,063]<br>TR13 [938]<br>00 [798] |
| **DEEDPAGE** | String | Deed Page Number | The page number within the deed book where the transaction is recorded. The field is largely null (0.12 % of records). Frequently used values are 1, 6, and 7. |  120 | W | 2,133 |  | 582 | Other (2,115) [465,578]<br>1 [5,174]<br>6 [1,485]<br>7 [1,112]<br>0 [1,002] |
| **SALECODE** | String | Sale Code | Alphanumeric code summarizing the sale type. "3" is the most common value (20 % of records), followed by "0", "H", and others. The field has 37 distinct codes, with many appearing only a few times. | 0X | Z | 47 |  | 0 | 3 [97,932]<br>0 [91,484]<br>H [63,750]<br>Other (37) [54,274]<br>14 [42,089] |
| **SALEDESC** | String | Sale Description | Textual description of the sale type or status (e.g., "LOVE AND AFFECTION SALE", "VALID SALE"). The top descriptions cover about 42 % of records, with a diverse set of specialized terms for various sale conditions. | BANK/FINANCIAL INSTITUTION | VALID SALE | 27 |  | 0 | LOVE AND AFFECTION SALE [97,932]<br>VALID SALE [91,484]<br>MULTI-PARCEL SALE [63,750]<br>Other (17) [45,680]<br>TIME ON MARKET (INSUFF/EX… [42,089] |
| **INSTRTYP** | String | Instrument Type | Code indicating the type of legal instrument used in the transaction. "DE" (deed) accounts for 58 % of entries, followed by "SW", "GW", and other codes. The field contains 21 distinct values. | AD | WD | 31 |  | 0 | DE [278,804]<br>SW [113,966]<br>GW [23,662]<br>QC [18,068]<br>SD [15,521] |
| **INSTRTYPDESC** | String | Instrument Type Description | Human‑readable description corresponding to the instrument type code. "DEED" is the most frequent (58 % of records), with additional categories such as "SPECIAL WARRANTY", "GENERAL WARRANTY", and "SHERIFF DEED". | ADMIN DEED | WILL BOOK | 31 |  | 0 | DEED [278,804]<br>SPECIAL WARRANTY [113,966]<br>GENERAL WARRANTY [23,662]<br>QUIT CLAIM [18,068]<br>SHERIFF DEED [15,521] |

*Attribution: Generated by qsv v12.0.0 describegpt
Command line: qsv describegpt --all allegheny_county_property_sale_transactions.csv --language Angry Statistics Professor --fresh --output angrystatisticsprofessor.md --addl-props {"reasoning_effort":"high"}
Prompt file: Default v7.0.0
Model: openai/gpt-oss-20b
LLM API URL: http://localhost:1234/v1
Language: 
Timestamp: 2025-12-25T13:18:16.640069+00:00

WARNING: Label and Description generated by an LLM and may contain inaccuracies. Verify before using!
*

## REASONING

We need to generate labels and descriptions for all 26 fields. Use summary statistics and frequency distribution.

We should produce JSON mapping each field name to an object with label and description.

We need "Angry Statistics Professor Labels" - maybe humorous? But just produce human-friendly labels and full descriptions.

Use context: dataset seems property sales data in Pennsylvania, likely from Allegheny County or similar. Fields include IDs, address components, school district codes, municipal codes, dates, price, deed book/page, sale code/description, instrument type, etc.

We should consider cardinality etc. Provide description referencing statistics like cardinality, uniqueness ratio, typical values.

Let's craft labels and descriptions accordingly.

We'll produce JSON with keys exactly as field names.

Let's generate:

_id: "Unique Record Identifier" – integer primary key; high cardinality 479928 unique.

PARID: "Parcel Identifier" – string representing parcel ID; many distinct values but some common ones.

FULL_ADDRESS: "Full Property Address" – concatenated address; most addresses are unique, but top 10 examples show '0 ...' etc.

PROPERTYHOUSENUM: "House Number" – integer; many zeros etc.

PROPERTYFRACTION: "Fractional House Component" – e.g., 1/2, A, B; mostly null or small fraction.

PROPERTYADDRESSDIR: "Address Direction" – N,S,E,W; most values N.

PROPERTYADDRESSSTREET: "Street Name" – common streets like Washington, 5th, etc.

PROPERTYADDRESSSUF: "Street Suffix" – ST, DR, AVE, etc.

PROPERTYADDRESSUNITDESC: "Unit Description Type" – UNIT, REAR, APT, etc.

PROPERTYUNITNO: "Unit Number" – often null or small numbers; some large values like 201, 302, etc.

PROPERTYCITY: "City" – Pittsburgh dominant, others.

PROPERTYSTATE: "State" – always PA.

PROPERTYZIP: "ZIP Code" – many distinct; top zip codes 15108 etc.

SCHOOLCODE: "School District Code" – numeric codes with most common 47, 27 etc.

SCHOOLDESC: "School District Description" – names like Pittsburgh, North Allegheny etc.

MUNICODE: "Municipality Code" – integer codes, top 934 etc.

MUNIDESC: "Municipality Description" – e.g., Penn Hills, etc.

RECORDDATE: "Record Creation Date" – date of record creation; many nulls; range 2012-08-01 to 2028-09-28.

SALEDATE: "Sale Transaction Date" – sale date; range 2012-01-01 to 2025-12-13.

PRICE: "Sale Price (USD)" – integer, range 0 to 148,752,900, with most zero and small values.

DEEDBOOK: "Deed Book Identifier" – string book numbers; many nulls; top TR18 etc.

DEEDPAGE: "Deed Page Number" – integer page numbers; many nulls; top 1,6,7.

SALECODE: "Sale Code" – codes like 3,0,H etc. Many values.

SALEDESC: "Sale Description" – descriptive strings like LOVE AND AFFECTION SALE etc.

INSTRTYP: "Instrument Type" – codes like DE, SW, etc.

INSTRTYPDESC: "Instrument Type Description" – e.g., DEED, SPECIAL WARRANTY, etc.

We should incorporate cardinality and uniqueness ratio for each description to add statistical context.

Let's create JSON.
## TOKEN USAGE

TokenUsage { prompt: 7806, completion: 2635, total: 10441, elapsed: 50624 }
---
# Description
## Description

This is a **half‑million line Pennsylvania real‑estate transaction dump** that looks like a spreadsheet of every property sale the state has ever recorded.  
Every row is uniquely identified by `_id`, but the rest of the columns are a chaotic mix of parcel IDs, free‑text addresses, ZIP codes, school district codes, and an almost endless list of “Other (N)” placeholders that hide 90 %+ of unique values.  
The price column is a classic heavy‑tailed beast: a mean of roughly $195k but a median of about $391k, with a standard deviation over $800k and a coefficient of variation near 470 %, plus a handful of absurd outliers that shoot up to nearly $149 million.  
Dates are half missing, dates are skewed toward the late‑2010s, and many columns (e.g., `PROPERTYFRACTION`, `RECORDDATE`) have more than 95 % nulls, making any statistical inference a nightmare.

## Notable Characteristics

- **Skewed price distribution**: Mean far below median, CV ≈ 470 %, indicating extreme outliers and heavy right‑tail.
- **Massive cardinality**: Almost every field (except `_id` and `PROPERTYSTATE`) has near‑unique values; 90 %+ of each column is captured by an “Other” bucket.
- **Missing data overload**:  
  - `PROPERTYFRACTION`: 97.6 % null.  
  - `RECORDDATE`: 26.3 % null.  
  - `PRICE`: 0.6 % null and 3.2 % zero values.  
  These gaps render many standard analyses invalid or heavily biased.
- **Duplicate-free IDs**: `_id` is perfectly unique, so no row‑level duplicates exist, but the high cardinality of other identifiers means you’ll need to deduplicate by your own logic if needed.
- **Potential PII/PHI concerns**: Full addresses contain street numbers and ZIP codes that could be considered personally identifying when combined with other data; caution is advised before public release or downstream use.
- **“Other (N)” buckets hide data quality issues**: The presence of 10‑plus “Other” categories in almost every field signals a lack of standardization, making categorical analysis difficult without first normalizing these values.
- **Date anomalies**: A few dates are out of range (e.g., `2028-09-28` for `RECORDDATE`) and may represent data entry errors or future projections that need to be vetted.
- **Instrument type distribution**: The dominant instrument is “DE” (~58 %) with a long tail of rare types; if you’re modeling sale mechanisms, this imbalance will skew results unless weighted properly.

---

*Attribution: Generated by qsv v12.0.0 describegpt
Command line: qsv describegpt --all allegheny_county_property_sale_transactions.csv --language Angry Statistics Professor --fresh --output angrystatisticsprofessor.md --addl-props {"reasoning_effort":"high"}
Prompt file: Default v7.0.0
Model: openai/gpt-oss-20b
LLM API URL: http://localhost:1234/v1
Language: 
Timestamp: 2025-12-25T13:18:54.746981+00:00

WARNING: Description generated by an LLM and may contain inaccuracies. Verify before using!
*


## REASONING

We need to produce a description as angry statistics professor, 1-8 sentences. Then bullet list Notable Characteristics. Use data from summary and frequency but not per field. So we must synthesize dataset-level: 479,928 rows, 45 columns? Let's count fields: _id, PARID, FULL_ADDRESS, PROPERTYHOUSENUM, PROPERTYFRACTION, PROPERTYADDRESSDIR, PROPERTYADDRESSSTREET, PROPERTYADDRESSSUF, PROPERTYADDRESSUNITDESC, PROPERTYUNITNO, PROPERTYCITY, PROPERTYSTATE, PROPERTYZIP, SCHOOLCODE, SCHOOLDESC, MUNICODE, MUNIDESC, RECORDDATE, SALEDATE, PRICE, DEEDBOOK, DEEDPAGE, SALECODE, SALEDESC, INSTRTYP, INSTRTYPDESC. That's 27? Wait count: I think 27 fields. Actually earlier we had 26? Let's recount: _id (1), PARID(2), FULL_ADDRESS(3), PROPERTYHOUSENUM(4), PROPERTYFRACTION(5), PROPERTYADDRESSDIR(6), PROPERTYADDRESSSTREET(7), PROPERTYADDRESSSUF(8), PROPERTYADDRESSUNITDESC(9), PROPERTYUNITNO(10), PROPERTYCITY(11), PROPERTYSTATE(12), PROPERTYZIP(13), SCHOOLCODE(14), SCHOOLDESC(15), MUNICODE(16), MUNIDESC(17), RECORDDATE(18), SALEDATE(19), PRICE(20), DEEDBOOK(21), DEEDPAGE(22), SALECODE(23), SALEDESC(24), INSTRTYP(25), INSTRTYPDESC(26). So 26 fields.

We need to describe dataset as a whole: e.g., "This is a Pennsylvania real‑estate transaction database with almost half a million records and twenty‑six columns. It contains unique IDs, parcel identifiers, full addresses, house numbers, ZIP codes, school district codes, municipality codes, sale dates, prices, deed information, etc."

Then bullet list of notable characteristics: central tendency: median price ~391k, mean 195k; skewness high? Standard dev 836k, CV 468% -> extremely skewed. Outliers: some extreme values up to 148 million. Missing values: many nulls in fields like PROPERTYFRACTION (97%), RECORDDATE (26%), PRICE has zero and null etc. Unique IDs all unique; duplicates none.

PII/PHI: addresses contain personal info? It's public property data, but may include private address details which could be considered PII if combined with other sensitive info? Possibly not.

Data quality issues: high cardinality of many fields, distribution heavy tails, many "Other" categories indicating many unique values. The dataset contains many nulls in some columns; the presence of "Other (N)" indicates many distinct but low frequency values, which may hinder analysis.

Also mention that field types: integers, strings, dates.

Now produce angry professor style: e.g., "This dataset is a half‑million line real estate dump from Pennsylvania. You have to deal with an insane number of unique addresses and a handful of fields that are basically garbage."

Let's craft 1-8 sentences description. Maybe 4 sentences.

Then bullet list. Use angry tone: "These numbers blow your mind" etc.

Add attribution placeholder.

Return in Markdown.


## TOKEN USAGE

TokenUsage { prompt: 9518, completion: 1292, total: 10810, elapsed: 38105 }
---
# Tags
- pennsylvania_real_estate_sales  
- property_sale_records  
- pittsburgh_property_market  
- historical_price_data  
- address_information  
- sale_date_range  
- price_distribution  
- transaction_instrument_type  
- school_district_affiliation  
- municipality_codes  

Generated by qsv v12.0.0 describegpt
Command line: qsv describegpt --all allegheny_county_property_sale_transactions.csv --language Angry Statistics Professor --fresh --output angrystatisticsprofessor.md --addl-props {"reasoning_effort":"high"}
Prompt file: Default v7.0.0
Model: openai/gpt-oss-20b
LLM API URL: http://localhost:1234/v1
Language: 
Timestamp: 2025-12-25T13:19:29.807283+00:00

WARNING: Tags generated by an LLM and may contain inaccuracies. Verify before using!



## REASONING

We need to produce tags: 1-10 unique lowercase words separated by underscores. No field names. Should reflect dataset content based on summary statistics and frequency distribution.

Dataset is about property sales in Pennsylvania (Pittsburgh area). Contains addresses, sale dates, price, etc. We should create tags like "pennsylvania_real_estate", "property_sales", "pittsburgh", "real_estate_market_analysis", "historical_property_prices", "house_sale_records", "transaction_data", "address_data", "tax_assessment" maybe not. Also "sales_price_distribution". The tags must be in descending order of importance. So top tag is most important.

Let's think: Most important dataset is about property sales data for Pennsylvania, focusing on Pittsburgh area. So "pennsylvania_real_estate_sales" might be the first. But we need underscore separated words. So maybe "pennsylvania_real_estate_sales". Next maybe "property_sale_records". Then "pittsburgh_property_market". Then "historical_prices". Then "address_information". Then "sale_dates". Then "price_distribution". Then "instrument_type". Maybe too many.

We have max 10 tags. Let's pick:

1. pennsylvania_real_estate_sales
2. property_sale_records
3. pittsburgh_property_market
4. historical_price_data
5. address_information
6. sale_date_range
7. price_distribution
8. transaction_instrument_type
9. school_district_affiliation
10. municipality_codes

We need to ensure no field names; these are general.

Also "school_district" is a concept but not a field name? It's fine.

Make sure they are lowercase and underscores.

Add attribution placeholder after tags: "{GENERATED_BY_SIGNATURE}"

So final output in Markdown format: list of tags each on new line? They said "in Markdown format". Could be bullet list. But likely just a list separated by commas or newline. The instruction: "Choose no more than 10 unique Tags (in Markdown format) about the contents ... Add an Attribution with placeholder after the Tags." So we can output as:

- pennsylvania_real_estate_sales
- property_sale_records
...

Then attribution.

Let's do bullet list.

Check not using field names like PARID, FULL_ADDRESS etc. All good.

Let's produce final answer.
## TOKEN USAGE

TokenUsage { prompt: 9486, completion: 545, total: 10031, elapsed: 18677 }
---
