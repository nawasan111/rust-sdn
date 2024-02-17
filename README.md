## Rust with SDN

### Description

OpenFlow version 1.0

### mininet

```
$ sudo mn --controller=remote,ip=127.0.0.1 --switch=ovsk,protocols=OpenFlow10 --topo=single,4 --mac
```