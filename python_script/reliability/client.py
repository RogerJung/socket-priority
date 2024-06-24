import socket
import time
import json
from multiprocessing import Process, Manager
import os

server_ip = os.getenv('SERVER_IP')

# Check if the SERVER_IP variable is set
if server_ip is not None:
    print(f"The SERVER_IP is: {server_ip}")
else:
    print("SERVER_IP environment variable is not set.")

client_ip = os.getenv('CLIENT_IP')

# Check if the CLIENT_IP variable is set
if client_ip is not None:
    print(f"The CLIENT_IP is: {client_ip}")
else:
    print("CLIENT_IP environment variable is not set.")

def send_data(host, port, message, interval, duration, results, index):
    sock = socket.socket(socket.AF_INET, socket.SOCK_DGRAM)
    if index < 4:
        sock.setsockopt(socket.SOL_SOCKET, socket.SO_PRIORITY, index)
    else:
        sock.setsockopt(socket.SOL_SOCKET, socket.SO_PRIORITY, 3)
    sock.bind((client_ip, port))
    server_address = (host, port)

    start_time = time.time()
    end_time = start_time + duration
    sent_packets = 0
    
    # if index == 0:
    #     interval = 0.0005
    # elif index == 1:
    #     interval = 0.0005
    # elif index == 2:
    #     interval = 0.00005
    # elif index == 3:
    #     interval = 0.00005

    if index == 0:
        interval = 0.002
    elif index == 1:
        interval = 0.002
    elif index == 2:
        interval = 0.0002
    elif index == 3:
        interval = 0.0002
    else:
        interval = 0.00005

    ## Interval v.s. bitrate
    #  Packet size : 1024 bytes
    #  4 ports
    #  0         -> 9.38 Gbit/s
    #  0.002     -> 4   Mbit/s
    #  0.0002    -> 32  Mbit/s
    #  0.0001    -> 54  Mbit/s
    #  0.00005   -> 78  Mbit/s
    #  0.00001   -> 95  Mbit/s
    #  0.000001  -> 100 Mbit/s
    #  0.0000001 -> 145 Mbit/s

    ## Interval v.s. bitrate
    #  Packet size: 1024 bytes
    #  7 ports
    #  0.0001   -> 55  Mbit/s (100%)
    #  0.00005  -> 80  Mbit/s (84%, 95%)
    #  0.000025 -> 100 Mbit/s (72%)
    #  0.00002  -> 110 Mbit/s
    #  0.00001  -> 120 Mbit/s
    #  0.000001 -> 145 Mbit/s

        
    message = json.dumps(message)

    while time.time() < end_time:
        sock.sendto(message.encode('utf-8'), server_address)
        sent_packets += 1
        time.sleep(interval)
    

    end_message = json.dumps(['end', sent_packets])

    for i in range(1000):
        sock.sendto(end_message.encode('utf-8'), server_address)
    	    
    results[index] = sent_packets
    sock.close()

def main():
    host = server_ip
    ports = [8000, 8001, 8002, 8003, 10000, 10001, 10002]  # 多个端口进行测试
    bitrate = 1 * 1024 * 1024  # 1 Mbits/sec
    duration = 10               # 发送数据的持续时间为10秒
    msg = ""
    message = [msg.zfill(1020)]
    
    interval = len(message) * 8 / bitrate  # 计算发送间隔
    
    manager = Manager()
    results = manager.list([0] * len(ports))


    processes = []
    for i, port in enumerate(ports):
        process = Process(target=send_data, args=(host, port, message, interval, duration, results, i))
        processes.append(process)
        process.start()
    
    for process in processes:
        process.join()
    
    for i, port in enumerate(ports):
        print(f"Port {port}: Sent {results[i]} packets")

if __name__ == "__main__":
    main()

