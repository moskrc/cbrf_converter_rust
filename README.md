# Converts currency rates from XML to JSON format

This program retrieves currency exchange rate data from the [Central Bank of Russia](https://www.cbr.ru) in [XML format](https://www.cbr.ru/scripts/XML_daily.asp) and outputs it in JSON. If you specify the `--debug` parameter, additional information will be provided in a human-readable format.

## How to build

```bash
$ cargo build
$ cargo run -- --help
```

## How to use

```bash
$ ./converter --help
Convert currency rates from XML to JSON format

Usage: converter [OPTIONS]

Options:
  -d, --debug
  -h, --help     Print help
  -V, --version  Print version
```

```bash
$ ./converter
{"date":"20.01.2024","name":"Foreign Currency Market","valutes":[{"id":"R01230","num_code":"784","char_code":"AED","nominal":1,"name":"Дирхам ОАЭ","value":"24,1224","vunit_rate":"24,1224"},{"id":"R01060","num_code":"051","char_code":"AMD","nominal":100,"name":"Армянских драмов","value":"21,8513","vunit_rate":"0,218513"},{"id":"R01010","num_code":"036","char_code":"AUD","nominal":1,"name":"Австралийский доллар","value":"58,2831","vunit_rate":"58,2831"},{"id":"R01020A","num_code":"944","char_code":"AZN","nominal":1,"name":"Азербайджанский манат","value":"52,1115","vunit_rate":"52,1115"},{"id":"R01100","num_code":"975","char_code":"BGN","nominal":1,"name":"Болгарский лев","value":"49,2575","vunit_rate":"49,2575"},{"id":"R01115","num_code":"986","char_code":"BRL","nominal":1,"name":"Бразильский реал","value":"17,9331","vunit_rate":"17,9331"},{"id":"R01090B","num_code":"933","char_code":"BYN","nominal":1,"name":"Белорусский рубль","value":"28,0019","vunit_rate":"28,0019"},{"id":"R01350","num_code":"124","char_code":"CAD","nominal":1,"name":"Канадский доллар","value":"65,5976","vunit_rate":"65,5976"},{"id":"R01775","num_code":"756","char_code":"CHF","nominal":1,"name":"Швейцарский франк","value":"102,0030","vunit_rate":"102,003"},{"id":"R01375","num_code":"156","char_code":"CNY","nominal":1,"name":"Китайский юань","value":"12,2706","vunit_rate":"12,2706"},{"id":"R01760","num_code":"203","char_code":"CZK","nominal":10,"name":"Чешских крон","value":"38,9268","vunit_rate":"3,89268"},{"id":"R01215","num_code":"208","char_code":"DKK","nominal":1,"name":"Датская крона","value":"12,9194","vunit_rate":"12,9194"},{"id":"R01240","num_code":"818","char_code":"EGP","nominal":10,"name":"Египетских фунтов","value":"28,6764","vunit_rate":"2,86764"},{"id":"R01239","num_code":"978","char_code":"EUR","nominal":1,"name":"Евро","value":"96,3835","vunit_rate":"96,3835"},{"id":"R01035","num_code":"826","char_code":"GBP","nominal":1,"name":"Фунт стерлингов Соединенного королевства","value":"112,2607","vunit_rate":"112,2607"},{"id":"R01210","num_code":"981","char_code":"GEL","nominal":1,"name":"Грузинский лари","value":"33,3344","vunit_rate":"33,3344"},{"id":"R01200","num_code":"344","char_code":"HKD","nominal":1,"name":"Гонконгский доллар","value":"11,3504","vunit_rate":"11,3504"},{"id":"R01135","num_code":"348","char_code":"HUF","nominal":100,"name":"Венгерских форинтов","value":"25,1696","vunit_rate":"0,251696"},{"id":"R01280","num_code":"360","char_code":"IDR","nominal":10000,"name":"Индонезийских рупий","value":"56,6792","vunit_rate":"0,00566792"},{"id":"R01270","num_code":"356","char_code":"INR","nominal":10,"name":"Индийских рупий","value":"10,6750","vunit_rate":"1,0675"},{"id":"R01820","num_code":"392","char_code":"JPY","nominal":100,"name":"Японских иен","value":"59,8255","vunit_rate":"0,598255"},{"id":"R01370","num_code":"417","char_code":"KGS","nominal":100,"name":"Киргизских сомов","value":"99,1850","vunit_rate":"0,99185"},{"id":"R01815","num_code":"410","char_code":"KRW","nominal":1000,"name":"Вон Республики Корея","value":"66,1610","vunit_rate":"0,066161"},{"id":"R01335","num_code":"398","char_code":"KZT","nominal":100,"name":"Казахстанских тенге","value":"19,6081","vunit_rate":"0,196081"},{"id":"R01500","num_code":"498","char_code":"MDL","nominal":10,"name":"Молдавских леев","value":"50,0294","vunit_rate":"5,00294"},{"id":"R01535","num_code":"578","char_code":"NOK","nominal":10,"name":"Норвежских крон","value":"84,1698","vunit_rate":"8,41698"},{"id":"R01530","num_code":"554","char_code":"NZD","nominal":1,"name":"Новозеландский доллар","value":"54,2346","vunit_rate":"54,2346"},{"id":"R01565","num_code":"985","char_code":"PLN","nominal":1,"name":"Польский злотый","value":"21,9885","vunit_rate":"21,9885"},{"id":"R01355","num_code":"634","char_code":"QAR","nominal":1,"name":"Катарский риал","value":"24,3378","vunit_rate":"24,3378"},{"id":"R01585F","num_code":"946","char_code":"RON","nominal":1,"name":"Румынский лей","value":"19,3702","vunit_rate":"19,3702"},{"id":"R01805F","num_code":"941","char_code":"RSD","nominal":100,"name":"Сербских динаров","value":"82,2502","vunit_rate":"0,822502"},{"id":"R01770","num_code":"752","char_code":"SEK","nominal":10,"name":"Шведских крон","value":"84,7772","vunit_rate":"8,47772"},{"id":"R01625","num_code":"702","char_code":"SGD","nominal":1,"name":"Сингапурский доллар","value":"66,0082","vunit_rate":"66,0082"},{"id":"R01675","num_code":"764","char_code":"THB","nominal":10,"name":"Таиландских батов","value":"24,9161","vunit_rate":"2,49161"},{"id":"R01670","num_code":"972","char_code":"TJS","nominal":10,"name":"Таджикских сомони","value":"80,8653","vunit_rate":"8,08653"},{"id":"R01710A","num_code":"934","char_code":"TMT","nominal":1,"name":"Новый туркменский манат","value":"25,3113","vunit_rate":"25,3113"},{"id":"R01700J","num_code":"949","char_code":"TRY","nominal":10,"name":"Турецких лир","value":"29,4202","vunit_rate":"2,94202"},{"id":"R01720","num_code":"980","char_code":"UAH","nominal":10,"name":"Украинских гривен","value":"23,4927","vunit_rate":"2,34927"},{"id":"R01235","num_code":"840","char_code":"USD","nominal":1,"name":"Доллар США","value":"88,5896","vunit_rate":"88,5896"},{"id":"R01717","num_code":"860","char_code":"UZS","nominal":10000,"name":"Узбекских сумов","value":"71,3273","vunit_rate":"0,00713273"},{"id":"R01150","num_code":"704","char_code":"VND","nominal":10000,"name":"Вьетнамских донгов","value":"36,8555","vunit_rate":"0,00368555"},{"id":"R01589","num_code":"960","char_code":"XDR","nominal":1,"name":"СДР (специальные права заимствования)","value":"117,9260","vunit_rate":"117,926"},{"id":"R01810","num_code":"710","char_code":"ZAR","nominal":10,"name":"Южноафриканских рэндов","value":"46,6497","vunit_rate":"4,66497"}]}
```

```bash
 $ ./converter --debug
=================================== REPORT ====================================
Date: 20.01.2024
Name: Foreign Currency Market
-------------------------------------------------------------------------------
id       num      char   nominal  value      vunit_rate   name
-------------------------------------------------------------------------------
R01230   784      AED    1        24,1224    24,1224      Дирхам ОАЭ
R01060   051      AMD    100      21,8513    0,218513     Армянских драмов
R01010   036      AUD    1        58,2831    58,2831      Австралийский доллар
R01020A  944      AZN    1        52,1115    52,1115      Азербайджанский манат
R01100   975      BGN    1        49,2575    49,2575      Болгарский лев
R01115   986      BRL    1        17,9331    17,9331      Бразильский реал
R01090B  933      BYN    1        28,0019    28,0019      Белорусский рубль
R01350   124      CAD    1        65,5976    65,5976      Канадский доллар
R01775   756      CHF    1        102,0030   102,003      Швейцарский франк
R01375   156      CNY    1        12,2706    12,2706      Китайский юань
R01760   203      CZK    10       38,9268    3,89268      Чешских крон
R01215   208      DKK    1        12,9194    12,9194      Датская крона
R01240   818      EGP    10       28,6764    2,86764      Египетских фунтов
R01239   978      EUR    1        96,3835    96,3835      Евро
R01035   826      GBP    1        112,2607   112,2607     Фунт стерлингов Соединенного королевства
R01210   981      GEL    1        33,3344    33,3344      Грузинский лари
R01200   344      HKD    1        11,3504    11,3504      Гонконгский доллар
R01135   348      HUF    100      25,1696    0,251696     Венгерских форинтов
R01280   360      IDR    10000    56,6792    0,00566792   Индонезийских рупий
R01270   356      INR    10       10,6750    1,0675       Индийских рупий
R01820   392      JPY    100      59,8255    0,598255     Японских иен
R01370   417      KGS    100      99,1850    0,99185      Киргизских сомов
R01815   410      KRW    1000     66,1610    0,066161     Вон Республики Корея
R01335   398      KZT    100      19,6081    0,196081     Казахстанских тенге
R01500   498      MDL    10       50,0294    5,00294      Молдавских леев
R01535   578      NOK    10       84,1698    8,41698      Норвежских крон
R01530   554      NZD    1        54,2346    54,2346      Новозеландский доллар
R01565   985      PLN    1        21,9885    21,9885      Польский злотый
R01355   634      QAR    1        24,3378    24,3378      Катарский риал
R01585F  946      RON    1        19,3702    19,3702      Румынский лей
R01805F  941      RSD    100      82,2502    0,822502     Сербских динаров
R01770   752      SEK    10       84,7772    8,47772      Шведских крон
R01625   702      SGD    1        66,0082    66,0082      Сингапурский доллар
R01675   764      THB    10       24,9161    2,49161      Таиландских батов
R01670   972      TJS    10       80,8653    8,08653      Таджикских сомони
R01710A  934      TMT    1        25,3113    25,3113      Новый туркменский манат
R01700J  949      TRY    10       29,4202    2,94202      Турецких лир
R01720   980      UAH    10       23,4927    2,34927      Украинских гривен
R01235   840      USD    1        88,5896    88,5896      Доллар США
R01717   860      UZS    10000    71,3273    0,00713273   Узбекских сумов
R01150   704      VND    10000    36,8555    0,00368555   Вьетнамских донгов
R01589   960      XDR    1        117,9260   117,926      СДР (специальные права заимствования)
R01810   710      ZAR    10       46,6497    4,66497      Южноафриканских рэндов

```