import socket
import time

SERVER_HOST = '192.168.1.1'
SERVER_PORT = 12345
BUFFER_SIZE = 1024

def handle_client(client_socket):
    start_time = time.time()
    total_data = 0

    while True:
        data = client_socket.recv(BUFFER_SIZE)
        if not data:
            break
        total_data += len(data)

    end_time = time.time()
    total_time = end_time - start_time
    bandwidth = total_data / total_time / 1024 / 1024  # Bandwidth in MB/s
    latency = total_time * 1000  # Latency in milliseconds

    print(f"Bandwidth: {bandwidth:.2f} MB/s")
    print(f"Latency: {latency:.2f} ms")

    client_socket.close()

def start_server():
    server_socket = socket.socket(socket.AF_INET, socket.SOCK_STREAM)
    server_socket.bind((SERVER_HOST, SERVER_PORT))
    server_socket.listen(1)
    print(f"Server listening on {SERVER_HOST}:{SERVER_PORT}")

    prio = 0
    while prio < 4:
        client_socket, address = server_socket.accept()
        print(f"Connected to client: {address}")
        print(f"Priority: {prio}")
        handle_client(client_socket)
        prio += 1

if __name__ == '__main__':
    start_server()
