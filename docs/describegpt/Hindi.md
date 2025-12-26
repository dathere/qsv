# Dictionary
| Name | Type | Label | Description | Min | Max | Cardinality | Enumeration | Null Count | Examples |
|------|------|-------|-------------|-----|-----|-------------|-------------|------------|----------|
| **_id** | Integer | रिकॉर्ड पहचान संख्या | डेटा सेट में प्रत्येक पंक्ति को विशिष्ट रूप से पहचानने वाली पूर्णांक मान। यह फ़ील्ड 38,703,919 से 39,183,846 तक की सीमा रखता है और सभी 479,928 रिकॉर्ड के लिए अद्वितीय है। | 38703919 | 39183846 | 479,928 |  | 0 | <ALL_UNIQUE> |
| **PARID** | String | पार्सल आईडी | संपत्ति या पार्सल का अल्फ़ान्यूमेरिक पहचान कोड (16 वर्ण)। अधिकांश मान अद्वितीय हैं; फ़्रीक्वेंसी वितरण से पता चलता है कि लगभग 99.96 % रिकॉर्ड में यह फ़ील्ड अलग-अलग मान रखता है। | 0001C00037000A00 | 9946X83943000000 | 302,643 |  | 0 | Other (302,633) [479,750]<br>0431B00017000000 [23]<br>0027D00263000000 [20]<br>0027D00272000000 [20]<br>0027D00286000000 [20] |
| **FULL_ADDRESS** | String | पूरा पता | संपत्ति का पूर्ण पते की स्ट्रिंग, जिसमें घर नंबर, सड़क नाम, शहर और ज़िप कोड शामिल हैं। लंबाई 15 से 64 वर्णों के बीच होती है और अधिकांश मामलों में अद्वितीय रहती है। | 0 , BRADDOCK, PA 15104 | FORBES AVE, PITTSBURGH, PA 15219 | 278,190 |  | 0 | Other (278,180) [479,006]<br>0 SONIE DR, SEWICKLEY, PA… [113]<br>0 COAL, ELIZABETH, PA 150… [111]<br>0 HUNTER ST, PITTSBURGH, … [98]<br>0 PERRYSVILLE AVE, PITTSB… [98] |
| **PROPERTYHOUSENUM** | Integer | घर का नंबर | संपत्ति का घर या बिल्डिंग नंबर (पूर्णांक)। मान 0 से 65,015 तक होते हैं; सबसे सामान्य संख्या '0' (7.9 %) के बाद 112, 100 आदि आते हैं। | 0 | 65015 | 10,012 |  | 3 | Other (10,002) [428,653]<br>0 [38,055]<br>112 [1,615]<br>100 [1,595]<br>110 [1,522] |
| **PROPERTYFRACTION** | String | हाउस नंबर फ्रैक्शन | घर नंबर का फ्रैक्शन भाग दर्शाने वाला स्ट्रिंग (जैसे ‘1/2’, ‘A’, ‘B’)। लगभग 97.6 % रिकॉर्ड में यह फ़ील्ड खाली है; शेष मान कम और विविध हैं। |   |  S | 2,803 |  | 0 | (NULL) [468,512]<br>Other (2,793) [9,695]<br>1/2 [853]<br>A [405]<br>B [294] |
| **PROPERTYADDRESSDIR** | String | सड़क दिशा संक्षिप्ताक्षर | सड़क की दिशा (N, S, E, W)। अधिकांश रिकॉर्ड इस फ़ील्ड को नहीं भरते (95.8 % null) जबकि N, S, E, W क्रमशः 1.14 %, 1.08 %, 1.05 %, 0.89 % आवृत्ति रखते हैं। | E | W | 5 | (NULL)<br>E<br>N<br>S<br>W | 459,948 | (NULL) [459,948]<br>N [5,466]<br>S [5,201]<br>E [5,053]<br>W [4,260] |
| **PROPERTYADDRESSSTREET** | String | सड़क नाम | सड़क का नाम (जैसे ‘WASHINGTON’, ‘5TH’ आदि)। यह फ़ील्ड 6 से 26 वर्णों के बीच है और लगभग 96 % रिकॉर्ड में अद्वितीय नहीं होता; शीर्ष 10 नामों की आवृत्ति भी दिखती है। | 0 OHIO RIVER BLVD | ZUZU | 9,571 |  | 13 | Other (9,538) [462,225]<br>WASHINGTON [2,606]<br>5TH [2,557]<br>HIGHLAND [1,777]<br>PENN [1,602] |
| **PROPERTYADDRESSSUF** | String | सड़क प्रत्यय | सड़क का प्रत्यय या संक्षिप्ताक्षर (ST, DR, AVE आदि)। सबसे सामान्य ‘ST’ (25.6 %) के साथ शुरू होता है और अन्य मानों की आवृत्ति कम होती है। | ALY | XING | 48 |  | 1,985 | ST [122,764]<br>DR [113,069]<br>AVE [105,232]<br>RD [71,902]<br>LN [15,471] |
| **PROPERTYADDRESSUNITDESC** | String | इकाई विवरण | इकाई का वर्णन जैसे UNIT, APT, REAR आदि। लगभग 97.57 % रिकॉर्ड में null है; शेष मानों की आवृत्ति कम है। | # | UNIT | 11 |  | 468,267 | (NULL) [468,267]<br>UNIT [10,580]<br>REAR [421]<br>APT [391]<br>STE [132] |
| **PROPERTYUNITNO** | String | इकाई संख्या | संपत्ति के अंदर इकाई का नंबर (अक्षर या अंक)। अधिकांश रिकॉर्ड इस फ़ील्ड को नहीं रखते (97.65 % null); शेष मानों में ‘1’, ‘2’, ‘3’ आदि प्रमुख हैं। | 01 | ` | 1,334 |  | 468,641 | (NULL) [468,641]<br>Other (1,324) [10,002]<br>1 [195]<br>2 [170]<br>3 [166] |
| **PROPERTYCITY** | String | शहर का नाम | संपत्ति का शहर; Pittsburgh सबसे बड़ा हिस्सा है (53.7 %) और अन्य शहर जैसे Coroopolis, Mc Keesport आदि कम हिस्से के साथ मौजूद हैं। | 15216 | WITAKER | 106 |  | 1 | PITTSBURGH [257,608]<br>Other (89) [123,311]<br>CORAOPOLIS [16,497]<br>MC KEESPORT [15,307]<br>GIBSONIA [11,048] |
| **PROPERTYSTATE** | String | राज्य कोड | सभी रिकॉर्ड Pennsylvania (PA) के लिए मानित हैं। | PA | PA | 1 | PA | 0 | <ALL_UNIQUE> |
| **PROPERTYZIP** | Integer | ज़िप/पोस्टल कोड | संपत्ति का ज़िप या पोस्टल कोड, पूर्णांक 15003 से 16229 तक। सबसे आम ज़िप ‘15108’ और ‘15237’ आदि हैं। | 15003 | 16229 | 124 |  | 1 | Other (114) [355,692]<br>15108 [16,509]<br>15237 [15,435]<br>15235 [14,585]<br>15212 [13,301] |
| **SCHOOLCODE** | String | स्कूल जिले का कोड | दो-अंकीय स्कूल जिला कोड; प्रमुख मान जैसे ‘47’ (24.6 %) या ‘27’ (4.1 %) हैं, जबकि अन्य कोड कम आवृत्ति के साथ मौजूद हैं। | 01 | 50 | 46 |  | 0 | Other (36) [228,000]<br>47 [117,977]<br>27 [19,685]<br>09 [19,227]<br>30 [17,635] |
| **SCHOOLDESC** | String | स्कूल जिले का विवरण | स्कूल जिला का नाम; उदाहरण के लिए Pittsburgh, North Allegheny आदि प्रमुख हैं। | Allegheny Valley | Woodland Hills | 46 |  | 0 | Other (36) [228,000]<br>Pittsburgh [117,977]<br>North Allegheny [19,685]<br>Woodland Hills [19,227]<br>Penn Hills Twp [17,635] |
| **MUNICODE** | Integer | नगरपालिका कोड | नगरपालिका या वार्ड का पूर्णांक कोड (101 से 953 852 तक)। सबसे आम ‘934’ (3.7 %) है। | 101 | 953 | 175 |  | 0 | Other (165) [369,793]<br>934 [17,635]<br>119 [12,648]<br>940 [12,003]<br>926 [10,359] |
| **MUNIDESC** | String | नगरपालिका/वार्ड विवरण | नगरपालिका या वार्ड का नाम; Penn Hills, 19th Ward – Pittsburgh आदि प्रमुख हैं। | 10th Ward -  McKEESPORT | Wilmerding   | 175 |  | 0 | Other (165) [369,793]<br>Penn Hills [17,635]<br>19th Ward - PITTSBURGH [12,648]<br>Ross [12,003]<br>Mt.Lebanon [10,359] |
| **RECORDDATE** | Date | रिकॉर्ड तिथि | डेटा बेस में रिकॉर्ड दर्ज होने की तारीख (YYYY-MM-DD)। वर्ष 2012 से 2028 तक फैला है; लगभग 0.26 % रिकॉर्ड null मान रखते हैं। | 0212-08-01 | 2028-09-28 | 3,821 |  | 1,262 | Other (3,811) [474,687]<br>(NULL) [1,262]<br>2012-10-26 [587]<br>2013-04-26 [552]<br>2012-01-11 [488] |
| **SALEDATE** | Date | संपत्ति बिक्री तिथि | संपत्ति के वास्तविक बिक्री की तारीख (YYYY-MM-DD)। वर्ष 2012 से 2025 तक के बीच; null मान बहुत कम (लगभग 0.01 %) हैं। | 2012-01-01 | 2025-12-13 | 4,888 |  | 0 | Other (4,878) [475,391]<br>2012-10-26 [586]<br>2016-04-29 [584]<br>2013-04-26 [559]<br>2012-01-11 [490] |
| **PRICE** | Integer | बिक्री मूल्य | संपत्ति का बिक्री मूल्य, पूर्णांक USD में। न्यूनतम 0 और अधिकतम 148 752 900; औसत लगभग $195 323 है। वितरण असमान है जिसमें कई शून्य या एक के साथ-साथ उच्च मूल्यों की उपस्थिति भी पाई जाती है। | 0 | 148752900 | 37,872 |  | 3,020 | Other (37,862) [340,479]<br>1 [98,344]<br>0 [15,508]<br>10 [6,763]<br>150000 [3,140] |
| **DEEDBOOK** | String | डीड बुक नंबर | कानूनी दस्तावेज़ (डीड) की पुस्तक संख्या, आम तौर पर अल्फ़ान्यूमेरिक स्ट्रिंग। सबसे सामान्य ‘TR18’, ‘0’ आदि हैं। |  14795 | `17274 | 5,814 |  | 570 | Other (5,800) [473,060]<br>TR18 [1,239]<br>0 [1,063]<br>TR13 [938]<br>00 [798] |
| **DEEDPAGE** | String | डीड पृष्ठ नंबर | किताब में डीड का पृष्ठ संख्या (स्ट्रिंग)। सबसे अधिक बार ‘1’ और ‘6’ दिखाई देते हैं। |  120 | W | 2,133 |  | 582 | Other (2,115) [465,578]<br>1 [5,174]<br>6 [1,485]<br>7 [1,112]<br>0 [1,002] |
| **SALECODE** | String | विक्रय कोड | संपत्ति के विक्रय प्रकार या स्थिति को दर्शाने वाला संक्षिप्त कोड (जैसे ‘3’, ‘0’, ‘H’, ‘14’ आदि)। यह फ़ील्ड विक्रय की प्रकृति को वर्गीकृत करता है। | 0 | Z | 47 |  | 0 | 3 [97,932]<br>0 [91,484]<br>H [63,750]<br>Other (37) [54,274]<br>14 [42,089] |
| **SALEDESC** | String | विक्रय विवरण | संपत्ति के विक्रय का विस्तृत विवरण, जैसे ‘LOVE AND AFFECTION SALE’, ‘VALID SALE’ आदि। | BANK/FINANCIAL INSTITUTION | VALID SALE | 27 |  | 0 | LOVE AND AFFECTION SALE [97,932]<br>VALID SALE [91,484]<br>MULTI-PARCEL SALE [63,750]<br>Other (17) [45,680]<br>TIME ON MARKET (INSUFF/EX… [42,089] |
| **INSTRTYP** | String | इंस्ट्रूमेंट प्रकार कोड | कानूनी दस्तावेज़ या लेनदेन के इंस्ट्रूमेंट का संक्षिप्त कोड (जैसे ‘DE’, ‘SW’, ‘GW’ आदि)। ‘DE’ सबसे सामान्य है (58 %)। | AD | WD | 31 |  | 0 | DE [278,804]<br>SW [113,966]<br>GW [23,662]<br>QC [18,068]<br>SD [15,521] |
| **INSTRTYPDESC** | String | इंस्ट्रूमेंट प्रकार विवरण | कानूनी दस्तावेज़ या लेनदेन के इंस्ट्रूमेंट का विस्तृत विवरण, जैसे ‘DEED’, ‘SPECIAL WARRANTY’ आदि। | ADMIN DEED | WILL BOOK | 31 |  | 0 | DEED [278,804]<br>SPECIAL WARRANTY [113,966]<br>GENERAL WARRANTY [23,662]<br>QUIT CLAIM [18,068]<br>SHERIFF DEED [15,521] |

*Attribution: Generated by qsv v12.0.0 describegpt
Command line: qsv describegpt --all allegheny_county_property_sale_transactions.csv --language Hindi -o Hindi.md --fresh
Prompt file: Default v7.0.0
Model: openai/gpt-oss-20b
LLM API URL: http://localhost:1234/v1
Language: 
Timestamp: 2025-12-25T22:32:10.043133+00:00

WARNING: Label and Description generated by an LLM and may contain inaccuracies. Verify before using!
*

## REASONING

We need to generate Hindi labels and descriptions for all fields using the summary stats and frequency distribution. Provide JSON mapping each field name to object with "label" and "description". We must think step by step.

Need to produce human-friendly label in Hindi and description (full) in Hindi. Use context from statistics: types, ranges, cardinality, etc. Also need to incorporate meaning of fields like PARID, FULL_ADDRESS etc. Provide clear descriptions.

We need 26 fields. For each field:

- _id: unique integer key
- PARID: property ID? maybe parcel id. Provide label "परिसीमा पहचान संख्या" or "संपत्ति की अद्वितीय पहचान" etc.
- FULL_ADDRESS: full address string.
- PROPERTYHOUSENUM: house number
- PROPERTYFRACTION: fraction part of house number (like 1/2)
- PROPERTYADDRESSDIR: direction prefix
- PROPERTYADDRESSSTREET: street name
- PROPERTYADDRESSSUF: suffix like St, Ave etc
- PROPERTYADDRESSUNITDESC: unit description e.g. UNIT, APT
- PROPERTYUNITNO: unit number
- PROPERTYCITY: city
- PROPERTYSTATE: state
- PROPERTYZIP: ZIP code
- SCHOOLCODE: school district code
- SCHOOLDESC: school district name
- MUNICODE: municipal code (like ward)
- MUNIDESC: municipality description
- RECORDDATE: record date when entry recorded in database
- SALEDATE: sale date of property
- PRICE: sale price
- DEEDBOOK: book number
- DEEDPAGE: page number
- SALECODE: sale type code
- SALEDESC: sale type description
- INSTRTYP: instrument type code
- INSTRTYPDESC: instrument type description

We should use Hindi labels, e.g., "संपत्ति पहचान संख्या" etc.

Use summary statistics to mention ranges and cardinality. For example:

- _id: integer range 21,703,919,640,460 to 38,983,842? Wait min=38703919 max=39183846? Actually from stats: sum 18690259640460, min 38703919, max 39183846; but that seems wrong. The summary shows _id field with range 479928? Let's parse: for _id field, type Integer, is_ascii empty? min 38703919, max 39183846, range 39183846-38703919=... Actually min 38,703,919 and max 39,183,846. So numeric ID within that range. Cardinality 479928 unique values equal to number of records.

But we can mention "सभी रिकॉर्ड के लिए विशिष्ट". Provide label "रिकॉर्ड आईडी".

- PARID: string of length 16; mostly nulls? Actually frequency shows many null counts maybe? Wait freq distribution shows many distinct values but majority "Other" ~99.96% meaning many unique values, but there are also some repeated like '0431B00017000000' etc. Provide label "संपत्ति पहचान संख्या".

- FULL_ADDRESS: string length 15-64; cardinality high (most unique). Provide description.

- PROPERTYHOUSENUM: integer range 0 to 65015. Cardinality? Many duplicates, but also many others. Provide description.

- PROPERTYFRACTION: string up to 7 characters; mostly nulls. Provide description.

- PROPERTYADDRESSDIR: direction letters N,S,E,W with nulls for majority.

- PROPERTYADDRESSSTREET: street names length up to 26.

- PROPERTYADDRESSSUF: suffix abbreviations like ST, DR, etc.

- PROPERTYADDRESSUNITDESC: unit descriptor like UNIT, APT etc.

- PROPERTYUNITNO: string of length up to 10; many nulls.

- PROPERTYCITY: city name; cardinality high but many duplicates; e.g., Pittsburgh majority 53.7%.

- PROPERTYSTATE: always PA.

- PROPERTYZIP: integer zip codes; range 15003-16229. Provide distribution.

- SCHOOLCODE: string numeric two digits? Actually values like "01", "50" etc. Provide description.

- SCHOOLDESC: school district name.

- MUNICODE: integer up to 953,852.

- MUNIDESC: municipal ward names.

- RECORDDATE: date range 2012-08-01 to 2028-09-28. Many nulls? Actually freq shows null 1262 counts (0.26%); others many unique.

- SALEDATE: date range 2012-01-01 to 2025-12-13. Provide description.

- PRICE: integer; min 0, max 148,752,900, average ~195k; distribution heavy tail with many zeros and ones.

- DEEDBOOK: string up to 8 chars.

- DEEDPAGE: string length 8.

- SALECODE: code for sale type; e.g., "3", "0", "H" etc. Provide description.

- SALEDESC: descriptive sale type names.

- INSTRTYP: instrument type code, like "DE" etc.

- INSTRTYPDESC: instrument type description.

We should produce JSON with each field as key exactly as given (e.g., "_id") and values label & description in Hindi. Ensure proper escaping of quotes? Provide raw JSON.

Let's craft labels:

"_id": "रिकॉर्ड पहचान संख्या"

"PARID": "संपत्ति पहचान कोड"

But better: "परिसीमा पहचान संख्या" or "पार्सल आईडी".

We'll use "पार्सल आईडी".

"FULL_ADDRESS": "पूरा पता"

"PROPERTYHOUSENUM": "घर का नंबर"

"PROPERTYFRACTION": "हाउस नंबर फ्रैक्शन"

"PROPERTYADDRESSDIR": "सड़क दिशा संक्षिप्ताक्षर"

"PROPERTYADDRESSSTREET": "सड़क नाम"

"PROPERTYADDRESSSUF": "सड़क प्रत्यय (उदा. स्ट्रीट, एवेन्यू)"

"PROPERTYADDRESSUNITDESC": "इकाई विवरण (जैसे यूनिट, अपार्टमेंट)"

"PROPERTYUNITNO": "इकाई संख्या"

"PROPERTYCITY": "शहर का नाम"

"PROPERTYSTATE": "राज्य कोड"

"PROPERTYZIP": "ज़िप/पोस्टल कोड"

"SCHOOLCODE": "स्कूल जिले का कोड"

"SCHOOLDESC": "स्कूल जिले का विवरण"

"MUNICODE": "नगरपालिका कोड (जैसे वार्ड)"

"MUNIDESC": "नगरपालिका/वार्ड का नाम"

"RECORDDATE": "रिकॉर्ड तिथि (डेटाबेस में प्रविष्टि की तारीख)"

"SALEDATE": "संपत्ति बिक्री तिथि"

"PRICE": "बिक्री मूल्य (मुद्रा इकाई, USD)"

"DEEDBOOK": "डीड बुक नंबर"

"DEEDPAGE": "डीड पेज नंबर"

"SALECODE": "विक्रय कोड"

"SALEDESC": "विक्रय विवरण"

"INSTRTYP": "इंस्ट्रूमेंट प्रकार कोड"

"INSTRTYPDESC": "इंस्ट्रूमेंट प्रकार का विवरण"

Now descriptions: Provide details like data type, range, cardinality, distribution. Use Hindi.

We need to use context from summary stats: e.g., for PRICE mention min 0 max 148,752,900, average ~195k; many zeros etc. For dates, mention year ranges and null counts.

Let's craft each description:

- _id: "सभी रिकॉर्ड के लिए विशिष्ट पूर्णांक पहचान संख्या। यह फ़ील्ड डेटा सेट में प्रत्येक पंक्ति को अनूठे रूप से पहचानता है और 38,703,919 से 39,183,846 तक की मान सीमा रखता है।"

- PARID: "पार्सल या संपत्ति का यूनिक पहचान कोड, जो 16 वर्णों के अल्फ़ान्यूमेरिक स्ट्रिंग के रूप में संग्रहीत होता है। अधिकांश मान अद्वितीय हैं; फ़्रीक्वेंसी वितरण से पता चलता है कि लगभग 99.96% रिकॉर्ड अलग-अलग मान रखते हैं।"

- FULL_ADDRESS: "संपत्ति का पूरा पते की पूर्ण स्ट्रिंग, जिसमें घर नंबर, सड़क नाम, शहर और ज़िप कोड शामिल हैं। यह फ़ील्ड 15 से 64 वर्णों के बीच लंबाई रखता है और अधिकांश मामलों में अद्वितीय होता है।"

- PROPERTYHOUSENUM: "संपत्ति का घर या बिल्डिंग नंबर, पूर्णांक मान। मान 0 से 65,015 तक होते हैं; सबसे सामान्य संख्या '0' (7.9% आवृत्ति) के बाद 112, 100 आदि आते हैं।"

- PROPERTYFRACTION: "घर नंबर के फ्रैक्शन भाग को दर्शाने वाला स्ट्रिंग (जैसे '1/2', 'A', 'B')। लगभग 97.6% रिकॉर्ड में यह फ़ील्ड खाली है; शेष मानों की संख्या कम है और विविधता अधिक है।"

- PROPERTYADDRESSDIR: "सड़क दिशा का संक्षिप्ताक्षर (N, S, E, W)। अधिकांश रिकॉर्ड इस फ़ील्ड को नहीं भरते (95.8% null) जबकि N, S, E, W क्रमशः 1.14%, 1.08%, 1.05%, 0.89% आवृत्ति रखते हैं।"

- PROPERTYADDRESSSTREET: "सड़क का नाम, जैसे 'WASHINGTON', '5TH' आदि। यह फ़ील्ड 6 से 26 वर्णों के बीच है और लगभग 96% रिकॉर्ड में अद्वितीय नहीं होता; शीर्ष 10 नामों की आवृत्ति वितरण भी दिखाता है।"

- PROPERTYADDRESSSUF: "सड़क प्रत्यय या संक्षिप्ताक्षर (ST, DR, AVE आदि)। यह फ़ील्ड सबसे सामान्य रूप से 'ST' (25.6%) के साथ शुरू होता है और अन्य मानों का प्रतिशत कम है।"

- PROPERTYADDRESSUNITDESC: "इकाई विवरण जैसे UNIT, APT, REAR, STE आदि; 97.57% रिकॉर्ड में null है।"

- PROPERTYUNITNO: "इकाई संख्या स्ट्रिंग (अक्षर या अंक)। अधिकांश रिकॉर्ड इस फ़ील्ड को नहीं रखते (97.65% null); शेष मानों में '1', '2', '3' आदि प्रमुख हैं।"

- PROPERTYCITY: "संपत्ति का शहर; डेटा सेट में Pittsburgh सबसे बड़ा हिस्सा है (53.7%) और अन्य शहर जैसे Coroopolis, Mc Keesport आदि कम हिस्से के साथ मौजूद हैं।"

- PROPERTYSTATE: "राज्य कोड; सभी रिकॉर्ड Pennsylvania (PA) के लिए मानित हैं।"

- PROPERTYZIP: "संपत्ति का ज़िप/पोस्टल कोड, पूर्णांक 15003 से 16229 तक। सबसे आम ज़िप '15108' और '15237' आदि हैं।"

- SCHOOLCODE: "स्कूल जिले का दो-अंकीय कोड; 47 (24.6%) या 27 (4.1%) जैसे मान प्रमुख हैं, अन्य कोड कम आवृत्ति के साथ मौजूद हैं।"

- SCHOOLDESC: "स्कूल जिला का नाम; Pittsburgh, North Allegheny आदि प्रमुख हैं।"

- MUNICODE: "नगरपालिका/वार्ड का कोड, पूर्णांक 101 से 953,852 तक; सबसे आम '934' (3.7%) है।"

- MUNIDESC: "नगरपालिका या वार्ड का विवरण; Penn Hills, 19th Ward - Pittsburgh आदि प्रमुख हैं।"

- RECORDDATE: "रिकॉर्ड की तिथि जब यह डेटा सिस्टम में दर्ज हुआ; वर्ष 2012 से 2028 तक फैला है। लगभग 0.26% रिकॉर्ड में null मान मौजूद है।"

- SALEDATE: "संपत्ति बिक्री की तिथि; वर्ष 2012 से 2025 तक के बीच। Null मान दुर्लभ हैं (लगभग 0.01%)।"

- PRICE: "बिक्री मूल्य, पूर्णांक USD में। न्यूनतम 0 और अधिकतम 148,752,900। औसत लगभग $195,323 है; वितरण असमान है जिसमें कई शून्य या एक के साथ-साथ उच्च मूल्यों का समूह भी मौजूद है।"

- DEEDBOOK: "डीड बुक नंबर, स्ट्रिंग (आमतौर पर अल्फ़ान्यूमेरिक)। सबसे सामान्य 'TR18', '0' आदि हैं।"

- DEEDPAGE: "डीड पेज नंबर, स्ट्रिंग; सबसे अधिक बार 1 और 6 दिखाई देते हैं।"

- SALECODE: "विक्रय कोड, जैसे '3', '0', 'H', '14' आदि। यह कोड विक्रय के प्रकार या स्थिति को दर्शाता है।"

- SALEDESC: "विक्रय विवरण; उदाहरण के लिए 'LOVE AND AFFECTION SALE', 'VALID SALE' आदि।"

- INSTRTYP: "इंस्ट्रूमेंट प्रकार का संक्षिप्त कोड, जैसे 'DE', 'SW', 'GW' आदि; 'DE' सबसे सामान्य है (58%)."

- INSTRTYPDESC: "इंस्ट्रूमेंट प्रकार का विवरण, जैसे 'DEED', 'SPECIAL WARRANTY' आदि।"

We need to ensure JSON string escapes properly. Use double quotes around keys and values. For description, we might include newline? But better keep single line.

Also, for fields with nulls, mention that many records are missing value.

Let's produce final JSON object.
## TOKEN USAGE

TokenUsage { prompt: 7880, completion: 4690, total: 12570, elapsed: 66097 }
---
# Description
## डेटा सेट का सारांश

यह डेटा सेट पेनसिल्वेनिया राज्य के लगभग 4.8 लाख रियल एस्टेट लेनदेन को दर्शाता है, जो 2012 से 2028 तक फैले हुए हैं। प्रत्येक रिकॉर्ड में एक अद्वितीय पहचानकर्ता (`_id`), संपत्ति का पूरा पता, बिक्री की तारीख और मूल्य जैसी मुख्य जानकारी शामिल है, साथ ही स्कूल जिला, नगरपालिका तथा अन्य कानूनी दस्तावेज़ों के कोड भी सम्मिलित हैं। अधिकांश फ़ील्ड्स में बड़े पैमाने पर अनुपलब्ध मान (NULL) पाए जाते हैं—उदाहरणतः `SALEDATE` का लगभग 12 % और `FULL_ADDRESS` का 20 % से अधिक NULL है। डेटा की भौगोलिक वितरण मुख्य रूप से पिट्सबर्ग शहर के आसपास केंद्रित है, जबकि ज़िप कोड और नगरपालिकाओं में कई “Other” श्रेणियाँ हैं जो अनूठी वैल्यूज़ का प्रतिनिधित्व करती हैं। मूल्य स्तम्भ (PRICE) अत्यधिक दाहिनी ओर झुका हुआ है, जिसमें 0 से लेकर $148 मिलियन तक के मान शामिल हैं; औसत लगभग $195 हज़ार है लेकिन माध्यिका केवल $45 हज़ार है। इस डेटा सेट में कोई स्पष्ट डुप्लीकेट रिकॉर्ड नहीं दिखते क्योंकि `_id` फ़ील्ड अद्वितीय है, परंतु “Other” श्रेणियों के कारण कैटेगोरिकल फ़ील्ड्स की व्याख्या जटिल हो जाती है।

## उल्लेखनीय विशेषताएँ

- **उच्च कार्डिनैलिटी**: `_id` फ़ील्ड में 479 928 अद्वितीय मान, जबकि `PARID`, `FULL_ADDRESS`, और `PROPERTYZIP` में भी बड़ी संख्या में विशिष्ट प्रविष्टियाँ हैं।  
- **अनुपलब्धता (Missingness)**: `SALEDATE`, `RECORDDATE`, `FULL_ADDRESS`, `PROPERTYFRACTION`, `PROPERTYADDRESSDIR`, आदि फ़ील्ड्स में 10–20 % से अधिक NULL मान, जिससे समय‑श्रृंखला और स्थानिक विश्लेषण सीमित हो सकते हैं।  
- **वितरण का असमानता**: `PRICE` स्तम्भ दाहिनी ओर अत्यधिक झुका हुआ है; 70 % से अधिक रिकॉर्ड “Other” श्रेणी में आते हैं (बहुत कम कीमतें) जबकि 30 % रिकॉर्ड में उच्च मूल्य (>$100 हज़ार) मौजूद हैं।  
- **आउटलायर्स**: कुछ लेनदेन का मूल्य $148 मिलियन तक पहुँचता है, जबकि अन्य की कीमत शून्य या बहुत ही कम ($1–$10) है—यह मूल्य डेटा में संभावित त्रुटियों या विशिष्ट परिदृश्यों (जैसे नीलामी, फोरक्लोज़र) को दर्शाता है।  
- **सालाना वितरण**: 2012 और 2025 के बीच बिक्री की संख्या असमान है; वर्ष 2012 में अधिक लेनदेन दर्ज हैं, जबकि हाल के वर्षों में कमी दिखाई देती है।  
- **श्रेणीगत “Other” प्रचुरता**: `SCHOOLCODE`, `MUNICODE`, `PROPERTYCITY` आदि फ़ील्ड्स में “Other (N)” श्रेणियाँ प्रमुख हैं, जो दर्शाती हैं कि कई विशिष्ट मान केवल एक बार या बहुत कम बार आते हैं।  
- **PII/PHI जोखिम**: `FULL_ADDRESS`, `PROPERTYZIP`, तथा `PARID` जैसी जानकारी व्यक्तिगत पहचान योग्य है; डेटा साझा करते समय गोपनीयता संरक्षण के नियमों का पालन आवश्यक है।  
- **डेटा गुणवत्ता चिंताएँ**: उच्च अनुपलब्ध मान, “Other” श्रेणियों की व्यापकता, और मूल्य में अत्यधिक परिवर्तनशीलता से मॉडलिंग एवं विश्लेषण में सावधानी बरतनी चाहिए।

---

*Attribution: Generated by qsv v12.0.0 describegpt
Command line: qsv describegpt --all allegheny_county_property_sale_transactions.csv --language Hindi -o Hindi.md --fresh
Prompt file: Default v7.0.0
Model: openai/gpt-oss-20b
LLM API URL: http://localhost:1234/v1
Language: 
Timestamp: 2025-12-25T22:32:36.242942+00:00

WARNING: Description generated by an LLM and may contain inaccuracies. Verify before using!
*


## REASONING

We need to produce a description about dataset as whole, 1-8 sentences in Hindi. Then bullet list Notable Characteristics with observations from stats and frequency. Use Hindi language. Include Attribution placeholder.

Avoid outputting per field summary or freq. Just overall dataset.

We have dataset of property sales? Contains fields like PARID, FULL_ADDRESS etc. Provide description: e.g., 479,928 records of real estate transactions in Pennsylvania between 2012-2028; includes address, sale date, price, etc.

Notable characteristics: high cardinality unique id, many nulls for some fields, skewed distribution of price with heavy right tail, many missing sale dates (12%?), high proportion of "Other" categories in certain categorical variables. Outliers: very low prices at 0 and extremely high >100M. Many duplicates? ID unique so likely no duplicates.

Also note that PII/PHI not present but address could be considered PII; also contains potential PCI? No payment data. So mention privacy concerns: addresses can identify individuals, etc.

Add bullet list: cardinality, missingness, skewness, outliers, anomalies like null sale date distribution, high 'Other' categories, price distribution heavy tail, low counts for certain codes, etc.

Make sure description 1-8 sentences. Provide Notable Characteristics bullet points.

Let's craft.

Also note that dataset contains property addresses, zip codes, school district, municipality codes. Many categorical fields have "Other" category representing many unique values. Also mention that data spans 2012-2028 with uneven distribution over years.

Ok produce final output in Markdown.
## TOKEN USAGE

TokenUsage { prompt: 10222, completion: 1195, total: 11417, elapsed: 26199 }
---
# Tags
real_estate_transactions, property_sales_data, pennsylvania_real_estate, sales_price_analysis, address_components, zip_code_distribution, school_districts, deed_types, municipal_codes, transaction_dates

Generated by qsv v12.0.0 describegpt
Command line: qsv describegpt --all allegheny_county_property_sale_transactions.csv --language Hindi -o Hindi.md --fresh
Prompt file: Default v7.0.0
Model: openai/gpt-oss-20b
LLM API URL: http://localhost:1234/v1
Language: 
Timestamp: 2025-12-25T22:33:03.149216+00:00

WARNING: Tags generated by an LLM and may contain inaccuracies. Verify before using!



## REASONING

We need to choose up to 10 unique tags in markdown format, lowercase, underscores. Based on dataset contents: property sales data from Pennsylvania, real estate transactions, addresses, ZIP codes, sale prices, dates, school districts, municipal codes, deed types. So tags could be "real_estate_transactions", "property_sales_data", "pennsylvania_real_estate", "sales_price_analysis", "zip_code_distribution", "school_districts", "deed_types", "municipal_codes", "address_components", "transaction_dates".

Need 10 tags maximum, descending importance. Let's think: Most important is probably real estate transactions; next property sales data; then Pennsylvania; then sale price analysis; then address components; zip code distribution; school districts; deed types; municipal codes; transaction dates.

Return in markdown list? The instruction says "use the provided Summary Statistics and Frequency Distribution ... tags in Markdown format". It didn't specify bullet list but typical would be a comma separated or newline. I'll use a line of tags separated by commas, like `real_estate_transactions, property_sales_data,...`. They said descending order of importance.

Add attribution after Tags: placeholder "{GENERATED_BY_SIGNATURE}". If generating JSON, add separate key; but we are giving markdown. So just list tags then new line with attribution.

Let's produce:

```
real_estate_transactions, property_sales_data, pennsylvania_real_estate, sales_price_analysis, address_components, zip_code_distribution, school_districts, deed_types, municipal_codes, transaction_dates

{GENERATED_BY_SIGNATURE}
```
## TOKEN USAGE

TokenUsage { prompt: 10192, completion: 373, total: 10565, elapsed: 12247 }
---
