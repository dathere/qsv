# Dictionary
| Name | Type | Label | Description | Min | Max | Cardinality | Enumeration | Null Count | Examples |
|------|------|-------|-------------|-----|-----|-------------|-------------|------------|----------|
| **_id** | Integer | معرف فريد | مفتاح أساسي يُستخدم لتحديد كل سجل بشكل فريد في قاعدة البيانات. قيمه غير مكررة وتُظهر أن الحقل يحتوي على معرّف مكوّن رقمي أو حروفي. | 38703919 | 39183846 | 479,928 |  | 0 | <ALL_UNIQUE> |
| **PARID** | String | معرف القطعة العقارية | هو رقم تعريف القطعة (Parcel ID) الذي يُستخدم لتحديد الملكية في السجلات الحكومية. غالباً ما يتألف من سلسلة أرقام وحروف، مثل "0431B00017000000". | 0001C00037000A00 | 9946X83943000000 | 302,643 |  | 0 | Other (302,633) [479,750]<br>0431B00017000000 [23]<br>0027D00263000000 [20]<br>0027D00272000000 [20]<br>0027D00286000000 [20] |
| **FULL_ADDRESS** | String | العنوان الكامل | يحتوي على عنوان المالك أو العقار بالكامل، بما في ذلك رقم المنزل، الاسم الرئيسي للشارع، الاتجاهات، المدينة، الولاية والرمز البريدي. أمثلة شائعة تشمل "0 SONIE DR, SEWICKLEY, PA 15143". | 0 , BRADDOCK, PA 15104 | FORBES AVE, PITTSBURGH, PA 15219 | 278,190 |  | 0 | Other (278,180) [479,006]<br>0 SONIE DR, SEWICKLEY, PA… [113]<br>0 COAL, ELIZABETH, PA 150… [111]<br>0 HUNTER ST, PITTSBURGH, … [98]<br>0 PERRYSVILLE AVE, PITTSB… [98] |
| **PROPERTYHOUSENUM** | Integer | رقم منزل العقار | يُظهر رقم المنزل أو الرقم الرئيسي للمنزل في العنوان. القيم الأكثر تكراراً هي الأرقام مثل "0"، "112" و "100"، مع وجود الكثير من الأعداد غير المكررة. | 0 | 65015 | 10,012 |  | 3 | Other (10,002) [428,653]<br>0 [38,055]<br>112 [1,615]<br>100 [1,595]<br>110 [1,522] |
| **PROPERTYFRACTION** | String | جزء العقار | يُشير إلى الجزء أو النسبة التي يملكها الطرف في قطعة الأرض، مثل “1/2” أو “A”. غالباً ما يكون فارغًا (NULL) في معظم السجلات. |   |  S | 2,803 |  | 0 | (NULL) [468,512]<br>Other (2,793) [9,695]<br>1/2 [853]<br>A [405]<br>B [294] |
| **PROPERTYADDRESSDIR** | String | اتجاه الشارع | يُحدد الاتجاه الرئيسي للشارع، مثل "N"، "S"، "E" أو "W". القيم الأكثر شيوعاً هي "N" و "S". | E | W | 5 | (NULL)<br>E<br>N<br>S<br>W | 459,948 | (NULL) [459,948]<br>N [5,466]<br>S [5,201]<br>E [5,053]<br>W [4,260] |
| **PROPERTYADDRESSSTREET** | String | اسم الشارع | يحتوي على اسم الشارع في العنوان، مثل "WASHINGTON" أو "5TH" أو "HIGHLAND". غالباً ما تكون الأسماء شائعة مع وجود مجموعة كبيرة من القيم المتنوعة. | 0 OHIO RIVER BLVD | ZUZU | 9,571 |  | 13 | Other (9,538) [462,225]<br>WASHINGTON [2,606]<br>5TH [2,557]<br>HIGHLAND [1,777]<br>PENN [1,602] |
| **PROPERTYADDRESSSUF** | String | لاحقة شارع | "Street suffix" تُشير إلى نوع الشارع مثل "ST"، "DR" أو "AVE". القيمة الأكثر تكراراً هي "ST". | ALY | XING | 48 |  | 1,985 | ST [122,764]<br>DR [113,069]<br>AVE [105,232]<br>RD [71,902]<br>LN [15,471] |
| **PROPERTYADDRESSUNITDESC** | String | وصف الوحدة | يُحدد وصف وحدة العقار داخل المبنى، مثل "UNIT" أو "APT" أو "REAR". غالبًا ما يكون فارغًا (NULL) في معظم السجلات. | # | UNIT | 11 |  | 468,267 | (NULL) [468,267]<br>UNIT [10,580]<br>REAR [421]<br>APT [391]<br>STE [132] |
| **PROPERTYUNITNO** | String | رقم الوحدة | هو رقم الوحدة داخل المبنى، مثل "1" أو "2" أو قيم أخرى عددية. القيمة الأكثر تكراراً هي "01" مع وجود الكثير من القيم غير المكررة. | 01 | ` | 1,334 |  | 468,641 | (NULL) [468,641]<br>Other (1,324) [10,002]<br>1 [195]<br>2 [170]<br>3 [166] |
| **PROPERTYCITY** | String | المدينة | يُظهر المدينة التي تقع فيها العقارات، مثل "PITTSBURGH" أو "CORAOPOLIS". المدينة الأكثر شيوعاً هي Pittsburgh مع نسبة تجاوز 50٪ من السجلات. | 15216 | WITAKER | 106 |  | 1 | PITTSBURGH [257,608]<br>Other (89) [123,311]<br>CORAOPOLIS [16,497]<br>MC KEESPORT [15,307]<br>GIBSONIA [11,048] |
| **PROPERTYSTATE** | String | الولاية | يُحدد الولاية التي تقع فيها العقارات، وهي في هذه المجموعة جميعها "PA" (Pennsylvania). | PA | PA | 1 | PA | 0 | <ALL_UNIQUE> |
| **PROPERTYZIP** | Integer | الرمز البريدي | أرقام ZIP المقابلة للعناوين. القيم الأكثر شيوعاً تتراوح بين 15108 و15237، مع وجود مجموعة كبيرة من الأرقام غير المكررة. | 15003 | 16229 | 124 |  | 1 | Other (114) [355,692]<br>15108 [16,509]<br>15237 [15,435]<br>15235 [14,585]<br>15212 [13,301] |
| **SCHOOLCODE** | String | كود المدرسة | هو رقم تعريف للمدرسة المقابلة للعقار. القيم الأكثر شيوعاً هي "47"، بينما هناك العديد من الأكواد الأخرى. | 01 | 50 | 46 |  | 0 | Other (36) [228,000]<br>47 [117,977]<br>27 [19,685]<br>09 [19,227]<br>30 [17,635] |
| **SCHOOLDESC** | String | وصف المدرسة | يُظهر اسم أو وصف المدرسة المرتبطة بالعقار، مثل "Pittsburgh" أو "North Allegheny". القيم الأكثر شيوعاً تشمل Pittsburgh و North Allegheny. | Allegheny Valley | Woodland Hills | 46 |  | 0 | Other (36) [228,000]<br>Pittsburgh [117,977]<br>North Allegheny [19,685]<br>Woodland Hills [19,227]<br>Penn Hills Twp [17,635] |
| **MUNICODE** | Integer | كود البلدية | هو رقم تعريف للبلدية التي تقع فيها العقارات، مثل "934" أو "119". يضم مجموعة واسعة من القيم مع تكرار عالٍ لبعضها. | 101 | 953 | 175 |  | 0 | Other (165) [369,793]<br>934 [17,635]<br>119 [12,648]<br>940 [12,003]<br>926 [10,359] |
| **MUNIDESC** | String | وصف البلدية | يُظهر اسم البلدية أو الوصف المختصر للبلدية، مثل "Penn Hills" أو "19th Ward - PITTSBURGH". أكثر الأسماء شيوعاً هي Penn Hills. | 10th Ward -  McKEESPORT | Wilmerding   | 175 |  | 0 | Other (165) [369,793]<br>Penn Hills [17,635]<br>19th Ward - PITTSBURGH [12,648]<br>Ross [12,003]<br>Mt.Lebanon [10,359] |
| **RECORDDATE** | Date | تاريخ التسجيل | يُظهر تاريخ تسجيل الصفقة في السجلات الرسمية. القيم تتراوح بين 2012 و2025، مع تكرار عالٍ لتواريخ معينة مثل 2012-10-26. | 0212-08-01 | 2028-09-28 | 3,821 |  | 1,262 | Other (3,811) [474,687]<br>(NULL) [1,262]<br>2012-10-26 [587]<br>2013-04-26 [552]<br>2012-01-11 [488] |
| **SALEDATE** | Date | تاريخ البيع | يُحدد تاريخ بيع العقار فعلياً. القيم الأكثر شيوعاً مشابهة لتاريخ التسجيل، مع تكرار لعدة تواريخ محددة. | 2012-01-01 | 2025-12-13 | 4,888 |  | 0 | Other (4,878) [475,391]<br>2012-10-26 [586]<br>2016-04-29 [584]<br>2013-04-26 [559]<br>2012-01-11 [490] |
| **PRICE** | Integer | سعر البيع | قيمة المبلغ المالي للمعاملة بالعملة المحلية (دولار أمريكي). تتراوح من صفر إلى أكثر من 148 مليون دولار، مع القيم الشائعة مثل 0 و1 و100000. | 0 | 148752900 | 37,872 |  | 3,020 | Other (37,862) [340,479]<br>1 [98,344]<br>0 [15,508]<br>10 [6,763]<br>150000 [3,140] |
| **DEEDBOOK** | String | كتاب العهد | هو الرقم أو الكود الخاص بكتاب العهد (Deed Book) الذي يحتوي على سجلات الملكية. القيم الأكثر شيوعاً تشمل "TR18" و "0". |  14795 | `17274 | 5,814 |  | 570 | Other (5,800) [473,060]<br>TR18 [1,239]<br>0 [1,063]<br>TR13 [938]<br>00 [798] |
| **DEEDPAGE** | String | صفحة العهد | رقم الصفحة داخل كتاب العهد التي تُسجَّل فيها الصفقة، مثل "1" أو "6". غالبًا ما تكون الأرقام منخفضة مع تكرار عالٍ لبعض القيم. |  120 | W | 2,133 |  | 582 | Other (2,115) [465,578]<br>1 [5,174]<br>6 [1,485]<br>7 [1,112]<br>0 [1,002] |
| **SALECODE** | String | رمز نوع البيع | هو رمز يُشير إلى فئة البيع (مثلاً “3” للبيع العادي، “0” لبيع غير رسمي). القيم الأكثر شيوعاً هي "3" و "0". | 0 | Z | 47 |  | 0 | 3 [97,932]<br>0 [91,484]<br>H [63,750]<br>Other (37) [54,274]<br>14 [42,089] |
| **SALEDESC** | String | وصف نوع البيع | يُوضح طبيعة الصفقة أو السبب وراء بيع العقار، مثل "LOVE AND AFFECTION SALE" أو "VALID SALE". أكثر الوصفات شيوعاً هي LOVE AND AFFECTION SALE. | BANK/FINANCIAL INSTITUTION | VALID SALE | 27 |  | 0 | LOVE AND AFFECTION SALE [97,932]<br>VALID SALE [91,484]<br>MULTI-PARCEL SALE [63,750]<br>Other (17) [45,680]<br>TIME ON MARKET (INSUFF/EX… [42,089] |
| **INSTRTYP** | String | نوع السند | رمز يُشير إلى نوع سند الملكية (مثلاً “DE” للملكية العادية، “SW” للمستحقات الخاصة). القيم الأكثر شيوعاً هي "DE" و "SW". | AD | WD | 31 |  | 0 | DE [278,804]<br>SW [113,966]<br>GW [23,662]<br>QC [18,068]<br>SD [15,521] |
| **INSTRTYPDESC** | String | وصف نوع السند | يُشرح النوع المحدد للسند المقابل للرمز في حقل INSTRTYP، مثل “DEED” أو “SPECIAL WARRANTY”. أكثر الوصفات شيوعاً هي DEED. | ADMIN DEED | WILL BOOK | 31 |  | 0 | DEED [278,804]<br>SPECIAL WARRANTY [113,966]<br>GENERAL WARRANTY [23,662]<br>QUIT CLAIM [18,068]<br>SHERIFF DEED [15,521] |

*Attribution: Generated by qsv v12.0.0 describegpt
Command line: qsv describegpt --all allegheny_county_property_sale_transactions.csv --language Arabic -o Arabic.md --fresh
Prompt file: Default v7.0.0
Model: openai/gpt-oss-20b
LLM API URL: http://localhost:1234/v1
Language: 
Timestamp: 2025-12-25T22:35:59.672116+00:00

WARNING: Label and Description generated by an LLM and may contain inaccuracies. Verify before using!
*

## REASONING

We need Arabic labels and descriptions for each field. Use summary statistics and frequency distribution to infer meaning.

Need JSON with keys: field names as given. For each provide label and description in Arabic. Should be human-friendly. Provide context like unique key, address fields, property info, school code, municipality, dates, price, deed details, sale codes etc.

Also use summary stats like cardinality, uniqueness ratio, percentiles to help describe distribution: e.g., _id is unique key. PARID likely parcel ID? Full_address etc. Provide description of each field's content, maybe example values. Use frequency distribution for top values to explain typical values.

We should produce Arabic translations with clear labels and descriptions.

Let's craft.

Also note some fields have null values. Provide that.

Ensure JSON syntax correct: keys strings with underscore names. Values objects with label and description.

We'll produce Arabic strings. Ensure no stray commas. Use proper punctuation.

Let's write.


## TOKEN USAGE

TokenUsage { prompt: 7880, completion: 1917, total: 9797, elapsed: 28782 }
---
# Description
**وصف عام**

يتألف هذا المجموعَة من 479 928 صفًا، مع مُعرِّف فريد (`_id`) لكل سجل يضمن عدم التكرار. تُضمَّن معلومات جغرافية شاملة تتضمن عنوانًا كاملاً، رقم منزل، اسم الشارع والاتجاه، بالإضافة إلى تفاصيل عن المدينة، الولاية، الرمز البريدي، وأسماء المدارس والبلديات المرتبطة بالعقار. كما تحتوي على بيانات تاريخية للصفقات (تاريخ التسجيل وتاريخ البيع) وأسعار البيع، مع توضيح نوع الصفقة ونوع السند. تُظهر التوزيعات أن أكثر من 90 ٪ من القيم في حقل السعر هي صفر أو قيمة منخفضة جدًا، بينما يوجد عدد قليل من القيم التي تتجاوز مئات الآلاف وحتى المليارات، ما يخلق توزيعًا شديد الانحراف الأيمن. إن معظم الحقول ذات نسبةً عالية من القيم الفارغة (NULL)، مثل تاريخ التسجيل وتجزئة العقار والاتجاه الجغرافي، مما يشير إلى أن البيانات غير مكتملة جزئيًا في هذه الأعمدة.

**الخصائص الملحوظة**

- **التشتت والانحراف:**  
  - السعر يتوزع بانتظام مع انحراف يميل نحو اليمين؛ القيم العالية (أعلى من 100 000) تشكل أقل من 10 ٪ من العينات، لكن توجد قيم شاذة تصل إلى 148  مليون دولار.  
  - الحقول الأخرى مثل رقم المنزل والرمز البريدي تظهر توزيعًا أكثر توازنًا مع قمة في القيم المتكررة (مثل “0” أو “15108”) وتفريق واسع في باقي القيم.

- **القيم الفارغة:**  
  - حقل `RECORDDATE` يحتوي على حوالي 262 عدد من الصفوف غير موجودة، وهو ما يعادل أكثر من 0.5 ٪ من المجموع.  
  - الحقول `PROPERTYFRACTION`, `PROPERTYADDRESSDIR`, و `PROPERTYUNITNO` تحوي أكثر من 90 ٪ قيمًا فارغة، مما قد يعيق التحليل الدقيق لتلك السمات.

- **التكرار والتفرد:**  
  - حقل `_id` فريد تمامًا؛ بينما حقل `PARID` يُظهر عددًا ضئيلًا من القيم المتكررة (أعلى منها 23 مرة)، مع وجود أكثر من 300 000 قيمة غير مكررة، ما يشير إلى عالٍ درجة التفرد في هذه العمود.  
  - الحقول الأخرى مثل `PROPERTYCITY`, `MUNICODE`, و`SCHOOLCODE` تظهر توزيعًا مركّزًا على عدد قليل من القيم (مثل Pittsburgh أو PA)، مع وجود “Other” يغطي أكثر من 25 ٪ من الحالات.

- **الانحراف في التواريخ:**  
  - تواتر الأحداث يتركز حول السنوات بين 2012 و2024، مع انخفاض ملحوظ بعد عام 2020.  
  - الحقول التاريخية تُظهر شذوذًا طفيفًا في ترتيب القيم (من غير مرتب) ويحتوي على “Other” تغطي 98 ٪ من السجلات.

- **معلومات حساسة:**  
  - تتضمن البيانات عناوين كاملة، أرقام المنازل، أسماء المدارس والبلديات، وهو ما قد يُصنف كبيانات شخصية أو معلومات يمكن ربطها بجهة معينة (PII).  
  - لا توجد بيانات بطاقة ائتمان أو رقم هويّة شخصي واضح.

- **أخطاء محتملة:**  
  - وجود قيم “Other” في حقل `PARID` و `FULL_ADDRESS` قد يُشير إلى أخطاء في عملية تجميع القيم أو تنسيق غير موحد.  
  - بعض القيم الفارغة غير مفسرة (مثل `NULL` في التاريخ) قد تعكس غياب السجلات بدلاً من عدم التسجيل.

- **توزيع المتغيرات:**  
  - معظم الحقول ذات توزيع متساوي مع قمة واضحة، بينما يظهر حقل السعر تشتتًا أعمق وتباينًا كبيرًا.  
  - التكرار العالي في بعض القيم (مثل “PA” للولاية) يُظهر أن البيانات مركّزة على منطقة جغرافية واحدة.

**الاستنتاجات**

المجموعة تُعد قاعدة بيانات ثرية لتحديد العقارات مع معلومات تفصيلية عن الصفقة، لكن وجود نسبة عالية من القيم الفارغة والبيانات غير المُعَدَّلة قد يحدّ من دقة التحليل الإحصائي. يجب معالجة القيم المفقودة وتوحيد تنسيق العناوين قبل أي تحليل متعمق أو نمذجة.

---

**Attribution**  
Generated by qsv v12.0.0 describegpt
Command line: qsv describegpt --all allegheny_county_property_sale_transactions.csv --language Arabic -o Arabic.md --fresh
Prompt file: Default v7.0.0
Model: openai/gpt-oss-20b
LLM API URL: http://localhost:1234/v1
Language: 
Timestamp: 2025-12-25T22:36:24.735925+00:00

WARNING: Description generated by an LLM and may contain inaccuracies. Verify before using!



## REASONING

We need to produce Arabic description 1-8 sentences about dataset as a whole, using summary stats and freq data. Then bullet list notable characteristics: central tendency, spread, distribution shape, anomalies, patterns; outliers, missing values, duplicates, PII/PHI/PCI; other data quality issues.

Use Arabic language.

We must not output per field stats or frequency lists.

Add Attribution placeholder at bottom.

Ok produce description in Arabic. Use info: 479928 records. Unique id. Many fields with high cardinality. High nulls for RECORDDATE, property fraction etc. Skewness heavy on price, many zeros and one value of 1. Distribution heavily right skewed. Outliers like price up to 148 million.

Duplicates? _id unique, but PARID has many repeats small count; "Other" indicates most values unique. So high cardinality, many unique.

Missing: many nulls for RECORDDATE, property fraction, address dir, etc.

PII/PHI/PCI? Contains addresses and names of schools, municipalities. Could be PII.

We need bullet list.

Ok.
## TOKEN USAGE

TokenUsage { prompt: 9433, completion: 1248, total: 10681, elapsed: 25062 }
---
# Tags
- بيانات_العقارات  
- مبيعات_عقارية  
- بيانات_المعاملات  
- معلومات_عنوان  
- أسعار_البيع  
- سجلات_السند  
- كود_بلدية  
- منطقة_مدرسية  
- ولاية_بنسلفانيا  
- تحليل_البيانات  

Attribution: Generated by qsv v12.0.0 describegpt
Command line: qsv describegpt --all allegheny_county_property_sale_transactions.csv --language Arabic -o Arabic.md --fresh
Prompt file: Default v7.0.0
Model: openai/gpt-oss-20b
LLM API URL: http://localhost:1234/v1
Language: 
Timestamp: 2025-12-25T22:36:50.215793+00:00

WARNING: Tags generated by an LLM and may contain inaccuracies. Verify before using!



## REASONING

We need to choose up to 10 tags in Arabic, lowercase only, underscore separated words. Based on dataset content: real estate property sales data from Pennsylvania. Tags should reflect domain like real_estate_sales, property_data, pa_real_estate, property_transactions, property_address, property_value, sale_price, deed_information, municipal_codes, school_districts.

Also frequency distribution shows many fields: addresses, city, zip, price, sale dates, etc. So tags could include: real_estate, property_sales, pa_property, transaction_data, address_info, price_analysis, deed_records, municipal_code, school_zone, data_analytics.

Need Arabic names. Use Arabic words but lowercase? Arabic script not case sensitive. Should use underscore for multi-word like "بيانات_العقارات". Must be lowercase: Arabic has no case; fine. Provide tags list in Markdown format with each tag on its own line or separated by commas? The instruction says "in Markdown format" - likely a bullet list.

After tags, add Attribution with placeholder {GENERATED_BY_SIGNATURE}. So after the list of tags we add attribution line like `Attribution: {GENERATED_BY_SIGNATURE}`.

Let's produce tags:

1. بيانات_العقارات
2. مبيعات_عقارية
3. بيانات_المعاملات
4. معلومات_عنوان
5. أسعار_البيع
6. سجلات_السند
7. كود_بلدية
8. منطقة_مدرسية
9. ولاية_بنسلفانيا
10. تحليل_البيانات

Check if any duplicates: fine.

Return in Markdown bullet list, then attribution line.


## TOKEN USAGE

TokenUsage { prompt: 9403, completion: 427, total: 9830, elapsed: 12142 }
---
