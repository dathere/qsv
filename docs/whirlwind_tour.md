### A whirlwind tour

> ℹ️ **NOTE:** This tour is primarily targeted to Linux and macOS users. Though qsv works on Windows, the tour assumes basic knowledge of command-line piping and redirection, and uses other command-line tools (curl, tee, head, etc.) that are not installed by default on Windows.  
For a more detailed, interactive tour (which also happens to be Windows-friendly)
see [100.dathere.com](https://100.dathere.com).

Let's say you're playing with some data from the
[Data Science Toolkit](https://github.com/petewarden/dstkdata), which contains
several CSV files. Maybe you're interested in the population counts of each
city in the world. So grab the 124MB, 2.7M row CSV file and start examining it:

```
# there are no headers in the original repo, so let's download a prepared CSV with headers
$ curl -LO https://raw.githubusercontent.com/wiki/dathere/qsv/files/wcp.zip
$ unzip wcp.zip
$ qsv headers wcp.csv
1   Country
2   City
3   AccentCity
4   Region
5   Population
6   Latitude
7   Longitude
```

The next thing you might want to do is get an overview of the kind of data that
appears in each column. The `stats` command will do this for you:

```
$ qsv stats wcp.csv | qsv table
field       type     is_ascii  sum            min           max         range     sort_order  sortiness  min_length  max_length  sum_length  avg_length  stddev_length  variance_length  cv_length  mean        sem        geometric_mean  harmonic_mean  stddev       variance          cv        nullcount  n_negative  n_zero  n_positive  max_precision  sparsity
Country     String   true                     ad            zw                    Unsorted    1          2           2           5398708     2           0              0                0                                                                                                         0                                                         0
City        String   false                     al lusayli   ??ykkvibaer           Unsorted    0.8056     1           87          26071762    9.6585      4.0135         16.1081          0.4155                                                                                                    0                                                         0
AccentCity  String   false                     Al Lusayli   ???zler            Unsorted    0.7503     1           87          26719888    9.8986      4.1308         17.0633          0.4173                                                                                                    0                                                         0
Region      String   true                     00            Z4                    Unsorted    0.3513     0           2           5303100     1.9646      0.0025         0                0.0013                                                                                                    4                                                         0
Population  Integer            2290536125     7             31480498    31480491  Unsorted    0.0036                                                                                                48730.6639  1422.5474  10888.1976      2663.625       308414.0419  95119221210.8852  632.8952  2652350    0           0       47004                      0.9826
Latitude    Float              76585211.1978  -54.9333333   82.483333   137.4167   Unsorted    0.0692                                                                                                28.3717     0.0134                                    21.9384      481.2922          77.3249   0          330964      81      2368309     9              0
Longitude   Float              75976506.6643  -179.9833333  180.0       359.9833   Unsorted    0.0688                                                                                                28.1462     0.038                                     62.4729      3902.8581         221.9586  0          633589      159     2065606     9              0
```

Wow! That was fast! It took just 0.72 seconds to compile all that.[^1] One reason for qsv's speed
is that ***it mainly works in "streaming" mode*** - computing statistics as it "streams"
the CSV file line by line. This also means it can gather statistics on arbitrarily large files,
as it does not have to load the entire file into memory.[^2]

But can we get more summary statistics? What's the median, the antimodes, the percentiles, the
Median Absolute Deviation (MAD), the cardinality of the data?  No problem. That's why `qsv stats`
has an `--everything` option to compute these more "expensive" stats. Expensive - as these
extended statistics can only be computed at the cost of loading the entire file into memory.

```
$ qsv stats wcp.csv --everything | qsv table
field       type     is_ascii  sum            min           max         range     sort_order  sortiness  min_length  max_length  sum_length  avg_length  stddev_length  variance_length  cv_length  mean        sem        geometric_mean  harmonic_mean  stddev       variance          cv        nullcount  n_negative  n_zero  n_positive  max_precision  sparsity  mad      lower_outer_fence  lower_inner_fence  q1       q2_median  q3       iqr      upper_inner_fence  upper_outer_fence  skewness  cardinality  uniqueness_ratio  mode         mode_count  mode_occurrences  antimode                                                                       antimode_count  antimode_occurrences  percentiles
Country     String   true                     ad            zw                    Unsorted    1          2           2           5398708     2           0              0                0                                                                                                         0                                                         0                                                                                                                                              231          0.0001            ru           1           176934            cc|nf|pn|tf|tk                                                                 5               1                     
City        String   false                     al lusayli   ??ykkvibaer           Unsorted    0.8056     1           87          26071762    9.6585      4.0135         16.1081          0.4155                                                                                                    0                                                         0                                                                                                                                              2008182      0.7439            san jose     1           313               *PREVIEW: al lusayli|amiri-ye `olya|bab el ahmar|baqeri `olya|chahar tang...   1741957         1                     
AccentCity  String   false                     Al Lusayli   ???zler            Unsorted    0.7503     1           87          26719888    9.8986      4.1308         17.0633          0.4173                                                                                                    0                                                         0                                                                                                                                              2025934      0.7505            San Antonio  1           307               *PREVIEW: Al Lusayli|Amiri-ye `Olya|Baqeri `Olya|B?b el Ahmar|Chahar Tang...  1762562         1                     
Region      String   true                     00            Z4                    Unsorted    0.3513     0           2           5303100     1.9646      0.0025         0                0.0013                                                                                                    4                                                         0                                                                                                                                              392          0.0001            04           1           143900            *PREVIEW: H1|H6|H9|I1|I4|K5|K6|L1|L7|M8                                        17              1                     
Population  Integer            2290536125     7             31480498    31480491  Unsorted    0.0036                                                                                                48730.6639  1422.5474  10888.1976      2663.625       308414.0419  95119221210.8852  632.8952  2652350    0           0       47004                      0.9826    8327     -69766.5           -33018             3730.5   10879      28229.5  24499    64978              101726.5           0.4164    28460        0.0105                         1           2652350           *PREVIEW: 100|10001|10002|100023|10003|10005|10007|10008|10009|100105          18970           1                     5: 1002|10: 1869|40: 7145|60: 15286|90: 76682|95: 145843
Latitude    Float              76585211.1978  -54.9333333   82.483333   137.4167  Unsorted    0.0692                                                                                                28.3717     0.0134                                    21.9384      481.2922          77.3249   0          330964      81      2368309     9              0         15.2667  -84.7706           -35.9076           12.9553  33.8667    45.5306  32.5753  94.3935            143.2564           -0.2839   255133       0.0945            50.8         1           1128              *PREVIEW: -.020556|-.025833|-.035|-.035833|-.047222|-.051944|-.054722|-.06...  79002           1                     5: -14.1667|10: -4.5167|40: 27|60: 38.0667|90: 53.1814|95: 56.4
Longitude   Float              75976506.6643  -179.9833333  180.0       359.9833  Unsorted    0.0688                                                                                                28.1462     0.038                                     62.4729      3902.8581         221.9586  0          633589      159     2065606     9              0         30.9583  -199.3667          -98.4917           2.3833   26.8803    69.6333  67.25    170.5083           271.3833           0.2715    407568       0.151             23.1         1           590               *PREVIEW: -.185556|-.208056|-.246944|-.248333|-.263889|-.271667|-.276389|...  162640          1                     5: -88.3|10: -75.5|40: 18.7333|60: 38.2167|90: 113.1|95: 124.6381

```

> ℹ️ **NOTE:** The `qsv table` command takes any CSV data and formats it into aligned columns
using [elastic tabstops](https://github.com/BurntSushi/tabwriter). You'll
notice that it even gets alignment right with respect to Unicode characters.

So, this command took 1.11 seconds to run on my machine, but we can speed
it up by creating an index and re-running the command:

```
qsv index wcp.csv
qsv stats wcp.csv --everything | qsv table
```

Which cuts it down to 0.41 seconds - 2.7x faster! (And creating the 21mb index took 0.22 seconds. 
What about the first `stats` without `--everything`? From 0.72 seconds to 0.08 seconds with an index - 9x faster!)

Notably, the same type of "statistics" command in another
[CSV command line toolkit](https://csvkit.readthedocs.io/)
takes about 10 seconds to produce a *subset* of statistics on the same data set. [Visidata](https://visidata.org)
takes much longer - ~1.5 minutes to calculate a *subset* of these statistics with its Describe sheet. 
Even python [pandas'](https://pandas.pydata.org/docs/reference/api/pandas.DataFrame.describe.html) 
`describe(include="all"))` took 12 seconds to calculate a *subset* of qsv's "streaming" statistics.[^3]

This is another reason for qsv's speed. Creating an index accelerated statistics gathering as it enables 
***multithreading & fast I/O***.

**For multithreading** - running `stats` with an index was 9x faster because it divided the file into 
16 equal chunks[^1] with ~170k records each, then running stats on each chunk in parallel across 16 
cores and merging the results in the end. It was "only" 9x, and not 16x faster as there is 
some overhead involved in multithreading. 

**For fast I/O** - let's say you wanted to grab the last 10 records:

```
$ qsv count --human-readable wcp.csv
2,699,354
$ qsv slice wcp.csv --start -10 | qsv table
Country  City               AccentCity         Region  Population  Latitude     Longitude
zw       zibalonkwe         Zibalonkwe         06                  -19.8333333  27.4666667
zw       zibunkululu        Zibunkululu        06                  -19.6666667  27.6166667
zw       ziga               Ziga               06                  -19.2166667  27.4833333
zw       zikamanas village  Zikamanas Village  00                  -18.2166667  27.95
zw       zimbabwe           Zimbabwe           07                  -20.2666667  30.9166667
zw       zimre park         Zimre Park         04                  -17.8661111  31.2136111
zw       ziyakamanas        Ziyakamanas        00                  -18.2166667  27.95
zw       zizalisari         Zizalisari         04                  -17.7588889  31.0105556
zw       zuzumba            Zuzumba            06                  -20.0333333  27.9333333
zw       zvishavane         Zvishavane         07      79876       -20.3333333  30.0333333
```

`qsv count` took 0.01 seconds and `qsv slice`, 0.01 seconds too! These commands are *instantaneous* 
with an index because for `count` - the index already precomputed the record count, and with `slice`,
*only the sliced portion* has to be parsed - because an index allowed us to jump directly to that 
part of the file. It didn't have to scan the entire file to get the last 10 records. For comparison,
without an index, slice took 0.30 seconds (30x slower).

> ℹ️ **NOTE:** Creating/updating an index itself is extremely fast as well. If you want
qsv to automatically create and update indices, set the environment var `QSV_AUTOINDEX_SIZE`.

Okay, okay! Let's switch gears and stop obsessing over how fast :rocket: qsv is... let's go back to exploring :mag_right:
the data set.

Hmmmm... the Population column has a lot of null values. How pervasive is that?
First, let's take a look at 10 "random" rows with `sample`. We use the `--seed` parameter
so we get a reproducible random sample. And then, let's display only the Country,
AccentCity and Population columns with the `select` command.

```
$ qsv sample --seed 42 10 wcp.csv | 
    qsv select Country,AccentCity,Population | 
    qsv table
Country  AccentCity       Population
cn       Shijidai         
cu       Santa Catalina   
de       Werscheresch     
es       La Cantera       
gw       Reino de Antula  
kz       Karakamys        
md       Peleriya         
se       Norra Lagnö      
sv       Santa Inés       
us       Tellico          
```

Whoops! The sample we got doesn't have population counts. It's quite pervasive.
Exactly how many cities have empty (NULL) population counts?

```
$ qsv frequency wcp.csv --limit 3 | qsv table
field       value              count    percentage  rank
Country     ru                 176934   6.55468     1
Country     us                 141989   5.26011     2
Country     cn                 117508   4.35319     3
Country     Other (228)        2262923  83.83202    0
City        san jose           313      0.0116      1
City        san antonio        310      0.01148     2
City        santa rosa         288      0.01067     3
City        Other (2,008,161)  2698443  99.96625    0
AccentCity  San Antonio        307      0.01137     1
AccentCity  Santa Rosa         288      0.01067     2
AccentCity  Santa Cruz         268      0.00993     3
AccentCity  Other (2,025,913)  2698491  99.96803    0
Region      04                 143900   5.33091     1
Region      02                 127736   4.7321      2
Region      03                 105455   3.90668     3
Region      Other (388)        2322259  86.0303     0
Region      (NULL)             4                    
Population  2310               12       0.02553     1
Population  2137               11       0.0234      2
Population  2230               11       0.0234      2
Population  Other (28,456)     46970    99.92767    0
Population  (NULL)             2652350              
Latitude    50.8               1128     0.04179     1
Latitude    50.95              1076     0.03986     2
Latitude    50.6               1043     0.03864     3
Latitude    Other (255,130)    2696107  99.87971    0
Longitude   23.1               590      0.02186     1
Longitude   23.2               586      0.02171     2
Longitude   23.05              575      0.0213      3
Longitude   Other (407,565)    2697603  99.93513    0
```

(The `qsv frequency` command builds a frequency table for each column in the
CSV data, with an "Other" rollup row summarizing values that fall outside the
top N. This one only took 1.16 seconds.)

So it seems that most cities do not have a population count associated with
them at all (2,652,350 to be exact). No matter — we can adjust our previous 
command so that it only shows rows with a population count:

```
$ qsv search --select Population '[0-9]' wcp.csv |
    qsv sample --seed 1 10 |
    qsv select Country,AccentCity,Population |
    tee sample.csv |
    qsv table
Country  AccentCity             Population
it       Lipari                 10649
cz       Benesov nad Ploucnici  4042
il       Kabul                  9301
us       Wahpeton               8395
in       Pachperwa              15057
ph       Pawing                 3316
us       Fort Dix               6868
ru       Skorodnoye             3677
ee       Haiba                  395
nl       Schoonhoven            12471
```

> ℹ️ **NOTE:** The `tee` command reads from standard input and writes 
to both standard output and one or more files at the same time. We do this so 
we can create the `sample.csv` file we need for the next step, and pipe the 
same data to the `qsv table` command.<br/>Why create `sample.csv`? Even though qsv is blazing-fast, we're just doing an 
initial investigation and a small 10-row sample is all we need to try out and
compose the different CLI commands needed to wrangle the data.

Erk. Which country is `ee`? What continent? No clue, but [datawookie](https://github.com/datawookie) 
has a CSV file called `country-continent.csv`.

```
$ curl -L https://raw.githubusercontent.com/datawookie/data-diaspora/master/spatial/country-continent-codes.csv > country_continent.csv
$ qsv headers country_continent.csv
1 # https://datahub.io/JohnSnowLabs/country-and-continent-codes-list
```

Huh!?! That's not what we were expecting. But if you look at the `country-continent.csv`
file, it starts with a comment with the `#` character. 

```
$ head -5 country_continent.csv
# https://datahub.io/JohnSnowLabs/country-and-continent-codes-list
continent,code,country,iso2,iso3,number
Asia,AS,"Afghanistan, Islamic Republic of",AF,AFG,4
Europe,EU,"Albania, Republic of",AL,ALB,8
Antarctica,AN,Antarctica (the territory South of 60 deg S),AQ,ATA,10
```

No worries, qsv got us covered with its `QSV_COMMENT_CHAR` environment variable. Setting it
to `#` tells qsv to ignore any lines in the CSV - may it be before the header, or even in the data
part of the CSV, that **starts with the character** we set it to.

```
$ export QSV_COMMENT_CHAR='#'

$ qsv headers country_continent.csv
1   continent
2   code
3   country
4   iso2
5   iso3
6   number
```

That's more like it. We can now do a join to see which countries and continents these are:

```
$ qsv join --ignore-case Country sample.csv iso2 country_continent.csv  | qsv table
Country  AccentCity             Population  continent      code  country                       iso2  iso3  number
it       Lipari                 10649       Europe         EU    Italy, Italian Republic       IT    ITA   380
cz       Benesov nad Ploucnici  4042        Europe         EU    Czech Republic                CZ    CZE   203
il       Kabul                  9301        Asia           AS    Israel, State of              IL    ISR   376
us       Wahpeton               8395        North America  NA    United States of America      US    USA   840
in       Pachperwa              15057       Asia           AS    India, Republic of            IN    IND   356
ph       Pawing                 3316        Asia           AS    Philippines, Republic of the  PH    PHL   608
us       Fort Dix               6868        North America  NA    United States of America      US    USA   840
ru       Skorodnoye             3677        Europe         EU    Russian Federation            RU    RUS   643
ru       Skorodnoye             3677        Asia           AS    Russian Federation            RU    RUS   643
ee       Haiba                  395         Europe         EU    Estonia, Republic of          EE    EST   233
nl       Schoonhoven            12471       Europe         EU    Netherlands, Kingdom of the   NL    NLD   528

```

`ee` is Estonia - never would have guessed that. Thing is, now we have several unneeded
columns, and the column names case formats are not consistent. Also, there are two records
for Skorodnoye - for both Europe and Asia. This is because the Russian Federation spans
both continents.
We're primarily interested in unique cities per country for the purposes of this tour,
so we need to filter these out.

Also, apart from renaming the columns, I want to reorder them to "City, Population, Country,
Continent".

No worries. Let's use the `select` (so we only get the columns we need, in the order we want), 
`dedup` (so we only get unique County/City combinations) and `rename` (columns in titlecase) commands: 

```
$ qsv join --ignore-case Country sample.csv iso2 country_continent.csv |
    qsv select 'AccentCity,Population,country,continent' |
    qsv dedup --select 'country,AccentCity' |
    qsv rename City,Population,Country,Continent |
    qsv table
City                   Population  Country                       Continent
Benesov nad Ploucnici  4042        Czech Republic                Europe
Haiba                  395         Estonia, Republic of          Europe
Pachperwa              15057       India, Republic of            Asia
Kabul                  9301        Israel, State of              Asia
Lipari                 10649       Italy, Italian Republic       Europe
Schoonhoven            12471       Netherlands, Kingdom of the   Europe
Pawing                 3316        Philippines, Republic of the  Asia
Skorodnoye             3677        Russian Federation            Asia
Fort Dix               6868        United States of America      North America
Wahpeton               8395        United States of America      North America
```

Nice! Notice the data is now sorted by Country,City too! That's because `dedup` sorts the
CSV records (loading the entire file into memory) to find duplicates - unless you tell it
the input is already sorted with `--sorted` for streaming-mode dedup.  

Now that we've composed all the commands we need, perhaps we can do this with the original CSV data? 
Not the tiny 10-row sample.csv file, but all 2.7 million rows in the 124MB `wcp.csv` file?!  

Indeed we can — because `qsv` is designed for speed - written in [Rust](https://www.rust-lang.org/) with 
[amortized memory allocations](https://blog.burntsushi.net/csv/#amortizing-allocations), using the 
performance-focused [jemalloc](https://github.com/jemalloc/jemalloc) allocator.

```
$ qsv join --ignore-case Country wcp.csv iso2 country_continent.csv |
    qsv search --select Population '[0-9]' |
    qsv select 'AccentCity,Population,country,continent,Latitude,Longitude' |
    qsv dedup --select 'country,AccentCity,Latitude,Longitude' --dupes-output wcp_dupes.csv |
    qsv rename City,Population,Country,Continent,Latitude,Longitude --output wcp_countrycontinent.csv

$ qsv sample 10 --seed 33 wcp_countrycontinent.csv | qsv table
City        Population  Country                            Continent  Latitude    Longitude
Cantilan    9408        Philippines, Republic of the       Asia       9.3336111   125.9775
Björnli     288         Norway, Kingdom of                 Europe     63.133333   9.683333
Bocsa       3394        Romania                            Europe     47.3        22.9166667
La Garriga  13006       Spain, Kingdom of                  Europe     41.6833333  2.2833333
Veriora     549         Estonia, Republic of               Europe     58.0041667  27.3519444
Vrani       1215        Romania                            Europe     45.0383333  21.4925
Vitim       3843        Russian Federation                 Asia       59.4511111  112.5577778
Panatau     2835        Romania                            Europe     45.3166667  26.3833333
Libenge     27051       Congo, Democratic Republic of the  Africa     3.65        18.633333
Volovo      4089        Russian Federation                 Asia       53.55       37.95

$ qsv count -H wcp_countrycontinent.csv
47,004
$ qsv count -H wcp_dupes.csv
5,155
```

We fine-tuned `dedup` by adding `Latitude` and `Longitude` as there may be 
multiple cities with the same name in a country. We also specified the 
`dupes-output` option so we can have a separate CSV of the duplicate records
it removed.

We're also just interested in cities with population counts. So we used `search`
with the regular expression `[0-9]`. This cuts down the file to 47,004 rows.

**The whole thing took ~2.5 seconds on my machine.** The performance of `join`,
in particular, comes from constructing a [SIMD](https://www.sciencedirect.com/topics/computer-science/single-instruction-multiple-data)-accelerated hash index of one of the CSV 
files. The `join` command does an inner join by default, but it also has left,
right and full outer, cross, anti and semi join support too. All from the command line,
without having to load the files into a database, index them, to do a SQL join.

Finally, can we create a CSV file for each country of all its cities? Yes we can, with
the `partition` command (and it took just 0.04 seconds to create all 209 country-city files!):

```
$ qsv partition Country bycountry wcp_countrycontinent.csv
$ cd bycountry
$ ls -1lhS | head -6 ; echo ... ; ls -1lhS | tail -5
total 7256
-rw-r--r--  319K  UnitedStatesofAmerica.csv
-rw-r--r--  260K  PhilippinesRepublicofthe.csv
-rw-r--r--  255K  RussianFederation.csv
-rw-r--r--  169K  IndiaRepublicof.csv
...
-rw-r--r--    1K  Aruba.csv
-rw-r--r--    1K  Anguilla.csv
-rw-r--r--    1K  Gibraltar.csv
-rw-r--r--    1K  Ukraine.csv
```

Examining the USA csv file:

```
$ qsv stats --everything UnitedStatesofAmerica.csv | qsv table --output usa-cities-stats.csv
$ less -S usa-cities-stats.csv
field       type     is_ascii  sum           min                       max                       range    sort_order  sortiness  min_length  max_length  sum_length  avg_length  stddev_length  variance_length  cv_length  mean        sem        geometric_mean  harmonic_mean  stddev       variance        cv        nullcount  n_negative  n_zero  n_positive  max_precision  sparsity  mad      lower_outer_fence  lower_inner_fence  q1        q2_median  q3        iqr     upper_inner_fence  upper_outer_fence  skewness  cardinality  uniqueness_ratio  mode                                                          mode_count  mode_occurrences  antimode                                                                       antimode_count  antimode_occurrences  percentiles
City        String   true                    Abbeville                 Zionsville                         Ascending   1          3           26          38199       9.1495      2.968          8.8093           0.3244                                                                                                  0                                                         0                                                                                                                                               3439         0.8237            Springfield                                                   1           11                *PREVIEW: Abbeville|Abilene|Abington|Acton|Acushnet|Acworth|Ada|Adelanto|...  3002            1                     
Population  Integer            179123400     216                       8107916                   8107700  Unsorted    0.0072                                                                                                42903.8084  2596.2217  22665.0898      15550.3713     167752.8889  28141031740.29  390.9977  0          0           0       4175                       0         8846     -60516             -24217.5           12081     19235      36280     24199   72578.5            108877             0.4087    3981         0.9535            10576|10945|11971|12115|13219|13250|8771|9944                 8           3                 *PREVIEW: 10003|10009|10012|10015|100158|10019|10030|10032|10034|10058          3795            1                     5: 7527|10: 9311|40: 15772|60: 24003|90: 73206|95: 117258
Country     String   true                    United States of America  United States of America           Ascending   1          24          24          100200      24          0              0                0                                                                                                       0                                                         0                                                                                                                                               1            0.0002            United States of America                                      1           4175                                                                                            0               0                     
Continent   String   true                    North America             North America                      Ascending   1          13          13          54275       13          0              0                0                                                                                                       0                                                         0                                                                                                                                               1            0.0002            North America                                                 1           4175                                                                                            0               0                     
Latitude    Float              158455.7902   17.9677778                71.2905556                53.3228  Unsorted    0.0987                                                                                                37.9535     0.0929     37.4081         36.7621        6.0032       36.0386         15.8173   0          0           0       4175        7              0         3.3047   10.4336            22.2444            34.0553   39.4694    41.9292   7.8739  53.74              65.5508            -0.3752   4010         0.9605            42.0333333                                                    1           5                 *PREVIEW: 17.9677778|17.9680556|17.9736111|17.9794444|17.9861111|18.00833...  3856            1                     5: 26.6403|10: 29.875|40: 37.9064|60: 40.7281|90: 43.9014|95: 45.2461
Longitude   Float              -377616.7798  -165.4063889              -65.3013889               100.105  Unsorted    0.0086                                                                                                -90.4471    0.2663                                    17.209       296.1482        -19.0265  0          4175        0       0           7              0         10.1369  -158.9414          -128.2139          -97.4864  -86.0342   -77.0014  20.485  -46.2739           -15.5464           -0.1181   4074         0.9758            -118.3516667|-71.0666667|-71.3972222|-71.4166667|-83.1500000  5           3                 *PREVIEW: -100.0166667|-100.3505556|-100.4055556|-100.4366667|-100.4991667...  3978            1                     5: -122.1286|10: -118.4381|40: -89.4556|60: -82.4894|90: -72.0339|95: -71.0106
```

Hhhmmm... clearly the worldcitiespop.csv file from the Data Science Toolkit does not have 
comprehensive coverage of City populations.

The US population is far more than 179,123,400 (Population sum) and 3,439 cities (City cardinality).
Perhaps we can get population info elsewhere with the `fetch` command...
But that's another tour by itself! 😄

[^1]: Timings collected by setting `QSV_LOG_LEVEL='debug'` on an Apple M4 Max (16 cores) MacBook Pro running macOS 26.5 with 64gb of unified memory.
[^2]: For example, running `qsv stats` on a CSV export of ALL of NYC's available 311 data from 2010 to Mar 2022 (27.8M rows, 16gb) took just 22.4 seconds with an index (which actually took longer to create - 39 seconds to create a 223mb index), and its memory footprint remained the same, pinning all 16 logical processors near 100% utilization on a Ryzen 7 4800H laptop with 32gb memory and 1 TB SSD running Windows 11. (qsv on current Apple Silicon hardware is even faster.)
[^3]: [Why is qsv exponentially faster than python pandas?](https://github.com/dathere/datapusher-plus/discussions/15)
