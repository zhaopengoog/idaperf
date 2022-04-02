idaperf
=======

Simple tool for viewing Linux perf traces in IDA Pro.

### Screenshot
![Screenshot][screenshot]

### Installation

#### Rust Installation for Newbies
If you don't have Rust installed yet, you can learn how to do so [here][rustup] or run the following command if you believe you are as lucky as me.

```shell
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```

Rust is required to compile the preprocessing util for the perf trace since
doing that work in IDA Python would likely have made it impossible to import
real world sized traces that tend to be many gigabytes in size.
#### Build IdaPerf
Use one of the following approaches.

```shell
cargo install --git https://github.com/zhaopengoog/idaperf.git
```
or

```shell
git clone https://github.com/zhaopengoog/idaperf.git
cd idaperf
cargo install --path .
```

### Usage

```shell
perf record -o ./perf.data -- ./your-app-here
perf script --no-demangle -i ./perf.data -F ip,sym,symoff,dso,event | idaperf your-binary-name-here > dump.csv
```

The `your-binary-name-here` string doesn't have to be the whole module/binary name,
it is sufficient to provide a substring that uniquely filters for symbols
in your module/binary.

After that, you can import the reduced dump into IDA as follows:
- Load your module into IDA
- `File -> Script file`
- Select the `idapy-import-perf-data.py` from this repo
- Select the `dump.csv` you previously created
- Done!

[screenshot]: ./assets/screenshot.png
[rustup]: https://www.rust-lang.org/tools/install
