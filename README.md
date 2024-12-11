# tor-cell-codec

Python bindings for Tor channel cell codec written in Rust. Created as a test task for Arti project.

## Prerequisites

- Python 3.8+
- Rust 1.70+

## Dependencies

```bash
# Install dependencies and build
pip install maturin
maturin develop
```

## Usage

```python
import tor_cell_codec

# encode fixed-length cell
encoded = tor_cell_codec.encode_channel_cell(1, 2, b"data")  # (circuit_id, command, body)
decoded = tor_cell_codec.decode_channel_cell(encoded)  # returns (circuit_id, command, body)

# encode variable-length cell
encoded = tor_cell_codec.encode_channel_cell(0, 128, b"vdata")  # VPADDING command requires circuit_id=0
decoded = tor_cell_codec.decode_channel_cell(encoded)
```

## Development

```bash
# Run tests
python test.py
```

## License

Same as Arti - Apache/MIT