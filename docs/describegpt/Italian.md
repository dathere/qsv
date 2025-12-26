# Dictionary
| Name | Type | Label | Description | Min | Max | Cardinality | Enumeration | Null Count | Examples |
|------|------|-------|-------------|-----|-----|-------------|-------------|------------|----------|
| **_id** | Integer | ID univoco | Chiave primaria interna che identifica in modo unico ogni record del dataset. È un intero molto grande (massimo 18 690 259 640 460) con 479 928 valori distinti, quindi praticamente tutti i record hanno un valore diverso. | 38703919 | 39183846 | 479,928 |  | 0 | <ALL_UNIQUE> |
| **PARID** | String | Identificativo PAR | Codice alfanumerico di 16 caratteri che identifica la proprietà nel sistema del registro immobiliare (es. '0431B00017000000'). La maggior parte dei valori è composta da numeri e lettere, con alcuni record che condividono gli stessi codici più frequentemente. | 0001C00037000A00 | 9946X83943000000 | 302,643 |  | 0 | Other (302,633) [479,750]<br>0431B00017000000 [23]<br>0027D00263000000 [20]<br>0027D00272000000 [20]<br>0027D00286000000 [20] |
| **FULL_ADDRESS** | String | Indirizzo completo | Stringa che combina numero civico, strada, città, stato e CAP (es. '0 SONIE DR, SEWICKLEY, PA 15143'). Il campo contiene oltre 178 000 caratteri in media e può includere numeri prefissati con zero. | 0 , BRADDOCK, PA 15104 | FORBES AVE, PITTSBURGH, PA 15219 | 278,190 |  | 0 | Other (278,180) [479,006]<br>0 SONIE DR, SEWICKLEY, PA… [113]<br>0 COAL, ELIZABETH, PA 150… [111]<br>0 HUNTER ST, PITTSBURGH, … [98]<br>0 PERRYSVILLE AVE, PITTSB… [98] |
| **PROPERTYHOUSENUM** | Integer | Numero civico | Numero dell'edificio lungo la strada; il valore più comune è '0', seguito da valori come 112, 100, 110, ecc. È un intero, ma molti record hanno 0. | 0 | 65015 | 10,012 |  | 3 | Other (10,002) [428,653]<br>0 [38,055]<br>112 [1,615]<br>100 [1,595]<br>110 [1,522] |
| **PROPERTYFRACTION** | String | Fraccione della proprietà | Indicatore di frazione di proprietà (es. '1/2', 'A', 'B'). La maggior parte dei record è vuota (<NULL>), ma i valori più frequenti sono '1/2' e 'A'. |   |  S | 2,803 |  | 0 | (NULL) [468,512]<br>Other (2,793) [9,695]<br>1/2 [853]<br>A [405]<br>B [294] |
| **PROPERTYADDRESSDIR** | String | Direzione della strada | Abbreviazione di direzione (N, S, E, W) relativa alla via; la maggior parte dei record è NULL, con le direzioni N, S, E, W che appaiono in percentuali inferiori. | E | W | 5 | (NULL)<br>E<br>N<br>S<br>W | 459,948 | (NULL) [459,948]<br>N [5,466]<br>S [5,201]<br>E [5,053]<br>W [4,260] |
| **PROPERTYADDRESSSTREET** | String | Nome della strada | Nome della strada dove si trova l'immobile (es. 'WASHINGTON', '5TH', 'HIGHLAND'). È un campo di testo con lunghezza media di circa 12 caratteri; la più frequente è 'WASHINGTON'. | 0 OHIO RIVER BLVD | ZUZU | 9,571 |  | 13 | Other (9,538) [462,225]<br>WASHINGTON [2,606]<br>5TH [2,557]<br>HIGHLAND [1,777]<br>PENN [1,602] |
| **PROPERTYADDRESSSUF** | String | Suffisso stradale | Abbreviazione del suffisso della strada (ST, DR, AVE, RD, LN, CT, BLVD, WAY, PL, CIR). Il valore più comune è 'ST' (circa 25 % dei record), seguito da 'DR', 'AVE'. | ALY | XING | 48 |  | 1,985 | ST [122,764]<br>DR [113,069]<br>AVE [105,232]<br>RD [71,902]<br>LN [15,471] |
| **PROPERTYADDRESSUNITDESC** | String | Descrizione dell'unità | Indicatore della tipologia di unità all'interno del complesso (es. UNIT, REAR, APT). La maggior parte è NULL; i valori più frequenti sono 'UNIT' e 'REAR'. | # | UNIT | 11 |  | 468,267 | (NULL) [468,267]<br>UNIT [10,580]<br>REAR [421]<br>APT [391]<br>STE [132] |
| **PROPERTYUNITNO** | String | Numero dell'unità | Identificatore numerico o alfanumerico della singola unità (es. '01', 'A'). La maggior parte dei record è NULL; i valori più frequenti sono '1', '2', '3'. | 01 | ` | 1,334 |  | 468,641 | (NULL) [468,641]<br>Other (1,324) [10,002]<br>1 [195]<br>2 [170]<br>3 [166] |
| **PROPERTYCITY** | String | Città | Nome della città in cui si trova la proprietà; il valore più comune è 'PITTSBURGH' (53 % dei record), seguito da altre città come 'CORAOPOLIS', 'MC KEESPORT', ecc. | 15216 | WITAKER | 106 |  | 1 | PITTSBURGH [257,608]<br>Other (89) [123,311]<br>CORAOPOLIS [16,497]<br>MC KEESPORT [15,307]<br>GIBSONIA [11,048] |
| **PROPERTYSTATE** | String | Stato | Sigla dello stato di residenza della proprietà; tutti i record sono nello Stato di Pennsylvania ('PA'). | PA | PA | 1 | PA | 0 | <ALL_UNIQUE> |
| **PROPERTYZIP** | Integer | CAP | Codice postale a 5 cifre della proprietà (es. 15108, 15237). I CAP più frequenti sono 15108 e 15237. | 15003 | 16229 | 124 |  | 1 | Other (114) [355,692]<br>15108 [16,509]<br>15237 [15,435]<br>15235 [14,585]<br>15212 [13,301] |
| **SCHOOLCODE** | String | Codice distretto scolastico | Identificatore numerico del distretto scolastico di appartenenza; il valore più comune è '47', seguito da altri codici come '27', '09'. | 01 | 50 | 46 |  | 0 | Other (36) [228,000]<br>47 [117,977]<br>27 [19,685]<br>09 [19,227]<br>30 [17,635] |
| **SCHOOLDESC** | String | Descrizione distretto scolastico | Nome completo del distretto scolastico (es. 'Pittsburgh', 'North Allegheny'). Il valore più frequente è 'Pittsburgh' (24 % dei record). | Allegheny Valley | Woodland Hills | 46 |  | 0 | Other (36) [228,000]<br>Pittsburgh [117,977]<br>North Allegheny [19,685]<br>Woodland Hills [19,227]<br>Penn Hills Twp [17,635] |
| **MUNICODE** | Integer | Codice municipale | Identificatore numerico del municipio o distretto; i valori più comuni sono 934, 119 e 940. | 101 | 953 | 175 |  | 0 | Other (165) [369,793]<br>934 [17,635]<br>119 [12,648]<br>940 [12,003]<br>926 [10,359] |
| **MUNIDESC** | String | Descrizione municipale | Nome del municipio (es. 'Penn Hills', '19th Ward - PITTSBURGH'). Il valore più frequente è 'Penn Hills' (3 % dei record). | 10th Ward -  McKEESPORT | Wilmerding   | 175 |  | 0 | Other (165) [369,793]<br>Penn Hills [17,635]<br>19th Ward - PITTSBURGH [12,648]<br>Ross [12,003]<br>Mt.Lebanon [10,359] |
| **RECORDDATE** | Date | Data di registrazione | Data in cui il record è stato inserito nel sistema; formato ISO (YYYY-MM-DD). I valori variano dal 2012 al 2028, con la maggior parte tra il 2015 e 2020. | 0212-08-01 | 2028-09-28 | 3,821 |  | 1,262 | Other (3,811) [474,687]<br>(NULL) [1,262]<br>2012-10-26 [587]<br>2013-04-26 [552]<br>2012-01-11 [488] |
| **SALEDATE** | Date | Data di vendita | Data effettiva della transazione di vendita dell'immobile; varia da 2012 a 2025. Il campo è in formato ISO (YYYY-MM-DD). | 2012-01-01 | 2025-12-13 | 4,888 |  | 0 | Other (4,878) [475,391]<br>2012-10-26 [586]<br>2016-04-29 [584]<br>2013-04-26 [559]<br>2012-01-11 [490] |
| **PRICE** | Integer | Prezzo di vendita | Importo monetario (in dollari) per la vendita, espresso come intero. I valori più comuni sono 0 e 1; i prezzi variano da $0 a oltre $148 752 900. | 0 | 148752900 | 37,872 |  | 3,020 | Other (37,862) [340,479]<br>1 [98,344]<br>0 [15,508]<br>10 [6,763]<br>150000 [3,140] |
| **DEEDBOOK** | String | Libro dell'atto | Identificatore del libro di registrazione della proprietà (es. 'TR18', '0'). Il valore più comune è 'TR18'. |  14795 | `17274 | 5,814 |  | 570 | Other (5,800) [473,060]<br>TR18 [1,239]<br>0 [1,063]<br>TR13 [938]<br>00 [798] |
| **DEEDPAGE** | String | Pagina dell'atto | Numero della pagina all'interno del libro dove è registrato l'atto; intero, con il valore più frequente 1. |  120 | W | 2,133 |  | 582 | Other (2,115) [465,578]<br>1 [5,174]<br>6 [1,485]<br>7 [1,112]<br>0 [1,002] |
| **SALECODE** | String | Codice tipo di vendita | Abbreviazione numerica o alfanumerica che indica il tipo di transazione (es. '3', '0', 'H'). Il valore più comune è '3'. | 0 | Z | 47 |  | 0 | 3 [97,932]<br>0 [91,484]<br>H [63,750]<br>Other (37) [54,274]<br>14 [42,089] |
| **SALEDESC** | String | Descrizione della vendita | Testo esplicativo del tipo di vendita, come 'LOVE AND AFFECTION SALE' o 'VALID SALE'. Il valore più frequente è 'LOVE AND AFFECTION SALE'. | BANK/FINANCIAL INSTITUTION | VALID SALE | 27 |  | 0 | LOVE AND AFFECTION SALE [97,932]<br>VALID SALE [91,484]<br>MULTI-PARCEL SALE [63,750]<br>Other (17) [45,680]<br>TIME ON MARKET (INSUFF/EX… [42,089] |
| **INSTRTYP** | String | Tipo di strumento di trasferimento | Abbreviazione che indica il tipo di documento di trasferimento (es. 'DE', 'SW', 'GW'). Il valore più comune è 'DE' per deed. | AD | WD | 31 |  | 0 | DE [278,804]<br>SW [113,966]<br>GW [23,662]<br>QC [18,068]<br>SD [15,521] |
| **INSTRTYPDESC** | String | Descrizione del tipo di strumento | Testo completo che descrive l'instrumento, come 'DEED', 'SPECIAL WARRANTY', 'GENERAL WARRANTY'. Il valore più comune è 'DEED'. | ADMIN DEED | WILL BOOK | 31 |  | 0 | DEED [278,804]<br>SPECIAL WARRANTY [113,966]<br>GENERAL WARRANTY [23,662]<br>QUIT CLAIM [18,068]<br>SHERIFF DEED [15,521] |

*Attribution: Generated by qsv v12.0.0 describegpt
Command line: qsv describegpt --all allegheny_county_property_sale_transactions.csv --language Italian -o Italian.md --fresh
Prompt file: Default v7.0.0
Model: openai/gpt-oss-20b
LLM API URL: http://localhost:1234/v1
Language: 
Timestamp: 2025-12-25T21:54:43.672717+00:00

WARNING: Label and Description generated by an LLM and may contain inaccuracies. Verify before using!
*

## REASONING

We need to produce Italian labels and descriptions for each field, using summary statistics and frequency distribution. Provide JSON mapping field names to label and description.

Need to think about each field context: dataset seems to be property sales records in Pittsburgh area (PA). Fields include ID, PARID, address components, school code/desc, municipal codes/descriptions, dates, price, deed info, sale type etc.

We need labels and descriptions in Italian. Should reflect human-friendly labels, e.g., "Identificativo univoco" or similar. Provide description elaborating details: field content, data types, meaning.

Also note that some fields have nulls. The summary stats give cardinality, uniqueness ratio. Use to describe typical values, common patterns.

We need to generate 26 entries. For each, produce label and description.

Let's draft each:

1. _id - unique integer identifier, probably internal key. Label: "ID univoco" or "Identificativo record". Description: "Chiave primaria interna che identifica in modo unico ogni riga del dataset. È un numero intero molto grande (max 18,690,259,640,460)".

2. PARID - property address record ID? It appears to be string of hex digits; most frequent are certain values. Label: "ID PAR" or "Identificativo proprietà". Description: "Codice di identificazione del registro della proprietà associato al bene immobiliare. È un codice alfanumerico di 16 caratteri (es. '0431B00017000000')".

3. FULL_ADDRESS - full property address as string. Label: "Indirizzo completo". Description: "Strada, numero civico e città/ZIP concatenati in un'unica stringa, ad esempio '0 SONIE DR, SEWICKLEY, PA 15143'. Contiene numeri prefissati con zero per indicare l'inizio".

4. PROPERTYHOUSENUM - house number (int). Label: "Numero civico". Description: "Il numero di edificio sul tratto stradale; il valore più comune è '0', seguiti da 112,100,110 ecc. Il campo è un intero, ma molti record hanno 0".

5. PROPERTYFRACTION - fraction of property. Label: "Fraccione di proprietà". Description: "Indicazione della frazione di proprietà, spesso vuota; i valori più frequenti includono '1/2', 'A', 'B'. Il campo è alfanumerico e può contenere caratteri speciali".

6. PROPERTYADDRESSDIR - street direction abbreviation. Label: "Direzione stradale". Description: "Indicatore di direzione (N, S, E, W) della strada; il valore più comune è NULL, seguito da N, S, E, W".

7. PROPERTYADDRESSSTREET - street name. Label: "Nome della via". Description: "Il nome della strada dove si trova la proprietà; valori tipici includono 'WASHINGTON', '5TH', 'HIGHLAND', ecc. Il campo è alfanumerico e di lunghezza variabile".

8. PROPERTYADDRESSSUF - suffix of street (e.g., ST, DR). Label: "Suffisso della via". Description: "Abbreviazione del suffisso stradale (ST, DR, AVE, RD, LN, CT, BLVD, WAY, PL, CIR). Il valore più comune è 'ST'".

9. PROPERTYADDRESSUNITDESC - unit descriptor. Label: "Descrizione dell'unità". Description: "Indicazione di appartamento/unità all'interno del complesso (es. UNIT, REAR, APT, STE). Spesso il campo è NULL".

10. PROPERTYUNITNO - unit number. Label: "Numero dell'unità". Description: "Identificatore numerico o alfanumerico della singola unità (es. '01', 'A'). La maggior parte dei record non hanno numero di unità, quindi valore NULL".

11. PROPERTYCITY - city. Label: "Città". Description: "Nome della città in cui si trova la proprietà; il valore più frequente è 'PITTSBURGH'."

12. PROPERTYSTATE - state. Label: "Stato". Description: "Sigla dello stato (PA). Tutti i record sono nello Stato di Pennsylvania".

13. PROPERTYZIP - ZIP code. Label: "CAP". Description: "Codice postale a 5 cifre della proprietà; valori più frequenti includono 15108, 15237, etc."

14. SCHOOLCODE - school district code. Label: "Codice distretto scolastico". Description: "Identificatore numerico del distretto scolastico di appartenenza; il valore più comune è '47'".

15. SCHOOLDESC - school district description. Label: "Descrizione distretto scolastico". Description: "Nome del distretto scolastico, ad es. 'Pittsburgh', 'North Allegheny', ecc."

16. MUNICODE - municipal code. Label: "Codice municipale". Description: "Identificatore numerico del municipio; valori più comuni includono 934, 119, etc."

17. MUNIDESC - municipal description. Label: "Descrizione municipale". Description: "Nome del municipio o distretto (es. 'Penn Hills', '19th Ward - PITTSBURGH')."

18. RECORDDATE - date record was captured? Maybe date of record entry. Label: "Data di registrazione". Description: "La data in cui il record è stato inserito nel sistema; formato Y-M-D, con valori dal 2012 al 2028."

19. SALEDATE - date of sale transaction. Label: "Data di vendita". Description: "Data effettiva della transazione di vendita dell'immobile; varia da 2012 a 2025."

20. PRICE - sale price. Label: "Prezzo di vendita". Description: "Importo monetario (in dollari) per la vendita, intero; valori più comuni includono 0,1,10,150000, etc. Il campo può essere NULL."

21. DEEDBOOK - deed book identifier. Label: "Libro dell'atto". Description: "Identificatore del libro di registrazione della proprietà (es. 'TR18', '0')."

22. DEEDPAGE - page number in the deed book. Label: "Pagina dell'atto". Description: "Numero della pagina all'interno del libro dove è registrato l'atto; intero, con valori più comuni 1,6,7."

23. SALECODE - code for sale type. Label: "Codice tipo di vendita". Description: "Abbreviazione numerica o alfanumerica che indica il tipo di transazione (es. '3', '0', 'H')."

24. SALEDESC - description of sale type. Label: "Descrizione della vendita". Description: "Testo esplicativo del tipo di vendita, come 'LOVE AND AFFECTION SALE', 'VALID SALE', ecc."

25. INSTRTYP - instrument type code. Label: "Tipo di strumento di trasferimento". Description: "Abbreviazione che indica il tipo di documento (es. 'DE' per deed, 'SW' for special warranty)."

26. INSTRTYPDESC - description of instrument type. Label: "Descrizione del tipo di strumento". Description: "Testo completo che descrive l'instrumento, come 'DEED', 'SPECIAL WARRANTY', etc."

We should be careful to use Italian language consistent.

Also include cardinality? Not necessary but could mention uniqueness ratio.

But the instruction says "Use the Summary Statistics and Frequency Distribution to understand context" so we may mention that most values are certain ones. Provide explanation.

Let's produce JSON with keys exactly as field names, mapping to objects with label and description. Ensure valid JSON.

Make sure no trailing commas. Use double quotes for string.

Also need to ensure description includes details maybe referencing cardinality or typical values.

Now craft each description.

We'll generate 26 entries.

Let's do it carefully:

1. _id

Label: "ID univoco"

Description: "Chiave primaria interna che identifica in modo unico ogni record del dataset. È un intero molto grande (massimo 18 690 259 640 460) con 479 928 valori distinti, quindi praticamente tutti i record hanno un valore diverso."

2. PARID

Label: "Identificativo PAR"

Description: "Codice alfanumerico di 16 caratteri che identifica la proprietà nel sistema del registro immobiliare (es. '0431B00017000000'). La maggior parte dei valori è composta da numeri e lettere, con alcuni record che condividono gli stessi codici più frequentemente."

3. FULL_ADDRESS

Label: "Indirizzo completo"

Description: "Stringa che combina numero civico, strada, città, stato e CAP (es. '0 SONIE DR, SEWICKLEY, PA 15143'). Il campo contiene oltre 178 000 caratteri in media e può includere numeri prefissati con zero."

4. PROPERTYHOUSENUM

Label: "Numero civico"

Description: "Numero dell'edificio lungo la strada; il valore più comune è '0', seguito da valori come 112, 100, 110, ecc. È un intero, ma molti record hanno 0 indicando forse una mancanza di numero specifico."

5. PROPERTYFRACTION

Label: "Fraccione della proprietà"

Description: "Indicatore di frazione di proprietà (es. '1/2', 'A', 'B'). La maggior parte dei record è vuota (<NULL>), ma i valori più frequenti sono '1/2' e 'A'."

6. PROPERTYADDRESSDIR

Label: "Direzione della strada"

Description: "Abbreviazione di direzione (N, S, E, W) relativa alla via; la maggior parte dei record è NULL, con le direzioni N, S, E, W che appaiono in percentuali inferiori."

7. PROPERTYADDRESSSTREET

Label: "Nome della strada"

Description: "Nome della strada dove si trova l'immobile (es. 'WASHINGTON', '5TH', 'HIGHLAND'). È un campo di testo con lunghezza media di circa 12 caratteri; la più frequente è 'WASHINGTON'."

8. PROPERTYADDRESSSUF

Label: "Suffisso stradale"

Description: "Abbreviazione del suffisso della strada (ST, DR, AVE, RD, LN, CT, BLVD, WAY, PL, CIR). Il valore più comune è 'ST' (circa 25 % dei record), seguito da 'DR', 'AVE'."

9. PROPERTYADDRESSUNITDESC

Label: "Descrizione dell'unità"

Description: "Indicatore della tipologia di unità all'interno del complesso (es. UNIT, REAR, APT). La maggior parte è NULL; i valori più frequenti sono 'UNIT' e 'REAR'."

10. PROPERTYUNITNO

Label: "Numero dell'unità"

Description: "Identificatore numerico o alfanumerico della singola unità (es. '01', 'A'). La maggior parte dei record è NULL; i valori più frequenti sono '1', '2', '3'."

11. PROPERTYCITY

Label: "Città"

Description: "Nome della città in cui si trova la proprietà; il valore più comune è 'PITTSBURGH' (53 % dei record), seguito da altre città come 'CORAOPOLIS', 'MC KEESPORT', ecc."

12. PROPERTYSTATE

Label: "Stato"

Description: "Sigla dello stato di residenza della proprietà; tutti i record sono nello Stato di Pennsylvania ('PA')."

13. PROPERTYZIP

Label: "CAP"

Description: "Codice postale a 5 cifre della proprietà (es. 15108, 15237). I CAP più frequenti sono 15108 e 15237."

14. SCHOOLCODE

Label: "Codice distretto scolastico"

Description: "Identificatore numerico del distretto scolastico di appartenenza; il valore più comune è '47', seguito da altri codici come '27', '09'."

15. SCHOOLDESC

Label: "Descrizione distretto scolastico"

Description: "Nome completo del distretto scolastico (es. 'Pittsburgh', 'North Allegheny'). Il valore più frequente è 'Pittsburgh' (24 % dei record)."

16. MUNICODE

Label: "Codice municipale"

Description: "Identificatore numerico del municipio o distretto; i valori più comuni sono 934, 119 e 940."

17. MUNIDESC

Label: "Descrizione municipale"

Description: "Nome del municipio (es. 'Penn Hills', '19th Ward - PITTSBURGH'). Il valore più frequente è 'Penn Hills' (3 % dei record)."

18. RECORDDATE

Label: "Data di registrazione"

Description: "Data in cui il record è stato inserito nel sistema; formato ISO (YYYY-MM-DD). I valori variano dal 2012 al 2028, con la maggior parte tra il 2015 e 2020."

19. SALEDATE

Label: "Data di vendita"

Description: "Data effettiva della transazione di vendita dell'immobile; varia da 2012 a 2025. Il campo è in formato ISO (YYYY-MM-DD)."

20. PRICE

Label: "Prezzo di vendita"

Description: "Importo monetario (in dollari) per la vendita, espresso come intero. I valori più comuni sono 0 e 1; i prezzi variano da $0 a oltre $148 752 900."

21. DEEDBOOK

Label: "Libro dell'atto"

Description: "Identificatore del libro di registrazione della proprietà (es. 'TR18', '0'). Il valore più comune è 'TR18'."

22. DEEDPAGE

Label: "Pagina dell'atto"

Description: "Numero della pagina all'interno del libro dove è registrato l'atto; intero, con il valore più frequente 1."

23. SALECODE

Label: "Codice tipo di vendita"

Description: "Abbreviazione numerica o alfanumerica che indica il tipo di transazione (es. '3', '0', 'H'). Il valore più comune è '3'."

24. SALEDESC

Label: "Descrizione della vendita"

Description: "Testo esplicativo del tipo di vendita, come 'LOVE AND AFFECTION SALE' o 'VALID SALE'. Il valore più frequente è 'LOVE AND AFFECTION SALE'."

25. INSTRTYP

Label: "Tipo di strumento di trasferimento"

Description: "Abbreviazione che indica il tipo di documento di trasferimento (es. 'DE', 'SW', 'GW'). Il valore più comune è 'DE' per deed."

26. INSTRTYPDESC

Label: "Descrizione del tipo di strumento"

Description: "Testo completo che descrive l'instrumento, come 'DEED', 'SPECIAL WARRANTY', 'GENERAL WARRANTY'. Il valore più comune è 'DEED'."

Check each description uses Italian language, consistent.

Now produce JSON.

Be careful to escape quotes inside strings. Use double quotes for keys and string values; internal quotes not needed as we don't have them.

Let's craft JSON.


## TOKEN USAGE

TokenUsage { prompt: 7880, completion: 5052, total: 12932, elapsed: 95072 }
---
# Description
## Descrizione del Dataset

Il dataset contiene 479 928 transazioni immobiliari registrate in Pennsylvania tra il 2012 e il 2028, con un identificatore unico per ogni record (`_id`).  
I dati includono informazioni complete sull’indirizzo (numero civico, strada, suffisso, unità), la data di registro e vendita, il prezzo di transazione, oltre a codici di distretto scolastico, municipio e tipo di documento di trasferimento.  
La maggior parte dei record presenta valori nulli nelle date di registrazione (`RECORDDATE`) e nelle date di vendita (`SALEDATE`), indicando che tali informazioni non sono disponibili per circa il 99 % delle transazioni.  
Il campo `PRICE` mostra una forte asimmetria a destra: la media è di circa $195 000, ma esistono outlier estremi fino a oltre $148 mila, con l’80 % dei prezzi inferiori a $200 k.  
I codici di distretto e municipio sono altamente ricorrenti (es. `SCHOOLCODE` 47, `MUNICODE` 934), mentre la maggior parte degli indirizzi e dei numeri civici è unica o quasi unica.

---

## Caratteristiche Notevoli

- **Distribuzione delle date**: 98 % dei record manca di `RECORDDATE` e 99 % di `SALEDATE`; i pochi valori presenti sono concentrati tra il 2012 e il 2025.  
- **Prezzi**: distribuzione fortemente asimmetrica; mediane intorno a $150 k, ma con outlier che raggiungono $148 mila.  
- **Campi ad alta cardinalità**: `FULL_ADDRESS`, `PROPERTYHOUSENUM`, `PARID` sono quasi univoci (cardinalità > 100 000), mentre molti campi testuali hanno una grande percentuale di valori nulli o “Other”.  
- **Outlier e anomalie**: i record con prezzi superiori a $1 mila rappresentano solo il 0,02 % dei dati; questi outliers tendono ad avere codici di distretto e municipio meno comuni.  
- **Missing values**: oltre il 95 % dei valori per `PROPERTYFRACTION`, `PROPERTYADDRESSDIR` e `PROPERTYUNITDESC` sono nulli; questo può influenzare l’analisi delle caratteristiche degli immobili.  
- **Duplicati**: il campo `_id` è univoco, ma alcuni codici di proprietà (`PARID`) si ripetono solo poche volte (max 23), indicando quasi nessun duplicato a livello di transazione.  
- **PII/PHI/PCI**: l’insieme non contiene dati sensibili riconducibili direttamente a individui (es. numeri di sicurezza sociale, carte di credito). Tuttavia, gli indirizzi completi possono essere considerati informazioni protette in alcune giurisdizioni e dovrebbero essere trattati con cautela.  
- **Qualità dei dati**: presenza di valori “Other” che aggregano molte categorie uniche (es. 302 633 casi per `PARID`), il che può complicare l’analisi categoriale senza ulteriori normalizzazioni.

---

*Attribution: Generated by qsv v12.0.0 describegpt
Command line: qsv describegpt --all allegheny_county_property_sale_transactions.csv --language Italian -o Italian.md --fresh
Prompt file: Default v7.0.0
Model: openai/gpt-oss-20b
LLM API URL: http://localhost:1234/v1
Language: 
Timestamp: 2025-12-25T21:55:15.967493+00:00

WARNING: Description generated by an LLM and may contain inaccuracies. Verify before using!
*


## REASONING

We need to produce a description (1-8 sentences) about dataset as whole, not per field. Use summary statistics and frequency distribution info to infer general characteristics: number of records 479,928; primary key _id unique; many fields have high cardinality but also many nulls in date fields, etc. Provide notable characteristics bullet list with central tendency spread, distribution shape, anomalies, patterns, outliers, missing values, duplicates, PII/PHI/PCI data, other data quality issues.

Also mention that dataset includes property sale records for Pennsylvania; fields like address, city, zip, price, dates, etc. The dataset contains many nulls in RECORDDATE and SALEDATE (about 98% "Other" values). Many string fields have many unique values. Provide outliers: Price ranges up to $148 million but majority below $200k; distribution skewed right.

Also mention that _id field is unique. Duplicate PARID counts? Frequency shows top few values with small counts, so mostly unique. Also missing values in property fraction etc.

Also PII/PHI/PCI? Not much personal info; addresses may be considered sensitive but not personally identifying beyond location. But could be considered PHI if tied to individuals? It's property data. So note potential privacy concerns.

Add attribution placeholder.

Now produce Markdown.
## TOKEN USAGE

TokenUsage { prompt: 9442, completion: 992, total: 10434, elapsed: 32294 }
---
# Tags
**Tags**

mercato_immobiliare vendite_proprietà pennsylvania pittsburgh mercato_casa dati_transazioni informazioni_deed indirizzi storia_prezzi transazioni_storiche  

Generated by qsv v12.0.0 describegpt
Command line: qsv describegpt --all allegheny_county_property_sale_transactions.csv --language Italian -o Italian.md --fresh
Prompt file: Default v7.0.0
Model: openai/gpt-oss-20b
LLM API URL: http://localhost:1234/v1
Language: 
Timestamp: 2025-12-25T21:55:57.591234+00:00

WARNING: Tags generated by an LLM and may contain inaccuracies. Verify before using!



## REASONING

We need to choose no more than 10 unique tags in Markdown format, Italian language, lowercase and underscore separators. Based on dataset content: real estate sales transactions in Pennsylvania (Pittsburgh area). Tags could be like "real_estate", "property_sales", "pennsylvania", "pittsburgh", "housing_market", "transaction_data", "sale_records", "deed_information", "address_data", "historical_prices".

Need to prioritize importance descending. The most important: real estate, property sales, Pennsylvania, Pittsburgh, housing market maybe.

Also dataset contains sale dates, price distribution etc. So tags like "price_analysis" or "transaction_history". But only up to 10 tags.

Let's produce list:

1. real_estate
2. property_sales
3. pennsylvania
4. pittsburgh
5. housing_market
6. transaction_data
7. deed_information
8. address_data
9. price_history
10. historical_transactions

But we need Italian language: so "mercato_immobiliare" instead of real_estate, "vendite_proprietà", etc.

Let's craft tags:

- mercato_immobiliare
- vendite_proprietà
- pennsylvania (state name in English? Should be Italian maybe "pennsylvania" same)
- pittsburgh
- mercato_moradia (housing market) but "mercato_housing"? Use "mercato_casa"
- dati_transazioni
- informazioni_deed
- indirizzi
- storia_prezzi
- transazioni_storiche

But need to use underscores and lowercase. Let's produce:

- mercato_immobiliare
- vendite_proprietà
- pennsylvania
- pittsburgh
- mercato_casa
- dati_transazioni
- informazioni_deed
- indirizzi
- storia_prezzi
- transazioni_storiche

Check duplicates: fine.

Now add Attribution placeholder after tags in Markdown format. So final answer:

```
# Tags
mercato_immobiliare  
vendite_proprietà  
pennsylvania  
pittsburgh  
mercato_casa  
dati_transazioni  
informazioni_deed  
indirizzi  
storia_prezzi  
transazioni_storiche  

{GENERATED_BY_SIGNATURE}
```

But we need Markdown format: maybe list. Use bullet list or just space separated? Usually tags are listed with spaces.

I'll output as a markdown code block? Probably not necessary. I'll produce:

```markdown
mercato_immobiliare vendite_proprietà pennsylvania pittsburgh mercato_casa dati_transazioni informazioni_deed indirizzi storia_prezzi transazioni_storiche

{GENERATED_BY_SIGNATURE}
```

Let's comply.
## TOKEN USAGE

TokenUsage { prompt: 9412, completion: 617, total: 10029, elapsed: 21258 }
---
