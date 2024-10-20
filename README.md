## UNIX CUT TOOL

This package copies the basic functionalities of the Unix cut tool. 
Works with csv, tsv etc. By default, the delimiter is set to a tab.

To get the 2 column in a tsv file, simply run

```bash
./cuttool -f2 sample.tsv
```

Or the first and third column in a csv file

```bash
./cuttool -f1,3 -d, fourchords.csv
```

This also works 

```bash
./cuttool -d, -f"1 2, 4" fourchords.csv
```

Cuttool also works well with other unix tools as shown below

```bash
 ./cuttool -f2 -d, fourchords.csv | uniq | wc -l 
```