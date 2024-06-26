import socket
import time
import threading
import json
from multiprocessing import Process, Manager
import os
import json

server_ip = os.getenv('SERVER_IP')

# Check if the SERVER_IP variable is set
if server_ip is not None:
    print(f"The SERVER_IP is: {server_ip}")
    pass
else:
    print("SERVER_IP environment variable is not set.")

client_ip = os.getenv('CLIENT_IP')

# Check if the CLIENT_IP variable is set
if client_ip is not None:
    print(f"The CLIENT_IP is: {client_ip}")
    pass
else:
    print("CLIENT_IP environment variable is not set.")

ports = [8000, 8001, 8002, 8003, 10000, 10001, 10002]  # 多个端口进行测试
msg_ports = [8000, 8001, 8002, 8003]
duration = 10               # 接收数据的持续时间为10秒

def receive_data(port, results, index):
    sock = socket.socket(socket.AF_INET, socket.SOCK_DGRAM)
    sock.bind((server_ip, port))

    received_packets = 0
    start_time = time.time()
    timestamps = []

    while True:
        try:
            data, addr = sock.recvfrom(4096)
            if json.loads(data.decode('utf-8'))[0] == 'end':
                total_packets = json.loads(data.decode('utf-8'))[1]
                break
            received_packets += 1
            timestamps.append(time.time())
        except socket.timeout:
            break

    end_time = time.time()

    total_time = end_time - start_time
    packet_intervals = [t - s for s, t in zip(timestamps, timestamps[1:])]
    jitter = sum(abs(interval - total_time / received_packets) for interval in packet_intervals) / len(packet_intervals) if received_packets > 1 else 0
    reliability = received_packets / total_packets * 100

    results[index] = {
        "received_packets": received_packets,
        "jitter": jitter,
        "reliability": reliability,
    }
    
    client_address = (client_ip, port)
    msg = json.dumps([port, reliability])
    
    for i in range(100):
        sock.sendto(msg.encode('utf-8'), client_address)

    sock.close()



def main():

    manager = Manager()
    results = manager.list([{} for _ in ports])
    processes = []

    for i, port in enumerate(ports):
        process = Process(target=receive_data, args=(port, results, i))
        processes.append(process)
        process.start()

    for process in processes:
        process.join()

    for i, port in enumerate(msg_ports):
        print(f"Port {port}:")
        print(f"  Received packets: {results[i]['received_packets']}")
        print(f"  Jitter: {results[i]['jitter']:.6f} seconds")
        print(f"  Reliability: {results[i]['reliability']:.3f} %")

if __name__ == "__main__":
    main()

