Generated using a Local LLM (openai/gpt-oss-20b) on LM Studio 0.4.2+2 running on a Macbook Pro M4 Max 64gb/Tahoe 26.3:

```bash
$ QSV_LLM_BASE_URL=http://localhost:1234/v1  \
   qsv describegpt --all /tmp/NYC_311_SR_2010-2020-sample-1M.csv \
   --language Mandarin \
   --output /tmp/nyc311-describegpt-mandarin.md
```
---
# Dictionary
| Name | Type | Label | Description | Min | Max | Cardinality | Enumeration | Null Count | Examples |
|------|------|-------|-------------|-----|-----|-------------|-------------|------------|----------|
| **Unique Key** | Integer | 唯一键 | 每条投诉记录的唯一整数标识符。根据统计，所有值均互不相同（1,000,000 条记录），因此可用作主键或索引。 | 11465364 | 48478173 | 1,000,000 |  | 0 | <ALL_UNIQUE> |
| **Created Date** | DateTime | 创建日期 | 投诉被提交到系统时的 UTC 时间戳，格式为 ISO‑8601（例如 2013‑01‑24T00:00:00+00:00）。时间范围从 2010 年 1 月 1 日至 2020 年 12 月 23 日。 | 2010-01-01T00:00:00+00:00 | 2020-12-23T01:25:51+00:00 | 841,014 |  | 0 | Other… [997,333]<br>01/24/2013 12:00:00 AM [347]<br>01/07/2014 12:00:00 AM [315]<br>01/08/2015 12:00:00 AM [283]<br>02/16/2015 12:00:00 AM [269] |
| **Closed Date** | DateTime | 关闭日期 | 投诉在系统中被标记为完成或已处理的 UTC 时间戳。许多记录为空（NULL），也有部分使用占位符 1900‑01‑01。时间范围从 1900 年至 2100 年。 | 1900-01-01T00:00:00+00:00 | 2100-01-01T00:00:00+00:00 | 688,837 |  | 28,619 | Other… [968,671]<br>(NULL) [28,619]<br>11/15/2010 12:00:00 AM [384]<br>11/07/2012 12:00:00 AM [329]<br>12/09/2010 12:00:00 AM [267] |
| **Agency** | String | 机构代码 | 负责处理该投诉的部门或机构的缩写（如 NYPD、HPD、DOT 等）。频率分布显示 NYPD 和 HPD 占比最高。 | 3-1-1 | TLC | 28 |  | 0 | NYPD [265,116]<br>HPD [258,033]<br>DOT [132,462]<br>DSNY [81,606]<br>DEP [75,895] |
| **Agency Name** | String | 机构名称 | 负责该投诉的完整机构全称，例如 New York City Police Department、Department of Housing Preservation and Development 等。 | 3-1-1 | Valuation Policy | 553 |  | 0 | New York City Police Depa… [265,038]<br>Department of Housing Pre… [258,019]<br>Department of Transportat… [132,462]<br>Other… [103,974]<br>Department of Environment… [75,895] |
| **Complaint Type** | String | 投诉类型 | 投诉的大类分类（如 Noise, HEAT/HOT WATER, Illegal Parking 等）。约 56% 的记录归为 “Other”，说明存在大量细分或未归类的投诉。 | ../../WEB-INF/web.xml;x= | ZTESTINT | 287 |  | 0 | Other… [563,561]<br>Noise - Residential [89,439]<br>HEAT/HOT WATER [56,639]<br>Illegal Parking [45,032]<br>Blocked Driveway [42,356] |
| **Descriptor** | String | 描述词 | 对投诉类型更具体的描述，例如 Loud Music/Party、HEAT、Pothole 等。该字段极其稀疏，约 67% 的记录为 “Other”。 | 1 Missed Collection | unknown odor/taste in drinking water (QA6) | 1,392 |  | 3,001 | Other… [671,870]<br>Loud Music/Party [93,646]<br>ENTIRE BUILDING [36,885]<br>HEAT [35,088]<br>No Access [31,631] |
| **Location Type** | String | 地点类型 | 事件发生地点的类别（例如 Residential Building、Street/Sidewalk、Park 等）。RESIDENTIAL BUILDING 是最常见的分类。 | 1-, 2- and 3- Family Home | Wooded Area | 162 |  | 239,131 | RESIDENTIAL BUILDING [255,562]<br>(NULL) [239,131]<br>Street/Sidewalk [145,653]<br>Residential Building/Hous… [92,765]<br>Street [92,190] |
| **Incident Zip** | String | 邮政编码 | 事件所在区域的五位数 ZIP 码，约 86% 的记录归为 “Other”，说明大部分地址未在前十名中出现。 | * | XXXXX | 535 |  | 54,978 | Other… [815,988]<br>(NULL) [54,978]<br>11226 [17,114]<br>10467 [14,495]<br>11207 [12,872] |
| **Incident Address** | String | 事件地址 | 完整的街道地址字符串（如 655 EAST 230 STREET）。此字段包含大量不重复值，约 99% 的记录为 “Other”。 | * * | west 155 street and edgecombe avenue | 341,996 |  | 174,700 | Other… [819,046]<br>(NULL) [174,700]<br>655 EAST  230 STREET [1,538]<br>78-15 PARSONS BOULEVARD [694]<br>672 EAST  231 STREET [663] |
| **Street Name** | String | 街道名称 | 主要街道名称，例如 BROADWAY、GRAND CONCOURSE 等。大部分值集中在前十名，但仍有 95% 为 “Other”。 | * | wyckoff avenue | 14,837 |  | 174,720 | Other… [784,684]<br>(NULL) [174,720]<br>BROADWAY [9,702]<br>GRAND CONCOURSE [5,851]<br>OCEAN AVENUE [3,946] |
| **Cross Street 1** | String | 交叉路口一 | 与主干道相交的第一条街道名称，常见于街角或路口事件。 | 1 AVE | mermaid | 16,238 |  | 320,401 | Other… [619,743]<br>(NULL) [320,401]<br>BEND [12,562]<br>BROADWAY [8,548]<br>3 AVENUE [6,154] |
| **Cross Street 2** | String | 交叉路口二 | 与主干道相交的第二条街道名称，适用于多路交汇点。 | 1 AVE | surf | 16,486 |  | 323,644 | Other… [623,363]<br>(NULL) [323,644]<br>BEND [12,390]<br>BROADWAY [8,833]<br>DEAD END [5,626] |
| **Intersection Street 1** | String | 交叉路一 | 事件地点对应的第一条交叉路街道，用于更精确描述道路交汇处。 | 1 AVE | flatlands AVE | 11,237 |  | 767,422 | (NULL) [767,422]<br>Other… [214,544]<br>BROADWAY [3,761]<br>CARPENTER AVENUE [2,918]<br>BEND [2,009] |
| **Intersection Street 2** | String | 交叉路二 | 事件地点对应的第二条交叉路街道，常与 Intersection Street 1 配合使用。 | 1 AVE | glenwood RD | 11,674 |  | 767,709 | (NULL) [767,709]<br>Other… [215,667]<br>BROADWAY [3,462]<br>BEND [1,942]<br>2 AVENUE [1,690] |
| **Address Type** | String | 地址类型 | 提供地址信息的方式，例如 ADDRESS、INTERSECTION、BLOCKFACE 等。ADDRESS 占比最高（81%）。 | ADDRESS | PLACENAME | 6 |  | 125,802 | ADDRESS [710,380]<br>INTERSECTION [133,361]<br>(NULL) [125,802]<br>BLOCKFACE [22,620]<br>LATLONG [7,421] |
| **City** | String | 城市/行政区 | 事件发生所在的行政区域，主要包括 Brooklyn、New York、Bronx 等。 | * | YORKTOWN HEIGHTS | 382 |  | 61,963 | BROOKLYN [296,254]<br>NEW YORK [189,069]<br>BRONX [181,168]<br>Other… [163,936]<br>(NULL) [61,963] |
| **Landmark** | String | 地标 | 附近的重要建筑或地点名称，例如 EAST 230 STREET、BROADWAY 等。大部分记录归为 “Other”。 | 1 AVENUE | ZULETTE AVENUE | 5,915 |  | 912,779 | (NULL) [912,779]<br>Other… [80,165]<br>EAST  230 STREET [1,545]<br>EAST  231 STREET [1,291]<br>BROADWAY [1,148] |
| **Facility Type** | String | 设施类型 | 与事件相关的公共设施类别，如 DSNY Garage、Precinct、School 等。 | DSNY Garage | School District | 6 |  | 145,478 | N/A [628,279]<br>Precinct [193,259]<br>(NULL) [145,478]<br>DSNY Garage [32,310]<br>School [617] |
| **Status** | String | 状态 | 投诉当前处理阶段（Closed、Pending、Open、In Progress 等）。Closed 占比最高（95%）。 | Assigned | Unspecified | 10 | Assigned<br>Closed<br>Closed - Testing<br>Email Sent<br>In Progress<br>Open<br>Pending<br>Started<br>Unassigned<br>Unspecified | 0 | Closed [952,522]<br>Pending [20,119]<br>Open [12,340]<br>In Progress [7,841]<br>Assigned [6,651] |
| **Due Date** | DateTime | 到期日期 | 预计完成或解决投诉的截止日期，格式同 Created Date。大多数记录为空。 | 1900-01-02T00:00:00+00:00 | 2021-06-17T16:34:13+00:00 | 345,077 |  | 647,794 | (NULL) [647,794]<br>Other… [350,746]<br>04/08/2015 10:00:58 AM [214]<br>05/02/2014 03:32:17 PM [183]<br>03/30/2018 10:10:39 AM [172] |
| **Resolution Description** | String | 解决说明 | 对投诉处理结果的文字描述，例如 “The Police Department responded …”。约 52% 的记录为 “Other”。 | A DOB violation was issued for failing to comply with an existing Stop Work Order. | Your request was submitted to the Department of Homeless Services. The City?s outreach team will assess the homeless individual and offer appropriate assistance within 2 hours. If you asked to know the outcome of your request, you will get a call within 2 hours. No further status will be available through the NYC 311 App, 311, or 311 Online. | 1,216 |  | 20,480 | Other… [511,739]<br>The Police Department res… [91,408]<br>The Department of Housing… [72,962]<br>The Police Department res… [63,868]<br>Service Request status fo… [52,155] |
| **Resolution Action Updated Date** | DateTime | 更新日期 | 最近一次更新解决动作的 UTC 时间戳，格式同 Created Date。许多记录为空（NULL）。 | 2009-12-31T01:35:00+00:00 | 2020-12-23T06:56:14+00:00 | 690,314 |  | 15,072 | Other… [982,148]<br>(NULL) [15,072]<br>11/15/2010 12:00:00 AM [385]<br>11/07/2012 12:00:00 AM [336]<br>12/09/2010 12:00:00 AM [273] |
| **Community Board** | String | 社区委员会 | 纽约市行政区划内的社区委员会编号，例如 12 MANHATTAN、01 BROOKLYN 等。 | 0 Unspecified | Unspecified STATEN ISLAND | 77 |  | 0 | Other… [751,635]<br>0 Unspecified [49,878]<br>12 MANHATTAN [29,845]<br>12 QUEENS [23,570]<br>01 BROOKLYN [21,714] |
| **BBL** | String | BBL 编号 | Borough‑Block‑Lot（县/区-街块-地段）唯一标识符，整数形式。大多数记录归为 “Other”。 | 0140694020 | 0140694020 | 268,383 |  | 243,046 | Other… [750,668]<br>(NULL) [243,046]<br>2048330028 [1,566]<br>4068290001 [696]<br>4015110001 [664] |
| **Borough** | String | 行政区 | 纽约市的四个主要行政区：Brooklyn、Queens、Manhattan、Bronx，以及 Unspecified/Statue Island。 | BRONX | Unspecified | 6 | BRONX<br>BROOKLYN<br>MANHATTAN<br>QUEENS<br>STATEN ISLAND<br>Unspecified | 0 | BROOKLYN [296,081]<br>QUEENS [228,818]<br>MANHATTAN [195,488]<br>BRONX [180,142]<br>Unspecified [49,878] |
| **X Coordinate (State Plane)** | Integer | 州平面坐标 X | 事件在州平面投影（NY State Plane）中的 X 坐标，单位为英尺。大部分值归为 “Other”。 | 913281 | 1067220 | 102,556 |  | 85,327 | Other… [908,535]<br>(NULL) [85,327]<br>1022911 [1,568]<br>1037000 [701]<br>1023174 [675] |
| **Y Coordinate (State Plane)** | Integer | 州平面坐标 Y | 事件在州平面投影（NY State Plane）中的 Y 坐标，单位为英尺。 | 121152 | 271876 | 116,092 |  | 85,327 | Other… [908,538]<br>(NULL) [85,327]<br>264242 [1,566]<br>202363 [706]<br>211606 [665] |
| **Open Data Channel Type** | String | 提交渠道类型 | 投诉通过何种方式提交到系统，例如 PHONE、ONLINE、MOBILE 等。PHONE 占比最高（约 50%）。 | MOBILE | UNKNOWN | 5 | MOBILE<br>ONLINE<br>OTHER<br>PHONE<br>UNKNOWN | 0 | PHONE [497,606]<br>UNKNOWN [230,402]<br>ONLINE [177,334]<br>MOBILE [79,892]<br>OTHER [14,766] |
| **Park Facility Name** | String | 公园设施名称 | 涉及的公园或游乐设施全名，常见如 Central Park、Riverside Park 等。大多数记录为 “Unspecified”。 | "Uncle" Vito F. Maranzano Glendale Playground | Zimmerman Playground | 1,889 |  | 0 | Unspecified [993,141]<br>Other… [5,964]<br>Central Park [261]<br>Riverside Park [136]<br>Prospect Park [129] |
| **Park Borough** | String | 公园所在行政区 | 与 Park Facility Name 对应的行政区（Brooklyn、Queens、Manhattan、Bronx）。 | BRONX | Unspecified | 6 | BRONX<br>BROOKLYN<br>MANHATTAN<br>QUEENS<br>STATEN ISLAND<br>Unspecified | 0 | BROOKLYN [296,081]<br>QUEENS [228,818]<br>MANHATTAN [195,488]<br>BRONX [180,142]<br>Unspecified [49,878] |
| **Vehicle Type** | String | 车辆类型 | 涉及车辆的类别，例如 Car Service、Ambulette / Paratransit 等。大多数记录为空（NULL）。 | Ambulette / Paratransit | Green Taxi | 5 |  | 999,652 | (NULL) [999,652]<br>Car Service [317]<br>Ambulette / Paratransit [19]<br>Commuter Van [11]<br>Green Taxi [1] |
| **Taxi Company Borough** | String | 出租车公司所在行政区 | 出租车或共享服务所属的行政区，包含 Brooklyn、Queens、Manhattan 等。 | BRONX | Staten Island | 11 |  | 999,156 | (NULL) [999,156]<br>BROOKLYN [207]<br>QUEENS [194]<br>MANHATTAN [171]<br>BRONX [127] |
| **Taxi Pick Up Location** | String | 出租车接客地点 | 乘客被接走的具体位置，例如 “Other”、“JFK Airport” 等。 | 1 5 AVENUE MANHATTAN | YORK AVENUE AND EAST 70 STREET | 1,903 |  | 992,129 | (NULL) [992,129]<br>Other… [4,091]<br>Other [2,006]<br>JFK Airport [562]<br>Intersection [486] |
| **Bridge Highway Name** | String | 桥梁/高速公路名称 | 涉及的桥梁或高速公路名称，如 Belt Pkwy、BQE/Gowanus Expwy 等。 | 145th St. Br - Lenox Ave | Willis Ave Br - 125th St/1st Ave | 68 |  | 997,711 | (NULL) [997,711]<br>Other… [779]<br>Belt Pkwy [276]<br>BQE/Gowanus Expwy [254]<br>Grand Central Pkwy [186] |
| **Bridge Highway Direction** | String | 桥梁/高速公路方向 | 行驶方向描述，例如 East/Long Island Bound、North/Bronx Bound 等。 | Bronx Bound | Westbound/To Goethals Br | 50 |  | 997,691 | (NULL) [997,691]<br>Other… [987]<br>East/Long Island Bound [210]<br>North/Bronx Bound [208]<br>East/Queens Bound [197] |
| **Road Ramp** | String | 道路出口类型 | 与事件相关的道路出口或入口类别，常见为 Roadway 或 Ramp。 | N/A | Roadway | 4 |  | 997,693 | (NULL) [997,693]<br>Roadway [1,731]<br>Ramp [555]<br>N/A [21] |
| **Bridge Highway Segment** | String | 桥梁/高速公路段 | 更细化的桥梁/高速公路段信息，例如 “Ramp” 或具体出口编号。 | 1-1-1265963747 | Wythe Ave/Kent Ave (Exit 31) | 937 |  | 997,556 | (NULL) [997,556]<br>Other… [2,144]<br>Ramp [92]<br>Roadway [54]<br>Clove Rd/Richmond Rd (Exi… [23] |
| **Latitude** | Float | 纬度 | 事件在地理坐标系统（WGS84）中的纬度，单位为度。 | 40.1123853 | 40.9128688 | 353,694 |  | 254,695 | Other… [739,329]<br>(NULL) [254,695]<br>40.89187241649303 [1,538]<br>40.1123853 [1,153]<br>40.89238451539139 [663] |
| **Longitude** | Float | 经度 | 事件在地理坐标系统（WGS84）中的经度，单位为度。 | -77.5195844 | -73.7005968 | 353,996 |  | 254,695 | Other… [739,329]<br>(NULL) [254,695]<br>-73.86016845296459 [1,538]<br>-77.5195844 [1,153]<br>-73.8592161325675 [663] |
| **Location** | String | 位置坐标 | 以字符串形式给出的纬度/经度对，例如 "(40.89187241649303, -73.86016845296459)"。 | (40.1123853, -77.5195844) | (40.9128688, -73.9024731) | 375,772 |  | 254,695 | Other… [739,329]<br>(NULL) [254,695]<br>(40.89187241649303, -73.8… [1,538]<br>(40.1123853, -77.5195844) [1,153]<br>(40.89238451539139, -73.8… [663] |

*Attribution: Generated by qsv v16.0.0 describegpt
Command line: target/debug/qsv describegpt /tmp/NYC_311_SR_2010-2020-sample-1M.csv --all --language Mandarin --output /tmp/nyc311-describegpt-mandarin.md --fresh
Prompt file: Default v7.0.0
Model: openai/gpt-oss-20b
LLM API URL: http://localhost:1234/v1
Language: 
Timestamp: 2026-02-10T22:03:22.583225+00:00

WARNING: Label and Description generated by an LLM and may contain inaccuracies. Verify before using!
*

## REASONING

We need Mandarin labels and descriptions for all 41 fields. Use summary stats & freq to understand context. Provide JSON mapping field names to label/description.

Need human-friendly label in Chinese, description in Chinese. Should explain what the field represents. Use domain knowledge: NYC 311 complaints dataset? Fields include complaint type etc. Use frequency distribution to mention common values maybe.

We need for each of 41 fields. Must be all included.

We'll produce JSON object with keys as provided field names (like "Unique Key", "Created Date", etc.). Provide Chinese label, e.g., "唯一键" or "记录唯一标识". Description: details like it's a unique identifier for complaint record; type integer; high cardinality; etc. Use summary stats to mention that all values unique.

For others: Created Date - timestamp when complaint created. Provide explanation, maybe timezone UTC? In dataset appears datetime strings with +00:00 offset. Note min and max dates. Provide typical range 2010-2020. Frequency distribution shows most frequent dates are early 2013 etc.

Closed Date - similar but may be null; mention date when complaint closed or resolved; many nulls; includes special placeholder 1900-01-01 etc. Use stats.

Agency - agency code: e.g., NYPD, HPD, DOT etc. Frequency distribution shows top agencies. Provide description.

Agency Name - full name of agency.

Complaint Type - type of complaint category (Noise, HEAT/HOT WATER etc.). Provide explanation and note many "Other".

Descriptor - more specific descriptor within complaint type; high cardinality.

Location Type - classification of location: residential building, street/sidewalk, etc. Frequency distribution shows categories.

Incident Zip - zip code where incident occurred.

Incident Address - full address string.

Street Name - main street.

Cross Street 1 and 2 - intersecting streets.

Intersection Street 1 & 2 - maybe alternative intersection representation.

Address Type - type of address: ADDRESS, INTERSECTION, etc.

City - city borough area? Actually borough names like Brooklyn, New York, Bronx. Provide description.

Landmark - notable landmark near incident.

Facility Type - e.g., DSNY Garage, Precinct, etc.

Status - status of complaint (Closed, Pending, Open, In Progress etc.).

Due Date - due date for resolution; many nulls.

Resolution Description - textual description of resolution.

Resolution Action Updated Date - timestamp when resolution action updated.

Community Board - community board number.

BBL - borough block lot identifier.

Borough - borough name.

X Coordinate (State Plane) / Y Coordinate (State Plane) - coordinates in state plane projection.

Open Data Channel Type - channel type used to submit complaint: PHONE, ONLINE, MOBILE etc.

Park Facility Name - park facility involved.

Park Borough - borough of the park.

Vehicle Type - vehicle type for taxi complaints maybe; but includes Car Service etc. Frequency distribution indicates Car Service majority but many null.

Taxi Company Borough - borough of taxi company.

Taxi Pick Up Location - pick up location.

Bridge Highway Name, Bridge Highway Direction, Road Ramp, Bridge Highway Segment - details for highway incidents.

Latitude, Longitude - geographic coordinates in degrees.

Location - string representation of lat/long pair.

We should use summary stats like cardinality and uniqueness ratio. For Unique Key: 1M records all unique. Provide description accordingly.

Also mention that some fields contain null values; frequency distribution shows "Other" categories etc.

Now produce JSON object with 41 entries.

Let's craft Chinese labels and descriptions.

We'll need to ensure Chinese is proper, maybe use simplified Chinese. Provide label like "唯一键". Description: "记录的唯一标识符，整数类型，每条记录唯一。"

Ok let's draft each.

We should be careful to include field names exactly as provided.

Return JSON. Ensure it's valid JSON with double quotes. Let's produce.


## TOKEN USAGE

TokenUsage { prompt: 13667, completion: 2905, total: 16572, elapsed: 44629 }
---
# Description
**描述**

该数据集包含一百万条纽约市311投诉记录，涵盖2010年1月1日至2020年12月23日的创建时间。每条记录都由唯一整数键标识，并记录了投诉机构、类型、地点、状态等信息。大多数字段采用文本或日期格式，其中“其他”类别占比较高，表明存在大量未归类或稀有值。坐标字段（纬度/经度）显示出明显的空间聚集，主要集中在曼哈顿和布鲁克林地区。整体上，该数据集既反映了市政服务需求，也揭示了记录过程中的不一致性与缺失情况。

**显著特征**

- **中心趋势与分散度**：创建日期中位数为2015‑11‑10，平均值略偏晚（约2015‑11‑18），显示投诉量在近年呈上升趋势。  
- **分布形态**：多数分类字段高度倾斜，超过一半的记录归入“Other”，如投诉类型、描述词和地址等。  
- **异常值与占位符**：日期字段出现1900‑01‑01和2100‑01‑01作为占位符；坐标字段有极端数值（如纬度<40.1或>41.0），可能为错误录入。  
- **缺失率**：某些关键字段缺失比例高达90%（如Incident Zip、Incident Address、Latitude/Longitude的“Other”占比超过99%）。  
- **重复性**：`Unique Key` 确保每条记录唯一，未发现实际重复。  
- **PII/PHI 关注点**：地址和精确坐标可能揭示个人住址或工作地点，应在共享时考虑匿名化或聚合处理。  
- **数据质量问题**：大量“Other”类别说明分类体系不完整；日期格式混杂（ISO 与自定义字符串）需标准化；数值字段存在异常范围与缺失。

Generated by qsv v16.0.0 describegpt
Command line: target/debug/qsv describegpt /tmp/NYC_311_SR_2010-2020-sample-1M.csv --all --language Mandarin --output /tmp/nyc311-describegpt-mandarin.md --fresh
Prompt file: Default v7.0.0
Model: openai/gpt-oss-20b
LLM API URL: http://localhost:1234/v1
Language: 
Timestamp: 2026-02-10T22:03:47.501709+00:00

WARNING: Description generated by an LLM and may contain inaccuracies. Verify before using!



## REASONING

We need to produce a description (1-8 sentences) about dataset as whole, in Chinese. Then Notable Characteristics bullet list. Must not include any field-level stats or frequencies. Just overall. Use data from summary and frequency but no per-field details.

Dataset: 1,000,000 records of NYC 311 complaints (likely). Data includes dates, agencies, types, addresses etc. We need to mention high cardinality fields (unique key), date range, many missing values, heavy skewness in complaint type distribution, majority 'Other', etc. Also note anomalies like '1900-01-01' and '2100-01-01' placeholders. Outliers: coordinate extremes. duplicates: unique key ensures no duplicate.

Also mention potential PII: address, coordinates, etc.

Add attribution placeholder.

Let's craft 5 sentences description. Then bullet list of characteristics: e.g., central tendency: median created date 2015-11-10; mean ~2015-11-18; skewness slight negative -0.086 for dates indicates left skew? For other numeric fields maybe not needed. Distribution shape: highly skewed, many categories with 'Other'. Outliers: '1900' and '2100' in date, coordinates extremes. Missing values high in many fields e.g., Incident Zip 86% Other; etc.

Also mention duplicates: unique key ensures uniqueness. Potential PII: addresses, names of agencies? Not personal info. But location data could be sensitive.

Now produce markdown.


## TOKEN USAGE

TokenUsage { prompt: 15504, completion: 775, total: 16279, elapsed: 24916 }
---
# Tags
- nyc_311  
- 城市投诉  
- 公共安全  
- 交通基础设施  
- 噪音投诉  
- 住房建筑  
- 开放数据  
- 市政服务  
- 城市规划  

Generated by qsv v16.0.0 describegpt
Command line: target/debug/qsv describegpt /tmp/NYC_311_SR_2010-2020-sample-1M.csv --all --language Mandarin --output /tmp/nyc311-describegpt-mandarin.md --fresh
Prompt file: Default v7.0.0
Model: openai/gpt-oss-20b
LLM API URL: http://localhost:1234/v1
Language: 
Timestamp: 2026-02-10T22:04:05.268757+00:00

WARNING: Tags generated by an LLM and may contain inaccuracies. Verify before using!



## REASONING

We need to produce tags in Mandarin, lower case, underscores separating words. No field names. Up to 10 unique tags, descending importance based on summary stats and freq distribution.

Dataset: NYC 311 complaints dataset (likely). Tags could be: "nyc_311", "complaint_management", "public_safety", "city_services", "transportation_infrastructure", "noise_complaints", "housing_and_buildings", "data_governance", "open_data", "urban_planning". Must be Mandarin, lower case, underscore. So in Chinese: e.g., "纽约市_311" but need to use characters? They said Mandarin, so we can use Chinese characters. But must be lowercase only and underscores separate words. Chinese characters don't have uppercase/lowercase. So fine.

We list tags in descending importance based on stats: most frequent categories: Agency (NYPD, HPD), Complaint Type (Noise residential etc.), Location type residential building, City boroughs. Also data volume high. Tags: "nyc_311", "城市投诉", "公共安全", "交通基础设施", "噪音投诉", "住房建筑", "开放数据", "市政服务", "城市规划". That's 9 tags.

We must add attribution placeholder "{GENERATED_BY_SIGNATURE}". In Markdown format after the tags. So produce:

- nyc_311
- 城市投诉
...

At bottom add attribution. Ensure no field names used.

Let's craft final answer.
## TOKEN USAGE

TokenUsage { prompt: 15474, completion: 371, total: 15845, elapsed: 17765 }
---
