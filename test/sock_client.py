#!/usr/bin/env python3
import socket
import sys
import threading

# 全局计数器
nums = 0
lock = threading.Lock()

def send_message(client_id, ip, port, message):
    global nums
    try:
        message=message+f"{client_id}"
        # 1. 连接到服务器
        with socket.socket(socket.AF_INET, socket.SOCK_STREAM) as s:
            print(f"[连接 {client_id}] 正在连接到...")
            s.connect((ip, port))
            print(f"[连接 {client_id}] 成功连接到!!!")
            
            # 2. 发送消息
            print(f"[连接 {client_id}] 发送消息:...")
            s.sendall(message.encode())
            print(f"[连接 {client_id}] 消息发送成功!!!")
            
            # 3. 接收响应
            data = s.recv(1024)
            with lock:  # 确保计数更新线程安全
                nums += 1
                print(f"[连接 {client_id}] Received[{nums}]: {data.decode()}")
            
        # 4. 关闭连接
        print(f"[连接 {client_id}] 连接已关闭")
        
    except Exception as e:
        print(f"[连接 {client_id}] Error: {e}")

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
    for i in range(client_count):
        thread = threading.Thread(target=send_message, args=(i + 1, ip, port, message))
        threads.append(thread)
        thread.start()

    for thread in threads:
        thread.join()

if __name__ == "__main__":
    main()
