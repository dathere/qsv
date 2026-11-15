# Validate command

Validates CSV against [JSON Schema](https://json-schema.org/), or just against [RFC 4180](https://www.loc.gov/preservation/digital/formats/fdd/fdd000323.shtml).

Uses [Stranger6667/jsonschema-rs](https://github.com/Stranger6667/jsonschema-rs) as validation engine, and supports JSON Schema Draft 2020-12.

## Usage Examples

### Simple Validation

people_schema.json
```
{
    "$id": "https://example.com/person.schema.json",
    "$schema": "https://json-schema.org/draft/2020-12/schema",
    "title": "Person",
    "type": "object",
    "properties": {
      "firstName": {
        "type": "string",
        "description": "The person's first name."
      },
      "lastName": {
        "type": "string",
        "description": "The person's last name.",
        "minLength": 2
      },
      "age": {
        "description": "Age in years which must be equal to or greater than 18.",
        "type": "integer",
        "minimum": 18
      }
    }
}
```

people.csv
```
firstName,lastName,age
John,Doe,21
Mickey,Mouse,10
Little,A,16
```

Example run
```
$ qsv validate people.csv people_schema.json  --quiet
$ ls people.csv*
people.csv  people.csv.invalid  people.csv.valid
$ cat people.csv.invalid 
firstName,lastName,age
Mickey,Mouse,10
Little,A,16
$ cat people.csv.valid
firstName,lastName,age
John,Doe,21
```

### Validation of the [Adur Public Toilets Dataset](https://datasets.opendata.esd.org.uk/details?submissionId=45)

adur-public-toilets.csv, with bad data manually inserted into rows 1 and 3
```
ExtractDate,OrganisationURI,OrganisationLabel,ServiceTypeURI,ServiceTypeLabel,LocationText,CoordinateReferenceSystem,GeoX,GeoY,GeoPointLicensingURL,Category,AccessibleCategory,RADARKeyNeeded,BabyChange,FamilyToilet,ChangingPlace,AutomaticPublicConvenience,FullTimeStaffing,PartOfCommunityScheme,CommunitySchemeName,ChargeAmount,InfoURL,OpeningHours,ManagedBy,ReportEmail,ReportTel,Notes,UPRN,Postcode,StreetAddress,GeoAreaURI,GeoAreaLabel
 ,http://opendatacommunities.org/id/district-council/adur,,http://id.esd.org.uk/service/579,Public toilets,BEACH GREEN PUBLIC CONVENIENCES BRIGHTON ROAD LANCING,OSGB36,518072,103649,http://www.ordnancesurvey.co.uk/business-and-government/help-and-support/public-sector/guidance/derived-data-exemptions.html,Female and male,Unisex,Yes,No,No,No,No,No,No,,,http://www.adur-worthing.gov.uk/streets-and-travel/public-toilets/,"S = 09:00 - 21:00 W = 09:00 - 17:00 ",ADC,surveyors@adur-worthing.gov.uk,01903 221471,,60001449,,BEACH GREEN PUBLIC CONVENIENCES BRIGHTON ROAD LANCING,,
07/07/2014 00:00,http://opendatacommunities.org/id/district-council/adur,Adur,http://id.esd.org.uk/service/579,Public toilets,PUBLIC CONVENIENCES MONKS RECREATION GROUND CRABTREE LANE LANCING,OSGB36,518225,104730,http://www.ordnancesurvey.co.uk/business-and-government/help-and-support/public-sector/guidance/derived-data-exemptions.html,Female and male,None,Yes,No,No,No,No,No,No,,,http://www.adur-worthing.gov.uk/streets-and-travel/public-toilets/,"S = 09:00 - 15:00 W = 09:00 - 15:00",ADC,surveyors@adur-worthing.gov.uk,01903 221471,,60002210,,PUBLIC CONVENIENCES MONKS RECREATION GROUND CRABTREE LANE LANCING,,
2014-07-07 00:00,http://opendatacommunities.org/id/district-council/adur,Adur,http://id.esd.org.uk/service/579,Public toilets,PUBLIC CONVENIENCES SHOPSDAM ROAD LANCING,OSGB3,518915,103795,http://www.ordnancesurvey.co.uk/business-and-government/help-and-support/public-sector/guidance/derived-data-exemptions.html,Mens,Unisex,Yes,No,No,No,No,No,No,,,http://www.adur-worthing.gov.uk/streets-and-travel/public-toilets/,"S = 09:00 - 21:00 W = 09:00 - 17:00",ADC,surveyors@adur-worthing.gov.uk,01903 221471,,60007428,,PUBLIC CONVENIENCES SHOPSDAM ROAD LANCING,,
07/07/2014 00:00,http://opendatacommunities.org/id/district-council/adur,Adur,http://id.esd.org.uk/service/579,Public toilets,PUBLIC CONVENIENCES YEW TREE CLOSE LANCING,OSGB36,518222,104168,http://www.ordnancesurvey.co.uk/business-and-government/help-and-support/public-sector/guidance/derived-data-exemptions.html,Female and male,Unisex,Yes,No,No,No,No,No,No,,,http://www.adur-worthing.gov.uk/streets-and-travel/public-toilets/,"S = 09:00 - 21:00 W = 09:00 - 17:00",ADC,surveyors@adur-worthing.gov.uk,01903 221471,,60008859,,PUBLIC CONVENIENCES YEW TREE CLOSE LANCING,,
07/07/2014 00:00,http://opendatacommunities.org/id/district-council/adur,Adur,http://id.esd.org.uk/service/579,Public toilets,PUBLIC CONVENIENCES BEACH GREEN SHOREHAM-BY-SEA,OSGB36,521299,104515,http://www.ordnancesurvey.co.uk/business-and-government/help-and-support/public-sector/guidance/derived-data-exemptions.html,Female and male,Unisex,Yes,No,No,No,No,No,No,,,http://www.adur-worthing.gov.uk/streets-and-travel/public-toilets/,"S = 09:00 - 21:00 W = 09:00 - 17:00",ADC,surveyors@adur-worthing.gov.uk,01903 221471,,60009402,,PUBLIC CONVENIENCES BEACH GREEN SHOREHAM-BY-SEA,,
07/07/2014 00:00,http://opendatacommunities.org/id/district-council/adur,Adur,http://id.esd.org.uk/service/579,Public toilets,PUBLIC CONVENIENCES ADUR RECREATION GROUND BRIGHTON ROAD SHOREHAM-BY-SEA,OSGB36,521048,104977,http://www.ordnancesurvey.co.uk/business-and-government/help-and-support/public-sector/guidance/derived-data-exemptions.html,Female and male,Unisex,Yes,No,No,No,No,No,No,,,http://www.adur-worthing.gov.uk/streets-and-travel/public-toilets/,"S = 08:00 - 21:00 W = 08:00 - 17:00",ADC,surveyors@adur-worthing.gov.uk,01903 221471,,60009666,,PUBLIC CONVENIENCES ADUR RECREATION GROUND BRIGHTON ROAD SHOREHAM-BY-SEA,,
07/07/2014 00:00,http://opendatacommunities.org/id/district-council/adur,Adur,http://id.esd.org.uk/service/579,Public toilets,PUBLIC CONVENIENCES FORTHAVEN SHOREHAM-BY-SEA,OSGB36,523294,104588,http://www.ordnancesurvey.co.uk/business-and-government/help-and-support/public-sector/guidance/derived-data-exemptions.html,Female and male,Unisex,Yes,No,No,No,No,No,No,,,http://www.adur-worthing.gov.uk/streets-and-travel/public-toilets/,"S = 09:00 - 21:00 W = 09:00 - 17:00",ADC,surveyors@adur-worthing.gov.uk,01903 221471,,60011970,,PUBLIC CONVENIENCES FORTHAVEN SHOREHAM-BY-SEA,,
07/07/2014 00:00,http://opendatacommunities.org/id/district-council/adur,Adur,http://id.esd.org.uk/service/579,Public toilets,PUBLIC CONVENIENCES MIDDLE STREET SHOREHAM-BY-SEA,OSGB36,521515,105083,http://www.ordnancesurvey.co.uk/business-and-government/help-and-support/public-sector/guidance/derived-data-exemptions.html,Female and male,Unisex,Yes,No,No,No,No,No,No,,,http://www.adur-worthing.gov.uk/streets-and-travel/public-toilets/,"S = 09:00 - 21:00 W = 09:00 - 17:00",ADC,surveyors@adur-worthing.gov.uk,01903 221471,,60014163,,PUBLIC CONVENIENCES MIDDLE STREET SHOREHAM-BY-SEA,,
07/07/2014 00:00,http://opendatacommunities.org/id/district-council/adur,Adur,http://id.esd.org.uk/service/579,Public toilets,PUBLIC CONVENIENCES CEMETERY MILL LANE SHOREHAM-BY-SEA,OSGB36,521440,105725,http://www.ordnancesurvey.co.uk/business-and-government/help-and-support/public-sector/guidance/derived-data-exemptions.html,Female and male,None,No,No,No,No,No,No,No,,,http://www.adur-worthing.gov.uk/streets-and-travel/public-toilets/,,ADC,surveyors@adur-worthing.gov.uk,01903 221471,Grounds staff only not public,60014340,,PUBLIC CONVENIENCES CEMETERY MILL LANE SHOREHAM-BY-SEA,,
07/07/2014 00:00,http://opendatacommunities.org/id/district-council/adur,Adur,http://id.esd.org.uk/service/579,Public toilets,PUBLIC CONVENIENCES SOUTH PAVILION BUCKINGHAM PARK UPPER SHOREHAM ROAD SHOREHAM-BY-SEA,OSGB36,522118,105939,http://www.ordnancesurvey.co.uk/business-and-government/help-and-support/public-sector/guidance/derived-data-exemptions.html,Female and male,None,No,No,No,No,No,No,No,,,http://www.adur-worthing.gov.uk/streets-and-travel/public-toilets/,"S = 09:00 - 21:00 W = 09:00 - 17:00",ADC,surveyors@adur-worthing.gov.uk,01903 221471,,60017866,,PUBLIC CONVENIENCES SOUTH PAVILION BUCKINGHAM PARK UPPER SHOREHAM ROAD SHOREHAM-BY-SEA,,
07/07/2014 00:00,http://opendatacommunities.org/id/district-council/adur,Adur,http://id.esd.org.uk/service/579,Public toilets,PUBLIC CONVENIENCE SOUTHWICK STREET SOUTHWICK,OSGB36,524401,105405,http://www.ordnancesurvey.co.uk/business-and-government/help-and-support/public-sector/guidance/derived-data-exemptions.html,Female and male,Unisex,Yes,No,No,No,No,No,No,,,http://www.adur-worthing.gov.uk/streets-and-travel/public-toilets/,"S = 08:00 - 21:00 W = 08:00 - 17:00",ADC,surveyors@adur-worthing.gov.uk,01903 221471,,60026354,,PUBLIC CONVENIENCE SOUTHWICK STREET SOUTHWICK,,
07/07/2014 00:00,http://opendatacommunities.org/id/district-council/adur,Adur,http://id.esd.org.uk/service/579,Public toilets,WEST BEACH PUBLIC CONVENIENCES WEST BEACH ROAD LANCING,OSGB36,520354,104246,http://www.ordnancesurvey.co.uk/business-and-government/help-and-support/public-sector/guidance/derived-data-exemptions.html,Female and male,Unisex,Yes,No,No,No,No,No,No,,,http://www.adur-worthing.gov.uk/streets-and-travel/public-toilets/,"S = 09:00 - 21:00 W = 09:00 - 17:00",,surveyors@adur-worthing.gov.uk,01903 221471,,60028994,,WEST BEACH PUBLIC CONVENIENCES WEST BEACH ROAD LANCING,,
07/07/2014 00:00,http://opendatacommunities.org/id/district-council/adur,Adur,http://id.esd.org.uk/service/579,Public toilets,BEACH TOILETS BASIN ROAD SOUTH SOUTHWICK,OSGB36,524375,104753,http://www.ordnancesurvey.co.uk/business-and-government/help-and-support/public-sector/guidance/derived-data-exemptions.html,Female and male,Unisex,Yes,No,No,No,No,No,No,,,http://www.adur-worthing.gov.uk/streets-and-travel/public-toilets/,"S = 09:00 - 21:00 W = 09:00 - 17:00",ADC,surveyors@adur-worthing.gov.uk,01903 221471,,60029181,,BEACH TOILETS BASIN ROAD SOUTH SOUTHWICK,,
07/07/2014 00:00,http://opendatacommunities.org/id/district-council/adur,Adur,http://id.esd.org.uk/service/579,Public toilets,PUBLIC CONVENIENCE NORTH PAVILION BUCKINGHAM PARK UPPER SHOREHAM ROAD SHOREHAM-BY-SEA,OSGB36,522007,106062,http://www.ordnancesurvey.co.uk/business-and-government/help-and-support/public-sector/guidance/derived-data-exemptions.html,Female and male,None,No,No,No,No,No,No,No,,,http://www.adur-worthing.gov.uk/streets-and-travel/public-toilets/,,ADC,surveyors@adur-worthing.gov.uk,01903 221471,Grounds staff only not public,60032527,,PUBLIC CONVENIENCE NORTH PAVILION BUCKINGHAM PARK UPPER SHOREHAM ROAD SHOREHAM-BY-SEA,,
07/07/2014 00:00,http://opendatacommunities.org/id/district-council/adur,Adur,http://id.esd.org.uk/service/579,Public toilets,PUBLIC CONVENIENCES CIVIC CENTRE HAM ROAD SHOREHAM-BY-SEA,OSGB36,522083,105168,http://www.ordnancesurvey.co.uk/business-and-government/help-and-support/public-sector/guidance/derived-data-exemptions.html,Female and male,Unisex,Yes,No,No,No,No,No,No,,,http://www.adur-worthing.gov.uk/streets-and-travel/public-toilets/,09.00 - 17.00,ADC,surveyors@adur-worthing.gov.uk,01903 221471,,60034215,,PUBLIC CONVENIENCES CIVIC CENTRE HAM ROAD SHOREHAM-BY-SEA,,

```

Manually crafted JSON Schema [public_toilets.schema.json](https://gist.githubusercontent.com/mhuang74/076c0cfdfaf14b05638569839744565a/raw/8f693f353201d1ab2c2255ed987b0b58a23dd4b9/public_toilets.schema.json) JSON Table Schema format, with a few validation rules ported as illustsration:
* 6 mandatory fields (ExtractDate, OrganisationLabel, ServiceTypeLabel, LocationText, GeoX, GeoY)
* ExtractDate: date format
* Category: allowed values
* CoordinateReferenceSystem: allowed values

example run:

```
$ QSV_LOG_LEVEL=debug qsvlite validate adur-public-toilets.csv https://gist.githubusercontent.com/mhuang74/076c0cfdfaf14b05638569839744565a/raw/8f693f353201d1ab2c2255ed987b0b58a23dd4b9/public_toilets.schema.json --quiet
2 out of 15 records invalid.

$ wc -l adur-public-toilets.csv*
   16 adur-public-toilets.csv
    3 adur-public-toilets.csv.invalid
   14 adur-public-toilets.csv.valid
    5 adur-public-toilets.csv.validation-errors.tsv
```

adur-public-toilets.csv.validation-errors.tsv is a tab-separated report with a `row_number`, `field`, and
`error` column. Here it shows 2 rows failed validation, with 4 rules violated (the example below is
illustrative):
```
row_number	field	error
1	ExtractDate	null is not of type "string"
1	OrganisationLabel	null is not of type "string"
3	CoordinateReferenceSystem	"OSGB3" does not match "(WGS84|OSGB36)"
3	Category	"Mens" does not match "(Female|Male|Female and Male|Unisex|Male urinal|Children only|None)"
```

### SARIF output for GitHub code scanning

New in qsv 29.0.0 ([#5031](https://github.com/dathere/qsv/pull/5031)). `--sarif <file>` also writes the JSON Schema errors as a [SARIF 2.1.0](https://www.oasis-open.org/standard/sarif-v2-1-0/) log. SARIF is the format static analysis tools use, so GitHub code scanning and other SARIF viewers can show invalid rows as alerts on the CSV file, at the line where each row starts.

The files below are in [`resources/examples/validate-sarif`](../resources/examples/validate-sarif). `sales.csv` has one error of each kind:

```
order_id,order_date,sku,region,quantity,unit_price,customer_email,notes
1001,2026-10-01,ABC-1234,North,5,$19.99,ana@example.com,
1002,2026-10-02,ABC-1235,South,12,$4.50,ben@example.com,"Gift wrap,
deliver after 5pm"
1003,2026-13-02,ABC-1236,East,3,$7.25,cara@example.com,
1004,2026-10-04,abc-77,West,1,$2.00,dev@example.com,
...
```

example run:

```
$ qsv validate --split-ragged sales.csv sales.schema.json --sarif sales.csv.sarif
Writing invalid/valid/error files...
11 out of 14 records invalid.
```

The usual `sales.csv.validation-errors.tsv` report is still written. The SARIF log holds the same 11 errors, one result each. This is the result for row 3, whose `order_date` isn't a valid date:

```json
{
  "ruleId": "format",
  "ruleIndex": 3,
  "level": "error",
  "message": {
    "text": "Row 3, column \"order_date\": \"2026-13-02\" is not a \"date\""
  },
  "locations": [
    {
      "physicalLocation": {
        "artifactLocation": {
          "uri": "resources/examples/validate-sarif/sales.csv",
          "index": 0
        },
        "region": {
          "startLine": 5
        }
      }
    }
  ]
}
```

* `ruleId` is the JSON Schema keyword that failed (`format`, `pattern`, `enum`, `minimum`, …), plus `dynamicEnum`, `uniqueCombinedWith` and `ragged-row` (with `--split-ragged`).
* `startLine` is the file line where the row starts, not the row number. Row 2's quoted `notes` field spans two lines, so row 3 starts on line 5.
* `uri` is relative to the root of the git checkout the CSV is in (here, the qsv repo), so results point at the right file wherever qsv runs. Outside a checkout, it is relative to the current directory.
* The log is written even when every row is valid (with no results), so a CI run clears earlier alerts.
* A run that stops with an error writes no SARIF file, and removes one left by an earlier run. That happens on a CSV read error, or on a row with the wrong number of fields when `--split-ragged` isn't set, which is why the example uses it. `--fail-fast` is different: it stops after the first batch with an invalid row, but still writes the results for the rows it validated.
* Output stops at 25,000 results by default, GitHub code scanning's limit per run. `--sarif-max-results <n>` sets another cap, and 0 means no limit.
* `--sarif` works only in JSON Schema mode with a file input. RFC 4180 mode and stdin reject it.

[`github-workflow.yml`](../resources/examples/validate-sarif/github-workflow.yml) in the same directory is a complete GitHub Actions workflow. It validates the CSV, uploads the log with `github/codeql-action/upload-sarif` (the job needs `security-events: write`), and then fails the job if any row was invalid. Run unchanged on a public repository ([jqnatividad/qsv-sarif-test](https://github.com/jqnatividad/qsv-sarif-test)) with qsv 29.0.0, it raised one code scanning alert per error at the right line of `sales.csv`. On a pull request that added an invalid row ([PR #1](https://github.com/jqnatividad/qsv-sarif-test/pull/1)), it flagged only that new error, with a review comment on its line. Code scanning on a private repository needs GitHub Advanced Security.

See the [Validation & Schema wiki page](https://github.com/dathere/qsv/wiki/Validation-and-Schema) for more, including a walk-through of this example.


