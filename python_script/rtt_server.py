import socket
import argparse

HOST='192.168.225.73'

parser = argparse.ArgumentParser(description="Socket Server with Python.")

parser.add_argument("--port", "-p", type=int, required=True)
args = parser.parse_args()

def start_server():
    # Create a TCP/IP socket
    server_socket = socket.socket(socket.AF_INET, socket.SOCK_STREAM)

    # Bind the socket to the address and port
    server_socket.bind((HOST, args.port))

    # Listen for incoming connections
    server_socket.listen(1)
    print(f"Server listening on {HOST}:{args.port}")

    while True:
        # Wait for a connection
        client_socket, client_address = server_socket.accept()
        try:
            print(f"Connection from {client_address}")

            # Receive the data
            data = client_socket.recv(1024)
            print(f"Received: {data.decode()}")

            # Send the data back to the client
            client_socket.sendall(data)
            print("Sent back the data")

        finally:
            # Clean up the connection
            client_socket.close()

if __name__ == "__main__":
    start_server()
