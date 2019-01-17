```
Zarrebin 0.1.1
Hosein Khansari <hrk.x11@gmail.com>
find files in terminal

USAGE:
    zarrebin [FLAGS] [OPTIONS] [--] [dir]

FLAGS:
    -f, --flat              disable recursion (equivalent to --depth 0)
    -h, --help              Prints help information
    -I, --ignore-errors     don't print any error info
    -H, --ignore-hiddens    ignore hidden entries
    -M, --machine           use machine-readable output
    -S, --no-stats          disable statistics
    -V, --version           Prints version information

OPTIONS:
    -d, --max-depth <depth>       depth for recursion (if negative, goes into every sub-directory if possible)
    -x, --exclude <exclude>...    directories to exclude (doesn't work with sub-directories yet)
    -e, --ext <extension>...      filter files by extension
    -n, --name <name>             filter by name

ARGS:
    <dir>    directory to search in
```
