#!/usr/bin/env python3
import socket
import sys
import threading

# 全局计数器
nums = 0
lock = threading.Lock()

def send_message(ip, port, message):
    global nums
    try:
        with socket.socket(socket.AF_INET, socket.SOCK_STREAM) as s:
            s.connect((ip, port))
            s.sendall(message.encode())
            data = s.recv(1024)
            with lock:  # 确保计数更新线程安全
                nums += 1
                print(f"Received[{nums}]: {data.decode()}")
    except Exception as e:
        print(f"Error: {e}")

def main():
    if len(sys.argv) != 4:
        print("Usage: python socket_client.py <IP:PORT> <MESSAGE> <CLIENT_COUNT>")
        sys.exit(1)

    # Parse arguments
    ip_port = sys.argv[1].split(":")
    ip = ip_port[0]
    port = int(ip_port[1])
    message = sys.argv[2]
    client_count = int(sys.argv[3])

    # Create multiple clients to send messages
    threads = []
    for _ in range(client_count):
        thread = threading.Thread(target=send_message, args=(ip, port, message))
        threads.append(thread)
        thread.start()

    for thread in threads:
        thread.join()

if __name__ == "__main__":
    main()
