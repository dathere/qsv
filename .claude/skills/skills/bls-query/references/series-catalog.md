# BLS Series ID Catalog

A mapping of common economic topics to their BLS series IDs. Use this to resolve natural language queries.

## Consumer Price Index (CPI)

| Series ID | Description | Seasonal Adj |
|---|---|---|
| CUUR0000SA0 | CPI-U: All Items, U.S. City Average | No |
| CUSR0000SA0 | CPI-U: All Items, U.S. City Average | Yes |
| CUUR0000SA0L1E | CPI-U: All Items Less Food and Energy (Core CPI) | No |
| CUSR0000SA0L1E | CPI-U: All Items Less Food and Energy (Core CPI) | Yes |
| CUUR0000SA0E | CPI-U: All Items Less Energy | No |
| CUUR0000SAF1 | CPI-U: Food at Home | No |
| CUUR0000SAF11 | CPI-U: Food at Home - Cereals and Bakery Products | No |
| CUUR0000SAH1 | CPI-U: Shelter | No |
| CUUR0000SAM | CPI-U: Medical Care | No |
| CUUR0000SETB01 | CPI-U: Gasoline (All Types) | No |
| CUUR0000SAF | CPI-U: Food and Beverages | No |
| CUUR0000SEMD01 | CPI-U: Physicians' Services | No |
| CUUR0000SEMC01 | CPI-U: Professional Medical Services | No |
| CUUR0000SEMF01 | CPI-U: Hospital Services | No |

### CPI by Region
| Series ID | Description |
|---|---|
| CUUR0100SA0 | CPI-U: Northeast |
| CUUR0200SA0 | CPI-U: Midwest |
| CUUR0300SA0 | CPI-U: South |
| CUUR0400SA0 | CPI-U: West |
| CUURS49ASA0 | CPI-U: Los Angeles-Long Beach-Anaheim |
| CUURS49BSA0 | CPI-U: San Francisco-Oakland-Hayward |
| CUURS12ASA0 | CPI-U: Atlanta-Sandy Springs-Roswell |

### Chained CPI
| Series ID | Description |
|---|---|
| SUUR0000SA0 | Chained CPI-U: All Items |

## Employment Situation

### Unemployment
| Series ID | Description | Seasonal Adj |
|---|---|---|
| LNS14000000 | Unemployment Rate (16 years and over) | Yes |
| LNU04000000 | Unemployment Rate (16 years and over) | No |
| LNS14000001 | Unemployment Rate - Men | Yes |
| LNS14000002 | Unemployment Rate - Women | Yes |
| LNS14000003 | Unemployment Rate - White | Yes |
| LNS14000006 | Unemployment Rate - Black or African American | Yes |
| LNS14000009 | Unemployment Rate - Hispanic or Latino | Yes |
| LNS13000000 | Unemployment Level | Yes |
| LNS14000012 | Unemployment Rate - 16-19 years | Yes |
| LNS14000024 | Unemployment Rate - 20 years and over, Men | Yes |
| LNS14000025 | Unemployment Rate - 20 years and over, Women | Yes |

### Labor Force
| Series ID | Description | Seasonal Adj |
|---|---|---|
| LNS11000000 | Civilian Labor Force Level | Yes |
| LNS11300000 | Labor Force Participation Rate | Yes |
| LNS12000000 | Employment Level | Yes |
| LNS12300000 | Employment-Population Ratio | Yes |

## Current Employment Statistics (CES) - Nonfarm Payrolls

| Series ID | Description | Seasonal Adj |
|---|---|---|
| CES0000000001 | Total Nonfarm Employment | Yes |
| CES0500000001 | Total Private Employment | Yes |
| CES1000000001 | Mining and Logging Employment | Yes |
| CES2000000001 | Construction Employment | Yes |
| CES3000000001 | Manufacturing Employment | Yes |
| CES4000000001 | Trade, Transportation, and Utilities Employment | Yes |
| CES5000000001 | Information Employment | Yes |
| CES5500000001 | Financial Activities Employment | Yes |
| CES6000000001 | Professional and Business Services Employment | Yes |
| CES6500000001 | Education and Health Services Employment | Yes |
| CES7000000001 | Leisure and Hospitality Employment | Yes |
| CES9000000001 | Government Employment | Yes |

