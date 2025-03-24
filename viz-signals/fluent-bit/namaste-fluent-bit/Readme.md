# Namaste Fluent Bit

## How to

1. Install [Fluent Bit](https://docs.fluentbit.io/manual/installation/getting-started-with-fluent-bit)
2. Run it
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