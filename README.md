# mtg-inventory

## Compiling on windows

Thanks to [this](https://stackoverflow.com/a/76427629) stack overflow answer.

1. Run following in CMD (not powershell): 

```
choco install sqlite
cd C:\ProgramData\chocolatey\lib\SQLite\tools
call "C:\Program Files (x86)\Microsoft Visual Studio\2019\BuildTools\VC\Auxiliary\Build\vcvars64.bat"
lib /machine:x64 /def:sqlite3.def /out:sqlite3.lib
```
2. Set env variable `SQLITE3_LIB_DIR=C:\ProgramData\chocolatey\lib\SQLite\tools`
3. Add to path `C:\ProgramData\chocolatey\lib\SQLite\tools`
4. Run `cargo run --release` to make sure everything works
