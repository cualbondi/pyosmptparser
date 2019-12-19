# A python wrapper for [osmptparser](https://github.com/cualbondi/osmptparser)

## Install

```
pip install pyosmptparser
```

pypi package: https://pypi.org/project/pyosmptparser/

## Usage

```
import pyosmptparser
p = pyosmptparser.Parser('ecuador-osm.pbf')
pts = p.get_public_transports(150)
pt1 = [p for p in pts if p.id == 85965][0]
print(pt1)
```

(see [test_pyosmptparser.py](/test_pyosmptparser.py) file for a more complete example)

## Develop

```
git clone git@github.com:cualbondi/pyosmptparser.git
cd pyosmptparser
rustup override set nightly
virtualenv --python python3 .env
source .env/bin/activate

# run tests
pip3 install tox pyo3-pack
tox

# build and copy
pyo3-pack develop
cp target/debug/libpyosmptparser.so pyosmptparser.so

# build and deploy
pyo3-pack publish
```
