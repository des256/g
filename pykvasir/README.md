# Kvasir Engine - Python Interface

## Build Instructions

First, compile the rust part:

```
cargo build --release
```

Then, create the Python package:

```
python3 setup.py bdist_wheel
```

And finally, install this package on the local machine:

```
python3 -m pip install dist/pykvasir-0.1-py3-none-all.whl
```

## API

Prettymuch the rust API translates directly to Python.
