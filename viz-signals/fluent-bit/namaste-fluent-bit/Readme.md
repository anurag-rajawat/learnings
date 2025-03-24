# Namaste Fluent Bit

## Prerequisites
- [Fluent Bit](https://docs.fluentbit.io/manual/installation/getting-started-with-fluent-bit)
- Any HTTP client

## How to

Start Fluent Bit
```shell
fluent-bit -c namaste-fluent-bit/namaste.yaml
```

### Expected output
```shell
Fluent Bit v3.2.9
* Copyright (C) 2015-2025 The Fluent Bit Authors
* Fluent Bit is a CNCF sub-project under the umbrella of Fluentd
* https://fluentbit.io

______ _                  _    ______ _ _           _____  _____ 
|  ___| |                | |   | ___ (_) |         |____ |/ __  \
| |_  | |_   _  ___ _ __ | |_  | |_/ /_| |_  __   __   / /`' / /'
|  _| | | | | |/ _ \ '_ \| __| | ___ \ | __| \ \ / /   \ \  / /  
| |   | | |_| |  __/ | | | |_  | |_/ / | |_   \ V /.___/ /./ /___
\_|   |_|\__,_|\___|_| |_|\__| \____/|_|\__|   \_/ \____(_)_____/


[2025/03/24 22:58:17] [ info] [fluent bit] version=3.2.9, commit=, pid=30791
[2025/03/24 22:58:17] [ info] [storage] ver=1.5.2, type=memory, sync=normal, checksum=off, max_chunks_up=128
[2025/03/24 22:58:17] [ info] [simd    ] disabled
[2025/03/24 22:58:17] [ info] [cmetrics] version=0.9.9
[2025/03/24 22:58:17] [ info] [ctraces ] version=0.6.1
[2025/03/24 22:58:17] [ info] [input:dummy:dummy.0] initializing
[2025/03/24 22:58:17] [ info] [input:dummy:dummy.0] storage_strategy='memory' (memory only)
[2025/03/24 22:58:17] [ info] [sp] stream processor started
[2025/03/24 22:58:17] [ info] [output:stdout:stdout.0] worker #0 started
[{"date":"2025-03-24T17:28:18.232727Z","Namaste":"Fluent Bit"}]
```

### Health Check

Start Fluent Bit
```shell
fluent-bit -c namaste-fluent-bit/namaste-with-health.yaml
```

### Expected output
```shell
Fluent Bit v3.2.9
* Copyright (C) 2015-2025 The Fluent Bit Authors
* Fluent Bit is a CNCF sub-project under the umbrella of Fluentd
* https://fluentbit.io

______ _                  _    ______ _ _           _____  _____ 
|  ___| |                | |   | ___ (_) |         |____ |/ __  \
| |_  | |_   _  ___ _ __ | |_  | |_/ /_| |_  __   __   / /`' / /'
|  _| | | | | |/ _ \ '_ \| __| | ___ \ | __| \ \ / /   \ \  / /  
| |   | | |_| |  __/ | | | |_  | |_/ / | |_   \ V /.___/ /./ /___
\_|   |_|\__,_|\___|_| |_|\__| \____/|_|\__|   \_/ \____(_)_____/


[2025/03/24 23:06:47] [ info] [fluent bit] version=3.2.9, commit=, pid=31951
[2025/03/24 23:06:47] [ info] [storage] ver=1.5.2, type=memory, sync=normal, checksum=off, max_chunks_up=128
[2025/03/24 23:06:47] [ info] [simd    ] disabled
[2025/03/24 23:06:47] [ info] [cmetrics] version=0.9.9
[2025/03/24 23:06:47] [ info] [ctraces ] version=0.6.1
[2025/03/24 23:06:47] [ info] [input:dummy:dummy.0] initializing
[2025/03/24 23:06:47] [ info] [input:dummy:dummy.0] storage_strategy='memory' (memory only)
[2025/03/24 23:06:47] [ info] [output:stdout:stdout.0] worker #0 started
[2025/03/24 23:06:47] [ info] [http_server] listen iface=0.0.0.0 tcp_port=2020
[2025/03/24 23:06:47] [ info] [sp] stream processor started
[{"date":"2025-03-24T17:36:48.196589Z","Namaste":"Fluent Bit"}]
[{"date":"2025-03-24T17:36:49.197409Z","Namaste":"Fluent Bit"}]
```

### Checking Health
Using [httpie](https://github.com/httpie/cli)
```shell
http :2020/api/v1/health
```

```shell
HTTP/1.1 200 OK
Date: Mon, 24 Mar 2025 17:37:43 GMT
Server: Monkey/1.8.2
Transfer-Encoding: chunked

ok
```

### Metrics

> [!TIP]
> `/v2/metrics` endpoint provided Prometheus formatted data and more than `/v1/metrics` endpoint.

```shell
http :2020/api/v1/metrics
```

```json
{
  "input": {
    "dummy.0": {
      "records": 434,
      "bytes": 17794
    }
  },
  "filter": {},
  "output": {
    "stdout.0": {
      "proc_records": 433,
      "proc_bytes": 17753,
      "errors": 0,
      "retries": 0,
      "retries_failed": 0,
      "dropped_records": 0,
      "retried_records": 0
    }
  }
}
```
