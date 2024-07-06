curl 127.0.0.1:8090
```
Request: [
    "GET / HTTP/1.1",
    "Host: 127.0.0.1:8090",
    "User-Agent: curl/7.68.0",
    "Accept: */*",
]
```
刚才的文本挺长的，但其实符合以下的格式:

Method Request-URI HTTP-Version                                         "GET / HTTP/1.1",
headers CRLF                                                            "Host: 127.0.0.1:8090",
message-body
第一行 Method 是请求的方法，例如 GET、POST 等，Request-URI 是该请求希望访问的目标资源路径，例如 /、/hello/world 等
类似 JSON 格式的数据都是 HTTP 请求报头 headers，例如 "Host: 127.0.0.1:7878"
至于 message-body 是消息体， 它包含了用户请求携带的具体数据，例如更改用户名的请求，就要提交新的用户名数据，至于刚才的 GET 请求，它是没有 message-body 的