import uuid

input_str = input("Enter 16 bytes (hex) separated by spaces: ")

try:
    byte_list = [int(b, 16) for b in input_str.strip().split()]
    if len(byte_list) != 16:
        raise ValueError("Error: you must enter exactly 16 bytes.")
except ValueError as e:
    print("Invalid input:", e)
    exit(1)

raw_bytes = bytes(byte_list)

u = uuid.UUID(bytes=raw_bytes)
print("UUID:", u)
