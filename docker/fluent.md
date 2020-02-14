```bash
docker run -d -p 24224:24224 -p 24224:24224/udp -v /data:/fluentd/log fluent/fluentd:v1.3-debian-1
```

