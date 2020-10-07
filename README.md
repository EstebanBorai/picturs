<div align="center">
  <img src="https://raw.githubusercontent.com/EstebanBorai/picturs/main/assets/test.png" height="150" width="150" />
  <h1>picturs</h1>
  <small>Command-line utility to manipulate image files</small>
</div>

## Description

**picturs** is a CLI utility that resizes and encode image files for multiple platforms, such
as MacOS, Linux and Windows.

## Installation

```bash
cargo install picturs
```

## Usage

`picturs [OPTIONS] <input_file> --target <target>`

The only argument required is the `input_file` which should be a valid image file. Specs
about the [File Requirements](https://github.com/estebanborai/picturs#file-requirements).

### Options

Short | Long | Usage | Description
-- | -- | -- | --
`-o` | `--output` | `picturs -o icons/` | The path to the output directory for your icons
`-t` | `--target` | `picturs -t osx, windows` | The target OS for icon files

## File Requirements

Depending on the `target` OS, your `input file` must match different sizes for
**picturs** to process such file.

Target OS | Target | Minimum Expected Dimensions | Format
--- | --- | --- | ---
macOS | `osx` | 512x512 | `PNG`
Linux | `linux` | 16x16 | `PNG`
Windows | `windows` | 256x256 | `PNG`


## Contributing

Contributions to this repository are welcome, feel free to open either an Issue or
Pull Request.

## License

Licensed under the MIT License
