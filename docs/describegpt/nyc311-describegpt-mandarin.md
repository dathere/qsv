Generated using a Local LLM (openai/gpt-oss-20b) on LM Studio 0.3.34 Build 1 running on a Macbook Pro M4 Max 64gb/Tahoe 26.2:

```bash
$ QSV_LLM_BASE_URL=https://api.together.xyz/v1 QSV_LLM_APIKEY=<KEY> \
   qsv describegpt --all NYC_311_SR_2010-2020-sample-1M.csv \
   --language Mandarin \
   --model openai/gpt-oss-120b \
   --output /tmp/nyc311-describegpt-mandarin.md
```
---
# Dictionary
| Name | Type | Label | Description | Min | Max | Cardinality | Enumeration | Null Count | Examples |
|------|------|-------|-------------|-----|-----|-------------|-------------|------------|----------|
| **Unique Key** | Integer | 唯一键 | 每条记录的全局唯一标识符，系统自动生成的整数。它在整个数据集中没有重复，用于唯一定位和关联其他表格。 | 11465364 | 48478173 | 1,000,000 |  | 0 | <ALL_UNIQUE> |
| **Created Date** | DateTime | 创建日期 | 投诉或服务请求在系统中首次被录入的时间戳，精确到秒。大多数记录的创建日期集中在 2010‑2020 年之间，表示数据的收集周期。 | 2010-01-01T00:00:00+00:00 | 2020-12-23T01:25:51+00:00 | 841,014 |  | 0 | Other (841,004) [997,333]<br>01/24/2013 12:00:00 AM [347]<br>01/07/2014 12:00:00 AM [315]<br>01/08/2015 12:00:00 AM [283]<br>02/16/2015 12:00:00 AM [269] |
| **Closed Date** | DateTime | 关闭日期 | 该投诉或服务请求被标记为完成或关闭的时间戳。若为 NULL 则表示仍在处理中或尚未关闭。 | 1900-01-01T00:00:00+00:00 | 2100-01-01T00:00:00+00:00 | 688,837 |  | 28,619 | Other (688,827) [968,897]<br>(NULL) [28,619]<br>11/15/2010 12:00:00 AM [384]<br>11/07/2012 12:00:00 AM [329]<br>12/09/2010 12:00:00 AM [267] |
| **Agency** | String | 负责机构代码 | 处理该请求的纽约市政府部门缩写，如 NYPD（警察局）、HPD（住房与发展部）等。频率最高的前五个机构占约 70% 的记录。 | 3-1-1 | TLC | 28 |  | 0 | NYPD [265,116]<br>HPD [258,033]<br>DOT [132,462]<br>DSNY [81,606]<br>DEP [75,895] |
| **Agency Name** | String | 负责机构全称 | 对应 "Agency" 字段的完整机构名称，例如 "New York City Police Department"。常见机构包括警察局、住房与发展部、交通局等。 | 3-1-1 | Valuation Policy | 553 |  | 0 | New York City Police Depa… [265,038]<br>Department of Housing Pre… [258,019]<br>Department of Transportat… [132,462]<br>Other (543) [103,974]<br>Department of Environment… [75,895] |
| **Complaint Type** | String | 投诉类别 | 市民提交的投诉主题，如 "Noise - Residential"、"HEAT/HOT WATER"、"Illegal Parking" 等。前十类覆盖约 45% 的记录，其余归入 "Other" 类别。 | ../../WEB-INF/web.xml;x= | ZTESTINT | 287 |  | 0 | Other (277) [563,561]<br>Noise - Residential [89,439]<br>HEAT/HOT WATER [56,639]<br>Illegal Parking [45,032]<br>Blocked Driveway [42,356] |
| **Descriptor** | String | 详细描述 | 对投诉类型的进一步说明，例如 "Loud Music/Party"、"ENTIRE BUILDING"、"HEAT" 等。最常见的描述占约 9.4%。其余大多数记录归为 "Other"。 | 1 Missed Collection | unknown odor/taste in drinking water (QA6) | 1,392 |  | 3,001 | Other (1,382) [674,871]<br>Loud Music/Party [93,646]<br>ENTIRE BUILDING [36,885]<br>HEAT [35,088]<br>No Access [31,631] |
| **Location Type** | String | 地点类型 | 投诉发生地点的类别，如 "RESIDENTIAL BUILDING"、"Street/Sidewalk"、"Store/Commercial" 等。约 25.6% 为住宅建筑，约 24% 为缺失值 (NULL)。 | 1-, 2- and 3- Family Home | Wooded Area | 162 |  | 239,131 | RESIDENTIAL BUILDING [255,562]<br>(NULL) [239,131]<br>Street/Sidewalk [145,653]<br>Residential Building/Hous… [92,765]<br>Street [92,190] |
| **Incident Zip** | String | 邮政编码 | 投诉现场的五位数邮政编码。最常出现的前十个邮编每个约占 1% 左右，超过 82% 的记录归入 "Other"（未列出的邮编）。 | * | XXXXX | 535 |  | 54,978 | Other (525) [827,654]<br>(NULL) [54,978]<br>11226 [17,114]<br>10467 [14,495]<br>11207 [12,872] |
| **Incident Address** | String | 详细地址 | 投诉发生的具体街道地址。如 "655 EAST  230 STREET"。约 17.5% 的记录为空，最常出现的前十个地址仅占 0.15% 以下，绝大多数归为 "Other"。 | * * | west 155 street and edgecombe avenue | 341,996 |  | 174,700 | Other (341,986) [819,378]<br>(NULL) [174,700]<br>655 EAST  230 STREET [1,538]<br>78-15 PARSONS BOULEVARD [694]<br>672 EAST  231 STREET [663] |
| **Street Name** | String | 街道名称 | 地址所在的街道名称，如 "BROADWAY"、"GRAND CONCOURSE" 等。最常出现的十个街道合计约 18% 的记录，其余为其他街道或缺失。 | * | wyckoff avenue | 14,837 |  | 174,720 | Other (14,827) [787,222]<br>(NULL) [174,720]<br>BROADWAY [9,702]<br>GRAND CONCOURSE [5,851]<br>OCEAN AVENUE [3,946] |
| **Cross Street 1** | String | 交叉街道 1 | 地址所在的第一条交叉街道（若有），常见值如 "BEND"、"BROADWAY" 等。约 32% 的记录为缺失 (NULL)。 | 1 AVE | mermaid | 16,238 |  | 320,401 | Other (16,228) [623,317]<br>(NULL) [320,401]<br>BEND [12,562]<br>BROADWAY [8,548]<br>3 AVENUE [6,154] |
| **Cross Street 2** | String | 交叉街道 2 | 地址所在的第二条交叉街道（若有），常见值同上。约 32% 的记录为缺失 (NULL)。 | 1 AVE | surf | 16,486 |  | 323,644 | Other (16,476) [626,168]<br>(NULL) [323,644]<br>BEND [12,390]<br>BROADWAY [8,833]<br>DEAD END [5,626] |
| **Intersection Street 1** | String | 交叉口街道 1 | 若地址在交叉口，记录的第一条街道名称。常见值包括 "BROADWAY"、"CARPENTER AVENUE" 等。约 76% 的记录为缺失。 | 1 AVE | flatlands AVE | 11,237 |  | 767,422 | (NULL) [767,422]<br>Other (11,227) [215,482]<br>BROADWAY [3,761]<br>CARPENTER AVENUE [2,918]<br>BEND [2,009] |
| **Intersection Street 2** | String | 交叉口街道 2 | 若地址在交叉口，记录的第二条街道名称。常见值同上。约 77% 的记录为缺失。 | 1 AVE | glenwood RD | 11,674 |  | 767,709 | (NULL) [767,709]<br>Other (11,664) [216,748]<br>BROADWAY [3,462]<br>BEND [1,942]<br>2 AVENUE [1,690] |
| **Address Type** | String | 地址类型 | 记录的地址表现形式，如 "ADDRESS"（完整地址）、"INTERSECTION"（交叉口）、"BLOCKFACE"（街段）等。约 71% 为完整地址。 | ADDRESS | PLACENAME | 6 | (NULL)<br>ADDRESS<br>BLOCKFACE<br>INTERSECTION<br>LATLONG<br>PLACENAME | 125,802 | ADDRESS [710,380]<br>INTERSECTION [133,361]<br>(NULL) [125,802]<br>BLOCKFACE [22,620]<br>LATLONG [7,421] |
| **City** | String | 城市（区） | 投诉所在的纽约市区，如 "BROOKLYN"、"NEW YORK"（曼哈顿）等。约 30% 为布鲁克林，约 19% 为布朗克斯，其余区分布不均。 | * | YORKTOWN HEIGHTS | 382 |  | 61,963 | BROOKLYN [296,254]<br>NEW YORK [189,069]<br>BRONX [181,168]<br>Other (372) [171,028]<br>(NULL) [61,963] |
| **Landmark** | String | 地标 | 投诉地点附近的著名地标或建筑。大多数记录为空（约 91%），出现的地标如 "EAST  230 STREET"、"BROADWAY" 等。 | 1 AVENUE | ZULETTE AVENUE | 5,915 |  | 912,779 | (NULL) [912,779]<br>Other (5,905) [80,508]<br>EAST  230 STREET [1,545]<br>EAST  231 STREET [1,291]<br>BROADWAY [1,148] |
| **Facility Type** | String | 设施类型 | 投诉所在设施的分类，例如 "N/A"、"Precinct"（警区）或 "DSNY Garage"（市清洁局车库）。约 63% 为 "N/A"，约 19% 为警区。 | DSNY Garage | School District | 6 | (NULL)<br>DSNY Garage<br>N/A<br>Precinct<br>School<br>School District | 145,478 | N/A [628,279]<br>Precinct [193,259]<br>(NULL) [145,478]<br>DSNY Garage [32,310]<br>School [617] |
| **Status** | String | 当前状态 | 投诉的处理进度状态，如 "Closed"（已关闭）、"Pending"（待处理）、"Open"（打开）等。约 95% 已关闭，少数为未完成状态。 | Assigned | Unspecified | 10 | Assigned<br>Closed<br>Closed - Testing<br>Email Sent<br>In Progress<br>Open<br>Pending<br>Started<br>Unassigned<br>Unspecified | 0 | Closed [952,522]<br>Pending [20,119]<br>Open [12,340]<br>In Progress [7,841]<br>Assigned [6,651] |
| **Due Date** | DateTime | 截止日期 | 系统为该请求规定的完成截止时间。多数记录为空（约 65%），其余为具体时间点，分布在 2014‑2020 年之间。 | 1900-01-02T00:00:00+00:00 | 2021-06-17T16:34:13+00:00 | 345,077 |  | 647,794 | (NULL) [647,794]<br>Other (345,067) [350,849]<br>04/08/2015 10:00:58 AM [214]<br>05/02/2014 03:32:17 PM [183]<br>03/30/2018 10:10:39 AM [172] |
| **Resolution Description** | String | 处理结果描述 | 对请求的最终处理措施进行文字说明，如警察部门的响应、检查结果、是否发出违规通知等。最常见的前五条描述占约 35%，其余归类为 "Other"。 | A DOB violation was issued for failing to comply with an existing Stop Work Order. | Your request was submitted to the Department of Homeless Services. The City?s outreach team will assess the homeless individual and offer appropriate assistance within 2 hours. If you asked to know the outcome of your request, you will get a call within 2 hours. No further status will be available through the NYC 311 App, 311, or 311 Online. | 1,216 |  | 20,480 | Other (1,206) [532,002]<br>The Police Department res… [91,408]<br>The Department of Housing… [72,962]<br>The Police Department res… [63,868]<br>Service Request status fo… [52,155] |
| **Resolution Action Updated Date** | DateTime | 处理行动更新时间 | 记录最新一次处理行动的时间戳。约 98% 的记录为 "Other"（缺失），其余为具体日期。 | 2009-12-31T01:35:00+00:00 | 2020-12-23T06:56:14+00:00 | 690,314 |  | 15,072 | Other (690,304) [982,378]<br>(NULL) [15,072]<br>11/15/2010 12:00:00 AM [385]<br>11/07/2012 12:00:00 AM [336]<br>12/09/2010 12:00:00 AM [273] |
| **Community Board** | String | 社区委员会 | 负责辖区的社区委员会编号与区县，如 "12 MANHATTAN"、"01 BROOKLYN" 等。约 75% 的记录归入 "Other"（未列出），其余分布在多个具体委员会。 | 0 Unspecified | Unspecified STATEN ISLAND | 77 |  | 0 | Other (67) [751,635]<br>0 Unspecified [49,878]<br>12 MANHATTAN [29,845]<br>12 QUEENS [23,570]<br>01 BROOKLYN [21,714] |
| **BBL** | String | 楼宇标识码 (BBL) | 纽约市房地产的唯一标识符，由区号、街道号、地块号组成。约 24% 为缺失，最常见的前十个 BBL 只占极小比例，其余为其他唯一值。 | 0000000000 | 5270000501 | 268,383 |  | 243,046 | Other (268,373) [751,031]<br>(NULL) [243,046]<br>2048330028 [1,566]<br>4068290001 [696]<br>4015110001 [664] |
| **Borough** | String | 行政区（区） | 投诉所在的纽约五大行政区之一：BROOKLYN、QUEENS、MANHATTAN、BRONX、STATEN ISLAND。约 30% 为布鲁克林，约 23% 为皇后区。 | BRONX | Unspecified | 6 | BRONX<br>BROOKLYN<br>MANHATTAN<br>QUEENS<br>STATEN ISLAND<br>Unspecified | 0 | BROOKLYN [296,081]<br>QUEENS [228,818]<br>MANHATTAN [195,488]<br>BRONX [180,142]<br>Unspecified [49,878] |
| **X Coordinate (State Plane)** | Integer | X 坐标（州平面坐标） | 基于纽约州平面坐标系的水平坐标，用于空间定位。约 8.5% 为缺失，常见值集中在 960,000‑1,050,000 范围。 | 913281 | 1067220 | 102,556 |  | 85,327 | Other (102,546) [908,877]<br>(NULL) [85,327]<br>1022911 [1,568]<br>1037000 [701]<br>1023174 [675] |
| **Y Coordinate (State Plane)** | Integer | Y 坐标（州平面坐标） | 基于纽约州平面坐标系的垂直坐标，用于空间定位。约 8.5% 为缺失，常见值集中在 120,000‑270,000 范围。 | 121152 | 271876 | 116,092 |  | 85,327 | Other (116,082) [908,868]<br>(NULL) [85,327]<br>264242 [1,566]<br>202363 [706]<br>211606 [665] |
| **Open Data Channel Type** | String | 开放数据渠道类型 | 市民提交请求的渠道，如 "PHONE"、"ONLINE"、"MOBILE"、"UNKNOWN" 等。电话渠道占约 50%，在线渠道约 18%。 | MOBILE | UNKNOWN | 5 | MOBILE<br>ONLINE<br>OTHER<br>PHONE<br>UNKNOWN | 0 | PHONE [497,606]<br>UNKNOWN [230,402]<br>ONLINE [177,334]<br>MOBILE [79,892]<br>OTHER [14,766] |
| **Park Facility Name** | String | 公园设施名称 | 若投诉涉及公园设施，记录的具体名称，例如 "Central Park"、"Prospect Park" 等。约 99% 为 "Unspecified"（未指定），少量为具体公园。 | "Uncle" Vito F. Maranzano Glendale Playground | Zimmerman Playground | 1,889 |  | 0 | Unspecified [993,141]<br>Other (1,879) [5,964]<br>Central Park [261]<br>Riverside Park [136]<br>Prospect Park [129] |
| **Park Borough** | String | 公园所在区 | 公园设施对应的行政区，同 "Borough" 字段的取值。大多数记录为 "Unspecified"，其余分布在五大区。 | BRONX | Unspecified | 6 | BRONX<br>BROOKLYN<br>MANHATTAN<br>QUEENS<br>STATEN ISLAND<br>Unspecified | 0 | BROOKLYN [296,081]<br>QUEENS [228,818]<br>MANHATTAN [195,488]<br>BRONX [180,142]<br>Unspecified [49,878] |
| **Vehicle Type** | String | 车辆类型 | 涉及的车辆分类，如 "Car Service"、"Ambulette / Paratransit"、"Green Taxi" 等。约 99.97% 为缺失，表示大多数记录与车辆无关。 | Ambulette / Paratransit | Green Taxi | 5 | (NULL)<br>Ambulette / Paratransit<br>Car Service<br>Commuter Van<br>Green Taxi | 999,652 | (NULL) [999,652]<br>Car Service [317]<br>Ambulette / Paratransit [19]<br>Commuter Van [11]<br>Green Taxi [1] |
| **Taxi Company Borough** | String | 出租车公司所在区 | 登记的出租车公司所属区县。约 99.92% 为缺失，极少数记录标明区县（如 BROOKLYN、QUEENS）。 | BRONX | Staten Island | 11 |  | 999,156 | (NULL) [999,156]<br>BROOKLYN [207]<br>QUEENS [194]<br>MANHATTAN [171]<br>BRONX [127] |
| **Taxi Pick Up Location** | String | 出租车上车地点 | 乘客搭乘出租车的地点描述，常见为 "Other"、机场名称或具体街道交叉口。约 99.21% 为缺失或 "Other"。 | 1 5 AVENUE MANHATTAN | YORK AVENUE AND EAST 70 STREET | 1,903 |  | 992,129 | (NULL) [992,129]<br>Other [4,091]<br>Other (1,893) [2,021]<br>JFK Airport [562]<br>Intersection [486] |
| **Bridge Highway Name** | String | 桥梁/高速公路名称 | 涉及的桥梁或高速公路名称，如 "Belt Pkwy"、"BQE/Gowanus Expwy" 等。约 99.77% 为缺失，少数记录具体名称。 | 145th St. Br - Lenox Ave | Willis Ave Br - 125th St/1st Ave | 68 |  | 997,711 | (NULL) [997,711]<br>Other (58) [851]<br>Belt Pkwy [276]<br>BQE/Gowanus Expwy [254]<br>Grand Central Pkwy [186] |
| **Bridge Highway Direction** | String | 桥梁/高速公路方向 | 桥梁或高速公路的行驶方向，如 "East/Long Island Bound"、"North/Bronx Bound" 等。约 99.77% 为缺失。 | Bronx Bound | Westbound/To Goethals Br | 50 |  | 997,691 | (NULL) [997,691]<br>Other (40) [1,064]<br>East/Long Island Bound [210]<br>North/Bronx Bound [208]<br>East/Queens Bound [197] |
| **Road Ramp** | String | 道路坡道类型 | 记录是否为道路坡道或相关设施，如 "Roadway"、"Ramp" 等。约 99.77% 为缺失。 | N/A | Roadway | 4 | (NULL)<br>N/A<br>Ramp<br>Roadway | 997,693 | (NULL) [997,693]<br>Roadway [1,731]<br>Ramp [555]<br>N/A [21] |
| **Bridge Highway Segment** | String | 桥梁/高速路段 | 具体的桥梁或高速路段名称或描述，如 "Ramp"、"Roadway"、特定出口描述等。约 99.76% 为缺失。 | 1-1-1265963747 | Wythe Ave/Kent Ave (Exit 31) | 937 |  | 997,556 | (NULL) [997,556]<br>Other (927) [2,159]<br>Ramp [92]<br>Roadway [54]<br>Clove Rd/Richmond Rd (Exi… [23] |
| **Latitude** | Float | 纬度 | 地点的纬度坐标（WGS84），值范围约 40.11‑40.91。约 25.5% 为缺失，主要用于空间可视化。 | 40.1123853 | 40.9128688 | 353,694 |  | 254,695 | Other (353,684) [739,574]<br>(NULL) [254,695]<br>40.89187241649303 [1,538]<br>40.1123853 [1,153]<br>40.89238451539139 [663] |
| **Longitude** | Float | 经度 | 地点的经度坐标（WGS84），值范围约 -77.52 到 -73.70。约 25.5% 为缺失。 | -77.5195844 | -73.7005968 | 353,996 |  | 254,695 | Other (353,986) [739,574]<br>(NULL) [254,695]<br>-73.86016845296459 [1,538]<br>-77.5195844 [1,153]<br>-73.8592161325675 [663] |
| **Location** | String | 位置坐标 | 以 "(纬度, 经度)" 形式呈现的完整地理坐标字符串。约 25.5% 为缺失，常用于地图展示。 | (40.1123853, -77.5195844) | (40.9128688, -73.9024731) | 375,772 |  | 254,695 | Other (375,762) [739,574]<br>(NULL) [254,695]<br>(40.89187241649303, -73.8… [1,538]<br>(40.1123853, -77.5195844) [1,153]<br>(40.89238451539139, -73.8… [663] |

*Attribution: Generated by qsv v11.0.2 describegpt
Command line: target/debug/qsv describegpt --all NYC_311_SR_2010-2020-sample-1M.csv --language Mandarin --model openai/gpt-oss-120b --output nyc311-describegpt-mandarin.md
Prompt file: Default v6.3.0
Model: openai/gpt-oss-120b
LLM API URL: https://api.together.xyz/v1
Timestamp: 2025-12-14T04:37:08.922417+00:00

WARNING: Label and Description generated by an LLM and may contain inaccuracies. Verify before using!*

## REASONING

We must generate Mandarin labels and descriptions for each field. Use statistics and frequency to infer meaning. List all fields from summary stats: Unique Key, Created Date, Closed Date, Agency, Agency Name, Complaint Type, Descriptor, Location Type, Incident Zip, Incident Address, Street Name, Cross Street 1, Cross Street 2, Intersection Street 1, Intersection Street 2, Address Type, City, Landmark, Facility Type, Status, Due Date, Resolution Description, Resolution Action Updated Date, Community Board, BBL, Borough, X Coordinate (State Plane), Y Coordinate (State Plane), Open Data Channel Type, Park Facility Name, Park Borough, Vehicle Type, Taxi Company Borough, Taxi Pick Up Location, Bridge Highway Name, Bridge Highway Direction, Road Ramp, Bridge Highway Segment, Latitude, Longitude, Location.

We need JSON mapping each field to label and description in Mandarin (Chinese). Provide human-friendly label (Chinese) and detailed description.

We'll craft based on typical NYC 311 dataset fields.

Let's produce concise yet descriptive paragraphs.

Return JSON.


## TOKEN USAGE

TokenUsage { prompt: 13169, completion: 2959, total: 16128, elapsed: 32420 }
---
# Description
**描述**  
该数据集包含约 **100 万条纽约市 311 请求记录**，时间跨度从 **2010‑01‑01** 到 **2020‑12‑23**。每条记录由全局唯一的 **Unique Key** 标识，包含创建时间、关闭时间、所属机构、投诉类型、地点信息及处理状态等字段。大多数请求已经 **Closed（约 95%）**，而 **Complaint Type**、**Descriptor**、**Resolution Description** 等字段呈高度分散，超过一半的记录归入 “Other” 类别，说明实际投诉内容种类繁多。地理信息（坐标、街道、邮编）仅约 **25%** 完备，且 **Incident Zip**、**Incident Address** 等字段的唯一值比例极高，几乎每条记录都是唯一位置。提交渠道主要为 **电话（≈50%）**，其次是 **未知（≈23%）** 与 **在线（≈18%）**。部分字段（如 **Address、Latitude/Longitude、BBL**）可能包含 **个人可识别信息（PII）**，需在使用时注意隐私合规。  

**显著特征**  
- **规模与唯一性**：100 万唯一键；多数时间、地址、邮编字段均为独立值（高基数），导致 “Other” 类别占比超 80%。  
- **缺失值**：约 **25%** 的坐标数据缺失；**Incident Zip**、**Incident Address**、**Landmark** 等字段缺失率分别为 5.5%、17.5% 与 91%。  
- **类别分布不均**：**Agency** 前三家（NYPD、HPD、DOT）共占约 **66%**，其余机构分散；**Complaint Type** 前十类仅占约 **44%**，其余被归为 “Other”。  
- **时间分布**：创建日期高度离散，前十个具体日期仅占 **约 3%**，其余 97% 分布在其他日期；关闭日期同理。  
- **地理集中**：**Borough** 分布相对均衡，Brooklyn、Queens、Manhattan、Bronx 各占约 **20‑30%**，但 **Location Type** 中 “RESIDENTIAL BUILDING” 与缺失值合计约 **50%**。  
- **处理渠道**：电话渠道主导（≈50%），但 **UNKNOWN** 渠道占比也不少（≈23%），提示数据收集过程可能存在来源不明记录。  
- **潜在 PII/PCI**：地址、坐标、BBL、邮编等字段可以定位个人住宅，使用前需脱敏或获得适当授权。  
- **数据质量**：大量 “Other” 类别与高基数字段导致难以直接进行汇总分析，需要先进行分类合并或抽样。  

---  
**属性**: 
Generated by qsv v11.0.2 describegpt
Command line: target/debug/qsv describegpt --all NYC_311_SR_2010-2020-sample-1M.csv --language Mandarin --model openai/gpt-oss-120b --output nyc311-describegpt-mandarin.md
Prompt file: Default v6.3.0
Model: openai/gpt-oss-120b
LLM API URL: https://api.together.xyz/v1
Timestamp: 2025-12-14T04:37:33.638980+00:00

WARNING: Generated by an LLM and may contain inaccuracies. Verify before using!
  


## REASONING

The task: generate a Mandarin description of dataset based on summary statistics and frequency distribution, not per-field but overall in 1-8 sentences. Then a bullet list of notable characteristics. Include attribution placeholder. Use markdown.

We need to synthesize overall dataset: size? Not directly given, but we infer from Unique Key cardinality = 1,000,000 records (since count 1000000). So dataset ~1 million rows.

Key points: Unique key all unique. Created Date distribution: many distinct dates (top 10 each <0.04%); "Other" covers 99.733% meaning many different dates. Closed Date similar, many missing (NULL 2.86%?) Actually Closed Date NULL count 28619 = 2.86% missing. Frequency shows many distinct dates.

Agency: top few agencies cover about half of records (NYDP 26.5%, HPD 25.8%, etc). Others 3.2% category.

Complaint Type: top few categories about 8-10% each, but "Other" 56% of records.

Descriptor: top few; "Other" 67% records.

Location Type: most common RESIDENTIAL BUILDING 25.6%; Null 23.9%; others.

Incident Zip: many missing (5.5% null) and "Other" 82.8% other zip codes beyond top 10.

Incident Address: 17.5% null; top few addresses tiny fraction; "Other" 81.94% other.

Street Name: many null 17.5%; top few 0.97% etc; "Other" 78.7% other.

Cross Street etc have many null and many other.

Address Type: majority ADDRESS 71%; INTERSECTION 13%; Null 12.6%.

City: top three boroughs (Brooklyn 29.6%, New York (Manhattan) 18.9%, Bronx 18.1%); 6.2% null; Others 17%.

Landmark: 91% null.

Facility Type: N/A 62.8%; Precinct 19.3%; null 14.5%.

Status: 95% Closed, remainder pending/open etc.

Due Date: 64.78% null; others spread across a few dates; "Other" 35% other.

Resolution Description: many distinct descriptions; "Other" 53%; top few together ~9%.

Resolution Action Updated Date: many null 98%; small others.

Community Board: Many "Other" 75%; top few small.

BBL: many null (24.3%); other many unique.

Borough: distribution across 5 boroughs, each about 20-30%; Unspecified 5%.

Coordinates: some null 8.5%; rest varied.

Open Data Channel: PHONE 49.8%; UNKNOWN 23%; ONLINE 17.7%; MOBILE 8%; OTHER 1.5%.

Park Facility Name: 99% Unspecified; small.

Vehicle Type: 99.96% null.

Taxi Company Borough: 99.9% null.

Taxi Pick Up Location: 99.2% null; others tiny.

Bridge Highway etc: mostly null.

Latitude/Longitude: 25% null each; rest have values; distribution around NYC coordinates.

Thus notable characteristics: high cardinality fields (Created Date, Incident Address, etc.) many unique values; large proportion of missing values in many fields; dominant categories for some fields; dataset contains PII? Possibly addresses, coordinates, BBL (building IDs) could be considered PII. Also timestamps.

Now produce description 1-8 sentences about dataset: maybe mention size, time range, main content: 311 service requests (NYC) with complaints, etc.

Now bullet list.

Finally attribution placeholder.

Make markdown.

Ok.
## TOKEN USAGE

TokenUsage { prompt: 15918, completion: 1478, total: 17396, elapsed: 24713 }
---
# Tags
- nyc_311_服务请求  
- 噪音投诉  
- 住宅建筑问题  
- 公共设施维护  
- 城市交通问题  
- 供热与管道问题  
- 空间坐标数据  
- 时间趋势分析  
- 多渠道提交  
- 跨部门责任  

Attribution: 
Generated by qsv v11.0.2 describegpt
Command line: target/debug/qsv describegpt --all NYC_311_SR_2010-2020-sample-1M.csv --language Mandarin --model openai/gpt-oss-120b --output nyc311-describegpt-mandarin.md
Prompt file: Default v6.3.0
Model: openai/gpt-oss-120b
LLM API URL: https://api.together.xyz/v1
Timestamp: 2025-12-14T04:38:15.341810+00:00

WARNING: Generated by an LLM and may contain inaccuracies. Verify before using!



## REASONING

We need to produce up to 10 Mandarin tags (lowercase, underscore separated) describing the dataset. Must not use field names. Base on summary stats and frequency distribution. Choose tags reflecting main themes: 311 service requests, NYC, complaints, noise, housing, public works, etc. Also include spatial/geographic, time series, open data channel. Tags in Mandarin, lowercase, underscores.

We need to add Attribution placeholder after tags. Use markdown format, list of tags each as `- tag`. At bottom add attribution line.

Ensure no field names like "complaint_type". Use concepts.

We can think: "纽约市311服务请求", "噪音投诉", "住宅建筑问题", "公共设施维护", "城市交通与道路", "住宅区漏水与供热", "空间坐标数据", "时间序列分析", "多渠道提交", "政府部门交叉". Need up to 10, choose most important.

Order descending importance.

Let's craft tags: 
1. nyc_311_service_requests
2. noise_complaints
3. residential_building_issues
4. public_infrastructure_maintenance
5. city_transportation_issues
6. heating_and_plumbing_problems
7. geographic_spatial_data
8. temporal_trends_analysis
9. multi_channel_submission
10. interagency_responsibility

All lowercase, underscores, Mandarin? Wait tags should be Mandarin (Chinese) words, not English. Must be Mandarin, but lowercase only and underscores. In Mandarin, characters are not case-sensitive. Lowercase concept doesn't apply. But instruction: "lowercase only and use _ to separate words". For Mandarin, we can use pinyin (romanization) maybe? Typically tags are English. But they require Mandarin tags. Could use Chinese characters combined with underscores? Not typical. Probably they want Chinese words in pinyin (lowercase) separated by underscores. Or Chinese characters with underscores (they are not uppercase). It's okay. We'll produce pinyin versions representing Mandarin: e.g., "nyc_311_fuwu_zhengqiu" (but "nyc" is English). Better use Chinese: "nyc_311_服务请求" mixing. Might be okay. To be safe, use pinyin tags: "shanghai"? No dataset is NYC. Use "nyc" maybe acceptable. Provide tags in Mandarin pinyin: "nyc_311_fuwu_qingqiu". Others: "zaoyin_tousu", "zhuzhai_jianzhuwenti", "gonggong_sheshi_weihu", "chengshi_jiaotong_wenti", "re_shui_guanxi_wenti", "weidu_weizhi_shuju", "shijian_qushi_fenxi", "duogong_tou_shang_chuangjian", "jiguan_huobang_zhui". But we need clarity.

I'll create tags using Chinese characters with underscores, like "nyc_311_服务请求". Chinese characters are not case. It's fine.

Let's create 10 tags:

- nyc_311_服务请求
- 噪音投诉
- 住宅建筑问题
- 公共设施维护
- 城市交通问题
- 供热与管道问题
- 空间坐标数据
- 时间趋势分析
- 多渠道提交
- 跨部门责任

Add attribution line: `Attribution: {GENERATED_BY_SIGNATURE}`

Return in markdown.
## TOKEN USAGE

TokenUsage { prompt: 15879, completion: 805, total: 16684, elapsed: 24874 }
---
