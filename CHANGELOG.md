## v4.0.0

Areas: upgrade to [osmptparser v4.0.0](https://github.com/cualbondi/osmptparser/blob/master/CHANGELOG.md), which assembles areas as real multipolygons and keeps relation member roles.

 - Added: `Parser.get_areas(gap)` returns a list of `Area` for the relations matching the filter (`Parser.new_aa`, or any `Parser(path, filter)`) plus the standalone closed ways matching it.
 - Added: `Area` class: `id`, `id_type` (`'r'` relation / `'w'` way), `tags`, `info`, `geometry` (list of polygons, each a list of closed `(lon, lat)` rings, first ring outer, rest holes; `[]` when broken), `status` (`ParseStatus`), `repr()`.
 - Added: `Node.role`, the osm relation member role of a stop (`'stop'`, `'platform'`, ...), also shown in `repr()`.
 - Added: `areas.pbf` test fixture and tests for `get_areas`, `Node.role` and the missing-file error of `new_aa`.
 - Unchanged: `Parser.new_ptv2`, `Parser.new_aa`, `Parser(path, filter)`, `get_public_transports(gap)` and the `PublicTransport` / `ParseStatus` attributes.

## v3.0.0

Upgrade to [osmptparser v3.0.0](https://github.com/cualbondi/osmptparser/blob/master/CHANGELOG.md) (faster, bugfixes). The Python API is unchanged: `Parser.new_ptv2`, `Parser.new_aa`, `Parser(path, filter)`, `get_public_transports(gap)`, and the `PublicTransport` / `Node` / `ParseStatus` attributes keep the same names and values.

 - Changed: built on osmptparser 3.0.0. Progress logging to stderr is now silent unless the `OSMSM_VERBOSE` env var is set.
 - Changed: a missing or unreadable `.pbf` raises `FileNotFoundError` / `OSError` instead of aborting the interpreter with a Rust panic.
 - Changed: wheels are now [abi3](https://docs.python.org/3/c-api/stable.html) (`cp39-abi3`), so one wheel per platform works on every CPython >= 3.9 (previously only a cp312 wheel was published).
 - Added: `pyosmptparser.__version__`.
 - Added: `repr()` for `PublicTransport`, `Node` and `ParseStatus`.
 - Added: GitHub Actions workflow that builds Linux (glibc + musl, x86_64 + aarch64), macOS and Windows wheels plus the sdist, runs the tests, and publishes to PyPI on `v*` tags.
 - Internal: pyo3 0.22 -> 0.27, Rust edition 2021.

## v2.2.0

 - Upgrade to osmptparser 2.2.0 and latest maturin.

## v1.2.2

 - Upgrade to osmptparser 1.2.2 and maturin.

## v1.2.0

 - Added `info` field to expose public transport osm metadata.

## v1.1.0

 - Added `ParseStatus` to `PublicTransport`.

## v1.0.0

 - Initial release.
