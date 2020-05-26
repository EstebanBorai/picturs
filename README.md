<div align="center">
  <img src="https://raw.githubusercontent.com/estebanborai/icontron/master/assets/test.png" height="150" width="150" />
  <h1>icontron</h1>
  <small>:electron: CLI for building Electron icons from a file</small>
</div>

```
icontron 0.1.0
Esteban Borai <estebanborai@gmail.com> (https://github.com/estebanborai)

USAGE:
    icontron [OPTIONS] --file <FILE>

FLAGS:
    -h, --help       Prints help information
    -V, --version    Prints version information

OPTIONS:
    -f, --file <FILE>                  Image file to create the icons from
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
