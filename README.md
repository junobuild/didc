# JunoBuild-Didc

`junobuild-didc` is a CLI tool for generating JavaScript, TypeScript, or DID output from a Candid `.did` file using the [didc](https://github.com/dfinity/candid) tool.

This utility is designed to be integrated in Juno's CLI.

## Installation

You can install `junobuild-didc` globally using Cargo by running:

```bash
cargo install junobuild-didc
```

## Usage

You can use the `junobuild-didc` command to generate JavaScript or TypeScript bindings from a DID file.

```bash
junobuild-didc --input <input_file.did> --target <js|ts> [--output <output_file>]
```

### Example

```bash
junobuild-didc --input ./path/to/interface.did --target ts --output ./generated/output.ts
```

- `--input` or `-i`: Path to the `.did` file.
- `--target` or `-t`: Choose either `js` for JavaScript or `ts` for TypeScript output.
- `--output` or `-o`: (Optional) Specify the output file. If omitted, the generated content will be printed to the console.

## Development

### Building from Source

If you want to build the tool from the source, you can clone the repository and build it yourself.

```bash
git clone https://github.com/junobuild/didc
cd didc
cargo build --release
```

Once built, the binary will be located in `target/release`. You can run it from the root directory:

```bash
./target/release/junobuild-didc --help
```

This will display the available commands and usage instructions.

### Running Tests

You can run the tests with:

```bash
cargo test
```

## License

This project is licensed under the MIT License. See the [LICENSE](LICENSE) file for details.