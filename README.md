# Notes

Endianess seems to be big endian (network endianess)

# Important Locations

* Private Server IP: PhotonNetwork.ConnectUsingSettings

# Communication

## Packet Locations

### First Packet
Issued in `ExitGames.Client.Photon.Enetpeer.SendOutgoingCommands()`

data[0x0..0x2] = peer id
data[0x2] = use crc
data[0x3] = udp command count
data[0x4..0x8] = time
data[0x8..0xc] = challenge
data[0xc..0x38] = udp command (?)