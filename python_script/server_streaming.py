import socket
import time
import argparse

SERVER_HOST = '192.168.1.1'
BUFFER_SIZE = 1024

FRAME_SIZE = 6000000

parser = argparse.ArgumentParser(description="Socket Server with Python.")

parser.add_argument("--port", "-p", type=int, required=True)
args = parser.parse_args()


def handle_client(client_socket):
    total_data = 0
    start_time = time.time()
    while True:
    	data = client_socket.recv(BUFFER_SIZE)
    	if not data:
    	    break
    	total_data += len(data)
    	
    	if total_data > FRAME_SIZE:

    	    end_time = time.time()
    	    total_time = end_time - start_time
    	    bandwidth = total_data / total_time / 1024 / 1024  # Bandwidth in MB/s
    	    latency = total_time * 1000  # Latency in milliseconds

    	    print(f"Bandwidth: {bandwidth:.2f} MB/s")
    	    print(f"Latency: {latency:.2f} ms")
    	    total_data = 0
    	    start_time = time.time()

    client_socket.close()

def start_server(args):
    server_socket = socket.socket(socket.AF_INET, socket.SOCK_STREAM)
    server_socket.bind((SERVER_HOST, args.port))
    server_socket.listen(1)
    print(f"Server listening on {SERVER_HOST}:{args.port}")

    client_socket, address = server_socket.accept()
    print(f"Connected to client: {address}")
    handle_client(client_socket)

if __name__ == '__main__':
    start_server(args)
