# Dictionary
| Name | Type | Label | Description | Min | Max | Cardinality | Enumeration | Null Count | Examples |
|------|------|-------|-------------|-----|-----|-------------|-------------|------------|----------|
| **_id** | Integer | Unikong ID | Ang _id ay isang natatanging, numerong identifier para sa bawat rekord. Lahat ng halaga nito ay magkakaiba (100% uniqueness) at ginagamit bilang primary key kapag kinokonekta ang datos. | 38703919 | 39183846 | 479,928 |  | 0 | <ALL_UNIQUE> |
| **PARID** | String | Property Reference ID (PARID) | Ang PARID ay isang alfanumerikong code na kumakatawan sa parcel ng lupa. Maraming pare-pareho, ngunit may ilang mga pinaka-madalas tulad ng `0431B00017000000`. May malaking bilang ng “Other” na kategorya dahil sa napakaraming unique values. | 0001C00037000A00 | 9946X83943000000 | 302,643 |  | 0 | Other (302,633) [479,750]<br>0431B00017000000 [23]<br>0027D00263000000 [20]<br>0027D00272000000 [20]<br>0027D00286000000 [20] |
| **FULL_ADDRESS** | String | Buong Address | Naglalaman ito ng kumpletong street address kasama ang house number, direction, suffix at unit description. Marami itong leading zeros (e.g., `0 SONIE DR`). Ang pinaka-madalas ay mga kalsadang nasa Pittsburgh. | 0 , BRADDOCK, PA 15104 | FORBES AVE, PITTSBURGH, PA 15219 | 278,190 |  | 0 | Other (278,180) [479,006]<br>0 SONIE DR, SEWICKLEY, PA… [113]<br>0 COAL, ELIZABETH, PA 150… [111]<br>0 HUNTER ST, PITTSBURGH, … [98]<br>0 PERRYSVILLE AVE, PITTSB… [98] |
| **PROPERTYHOUSENUM** | Integer | Numero ng Bahay | Ang numeric na bahagi ng address. May range mula 0 hanggang 65,015 at ang pinakamaraming house number ay `0` (7.9%). Karamihan sa mga record ay may valid numbers. | 0 | 65015 | 10,012 |  | 3 | Other (10,002) [428,653]<br>0 [38,055]<br>112 [1,615]<br>100 [1,595]<br>110 [1,522] |
| **PROPERTYFRACTION** | String | Fractional Address | Optional na fraction o letter ng property, tulad ng ‘1/2’, ‘A’ o ‘B’. Ang majority (97.6%) ay null; ang ibang values ay mga fraction o letra. |   |  S | 2,803 |  | 0 | (NULL) [468,512]<br>Other (2,793) [9,695]<br>1/2 [853]<br>A [405]<br>B [294] |
| **PROPERTYADDRESSDIR** | String | Direksyon ng Kalsada | Cardinal direction bago ang street name: N, S, E, W. May malawak na distribution ngunit karamihan ay null (95.8%). | E | W | 5 | (NULL)<br>E<br>N<br>S<br>W | 459,948 | (NULL) [459,948]<br>N [5,466]<br>S [5,201]<br>E [5,053]<br>W [4,260] |
| **PROPERTYADDRESSSTREET** | String | Pangalan ng Kalye | Ang pangalan ng kalsada. Ang pinaka-madalas ay ‘WASHINGTON’ at ‘5TH’, ngunit maraming iba pang unique values. | 0 OHIO RIVER BLVD | ZUZU | 9,571 |  | 13 | Other (9,538) [462,225]<br>WASHINGTON [2,606]<br>5TH [2,557]<br>HIGHLAND [1,777]<br>PENN [1,602] |
| **PROPERTYADDRESSSUF** | String | Sufiks ng Kalsada | Street type abbreviation tulad ng ST, DR, AVE. Pinakamadalas ang `ST` (25.6%) at `DR` (23.6%). | ALY | XING | 48 |  | 1,985 | ST [122,764]<br>DR [113,069]<br>AVE [105,232]<br>RD [71,902]<br>LN [15,471] |
| **PROPERTYADDRESSUNITDESC** | String | Deskripsyon ng Yunit | Textual indicator kung anong klase ng unit: UNIT, APT, REAR, STE, BLDG, etc. Karamihan ay null (97.6%). | # | UNIT | 11 |  | 468,267 | (NULL) [468,267]<br>UNIT [10,580]<br>REAR [421]<br>APT [391]<br>STE [132] |
| **PROPERTYUNITNO** | String | Numero ng Yunit | Alfanumerikong identifier ng unit sa loob ng building. Ang majority ay null (97.6%) at may ilang common values tulad ng `1`, `2` o `A`. | 01 | ` | 1,334 |  | 468,641 | (NULL) [468,641]<br>Other (1,324) [10,002]<br>1 [195]<br>2 [170]<br>3 [166] |
| **PROPERTYCITY** | String | Lungsod/Barangay | Nagan ng munisipyo kung saan matatagpuan ang property. Ang majority ay Pittsburgh (53.7%) at may iba pang lungsod sa Pennsylvania. | 15216 | WITAKER | 106 |  | 1 | PITTSBURGH [257,608]<br>Other (89) [123,311]<br>CORAOPOLIS [16,497]<br>MC KEESPORT [15,307]<br>GIBSONIA [11,048] |
| **PROPERTYSTATE** | String | Estado | State code; laging PA para sa dataset na ito. | PA | PA | 1 | PA | 0 | <ALL_UNIQUE> |
| **PROPERTYZIP** | Integer | Zip Code | 5‑digit postal code ng property. Ang pinakamaraming values ay 15108, 15237 at iba pang mga zip codes sa Pittsburgh area. | 15003 | 16229 | 124 |  | 1 | Other (114) [355,692]<br>15108 [16,509]<br>15237 [15,435]<br>15235 [14,585]<br>15212 [13,301] |
| **SCHOOLCODE** | String | Kodigo ng Paaralan | Numeric code na kumakatawan sa school district kung saan matatagpuan ang property. Ang pinaka-madalas ay `47` (24.6%) at iba pang mga district codes. | 01 | 50 | 46 |  | 0 | Other (36) [228,000]<br>47 [117,977]<br>27 [19,685]<br>09 [19,227]<br>30 [17,635] |
| **SCHOOLDESC** | String | Pangalan ng Paaralan | Human‑readable name ng school district, tulad ng ‘Pittsburgh’, ‘North Allegheny’ atbp. | Allegheny Valley | Woodland Hills | 46 |  | 0 | Other (36) [228,000]<br>Pittsburgh [117,977]<br>North Allegheny [19,685]<br>Woodland Hills [19,227]<br>Penn Hills Twp [17,635] |
| **MUNICODE** | Integer | Municipality Code | Alfanumerikong code na kumakatawan sa local government unit. Pinakamadalas ang `934` (3.7%). | 101 | 953 | 175 |  | 0 | Other (165) [369,793]<br>934 [17,635]<br>119 [12,648]<br>940 [12,003]<br>926 [10,359] |
| **MUNIDESC** | String | Pangalan ng Munisipyo | Full name ng munisipyo, halimbawa ‘Penn Hills’ o ‘19th Ward - PITTSBURGH’. May malaking bilang ng unique values. | 10th Ward -  McKEESPORT | Wilmerding   | 175 |  | 0 | Other (165) [369,793]<br>Penn Hills [17,635]<br>19th Ward - PITTSBURGH [12,648]<br>Ross [12,003]<br>Mt.Lebanon [10,359] |
| **RECORDDATE** | Date | Petsa ng Record | Date kung kailan naitala ang record sa system. Marami itong null (0.26%) at karamihan ay nasa 2012‑2025. | 0212-08-01 | 2028-09-28 | 3,821 |  | 1,262 | Other (3,811) [474,687]<br>(NULL) [1,262]<br>2012-10-26 [587]<br>2013-04-26 [552]<br>2012-01-11 [488] |
| **SALEDATE** | Date | Petsa ng Benta | Actual transaction date ng property sale. May ilang missing values, ngunit karaniwang nasa parehong saklaw ng RECORDDATE. | 2012-01-01 | 2025-12-13 | 4,888 |  | 0 | Other (4,878) [475,391]<br>2012-10-26 [586]<br>2016-04-29 [584]<br>2013-04-26 [559]<br>2012-01-11 [490] |
| **PRICE** | Integer | Presyo | Bilang na nagpapakita ng halaga ng benta sa US dollars. Kabilang dito ang zero at null entries; ang average ay humigit-kumulang $195k. | 0 | 148752900 | 37,872 |  | 3,020 | Other (37,862) [340,479]<br>1 [98,344]<br>0 [15,508]<br>10 [6,763]<br>150000 [3,140] |
| **DEEDBOOK** | String | Libro ng Deed | Alfanumerikong identifier para sa deed book (e.g., `TR18`, `0`). May “Other” category na sumasaklaw sa karamihan. |  14795 | `17274 | 5,814 |  | 570 | Other (5,800) [473,060]<br>TR18 [1,239]<br>0 [1,063]<br>TR13 [938]<br>00 [798] |
| **DEEDPAGE** | String | Pahina ng Deed | Identifier ng page number within the deed book. Karamihan ay numeric (1‑552) at may “Other” na naglalaman ng maraming entries. |  120 | W | 2,133 |  | 582 | Other (2,115) [465,578]<br>1 [5,174]<br>6 [1,485]<br>7 [1,112]<br>0 [1,002] |
| **SALECODE** | String | Kodigo ng Benta | Short code na kumakatawan sa uri ng sale, tulad ng `3`, `0`, `H`. Ang pinaka-madalas ay `3` at `0` (20%+). | 0 | Z | 47 |  | 0 | 3 [97,932]<br>0 [91,484]<br>H [63,750]<br>Other (37) [54,274]<br>14 [42,089] |
| **SALEDESC** | String | Deskripsyon ng Benta | Human‑readable description ng transaction type, halimbawa ‘LOVE AND AFFECTION SALE’, ‘VALID SALE’, o ‘QUIT CLAIM’. | BANK/FINANCIAL INSTITUTION | VALID SALE | 27 |  | 0 | LOVE AND AFFECTION SALE [97,932]<br>VALID SALE [91,484]<br>MULTI-PARCEL SALE [63,750]<br>Other (17) [45,680]<br>TIME ON MARKET (INSUFF/EX… [42,089] |
| **INSTRTYP** | String | Kodigo ng Instrumento | Code na nagpapakita kung anong uri ng legal instrument ang ginamit: DE (Deed), SW (Special Warranty), GW (General Warranty), atbp. Ang DE ay pinakapopular. | AD | WD | 31 |  | 0 | DE [278,804]<br>SW [113,966]<br>GW [23,662]<br>QC [18,068]<br>SD [15,521] |
| **INSTRTYPDESC** | String | Deskripsyon ng Instrumento | Full description ng instrument type, tulad ng ‘DEED’, ‘SPECIAL WARRANTY’, ‘GENERAL WARRANTY’, ‘QUIT CLAIM’ at iba pa. | ADMIN DEED | WILL BOOK | 31 |  | 0 | DEED [278,804]<br>SPECIAL WARRANTY [113,966]<br>GENERAL WARRANTY [23,662]<br>QUIT CLAIM [18,068]<br>SHERIFF DEED [15,521] |

*Attribution: Generated by qsv v12.0.0 describegpt
Command line: qsv describegpt --all allegheny_county_property_sale_transactions.csv --language Taglish -o Taglish.md --fresh
Prompt file: Default v7.0.0
Model: openai/gpt-oss-20b
LLM API URL: http://localhost:1234/v1
Language: 
Timestamp: 2025-12-25T22:26:58.932324+00:00

WARNING: Label and Description generated by an LLM and may contain inaccuracies. Verify before using!
*

## REASONING

We need to generate Taglish Labels and Descriptions for all fields using summary stats and frequency distribution.

Taglish: mix of Tagalog/English? Probably labels in Filipino + English, e.g., "Unique Key" maybe "Kumunikadong Susi". But we can keep simple but mix. Provide human-friendly label, description in Taglish (some Tagalog words). Use context from stats and freq. For each field.

We need JSON output: keys are field names exactly as given? They use underscores. Should preserve.

We should produce for 26 fields.

Use summary statistics to infer types, ranges, uniqueness etc.

Let's craft labels:

_id: "Unikong ID" or "Unique Identifier". description: unique numeric identifier per record; no duplicates; all values distinct.

PARID: property address reference id? from freq shows many with prefix. label: "Property Reference ID (PARID)". description: alphanumeric code linking to property parcel records.

FULL_ADDRESS: full address string. label: "Buong Address" or "Full Address". description: complete street address including number, direction, suffix, unit etc; some missing leading numbers like '0'.

PROPERTYHOUSENUM: house number. label: "Numero ng Bahay". description: numeric part of property address; includes zero for some addresses.

PROPERTYFRACTION: fractional part. label: "Fractional Address". description: optional fraction of a lot or split (e.g., 1/2, A, B). Many nulls.

PROPERTYADDRESSDIR: direction (N,S,E,W). label: "Direksyon ng Kalsada". description: cardinal direction preceding street name.

PROPERTYADDRESSSTREET: street name. label: "Pangalan ng Kalye". description: common street names; many unique values but top 10 listed.

PROPERTYADDRESSSUF: suffix like ST, DR. label: "Sufiks ng Kalsada". description: abbreviation of street type.

PROPERTYADDRESSUNITDESC: unit descriptor (UNIT, APT). label: "Deskripsyon ng Yunit". description: textual indicator for apartment or unit types; many nulls.

PROPERTYUNITNO: unit number. label: "Numero ng Yunit". description: alphanumeric identifier within building.

PROPERTYCITY: city/town. label: "Lungsod/Barangay". description: municipality name; majority Pittsburgh.

PROPERTYSTATE: state code; always PA. label: "Estado".

PROPERTYZIP: zip code. label: "Zip Code". description: 5-digit postal code.

SCHOOLCODE: school district code. label: "Kodigo ng Paaralan". description: numeric code representing local school district; many values but top ones listed.

SCHOOLDESC: school district name. label: "Pangalan ng Paaralan".

MUNICODE: municipality code. label: "Municipality Code". description: numeric code for the local government unit.

MUNIDESC: municipality description/name. label: "Lungsod/Town Name" or "Municipalidad".

RECORDDATE: date record entered. label: "Petsa ng Record". description: date when transaction was recorded in system; many nulls.

SALEDATE: sale date. label: "Petsa ng Benta". description: actual transaction date; some missing.

PRICE: price in dollars. label: "Presyo". description: sale amount; includes zero and null values.

DEEDBOOK: book number. label: "Libro ng Deed". description: alphanumeric code for deed book; many other codes.

DEEDPAGE: page number. label: "Pahina ng Deed". description: numeric or alphanumeric page identifier within the book.

SALECODE: sale type code. label: "Kodigo ng Benta". description: shorthand classification of sale (e.g., 3,0,H). Provide meaning? Could mention typical codes.

SALEDESC: sale description text. label: "Deskripsyon ng Benta". description: human-readable description; includes LOVE AND AFFECTION SALE etc.

INSTRTYP: instrument type code. label: "Kodigo ng Instrumento". description: code indicating legal instrument (DE, SW...). 

INSTRTYPDESC: instrument type description. label: "Deskripsyon ng Instrumento". description: full description like DEED, SPECIAL WARRANTY etc.

We need Taglish mix; use some Filipino words but keep English where needed for clarity.

Return JSON object with each field mapping to {label,description}.

Let's craft descriptions using context: e.g., mention uniqueness ratio from summary stats. Use cardinality info: id unique; parid many duplicates (top 10). Provide ranges: house number range etc.

We should also mention that some fields have null counts. Eg FULL_ADDRESS has many leading zeros. propertyaddressdir mostly N,S,E,W but with high proportion of nulls.

Ok produce final JSON. Ensure proper escaping, no trailing commas. Use double quotes. We'll keep description in Taglish but still comprehensible.

Let's craft each.

We should include "Taglish" meaning mixing Filipino and English; e.g., "Kumunikadong Susi" or "Unique Identifier". Might be okay to use mixture.

Let's produce final JSON.
## TOKEN USAGE

TokenUsage { prompt: 7881, completion: 2638, total: 10519, elapsed: 49426 }
---
# Description
**Dataset Description (Taglish)**  
Ang dataset na ito ay binubuo ng halos 480 k records ng property sales sa Pennsylvania mula 2012 hanggang 2025.  Kabilang dito ang mga parcel identifiers, buong address, sale dates, presyo at iba't‑ibang klase ng deed o instrument.  Ang “_id” field ay nag‑representa ng natatanging key para sa bawat entry, kaya walang duplicate na row.  Ang presyo ay mula zero hanggang humigit-kumulang $149 million, ngunit halos lahat ng values ay nasa range ng ilang daang libo, at ang distribusyon nito ay napakailalim‑right‑skewed na may maraming outliers.

**Notable Characteristics**

- **High cardinality & uniqueness** – ang `_id` ay 100% unique; maraming fields (e.g., PARID, FULL_ADDRESS) ay “Other” sa top‑10 distribution (99.9%+).
- **Price spread** – mean na ~US $195k ngunit CV > 4, ibig sabihin napakaraming variation at ilang outlier values.
- **Missingness** – RECORDDATE may 0.26% nulls; PRICE 0.63%; maraming address components (direction, fraction, unit description) ay null sa higit 95–98% ng records.
- **Skewed categorical fields** – SALECODE, SALEDESC, INSTRTYP at INSTRTYPDESC dominated by a few top values but majority “Other” (e.g., sale codes: 3 & 0 together ~39 % only).
- **Zip code distribution** – 74% ng ZIPs fall into “Other”, indicating many unique or rarely‑seen postal codes.
- **PII/PHI considerations** – ang dataset ay naglalaman ng mga address at zip code na maaaring i‑classify bilang PII, ngunit walang personal identifiers tulad ng pangalan o social security number.  Walang PCI data dito.
- **Temporal gaps** – RECORDDATE at SALEDATE may “Other (3,811)” or “Other (4,878)” na nagrerepresenta ng hindi kilalang petsa; ito ay maaaring mag‑indicate ng incomplete date entries.
- **Instrument type balance** – DE (deed) accounts for ~58 % of transactions, whereas SW and GW combined at ~28 %; may indikasyon ng predominance ng simple deeds.

> *Attribution: Generated by qsv v12.0.0 describegpt
Command line: qsv describegpt --all allegheny_county_property_sale_transactions.csv --language Taglish -o Taglish.md --fresh
Prompt file: Default v7.0.0
Model: openai/gpt-oss-20b
LLM API URL: http://localhost:1234/v1
Language: 
Timestamp: 2025-12-25T22:27:26.900068+00:00

WARNING: Description generated by an LLM and may contain inaccuracies. Verify before using!
*


## REASONING

We need to produce a Taglish description of dataset as whole, 1-8 sentences. Then Notable Characteristics bullet list, all Taglish. Use data from summary and freq. Avoid field-specific details. Provide overall stats: number records 479928. Data about property sales in PA, 2012-2025. Many missing values (e.g., RECORDDATE null 1262). High cardinality for IDs unique. Skewness of PRICE etc. Outliers: price has high variance and skewed distribution. Some fields have many 'Other' categories. Also mention duplicates? Uniqueness ratio 1 for ID. PII/PHI? Contains addresses, zip codes, but no names. Provide attribution.

Let's craft description:

- Data set comprises nearly 480k property transaction records in Pennsylvania from 2012-2025.
- Contains address components, parcel IDs, sale dates, prices, instrument types etc.
- Prices range from zero to ~149M with heavy right skew and outliers; mean around $195k but median lower? Use percentiles: 50th percentile at 19194240431? Wait weird. Actually price stats: min 0 max 148,752,900, mean 195k. So heavy skew.
- High missingness in fields like RECORDDATE (~0.26%), PRICE null (0.63%).
- Many categorical fields dominated by few top values but majority "Other" categories e.g., PARID other 99.96%.
- Address components: majority null for direction, fraction etc.

Notable characteristics bullets:

- Unique ID per record; no duplicates.
- Skewed price distribution with long tail; high variance and coefficient of variation ~0.44? Actually cv length etc. Use given stats: price stddev 914672.4 mean 195323 => CV ~4.68 >1 indicates high spread.
- Missing values in many fields, especially RECORDDATE (1262 nulls), PRICE (3020 nulls).
- Many 'Other' categories for categorical fields; top 10 not representing majority.
- Addresses mostly missing direction/fraction/unitdesc; only 5% have unit description etc.
- ZIP code distribution: 74% "Other" meaning many unique zips.
- PHI/PII: addresses, zip codes considered PII but no personal names. Not PCI.

Let's craft final output.
## TOKEN USAGE

TokenUsage { prompt: 9267, completion: 1012, total: 10279, elapsed: 27966 }
---
# Tags
**Tags**

- property_sales  
- pittsburgh_real_estate  
- pa_real_estate  
- address_data  
- price_analysis  
- deed_records  
- municipal_info  
- sale_history  
- real_estate_market  
- data_quality  

Generated by qsv v12.0.0 describegpt
Command line: qsv describegpt --all allegheny_county_property_sale_transactions.csv --language Taglish -o Taglish.md --fresh
Prompt file: Default v7.0.0
Model: openai/gpt-oss-20b
LLM API URL: http://localhost:1234/v1
Language: 
Timestamp: 2025-12-25T22:27:57.354832+00:00

WARNING: Tags generated by an LLM and may contain inaccuracies. Verify before using!



## REASONING

We need to choose no more than 10 unique tags in Markdown format. They must be Taglish words, lowercase, underscore separated. Use dataset content: real estate property transactions in Pittsburgh PA area. Tags based on domain: real_estate, property_sales, Pennsylvania, Pittsburgh, address_data, transaction_history, price_analysis, deed_records, municipal_info, data_quality maybe.

But ensure tags are relevant to dataset; no field names. Taglish style e.g., "real_estate", "property_sales" but Taglish would be like "real_estate". Actually Taglish uses Taglish words: mix of Tagalog and English? Eg "data_pangkalakal"? But simpler: "real_estate" is okay as Taglish because it's combination of English word with underscore. Might want "pittsburgh_property", "pa_real_estate", "sale_transactions".

We need descending importance based on summary stats and freq distribution. Highest cardinality unique values? ID all unique, so uniqueness high but not helpful for tags. The dataset's main content: property sales data, location in Pennsylvania (Pittsburgh), addresses, price info, sale dates, deed types.

So top tags: "property_sales", "pittsburgh_real_estate", "pa_real_estate", "address_data", "price_analysis", "deed_records", "municipal_info", "sale_history", "real_estate_market".

But limit 10. Let's pick:

1. property_sales
2. pittsburgh_real_estate
3. pa_real_estate
4. address_data
5. price_analysis
6. deed_records
7. municipal_info
8. sale_history
9. real_estate_market
10. data_quality

But ensure Taglish style: maybe "property_sales" fine, but "pittsburgh_real_estate" fine.

Now output as Markdown list? They said tags in Markdown format, no more than 10 unique tags. Probably a bullet list or space separated line? Just tags separated by commas? Provide Attribution placeholder after tags. Use markdown code block? The instruction: "Add an Attribution with the placeholder "{GENERATED_BY_SIGNATURE}" after the Tags. If generating JSON format, add the Attribution as a separate key at the top level of the JSON object, after the Tags, otherwise add it at the bottom of the Tags in Markdown format."

Thus we produce tags as markdown list or line and then attribution on next line.

Let's produce:

- property_sales
- pittsburgh_real_estate
- pa_real_estate
- address_data
- price_analysis
- deed_records
- municipal_info
- sale_history
- real_estate_market
- data_quality

Then a line: `{GENERATED_BY_SIGNATURE}`

That satisfies.
## TOKEN USAGE

TokenUsage { prompt: 9236, completion: 629, total: 9865, elapsed: 16756 }
---
