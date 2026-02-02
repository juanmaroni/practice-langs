import typing

def dec_to_octets(address: str) -> str:
    return ['{0:08b}'.format(int(oct)) for oct in address.split('.')]

def split_octects(address: str) -> str:
    return [int(oct) for oct in address.split('.')]

def get_network_address(ip: str, netmask: str) -> str:
    lst_ip = split_octects(ip)
    lst_netmask = split_octects(netmask)

    return ('.').join([str(i & j) for (i, j) in zip(lst_ip, lst_netmask)])

def get_broadcast_address(netword_address: str, netmask: str) -> str:
    lst_network = split_octects(netword_address)
    lst_netmask = split_octects(netmask)
    
    return ('.').join([str(i | 255 ^ j) for (i, j) in zip(lst_network, lst_netmask)])

def get_host(ip: str, netmask: str) -> str:
    lst_ip = split_octects(ip)
    lst_netmask = split_octects(netmask)
    
    return ('.').join([str(i & ~j) for (i, j) in zip(lst_ip, lst_netmask)])

def n_hosts(netmask: str) -> str:
    # Bitwise NOT does not work well here, so I am using a XOR with all 1's (255)
    # RFC1878
    return 2 ** ''.join([bin(255 ^ int(oct)) for oct in netmask.split('.')]).count('1')
