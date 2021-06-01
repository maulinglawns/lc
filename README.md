# lc (linecount)
Count &amp; display number of lines in files in a directory

## Usage
````
USAGE:
    lc [FLAGS] <TARGETDIR>

FLAGS:
    -d, --descend     Sort descending by linecount
    -h, --help        Prints help information
    -t, --textonly    Skip 'non-text' files. E.g. '.mp3', 'jpg', '.tar' etc.
    -V, --version     Prints version information

ARGS:
    <TARGETDIR>    Look for files in this directory
````
      
By default files are sorted by linecount in _ascending_ order.

## Caveats
The `-t` flag makes an educated (actually a _very_ educated) guess on most
text files being just 'text'.  
In other words, there can be possible corner cases.
      
See: https://crates.io/crates/infer for more information.

## Example
````
lc ../sonarlogs/
../sonarlogs/:
Filename                                           Lines
--------------------------------------------------------
es.log                                             29   
ce.log                                             82   
sonar.log                                          103  
web.log                                            171  
````
