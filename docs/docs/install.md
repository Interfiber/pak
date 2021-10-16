# Installing Pak

## Dependencies

1. MacOS cli tools installed ```xcode-select --install```
2. Lua version ```5.4.3```
3. Luarocks version ```3.7.0```

## Installing pak
After you have installed all of the dependencies run the following command in you're terminal
```bash
/bin/bash -c "$(curl -fsSL https://raw.githubusercontent.com/Interfiber/pak/main/tools/installer)"
```
After the install completes add the following line to you're shell profile if you have not already
```bash
source $HOME/.pak/source_file
```