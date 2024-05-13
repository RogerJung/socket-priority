import socket
import time

SERVER_HOST = '192.168.1.1'
SERVER_PORT = 12345
BUFFER_SIZE = 1024
DATA_SIZE = 100 * 1024 * 1024  # 100 MB of data

def send_data(client_socket):
    data = b'0' * BUFFER_SIZE
    total_data_sent = 0

    while total_data_sent < DATA_SIZE:
        data_sent = client_socket.send(data)
        total_data_sent += data_sent

    client_socket.shutdown(socket.SHUT_WR)

def start_client(prio):
    client_socket = socket.socket(socket.AF_INET, socket.SOCK_STREAM)
    client_socket.setsockopt(socket.SOL_SOCKET, socket.SO_PRIORITY, prio)
    client_socket.connect((SERVER_HOST, SERVER_PORT))
    print(f"Connected to server: {SERVER_HOST}:{SERVER_PORT}")

    start_time = time.time()
    send_data(client_socket)
    end_time = time.time()

    total_time = end_time - start_time
    print(f"Total time: {total_time:.2f} seconds")

    client_socket.close()

if __name__ == '__main__':
    for i in range(4):
        start_client(i)