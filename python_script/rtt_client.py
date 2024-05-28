import socket
import time
import argparse
import os

HOST=os.getenv('SERVER_IP')

parser = argparse.ArgumentParser(description="Socket Client with Python.")

parser.add_argument("--port", "-p", type=int, required=True)
parser.add_argument("--priority", "-i", type=int, required=True)
args = parser.parse_args()

def measure_rtt(message, timeout=2):
    while True:
        try:
            # Create a TCP/IP socket
            sock = socket.socket(socket.AF_INET, socket.SOCK_STREAM)
            # sock.setsockopt(socket.SOL_SOCKET, socket.SO_PRIORITY, args.priority)
            sock.settimeout(timeout)

            # Connect the socket to the server
            sock.connect((HOST, args.port))

            # Send data
            start_time = time.time()
            sock.sendall(message.encode())

            # Receive response
            data = sock.recv(1024)
            end_time = time.time()

            # Calculate RTT
            rtt = end_time - start_time
            rtt *= 1000
            print(f"Received: {data.decode()}")
            print(f"Round Trip Time (RTT): {rtt:.0f} ms")

        except socket.timeout:
            print("Request timed out.")
        except Exception as e:
            print(f"An error occurred: {e}")
        time.sleep(0.1)

if __name__ == "__main__":
    message = "ping"    # Message to send to the server
    
    measure_rtt(message)
