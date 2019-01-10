# Zarrebin
🔎 find files from terminal
## Usage
```sh
zarrebin [FLAGS] [OPTIONS] [--] [dir]
```
It can accept options and **filters**. 
## Filters
- `-e, --ext <extension>...` filter files by extension
- `-n, --name <name>` filter by name
- `-c, --containing <substring>` filter files that contain the given text
## Build
```sh
git clone https://github.com/hkh12/zarrebin.git
cd zarrebin
cargo build --release
```
> Maybe I'll publish it as a snap...!