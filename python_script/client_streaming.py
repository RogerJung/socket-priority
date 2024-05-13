import socket
import time
import argparse

SERVER_HOST = '192.168.1.1'
BUFFER_SIZE = 1024
# DATA_SIZE = 100 * 1024 * 1024  # 100 MB of data

parser = argparse.ArgumentParser(description="Socket Client with Python.")

parser.add_argument("--port", "-p", type=int, required=True)
parser.add_argument("--priority", "-i", type=int, required=True)
args = parser.parse_args()

def send_data(client_socket):
    data = b'0' * BUFFER_SIZE
    total_data_sent = 0

    while True:
        data_sent = client_socket.send(data)
        total_data_sent += data_sent

    client_socket.shutdown(socket.SHUT_WR)

def start_client(args):
    client_socket = socket.socket(socket.AF_INET, socket.SOCK_STREAM)
    client_socket.setsockopt(socket.SOL_SOCKET, socket.SO_PRIORITY, args.priority)
    client_socket.connect((SERVER_HOST, args.port))
    print(f"Connected to server: {SERVER_HOST}:{args.port}")

    start_time = time.time()
    send_data(client_socket)
    end_time = time.time()

    total_time = end_time - start_time
    print(f"Total time: {total_time:.2f} seconds")

    client_socket.close()

if __name__ == '__main__':
    start_client(args)
