Generated using a Local LLM (openai/gpt-oss-20b) on LM Studio 0.3.34 Build 1 running on a Macbook Pro M4 Max 64gb/Tahoe 26.2:

```bash
$ QSV_LLM_BASE_URL=http://localhost:1234/v1 qsv describegpt NYC_311_SR_2010-2020-sample-1M.csv --all \
     --language Spanish \
     --output nyc311-describegpt-spanish.md
```
---
# Dictionary
| Name | Type | Label | Description | Min | Max | Cardinality | Enumeration | Null Count | Examples |
|------|------|-------|-------------|-----|-----|-------------|-------------|------------|----------|
| **Unique Key** | Integer | Clave única | Identificador numérico que distingue de forma inequívoca cada registro del conjunto de datos. Se trata de una clave primaria generada automáticamente y no se repite en ningún otro registro. | 11465364 | 48478173 | 1,000,000 |  | 0 | <ALL_UNIQUE> |
| **Created Date** | DateTime | Fecha de creación | Momento en el que la solicitud o reclamo fue registrado en el sistema 311. El rango cubre desde principios de 2010 hasta finales de 2020, con valores frecuentes alrededor del año 2013‑2015. | 2010-01-01T00:00:00+00:00 | 2020-12-23T01:25:51+00:00 | 841,014 |  | 0 | Other (841,004) [997,333]<br>01/24/2013 12:00:00 AM [347]<br>01/07/2014 12:00:00 AM [315]<br>01/08/2015 12:00:00 AM [283]<br>02/16/2015 12:00:00 AM [269] |
| **Closed Date** | DateTime | Fecha de cierre | Timestamp que indica cuándo se resolvió o cerró la solicitud. Muchos registros son nulos (≈2 861), lo que sugiere que aún no se completaron, mientras que los valores presentes suelen encontrarse en el rango 2010‑2021. | 1900-01-01T00:00:00+00:00 | 2100-01-01T00:00:00+00:00 | 688,837 |  | 28,619 | Other (688,827) [968,897]<br>(NULL) [28,619]<br>11/15/2010 12:00:00 AM [384]<br>11/07/2012 12:00:00 AM [329]<br>12/09/2010 12:00:00 AM [267] |
| **Agency** | String | Agencia responsable | Departamento o entidad gubernamental encargada de responder al reclamo. Los más comunes son NYPD (26 %), HPD (26 %) y DOT (13 %). Existen casos de “Other” que representan diversas agencias menores. | 3-1-1 | TLC | 28 |  | 0 | NYPD [265,116]<br>HPD [258,033]<br>DOT [132,462]<br>DSNY [81,606]<br>DEP [75,895] |
| **Agency Name** | String | Nombre de la agencia | Denominación completa del organismo responsable. Los nombres más frecuentes son "New York City Police Department" y "Department of Housing Preservation and Development", entre otros. | 3-1-1 | Valuation Policy | 553 |  | 0 | New York City Police Depa… [265,038]<br>Department of Housing Pre… [258,019]<br>Department of Transportat… [132,462]<br>Other (543) [103,974]<br>Department of Environment… [75,895] |
| **Complaint Type** | String | Tipo de reclamo | Categoría que describe la naturaleza del problema reportado, por ejemplo “Noise - Residential”, “HEAT/HOT WATER” o “Illegal Parking”. La mayoría de los registros pertenecen a las diez primeras categorías. | ../../WEB-INF/web.xml;x= | ZTESTINT | 287 |  | 0 | Other (277) [563,561]<br>Noise - Residential [89,439]<br>HEAT/HOT WATER [56,639]<br>Illegal Parking [45,032]<br>Blocked Driveway [42,356] |
| **Descriptor** | String | Descripción detallada | Subcategoría que especifica el problema dentro del tipo general, como "Loud Music/Party" o "Street Light Out". Los valores más frecuentes cubren una variedad de situaciones urbanas comunes. | 1 Missed Collection | unknown odor/taste in drinking water (QA6) | 1,392 |  | 3,001 | Other (1,382) [674,871]<br>Loud Music/Party [93,646]<br>ENTIRE BUILDING [36,885]<br>HEAT [35,088]<br>No Access [31,631] |
| **Location Type** | String | Tipo de ubicación | Clasificación del lugar donde ocurrió el reclamo: residencial, callejón, parque, etc. La categoría más frecuente es "RESIDENTIAL BUILDING" (≈26 %). | 1-, 2- and 3- Family Home | Wooded Area | 162 |  | 239,131 | RESIDENTIAL BUILDING [255,562]<br>(NULL) [239,131]<br>Street/Sidewalk [145,653]<br>Residential Building/Hous… [92,765]<br>Street [92,190] |
| **Incident Zip** | String | Código postal del incidente | Código ZIP asociado al lugar de la incidencia. La mayoría de los registros son valores específicos, pero existe una gran proporción de “Other” que agrupa códigos menos frecuentes. | * | XXXXX | 535 |  | 54,978 | Other (525) [827,654]<br>(NULL) [54,978]<br>11226 [17,114]<br>10467 [14,495]<br>11207 [12,872] |
| **Incident Address** | String | Dirección completa del incidente | Texto libre con la dirección donde ocurrió el reclamo. Se observa una alta variabilidad y un alto porcentaje de valores únicos, lo cual indica direcciones muy específicas o faltantes (≈17 %). | * * | west 155 street and edgecombe avenue | 341,996 |  | 174,700 | Other (341,986) [819,378]<br>(NULL) [174,700]<br>655 EAST  230 STREET [1,538]<br>78-15 PARSONS BOULEVARD [694]<br>672 EAST  231 STREET [663] |
| **Street Name** | String | Nombre de la calle principal | Nombre de la vía más prominente en el lugar del incidente. Los nombres más comunes incluyen "BROADWAY", "GRAND CONCOURSE" y otras calles de alta frecuencia. | * | wyckoff avenue | 14,837 |  | 174,720 | Other (14,827) [787,222]<br>(NULL) [174,720]<br>BROADWAY [9,702]<br>GRAND CONCOURSE [5,851]<br>OCEAN AVENUE [3,946] |
| **Cross Street 1** | String | Calle cruzada 1 | Segunda calle que cruza o se encuentra cercana a la ubicación principal. La mayoría son valores comunes como "BROADWAY", “3 AVENUE”, etc., pero también hay un amplio espectro de calles menos frecuentes. | 1 AVE | mermaid | 16,238 |  | 320,401 | Other (16,228) [623,317]<br>(NULL) [320,401]<br>BEND [12,562]<br>BROADWAY [8,548]<br>3 AVENUE [6,154] |
| **Cross Street 2** | String | Calle cruzada 2 | Tercera calle relevante al incidente. Al igual que Cross Street 1, se observan valores repetidos y una gran cantidad de códigos “Other” para casos poco comunes. | 1 AVE | surf | 16,486 |  | 323,644 | Other (16,476) [626,168]<br>(NULL) [323,644]<br>BEND [12,390]<br>BROADWAY [8,833]<br>DEAD END [5,626] |
| **Intersection Street 1** | String | Intersección calle 1 | Primera calle en la intersección donde ocurrió el reclamo. Los datos incluyen calles muy habituales como "BROADWAY" y una amplia variedad de otras vías. | 1 AVE | flatlands AVE | 11,237 |  | 767,422 | (NULL) [767,422]<br>Other (11,227) [215,482]<br>BROADWAY [3,761]<br>CARPENTER AVENUE [2,918]<br>BEND [2,009] |
| **Intersection Street 2** | String | Intersección calle 2 | Segunda calle en la intersección relevante al incidente, con un patrón similar a Intersection Street 1. | 1 AVE | glenwood RD | 11,674 |  | 767,709 | (NULL) [767,709]<br>Other (11,664) [216,748]<br>BROADWAY [3,462]<br>BEND [1,942]<br>2 AVENUE [1,690] |
| **Address Type** | String | Tipo de dirección | Clasificación del formato de dirección: "ADDRESS", "INTERSECTION", etc. La mayoría corresponde a direcciones completas (≈71 %). | ADDRESS | PLACENAME | 6 | (NULL)<br>ADDRESS<br>BLOCKFACE<br>INTERSECTION<br>LATLONG<br>PLACENAME | 125,802 | ADDRESS [710,380]<br>INTERSECTION [133,361]<br>(NULL) [125,802]<br>BLOCKFACE [22,620]<br>LATLONG [7,421] |
| **City** | String | Ciudad o distrito | Distrito en el que se localiza el incidente, con valores típicos como "BROOKLYN", "NEW YORK" y "BRONX". Hay una gran proporción de registros “Other” que agrupan distritos menos frecuentes. | * | YORKTOWN HEIGHTS | 382 |  | 61,963 | BROOKLYN [296,254]<br>NEW YORK [189,069]<br>BRONX [181,168]<br>Other (372) [171,028]<br>(NULL) [61,963] |
| **Landmark** | String | Punto de referencia | Referencia notable cercana al lugar del reclamo, como nombres de calles o edificios emblemáticos. La mayoría son valores únicos y se observa una alta variabilidad. | 1 AVENUE | ZULETTE AVENUE | 5,915 |  | 912,779 | (NULL) [912,779]<br>Other (5,905) [80,508]<br>EAST  230 STREET [1,545]<br>EAST  231 STREET [1,291]<br>BROADWAY [1,148] |
| **Facility Type** | String | Tipo de instalación | Clasificación de la infraestructura donde ocurrió el incidente: "DSNY Garage", "Precinct" u otros. La categoría dominante es “N/A” (≈63 %). | DSNY Garage | School District | 6 | (NULL)<br>DSNY Garage<br>N/A<br>Precinct<br>School<br>School District | 145,478 | N/A [628,279]<br>Precinct [193,259]<br>(NULL) [145,478]<br>DSNY Garage [32,310]<br>School [617] |
| **Status** | String | Estado del reclamo | Fase actual del proceso de resolución, por ejemplo "Closed", "Pending", "Open" o "In Progress". La mayoría de los registros están cerrados. | Assigned | Unspecified | 10 | Assigned<br>Closed<br>Closed - Testing<br>Email Sent<br>In Progress<br>Open<br>Pending<br>Started<br>Unassigned<br>Unspecified | 0 | Closed [952,522]<br>Pending [20,119]<br>Open [12,340]<br>In Progress [7,841]<br>Assigned [6,651] |
| **Due Date** | DateTime | Fecha límite | Plazo estimado para resolver el reclamo. Un gran número de valores es nulo (≈65 %) y las fechas más comunes se encuentran en la década del 2010‑2021. | 1900-01-02T00:00:00+00:00 | 2021-06-17T16:34:13+00:00 | 345,077 |  | 647,794 | (NULL) [647,794]<br>Other (345,067) [350,849]<br>04/08/2015 10:00:58 AM [214]<br>05/02/2014 03:32:17 PM [183]<br>03/30/2018 10:10:39 AM [172] |
| **Resolution Description** | String | Descripción de la resolución | Texto explicativo sobre la acción tomada o el resultado final. Incluye mensajes como “The Police Department responded to the complaint…”, con una gran variedad de frases y un alto porcentaje de registros “Other”. | A DOB violation was issued for failing to comply with an existing Stop Work Order. | Your request was submitted to the Department of Homeless Services. The City?s outreach team will assess the homeless individual and offer appropriate assistance within 2 hours. If you asked to know the outcome of your request, you will get a call within 2 hours. No further status will be available through the NYC 311 App, 311, or 311 Online. | 1,216 |  | 20,480 | Other (1,206) [532,002]<br>The Police Department res… [91,408]<br>The Department of Housing… [72,962]<br>The Police Department res… [63,868]<br>Service Request status fo… [52,155] |
| **Resolution Action Updated Date** | DateTime | Fecha de actualización de la resolución | Timestamp del último cambio en el estado o la descripción de la resolución. La mayoría es nula, indicando que no hubo actualizaciones posteriores. | 2009-12-31T01:35:00+00:00 | 2020-12-23T06:56:14+00:00 | 690,314 |  | 15,072 | Other (690,304) [982,378]<br>(NULL) [15,072]<br>11/15/2010 12:00:00 AM [385]<br>11/07/2012 12:00:00 AM [336]<br>12/09/2010 12:00:00 AM [273] |
| **Community Board** | String | Junta comunitaria | Número identificador de la junta comunitaria correspondiente al distrito donde ocurrió el incidente. Los valores más frecuentes incluyen los números 0 (especificado) y 12, 01, 03, etc. | 0 Unspecified | Unspecified STATEN ISLAND | 77 |  | 0 | Other (67) [751,635]<br>0 Unspecified [49,878]<br>12 MANHATTAN [29,845]<br>12 QUEENS [23,570]<br>01 BROOKLYN [21,714] |
| **BBL** | String | Código BBL (Borough/Block/Lot) | Identificador único de bloque y lote dentro del borough. La mayoría de registros son valores específicos; la categoría “Other” agrupa los bloques menos comunes. | 0000000000 | 5270000501 | 268,383 |  | 243,046 | Other (268,373) [751,031]<br>(NULL) [243,046]<br>2048330028 [1,566]<br>4068290001 [696]<br>4015110001 [664] |
| **Borough** | String | Borough | Distrito (borough) donde se localiza el incidente: "BROOKLYN", "QUEENS", "MANHATTAN", "BRONX" o "STATEN ISLAND". Los valores más frecuentes son los cuatro primeros. | BRONX | Unspecified | 6 | BRONX<br>BROOKLYN<br>MANHATTAN<br>QUEENS<br>STATEN ISLAND<br>Unspecified | 0 | BROOKLYN [296,081]<br>QUEENS [228,818]<br>MANHATTAN [195,488]<br>BRONX [180,142]<br>Unspecified [49,878] |
| **X Coordinate (State Plane)** | Integer | Coordenada X (Plano Estatal) | Coordenada longitudinal en el sistema de referencia del Plano Estatal de Nueva York, expresada en pies. La mayoría de los registros son valores numéricos específicos; la categoría “Other” agrupa valores poco frecuentes. | 913281 | 1067220 | 102,556 |  | 85,327 | Other (102,546) [908,877]<br>(NULL) [85,327]<br>1022911 [1,568]<br>1037000 [701]<br>1023174 [675] |
| **Y Coordinate (State Plane)** | Integer | Coordenada Y (Plano Estatal) | Coordenada latitudinal en el sistema de referencia del Plano Estatal de Nueva York, expresada en pies. Al igual que X, la mayoría son valores numéricos concretos. | 121152 | 271876 | 116,092 |  | 85,327 | Other (116,082) [908,868]<br>(NULL) [85,327]<br>264242 [1,566]<br>202363 [706]<br>211606 [665] |
| **Open Data Channel Type** | String | Tipo de canal de datos abierto | Medio por el cual se envió la solicitud: "PHONE", "ONLINE", "MOBILE" o "UNKNOWN". La categoría más frecuente es "PHONE" (≈50 %). | MOBILE | UNKNOWN | 5 | MOBILE<br>ONLINE<br>OTHER<br>PHONE<br>UNKNOWN | 0 | PHONE [497,606]<br>UNKNOWN [230,402]<br>ONLINE [177,334]<br>MOBILE [79,892]<br>OTHER [14,766] |
| **Park Facility Name** | String | Nombre del parque | Denominación de la instalación de recreo donde ocurrió el reclamo, por ejemplo “Central Park” o “Riverside Park”. La mayoría de los registros son valores únicos. | "Uncle" Vito F. Maranzano Glendale Playground | Zimmerman Playground | 1,889 |  | 0 | Unspecified [993,141]<br>Other (1,879) [5,964]<br>Central Park [261]<br>Riverside Park [136]<br>Prospect Park [129] |
| **Park Borough** | String | Borough del parque | Distrito en el que se encuentra el parque. Los valores habituales coinciden con los boroughs típicos de la ciudad. | BRONX | Unspecified | 6 | BRONX<br>BROOKLYN<br>MANHATTAN<br>QUEENS<br>STATEN ISLAND<br>Unspecified | 0 | BROOKLYN [296,081]<br>QUEENS [228,818]<br>MANHATTAN [195,488]<br>BRONX [180,142]<br>Unspecified [49,878] |
| **Vehicle Type** | String | Tipo de vehículo | Clasificación del vehículo involucrado, por ejemplo "Car Service" o "Ambulette / Paratransit". La mayoría son registros nulos (≈99 %). | Ambulette / Paratransit | Green Taxi | 5 | (NULL)<br>Ambulette / Paratransit<br>Car Service<br>Commuter Van<br>Green Taxi | 999,652 | (NULL) [999,652]<br>Car Service [317]<br>Ambulette / Paratransit [19]<br>Commuter Van [11]<br>Green Taxi [1] |
| **Taxi Company Borough** | String | Borough de la compañía de taxi | Distrito donde opera la compañía de taxis asociada al incidente. Los valores más frecuentes son los boroughs habituales. | BRONX | Staten Island | 11 |  | 999,156 | (NULL) [999,156]<br>BROOKLYN [207]<br>QUEENS [194]<br>MANHATTAN [171]<br>BRONX [127] |
| **Taxi Pick Up Location** | String | Ubicación de recogida del taxi | Lugar donde el pasajero fue recogido, con opciones como "JFK Airport", "Intersection" o direcciones específicas. La mayoría de los registros son valores únicos. | 1 5 AVENUE MANHATTAN | YORK AVENUE AND EAST 70 STREET | 1,903 |  | 992,129 | (NULL) [992,129]<br>Other [4,091]<br>Other (1,893) [2,021]<br>JFK Airport [562]<br>Intersection [486] |
| **Bridge Highway Name** | String | Nombre de la autopista o puente | Denominación del tramo de carretera donde ocurrió el incidente, por ejemplo “Belt Pkwy” o “Grand Central Pkwy”. | 145th St. Br - Lenox Ave | Willis Ave Br - 125th St/1st Ave | 68 |  | 997,711 | (NULL) [997,711]<br>Other (58) [851]<br>Belt Pkwy [276]<br>BQE/Gowanus Expwy [254]<br>Grand Central Pkwy [186] |
| **Bridge Highway Direction** | String | Dirección de la autopista/puente | Sentido en que se dirige la vía: "East/Long Island Bound", "North/Bronx Bound", etc. | Bronx Bound | Westbound/To Goethals Br | 50 |  | 997,691 | (NULL) [997,691]<br>Other (40) [1,064]<br>East/Long Island Bound [210]<br>North/Bronx Bound [208]<br>East/Queens Bound [197] |
| **Road Ramp** | String | Rampas de carretera | Tipo de acceso a la carretera, como “Roadway” o “Ramp”. La mayoría de los registros son valores nulos (≈99 %). | N/A | Roadway | 4 | (NULL)<br>N/A<br>Ramp<br>Roadway | 997,693 | (NULL) [997,693]<br>Roadway [1,731]<br>Ramp [555]<br>N/A [21] |
| **Bridge Highway Segment** | String | Segmento de autopista/puente | Descripción del tramo específico de la vía, por ejemplo "Ramp" o nombres de salida. | 1-1-1265963747 | Wythe Ave/Kent Ave (Exit 31) | 937 |  | 997,556 | (NULL) [997,556]<br>Other (927) [2,159]<br>Ramp [92]<br>Roadway [54]<br>Clove Rd/Richmond Rd (Exi… [23] |
| **Latitude** | Float | Latitud | Coordenada geográfica en grados decimales que indica la posición norte‑sur del incidente. El rango se sitúa entre 40.112 y 40.913, con una gran proporción de valores únicos. | 40.1123853 | 40.9128688 | 353,694 |  | 254,695 | Other (353,684) [739,574]<br>(NULL) [254,695]<br>40.89187241649303 [1,538]<br>40.1123853 [1,153]<br>40.89238451539139 [663] |
| **Longitude** | Float | Longitud | Coordenada geográfica en grados decimales que indica la posición este‑oeste del incidente. El rango abarca desde -77.519 hasta -73.700, con una alta variabilidad de valores. | -77.5195844 | -73.7005968 | 353,996 |  | 254,695 | Other (353,986) [739,574]<br>(NULL) [254,695]<br>-73.86016845296459 [1,538]<br>-77.5195844 [1,153]<br>-73.8592161325675 [663] |
| **Location** | String | Ubicación (coordenadas) | Representación textual de la latitud y longitud en formato "(lat, lon)". La mayoría de los registros corresponden a coordenadas únicas; algunos están marcados como “Other” cuando no se dispone de datos. | (40.1123853, -77.5195844) | (40.9128688, -73.9024731) | 375,772 |  | 254,695 | Other (375,762) [739,574]<br>(NULL) [254,695]<br>(40.89187241649303, -73.8… [1,538]<br>(40.1123853, -77.5195844) [1,153]<br>(40.89238451539139, -73.8… [663] |

*Attribution: Generated by qsv v11.0.2 describegpt
Command line: target/debug/qsv describegpt --all NYC_311_SR_2010-2020-sample-1M.csv --language Spanish --output nyc311-describegpt-spanish.md
Prompt file: Default v6.3.0
Model: openai/gpt-oss-20b
LLM API URL: http://localhost:1234/v1
Timestamp: 2025-12-13T19:00:25.502605+00:00

WARNING: Label and Description generated by an LLM and may contain inaccuracies. Verify before using!*

## REASONING

We need to produce Spanish labels and descriptions for each field in the dataset. We have summary stats and frequency distribution. Need to generate JSON mapping each field name to label and description.

We must use context: e.g., Unique Key is primary key. Created Date, Closed Date timestamps. Agency names etc. Provide human-friendly label and description (multiple sentences). Spanish. Use data info from frequency distribution to mention common values maybe. But we should give general description. Should include field purpose, possible values.

We have many fields; list them: Unique Key, Created Date, Closed Date, Agency, Agency Name, Complaint Type, Descriptor, Location Type, Incident Zip, Incident Address, Street Name, Cross Street 1, Cross Street 2, Intersection Street 1, Intersection Street 2, Address Type, City, Landmark, Facility Type, Status, Due Date, Resolution Description, Resolution Action Updated Date, Community Board, BBL, Borough, X Coordinate (State Plane), Y Coordinate (State Plane), Open Data Channel Type, Park Facility Name, Park Borough, Vehicle Type, Taxi Company Borough, Taxi Pick Up Location, Bridge Highway Name, Bridge Highway Direction, Road Ramp, Bridge Highway Segment, Latitude, Longitude, Location.

We need to produce JSON. Should include all fields; ensure keys match exactly as given in summary stats. Use Spanish labels like "Clave única", "Fecha de creación". Provide description referencing dataset context: New York City 311 complaints? The dataset appears NYC 311 data (complaints). So label and description accordingly.

Let's craft for each field.

We must consider frequency distribution for e.g., Agency most frequent: NYPD etc. Might mention typical values.

Also note some fields have many nulls; we can mention that.

Ok produce JSON. Let's write.


## TOKEN USAGE

TokenUsage { prompt: 13169, completion: 2882, total: 16051, elapsed: 50486 }
---
# Description
**Descripción del conjunto de datos**

Este conjunto contiene 1 000 000 de registros de quejas y solicitudes realizadas a través del sistema NYC‑311 entre enero de 2010 y diciembre de 2020. Cada registro incluye información sobre la agencia responsable, el tipo de problema, la ubicación (dirección, calles cruzadas e intersecciones), datos geográficos (latitud, longitud y coordenadas en plano estatal) y el estado de la solicitud (cerrada, pendiente u abierta). La mayoría de los incidentes se reportan en barrios residenciales, con “RESIDENTIAL BUILDING” representando el 25 % del total. Los valores más frecuentes para las fechas de creación y cierre están concentrados entre 2013‑2015, mientras que la fecha límite suele ser nula en dos tercios de los registros.

**Características notables**

- **Gran cardinalidad y “Other” dominante**: campos como *Incident Address*, *Street Name* y *Cross Street 1/2* presentan más del 70 % de valores etiquetados como “Other (N)”, indicando una amplia variedad de entradas únicas o poco frecuentes.
- **Distribución sesgada**: la mayoría de los campos exhiben una distribución altamente asimétrica, con un pico en el valor principal y colas extensas de valores menores. El sesgo positivo se observa especialmente en *Latitude* y *Longitude* debido a la presencia de valores atípicos fuera del rango típico de Manhattan.
- **Valores faltantes**:  
  - *Closed Date*: 2 861 registros (0,29 %) nulos, lo que sugiere que muchas solicitudes no han sido cerradas.  
  - *Due Date*: 647 794 registros (64,78 %) nulos, indicando ausencia frecuente de plazos establecidos.  
  - *Resolution Description* y *Resolution Action Updated Date*: también presentan valores nulos en un pequeño porcentaje.
- **Outliers geográficos**: la presencia de coordenadas con longitud ≈ ‑77 o latitud fuera del rango 40‑41 sugiere datos erróneos o entradas no relacionadas con NYC.  
- **Datos potencialmente sensibles**: aunque el conjunto carece de identificadores personales explícitos, las direcciones completas y las coordenadas GPS pueden revelar ubicaciones de hogares o negocios, lo que requiere precaución al compartir los datos.
- **Unicidad de clave**: el campo *Unique Key* garantiza la ausencia de duplicados; cada registro es único por diseño.  
- **Temporalidad**: la media de la fecha de creación se sitúa alrededor del 10 noviembre de 2015, con una desviación estándar de ~1 mes, reflejando un flujo constante de quejas a lo largo de la década.
- **Calidad de datos**: existen valores inconsistentes en campos como *Agency* y *Agency Name* (por ejemplo, “TLC” sin nombre completo), así como la presencia de categorías “Unspecified” o “Other (N)” que podrían requerir limpieza o normalización.

---


Generated by qsv v11.0.2 describegpt
Command line: target/debug/qsv describegpt --all NYC_311_SR_2010-2020-sample-1M.csv --language Spanish --output nyc311-describegpt-spanish.md
Prompt file: Default v6.3.0
Model: openai/gpt-oss-20b
LLM API URL: http://localhost:1234/v1
Timestamp: 2025-12-13T19:00:51.220645+00:00

WARNING: Generated by an LLM and may contain inaccuracies. Verify before using!



## REASONING

We need to produce a dataset description and notable characteristics. Use summary stats and freq distributions. We must not output per-field details. Must be 1-8 sentences.

Let's craft Spanish description: talk about NYC 311 complaints data, 1M records, timeframe 2010-2020, many fields etc. Mention high cardinality in some fields like Incident Address, Street Name, etc. Note missing values percentages (Closed Date 2.86% null, Due Date 64.78% null, etc). Distribution: most fields have high uniqueness or "Other" categories.

Also mention that data contains PII? Probably addresses and phone numbers maybe but not present; but location coordinates could be considered sensitive. Provide note about duplicates? Unique Key unique so no duplicates.

Outliers: Latitude/Longitude ranges; some coordinate values outside city (like -77 lat?). Actually -77 long is far west, maybe missing or erroneous.

Also mention skewness: many fields have heavy right-skewed distribution with many "Other" categories. Provide central tendency: mean of created date around 2015-11-10.

Ok produce description and bullet list.

Let's produce in Spanish Markdown.

Add attribution placeholder at bottom.


## TOKEN USAGE

TokenUsage { prompt: 15692, completion: 902, total: 16594, elapsed: 25716 }
---
# Tags
nyc_311  
reclamos  
datos_abiertos  
ciudad_ny  
servicios_publicos  
ubicacion_geografica  
tipo_reclamo  
agencia_governamental  
control_ruido  
infracciones  


Generated by qsv v11.0.2 describegpt
Command line: target/debug/qsv describegpt --all NYC_311_SR_2010-2020-sample-1M.csv --language Spanish --output nyc311-describegpt-spanish.md
Prompt file: Default v6.3.0
Model: openai/gpt-oss-20b
LLM API URL: http://localhost:1234/v1
Timestamp: 2025-12-13T19:01:13.684510+00:00

WARNING: Generated by an LLM and may contain inaccuracies. Verify before using!



## REASONING

We need to produce Spanish tags, no more than 10, descending importance. Use Summary Statistics and Frequency Distribution to guide.

Dataset appears to be NYC 311 service requests dataset: complaints about noise, heating, parking, etc. Key attributes include complaint type, location, agency, status, city boroughs, etc. The most frequent values: Complaint Type (Noise - Residential), Agency (NYPD, HPD, DOT). City: Brooklyn and New York.

Also many unique fields but we need tags.

We must pick up to 10 Spanish tags separated by underscores, all lowercase. Should be descriptive of dataset content. Based on data characteristics: "ciudad", "quejas", "servicios públicos", "NYC", "311", "reclamos", "municipio", "datos abiertos". Also maybe "infracciones", "seguridad pública", "infraestructura urbana".

We need descending importance: likely most important tag is "nyc_311" or "data_abiertos_nyc". But must be Spanish. Maybe "reclamos_nyc_311" but that has underscores.

Let's think: The dataset is about complaints in NYC, so tags: "nyc", "311", "reclamos", "ciudad_ny", "servicios_publicos", "datos_abiertos", "agencia_governamental", "ubicacion_geografica", "tipo_reclamo".

We need to choose no more than 10. Let's order by importance:

1. nyc_311 (most specific)
2. reclamos
3. datos_abiertos
4. ciudad_ny
5. servicios_publicos
6. ubicacion_geografica
7. tipo_reclamo
8. agencia_governamental
9. infracciones
10. noise_control maybe "control_ruido" but less.

But we need to think which are more important: The dataset's unique key is 311, so "nyc_311" stands out. Next "reclamos". Then "datos_abiertos". City and boroughs are significant. So choose tags:

- nyc_311
- reclamos
- datos_abiertos
- ciudad_ny
- servicios_publicos
- ubicacion_geografica
- tipo_reclamo
- agencia_governamental
- infracciones (maybe not as common)
- ruido

But "infracciones" might refer to violations. Many complaints are about noise, heating etc. Could include "ruido". But we can use "control_ruido".

Let's produce tags: 10.

Order:

1. nyc_311
2. reclamos
3. datos_abiertos
4. ciudad_ny
5. servicios_publicos
6. ubicacion_geografica
7. tipo_reclamo
8. agencia_governamental
9. control_ruido
10. infracciones

But check Spanish: "nyc" is English but still used; we can keep it as "nyc". All lowercase.

Let's output in markdown tags separated by spaces? The instruction says "in Markdown format" and each tag likely separate lines or space-separated? Usually tags list maybe bullet points? Provide as a list with each tag on new line. Let's produce:

```
nyc_311
reclamos
datos_abiertos
ciudad_ny
servicios_publicos
ubicacion_geografica
tipo_reclamo
agencia_governamental
control_ruido
infracciones
{GENERATED_BY_SIGNATURE}
```

Make sure to add attribution after tags. Use newline. Let's do that.
## TOKEN USAGE

TokenUsage { prompt: 15653, completion: 802, total: 16455, elapsed: 22457 }
---
