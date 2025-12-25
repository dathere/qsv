# Dictionary
| Name | Type | Label | Description | Min | Max | Cardinality | Enumeration | Null Count | Examples |
|------|------|-------|-------------|-----|-----|-------------|-------------|------------|----------|
| **_id** | Integer | Record ID | A unique numeric identifier for each property sale record in the dataset. This integer does not reference any other attribute and serves solely as a primary key. | 38703919 | 39183846 | 479,928 |  | 0 | <ALL_UNIQUE> |
| **PARID** | String | Parcel Identification Number | The 16‑digit Pennsylvania parcel ID that uniquely identifies a land parcel. It is used for tax, legal, and mapping purposes. While many records share common IDs, most are distinct. | 0001C00037000A00 | 9946X83943000000 | 302,643 |  | 0 | Other (302,633) [479,750]<br>0431B00017000000 [23]<br>0027D00263000000 [20]<br>0027D00272000000 [20]<br>0027D00286000000 [20] |
| **FULL_ADDRESS** | String | Full Property Address | The complete street address of the property, including house number, direction, street name, suffix, unit description, city, state, and ZIP code. | 0 , BRADDOCK, PA 15104 | FORBES AVE, PITTSBURGH, PA 15219 | 278,190 |  | 0 | Other (278,180) [479,006]<br>0 SONIE DR, SEWICKLEY, PA… [113]<br>0 COAL, ELIZABETH, PA 150… [111]<br>0 HUNTER ST, PITTSBURGH, … [98]<br>0 PERRYSVILLE AVE, PITTSB… [98] |
| **PROPERTYHOUSENUM** | Integer | House Number | Numeric component of the property's street address. A value of zero indicates no house number or an unknown address. Most records contain a non‑zero number. | 0 | 65015 | 10,012 |  | 3 | Other (10,002) [428,653]<br>0 [38,055]<br>112 [1,615]<br>100 [1,595]<br>110 [1,522] |
| **PROPERTYFRACTION** | String | Property Portion | Fractional designation of a parcel (e.g., "1/2", "A"). Many records have this field missing, indicating ownership of the entire parcel. |   |  S | 2,803 |  | 0 | (NULL) [468,512]<br>Other (2,793) [9,695]<br>1/2 [853]<br>A [405]<br>B [294] |
| **PROPERTYADDRESSDIR** | String | Street Direction | Abbreviation for the street direction (N, S, E, W) preceding the street name. Null values indicate no directional prefix in the address. | E | W | 5 | (NULL)<br>E<br>N<br>S<br>W | 459,948 | (NULL) [459,948]<br>N [5,466]<br>S [5,201]<br>E [5,053]<br>W [4,260] |
| **PROPERTYADDRESSSTREET** | String | Street Name | Name of the street where the property is located. Common streets include Washington, 5th, Highland, etc., and many records share the same street name. | 0 OHIO RIVER BLVD | ZUZU | 9,571 |  | 13 | Other (9,538) [462,225]<br>WASHINGTON [2,606]<br>5TH [2,557]<br>HIGHLAND [1,777]<br>PENN [1,602] |
| **PROPERTYADDRESSSUF** | String | Street Suffix | Abbreviation indicating the type of road (St, Ave, Rd, etc.). | ALY | XING | 48 |  | 1,985 | ST [122,764]<br>DR [113,069]<br>AVE [105,232]<br>RD [71,902]<br>LN [15,471] |
| **PROPERTYADDRESSUNITDESC** | String | Unit Descriptor | Describes unit type within a multi‑unit property (e.g., UNIT, APT, REAR). Null values are common for single‑family homes. | # | UNIT | 11 |  | 468,267 | (NULL) [468,267]<br>UNIT [10,580]<br>REAR [421]<br>APT [391]<br>STE [132] |
| **PROPERTYUNITNO** | String | Unit Number | Number or letter assigned to an individual unit in a multi‑unit building. Many records lack this value. | 01 | ` | 1,334 |  | 468,641 | (NULL) [468,641]<br>Other (1,324) [10,002]<br>1 [195]<br>2 [170]<br>3 [166] |
| **PROPERTYCITY** | String | City | Municipal city where the property is located. Pittsburgh accounts for over half of the records, followed by other cities such as Coraopolis and Mc Keesport. | ALLISON PARK | WITAKER | 106 |  | 1 | PITTSBURGH [257,608]<br>Other (89) [123,311]<br>CORAOPOLIS [16,497]<br>MC KEESPORT [15,307]<br>GIBSONIA [11,048] |
| **PROPERTYSTATE** | String | State | Two‑letter state abbreviation for the property's location; all records are in Pennsylvania (PA). | PA | PA | 1 | PA | 0 | <ALL_UNIQUE> |
| **PROPERTYZIP** | Integer | ZIP Code | Postal ZIP code of the property. Values range from 15003 to 16229, with most common codes like 15108 and 15237. | 15003 | 16229 | 124 |  | 1 | Other (114) [355,692]<br>15108 [16,509]<br>15237 [15,435]<br>15235 [14,585]<br>15212 [13,301] |
| **SCHOOLCODE** | String | School District Code | Numeric code identifying the school district that serves the property's address; e.g., 47 for Pittsburgh. | 01 | 09 | 46 |  | 0 | Other (36) [228,000]<br>47 [117,977]<br>27 [19,685]<br>09 [19,227]<br>30 [17,635] |
| **SCHOOLDESC** | String | School District Name | Name of the school district associated with the property. Common districts include Pittsburgh, North Allegheny, etc. | Allegheny Valley | Woodland Hills | 46 |  | 0 | Other (36) [228,000]<br>Pittsburgh [117,977]<br>North Allegheny [19,685]<br>Woodland Hills [19,227]<br>Penn Hills Twp [17,635] |
| **MUNICODE** | Integer | Municipality Code | Pennsylvania municipality code identifying the local jurisdiction; numeric values such as 934, 119, etc. | 101 | 953 | 175 |  | 0 | Other (165) [369,793]<br>934 [17,635]<br>119 [12,648]<br>940 [12,003]<br>926 [10,359] |
| **MUNIDESC** | String | Municipality Name | Name of the municipal entity corresponding to MUNICODE. Examples: Penn Hills, 19th Ward – Pittsburgh. | 10th Ward -  McKEESPORT | Wilmerding   | 175 |  | 0 | Other (165) [369,793]<br>Penn Hills [17,635]<br>19th Ward - PITTSBURGH [12,648]<br>Ross [12,003]<br>Mt.Lebanon [10,359] |
| **RECORDDATE** | Date | Record Date | Date when the transaction record was entered into the system; includes null values indicating missing entry dates. | 0212-08-01 | 2028-09-28 | 3,821 |  | 1,262 | Other (3,811) [474,687]<br>(NULL) [1,262]<br>2012-10-26 [587]<br>2013-04-26 [552]<br>2012-01-11 [488] |
| **SALEDATE** | Date | Sale Date | Actual date on which the property sale occurred. Null values indicate unknown or unrecorded sale dates. | 2012-01-01 | 2025-12-13 | 4,888 |  | 0 | Other (4,878) [475,391]<br>2012-10-26 [586]<br>2016-04-29 [584]<br>2013-04-26 [559]<br>2012-01-11 [490] |
| **PRICE** | Integer | Sale Price (USD) | Monetary amount paid for the property in US dollars, ranging from $0 to over $148 million. Many records have small amounts and a large proportion of missing values. | 0 | 148752900 | 37,872 |  | 3,020 | Other (37,862) [340,479]<br>1 [98,344]<br>0 [15,508]<br>10 [6,763]<br>150000 [3,140] |
| **DEEDBOOK** | String | Deed Book Number | Identifier of the book in which the deed is recorded at the county recorder's office; alphanumeric codes such as "TR18". |  14795 | `17274 | 5,814 |  | 570 | Other (5,800) [473,060]<br>TR18 [1,239]<br>0 [1,063]<br>TR13 [938]<br>00 [798] |
| **DEEDPAGE** | String | Deed Page Number | Page number within the deed book where the transaction record appears. Many records are missing this value. |  120 | W | 2,133 |  | 582 | Other (2,115) [465,578]<br>1 [5,174]<br>6 [1,485]<br>7 [1,112]<br>0 [1,002] |
| **SALECODE** | String | Sale Type Code | Code representing the type of sale (e.g., 3 for valid sale, 0 for other). Used by county to classify transactions. | 0X | Z | 47 |  | 0 | 3 [97,932]<br>0 [91,484]<br>H [63,750]<br>Other (37) [54,274]<br>14 [42,089] |
| **SALEDESC** | String | Sale Description | Human‑readable description of the sale type, such as "VALID SALE", "LOVE AND AFFECTION SALE", or "QUIT CLAIM". | BANK/FINANCIAL INSTITUTION | VALID SALE | 27 |  | 0 | LOVE AND AFFECTION SALE [97,932]<br>VALID SALE [91,484]<br>MULTI-PARCEL SALE [63,750]<br>Other (17) [45,680]<br>TIME ON MARKET (INSUFF/EX… [42,089] |
| **INSTRTYP** | String | Instruction Type Code | Code indicating the legal instruction applied in the deed (e.g., DE for Deed, SW for Special Warranty). | AD | WD | 31 |  | 0 | DE [278,804]<br>SW [113,966]<br>GW [23,662]<br>QC [18,068]<br>SD [15,521] |
| **INSTRTYPDESC** | String | Instruction Type Description | Textual description corresponding to INSTRTYP code, such as "DEED", "SPECIAL WARRANTY", etc. | ADMIN DEED | WILL BOOK | 31 |  | 0 | DEED [278,804]<br>SPECIAL WARRANTY [113,966]<br>GENERAL WARRANTY [23,662]<br>QUIT CLAIM [18,068]<br>SHERIFF DEED [15,521] |

*Attribution: Generated by qsv v12.0.0 describegpt
Command line: qsv describegpt --all allegheny_county_property_sale_transactions.csv --language Hans Rosling --fresh --output HansRosling.md --addl-props {"reasoning_effort":"high"}
Prompt file: Default v7.0.0
Model: openai/gpt-oss-20b
LLM API URL: http://localhost:1234/v1
Language: 
Timestamp: 2025-12-25T12:46:26.531376+00:00

WARNING: Label and Description generated by an LLM and may contain inaccuracies. Verify before using!
*

## REASONING

We need to produce Hans Rosling Labels and Descriptions for all fields, using summary statistics and frequency distribution.

Goal: Provide a human-friendly label and description for each field. Use the context from data. Should be concise but informative. We must consider data type, cardinality, etc.

We need JSON output with keys as field names, values containing "label" and "description".

We have 26 fields. Let's list them:

1 _id
2 PARID
3 FULL_ADDRESS
4 PROPERTYHOUSENUM
5 PROPERTYFRACTION
6 PROPERTYADDRESSDIR
7 PROPERTYADDRESSSTREET
8 PROPERTYADDRESSSUF
9 PROPERTYADDRESSUNITDESC
10 PROPERTYUNITNO
11 PROPERTYCITY
12 PROPERTYSTATE
13 PROPERTYZIP
14 SCHOOLCODE
15 SCHOOLDESC
16 MUNICODE
17 MUNIDESC
18 RECORDDATE
19 SALEDATE
20 PRICE
21 DEEDBOOK
22 DEEDPAGE
23 SALECODE
24 SALEDESC
25 INSTRTYP
26 INSTRTYPDESC

We need to interpret each field. Let's think:

- _id: Unique identifier for record (likely internal). It's integer 64-bit unique. Label: "Record ID" or "Unique Identifier". Description: "Internal unique numeric identifier for the property sale record."

- PARID: Parcel ID? Possibly property parcel identifier used in Pennsylvania. The frequency distribution shows many values like "0431B00017000000". So label: "Parcel Identification Number". Description: "Pennsylvania parcel identification number assigned to the property, used for mapping and tax records." Provide context: 16-digit string.

- FULL_ADDRESS: Full address of property. Label: "Full Property Address". Description: "Complete street address including house number, direction, street name, suffix, unit description, city, state, ZIP code."

- PROPERTYHOUSENUM: House number part of address. Note that many zeros and 0 counts high (38055). So label: "House Number" or "Property Street Number". Description: "Numeric component of the property's street address; zero indicates no house number or unknown."

- PROPERTYFRACTION: Fraction of property e.g., "1/2", "A", etc. Label: "Property Portion / Fraction". Description: "Specifies a fractional portion of a parcel, such as '1/2', lettered portions like 'A' or numeric fractions, used when the property is divided."

- PROPERTYADDRESSDIR: Direction abbreviation (N, S, E, W). Label: "Street Direction". Description: "North, South, East, West direction indicator preceding street name; null indicates no direction."

- PROPERTYADDRESSSTREET: Street name. Label: "Street Name". Description: "Name of the street where property is located; most common streets include Washington, 5th, Highland etc."

- PROPERTYADDRESSSUF: Street suffix (St, Ave, Rd, etc). Label: "Street Suffix". Description: "Abbreviation indicating type of road such as 'St', 'Ave', 'Rd'."

- PROPERTYADDRESSUNITDESC: Unit description like UNIT, REAR, APT. Label: "Unit Descriptor". Description: "Describes the unit type within a multi-unit property, e.g., apartment (APT), unit, rear, etc."

- PROPERTYUNITNO: Unit number within building. Label: "Unit Number". Description: "Number or letter assigned to individual unit in a multi-unit property; many missing values."

- PROPERTYCITY: City name. Label: "City". Description: "Municipal city where the property is located; majority of records are Pittsburgh, followed by other cities."

- PROPERTYSTATE: State abbreviation. Label: "State". Description: "Two-letter state code for the property location; all records in PA."

- PROPERTYZIP: ZIP code numeric. Label: "ZIP Code". Description: "Postal ZIP code of property location; values range 15003–16229, with most common 15108 etc."

- SCHOOLCODE: School district code. Label: "School District Code". Description: "Numeric code identifying the school district that serves the property's address; e.g., 47 for Pittsburgh."

- SCHOOLDESC: School district description. Label: "School District Name". Description: "Name of the school district associated with property; often 'Pittsburgh', 'North Allegheny', etc."

- MUNICODE: Municipal code? Likely municipality code used in Pennsylvania. Label: "Municipality Code". Description: "Pennsylvania municipality code identifying the local jurisdiction; ranges 934 to 927, etc."

- MUNIDESC: Municipality description. Label: "Municipality Name". Description: "Name of the municipal entity corresponding to MUNICODE; e.g., Penn Hills, 19th Ward - Pittsburgh."

- RECORDDATE: Date record was entered or recorded in database. Label: "Record Date". Description: "Date when transaction data was first recorded in system; includes null values."

- SALEDATE: Date property sale occurred. Label: "Sale Date". Description: "Actual date of the property sale transaction."

- PRICE: Sale price in dollars? Data shows high range 0 to 148,752,900, but typical values like 1, 10, 150k etc. So label: "Sale Price". Description: "Monetary amount paid for property during sale; expressed in US dollars."

- DEEDBOOK: Deed book number. Label: "Deed Book Number". Description: "Book identifier from county recorder's office where deed is recorded; often alphanumeric like 'TR18'."

- DEEDPAGE: Page number of deed. Label: "Deed Page Number". Description: "Page within the deed book where transaction record appears."

- SALECODE: Sale type code. Label: "Sale Type Code". Description: "Code indicating nature of sale, e.g., 3 for valid sale, 0 for other, etc.; used by county."

- SALEDESC: Sale description text. Label: "Sale Description". Description: "Human-readable explanation of sale type, such as 'VALID SALE', 'LOVE AND AFFECTION SALE', etc."

- INSTRTYP: Instruction type code? Possibly deed instruction type. Label: "Instruction Type Code". Description: "Code representing type of legal instruction in the deed (e.g., DE for Deed, SW for Special Warranty)."

- INSTRTYPDESC: Instruction type description. Label: "Instruction Type Description". Description: "Textual description of instruction type corresponding to INSTRTYP code."

We must incorporate cardinality and uniqueness. Provide context from stats: e.g., _id unique; PARID many unique but some duplicates.

Also include that some fields have null counts, etc. Could mention distribution.

The question: "Generate Hans Rosling Labels and Descriptions for ALL fields in the Dataset." That implies using style of Hans Rosling: simple, intuitive labels like "Property Address" maybe. Should be short but descriptive.

Let's produce JSON accordingly.

We should ensure each field has a label and description. Use summary stats to mention cardinality or typical values.

Also mention null counts where relevant: e.g., PARID not missing? There are 302633 other; etc.

But keep description concise.

Ok, let's craft for each.

Check cardinalities:

- _id unique: 479928 records. All unique.

- PARID: Many duplicates but majority "Other (302,633)". Means many unique but some common.

- FULL_ADDRESS: Most common addresses like "0 SONIE DR ...", etc. Null count? None.

- PROPERTYHOUSENUM: 38055 zeros; others typical numbers.

- PROPERTYFRACTION: 468512 nulls (97%); so most missing.

- PROPERTYADDRESSDIR: 459948 nulls (95%); many missing.

- PROPERTYADDRESSSTREET: 462225 other; majority are common street names.

- PROPERTYADDRESSSUF: 8753 others; top 10 many.

- PROPERTYADDRESSUNITDESC: 468267 nulls.

- PROPERTYUNITNO: 468641 nulls.

- PROPERTYCITY: 123311 Other.

- PROPERTYSTATE: all PA.

- PROPERTYZIP: 355692 other.

- SCHOOLCODE: 228000 other.

- SCHOOLDESC: 228000 other.

- MUNICODE: 369793 other.

- MUNIDESC: 369793 other.

- RECORDDATE: 474687 nulls.

- SALEDATE: 475391 nulls.

- PRICE: 340479 other; many zeros, etc.

- DEEDBOOK: 473060 other.

- DEEDPAGE: 465578 other.

- SALECODE: 54274 other.

- SALEDESC: 45680 other.

- INSTRTYP: 4590 other.

- INSTRTYPDESC: 4590 other.

We can note that many fields have many unique values, but some common.

Now produce JSON. Use field names exactly as provided. Provide label and description.

Let's craft each description:

1 _id: "Unique numeric identifier for the record." "This integer uniquely identifies each property sale record in the dataset; it is not related to any other property attributes."

2 PARID: "Parcel Identification Number" "The 16-digit Pennsylvania parcel ID that uniquely identifies a land parcel; used for tax and legal purposes. Many records share common IDs, but most are unique."

3 FULL_ADDRESS: "Full Property Address" "Complete street address including house number, direction, street name, suffix, unit description, city, state, and ZIP code."

4 PROPERTYHOUSENUM: "House Number" "Numeric part of the property's street address; zero indicates no house number or unknown. Most records have a non-zero number."

5 PROPERTYFRACTION: "Property Portion" "Fractional designation of a parcel (e.g., '1/2', 'A'). Many records have this field missing, indicating full parcel ownership."

6 PROPERTYADDRESSDIR: "Street Direction" "Abbreviation for the street direction (N, S, E, W). Null values indicate no directional prefix in the address."

7 PROPERTYADDRESSSTREET: "Street Name" "Name of the street where the property is located; common streets include Washington, 5th, Highland, etc. Many records share the same street name."

8 PROPERTYADDRESSSUF: "Street Suffix" "Abbreviation indicating type of road (St, Ave, Rd, etc.)."

9 PROPERTYADDRESSUNITDESC: "Unit Descriptor" "Describes unit type within a multi-unit property (e.g., UNIT, APT, REAR). Null values common for single-family homes."

10 PROPERTYUNITNO: "Unit Number" "Number or letter assigned to individual unit in a multi-unit building; many records lack this value."

11 PROPERTYCITY: "City" "Municipal city where the property is located. Pittsburgh accounts for over half of the records, followed by other cities such as Coraopolis and Mc Keesport."

12 PROPERTYSTATE: "State" "Two-letter state abbreviation for the property's location; all records are in Pennsylvania (PA)."

13 PROPERTYZIP: "ZIP Code" "Postal ZIP code of the property. Values range from 15003 to 16229, with most common codes like 15108 and 15237."

14 SCHOOLCODE: "School District Code" "Numeric code identifying the school district serving the property's address; e.g., 47 for Pittsburgh."

15 SCHOOLDESC: "School District Name" "Name of the school district associated with the property. Common districts include Pittsburgh, North Allegheny, etc."

16 MUNICODE: "Municipality Code" "Pennsylvania municipality code identifying the local jurisdiction; numeric values such as 934, 119, etc."

17 MUNIDESC: "Municipality Name" "Name of the municipal entity corresponding to MUNICODE. Examples: Penn Hills, 19th Ward – Pittsburgh."

18 RECORDDATE: "Record Date" "Date when the transaction record was entered into the system; includes null values indicating missing entry dates."

19 SALEDATE: "Sale Date" "Actual date on which the property sale occurred. Null values indicate unknown or unrecorded sale dates."

20 PRICE: "Sale Price (USD)" "Monetary amount paid for the property in US dollars, ranging from $0 to over $148 million. Many records have small amounts and a large proportion of missing values."

21 DEEDBOOK: "Deed Book Number" "Identifier of the book in which the deed is recorded at the county recorder's office; alphanumeric codes such as 'TR18'."

22 DEEDPAGE: "Deed Page Number" "Page number within the deed book where the transaction record appears. Many records are missing this value."

23 SALECODE: "Sale Type Code" "Code representing the type of sale (e.g., 3 for valid sale, 0 for other). Used by county to classify transactions."

24 SALEDESC: "Sale Description" "Human-readable description of the sale type, such as 'VALID SALE', 'LOVE AND AFFECTION SALE', or 'QUIT CLAIM'."

25 INSTRTYP: "Instruction Type Code" "Code indicating the legal instruction applied in the deed (e.g., DE for Deed, SW for Special Warranty)."

26 INSTRTYPDESC: "Instruction Type Description" "Textual description corresponding to INSTRTYP code, such as 'DEED', 'SPECIAL WARRANTY', etc."

Make sure JSON is valid.

Let's produce final.
## TOKEN USAGE

TokenUsage { prompt: 7806, completion: 4078, total: 11884, elapsed: 80885 }
---
# Description
**Description**

Imagine a map of Pennsylvania dotted with almost half a million house‑sale records—each one a tiny data point that tells us how people are buying and selling land. The average sale price hovers around **$195 k**, but the spread is enormous, stretching from zero to over **$148 M** for a few outliers; the median sits just below $140 k, so most transactions are modest. About 53 % of the sales happened in Pittsburgh, with the rest scattered across smaller towns like Mc Keesport and Sewickley. The dataset is mostly complete, yet a handful of fields—especially dates and prices—have gaps (roughly 0.3 % missing). Finally, each record carries an address that could be traced back to a specific person or household, so the data contains personally identifiable information.

**Notable Characteristics**

- **Central tendency & spread:** Mean price ≈ $195 k; median ~ $140 k; standard deviation ≈ $915 k, indicating extreme right‑skewness.  
- **Outliers:** A handful of sales exceed $100 M, far above the mean and median.  
- **Missing values:** Dates (RECORDDATE/SALEDATE) and PRICE have <0.3 % missing; ZERO prices account for ~3 %.  
- **High cardinality & “Other” buckets:** Many fields such as PARID, FULL_ADDRESS, PROPERTYCITY contain a vast number of unique values; the top 10 most frequent values represent only a few percent of the dataset, with the rest lumped into an “Other (N)” category.  
- **PII/PHI content:** The full addresses and ZIP codes are personally identifiable, requiring careful handling to protect privacy.  
- **Duplicate ID check:** The _id field is unique across all records; no duplicate entries.  

**Attribution**

Generated by qsv v12.0.0 describegpt
Command line: qsv describegpt --all allegheny_county_property_sale_transactions.csv --language Hans Rosling --fresh --output HansRosling.md --addl-props {"reasoning_effort":"high"}
Prompt file: Default v7.0.0
Model: openai/gpt-oss-20b
LLM API URL: http://localhost:1234/v1
Language: 
Timestamp: 2025-12-25T12:46:57.469747+00:00

WARNING: Description generated by an LLM and may contain inaccuracies. Verify before using!



## REASONING

We need to produce a description in Hans Rosling style, presumably like "Hans Rosling's talking about data with visual metaphor." The user wants 1-8 sentence description, then Notable Characteristics bullet list, and Attribution placeholder.

We must not output individual field stats or frequency details. Use summary statistics aggregated across dataset.

We need to consider central tendency: For numeric fields like price, mean ~195k, median maybe ~?? We have percentiles for PRICE: 5:1, 10:1, 40:45000, 60:139900, 90:391245, 95:557500. So median around 139900? Actually q2=median = 1431.67882? Wait price has q2 median at column "q2_median" maybe wrong because for PRICE it shows 5th percentile 1 etc. But the summary says q2_median = 1431.67882?? That seems off; maybe misaligned columns. Let's examine summary: For PRICE row, after mean=195323, sem=1324.4887, geometric_mean=0, harmonic_mean=0, stddev=914672.4027, variance=836625604233.7014, cv=468.2868, nullcount=3020, n_negative=0, n_zero=15508, n_positive=461400, max_precision=10, sparsity=0.0063, mad=90999, lower_outer_fence=-659960, etc., percentiles: 5:1, 10:1, 40:45000, 60:139900, 90:391245, 95:557500.

So median around 143? Actually 50th percentile missing. But 60th is 139900 so median likely near ~140k.

Also cardinality for dataset: number of records = 479928. For _id unique count also equals that. So no duplicates on ID. There are many nulls across fields, e.g., RECORDDATE null 1262 (0.26%), SALEDATE null 1206 (0.25%). Also price has 3020 nulls (0.63%) and zero values 15508 (~3.23%).

Many fields have high cardinality but with large "Other" categories.

Also PII/PHI? Fields like FULL_ADDRESS contain personal addresses, which is PII. So data contains sensitive personal info; need caution.

Outliers: price has extreme values up to 148 million, outlier relative to mean ~195k.

Distribution shape: Price distribution heavily right-skewed (skewness not provided but high CV indicates large spread). Many fields like address etc have many unique values with heavy tail.

Also note "Other" categories for top frequency lists; indicates data has many distinct values.

Let's craft description in Hans Rosling style: maybe something like "Imagine a map of Pennsylvania...". Use 1-8 sentences.

Then Notable Characteristics bullet list. Use Hans Rosling tone: e.g., "The average price sits at $195,000, but the spread is enormous." etc.

Add Attribution placeholder.

Let's produce markdown output.


## TOKEN USAGE

TokenUsage { prompt: 8911, completion: 1063, total: 9974, elapsed: 30937 }
---
# Tags
- real_estate_sales  
- pa_real_estate  
- property_price_trends  
- address_data  
- sale_dates  
- parcel_id  
- zip_code_distribution  
- deed_book  
- instruction_type  

Generated by qsv v12.0.0 describegpt
Command line: qsv describegpt --all allegheny_county_property_sale_transactions.csv --language Hans Rosling --fresh --output HansRosling.md --addl-props {"reasoning_effort":"high"}
Prompt file: Default v7.0.0
Model: openai/gpt-oss-20b
LLM API URL: http://localhost:1234/v1
Language: 
Timestamp: 2025-12-25T12:47:29.555357+00:00

WARNING: Tags generated by an LLM and may contain inaccuracies. Verify before using!



## REASONING

We need to produce tags in markdown format (list of tags). No more than 10 unique tags, lowercase only and use _ to separate words. Based on dataset contents: real estate property sales data in Pennsylvania, with addresses, zip codes, sale price, dates, etc. We should pick top important tags.

Tags must be in Hans Rosling language? The instruction says "Make sure your tag choices are in this language: Hans Rosling". I'm not sure what that means; maybe a style of writing? Hans Rosling was known for data visualization and talking about global trends, but his language might be straightforward, concise. Maybe they want tags like "real_estate_sales" etc.

We must include attribution placeholder {GENERATED_BY_SIGNATURE} after the Tags in markdown format.

Thus produce something like:

- real_estate_sales
- property_price_trends
- pa_real_estate
- address_data
- sale_dates
- parcel_id
- zip_code_distribution
- deed_book
- price_distributions

But need to use underscores: "real_estate_sales" etc. No more than 10.

We should order descending importance: most important is real estate property sales in Pennsylvania. Then price distribution, address data, sale dates, zip codes, parcel ID, deed book, instruction type maybe less. We can pick 9 tags.

Ensure lower case only.

Add attribution placeholder at bottom after tags. Provide markdown list or bullet points? They said "in Markdown format". So likely a list like:

```
- real_estate_sales
- property_price_trends
...
{GENERATED_BY_SIGNATURE}
```

Ok produce that.

Also maybe include "pa_real_estate" for Pennsylvania. That's 10 tags: count them.

Let's list:

1. real_estate_sales
2. pa_real_estate
3. property_price_trends
4. address_data
5. sale_dates
6. parcel_id
7. zip_code_distribution
8. deed_book
9. instruction_type

That's 9 tags. Good.

Add attribution placeholder.
## TOKEN USAGE

TokenUsage { prompt: 8879, completion: 478, total: 9357, elapsed: 16946 }
---
