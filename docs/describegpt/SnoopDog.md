# Dictionary
| Name | Type | Label | Description | Min | Max | Cardinality | Enumeration | Null Count | Examples |
|------|------|-------|-------------|-----|-----|-------------|-------------|------------|----------|
| **_id** | Integer | Unique Record Identifier | A system-generated numeric key that uniquely identifies every property sale record in the dataset. With 479,928 records and a uniqueness ratio of 1.0, this field serves as the primary key for database joins and indexing. | 38703919 | 39183846 | 479,928 |  | 0 | <ALL_UNIQUE> |
| **PARID** | String | Parcel Identification Number | The county-assigned parcel ID that uniquely identifies each property. The dataset contains roughly 302,643 distinct PARIDs (most frequent values such as "0431B00017000000" appear only a handful of times), while the majority of records are grouped under an "Other (302,633)" bucket indicating many unique parcel numbers. | 0001C00037000A00 | 9946X83943000000 | 302,643 |  | 0 | Other (302,633) [479,750]<br>0431B00017000000 [23]<br>0027D00263000000 [20]<br>0027D00272000000 [20]<br>0027D00286000000 [20] |
| **FULL_ADDRESS** | String | Complete Property Address | A concatenated string that includes street number, name, direction, suffix, unit, city, state and ZIP. The most common addresses are "0 SONIE DR, SEWICKLEY, PA 15143" (113 occurrences) and "0 COAL, ELIZABETH, PA 15037" (111). Despite high cardinality (479,928 distinct addresses), many records share these top ten values. | 0 , BRADDOCK, PA 15104 | FORBES AVE, PITTSBURGH, PA 15219 | 278,190 |  | 0 | Other (278,180) [479,006]<br>0 SONIE DR, SEWICKLEY, PA… [113]<br>0 COAL, ELIZABETH, PA 150… [111]<br>0 HUNTER ST, PITTSBURGH, … [98]<br>0 PERRYSVILLE AVE, PITTSB… [98] |
| **PROPERTYHOUSENUM** | Integer | House Number | Numeric portion of the street address. The value "0" appears in 38,055 records (7.9 % of all entries) indicating vacant lots or unnumbered properties; other common numbers include 112 (1.6 %), 100 (1.6 %) and 110 (1.3 %). Approximately 89 % of house numbers fall into an "Other (10,002)" bucket. | 0 | 65015 | 10,012 |  | 3 | Other (10,002) [428,653]<br>0 [38,055]<br>112 [1,615]<br>100 [1,595]<br>110 [1,522] |
| **PROPERTYFRACTION** | String | Property Fraction | Describes a fractional portion of the property such as "1/2", letters (A‑C), or negative fractions. The majority of records (97.6 %) contain NULL values, while only 0.18 % have "1/2" and 0.08 % have "A". |   |  S | 2,803 |  | 0 | (NULL) [468,512]<br>Other (2,793) [9,695]<br>1/2 [853]<br>A [405]<br>B [294] |
| **PROPERTYADDRESSDIR** | String | Street Direction Prefix | Indicates the direction of the street (N, S, E, W). N is most frequent (5.4 %), followed by S (5.1 %) and E (5.0 %). Nearly 96 % of records contain a valid direction; NULL values appear in 95.8 %. | E | W | 5 | (NULL)<br>E<br>N<br>S<br>W | 459,948 | (NULL) [459,948]<br>N [5,466]<br>S [5,201]<br>E [5,053]<br>W [4,260] |
| **PROPERTYADDRESSSTREET** | String | Street Name | The name component of the street address. Washington, 5th, HIGHLAND, PENN, and LINCOLN are the top five street names, each representing roughly half a percent of all records. The dataset contains many unique street names (cardinality close to the record count). | 0 OHIO RIVER BLVD | ZUZU | 9,571 |  | 13 | Other (9,538) [462,225]<br>WASHINGTON [2,606]<br>5TH [2,557]<br>HIGHLAND [1,777]<br>PENN [1,602] |
| **PROPERTYADDRESSSUF** | String | Street Suffix | Common suffixes such as ST, DR, AVE, RD, LN, CT, BLVD, WAY, PL, CIR. The most frequent suffix is ST (25.6 %), followed by DR (23.6 %) and AVE (21.9 %). An "Other (38)" bucket accounts for 1.8 % of all values. | ALY | XING | 48 |  | 1,985 | ST [122,764]<br>DR [113,069]<br>AVE [105,232]<br>RD [71,902]<br>LN [15,471] |
| **PROPERTYADDRESSUNITDESC** | String | Unit Description | Descriptor indicating the type of unit, e.g., UNIT, REAR, APT, STE, BLDG. NULL values dominate (97.6 %). The top descriptor "UNIT" appears in 2.2 % of records. | # | UNIT | 11 |  | 468,267 | (NULL) [468,267]<br>UNIT [10,580]<br>REAR [421]<br>APT [391]<br>STE [132] |
| **PROPERTYUNITNO** | String | Unit Number | Alphanumeric unit identifier when a property has multiple units. Most records contain NULL (97.6 %). The most frequent numeric unit is "1" (0.04 %) and the letter "A" (0.03 %). | 01 | ` | 1,334 |  | 468,641 | (NULL) [468,641]<br>Other (1,324) [10,002]<br>1 [195]<br>2 [170]<br>3 [166] |
| **PROPERTYCITY** | String | City | The city in which the property is located. Pittsburgh is the most common city, accounting for 53.7 % of records, followed by CORAOPOLIS (3.4 %) and MC KEESPORT (3.2 %). | ALLISON PARK | WITAKER | 106 |  | 1 | PITTSBURGH [257,608]<br>Other (89) [123,311]<br>CORAOPOLIS [16,497]<br>MC KEESPORT [15,307]<br>GIBSONIA [11,048] |
| **PROPERTYSTATE** | String | State Code | The state abbreviation; all records are from Pennsylvania (PA), giving a 100 % uniqueness ratio. | PA | PA | 1 | PA | 0 | <ALL_UNIQUE> |
| **PROPERTYZIP** | Integer | ZIP Code | Five‑digit ZIP code of the property. The most common ZIPs are 15108, 15237, and 15235, each representing about 3–4 % of records. Over 74 % of all ZIP codes fall into an "Other (114)" bucket. | 15003 | 16229 | 124 |  | 1 | Other (114) [355,692]<br>15108 [16,509]<br>15237 [15,435]<br>15235 [14,585]<br>15212 [13,301] |
| **SCHOOLCODE** | String | School District Code | Numerical code that identifies the school district for a property. The top code is 47, covering 24.6 % of records; other codes such as 27 and 09 each account for roughly 4 %. The "Other (36)" bucket represents 47.5 %. | 01 | 09 | 46 |  | 0 | Other (36) [228,000]<br>47 [117,977]<br>27 [19,685]<br>09 [19,227]<br>30 [17,635] |
| **SCHOOLDESC** | String | School District Description | Human‑readable name of the school district. Pittsburgh is the most frequent description (24.6 %), followed by North Allegheny, Woodland Hills, and Penn Hills Twp. An "Other (36)" bucket captures 47.5 % of all entries. | Allegheny Valley | Woodland Hills | 46 |  | 0 | Other (36) [228,000]<br>Pittsburgh [117,977]<br>North Allegheny [19,685]<br>Woodland Hills [19,227]<br>Penn Hills Twp [17,635] |
| **MUNICODE** | Integer | Municipal Code | Numeric code representing the municipality (borough or township). The most common codes are 934, 119, and 940, each accounting for roughly 3–4 %. The majority of records belong to an "Other (165)" bucket. | 101 | 953 | 175 |  | 0 | Other (165) [369,793]<br>934 [17,635]<br>119 [12,648]<br>940 [12,003]<br>926 [10,359] |
| **MUNIDESC** | String | Municipal Description | Human‑readable name of the municipality. Penn Hills is the most frequent description (3.7 %), followed by 19th Ward – PITTSBURGH, Ross, and Mt. Lebanon. An "Other (165)" bucket represents 77.1 %. | 10th Ward -  McKEESPORT | Wilmerding   | 175 |  | 0 | Other (165) [369,793]<br>Penn Hills [17,635]<br>19th Ward - PITTSBURGH [12,648]<br>Ross [12,003]<br>Mt.Lebanon [10,359] |
| **RECORDDATE** | Date | Record Creation Date | Date when the property sale record was entered into the system. The range spans from early 2012 to late 2028, with a concentration around 2013 and 2016. NULL values appear in 0.26 % of records. | 0212-08-01 | 2028-09-28 | 3,821 |  | 1,262 | Other (3,811) [474,687]<br>(NULL) [1,262]<br>2012-10-26 [587]<br>2013-04-26 [552]<br>2012-01-11 [488] |
| **SALEDATE** | Date | Sale Date | Date on which the property sale actually occurred. The distribution covers 2012 to 2025, with a peak around 2013 and 2016. NULL values are rare (0.01 %). | 2012-01-01 | 2025-12-13 | 4,888 |  | 0 | Other (4,878) [475,391]<br>2012-10-26 [586]<br>2016-04-29 [584]<br>2013-04-26 [559]<br>2012-01-11 [490] |
| **PRICE** | Integer | Sale Price (USD) | Monetary value of the property sale in U.S. dollars. Prices range from $0 to over $148 million, with a median around $39 k and a mean of $195 k. The top ten price buckets include $1, $0, $10, and $150 000, while 70.9 % of prices fall into an "Other (37,862)" bucket. | 0 | 148752900 | 37,872 |  | 3,020 | Other (37,862) [340,479]<br>1 [98,344]<br>0 [15,508]<br>10 [6,763]<br>150000 [3,140] |
| **DEEDBOOK** | String | Deed Book Number | Identifier for the book in which the deed is recorded. The most common values are TR18, 0, and TR13; an "Other (5,800)" bucket captures 98.6 % of records. |  14795 | `17274 | 5,814 |  | 570 | Other (5,800) [473,060]<br>TR18 [1,239]<br>0 [1,063]<br>TR13 [938]<br>00 [798] |
| **DEEDPAGE** | String | Deed Page Number | Page number within the deed book where the transaction is recorded. The most frequent page numbers are 1, 6, and 7; a large "Other (2,115)" bucket accounts for 97 %. |  120 | W | 2,133 |  | 582 | Other (2,115) [465,578]<br>1 [5,174]<br>6 [1,485]<br>7 [1,112]<br>0 [1,002] |
| **SALECODE** | String | Sale Type Code | Alphanumeric code that classifies the sale type. The most common codes are 3, 0, and H, representing 20 %, 19 % and 13 % of records respectively; an "Other (37)" bucket covers 11 %. | 0X | Z | 47 |  | 0 | 3 [97,932]<br>0 [91,484]<br>H [63,750]<br>Other (37) [54,274]<br>14 [42,089] |
| **SALEDESC** | String | Sale Description | Human‑readable description of the sale type. The top descriptions are LOVE AND AFFECTION SALE, VALID SALE, and MULTI‑PARCEL SALE, together accounting for roughly 53 % of records. An "Other (17)" bucket represents 9.5 %. | BANK/FINANCIAL INSTITUTION | VALID SALE | 27 |  | 0 | LOVE AND AFFECTION SALE [97,932]<br>VALID SALE [91,484]<br>MULTI-PARCEL SALE [63,750]<br>Other (17) [45,680]<br>TIME ON MARKET (INSUFF/EX… [42,089] |
| **INSTRTYP** | String | Instruction Type Code | Code indicating the type of instruction or deed used in the transaction. The most common code is DE, accounting for 58 % of records; other codes such as SW (23 %) and GW (5 %) also appear frequently. An "Other (21)" bucket covers less than 1 %. | AD | WD | 31 |  | 0 | DE [278,804]<br>SW [113,966]<br>GW [23,662]<br>QC [18,068]<br>SD [15,521] |
| **INSTRTYPDESC** | String | Instruction Type Description | Human‑readable description of the instruction type. DEED is the most frequent description (58 %), followed by SPECIAL WARRANTY and GENERAL WARRANTY. The "Other (21)" bucket accounts for roughly 0.96 %. | ADMIN DEED | WILL BOOK | 31 |  | 0 | DEED [278,804]<br>SPECIAL WARRANTY [113,966]<br>GENERAL WARRANTY [23,662]<br>QUIT CLAIM [18,068]<br>SHERIFF DEED [15,521] |

*Attribution: Generated by qsv v12.0.0 describegpt
Command line: qsv describegpt --all allegheny_county_property_sale_transactions.csv --language Snoop Dog --fresh --output SnoopDog.md --addl-props {"reasoning_effort":"high"}
Prompt file: Default v7.0.0
Model: openai/gpt-oss-20b
LLM API URL: http://localhost:1234/v1
Language: 
Timestamp: 2025-12-25T12:42:08.911140+00:00

WARNING: Label and Description generated by an LLM and may contain inaccuracies. Verify before using!
*

## REASONING

We need to generate labels and descriptions for each field in the dataset, using summary stats and frequency distribution.

We have 26 fields:

1. _id
2. PARID
3. FULL_ADDRESS
4. PROPERTYHOUSENUM
5. PROPERTYFRACTION
6. PROPERTYADDRESSDIR
7. PROPERTYADDRESSSTREET
8. PROPERTYADDRESSSUF
9. PROPERTYADDRESSUNITDESC
10. PROPERTYUNITNO
11. PROPERTYCITY
12. PROPERTYSTATE
13. PROPERTYZIP
14. SCHOOLCODE
15. SCHOOLDESC
16. MUNICODE
17. MUNIDESC
18. RECORDDATE
19. SALEDATE
20. PRICE
21. DEEDBOOK
22. DEEDPAGE
23. SALECODE
24. SALEDESC
25. INSTRTYP
26. INSTRTYPDESC

We need to produce JSON with each field as key and object containing label, description.

The labels should be human-friendly. Descriptions: multiple sentences, referencing the stats/freq.

We'll use summary stats like cardinality, uniqueness ratio, etc. Also frequency distribution indicates most common values. For fields with many unique values (like _id), note it's a unique key.

We need to consider the context: dataset appears to be property sale records in Pennsylvania (e.g., property addresses, school districts). Provide description accordingly.

Let's craft for each field.

Also we should reference cardinality: e.g., _id 479928 unique. PARID 302643? Wait summary stats had cardinality maybe not given but can infer from freq distribution "Other" count etc. But use general info.

Let's create labels:

1. _id: Unique Record Identifier
2. PARID: Parcel Identification Number
3. FULL_ADDRESS: Complete Property Address
4. PROPERTYHOUSENUM: House Number
5. PROPERTYFRACTION: Fractional Portion of the House (e.g., 1/2)
6. PROPERTYADDRESSDIR: Street Direction Prefix
7. PROPERTYADDRESSSTREET: Street Name
8. PROPERTYADDRESSSUF: Street Suffix (e.g., Ave, St)
9. PROPERTYADDRESSUNITDESC: Unit Description (e.g., Apt, Suite)
10. PROPERTYUNITNO: Unit Number
11. PROPERTYCITY: City of the Property
12. PROPERTYSTATE: State Code (PA)
13. PROPERTYZIP: ZIP Code
14. SCHOOLCODE: School District Code
15. SCHOOLDESC: School District Description
16. MUNICODE: Municipal Code (likely borough or township)
17. MUNIDESC: Municipal Description
18. RECORDDATE: Record Creation Date
19. SALEDATE: Sale Date
20. PRICE: Sale Price in USD
21. DEEDBOOK: Deed Book Number
22. DEEDPAGE: Deed Page Number
23. SALECODE: Sale Type Code
24. SALEDESC: Sale Description
25. INSTRTYP: Instruction Type Code
26. INSTRTYPDESC: Instruction Type Description

Now descriptions:

Use summary stats for each field.

For example, _id: 479928 records, all unique (unique ratio 1). It's a primary key.

PARID: 302643 cardinality? Frequency distribution shows top values count of 23 etc; "Other" covers 302633. So PARID has many distinct values (~303k). It is the parcel identification number assigned by the county.

FULL_ADDRESS: Contains full property address including street, city, zip. Most frequent addresses are 0 SONIE DR etc. Many unique entries (479928 records with many duplicates). Provide description about typical structure.

PROPERTYHOUSENUM: House number; most common house numbers 0 (38055), 112 (1615) etc. Many zeros indicating vacant lots? Unique ratio maybe low but high count of other values.

PROPERTYFRACTION: Fraction of property, e.g., 1/2, A, B, etc. Majority NULL (468512). Provide description.

PROPERTYADDRESSDIR: Street direction (N,S,E,W). Most common N (5466) etc. Nulls also many.

PROPERTYADDRESSSTREET: Street name; most frequent Washington, 5th, etc. Many unique names.

PROPERTYADDRESSSUF: Suffix like ST, DR, AVE. Majority ST.

PROPERTYADDRESSUNITDESC: Unit descriptor such as UNIT, REAR, APT etc. Most NULL.

PROPERTYUNITNO: Unit number; majority NULL.

PROPERTYCITY: City; most frequent PITTSBURGH (257608). Provide description.

PROPERTYSTATE: PA for all records.

PROPERTYZIP: ZIP code; many unique; top 15108 etc. Provide distribution.

SCHOOLCODE: School district codes, e.g., 47, 27 etc. Provide explanation.

SCHOOLDESC: Name of school district.

MUNICODE: Municipal code, e.g., 934 etc. Provide description.

MUNIDESC: Municipal description.

RECORDDATE: Date record created; range 0212-08-01 to 2028-09-28? Actually min date 2012-08-01? Wait summary shows min "0212-08-01" which might be truncated. But frequency distribution shows many NULL (1262). Provide description.

SALEDATE: Date of sale; range 2012-01-01 to 2025-12-13. Provide distribution.

PRICE: Sale price in USD; min 0, max 148,752,900? Wait summary max is 148,752,900 but maybe mis reading. Actually the summary shows "max 148752900" (maybe $148M). The range of values: min 0, many zeros etc. Provide description.

DEEDBOOK: Deed book number; most common TR18 etc. Provide description.

DEEDPAGE: Page number; most common 1,6 etc.

SALECODE: Sale code: e.g., 3, 0, H, etc. Provide explanation of codes.

SALEDESC: Description of sale type (e.g., LOVE AND AFFECTION SALE). Provide typical values.

INSTRTYP: Instruction type code: DE, SW, GW etc.

INSTRTYPDESC: Description of instruction type.

We need to incorporate statistics like uniqueness ratio, cardinality, top frequencies. For fields with high uniqueness, note that.

Let's produce JSON. Ensure correct formatting.

We'll produce one object per field with label and description strings.

I'll craft each description carefully referencing the stats and freq distribution.

Ok let's write.
## TOKEN USAGE

TokenUsage { prompt: 7806, completion: 3467, total: 11273, elapsed: 59166 }
---
# Description
## Description  
Yo, this dataset is straight‑up a real estate jam from Pennsylvania – almost half a million records, each one a unique deal stamped with an ID that never repeats. Prices are low‑key to the bottom (most at zero or a few bucks) but some hits hit the 148 million mark, giving it a heavy right‑skew vibe. The street game is wild; Washington and 5th lead the pack, yet most addresses drop into the “Other” bucket because they’re all unique. A lot of fields are ghosted out – fractions, directions, unit descriptors, even dates – so you’ll see those NULLs pop up a ton. And yeah, parcel IDs and full addresses could be PII, so keep that data locked tight.

## Notable Characteristics  
- **Central Tendency & Spread**: Mean price ~ $195k, median ~ $39k, huge variance; skewness high on the right side due to outliers like $148M.  
- **Distribution Shape**: Price distribution is heavily right‑skewed; address uniqueness drives a long tail in categorical fields.  
- **Outliers & Anomalies**: Prices above $100k are rare but present; some dates have nulls or "Other" categories, indicating data gaps.  
- **Missing Values**: Fields like `PROPERTYFRACTION`, `PROPERTYADDRESSDIR`, and `RECORDDATE` show > 95% NULLs, causing sparsity issues.  
- **Duplicates & Uniqueness**: `_id` is perfectly unique; many other fields have high cardinality but still cluster in top 10 values (e.g., `SCHOOLCODE` “47”).  
- **PII/PHI/PCI Risks**: Full addresses, parcel IDs, and ZIP codes can reveal personal location info – treat as sensitive.  
- **Data Quality Issues**: Over 70% of `PRICE` entries are in the "Other" bucket (generic counts), indicating potential mis‑categorization; also “Other (N)” categories obscure actual diversity.

Generated by qsv v12.0.0 describegpt
Command line: qsv describegpt --all allegheny_county_property_sale_transactions.csv --language Snoop Dog --fresh --output SnoopDog.md --addl-props {"reasoning_effort":"high"}
Prompt file: Default v7.0.0
Model: openai/gpt-oss-20b
LLM API URL: http://localhost:1234/v1
Language: 
Timestamp: 2025-12-25T12:42:34.356875+00:00

WARNING: Description generated by an LLM and may contain inaccuracies. Verify before using!



## REASONING

We need to produce a description in Snoop Dog style, one 1-8 sentence description about dataset as whole, not per field. Then "Notable Characteristics" bullet list. Must use Snoop Dog language (slang). Use the summary and frequency stats to craft. Avoid outputting per-field statistics. Provide only overall dataset description.

We need to incorporate key aspects: number of records 479,928; variety of addresses, many unique values, high uniqueness ratio for id etc. Price distribution with many zeros/ones etc. Many missing values in some fields. Distribution shape (skewness). Outliers (high price). PII? The dataset contains addresses and parcel IDs which could be considered PII. Also some null counts.

Let's craft Snoop Dog style: "Yo, check this out, we got a massive real estate hustle with almost half a million records from Pennsylvania, all about property sales, prices that range from zero to 148 million, most of them chill at the low end but a few go big. The addresses are mad diverse, with some streets like Washington and 5th dominating, but most addresses are unique. There’s a ton of missing data in fields like fraction, direction, unit description, etc., and you can see the distribution’s skewed to the right because prices drop heavy. Some fields have outliers—like that $148M sale—and lots of zeros. The dataset also holds parcel IDs and addresses which could be PII/PHI, so keep it safe."

Need 1-8 sentences; we can do maybe 5 sentences.

Then bullet list: central tendency, spread, distribution shape, anomalies/outliers, missing values, duplicates, PII/PHI/PCI. Also data quality issues like high sparsity in some fields, many nulls, 'Other' categories.

Let's write in Snoop Dog style with slang but still clear.

Add Attribution placeholder.

Ensure Markdown format: Use headings (#), bullet list. Provide no extra commentary.

Let's craft.
## TOKEN USAGE

TokenUsage { prompt: 9742, completion: 853, total: 10595, elapsed: 25444 }
---
# Tags
- real_estate_swag  
- pa_property_flow  
- residential_price_juice  
- property_address_details  
- municipal_districts  
- school_district_info  
- deed_and_instruction_types  
- transaction_dates  
- price_distribution  
- property_characteristics  

Generated by qsv v12.0.0 describegpt
Command line: qsv describegpt --all allegheny_county_property_sale_transactions.csv --language Snoop Dog --fresh --output SnoopDog.md --addl-props {"reasoning_effort":"high"}
Prompt file: Default v7.0.0
Model: openai/gpt-oss-20b
LLM API URL: http://localhost:1234/v1
Language: 
Timestamp: 2025-12-25T12:43:17.493498+00:00

WARNING: Tags generated by an LLM and may contain inaccuracies. Verify before using!



## REASONING

We need to produce tags in Markdown format (list or bullet). Tags must be lowercase only, underscore separated words. No field names. Use Snoop Dog language: presumably slang? The user says "Make sure your tag choices are in this language: Snoop Dog". So we should write the tags using Snoop Dog style slang. But still need to be valid tags: lowercase and underscores. Maybe use phrases like "real-estate_sales", but in Snoop Dog vibe: maybe "snoop_real_estate" etc. The tags must describe dataset content, top importance.

We choose up to 10 unique tags.

Based on summary stats: It's a property sale dataset for Pennsylvania (PA) with lots of fields: addresses, prices, dates, school districts, municipalities, deed info, sale types.

Top tags:

1. real_estate_sales
2. pa_property_transactions
3. residential_sale_prices
4. property_address_details
5. municipal_districts
6. school_district_info
7. deed_and_instruction_types
8. transaction_dates
9. price_distribution
10. property_characteristics

Now Snoop Dog language: maybe incorporate "yo", "real-estate" etc. But tags must be lowercase and underscore. We can add slang words like "snoop_gangsta". But we need to keep it descriptive. For example:

- real_estate_sales -> could be "real_estate_snoop"
- pa_property_transactions -> "pa_property_rap"
But maybe too contrived.

Better: Use Snoop Dog style phrasing but still tags. Could use words like "snoop_real_estate" or "real-estate_freestyle". But we need to keep lowercase underscore.

Possible tags:

- real_estate_sales
- pa_property_transactions
- residential_sale_prices
- property_address_details
- municipal_districts
- school_district_info
- deed_and_instruction_types
- transaction_dates
- price_distribution
- property_characteristics

But that is standard. To add Snoop Dog vibe: maybe "real_estate_flow", "pa_property_juice". But still need descriptive.

Let's craft tags:

1. real_estate_sales (standard)
2. pa_property_transactions (state focus)
3. residential_sale_prices (prices)
4. property_address_details (address)
5. municipal_districts (municipal codes)
6. school_district_info (school code)
7. deed_and_instruction_types (deed book, instruction type)
8. transaction_dates (record & sale date)
9. price_distribution (price stats)
10. property_characteristics (house number, fraction, unit desc)

Now incorporate Snoop Dog language: maybe add "snoop_rap" but tags should describe dataset. Maybe we can prefix with "snoop_" to indicate style. Eg "snoop_real_estate_sales". But might be too generic.

Let's see user instructions: "Make sure your tag choices are in this language: Snoop Dog". So the content of tags must be in Snoop Dog slang? That is ambiguous. Possibly they want tags like "real-estate-swag" but still lowercase underscore. Eg "real_estate_swag", "pa_property_juice".

We can incorporate slang words: "snoop_real_estate", "pa_property_juice", "residential_sale_prices_fresh", etc.

But must keep only lowercase and underscores, no hyphens or spaces. So we can use:

- real_estate_swag
- pa_property_flow
- residential_price_juice
- property_address_details (keep as is)
- municipal_districts (as is)
- school_district_info (as is)
- deed_and_instruction_types (as is)
- transaction_dates (as is)
- price_distribution (as is)
- property_characteristics (as is)

But the first three include slang. But are they too unnatural? Maybe okay.

Alternatively, we could use "snoop_real_estate" for all tags to indicate Snoop Dog style. But user likely expects some tags that reflect dataset content but with Snoop Dog flair.

Let's produce 10 tags:

1. real_estate_swag
2. pa_property_flow
3. residential_price_juice
4. property_address_details
5. municipal_districts
6. school_district_info
7. deed_and_instruction_types
8. transaction_dates
9. price_distribution
10. property_characteristics

Now attribution placeholder: "{GENERATED_BY_SIGNATURE}" after tags. Use Markdown format.

So output:

- real_estate_swag
- pa_property_flow
- residential_price_juice
- property_address_details
- municipal_districts
- school_district_info
- deed_and_instruction_types
- transaction_dates
- price_distribution
- property_characteristics

{GENERATED_BY_SIGNATURE}

We need to use Markdown format: bullet list or maybe just tags separated by commas? The instruction says "in Markdown format". Typically list. We'll use bullet list.

Let's produce final answer.
## TOKEN USAGE

TokenUsage { prompt: 9710, completion: 1095, total: 10805, elapsed: 30448 }
---
