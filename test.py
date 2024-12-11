import tor_cell_codec

def print_hex_dump(data):
    """print hex dump of bytes in a readable format"""
    if len(data) > 32:
        hex_str = ' '.join(f'{b:02x}' for b in data[:16])
        hex_str += ' ... '
        hex_str += ' '.join(f'{b:02x}' for b in data[-16:])
    else:
        hex_str = ' '.join(f'{b:02x}' for b in data)
    return hex_str

def test_fixed_length():
    """test fixed-length cell handling"""
    circ_id, cmd, body = 1, 2, b"test_body"  # data cell
    
    encoded = tor_cell_codec.encode_channel_cell(circ_id, cmd, body)
    decoded = tor_cell_codec.decode_channel_cell(encoded)

    print(f"\n[fixed cell test]")
    print(f"input: circ={circ_id} cmd={cmd} body={body!r}")
    print(f"encoded [{len(encoded)}]: {print_hex_dump(encoded)}")
    print(f"decoded: circ={decoded[0]} cmd={decoded[1]} body={decoded[2]!r}")

def test_variable_length():
    """test variable-length cell handling"""
    circ_id, cmd, body = 0, 128, b"variable test data"  # vpadding cell
    encoded = tor_cell_codec.encode_channel_cell(circ_id, cmd, body)
    decoded = tor_cell_codec.decode_channel_cell(encoded)

    print(f"\n[variable cell test]")
    print(f"input: circ={circ_id} cmd={cmd} body={body!r}")
    print(f"encoded [{len(encoded)}]: {print_hex_dump(encoded)}")
    print(f"decoded: circ={decoded[0]} cmd={decoded[1]} body={decoded[2]!r}")

def test_error_cases():
    """test error handling"""
    print("\n=== Error Cases ===")
    
    try:
        # vpadding with non-zero circuit id
        tor_cell_codec.encode_channel_cell(1, 128, b"test")
    except ValueError as e:
        print(f"Error (expected): {e}")
        
    try:
        # data cell with zero circuit id
        tor_cell_codec.encode_channel_cell(0, 2, b"test")
    except ValueError as e:
        print(f"Error (expected): {e}")
        
    try:
        # oversized cell body
        tor_cell_codec.encode_channel_cell(1, 2, b"x" * 510)
    except ValueError as e:
        print(f"Error (expected): {e}")

if __name__ == "__main__":
    test_fixed_length()
    test_variable_length()
    test_error_cases()