# Dictionary
| Name | Type | Label | Description | Min | Max | Cardinality | Enumeration | Null Count | Examples |
|------|------|-------|-------------|-----|-----|-------------|-------------|------------|----------|
| **_id** | Integer | Identifiant Unique | Clé primaire interne générée par la base de données (type Integer). Chaque valeur est unique ; cardinalité = 479 928 et ratio d’unicité = 1.0, ce qui le rend idéal pour référencer un enregistrement précis. | 38703919 | 39183846 | 479,928 |  | 0 | <ALL_UNIQUE> |
| **PARID** | String | Identifiant Parcelle (PARID) | Code alphanumérique d’identification de la parcelle immobilière. La distribution montre qu’un petit nombre de valeurs sont récurrentes (ex : 0431B00017000000, 0027D00263000000) tandis que la majorité des enregistrements appartiennent à la catégorie « Other » indiquant des codes uniques ou rares. | 0001C00037000A00 | 9946X83943000000 | 302,643 |  | 0 | Other (302,633) [479,750]<br>0431B00017000000 [23]<br>0027D00263000000 [20]<br>0027D00272000000 [20]<br>0027D00286000000 [20] |
| **FULL_ADDRESS** | String | Adresse Complète | Chaîne qui combine numéro, rue, ville, état et code postal. La fréquence montre quelques adresses courantes (ex : 0 SONIE DR, SEWICKLEY, PA 15143) mais plus de 99 % des enregistrements sont uniques, reflétant la granularité élevée du champ. | 0 , BRADDOCK, PA 15104 | FORBES AVE, PITTSBURGH, PA 15219 | 278,190 |  | 0 | Other (278,180) [479,006]<br>0 SONIE DR, SEWICKLEY, PA… [113]<br>0 COAL, ELIZABETH, PA 150… [111]<br>0 HUNTER ST, PITTSBURGH, … [98]<br>0 PERRYSVILLE AVE, PITTSB… [98] |
| **PROPERTYHOUSENUM** | Integer | Numéro de Maison | Valeur numérique indiquant le numéro principal d’une propriété. Le nombre 0 est le plus fréquent (≈ 7 900 occurrences) suivi par 112, 100, 110, etc. Une grande partie du champ se trouve dans la catégorie « Other » (≈ 89 %), signifiant que de nombreux numéros sont uniques ou peu répandus. | 0 | 65015 | 10,012 |  | 3 | Other (10,002) [428,653]<br>0 [38,055]<br>112 [1,615]<br>100 [1,595]<br>110 [1,522] |
| **PROPERTYFRACTION** | String | Fraction d'Adresse | Partie fractionnaire du numéro de maison (ex : 1/2, A). La majorité des valeurs sont nulles (≈ 97.6 %) ; les fractions courantes incluent 1/2, A, B, etc., représentant la quasi‑majorité des enregistrements non nuls. |   |  S | 2,803 |  | 0 | (NULL) [468,512]<br>Other (2,793) [9,695]<br>1/2 [853]<br>A [405]<br>B [294] |
| **PROPERTYADDRESSDIR** | String | Direction de Rue | Orientation cardinale associée à l’adresse (N, S, E, W). Environ 95 % des lignes sont vides. Les valeurs N et S représentent chacune ≈ 1 % du total, tandis que E et W sont moins fréquents. | E | W | 5 | (NULL)<br>E<br>N<br>S<br>W | 459,948 | (NULL) [459,948]<br>N [5,466]<br>S [5,201]<br>E [5,053]<br>W [4,260] |
| **PROPERTYADDRESSSTREET** | String | Nom de Rue | Nom complet de la rue (ex : WASHINGTON, 5TH). Les dix premiers noms couvrent ≈ 4 % du total; le reste est dispersé sur plus de 9 000 valeurs différentes, ce qui donne une forte cardinalité et un ratio d’unicité élevé. | 0 OHIO RIVER BLVD | ZUZU | 9,571 |  | 13 | Other (9,538) [462,225]<br>WASHINGTON [2,606]<br>5TH [2,557]<br>HIGHLAND [1,777]<br>PENN [1,602] |
| **PROPERTYADDRESSSUF** | String | Suffixe de Rue | Type d’itinéraire (ST, DR, AVE, etc.). Les suffixes ST, DR et AVE représentent plus de 70 % des valeurs non nulles. La catégorie « Other » couvre ≈ 1.8 % du total. | ALY | XING | 48 |  | 1,985 | ST [122,764]<br>DR [113,069]<br>AVE [105,232]<br>RD [71,902]<br>LN [15,471] |
| **PROPERTYADDRESSUNITDESC** | String | Description d'Unité | Texte décrivant la composante unité (UNIT, APT, REAR). Environ 97.6 % des lignes sont vides ; les valeurs UNIT et APT représentent respectivement ≈ 2 % et 0.08 % du total. | # | UNIT | 11 |  | 468,267 | (NULL) [468,267]<br>UNIT [10,580]<br>REAR [421]<br>APT [391]<br>STE [132] |
| **PROPERTYUNITNO** | String | Numéro d'Unité | Identifiant (numérique ou alphanumérique) de l’unité à l’intérieur de la propriété. Les chiffres 1, 2, 3 sont les plus fréquents mais la plupart des valeurs restent uniques. | 01 | ` | 1,334 |  | 468,641 | (NULL) [468,641]<br>Other (1,324) [10,002]<br>1 [195]<br>2 [170]<br>3 [166] |
| **PROPERTYCITY** | String | Ville | Nom de la ville où se situe la propriété. Pittsburgh domine avec 53.7 % du total; d’autres villes comme Coraopolis, Mc Keesport, Gibsonia apparaissent moins fréquemment. Un grand nombre de valeurs restent uniques (≈ 25 %). | 15216 | WITAKER | 106 |  | 1 | PITTSBURGH [257,608]<br>Other (89) [123,311]<br>CORAOPOLIS [16,497]<br>MC KEESPORT [15,307]<br>GIBSONIA [11,048] |
| **PROPERTYSTATE** | String | État | Code ISO de l’État (PA). Toutes les entrées sont « PA », ce qui rend le champ entièrement homogène. | PA | PA | 1 | PA | 0 | <ALL_UNIQUE> |
| **PROPERTYZIP** | Integer | Code Postal | Code postal (5 chiffres) de la propriété. Les codes 15108, 15237, 15235 sont les plus fréquents, mais 74 % des valeurs sont uniques, indiquant une grande dispersion. | 15003 | 16229 | 124 |  | 1 | Other (114) [355,692]<br>15108 [16,509]<br>15237 [15,435]<br>15235 [14,585]<br>15212 [13,301] |
| **SCHOOLCODE** | String | Code d'École | Numéro de district scolaire (ex : 47). Le code 47 représente ≈ 24.6 % du total ; les autres codes couvrent le reste, avec une forte diversité. | 01 | 50 | 46 |  | 0 | Other (36) [228,000]<br>47 [117,977]<br>27 [19,685]<br>09 [19,227]<br>30 [17,635] |
| **SCHOOLDESC** | String | Description du District Scolaire | Nom complet du district (Pittsburgh, North Allegheny, etc.). Les dix premiers noms constituent ≈ 24.6 % des enregistrements ; la majorité sont uniques. | Allegheny Valley | Woodland Hills | 46 |  | 0 | Other (36) [228,000]<br>Pittsburgh [117,977]<br>North Allegheny [19,685]<br>Woodland Hills [19,227]<br>Penn Hills Twp [17,635] |
| **MUNICODE** | Integer | Code Municipal | Identifiant numérique de la municipalité (ex : 934). Le code 934 représente ≈ 3.7 % du total, tandis que les autres codes couvrent le reste ; un grand nombre d’enregistrements restent uniques. | 101 | 953 | 175 |  | 0 | Other (165) [369,793]<br>934 [17,635]<br>119 [12,648]<br>940 [12,003]<br>926 [10,359] |
| **MUNIDESC** | String | Description Municipale | Nom ou désignation de la municipalité (Penn Hills, 19th Ward, etc.). Les dix premiers noms représentent ≈ 3.7 % du total; le reste est très dispersé. | 10th Ward -  McKEESPORT | Wilmerding   | 175 |  | 0 | Other (165) [369,793]<br>Penn Hills [17,635]<br>19th Ward - PITTSBURGH [12,648]<br>Ross [12,003]<br>Mt.Lebanon [10,359] |
| **RECORDDATE** | Date | Date d'enregistrement | Date à laquelle l’enregistrement a été créé ou ajouté (format YYYY‑MM‑DD). Les dates les plus fréquentes sont en 2012 et 2016. Environ 99 % des lignes contiennent une valeur; seulement 0.26 % sont nulles. | 0212-08-01 | 2028-09-28 | 3,821 |  | 1,262 | Other (3,811) [474,687]<br>(NULL) [1,262]<br>2012-10-26 [587]<br>2013-04-26 [552]<br>2012-01-11 [488] |
| **SALEDATE** | Date | Date de Vente | Date à laquelle la propriété a été vendue. Les dates les plus fréquentes apparaissent en 2016 et 2012. La majorité (≈ 99 %) des lignes contiennent une valeur valide. | 2012-01-01 | 2025-12-13 | 4,888 |  | 0 | Other (4,878) [475,391]<br>2012-10-26 [586]<br>2016-04-29 [584]<br>2013-04-26 [559]<br>2012-01-11 [490] |
| **PRICE** | Integer | Prix de Vente | Montant de la vente exprimé en dollars (entier). Les valeurs les plus fréquentes sont 1, 0 et 10. Un grand nombre d’enregistrements ont un prix nul ou appartiennent à la catégorie « Other » (~71 %), indiquant que beaucoup de transactions ne disposent pas d’un montant précis. | 0 | 148752900 | 37,872 |  | 3,020 | Other (37,862) [340,479]<br>1 [98,344]<br>0 [15,508]<br>10 [6,763]<br>150000 [3,140] |
| **DEEDBOOK** | String | Livre du Acte | Identifiant du livre où le titre de propriété est enregistré (ex : TR18, 0). La valeur TR18 est la plus fréquente. Une majorité des livres sont uniques ou peu répandus. |  14795 | `17274 | 5,814 |  | 570 | Other (5,800) [473,060]<br>TR18 [1,239]<br>0 [1,063]<br>TR13 [938]<br>00 [798] |
| **DEEDPAGE** | String | Page du Livre | Numéro de page correspondant à l’acte dans le livre. La page 1 est la plus courante, suivie par les pages 6, 7 et 0. Le champ contient peu de valeurs nulles. |  120 | W | 2,133 |  | 582 | Other (2,115) [465,578]<br>1 [5,174]<br>6 [1,485]<br>7 [1,112]<br>0 [1,002] |
| **SALECODE** | String | Code de Vente | Code indiquant le type de transaction (ex : 3, 0, H). Le code « 3 » est le plus fréquent (~20 % du total), suivi par 0 et H. Les codes représentent des catégories légales ou administratives. | 0 | Z | 47 |  | 0 | 3 [97,932]<br>0 [91,484]<br>H [63,750]<br>Other (37) [54,274]<br>14 [42,089] |
| **SALEDESC** | String | Description de la Vente | Texte détaillant le type d’opération (ex : LOVE AND AFFECTION SALE, VALID SALE). Le premier texte couvre ~20 % du total. Les descriptions varient entre ventes légitimes, transactions de sauvegarde et autres. | BANK/FINANCIAL INSTITUTION | VALID SALE | 27 |  | 0 | LOVE AND AFFECTION SALE [97,932]<br>VALID SALE [91,484]<br>MULTI-PARCEL SALE [63,750]<br>Other (17) [45,680]<br>TIME ON MARKET (INSUFF/EX… [42,089] |
| **INSTRTYP** | String | Type d'Instrument | Code court du type d’acte (DE = deed, SW = special warranty, etc.). Le code DE est le plus fréquent (~58 %). Les codes représentent les différents types de titres ou garanties. | AD | WD | 31 |  | 0 | DE [278,804]<br>SW [113,966]<br>GW [23,662]<br>QC [18,068]<br>SD [15,521] |
| **INSTRTYPDESC** | String | Description du Type d'Instrument | Texte complet décrivant le type d’acte (DEED, SPECIAL WARRANTY, GENERAL WARRANTY, etc.). La description DEED correspond au code DE et est la plus courante (~58 %). | ADMIN DEED | WILL BOOK | 31 |  | 0 | DEED [278,804]<br>SPECIAL WARRANTY [113,966]<br>GENERAL WARRANTY [23,662]<br>QUIT CLAIM [18,068]<br>SHERIFF DEED [15,521] |

*Attribution: Generated by qsv v12.0.0 describegpt
Command line: qsv describegpt --all allegheny_county_property_sale_transactions.csv --language Franglais -o Franglais.md --fresh
Prompt file: Default v7.0.0
Model: openai/gpt-oss-20b
LLM API URL: http://localhost:1234/v1
Language: 
Timestamp: 2025-12-26T00:20:14.593902+00:00

WARNING: Label and Description generated by an LLM and may contain inaccuracies. Verify before using!
*

## REASONING

We need to generate Franglais labels and descriptions for all 26 fields. Use summary statistics and frequency distribution to understand context. Provide label human-friendly; description full explanation. Output JSON with field names as keys. Should be in French/English mix (Franglais). We must think step by step.

Let's consider each field:

1. _id: unique identifier, internal. Label: "Identifiant Unique". Description: "Clé primaire interne générée par la base de données, utilisée pour référencer un enregistrement unique."

2. PARID: property parcel ID maybe? Provide label: "Identifiant Parcelle (PARID)". Description: "Code d'identification du lot immobilier, format alphanumérique; plusieurs valeurs courantes indiquées dans la distribution; 'Other' représente la majorité."

3. FULL_ADDRESS: full address string. Label: "Adresse Complète". Description: "Combinaison complète de l'adresse (numéro, rue, ville, état, code postal). Les données incluent des valeurs fréquentes comme '0 SONIE DR...' ; un grand nombre d'adresses uniques."

4. PROPERTYHOUSENUM: house number field. Label: "Numéro de Maison". Description: "Numéro de propriété; 0 est le plus fréquent (7,9k occurrences), suivi par 112,100 etc. La majorité des valeurs sont uniques ou regroupées dans 'Other'."

5. PROPERTYFRACTION: fraction part of address (e.g., 1/2). Label: "Fraction d'Adresse". Description: "Partie fractionnaire du numéro de maison; la plupart sont nulles (~97 %) ; les fractions courantes incluent '1/2', 'A', 'B', etc."

6. PROPERTYADDRESSDIR: direction of address (N,S,E,W). Label: "Direction de Rue". Description: "Indique la direction cardinale pour l'adresse, majoritairement N et S (~1 % chacun), avec un grand nombre d'enregistrements vides."

7. PROPERTYADDRESSSTREET: street name. Label: "Nom de Rue". Description: "Le nom de la rue; 'WASHINGTON', '5TH' sont les plus fréquents. La plupart des rues sont uniques (96 %)."

8. PROPERTYADDRESSSUF: suffixe de rue (St, Dr). Label: "Suffixe de Rue". Description: "Terminus de l'adresse indiquant le type d’itinéraire; 'ST', 'DR', 'AVE' dominent."

9. PROPERTYADDRESSUNITDESC: unit description (Unit, Apt). Label: "Description d'unité". Description: "Texte décrivant la composante unité de l’adresse; 'UNIT', 'REAR', 'APT' les plus fréquents; la plupart sont vides."

10. PROPERTYUNITNO: unit number. Label: "Numéro d'Unité". Description: "Identifiant numérique ou alphanumérique pour l'unité; 1,2,3 etc.; majorité des valeurs sont uniques."

11. PROPERTYCITY: city. Label: "Ville". Description: "Nom de la ville où se situe la propriété; Pittsburgh majoritaire (~54 %) ; d’autres villes listées; un grand nombre d'enregistrements uniques."

12. PROPERTYSTATE: state. Label: "État". Description: "Code d’état (PA). Toutes les propriétés sont dans l'État du Pennsylvanie."

13. PROPERTYZIP: zip code. Label: "Code Postal". Description: "Code postal de la propriété; 15108,15237 etc.; la plupart des codes sont uniques (74 %)."

14. SCHOOLCODE: school district code? Label: "Code d'École". Description: "Code numérique pour le district scolaire; '47' est le plus courant. Un grand nombre de valeurs différentes."

15. SCHOOLDESC: description of school district. Label: "Description du District Scolaire". Description: "Nom complet du district; 'Pittsburgh', 'North Allegheny', etc.; la majorité des enregistrements sont uniques."

16. MUNICODE: municipal code? Label: "Code Municipal". Description: "Numéro d'identification de la municipalité ; 934,119 etc.; majoritairement unique (77 %)."

17. MUNIDESC: description of municipality. Label: "Description Municipale". Description: "Nom ou désignation du secteur municipal; 'Penn Hills', '19th Ward' etc."

18. RECORDDATE: date of record creation? Label: "Date d'enregistrement". Description: "Date de création/ajout de l’enregistrement ; valeurs varient 2012-2028 ; grande majorité non nulles (98 %)."

19. SALEDATE: date of sale. Label: "Date de Vente". Description: "Date à laquelle la propriété a été vendue; dates communes 2016-04-29 etc.; 99 % ont une valeur."

20. PRICE: sale price. Label: "Prix de Vente". Description: "Montant en dollars ; valeurs les plus fréquentes sont 1,0 et 10 ; un grand nombre d’enregistrements ont des prix nulls ou 'Other' (70 %)."

21. DEEDBOOK: book number of deed. Label: "Livre du Acte". Description: "Identifiant du livre où le titre de propriété est enregistré; 'TR18', '0', etc.; majorité d’entries uniques."

22. DEEDPAGE: page number within the deed book. Label: "Page du Livre". Description: "Numéro de page correspondant au registre des actes; la valeur 1 est la plus fréquente."

21 again? Actually 22.

23. SALECODE: code of sale type. Label: "Code de Vente". Description: "Code indiquant le type de transaction (3,0,H etc.). Le code '3' est le plus fréquent."

24. SALEDESC: description of sale. Label: "Description de la Vente". Description: "Texte détaillant le type d’opération; 'LOVE AND AFFECTION SALE', 'VALID SALE', etc.; les plus fréquents 97932 etc."

25. INSTRTYP: instrument type code? Label: "Type d'Instrument". Description: "Code du type d'acte (DE, SW...)."

26. INSTRTYPDESC: description of instrument type. Label: "Description du Type d'Instrument".

We need to incorporate details from stats: cardinality, uniqueness ratio, null counts etc.

Also frequency distribution gives top values and 'Other'.

Now produce JSON with each field key.

Let's craft labels in Franglais style: e.g., "Identifiant Unique", "ID Parcelle (PARID)", "Adresse Complète", etc. Provide description mixing French/English.

We must include mention of cardinality, uniqueness ratio maybe from summary stats? We have cardinality and uniqueness_ratio columns but not fully captured. In summary stats we see cardinality, uniqueness_ratio columns. For example _id: cardinality 479928, uniqueness_ratio 1. So label "Identifiant Unique" description includes that it's unique key.

We can incorporate typical values: e.g., for PARID top 5 values etc. Provide context: many unique but small subset repeated.

Let's craft each description accordingly.

Now produce JSON output. Ensure keys exactly field names as given.

Check all fields: _id, PARID, FULL_ADDRESS, PROPERTYHOUSENUM, PROPERTYFRACTION, PROPERTYADDRESSDIR, PROPERTYADDRESSSTREET, PROPERTYADDRESSSUF, PROPERTYADDRESSUNITDESC, PROPERTYUNITNO, PROPERTYCITY, PROPERTYSTATE, PROPERTYZIP, SCHOOLCODE, SCHOOLDESC, MUNICODE, MUNIDESC, RECORDDATE, SALEDATE, PRICE, DEEDBOOK, DEEDPAGE, SALECODE, SALEDESC, INSTRTYP, INSTRTYPDESC.

Let's write.
## TOKEN USAGE

TokenUsage { prompt: 7881, completion: 3641, total: 11522, elapsed: 52488 }
---
# Description
# Description

Ce dataset de 479 928 enregistrements capture l’historique des ventes immobilières dans l’État de Pennsylvanie.  
Il combine des identifiants uniques (`_id`), des codes de parcelle (`PARID`) et un grand nombre d’attributs d’adresse (numéro, rue, ville, code postal) ainsi que des métadonnées transactionnelles (date de vente, prix, type de titre).  
La majorité des champs sont fortement dispersés : 100 % des `_id` sont uniques, plus de 90 % des numéros de maison et des codes ZIP restent “Other” (c’est‑à‑dire non listés parmi les dix valeurs les plus fréquentes).  
Les variables catégorielles comme `SALECODE`, `SALEDESC`, `INSTRTYP` montrent une forte asymétrie, avec un ou deux types dominants représentant près de 40 % des transactions.  
Sur le plan numérique, la variable `PRICE` est très biaisée vers le bas : plus de 70 % des valeurs sont classées dans “Other”, et les rares prix élevés (jusqu’à 148 M$) créent un écart considérable par rapport à la moyenne d’environ 195 k$.  

## Notable Characteristics

- **Cardinalité élevée**: `_id` unique, `PARID` quasi‑unique, majorités “Other” dans les champs d’adresse → haute granularité.  
- **Valeurs manquantes**: ~0,26 % de `RECORDDATE`, ~0,26 % de `SALEDATE`; ~3 % de `PRICE` sont nulles.  
- **Skewness forte** dans les variables catégorielles (`SALECODE`, `SALEDESC`, `INSTRTYP`) – un ou deux codes couvrent 20–40 % des observations.  
- **Outliers**: prix extrêmes (jusqu’à 148 M$) et dates très éloignées de la médiane (années 2012‑2025).  
- **“Other” catégories**: la plupart des valeurs uniques sont regroupées sous “Other”, ce qui limite l’interprétabilité sans une exploration supplémentaire.  
- **Pas de doublons détectés** grâce à l’unicité du champ `_id`.  
- **PII/PHI**: adresses complètes et numéros de maison exposent des informations personnelles susceptibles d’être sensibles; attention lors du partage ou de la publication.  
- **Qualité des données**: présence de codes incohérents (ex : `PARID` avec lettres majuscules) et de valeurs “NULL” encodées comme chaînes vides dans plusieurs champs textuels.

> *Attribution : Generated by qsv v12.0.0 describegpt
Command line: qsv describegpt --all allegheny_county_property_sale_transactions.csv --language Franglais -o Franglais.md --fresh
Prompt file: Default v7.0.0
Model: openai/gpt-oss-20b
LLM API URL: http://localhost:1234/v1
Language: 
Timestamp: 2025-12-26T00:20:39.599945+00:00

WARNING: Description generated by an LLM and may contain inaccuracies. Verify before using!
*


## REASONING

We need to produce a description in French-English mix (Franglais). Use summary stats & freq distribution for overall dataset, not per field. Provide 1-8 sentences. Then Notable Characteristics bullet list. Avoid per-field details. Must mention central tendency, spread, anomalies, patterns, missing values, duplicates, PII/PHI/PCI etc. Then attribution placeholder.

Use markdown format.

Let's craft.

We have 479,928 records. Fields: many address fields, date, price, sale type, deed type etc.

Need to note distribution shape: many unique identifiers, high cardinality in _id and PARID; high missing rates in some fields like RECORDDATE (0.26% null), SALEDATE similar; PRICE has many zeros or nulls 3%, plus a large "Other" group (~71%) indicating most prices not captured. Many categorical fields with skewed distributions: e.g., sale code 3 dominates ~20%; property city Pittsburgh 54%. Outliers: price extreme high values like 148,752,900 but mean 195k; skewness etc. There are duplicates? _id unique, so no duplicates.

PII/PHI: addresses contain personal info (home addresses). So mention privacy concerns.

Let's craft description: maybe 4-5 sentences.

Then bullets: cardinality high for ID fields, missing values in dates and price, heavy skewness in categorical fields, presence of outliers in price, data quality issues like "Other" categories representing many unique values; potential duplication? no duplicates due to _id. Privacy concerns with addresses, etc.

Ok produce final output.
## TOKEN USAGE

TokenUsage { prompt: 9711, completion: 932, total: 10643, elapsed: 25002 }
---
# Tags
- property_sales  
- real_estate_transactions  
- pennsylvania_realty  
- sale_price  
- deed_type  
- school_districts  
- municipal_data  
- address_components  
- transaction_dates  
- property_address  

Generated by qsv v12.0.0 describegpt
Command line: qsv describegpt --all allegheny_county_property_sale_transactions.csv --language Franglais -o Franglais.md --fresh
Prompt file: Default v7.0.0
Model: openai/gpt-oss-20b
LLM API URL: http://localhost:1234/v1
Language: 
Timestamp: 2025-12-26T00:21:11.870927+00:00

WARNING: Tags generated by an LLM and may contain inaccuracies. Verify before using!



## REASONING

We need to produce tags in Markdown format, no field names. Up to 10 unique tags, descending importance based on dataset content from summary statistics and frequency distribution. Use franglais style: lowercase only and use underscore separators.

Dataset: real estate transactions data for Pennsylvania properties. Contains addresses, sale dates, prices, deed info, school district codes, municipal codes, etc.

Important topics: property sales, real estate, Pennsylvania, transaction records, property addresses, sale price, deed type, municipal data, school districts, address components, property numbers, sale description types, instrument type, etc.

We need to pick tags that help categorize dataset. Up to 10.

Possible tags: property_sales, real_estate_transactions, pennsylvania_realty, property_address, sale_price, deed_type, school_districts, municipal_data, transaction_dates, address_components.

In franglais style with underscores:

property_sales
real_estate_transactions
pennsylvania_realty
property_address
sale_price
deed_type
school_districts
municipal_data
transaction_dates
address_components

But we need descending importance. Which is most important? Probably property_sales or real_estate_transactions, but they are similar. Maybe "property_sales" first.

We can choose 10 tags: property_sales, real_estate_transactions, pennsylvania_realty, sale_price, deed_type, school_districts, municipal_data, address_components, transaction_dates, property_address.

But we have only up to 10; we can keep these.

Make sure no field names: avoid "PARID" etc. Good.

We need attribution placeholder {GENERATED_BY_SIGNATURE} after tags in Markdown format.

So final output:

```
- property_sales
- real_estate_transactions
- pennsylvania_realty
- sale_price
- deed_type
- school_districts
- municipal_data
- address_components
- transaction_dates
- property_address

{GENERATED_BY_SIGNATURE}
```

But maybe tags should be just list? Markdown format: could be bullet list or comma separated. Use bullet list.

Let's output accordingly.
## TOKEN USAGE

TokenUsage { prompt: 9680, completion: 488, total: 10168, elapsed: 17747 }
---
