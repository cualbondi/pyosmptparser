# pyosmptparser

Python bindings for [osmptparser](https://github.com/cualbondi/osmptparser), a fast, parallel parser that reads public transport (PTv2) relations from an OpenStreetMap `.pbf` extract, reconnects broken/disconnected ways with some tolerance and returns each route as an ordered geometry plus its stops.

pypi package: https://pypi.org/project/pyosmptparser/

## Install

```
pip install pyosmptparser
```

Prebuilt wheels are published for Linux (glibc and musl, x86_64 and aarch64), macOS (Intel and Apple Silicon) and Windows, for any CPython >= 3.9 (abi3). Other platforms build from the sdist, which needs a Rust toolchain.

## Usage

```python
import pyosmptparser

p = pyosmptparser.Parser.new_ptv2('ecuador-latest.osm.pbf')   # all PTv2 route relations
pts = p.get_public_transports(150)                            # join ways with gaps up to 150 m

for pt in pts:
    pt.id            # osm relation id
    pt.tags          # dict of osm tags, e.g. pt.tags['name']
    pt.info          # dict of osm metadata: version, timestamp (epoch seconds), changeset, uid, user
    pt.geometry      # list of linestrings, each a list of (lon, lat); 1 linestring when fully joined
    pt.stops         # list of Node(id, lon, lat, tags) in relation member order
    pt.status.code   # 0 ok, 101 sorted, 102 joined, 103 joined+sorted, 501 broken
    pt.status.detail
```

Other constructors:

```python
pyosmptparser.Parser.new_aa('file.pbf')                       # administrative boundary relations
pyosmptparser.Parser('file.pbf', 'route=bus,trolleybus')      # any osmptparser tag filter
pyosmptparser.Parser.new_ptv2('file.pbf', num_threads=4)      # default: all cpus
```

A missing or unreadable file raises `FileNotFoundError` / `OSError`. Set `OSMSM_VERBOSE=1` to get the parser's progress log on stderr.

See [test_pyosmptparser.py](/test_pyosmptparser.py) for a complete example.

## Develop

Needs a stable Rust toolchain (https://rustup.rs) and Python >= 3.9.

```
git clone git@github.com:cualbondi/pyosmptparser.git
cd pyosmptparser
python3 -m venv .venv
source .venv/bin/activate
pip install maturin pytest

# build the extension into the venv and run the tests
maturin develop --release
pytest

# rust lint, same as CI
cargo fmt -- --check && cargo clippy --all-targets -- -D warnings

# build a wheel + sdist into dist/
maturin build --release --sdist --out dist
```

To use a local checkout of the Rust library, point the dependency at it in `Cargo.toml` (`osmptparser = { path = "../osmptparser" }`).

## Release

1. Bump `version` in `Cargo.toml` (and `osmptparser` there if the Rust library was released), update `CHANGELOG.md`, commit.
2. Tag and push: `git tag v3.0.0 && git push origin master v3.0.0`.
3. The [CI workflow](.github/workflows/CI.yml) builds all wheels and the sdist, runs the tests, and publishes to PyPI through [trusted publishing](https://docs.pypi.org/trusted-publishers/) (one-time setup on PyPI: owner `cualbondi`, repository `pyosmptparser`, workflow `CI.yml`, environment `pypi`).

Manual alternative: `maturin publish` with a PyPI token (`MATURIN_PYPI_TOKEN`).
