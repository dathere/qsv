# Dictionary
| Name | Type | Label | Description | Min | Max | Cardinality | Enumeration | Null Count | Examples |
|------|------|-------|-------------|-----|-----|-------------|-------------|------------|----------|
| **_id** | Integer | Identificador Único | Campo que contém um identificador numérico único para cada registro no conjunto de dados, garantindo a distinção entre diferentes transações imobiliárias. Todos os valores são distintos, resultando em uma taxa de unicidade de 100 %. | 38703919 | 39183846 | 479,928 |  | 0 | <ALL_UNIQUE> |
| **PARID** | String | Código do Parcela | Código alfanumérico que identifica unicamente o lote ou parcela na propriedade, usado em registros cadastrais e processos de venda. Existem 302 633 valores distintos dentre os 479 928 registros, representando cerca de 63 % de unicidade. | 0001C00037000A00 | 9946X83943000000 | 302,643 |  | 0 | Other (302,633) [479,750]<br>0431B00017000000 [23]<br>0027D00263000000 [20]<br>0027D00272000000 [20]<br>0027D00286000000 [20] |
| **FULL_ADDRESS** | String | Endereço Completo | Campo composto que reúne todas as informações necessárias para localizar a propriedade: número da casa, nome da rua, direção (N/S/E/W), sufixo da rua, descrição da unidade, cidade, estado e CEP. Os endereços mais frequentes são ‘0 SONIE DR, SEWICKLEY, PA 15143’ e ‘0 COAL, ELIZABETH, PA 15037’. Aproximadamente 58 % dos registros têm endereços únicos. | 0 , BRADDOCK, PA 15104 | FORBES AVE, PITTSBURGH, PA 15219 | 278,190 |  | 0 | Other (278,180) [479,006]<br>0 SONIE DR, SEWICKLEY, PA… [113]<br>0 COAL, ELIZABETH, PA 150… [111]<br>0 HUNTER ST, PITTSBURGH, … [98]<br>0 PERRYSVILLE AVE, PITTSB… [98] |
| **PROPERTYHOUSENUM** | Integer | Número de Residência | Número atribuído à residência ou edifício na propriedade. Valores comuns incluem ‘0’ (sem número), ‘112’, ‘100’, ‘110’, entre outros. A maioria dos registros possui um número válido, com 89 % dos valores agrupados em 10 002 valores distintos. | 0 | 65015 | 10,012 |  | 3 | Other (10,002) [428,653]<br>0 [38,055]<br>112 [1,615]<br>100 [1,595]<br>110 [1,522] |
| **PROPERTYFRACTION** | String | Frção da Propriedade | Indica se a parcela representa uma fração parcial de uma propriedade, por exemplo ‘1/2’, ‘A’ ou ‘B’. Mais de 97 % dos registros têm valor nulo (sem frção), enquanto os valores não nulos são distribuídos entre 2 793 entradas distintas. |   |  S | 2,803 |  | 0 | (NULL) [468,512]<br>Other (2,793) [9,695]<br>1/2 [853]<br>A [405]<br>B [294] |
| **PROPERTYADDRESSDIR** | String | Direção da Rua | Código abreviado que indica a direção relativa à rua (Norte, Sul, Este, Oeste). A maioria dos registros tem valor nulo; quando presente, os valores mais frequentes são ‘N’, ‘S’, ‘E’ e ‘W’. Aproximadamente 96 % dos registros possuem direção especificada. | E | W | 5 | (NULL)<br>E<br>N<br>S<br>W | 459,948 | (NULL) [459,948]<br>N [5,466]<br>S [5,201]<br>E [5,053]<br>W [4,260] |
| **PROPERTYADDRESSSTREET** | String | Nome da Rua | Texto que identifica o nome da rua onde a propriedade está localizada. Nomes como ‘WASHINGTON’, ‘5TH’, ‘HIGHLAND’ aparecem com maior frequência. A maioria dos registros possui um nome de rua distinto, mas 96 % dos valores são agrupados em 9 538 entradas diferentes. | 0 OHIO RIVER BLVD | ZUZU | 9,571 |  | 13 | Other (9,538) [462,225]<br>WASHINGTON [2,606]<br>5TH [2,557]<br>HIGHLAND [1,777]<br>PENN [1,602] |
| **PROPERTYADDRESSSUF** | String | Sufixo da Rua | A abreviação que descreve o tipo de via, por exemplo ‘ST’, ‘DR’, ‘AVE’. Os sufixos mais comuns são ‘ST’ (25 %), ‘DR’ (23 %) e ‘AVE’ (22 %). Cerca de 98 % dos registros têm um sufixo especificado. | ALY | XING | 48 |  | 1,985 | ST [122,764]<br>DR [113,069]<br>AVE [105,232]<br>RD [71,902]<br>LN [15,471] |
| **PROPERTYADDRESSUNITDESC** | String | Descrição da Unidade | Indicador textual do tipo de unidade dentro do edifício (por exemplo, ‘UNIT’, ‘REAR’, ‘APT’). A maioria dos registros tem valor nulo; quando presente, os valores mais frequentes são ‘UNIT’. Aproximadamente 98 % dos registros têm descrição de unidade especificada. | # | UNIT | 11 |  | 468,267 | (NULL) [468,267]<br>UNIT [10,580]<br>REAR [421]<br>APT [391]<br>STE [132] |
| **PROPERTYUNITNO** | String | Número da Unidade | Número interno ou identificador de unidade dentro do edifício. Valores como ‘1’, ‘2’, ‘3’ aparecem com maior frequência; a maioria dos registros tem valor nulo, representando cerca de 97 % das entradas. | 01 | ` | 1,334 |  | 468,641 | (NULL) [468,641]<br>Other (1,324) [10,002]<br>1 [195]<br>2 [170]<br>3 [166] |
| **PROPERTYCITY** | String | Cidade | Nome da cidade onde a propriedade está situada. Cidades como ‘PITTSBURGH’, ‘CORAOPOLIS’ e ‘MC KEESPORT’ são as mais frequentes, com Pittsburgh representando 53 % das entradas. | ALLISON PARK | WITAKER | 106 |  | 1 | PITTSBURGH [257,608]<br>Other (89) [123,311]<br>CORAOPOLIS [16,497]<br>MC KEESPORT [15,307]<br>GIBSONIA [11,048] |
| **PROPERTYSTATE** | String | Estado | Sigla do estado (ex.: PA) onde a propriedade está localizada. Todos os registros pertencem ao estado de Pennsylvania (PA). | PA | PA | 1 | PA | 0 | <ALL_UNIQUE> |
| **PROPERTYZIP** | Integer | CEP | Código postal numérico que identifica a área da propriedade. Os códigos mais frequentes incluem ‘15108’, ‘15237’ e ‘15235’. Cerca de 74 % das entradas têm valores fora dos top 10, indicando ampla variação. | 15003 | 16229 | 124 |  | 1 | Other (114) [355,692]<br>15108 [16,509]<br>15237 [15,435]<br>15235 [14,585]<br>15212 [13,301] |
| **SCHOOLCODE** | String | Código do Distrito Escolar | Número identificador de cada distrito escolar. Valores comuns incluem ‘47’, ‘27’ e ‘09’. O código 47 corresponde a 24 % das entradas. | 01 | 09 | 46 |  | 0 | Other (36) [228,000]<br>47 [117,977]<br>27 [19,685]<br>09 [19,227]<br>30 [17,635] |
| **SCHOOLDESC** | String | Nome do Distrito Escolar | Descrição textual do distrito escolar, como ‘Pittsburgh’, ‘North Allegheny’, etc. A maioria dos registros tem descrição ‘Pittsburgh’ (24 %). | Allegheny Valley | Woodland Hills | 46 |  | 0 | Other (36) [228,000]<br>Pittsburgh [117,977]<br>North Allegheny [19,685]<br>Woodland Hills [19,227]<br>Penn Hills Twp [17,635] |
| **MUNICODE** | Integer | Código Municipal | Número identificador de cada município. Os códigos mais frequentes são 934, 119 e 940, representando cerca de 3 % das entradas; a maioria dos registros pertence ao código 934. | 101 | 953 | 175 |  | 0 | Other (165) [369,793]<br>934 [17,635]<br>119 [12,648]<br>940 [12,003]<br>926 [10,359] |
| **MUNIDESC** | String | Descrição do Município | Nome do município associado ao código municipal, por exemplo ‘Penn Hills’, ‘19th Ward - PITTSBURGH’. A maioria corresponde a ‘Penn Hills’ (3 %). | 10th Ward -  McKEESPORT | Wilmerding   | 175 |  | 0 | Other (165) [369,793]<br>Penn Hills [17,635]<br>19th Ward - PITTSBURGH [12,648]<br>Ross [12,003]<br>Mt.Lebanon [10,359] |
| **RECORDDATE** | Date | Data de Registro | Data em que o registro foi criado ou atualizado. Os valores variam entre 2012‑08‑01 e 2028‑09‑28, com distribuição concentrada nos anos recentes (2013–2025). Aproximadamente 99 % dos registros têm data válida. | 0212-08-01 | 2028-09-28 | 3,821 |  | 1,262 | Other (3,811) [474,687]<br>(NULL) [1,262]<br>2012-10-26 [587]<br>2013-04-26 [552]<br>2012-01-11 [488] |
| **SALEDATE** | Date | Data da Venda | Data em que a venda ocorreu. A variação é semelhante à de RECORDDATE, com datas principais entre 2012 e 2025. Cerca de 99 % das entradas possuem uma data de venda válida. | 2012-01-01 | 2025-12-13 | 4,888 |  | 0 | Other (4,878) [475,391]<br>2012-10-26 [586]<br>2016-04-29 [584]<br>2013-04-26 [559]<br>2012-01-11 [490] |
| **PRICE** | Integer | Preço da Propriedade | Valor monetário da transação, mediano aproximadamente US$195 k, variando de $0 até $148 752 900. A maioria dos registros tem preços dentro de faixa típica do mercado imobiliário em Pennsylvania. | 0 | 148752900 | 37,872 |  | 3,020 | Other (37,862) [340,479]<br>1 [98,344]<br>0 [15,508]<br>10 [6,763]<br>150000 [3,140] |
| **DEEDBOOK** | String | Livro do Título | Identificador textual que indica o livro onde o título da propriedade está registrado. Valores como ‘TR18’, ‘0’, ‘TR13’ são frequentes, representando cerca de 98 % dos registros. |  14795 | `17274 | 5,814 |  | 570 | Other (5,800) [473,060]<br>TR18 [1,239]<br>0 [1,063]<br>TR13 [938]<br>00 [798] |
| **DEEDPAGE** | String | Página do Título | Número de página dentro do livro do título, com valores mais comuns entre 1 e 6. A maioria dos registros tem páginas numéricas menores que 10, representando cerca de 97 % das entradas. |  120 | W | 2,133 |  | 582 | Other (2,115) [465,578]<br>1 [5,174]<br>6 [1,485]<br>7 [1,112]<br>0 [1,002] |
| **SALECODE** | String | Código da Venda | Códigos que identificam o tipo ou condição da venda, como ‘3’, ‘0’, ‘H’. Os códigos mais frequentes são ‘3’ (20 %) e ‘0’ (19 %). | 0X | Z | 47 |  | 0 | 3 [97,932]<br>0 [91,484]<br>H [63,750]<br>Other (37) [54,274]<br>14 [42,089] |
| **SALEDESC** | String | Descrição da Venda | Texto explicativo do motivo ou natureza da venda, incluindo termos como ‘LOVE AND AFFECTION SALE’, ‘VALID SALE’, ‘MULTI‑PARCEL SALE’. A maioria corresponde ao tipo ‘LOVE AND AFFECTION SALE’ (20 %). | BANK/FINANCIAL INSTITUTION | VALID SALE | 27 |  | 0 | LOVE AND AFFECTION SALE [97,932]<br>VALID SALE [91,484]<br>MULTI-PARCEL SALE [63,750]<br>Other (17) [45,680]<br>TIME ON MARKET (INSUFF/EX… [42,089] |
| **INSTRTYP** | String | Tipo de Instrumento | Código abreviado que indica o tipo de instrumento legal usado na transação, como ‘DE’ (deed), ‘SW’ (special warranty). O código mais frequente é ‘DE’, representando 58 % das entradas. | AD | WD | 31 |  | 0 | DE [278,804]<br>SW [113,966]<br>GW [23,662]<br>QC [18,068]<br>SD [15,521] |
| **INSTRTYPDESC** | String | Descrição do Tipo de Instrumento | Descrição textual correspondente ao tipo de instrumento legal, por exemplo ‘DEED’, ‘SPECIAL WARRANTY’. A maioria corresponde a ‘DEED’ (58 %). | ADMIN DEED | WILL BOOK | 31 |  | 0 | DEED [278,804]<br>SPECIAL WARRANTY [113,966]<br>GENERAL WARRANTY [23,662]<br>QUIT CLAIM [18,068]<br>SHERIFF DEED [15,521] |

*Attribution: Generated by qsv v12.0.0 describegpt
Command line: qsv describegpt --all allegheny_county_property_sale_transactions.csv --language Portugese --fresh --output Portugese.md --addl-props {"reasoning_effort":"high"}
Prompt file: Default v7.0.0
Model: openai/gpt-oss-20b
LLM API URL: http://localhost:1234/v1
Language: 
Timestamp: 2025-12-25T12:26:43.212037+00:00

WARNING: Label and Description generated by an LLM and may contain inaccuracies. Verify before using!
*

## REASONING

We need to generate Portuguese labels and descriptions for each field in the dataset, using summary statistics and frequency distribution to understand context and meaning.

The dataset fields:

1. _id
2. PARID
3. FULL_ADDRESS
4. PROPERTYHOUSENUM
5. PROPERTYFRACTION
6. PROPERTYADDRESSDIR
7. PROPERTYADDRESSSTREET
8. PROPERTYADDRESSSUF
9. PROPERTYADDRESSUNITDESC
10. PROPERTYUNITNO
11. PROPERTYCITY
12. PROPERTYSTATE
13. PROPERTYZIP
14. SCHOOLCODE
15. SCHOOLDESC
16. MUNICODE
17. MUNIDESC
18. RECORDDATE
19. SALEDATE
20. PRICE
21. DEEDBOOK
22. DEEDPAGE
23. SALECODE
24. SALEDESC
25. INSTRTYP
26. INSTRTYPDESC

We need labels and descriptions in Portuguese.

We should consider what each field represents: unique key, parcel id, full address, house number, fraction (like 1/2), direction (N,S,E,W), street name, suffix (ST, AVE, etc.), unit descriptor (UNIT, REAR), unit number, city, state, zip code, school district code, school district description, municipal code, municipal description, record date, sale date, price, deed book, deed page, sale code, sale description, instrument type code, instrument type description.

Use summary statistics to infer types and distributions. Use frequency distribution for top values.

We need human-friendly labels in Portuguese. For example:

- _id: "Identificador Único" (Unique Identifier)
- PARID: "Número do Parcela" or "Código de Parcelamento" but maybe "PARID - Identificador da Parcela".
- FULL_ADDRESS: "Endereço Completo"
- PROPERTYHOUSENUM: "Número da Casa" or "Número do Prédio". In the frequency, top values include 0,112 etc. So it's house number. But sometimes 0 indicates no house number? So label: "Número de Residência".

- PROPERTYFRACTION: "Frção de Propriedade" (like '1/2' etc). Use description.

- PROPERTYADDRESSDIR: "Direção da Rua" (N,S,E,W).

- PROPERTYADDRESSSTREET: "Nome da Rua".

- PROPERTYADDRESSSUF: "Sufixo da Rua" (e.g., ST, AVE, RD).

- PROPERTYADDRESSUNITDESC: "Descrição do Unidade" or "Tipo de Unidade" like UNIT, REAR.

- PROPERTYUNITNO: "Número da Unidade".

- PROPERTYCITY: "Cidade".

- PROPERTYSTATE: "Estado".

- PROPERTYZIP: "CEP" (Postal Code). Use Portuguese abbreviation.

- SCHOOLCODE: "Código do Distrito Escolar".

- SCHOOLDESC: "Nome do Distrito Escolar".

- MUNICODE: "Código Municipal".

- MUNIDESC: "Descrição Municipal".

- RECORDDATE: "Data de Registro".

- SALEDATE: "Data da Venda".

- PRICE: "Preço da Propriedade" or "Valor de Venda".

- DEEDBOOK: "Livro do Título".

- DEEDPAGE: "Página do Título".

- SALECODE: "Código de Venda".

- SALEDESC: "Descrição da Venda".

- INSTRTYP: "Tipo de Instrumento" (like 'DE', 'SW', etc). In frequency, codes like DE, SW.

- INSTRTYPDESC: "Descrição do Tipo de Instrumento".

We need descriptions that are multiple sentences. Provide context: e.g., For FULL_ADDRESS, description: includes street number, name, direction, suffix, unit descriptor, city, state, ZIP code.

Also mention cardinality and uniqueness ratio: e.g., _id is unique for each record.

Let's use summary stats: for _id, sum 18690259640460, min 38703919, max 39183846, etc. But not necessary to include numeric values in description? Could be nice but not required.

We can also mention that PARID is string, likely a unique parcel identifier.

Use Portuguese.

Return JSON object with keys field names exactly as given (e.g., "_id", "PARID", ...). Provide label and description strings.

Make sure to output valid JSON. Use double quotes around keys and values.

Let's craft each:

1. _id: label: "Identificador Único". Description: "Campo que contém um identificador numérico único para cada registro no conjunto de dados, garantindo a distinção entre diferentes transações imobiliárias."

2. PARID: label: "Código do Parcela". Description: "Código alfanumérico que identifica unicamente o lote ou parcela na propriedade, usado em registros cadastrais e processos de venda." Mention cardinality: 302,633 unique values out of 479,928.

3. FULL_ADDRESS: label: "Endereço Completo". Description: "Campo composto que reúne todas as informações necessárias para localizar a propriedade: número da casa, nome da rua, direção (N/S/E/W), sufixo da rua, descrição da unidade, cidade, estado e CEP." Mention frequency top addresses.

4. PROPERTYHOUSENUM: label: "Número de Residência". Description: "Número atribuído à residência ou edifício na propriedade; valores comuns incluem 0 (sem número) e números como 112, 100, etc.; a maioria dos registros possui um número válido."

5. PROPERTYFRACTION: label: "Frção da Propriedade". Description: "Indica se a parcela representa uma fração parcial de uma propriedade, por exemplo '1/2', 'A', 'B'; muitos registros têm valor nulo indicando ausência de fração."

6. PROPERTYADDRESSDIR: label: "Direção da Rua". Description: "Código abreviado que indica a direção relativa à rua (Norte, Sul, Este, Oeste). A maioria dos registros não possui valor (NULL) e quando presente, os valores mais frequentes são 'N', 'S', 'E', 'W'."

7. PROPERTYADDRESSSTREET: label: "Nome da Rua". Description: "Texto que identifica o nome da rua onde a propriedade está localizada; nomes como 'WASHINGTON', '5TH', 'HIGHLAND' aparecem com maior frequência."

8. PROPERTYADDRESSSUF: label: "Sufixo da Rua". Description: "A abreviação que descreve o tipo de via, por exemplo 'ST', 'DR', 'AVE'; os sufixos mais comuns são 'ST', 'DR', 'AVE'."

9. PROPERTYADDRESSUNITDESC: label: "Descrição da Unidade". Description: "Indicador textual do tipo de unidade dentro do edifício (por exemplo, 'UNIT', 'REAR', 'APT', 'STE'). A maioria dos registros tem valor nulo; quando presente, os valores mais frequentes são 'UNIT'."

10. PROPERTYUNITNO: label: "Número da Unidade". Description: "Número interno ou identificador de unidade dentro do edifício; valores como '1', '2', '3' aparecem com maior frequência; a maioria dos registros tem valor nulo."

11. PROPERTYCITY: label: "Cidade". Description: "Nome da cidade onde a propriedade está situada, com destaque para cidades como 'PITTSBURGH', 'CORAOPOLIS', 'MC KEESPORT'; 53% das entradas correspondem a Pittsburgh."

12. PROPERTYSTATE: label: "Estado". Description: "Sigla do estado (ex.: PA) onde a propriedade está localizada; todos os registros pertencem ao estado de Pennsylvania (PA)."

13. PROPERTYZIP: label: "CEP". Description: "Código postal numérico que identifica a área da propriedade; os códigos mais frequentes incluem 15108, 15237, 15235; 74% das entradas têm valores fora dos top 10."

14. SCHOOLCODE: label: "Código do Distrito Escolar". Description: "Número identificador de cada distrito escolar; valores comuns incluem '47', '27', '09'; cerca de 24% correspondem ao código 47."

15. SCHOOLDESC: label: "Nome do Distrito Escolar". Description: "Descrição textual do distrito escolar, como 'Pittsburgh', 'North Allegheny', etc.; a maioria corresponde a Pittsburgh."

16. MUNICODE: label: "Código Municipal". Description: "Número identificador de cada município; os códigos mais frequentes são 934, 119, 940; cerca de 3% dos registros têm valores não listados (Other)."

17. MUNIDESC: label: "Descrição do Município". Description: "Nome do município associado ao código municipal, por exemplo 'Penn Hills', '19th Ward - PITTSBURGH'; a maioria corresponde a Penn Hills."

18. RECORDDATE: label: "Data de Registro". Description: "Data em que o registro foi criado ou atualizado; os valores variam entre 2012-08-01 e 2028-09-28, com distribuição concentrada nos anos recentes (2013–2025)."

19. SALEDATE: label: "Data da Venda". Description: "Data em que a venda ocorreu; variação semelhante à de RECORDDATE, com datas principais entre 2012 e 2025."

20. PRICE: label: "Preço da Propriedade". Description: "Valor monetário da transação, mediano aproximadamente US$195k, variando de $0 até $148.752.900; a maioria dos registros tem preços dentro de faixa típica de mercado imobiliário em Pennsylvania."

21. DEEDBOOK: label: "Livro do Título". Description: "Identificador textual que indica o livro onde o título da propriedade está registrado; valores como 'TR18', '0', 'TR13' são frequentes."

22. DEEDPAGE: label: "Página do Título". Description: "Número de página dentro do livro do título, com valores mais comuns entre 1 e 6; a maioria dos registros tem páginas numéricas menores que 10."

23. SALECODE: label: "Código da Venda". Description: "Códigos que identificam o tipo ou condição da venda, como '3', '0', 'H'; os códigos mais frequentes são '3' (20%) e '0' (19%)."

24. SALEDESC: label: "Descrição da Venda". Description: "Texto explicativo do motivo ou natureza da venda, incluindo termos como 'LOVE AND AFFECTION SALE', 'VALID SALE', 'MULTI-PARCEL SALE'; a maioria corresponde ao tipo 'LOVE AND AFFECTION SALE'."

25. INSTRTYP: label: "Tipo de Instrumento". Description: "Código abreviado que indica o tipo de instrumento legal usado na transação, como 'DE' (deed), 'SW' (special warranty). O código mais frequente é 'DE', representando 58% das entradas."

26. INSTRTYPDESC: label: "Descrição do Tipo de Instrumento". Description: "Descrição textual correspondente ao tipo de instrumento legal, por exemplo 'DEED', 'SPECIAL WARRANTY'; a maioria corresponde a 'DEED'."

We need to produce JSON.

Make sure each description is multiple sentences? It says can be multiple sentences. We will add 2-3 sentences.

Also maybe mention cardinality ratio for fields with unique values.

Let's incorporate some of the summary stats: For _id, uniqueness ratio 1 (all unique). For PARID, 302,633 unique out of 479,928 ~63%. For FULL_ADDRESS, 278,190 unique ~58%. For PROPERTYHOUSENUM, cardinality 10,002 unique out of 479,928 ~2%? Actually "Other" count 428653 indicates many unique values; cardinality maybe high.

But we can mention that many records have nulls etc. Use frequency distribution for top values.

Ok produce JSON.

We need to output exactly as:

{
  "_id": { ... },
  "PARID": { ... }, ...
}

Let's craft each label and description accordingly. Ensure proper punctuation, Portuguese words with accent marks.

Also maybe include numeric examples in description: e.g., "exemplo: '0431B00017000000'". But keep it concise.

We'll produce JSON string.


## TOKEN USAGE

TokenUsage { prompt: 7805, completion: 4513, total: 12318, elapsed: 75319 }
---
# Description
**Descrição do Dataset**

Este conjunto contém **479 928 registros** de transações imobiliárias em Pennsylvania, cada um identificado por um campo `_id` único. Os dados abrangem informações sobre a parcela (PARID), endereço completo, número da casa e rua, detalhes de venda (data e preço), instrumento legal usado na transferência, distrito escolar, município e estado. A maioria dos campos possui alta cardinalidade e valores únicos, mas alguns apresentam grande proporção de valores nulos ou categorias “Other”, indicando que apenas uma pequena fração das ocorrências está representada nos top 10 frequentes. O campo **PRICE** mostra distribuição altamente assimétrica: mediana em torno de US$195 k, desvio padrão de cerca de US$836 k e coeficiente de variação 0.47; além disso, existem outliers que ultrapassam US$148 M. Datas de registro e venda são quase completas, com apenas ~0,26 % de registros nulos em `RECORDDATE`.

---

### Características Notáveis

- **Cardinalidade**: 479 928 linhas; `_id` é único, enquanto `PARID` tem 302 633 valores “Other” (menos do que 10 % dos top 10).
- **Proporção de Valores Únicos**: aproximadamente **79 %** do conjunto total.
- **Preço**: mediana ≈ US$195 k; desvio padrão ≈ US$836 k; CV ≈ 0.47; distribuição enviesada positivamente com outliers > US$148 M.
- **Valores Nulos**:
  - `PROPERTYFRACTION` (97,6 % nulo)
  - `PROPERTYADDRESSDIR` (95,8 % nulo)
  - `PROPERTYADDRESSUNITDESC` (97,5 % nulo)
  - `PROPERTYUNITNO` (97,6 % nulo)
  - `RECORDDATE` (~0,26 % nulo)
- **Distribuição de ZIP Codes**: 114 “Other” cobre ~74 % dos registros.
- **PII/PHI**: Endereço completo, nome da rua, cidade e CEP são informações potencialmente identificáveis.
- **Outliers em Preço**: valores acima de US$1 M indicam transações muito grandes ou erros de registro.
- **Instrumentos Legais**: `DE` (deed) representa mais que 58 % das transações; outros tipos são significativamente menos frequentes.

---

*Attribution:* Generated by qsv v12.0.0 describegpt
Command line: qsv describegpt --all allegheny_county_property_sale_transactions.csv --language Portugese --fresh --output Portugese.md --addl-props {"reasoning_effort":"high"}
Prompt file: Default v7.0.0
Model: openai/gpt-oss-20b
LLM API URL: http://localhost:1234/v1
Language: 
Timestamp: 2025-12-25T12:27:13.758531+00:00

WARNING: Description generated by an LLM and may contain inaccuracies. Verify before using!



## REASONING

We need to produce a description of dataset as a whole, 1-8 sentences, then bullet list Notable Characteristics, in Portuguese. Use summary statistics and frequency distribution data to inform overall features: number of records, field types, uniqueness, cardinality, missing values etc.

Dataset has 479,928 rows. Fields include ID unique, PARID many duplicates (302,633 others). Full address most frequent top addresses but many unique. Property house number mostly 0. Fractions mostly null. Direction mostly null. Street names common. Zip codes distribution. Dates: record date and sale date with many missing. Price distribution: median ~195k, high skew, large outliers. Many duplicates? ID unique so no duplicate records. PII/PHI? Address includes street addresses; likely PII. PCI not present. Outliers in price up to 148M. Missing values for many fields: e.g., property fraction, direction, unit desc, etc. Notable characteristics: high uniqueness ratio overall (0.79). Price skewness positive. Data quality issues: missing values high, top10 only represent small portion of data. Many 'Other' categories.

Let's craft description:

Dataset contains 479,928 registros de transações imobiliárias em Pennsylvania. Cada registro possui identificador único (_id) e informações sobre parcela, endereço completo, dados da venda (data, preço), instrumentos legais, distrito escolar, município, etc. A maioria dos campos apresenta grande cardinalidade e valores únicos, mas alguns campos têm alta proporção de nulos ou categorias “Other”, indicando que apenas uma pequena fração das ocorrências está representada nos top 10 frequentes. O campo PRICE mostra distribuição altamente assimétrica com mediana em torno de US$195 k e valores extremos superiores a US$148 M, sugerindo outliers significativos. Datas de registro e venda são quase completas, mas ainda há ~0,26% de registros nulos para RECORDDATE.

Now bullet list:

- Cardinalidade: 479,928 linhas; ID único; PARID com 302,633 “Other”, etc.
- Proporção de valores únicos: ~79 % do conjunto total (uniqueness ratio).
- Preço: median 195k, stddev 836k, CV 0.47; skewness positive; outliers >148M.
- Campos nulos: PARID? not; property fraction 97.6% null; address dir 95.8% null; unit desc 97.5% null; etc.
- Frequência top10 apenas ~2–3% do total, "Other" categorias dominam.
- Distribuição de ZIP codes: 114 “Other” covers 74 %.
- Dados PII: endereço completo, nome da rua, cidade e CEP podem identificar indivíduos.
- Outliers: Price >1M; record dates missing for 1262 rows (0.26%).
- Instrument types: DE predominant.

We must not output per field stats or frequencies individually. Provide aggregated info. Also include attribution placeholder.

Let's produce Markdown with sections.


## TOKEN USAGE

TokenUsage { prompt: 9568, completion: 1186, total: 10754, elapsed: 30545 }
---
# Tags
```markdown
- imobiliaria_pa  
- vendas_propriedade_pa  
- dados_endereco_imobiliario  
- preco_imovel_pa  
- registro_deed_pa  
- codigo_cep_pa  
- cidades_pittsburgh  
- tipo_venda_imob  
- distrito_escolar_pa  
- historico_transacoes  

Generated by qsv v12.0.0 describegpt
Command line: qsv describegpt --all allegheny_county_property_sale_transactions.csv --language Portugese --fresh --output Portugese.md --addl-props {"reasoning_effort":"high"}
Prompt file: Default v7.0.0
Model: openai/gpt-oss-20b
LLM API URL: http://localhost:1234/v1
Language: 
Timestamp: 2025-12-25T12:27:54.234446+00:00

WARNING: Tags generated by an LLM and may contain inaccuracies. Verify before using!

```


## REASONING

We need to produce tags in Markdown format, descending order of importance, up to 10 unique tags, lowercase only and use _ to separate words. Must not use field names. Should be Portuguese language.

Based on dataset: real estate transactions in Pennsylvania (PA). Contains addresses, city, state, zip, property address components, sale date, price, deed info, school district etc.

We need tags relevant: "real_estate", "pa_property_sales", "pittsburgh_real_estate", "house_prices", "property_data", "deed_records", "zip_code_analysis", "school_districts", "sale_type", "transaction_dates". Need to keep Portuguese. So e.g., "imobiliaria_pa" or "propriedade_praia"? Wait, it's Pennsylvania (PA). Portuguese: "propriedades_pa" maybe.

We need 10 tags descending importance.

Important aspects:

- Real estate transactions
- Pennsylvania property sales
- Property address and location data
- Sale price and date
- Deed types
- School districts
- Zip codes
- City distribution
- Sale type categories

Let's craft tags in Portuguese: lowercase, underscores. e.g., "imobiliaria_pa", "vendas_propriedade_pa", "dados_endereco_imobiliario", "preco_imovel_pa", "registro_deed_pa", "codigo_cep_pa", "cidades_pittsburgh", "tipo_venda_imob", "distrito_escolar_pa".

Now ordering: most important maybe "imobiliaria_pa" (real estate in PA). Then "vendas_propriedade_pa". Then "dados_endereco_imobiliario". Then "preco_imovel_pa". Then "registro_deed_pa". Then "codigo_cep_pa". Then "cidades_pittsburgh". Then "tipo_venda_imob". Then "distrito_escolar_pa".

That's 9 tags. Need up to 10; we can add "historico_data" maybe "historico_transacoes". So final list: 10.

Let's produce in Markdown:

- imobiliaria_pa
- vendas_propriedade_pa
- dados_endereco_imobiliario
- preco_imovel_pa
- registro_deed_pa
- codigo_cep_pa
- cidades_pittsburgh
- tipo_venda_imob
- distrito_escolar_pa
- historico_transacoes

Add attribution placeholder.

Let's output:

```markdown
- imobiliaria_pa  
- vendas_propriedade_pa  
- dados_endereco_imobiliario  
- preco_imovel_pa  
- registro_deed_pa  
- codigo_cep_pa  
- cidades_pittsburgh  
- tipo_venda_imob  
- distrito_escolar_pa  
- historico_transacoes  

{GENERATED_BY_SIGNATURE}
```

Check lowercase and underscores. Yes.

Now produce final answer.
## TOKEN USAGE

TokenUsage { prompt: 9537, completion: 666, total: 10203, elapsed: 21929 }
---
