# A python wrapper for osmptparser

```
git clone git@github.com:cualbondi/pyosmptparser.git
cd pyosmptparser
rustup override set nightly
virtualenv --python python3 .env
source .env/bin/activate

# run tests
pip3 install tox
tox

# build and copy
pyo3-pack develop
cp target/debug/libpyosmptparser.so pyosmptparser.so

# build and deploy

```
