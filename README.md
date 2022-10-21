# gluon

gluon is a replacement for the [Photon](https://doc.photonengine.com/en-us/realtime/current/connection-and-authentication/authentication/steam-auth)
endpoint which is ... questionable at best. Initially this project was created
to be able to host own [Interplanetary](https://store.steampowered.com/app/650220/Interplanetary_Enhanced_Edition)
servers, so you don't have to play together with potential hackers.

# Notes

Endianess seems to be big endian (network endianess)

# Important Locations

* Private Server IP: `PhotonNetwork.ConnectUsingSettings`
* Packet Receive Function: `ExitGames.Client.Photon.EnetPeer.ReceiveIncomingCommands`