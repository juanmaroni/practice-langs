from operations import *

ip1: str = '192.168.33.12'
mask1: str = '255.255.224.0'

ip2: str = '10.54.12.0'
mask2: str = '255.255.255.0'

ip2: str = '192.168.114.32'
mask2: str = '255.255.255.224'

# Network Address
network_address = get_network_address(ip2, mask2)
print(network_address)

# Broadcast Address
print(get_broadcast_address(network_address, mask2))

# Host Address
print(get_host(ip2, mask2))

# Amount of Hosts
print(n_hosts(mask2))
