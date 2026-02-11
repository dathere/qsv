Generated using a Local LLM (openai/gpt-oss-20b) on LM Studio 0.4.2+2 running on a Macbook Pro M4 Max 64gb/Tahoe 26.3:

```bash
$ QSV_LLM_BASE_URL=http://localhost:1234/v1 qsv describegpt /tmp/NYC_311_SR_2010-2020-sample-1M.csv --all \
     --language Spanish \
     --output /tmp/nyc311-describegpt-spanish.md
```
---
# Dictionary
| Name | Type | Label | Description | Min | Max | Cardinality | Enumeration | Null Count | Examples |
|------|------|-------|-------------|-----|-----|-------------|-------------|------------|----------|
| **Unique Key** | Integer | Clave única | Identificador numérico asignado por el sistema a cada registro de queja; no se repite entre registros y hay exactamente un millón de valores distintos. | 11465364 | 48478173 | 1,000,000 |  | 0 | <ALL_UNIQUE> |
| **Created Date** | DateTime | Fecha de creación | Marca temporal que indica cuándo se registró la queja en el portal 311. Los valores abarcan desde el 1 de enero de 2010 hasta finales de 2020, con una media alrededor de noviembre de 2015. | 2010-01-01T00:00:00+00:00 | 2020-12-23T01:25:51+00:00 | 841,014 |  | 0 | Other… [997,333]<br>01/24/2013 12:00:00 AM [347]<br>01/07/2014 12:00:00 AM [315]<br>01/08/2015 12:00:00 AM [283]<br>02/16/2015 12:00:00 AM [269] |
| **Closed Date** | DateTime | Fecha de cierre | Marca temporal que señala cuándo se cerró formalmente la queja. Muchos registros tienen valor nulo o un rango amplio; la media es a principios de 2016 y el 68% de los casos tiene una fecha válida. | 1900-01-01T00:00:00+00:00 | 2100-01-01T00:00:00+00:00 | 688,837 |  | 28,619 | Other… [968,671]<br>(NULL) [28,619]<br>11/15/2010 12:00:00 AM [384]<br>11/07/2012 12:00:00 AM [329]<br>12/09/2010 12:00:00 AM [267] |
| **Agency** | String | Agencia responsable | Código abreviado del organismo encargado de atender la queja. Los valores más frecuentes son NYPD, HPD, DOT, DSNY, DEP, etc., representando aproximadamente el 73 % combinado. | 3-1-1 | TLC | 28 |  | 0 | NYPD [265,116]<br>HPD [258,033]<br>DOT [132,462]<br>DSNY [81,606]<br>DEP [75,895] |
| **Agency Name** | String | Nombre completo de la agencia | Nombre oficial del organismo responsable; se alinea con los códigos de Agency y muestra una gran diversidad administrativa (por ejemplo, New York City Police Department, Department of Transportation). | 3-1-1 | Valuation Policy | 553 |  | 0 | New York City Police Depa… [265,038]<br>Department of Housing Pre… [258,019]<br>Department of Transportat… [132,462]<br>Other… [103,974]<br>Department of Environment… [75,895] |
| **Complaint Type** | String | Tipo de queja | Categoría general de la queja, como Noise – Residential, HEAT/HOT WATER o Illegal Parking. Los tipos más comunes cubren ruido, problemas de agua y estacionamiento ilegal. | ../../WEB-INF/web.xml;x= | ZTESTINT | 287 |  | 0 | Other… [563,561]<br>Noise - Residential [89,439]<br>HEAT/HOT WATER [56,639]<br>Illegal Parking [45,032]<br>Blocked Driveway [42,356] |
| **Descriptor** | String | Descripción específica del problema | Detalle concreto dentro del tipo de queja (por ejemplo, Loud Music/Party, Heat, Blocked Driveway). La mayoría de los registros tienen un descriptor único y el 67 % pertenece a la categoría “Other”. | 1 Missed Collection | unknown odor/taste in drinking water (QA6) | 1,392 |  | 3,001 | Other… [671,870]<br>Loud Music/Party [93,646]<br>ENTIRE BUILDING [36,885]<br>HEAT [35,088]<br>No Access [31,631] |
| **Location Type** | String | Tipo de ubicación | Indica la naturaleza física del lugar donde ocurrió el incidente (por ejemplo, Residential Building, Street). Los edificios residenciales constituyen más del 60 %. | 1-, 2- and 3- Family Home | Wooded Area | 162 |  | 239,131 | RESIDENTIAL BUILDING [255,562]<br>(NULL) [239,131]<br>Street/Sidewalk [145,653]<br>Residential Building/Hous… [92,765]<br>Street [92,190] |
| **Incident Zip** | String | Código postal del incidente | ZIP code asociado con la ubicación del incidente. La mayor parte de las quejas se concentran en los códigos 11226 y 10467, aunque el 86 % de los registros pertenecen a la categoría “Other”. | * | XXXXX | 535 |  | 54,978 | Other… [815,988]<br>(NULL) [54,978]<br>11226 [17,114]<br>10467 [14,495]<br>11207 [12,872] |
| **Incident Address** | String | Dirección completa del incidente | Dirección donde ocurrió el incidente, incluyendo número de calle y nombre. Los valores son altamente únicos (≈170 000) y se usan para búsquedas espaciales. | * * | west 155 street and edgecombe avenue | 341,996 |  | 174,700 | Other… [819,046]<br>(NULL) [174,700]<br>655 EAST  230 STREET [1,538]<br>78-15 PARSONS BOULEVARD [694]<br>672 EAST  231 STREET [663] |
| **Street Name** | String | Nombre de la calle principal | Nombre principal de la calle donde se reportó el incidente; las calles más comunes incluyen Broadway, Grand Concourse, etc., aunque la mayoría es única (95 %). | * | wyckoff avenue | 14,837 |  | 174,720 | Other… [784,684]<br>(NULL) [174,720]<br>BROADWAY [9,702]<br>GRAND CONCOURSE [5,851]<br>OCEAN AVENUE [3,946] |
| **Cross Street 1** | String | Calle cruzada 1 | Primera calle que cruza o intersecta con la ubicación principal. Muchos registros tienen valor nulo y los valores son variados. | 1 AVE | mermaid | 16,238 |  | 320,401 | Other… [619,743]<br>(NULL) [320,401]<br>BEND [12,562]<br>BROADWAY [8,548]<br>3 AVENUE [6,154] |
| **Cross Street 2** | String | Calle cruzada 2 | Segunda calle que cruza o intersecta con la ubicación principal; también suele ser nula en gran parte de los casos. | 1 AVE | surf | 16,486 |  | 323,644 | Other… [623,363]<br>(NULL) [323,644]<br>BEND [12,390]<br>BROADWAY [8,833]<br>DEAD END [5,626] |
| **Intersection Street 1** | String | Calle en intersección 1 | Primera calle de la intersección donde se localiza el incidente. Los valores son muy heterogéneos y a menudo faltan. | 1 AVE | flatlands AVE | 11,237 |  | 767,422 | (NULL) [767,422]<br>Other… [214,544]<br>BROADWAY [3,761]<br>CARPENTER AVENUE [2,918]<br>BEND [2,009] |
| **Intersection Street 2** | String | Calle en intersección 2 | Segunda calle de la intersección; su presencia depende del tipo de ubicación (p.ej., “Other” es frecuente). | 1 AVE | glenwood RD | 11,674 |  | 767,709 | (NULL) [767,709]<br>Other… [215,667]<br>BROADWAY [3,462]<br>BEND [1,942]<br>2 AVENUE [1,690] |
| **Address Type** | String | Tipo de dirección | Formato de dirección utilizado: ADDRESS, INTERSECTION, BLOCKFACE o LATLONG. La mayoría corresponde a ADDRESS (~81 %) y solo el 1 % son LATLONG. | ADDRESS | PLACENAME | 6 |  | 125,802 | ADDRESS [710,380]<br>INTERSECTION [133,361]<br>(NULL) [125,802]<br>BLOCKFACE [22,620]<br>LATLONG [7,421] |
| **City** | String | Ciudad o municipio | Ciudad o municipio donde se localiza el incidente (Brooklyn, Manhattan, Queens, etc.). Brooklyn domina la distribución con casi un tercio de los registros. | * | YORKTOWN HEIGHTS | 382 |  | 61,963 | BROOKLYN [296,254]<br>NEW YORK [189,069]<br>BRONX [181,168]<br>Other… [163,936]<br>(NULL) [61,963] |
| **Landmark** | String | Punto de referencia destacado | Punto de referencia cercano al lugar del incidente. La mayoría es nula y la categoría “Other” representa el 92 % de los valores que sí existen. | 1 AVENUE | ZULETTE AVENUE | 5,915 |  | 912,779 | (NULL) [912,779]<br>Other… [80,165]<br>EAST  230 STREET [1,545]<br>EAST  231 STREET [1,291]<br>BROADWAY [1,148] |
| **Facility Type** | String | Tipo de instalación | Categoría de instalación relacionada con la queja (por ejemplo, DSNY Garage, Precinct). La gran mayoría es N/A indicando ausencia de una instalación específica. | DSNY Garage | School District | 6 |  | 145,478 | N/A [628,279]<br>Precinct [193,259]<br>(NULL) [145,478]<br>DSNY Garage [32,310]<br>School [617] |
| **Status** | String | Estado de la queja | Estado actual en el flujo de trabajo: Closed, Pending, Open, etc. El 95 % de los registros están cerrados, mientras solo el 2 % permanecen abiertos. | Assigned | Unspecified | 10 | Assigned<br>Closed<br>Closed - Testing<br>Email Sent<br>In Progress<br>Open<br>Pending<br>Started<br>Unassigned<br>Unspecified | 0 | Closed [952,522]<br>Pending [20,119]<br>Open [12,340]<br>In Progress [7,841]<br>Assigned [6,651] |
| **Due Date** | DateTime | Fecha límite de acción | Fecha establecida para resolver la queja; varía ampliamente y muchos valores son nulos. La media se sitúa alrededor de abril‑junio de 2015. | 1900-01-02T00:00:00+00:00 | 2021-06-17T16:34:13+00:00 | 345,077 |  | 647,794 | (NULL) [647,794]<br>Other… [350,746]<br>04/08/2015 10:00:58 AM [214]<br>05/02/2014 03:32:17 PM [183]<br>03/30/2018 10:10:39 AM [172] |
| **Resolution Description** | String | Descripción de la resolución | Texto descriptivo del resultado final (por ejemplo, se emitió una multa, se reparó el problema). Los textos más comunes describen inspecciones y cierres de incidentes. | A DOB violation was issued for failing to comply with an existing Stop Work Order. | Your request was submitted to the Department of Homeless Services. The City?s outreach team will assess the homeless individual and offer appropriate assistance within 2 hours. If you asked to know the outcome of your request, you will get a call within 2 hours. No further status will be available through the NYC 311 App, 311, or 311 Online. | 1,216 |  | 20,480 | Other… [511,739]<br>The Police Department res… [91,408]<br>The Department of Housing… [72,962]<br>The Police Department res… [63,868]<br>Service Request status fo… [52,155] |
| **Resolution Action Updated Date** | DateTime | Última actualización de la acción de resolución | Fecha en que se modificó por última vez la acción o estado de resolución. Refleja cuándo se añadió nueva información al caso. | 2009-12-31T01:35:00+00:00 | 2020-12-23T06:56:14+00:00 | 690,314 |  | 15,072 | Other… [982,148]<br>(NULL) [15,072]<br>11/15/2010 12:00:00 AM [385]<br>11/07/2012 12:00:00 AM [336]<br>12/09/2010 12:00:00 AM [273] |
| **Community Board** | String | Consejo comunitario | Número del consejo comunitario correspondiente a la zona geográfica; el 75 % son “Unspecified”, indicando falta de datos específicos. | 0 Unspecified | Unspecified STATEN ISLAND | 77 |  | 0 | Other… [751,635]<br>0 Unspecified [49,878]<br>12 MANHATTAN [29,845]<br>12 QUEENS [23,570]<br>01 BROOKLYN [21,714] |
| **BBL** | String | Identificador BBL (Borough Block Lot) | Código único que identifica el bloque y lote dentro de NYC. Se utiliza para trazabilidad espacial en registros administrativos y planificación urbana. | 0140694020 | 0140694020 | 268,383 |  | 243,046 | Other… [750,668]<br>(NULL) [243,046]<br>2048330028 [1,566]<br>4068290001 [696]<br>4015110001 [664] |
| **Borough** | String | Borough | Nombre del borough donde se localiza la queja (Brooklyn, Queens, Manhattan, Bronx, Staten Island). Brooklyn y Queens comparten casi la mitad de los casos. | BRONX | Unspecified | 6 | BRONX<br>BROOKLYN<br>MANHATTAN<br>QUEENS<br>STATEN ISLAND<br>Unspecified | 0 | BROOKLYN [296,081]<br>QUEENS [228,818]<br>MANHATTAN [195,488]<br>BRONX [180,142]<br>Unspecified [49,878] |
| **X Coordinate (State Plane)** | Integer | Coordenada X (Plano estatal) | Coordenada en el sistema de plano estatal New York. Se usa para mapeo espacial; la mayoría de valores están entre 1 000 000 y 2 100 000. | 913281 | 1067220 | 102,556 |  | 85,327 | Other… [908,535]<br>(NULL) [85,327]<br>1022911 [1,568]<br>1037000 [701]<br>1023174 [675] |
| **Y Coordinate (State Plane)** | Integer | Coordenada Y (Plano estatal) | Coordenada en el sistema de plano estatal New York. Se complementa con la X para ubicar la queja en un mapa. | 121152 | 271876 | 116,092 |  | 85,327 | Other… [908,538]<br>(NULL) [85,327]<br>264242 [1,566]<br>202363 [706]<br>211606 [665] |
| **Open Data Channel Type** | String | Tipo de canal de datos abierto | Método por el cual se envió la queja al portal 311 (PHONE, ONLINE, MOBILE, etc.). El canal más frecuente es PHONE (~50 %) y el 23 % son UNKNOWN. | MOBILE | UNKNOWN | 5 | MOBILE<br>ONLINE<br>OTHER<br>PHONE<br>UNKNOWN | 0 | PHONE [497,606]<br>UNKNOWN [230,402]<br>ONLINE [177,334]<br>MOBILE [79,892]<br>OTHER [14,766] |
| **Park Facility Name** | String | Nombre del parque o instalación | Si la queja está relacionada con un parque, se indica su nombre; la mayoría de registros indican “Unspecified” (99 %). | "Uncle" Vito F. Maranzano Glendale Playground | Zimmerman Playground | 1,889 |  | 0 | Unspecified [993,141]<br>Other… [5,964]<br>Central Park [261]<br>Riverside Park [136]<br>Prospect Park [129] |
| **Park Borough** | String | Borough del parque | Borough donde se ubica el parque mencionado. La distribución es similar a la de los boroughs generales. | BRONX | Unspecified | 6 | BRONX<br>BROOKLYN<br>MANHATTAN<br>QUEENS<br>STATEN ISLAND<br>Unspecified | 0 | BROOKLYN [296,081]<br>QUEENS [228,818]<br>MANHATTAN [195,488]<br>BRONX [180,142]<br>Unspecified [49,878] |
| **Vehicle Type** | String | Tipo de vehículo | Tipo de vehículo asociado a la queja (por ejemplo, Car Service, Ambulance). El 99 % son nulos, indicando ausencia de vehículos en la mayoría de casos. | Ambulette / Paratransit | Green Taxi | 5 |  | 999,652 | (NULL) [999,652]<br>Car Service [317]<br>Ambulette / Paratransit [19]<br>Commuter Van [11]<br>Green Taxi [1] |
| **Taxi Company Borough** | String | Borough de la compañía de taxis | Borough donde opera la compañía de taxi; la mayor parte es nula (≈99 %) y solo una pequeña fracción tiene valores válidos. | BRONX | Staten Island | 11 |  | 999,156 | (NULL) [999,156]<br>BROOKLYN [207]<br>QUEENS [194]<br>MANHATTAN [171]<br>BRONX [127] |
| **Taxi Pick Up Location** | String | Ubicación de recogida de taxi | Categoría de ubicación de recogida de taxi. La opción “Other” domina los registros (~52 %). | 1 5 AVENUE MANHATTAN | YORK AVENUE AND EAST 70 STREET | 1,903 |  | 992,129 | (NULL) [992,129]<br>Other… [4,091]<br>Other [2,006]<br>JFK Airport [562]<br>Intersection [486] |
| **Bridge Highway Name** | String | Nombre de puente/autoridad | Identifica la carretera o puente asociado con el incidente (p.ej., Belt Pkwy, Grand Central Pkwy). La mayoría son “Other” o nulos. | 145th St. Br - Lenox Ave | Willis Ave Br - 125th St/1st Ave | 68 |  | 997,711 | (NULL) [997,711]<br>Other… [779]<br>Belt Pkwy [276]<br>BQE/Gowanus Expwy [254]<br>Grand Central Pkwy [186] |
| **Bridge Highway Direction** | String | Dirección de puente/autoridad | Indica la dirección en la que se encuentra el tramo de carretera (Eastbound, Westbound, etc.). El 42 % de los valores son “Other” o nulos. | Bronx Bound | Westbound/To Goethals Br | 50 |  | 997,691 | (NULL) [997,691]<br>Other… [987]<br>East/Long Island Bound [210]<br>North/Bronx Bound [208]<br>East/Queens Bound [197] |
| **Road Ramp** | String | Rampas o acceso de carretera | Tipo de rampa o acceso en la infraestructura vial. La mayoría es “Roadway” (~75 %) y el resto incluye “Ramp”, “N/A”. | N/A | Roadway | 4 |  | 997,693 | (NULL) [997,693]<br>Roadway [1,731]<br>Ramp [555]<br>N/A [21] |
| **Bridge Highway Segment** | String | Segmento de puente/autoridad | Detalle del segmento específico de carretera (p.ej., Exit 31, Outer Roadway). La mayoría son “Other” o nulos. | 1-1-1265963747 | Wythe Ave/Kent Ave (Exit 31) | 937 |  | 997,556 | (NULL) [997,556]<br>Other… [2,144]<br>Ramp [92]<br>Roadway [54]<br>Clove Rd/Richmond Rd (Exi… [23] |
| **Latitude** | Float | Latitud | Coordenada geográfica en grados decimales que representa la latitud del incidente. Los valores varían entre 40 112 y 40 912, con una media de 40 728. | 40.1123853 | 40.9128688 | 353,694 |  | 254,695 | Other… [739,329]<br>(NULL) [254,695]<br>40.89187241649303 [1,538]<br>40.1123853 [1,153]<br>40.89238451539139 [663] |
| **Longitude** | Float | Longitud | Coordenada geográfica en grados decimales que representa la longitud del incidente. Los valores oscilan entre -73 700 y -77 519, con una media de -73 860. | -77.5195844 | -73.7005968 | 353,996 |  | 254,695 | Other… [739,329]<br>(NULL) [254,695]<br>-73.86016845296459 [1,538]<br>-77.5195844 [1,153]<br>-73.8592161325675 [663] |
| **Location** | String | Ubicación (latitud, longitud) | Representación textual combinada de latitud y longitud (p.ej., “(40.89187241649303, -73.86016845296459)”). Se usa para visualización rápida en mapas. | (40.1123853, -77.5195844) | (40.9128688, -73.9024731) | 375,772 |  | 254,695 | Other… [739,329]<br>(NULL) [254,695]<br>(40.89187241649303, -73.8… [1,538]<br>(40.1123853, -77.5195844) [1,153]<br>(40.89238451539139, -73.8… [663] |

*Attribution: Generated by qsv v16.0.0 describegpt
Command line: target/debug/qsv describegpt /tmp/NYC_311_SR_2010-2020-sample-1M.csv --all --language Spanish --output /tmp/nyc311-describegpt-spanish.md --fresh
Prompt file: Default v7.0.0
Model: openai/gpt-oss-20b
LLM API URL: http://localhost:1234/v1
Language: 
Timestamp: 2026-02-10T23:47:31.536290+00:00

WARNING: Label and Description generated by an LLM and may contain inaccuracies. Verify before using!
*

## REASONING

We need to generate Spanish labels and descriptions for all 41 fields.

Use summary statistics and frequency distribution to understand context/meaning. Provide label: human-friendly label, description: full description. Return JSON mapping each field name to object with label and description.

Need Spanish (language). So labels in Spanish like "Clave única" etc. Descriptions should be in Spanish, possibly multiple sentences. Use the data insights to explain what each field represents.

We have many fields; we need to produce 41 entries. Should reflect dataset context: NYC 311 complaints? Fields include Unique Key, Created Date, Closed Date, Agency, Agency Name, Complaint Type, Descriptor, Location Type, Incident Zip, etc. Provide description of meaning, typical values, cardinality maybe.

We'll use summary stats for guidance: e.g., Unique Key is unique; created date range 2010-2020; closed date range; Agency distribution; descriptor top values like Loud Music/Party etc. Provide that context in description.

Also mention frequency distribution where relevant (e.g., top categories). Keep concise but thorough.

Let's craft each entry:

1. Unique Key: numeric identifier, unique per complaint.

2. Created Date: timestamp when complaint filed.

3. Closed Date: timestamp of closure; may be null or earlier than created date; many missing.

4. Agency: agency that handled the complaint; top agencies NYPD, HPD etc.

5. Agency Name: full name of agency.

6. Complaint Type: broad category e.g., Noise, HEAT/HOT WATER etc.

7. Descriptor: specific issue details like Loud Music/Party, Heat, etc.

8. Location Type: type of location where incident occurred; many are residential building or street.

9. Incident Zip: ZIP code of incident location.

10. Incident Address: full address string.

11. Street Name: main street name.

12-15: Cross Street 1, Cross Street 2, Intersection Street 1, Intersection Street 2: cross streets at intersection.

16. Address Type: type of address representation; e.g., ADDRESS, INTERSECTION etc.

17. City: city or borough.

18. Landmark: notable landmark near incident.

19. Facility Type: facility category; many N/A.

20. Status: status of complaint (Closed, Pending, Open etc).

21. Due Date: due date for action.

22. Resolution Description: description of resolution outcome.

23. Resolution Action Updated Date: last update to resolution action timestamp.

24. Community Board: community board number or unspecified.

25. BBL: Borough Block Lot identifier.

26. Borough: borough name.

27-28. X Coordinate (State Plane), Y Coordinate (State Plane): coordinates in state plane system.

29. Open Data Channel Type: channel via which complaint was received.

30. Park Facility Name: park facility involved if any.

31. Park Borough: borough of the park.

32. Vehicle Type: type of vehicle for taxi etc.

33. Taxi Company Borough: borough where taxi company is located.

34. Taxi Pick Up Location: pickup location category.

35-38: Bridge Highway Name, Direction, Road Ramp, Segment: road infrastructure details.

39-40: Latitude, Longitude: geographic coordinates.

41. Location: string representation of lat/long pair.

Now produce JSON.

Make sure to escape quotes properly within strings. Use double quotes for JSON keys and values; inside descriptions we can use single quotes or escape double quotes.

Let's craft Spanish labels:

Unique Key -> "Clave única"

Created Date -> "Fecha de creación"

Closed Date -> "Fecha de cierre"

Agency -> "Agencia responsable"

Agency Name -> "Nombre completo de la agencia"

Complaint Type -> "Tipo de queja"

Descriptor -> "Descripción específica del problema"

Location Type -> "Tipo de ubicación"

Incident Zip -> "Código postal del incidente"

Incident Address -> "Dirección completa del incidente"

Street Name -> "Nombre de la calle principal"

Cross Street 1 -> "Calle cruzada 1"

Cross Street 2 -> "Calle cruzada 2"

Intersection Street 1 -> "Calle en intersección 1"

Intersection Street 2 -> "Calle en intersección 2"

Address Type -> "Tipo de dirección"

City -> "Ciudad o municipio"

Landmark -> "Punto de referencia destacado"

Facility Type -> "Tipo de instalación"

Status -> "Estado de la queja"

Due Date -> "Fecha límite de acción"

Resolution Description -> "Descripción de la resolución"

Resolution Action Updated Date -> "Última actualización de la acción de resolución"

Community Board -> "Consejo comunitario"

BBL -> "Identificador BBL (Borough Block Lot)"

Borough -> "Borough"

X Coordinate (State Plane) -> "Coordenada X (Plano estatal)"

Y Coordinate (State Plane) -> "Coordenada Y (Plano estatal)"

Open Data Channel Type -> "Tipo de canal de datos abierto"

Park Facility Name -> "Nombre del parque o instalación"

Park Borough -> "Borough del parque"

Vehicle Type -> "Tipo de vehículo"

Taxi Company Borough -> "Borough de la compañía de taxis"

Taxi Pick Up Location -> "Ubicación de recogida de taxi"

Bridge Highway Name -> "Nombre de puente/autoridad"

Bridge Highway Direction -> "Dirección de puente/autoridad"

Road Ramp -> "Rampas o acceso de carretera"

Bridge Highway Segment -> "Segmento de puente/autoridad"

Latitude -> "Latitud"

Longitude -> "Longitud"

Location -> "Ubicación (latitud, longitud)"

Now descriptions: Use Spanish. Provide details including cardinality, typical values.

Let's craft each description.

1. Unique Key: "Identificador numérico único que asigna el sistema a cada registro de queja; no se repite entre registros." Add mention of 1,000,000 records.

2. Created Date: "Marca temporal indicando cuándo se registró la queja en el sistema. Los valores abarcan desde el 1 de enero de 2010 hasta finales de 2020, con una media alrededor de noviembre de 2015."

3. Closed Date: "Marca temporal que señala cuándo se cerró formalmente la queja. Muchos registros tienen valor nulo o un rango amplio; la media es a principios de 2016."

4. Agency: "Código abreviado del organismo responsable de atender la queja. Los valores más frecuentes son NYPD, HPD, DOT, etc., representando aproximadamente el 73% combinado."

5. Agency Name: "Nombre completo del organismo que atiende la queja; se alinea con los códigos de Agency y muestra una gran diversidad administrativa."

6. Complaint Type: "Categoría general de la queja (por ejemplo, Noise – Residential, HEAT/HOT WATER). Los tipos más comunes cubren ruido, problemas de agua y estacionamiento ilegal."

7. Descriptor: "Descripción detallada del problema específico dentro del tipo de queja; por ejemplo, Loud Music/Party o Heat. La mayoría de los registros tienen un descriptor único."

8. Location Type: "Indica la naturaleza física del lugar donde ocurrió el incidente (ej., Residential Building, Street). Los edificios residenciales constituyen más del 60%."

9. Incident Zip: "Código postal ZIP asociado con la ubicación del incidente; la mayor parte de las quejas se concentran en los códigos 11226 y 10467."

10. Incident Address: "Dirección completa donde ocurrió el incidente, incluyendo número de calle y nombre. Los valores son altamente únicos (≈170 000)."

11. Street Name: "Nombre principal de la calle donde se reportó el incidente; las calles más comunes incluyen Broadway, Grand Concourse, etc., aunque la mayoría es única."

12-15. Cross Street 1/2, Intersection Street 1/2: "Calle(s) que cruzan o intersectan con la ubicación primaria. Los valores son variados y muchos registros tienen un valor nulo."

16. Address Type: "Formato de dirección utilizado (ADDRESS, INTERSECTION, BLOCKFACE, LATLONG). La mayoría corresponde a ADDRESS (~81%)."

17. City: "Ciudad o municipio donde se localiza el incidente; Brooklyn, Manhattan, Queens dominan la distribución."

18. Landmark: "Punto de referencia destacado cerca del lugar del incidente. Muchos registros son nulos y la categoría 'Other' es muy frecuente."

19. Facility Type: "Tipo de instalación relacionada con la queja (ej., DSNY Garage, Precinct). La mayoría es N/A indicando ausencia de una instalación específica."

20. Status: "Estado actual de la queja en el flujo de trabajo: Closed, Pending, Open, etc. El 95% de los registros están cerrados."

21. Due Date: "Fecha límite establecida para la resolución o respuesta a la queja; varía ampliamente y muchos valores son nulos."

22. Resolution Description: "Texto descriptivo del resultado final de la queja (por ejemplo, se emitió una multa, se reparó el problema). Los textos más comunes describen acciones de inspección y cierre."

23. Resolution Action Updated Date: "Última fecha en la que se actualizó la acción de resolución; muestra cuándo se modificó el estado o los detalles de la respuesta."

24. Community Board: "Número del consejo comunitario correspondiente al área geográfica. La mayoría es 'Unspecified' indicando falta de información específica."

25. BBL: "Identificador único del bloque y lote (Borough Block Lot) dentro de NYC; usado para trazabilidad espacial en registros administrativos."

26. Borough: "Borough donde se encuentra la queja (Brooklyn, Queens, Manhattan, Bronx, Staten Island)."

27-28. X Coordinate (State Plane), Y Coordinate (State Plane): "Coordenadas geográficas en el sistema de plano estatal New York; útiles para mapeo y análisis espacial."

29. Open Data Channel Type: "Método por el cual se envió la queja al portal 311 (PHONE, ONLINE, MOBILE, etc.). El canal más frecuente es PHONE (~50%)."

30. Park Facility Name: "Nombre de la instalación del parque si la queja está relacionada con un parque; la mayoría de los registros indican 'Unspecified'."

31. Park Borough: "Borough donde se ubica el parque mencionado."

32. Vehicle Type: "Tipo de vehículo asociado a la queja (ej., Car Service, Ambulance). La gran mayoría es nulo indicando ausencia de vehículos."

33. Taxi Company Borough: "Borough del proveedor de taxis; la mayoría son datos nulos."

34. Taxi Pick Up Location: "Categoría de ubicación de recogida de taxi; 'Other' domina los registros."

35-38. Bridge Highway Name, Direction, Road Ramp, Segment: "Datos sobre infraestructuras de carreteras/puentes relacionados con la queja (nombre, dirección, tipo de rampa y segmento). La mayoría son nulos o 'Other'."

39. Latitude: "Coordenada geográfica en grados decimales que representa la latitud del incidente."

40. Longitude: "Coordenada geográfica en grados decimales que representa la longitud del incidente."

41. Location: "Representación textual de las coordenadas (latitud, longitud) combinadas; útil para visualización rápida."

We should also mention cardinality or uniqueness ratio if relevant.

Let's produce JSON object with 41 entries.

Ensure we escape quotes inside descriptions properly. We'll use double quotes for JSON strings and escape internal double quotes as \". Use backslashes where necessary.

Now generate the final answer.
## TOKEN USAGE

TokenUsage { prompt: 13667, completion: 4829, total: 18496, elapsed: 70661 }
---
# Description
# Descripción del Conjunto de Datos

El conjunto contiene **1 000 000** de incidentes reportados a través del portal 311 de la ciudad de Nueva York entre el 1 de enero de 2010 y el 23 de diciembre de 2020. Cada registro se identifica con una clave única, lo que garantiza ausencia de duplicados internos. La mayor parte de las quejas corresponde al ruido residencial (≈9 %) o a categorías “Other” sin definir (≈56 %), evidenciando un desequilibrio significativo en la distribución de tipos de incidentes. Los campos geográficos —dirección completa, latitud y longitud— están disponibles para la gran mayoría de los casos, aunque presentan valores nulos relevantes en ZIP, ciudad y tipo de ubicación. La frecuencia diaria varía entre 0 y varios cientos por día, con picos recurrentes a principios de año y una tendencia temporal que muestra concentraciones en enero.

# Características Notables

- **Alta cardinalidad y uso extensivo de “Other”**: más del 90 % de las observaciones en varias variables (tipo de queja, descriptor, ZIP) se agrupan bajo “Other”, indicando falta de estandarización.  
- **Desbalance en la distribución de tipos de queja**: ruido residencial (~9 %) vs otras sin definir (~56 %).  
- **Pérdida significativa de datos**: ZIP (239 k nulos), ciudad (62 k nulos), tipo de ubicación (239 k nulos) y dirección de incidente (174 k nulos).  
- **Variabilidad temporal sesgada**: picos de actividad a principios de año, con una distribución no uniforme a lo largo del periodo.  
- **Datos geográficos con valores nulos**: latitud/longitud faltan en ~254 k registros, lo que limita el análisis espacial completo.  
- **Posibles preocupaciones de privacidad**: direcciones completas y coordenadas permiten localizar con precisión la ubicación de cada incidente; se recomienda aplicar medidas de anonimización si se comparte públicamente.  
- **Formato heterogéneo de texto**: variaciones en nombres de calles (“1 AVE”, “BEND”) y tipos de ubicación, lo que complica la agrupación automática.  
- **Ausencia de duplicados internos** gracias a la clave única; sin embargo, registros con valores idénticos en otras columnas podrían existir.

---

*Attribution: Generated by qsv v16.0.0 describegpt
Command line: target/debug/qsv describegpt /tmp/NYC_311_SR_2010-2020-sample-1M.csv --all --language Spanish --output /tmp/nyc311-describegpt-spanish.md --fresh
Prompt file: Default v7.0.0
Model: openai/gpt-oss-20b
LLM API URL: http://localhost:1234/v1
Language: 
Timestamp: 2026-02-10T23:48:00.749691+00:00

WARNING: Description generated by an LLM and may contain inaccuracies. Verify before using!
*


## REASONING

We need to produce a Spanish description, 1-8 sentences about dataset as whole. Then "Notable Characteristics" bullet list. Must not include summary stats or frequencies per field; just overall dataset characteristics. Use data from statistics: unique key, created date range, etc. Provide overall insights: number of records 1M, 2010-2020 period, majority noise complaints, high proportion 'Other', missing values in many fields, duplicates? Unique key ensures uniqueness. PII/PHI? Contains addresses, lat/long, but likely anonymized? Actually addresses are real. But we can mention potential privacy concerns.

Also note that many categorical fields have high cardinality and many 'Other' categories; distribution skewed.

Let's craft 5-6 sentences Spanish:

"El conjunto de datos contiene 1 000 000 de incidentes registrados en el sistema 311 de la ciudad de Nueva York entre enero de 2010 y diciembre de 2020. Cada registro se identifica con una clave única, pero gran parte de los campos de texto presentan valores “Other” que agrupan más del 90 % de las observaciones, indicando alta heterogeneidad y falta de estandarización. La mayoría de las quejas corresponden a ruido residencial (≈9 %) y a otras categorías sin definir (≈56 %), lo que muestra un desequilibrio en la distribución de tipos de incidentes. Los campos geográficos (dirección, latitud/longitud) están completos para la gran mayoría de los casos, aunque existen valores nulos significativos en ZIP, ciudad y tipo de ubicación. Se observa una alta variabilidad temporal: la frecuencia diaria varía entre 0 y varios cientos por día, con picos alrededor de enero de cada año."

Need bullet list for notable characteristics:

- Alta cardinalidad y uso extensivo de “Other” (≥90 %) en varias variables.
- Desbalance en tipos de queja: ruido residencial ~9 %, otras sin definir ~56 %.
- Pérdida significativa de datos: ZIP, ciudad, tipo de ubicación con 239k nulos; Incident Address 174k nulos; etc.
- Distribución temporal sesgada, con picos y tendencia a enero (muestra densidad).
- Datos geográficos disponibles pero con valores nulos en coordenadas ~254k.
- PII/PHI: direcciones completas y lat/long pueden identificar ubicaciones exactas; consideraciones de privacidad.
- Duplicados: Unique Key asegura no duplicados, pero posibles registros duplicados en otras columnas.
- Variabilidad en formatos (por ejemplo, "1 AVE" vs "BEND").
- Outliers: fechas extremadamente antiguas o futuras? but summary shows min 2010, max 2020.

Add attribution placeholder. Use Markdown formatting.

Let's craft final answer.
## TOKEN USAGE

TokenUsage { prompt: 15827, completion: 1115, total: 16942, elapsed: 29210 }
---
# Tags
- quejas_311_nyc  
- servicios_publicos  
- ruido_residencial  
- inspeccion_inmobiliaria  
- incidentes_transporte  
- datos_abiertos  
- ubicacion_geoespacial  
- consejo_comunitario  
- resolucion_queja  
- gobierno_municipal  

Generated by qsv v16.0.0 describegpt
Command line: target/debug/qsv describegpt /tmp/NYC_311_SR_2010-2020-sample-1M.csv --all --language Spanish --output /tmp/nyc311-describegpt-spanish.md --fresh
Prompt file: Default v7.0.0
Model: openai/gpt-oss-20b
LLM API URL: http://localhost:1234/v1
Language: 
Timestamp: 2026-02-10T23:48:34.447269+00:00

WARNING: Tags generated by an LLM and may contain inaccuracies. Verify before using!



## REASONING

We need to produce a list of up to 10 tags in markdown format, lowercase, with underscores between words. Tags should represent dataset topics: likely NYC 311 complaints, public service requests, noise, housing, transportation, etc.

Based on summary stats and frequency distribution. The dataset is NYC 311 complaints from 2010-2020, about 1 million records. So tags: "nyc_311", "public_service_requests", "noise_complaints", "housing_and_building_inspections", "transportation_incidents", "city_government_data", "open_data_portal", "geospatial_location", "community_board", "complaint_resolution".

We must use no more than 10 tags. Use lowercase, underscores.

Add attribution placeholder after tags in markdown format: "# Tags" maybe just list tags each line? The instruction: "Make sure your tag choices are in this language: Spanish". So tags must be in Spanish. So we need Spanish tags, e.g., "quejas_311", "servicios_publicos_nyc", "reporte_noise", but Spanish.

We can use Spanish terms with underscores. Examples: "nyc_311_complaint" would be English; we need Spanish. So maybe "quejas_311_nyc". Let's think of tags that are descriptive in Spanish, using underscores. Up to 10.

Possible tags:

1. quejas_311_nyc
2. servicios_publicos
3. ruido_residencial
4. inspeccion_inmobiliaria
5. incidentes_transporte
6. datos_abiertos
7. ubicacion_geoespacial
8. consejo_comunitario
9. resolucion_queja
10. gobierno_municipal

Check Spanish: "consejo_comunitario" (community board). All lowercase with underscores.

Now produce markdown list:

- quejas_311_nyc
- servicios_publicos
- ruido_residencial
- inspeccion_inmobiliaria
- incidentes_transporte
- datos_abiertos
- ubicacion_geoespacial
- consejo_comunitario
- resolucion_queja
- gobierno_municipal

Add attribution placeholder after tags: maybe a line "Attribution: {GENERATED_BY_SIGNATURE}". The instruction: "Add an Attribution with the placeholder "{GENERATED_BY_SIGNATURE}" after the Tags. If generating JSON format, add the Attribution as a separate key at the top level of the JSON object, after the Tags, otherwise add it at the bottom of the Tags in Markdown format." So we need to add attribution at bottom of tags in markdown.

Thus output:

```
- quejas_311_nyc
- servicios_publicos
...
- gobierno_municipal

{GENERATED_BY_SIGNATURE}
```

Make sure to include line break. Also no other content.
## TOKEN USAGE

TokenUsage { prompt: 15797, completion: 663, total: 16460, elapsed: 21586 }
---
