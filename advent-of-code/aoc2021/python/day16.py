def parse_packet(packet):
    version, type_id, rest = int(packet[0:3], 2), int(packet[3:6], 2), packet[6:]
    print(version, type_id, rest)
    if type_id == 4:
        return version, 0, None
    length_type_id, rest = int(rest[0], 2), rest[1:]

def part1():
    data = '8A004A801A8002F478'
    binary = bin(int(data, 16))[2:]
    parse_packet(binary)


part1()
