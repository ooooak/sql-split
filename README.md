sql-split
------------
sql-split is a command line tool to split large sql dump file into small files. its designed to handle large files without eating the whole ram.


Usage 
----------

```bash
$ sql-split.exe file.sql --output=20mb
```

TODO:
1. ~~`parse cli params `output_size`~~
2. write tests
3. ~~fix unsafe code. remove die~~
4. better error reporting
5. add multi threading