### Wages and Earnings
| Series ID | Description | Seasonal Adj |
|---|---|---|
| CES0500000003 | Average Hourly Earnings, Total Private | Yes |
| CES0500000008 | Average Hourly Earnings, Total Private (Production) | Yes |
| CES0500000011 | Average Weekly Earnings, Total Private | Yes |

### Hours
| Series ID | Description | Seasonal Adj |
|---|---|---|
| CES0500000002 | Average Weekly Hours, Total Private | Yes |
| CES0500000007 | Average Weekly Hours, Total Private (Production) | Yes |

## Job Openings and Labor Turnover Survey (JOLTS)

| Series ID | Description | Seasonal Adj |
|---|---|---|
| JTS000000000000000JOL | Total Nonfarm Job Openings | Yes |
| JTS000000000000000HIL | Total Nonfarm Hires | Yes |
| JTS000000000000000TSL | Total Nonfarm Separations | Yes |
| JTS000000000000000QUL | Total Nonfarm Quits | Yes |
| JTS000000000000000LDL | Total Nonfarm Layoffs and Discharges | Yes |
| JTS000000000000000JOR | Total Nonfarm Job Openings Rate | Yes |
| JTS000000000000000HIR | Total Nonfarm Hires Rate | Yes |
| JTS000000000000000QUR | Total Nonfarm Quits Rate | Yes |

## Producer Price Index (PPI)

| Series ID | Description | Seasonal Adj |
|---|---|---|
| WPSFD4 | PPI: Finished Goods | No |
| WPSFD49104 | PPI: Finished Goods Less Food and Energy | No |
| WPSID61 | PPI: Intermediate Materials | No |
| WPUFD49116 | PPI: Final Demand | No |
| PCU------ | PPI by Industry (varies) | Varies |

## Employment Cost Index (ECI)

| Series ID | Description |
|---|---|
| CIU1010000000000A | ECI: Total Compensation, All Civilian |
| CIU2010000000000A | ECI: Wages and Salaries, All Civilian |
| CIU3010000000000A | ECI: Benefit Costs, All Civilian |

## Productivity

| Series ID | Description |
|---|---|
| PRS85006092 | Nonfarm Business: Labor Productivity (Output Per Hour) |
| PRS85006112 | Nonfarm Business: Unit Labor Costs |
| PRS85006152 | Nonfarm Business: Real Compensation Per Hour |

## Import/Export Prices

| Series ID | Description |
|---|---|
| EIUIR | Import Price Index: All Imports |
| EIUIR100 | Import Price Index: All Imports Excluding Petroleum |
| EIUIQ | Export Price Index: All Exports |

## Consumer Expenditure Survey

Survey abbreviation: CX. Series IDs vary widely. Use `get_popular_series` with survey `CX` to discover available series.

## Local Area Unemployment (LAUS)

Format: `LAUST${STATE_FIPS}0000000000003` for state unemployment rates.

| Series ID | Description |
|---|---|
| LASST060000000000003 | California Unemployment Rate |
| LASST480000000000003 | Texas Unemployment Rate |
| LASST360000000000003 | New York Unemployment Rate |
| LASST120000000000003 | Florida Unemployment Rate |

For metro areas, use `get_popular_series` with survey `LA`.

## Survey Abbreviation Quick Reference

| Abbr | Survey | Common Use |
|---|---|---|
| CU | CPI - All Urban Consumers | Inflation, cost of living |
| CW | CPI - Urban Wage Earners | Alternative CPI measure |
| CE | Current Employment Statistics | Jobs, payrolls, wages |
| LN | Labor Force Statistics (CPS) | Unemployment rate, labor force |
| LA | Local Area Unemployment | State/metro unemployment |
| JT | JOLTS | Job openings, quits, hires |
| WP | PPI - Commodities | Producer prices |
| PC | PPI - Industry Data | Producer prices by industry |
| EI | Import/Export Prices | Trade prices |
| CI/EC | Employment Cost Index | Compensation costs |
| PR | Productivity and Costs | Labor productivity |
| OE | Occupational Employment | Wages by occupation |
| CX | Consumer Expenditure | Household spending |
| IP | Industry Productivity | Output per hour by industry |
| BD | Business Employment Dynamics | Gross job gains/losses |
| TU | American Time Use | How people spend their time |
