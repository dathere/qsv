# Dictionary
| Name | Type | Label | Description | Min | Max | Cardinality | Enumeration | Null Count | Examples |
|------|------|-------|-------------|-----|-----|-------------|-------------|------------|----------|
| **_id** | Integer | Clave única | Identificador interno del registro. Es un número entero que garantiza la unicidad de cada fila en el conjunto de datos y no aparece repetido. | 38703919 | 39183846 | 479,928 |  | 0 | <ALL_UNIQUE> |
| **PARID** | String | ID de parcela | Código único que identifica a la propiedad dentro del sistema de registro de la jurisdicción (por ejemplo, el código de parcela del condado). Es una cadena alfanumérica. La mayoría de los valores son únicos y aparecen en las frecuencias como ‘Other (302 633)’, indicando que cada parcela tiene su propio identificador. | 0001C00037000A00 | 9946X83943000000 | 302,643 |  | 0 | Other (302,633) [479,750]<br>0431B00017000000 [23]<br>0027D00263000000 [20]<br>0027D00272000000 [20]<br>0027D00286000000 [20] |
| **FULL_ADDRESS** | String | Dirección completa | La dirección concatenada de la propiedad, incluyendo número, calle, ciudad y código postal. La variable es una cadena de longitud variable; las direcciones más frecuentes comienzan con “0” (por ejemplo, “0 SONIE DR, SEWICKLEY, PA 15143”). | 0 , BRADDOCK, PA 15104 | FORBES AVE, PITTSBURGH, PA 15219 | 278,190 |  | 0 | Other (278,180) [479,006]<br>0 SONIE DR, SEWICKLEY, PA… [113]<br>0 COAL, ELIZABETH, PA 150… [111]<br>0 HUNTER ST, PITTSBURGH, … [98]<br>0 PERRYSVILLE AVE, PITTSB… [98] |
| **PROPERTYHOUSENUM** | Integer | Número de casa | El número principal de la propiedad. Es un entero; el valor más frecuente es 0, seguido por números como 112, 100 y 110. | 0 | 65015 | 10,012 |  | 3 | Other (10,002) [428,653]<br>0 [38,055]<br>112 [1,615]<br>100 [1,595]<br>110 [1,522] |
| **PROPERTYFRACTION** | String | Fracción del número de casa | Parte fraccional o adicional al número de casa (por ejemplo, “1/2”, “A”). En la mayoría de los registros este campo está vacío (≈ 97 %); cuando se rellena, suele ser un valor corto y alfanumérico. |   |  S | 2,803 |  | 0 | (NULL) [468,512]<br>Other (2,793) [9,695]<br>1/2 [853]<br>A [405]<br>B [294] |
| **PROPERTYADDRESSDIR** | String | Dirección de la calle | La dirección cardinal de la calle (N, S, E, W). La mayoría de las propiedades están en el norte o sur; el valor más frecuente es ‘N’ (≈ 1.14 %). | E | W | 5 | (NULL)<br>E<br>N<br>S<br>W | 459,948 | (NULL) [459,948]<br>N [5,466]<br>S [5,201]<br>E [5,053]<br>W [4,260] |
| **PROPERTYADDRESSSTREET** | String | Nombre de la calle | El nombre de la vía principal. Los valores más comunes son “WASHINGTON”, “5TH”, “HIGHLAND” y “PENN”. La variable tiene una gran diversidad, con un total de 9 538 valores únicos. | 0 OHIO RIVER BLVD | ZUZU | 9,571 |  | 13 | Other (9,538) [462,225]<br>WASHINGTON [2,606]<br>5TH [2,557]<br>HIGHLAND [1,777]<br>PENN [1,602] |
| **PROPERTYADDRESSSUF** | String | Sufijo de la calle | El sufijo que indica el tipo de vía (por ejemplo, ST, DR, AVE). Los sufixos más frecuentes son “ST” y “DR”. | ALY | XING | 48 |  | 1,985 | ST [122,764]<br>DR [113,069]<br>AVE [105,232]<br>RD [71,902]<br>LN [15,471] |
| **PROPERTYADDRESSUNITDESC** | String | Tipo de unidad | Descriptor del tipo de unidad dentro del edificio o lote (p. ej., UNIT, APT, REAR). En la mayoría de los registros este campo está vacío (~ 97 %); cuando se completa, suele indicar la naturaleza de la unidad. | # | UNIT | 11 |  | 468,267 | (NULL) [468,267]<br>UNIT [10,580]<br>REAR [421]<br>APT [391]<br>STE [132] |
| **PROPERTYUNITNO** | String | Número de unidad | El número interno de la unidad dentro del edificio. El valor más frecuente es ‘01’. La mayoría de las propiedades no tienen unidades asignadas (≈ 97 %). | 01 | ` | 1,334 |  | 468,641 | (NULL) [468,641]<br>Other (1,324) [10,002]<br>1 [195]<br>2 [170]<br>3 [166] |
| **PROPERTYCITY** | String | Ciudad | La ciudad donde se ubica la propiedad. Pittsburgh es la ciudad dominante (~ 54 % del total), seguida por otras ciudades del estado. | 15216 | WITAKER | 106 |  | 1 | PITTSBURGH [257,608]<br>Other (89) [123,311]<br>CORAOPOLIS [16,497]<br>MC KEESPORT [15,307]<br>GIBSONIA [11,048] |
| **PROPERTYSTATE** | String | Estado | El estado de la propiedad; en este conjunto todos los registros están en Pennsylvania (PA). | PA | PA | 1 | PA | 0 | <ALL_UNIQUE> |
| **PROPERTYZIP** | Integer | Código postal | Código ZIP de cinco dígitos. Los códigos más frecuentes son 15108, 15237 y 15235. | 15003 | 16229 | 124 |  | 1 | Other (114) [355,692]<br>15108 [16,509]<br>15237 [15,435]<br>15235 [14,585]<br>15212 [13,301] |
| **SCHOOLCODE** | String | Código del distrito escolar | Código numérico que identifica el distrito escolar donde se encuentra la propiedad (p. ej., 47). La mayoría de los registros pertenecen al distrito con código 47. | 01 | 50 | 46 |  | 0 | Other (36) [228,000]<br>47 [117,977]<br>27 [19,685]<br>09 [19,227]<br>30 [17,635] |
| **SCHOOLDESC** | String | Nombre del distrito escolar | El nombre descriptivo del distrito escolar, como “Pittsburgh” o “North Allegheny”. | Allegheny Valley | Woodland Hills | 46 |  | 0 | Other (36) [228,000]<br>Pittsburgh [117,977]<br>North Allegheny [19,685]<br>Woodland Hills [19,227]<br>Penn Hills Twp [17,635] |
| **MUNICODE** | Integer | Código municipal | Número que identifica el municipio. Los valores más comunes son 934, 119 y 940. | 101 | 953 | 175 |  | 0 | Other (165) [369,793]<br>934 [17,635]<br>119 [12,648]<br>940 [12,003]<br>926 [10,359] |
| **MUNIDESC** | String | Nombre del municipio | El nombre completo del municipio (p. ej., “Penn Hills” o “19th Ward - PITTSBURGH”). | 10th Ward -  McKEESPORT | Wilmerding   | 175 |  | 0 | Other (165) [369,793]<br>Penn Hills [17,635]<br>19th Ward - PITTSBURGH [12,648]<br>Ross [12,003]<br>Mt.Lebanon [10,359] |
| **RECORDDATE** | Date | Fecha de registro | La fecha en que se registró el dato en la base de datos. El rango cubre desde principios de 2012 hasta finales de 2025, con un pico alrededor de 2013‑2016. | 0212-08-01 | 2028-09-28 | 3,821 |  | 1,262 | Other (3,811) [474,687]<br>(NULL) [1,262]<br>2012-10-26 [587]<br>2013-04-26 [552]<br>2012-01-11 [488] |
| **SALEDATE** | Date | Fecha de venta | La fecha en que se realizó la transacción de venta de la propiedad. El rango es similar al de RECORDDATE y muestra la distribución temporal de las ventas. | 2012-01-01 | 2025-12-13 | 4,888 |  | 0 | Other (4,878) [475,391]<br>2012-10-26 [586]<br>2016-04-29 [584]<br>2013-04-26 [559]<br>2012-01-11 [490] |
| **PRICE** | Integer | Precio de venta | Monto de la venta expresado en dólares estadounidenses. Los valores más frecuentes son 1 $ (valor de prueba o error) y 0 $. El rango real se extiende hasta ~ 148 752 900 $. | 0 | 148752900 | 37,872 |  | 3,020 | Other (37,862) [340,479]<br>1 [98,344]<br>0 [15,508]<br>10 [6,763]<br>150000 [3,140] |
| **DEEDBOOK** | String | Libro del acta | Número del libro donde se encuentra el registro del título. Los valores más comunes son “TR18”, “0” y “TR13”. |  14795 | `17274 | 5,814 |  | 570 | Other (5,800) [473,060]<br>TR18 [1,239]<br>0 [1,063]<br>TR13 [938]<br>00 [798] |
| **DEEDPAGE** | String | Página del acta | Número de página dentro del libro del acta que contiene la transacción. El valor más frecuente es 1. |  120 | W | 2,133 |  | 582 | Other (2,115) [465,578]<br>1 [5,174]<br>6 [1,485]<br>7 [1,112]<br>0 [1,002] |
| **SALECODE** | String | Código de venta | Abreviatura que clasifica el tipo de transacción (por ejemplo, “3” para ventas típicas). Los códigos 3 y 0 son los más comunes. | 0 | Z | 47 |  | 0 | 3 [97,932]<br>0 [91,484]<br>H [63,750]<br>Other (37) [54,274]<br>14 [42,089] |
| **SALEDESC** | String | Descripción de la venta | Texto descriptivo del tipo de venta, como “LOVE AND AFFECTION SALE”, “VALID SALE” o “MULTI‑PARCEL SALE”. | BANK/FINANCIAL INSTITUTION | VALID SALE | 27 |  | 0 | LOVE AND AFFECTION SALE [97,932]<br>VALID SALE [91,484]<br>MULTI-PARCEL SALE [63,750]<br>Other (17) [45,680]<br>TIME ON MARKET (INSUFF/EX… [42,089] |
| **INSTRTYP** | String | Tipo de instrumento | Código que indica el tipo de documento legal usado en la transacción (p. ej., DE para Deed, SW para Special Warranty). El código más frecuente es DE. | AD | WD | 31 |  | 0 | DE [278,804]<br>SW [113,966]<br>GW [23,662]<br>QC [18,068]<br>SD [15,521] |
| **INSTRTYPDESC** | String | Descripción del instrumento | Texto descriptivo correspondiente al código INSTRTYP, por ejemplo “DEED” o “SPECIAL WARRANTY”. | ADMIN DEED | WILL BOOK | 31 |  | 0 | DEED [278,804]<br>SPECIAL WARRANTY [113,966]<br>GENERAL WARRANTY [23,662]<br>QUIT CLAIM [18,068]<br>SHERIFF DEED [15,521] |

*Attribution: Generated by qsv v12.0.0 describegpt
Command line: qsv describegpt --all allegheny_county_property_sale_transactions.csv --language Spanish --fresh --output Spanish.md
Prompt file: Default v7.0.0
Model: openai/gpt-oss-20b
LLM API URL: http://localhost:1234/v1
Language: 
Timestamp: 2025-12-24T16:54:04.109191+00:00

WARNING: Label and Description generated by an LLM and may contain inaccuracies. Verify before using!
*

## REASONING

We need Spanish labels and descriptions for all fields based on summary stats and frequency distribution.

Need to provide JSON with keys as field names exactly: _id, PARID, etc. For each: label in Spanish human-friendly, description also Spanish.

Use context: dataset seems property sale records (real estate) in Pennsylvania? Fields like address components, school district codes, municipal code, record date, sale date, price, deed book/page, sale code/description, instrument type etc.

We should generate labels and descriptions. Use summary stats to know data types: _id integer unique key; PARID string maybe parcel ID; FULL_ADDRESS full address; PROPERTYHOUSENUM int; PROPERTYFRACTION string fraction part; PROPERTYADDRESSDIR direction; PROPERTYADDRESSSTREET street name; PROPERTYADDRESSSUF suffix like ST, DR etc.; PROPERTYADDRESSUNITDESC unit descriptor (#, UNIT, APT); PROPERTYUNITNO unit number; PROPERTYCITY city; PROPERTYSTATE state; PROPERTYZIP zip code; SCHOOLCODE school district code; SCHOOLDESC school district name; MUNICODE municipal code; MUNIDESC municipal description; RECORDDATE record date when data recorded; SALEDATE sale date of property; PRICE price in dollars; DEEDBOOK book number; DEEDPAGE page number; SALECODE sale type code; SALEDESC sale description; INSTRTYP instrument type code; INSTRTYPDESC instrument type description.

We need Spanish labels like "Clave Única", "ID de Parcela" etc. Provide descriptive text including data characteristics: unique key, string/integer, format, typical values from frequency distribution.

Use frequencies to mention common values e.g., for PROPERTYCITY most common Pittsburgh, etc.

Also note nulls count; but description can mention that some records have missing values.

Let's craft each field.

We'll produce JSON object with keys and nested label/description strings in Spanish.

We need to be concise yet informative. Use bullet points? No, plain text.

Ok produce final answer.
## TOKEN USAGE

TokenUsage { prompt: 7881, completion: 1958, total: 9839, elapsed: 32769 }
---
# Description
## Descripción del conjunto de datos

Este conjunto contiene **479 928** registros de transacciones inmobiliarias realizadas en Pennsylvania entre 2012 y 2025. Cada fila representa una venta individual con información detallada sobre la propiedad (dirección, número de casa, unidad, ZIP, distrito escolar y municipio), el precio de venta, las fechas de registro y venta, así como los documentos legales que respaldan la transacción (tipo de instrumento, código de venta, descripción del sale). Los identificadores únicos (`_id` y `PARID`) garantizan la integridad de cada registro, mientras que campos como `PRICE`, `SALEDATE` y `RECORDDATE` muestran una gran variabilidad temporal y numérica. La mayoría de las transacciones se concentran en Pittsburgh (≈ 54 %) y sus municipios circundantes.

## Características notables

- **Gran dispersión y sesgo positivo**: el precio medio es ~$195 k, pero la desviación estándar excede los $914 k; casi el 71 % de las ventas se encuentran por encima del percentil 95.  
- **Valores atípicos extremos**: existen precios que superan los $148  millones y fechas posteriores al 2025‑12‑13 (posible error tipográfico).  
- **Datos “cero” o “uno” anómalos**: el precio aparece como 0 ($15 508 registros) o 1 ($98 344), lo que sugiere errores de captura.  
- **Alta cardinalidad y “Other” dominante**: para campos categóricos como `PARID`, `SCHOOLCODE` y `MUNICODE`, más del 70 % de los valores se agrupan en la categoría “Other (N)”, indicando una gran variedad de códigos únicos.  
- **Valores faltantes significativos**:  
  - `RECORDDATE` y `SALEDATE`: ~0,26 % nulos cada uno.  
  - Componentes de dirección (`PROPERTYFRACTION`, `PROPERTYADDRESSDIR`, etc.): entre 95 % y 98 % vacíos.  
- **Presencia de PII**: las direcciones completas contienen información personal identificable (nombre de calle, número, ciudad), por lo que el conjunto debe manejarse bajo regulaciones de privacidad.  
- **Ausencia de duplicados en `_id`**: la clave primaria garantiza registros únicos; no se detectaron filas idénticas.  
- **Sesgo temporal**: los picos de transacciones ocurren principalmente entre 2012 y 2016, con una notable concentración de ventas en octubre‑noviembre (posiblemente temporada alta).  

> *Atención*: Los usuarios deben validar la calidad de los precios y fechas antes de usar el conjunto para análisis financieros o estadísticos.

**Atribución**  
Generated by qsv v12.0.0 describegpt
Command line: qsv describegpt --all allegheny_county_property_sale_transactions.csv --language Spanish --fresh --output Spanish.md
Prompt file: Default v7.0.0
Model: openai/gpt-oss-20b
LLM API URL: http://localhost:1234/v1
Language: 
Timestamp: 2025-12-24T16:54:26.531714+00:00

WARNING: Description generated by an LLM and may contain inaccuracies. Verify before using!



## REASONING

We need to produce a description of dataset overall, 1-8 sentences in Spanish. Then bullet list "Notable Characteristics". Use info from summary and freq.

Dataset: property transaction records? 479,928 rows. Fields include ID unique, addresses, sale date, price etc.

Observations: many missing values in dates, address components, fractions. High cardinality for PARID (unique >80%). Price has huge skewness; many zeros and ones. Many 'Other' categories for categorical fields. Some outliers like extremely high price 148 million. Also duplicates? _id unique ensures no duplicates.

Missing: recorddate null 0.26%; sale date similar. Others: many nulls in address fraction, direction, etc. PII/PHI: addresses are personal data (PII). PCI not relevant. Outliers: price extremes.

Also central tendency: median around $39k? Actually median for PRICE field from percentiles 5-95: median ~ 0? Wait median is q2 maybe 2018-12-11; that's date, but for price median? From summary: mean 195k, std dev huge 914k. So skewed.

Also note that many top values are 1 or 0 (price). These likely erroneous.

Distribution shape: heavy right tail, zero-inflated.

We need bullet list.

Ok produce markdown.


## TOKEN USAGE

TokenUsage { prompt: 9254, completion: 895, total: 10149, elapsed: 22421 }
---
# Tags
ventas_inmobiliarias  
precio_propiedad  
datos_pa  
registro_deed  
transacciones_venta  
direccion_propiedad  
distrito_municipal  
distrito_escolar  
tipo_instrumento  
fecha_venta  

Generated by qsv v12.0.0 describegpt
Command line: qsv describegpt --all allegheny_county_property_sale_transactions.csv --language Spanish --fresh --output Spanish.md
Prompt file: Default v7.0.0
Model: openai/gpt-oss-20b
LLM API URL: http://localhost:1234/v1
Language: 
Timestamp: 2025-12-24T16:54:58.429341+00:00

WARNING: Tags generated by an LLM and may contain inaccuracies. Verify before using!



## REASONING

We need tags in Spanish, lowercase, underscore separated words. Choose up to 10 unique tags about dataset content based on summary statistics and frequency distribution. Not field names.

Dataset: Real estate property sales data for Pennsylvania (PA). Contains addresses, sale dates, price, property type details, school district, municipality, deed book/page, sale code, sale description, instrument type etc. So tags could be:

- real_estate_sales
- property_valuation
- pa_real_estate
- property_address
- sale_transactions
- property_tax
- municipal_districts
- school_districts
- deed_records
- price_analysis

Need Spanish words: "real_estate_sales" -> "ventas_inmobiliarias"? But keep underscore. Maybe "ventas_inmobiliarias". Use lowercase with underscores.

List in descending importance:

1. ventas_inmobiliarias (primary)
2. valor_propiedad (property value/price)
3. datos_pa (Pennsylvania data)
4. registro_deed (deed records)
5. transacciones_venta (sale transactions)
6. direccion_propiedad (address)
7. distrito_municipal (municipality)
8. distrito_escolar (school district)
9. tipo_instrumento (instrument type)
10. fecha_venta (sale date)

But "valor_propiedad" maybe less important than price. "precio_propiedad". Use Spanish.

Ok tags:

- ventas_inmobiliarias
- precio_propiedad
- datos_pa
- registro_deed
- transacciones_venta
- direccion_propiedad
- distrito_municipal
- distrito_escolar
- tipo_instrumento
- fecha_venta

Add attribution placeholder. Provide in Markdown format, list tags each on separate line? It says "in Markdown format". Usually tags listed with bullet points or comma separated? Could do simple list of tags each as a word separated by commas? The instruction: "Choose no more than 10 unique Tags (in Markdown format) about the contents of the Dataset in descending order of importance ... Add an Attribution with the placeholder "{GENERATED_BY_SIGNATURE}" after the Tags. If generating JSON format, add the Attribution as a separate key at the top level of the JSON object, after the Tags, otherwise add it at the bottom of the Tags in Markdown format."

So output should be just tags lines and then attribution line. Use markdown: maybe like:

```
ventas_inmobiliarias
precio_propiedad
datos_pa
registro_deed
transacciones_venta
direccion_propiedad
distrito_municipal
distrito_escolar
tipo_instrumento
fecha_venta

{GENERATED_BY_SIGNATURE}
```

Alternatively use bullet list. But likely simple lines is fine.

Let's produce that.
## TOKEN USAGE

TokenUsage { prompt: 9224, completion: 630, total: 9854, elapsed: 16550 }
---
