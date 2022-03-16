# Toy DNS

A minimal prototype for DNS functionality identified in RFD 248.

## Usage

Run the server

```
cargo run --bin toy-dns-server -- --config-file example-config.toml
```

Add some records

```shell
# AAAA
./target/debug/toyadm add-aaaa pizza fd00::1701

# SRV
./target/debug/toyadm add-srv blueberry 47 47 47 muffin
```

View records through admin interface

```shell
./target/debug/toyadm list-records
[
    DnsKv {
        key: DnsRecordKey {
            name: "blueberry",
        },
        record: Srv(
            Srv {
                port: 47,
                prio: 47,
                target: "muffin",
                weight: 47,
            },
        ),
    },
    DnsKv {
        key: DnsRecordKey {
            name: "pizza",
        },
        record: Aaaa(
            fd00::1701,
        ),
    },
]
```

View records through `dig`.

```shell
dig -p 4753 pizza @localhost

; <<>> DiG 9.10.6 <<>> -p 4753 pizza @localhost
;; global options: +cmd
;; Got answer:
;; ->>HEADER<<- opcode: QUERY, status: NOERROR, id: 1395
;; flags: qr rd; QUERY: 1, ANSWER: 1, AUTHORITY: 0, ADDITIONAL: 0
;; WARNING: recursion requested but not available

;; QUESTION SECTION:
;pizza.				IN	A

;; ANSWER SECTION:
pizza.			0	IN	AAAA	fd00::1701

;; Query time: 1 msec
;; SERVER: ::1#4753(::1)
;; WHEN: Sat Mar 12 08:40:52 PST 2022
;; MSG SIZE  rcvd: 56
```

```shell
dig -p 4753 blueberry @localhost

; <<>> DiG 9.10.6 <<>> -p 4753 blueberry @localhost
;; global options: +cmd
;; Got answer:
;; ->>HEADER<<- opcode: QUERY, status: NOERROR, id: 24764
;; flags: qr rd; QUERY: 1, ANSWER: 1, AUTHORITY: 0, ADDITIONAL: 0
;; WARNING: recursion requested but not available

;; QUESTION SECTION:
;blueberry.			IN	A

;; ANSWER SECTION:
blueberry.		0	IN	SRV	47 47 47 muffin.

;; Query time: 1 msec
;; SERVER: ::1#4753(::1)
;; WHEN: Sat Mar 12 08:41:16 PST 2022
;; MSG SIZE  rcvd: 62
```
