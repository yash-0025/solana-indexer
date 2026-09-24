# NOTES WHILE BUILDING INDEXER

- An indexer does not validate blocks or participate in consensus; it connects downstream to an RPC or validator node, polls or streams raw ledger data, decodes byte buffers into typed domain reprentations and indexes them into storage


#### Architecture 
```
+-----------------------------------------------------------+
|                   Phase 1: CLI Indexer                    |
|                                                           |
|  [src/main.rs]                                            |
|       │                                                   |
|       ▼                                                   |
|  [RpcClient::new()]  ──(HTTP JSON-RPC)──►  Solana Cluster |
|       │                                    (Devnet / RPC) |
|       ▼                                         │         |
|  rpc.get_version()   ◄──(RpcVersionResponse)────┘         |
|       │                                                   |
|       ▼                                                   |
|  Terminal Banner + Cluster Verified                       |
+-----------------------------------------------------------+

```

