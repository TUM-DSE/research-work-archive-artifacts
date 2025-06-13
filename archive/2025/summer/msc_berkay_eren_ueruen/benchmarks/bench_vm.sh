#!/bin/bash
SERVER_PORT=2345
CLIENT_PORT=2346

# unbind the GPU from the host
sudo ../qemu_scripts/unbind.sh;

# create directories
mkdir -p results # this is redundant but nice to have here in case this script is called standalone
mkdir -p server_signals

# start VMs
cd vm_server
nixos-shell server.nix &

# Connect to LLMOS and start the server
while ! ssh -o StrictHostKeyChecking=no -p $SERVER_PORT root@localhost 'echo "Server VM live"'
do
    sleep 3
done

while ! ssh -o StrictHostKeyChecking=no -p $SERVER_PORT root@localhost 'bash /vm_scripts/server.sh' > server_output
do
    sleep 3
done &

cd ../vm_client
nixos-shell client.nix &

# Start the client
while ! ssh -o StrictHostKeyChecking=no -p $CLIENT_PORT root@localhost 'echo "Client VM live"'
do
    sleep 3
done

while ! ssh -o StrictHostKeyChecking=no -p $CLIENT_PORT root@localhost 'bash /vm_scripts/client.sh' > client_output
do
    sleep 3
done &

echo "SUCCESS!"
