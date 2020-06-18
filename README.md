<div align="center">
  <img src="https://raw.githubusercontent.com/estebanborai/iconize/master/assets/test.png" height="150" width="150" />
  <h1>iconize</h1>
  <small>CLI utility for building multiplatform icons</small>
</div>

## Capabilies
Iconize is a CLI capable of generate icon sets for MacOS, Linux and Windows.

```
iconize 0.1.0
Esteban Borai <estebanborai@gmail.com> (https://github.com/estebanborai)

USAGE:
    iconize [OPTIONS] --file <FILE>

FLAGS:
    -h, --help       Prints help information
    -V, --version    Prints version information

OPTIONS:
    -f, --file <FILE>                  Image file to create the icons from
    -o, --output <OUTPUT_DIRECTORY>    Path to directory where files will be written
    -t, --target <TARGET_LIST>         Target OS icon files
```

## Icons for each target

| Target OS  | Output Format | Input File Dimensions (Square) |
| ---------- | ---------- | ---------- |
| ⚠️ macOS | `icns` | 512 |
| Windows | `ico` | 256 |
| Linux | `png` | 256 |

> ⚠️ Not supported yet

## References
- [electron-builder/icons](https://www.electron.build/icons)
