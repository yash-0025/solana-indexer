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


- `solana-client`  - The official Solana create providing client implementation RpcClient to make JSON-RPC calls over HTTP
- `solana-sdk` - Defines fundamental on-chain primitives (Pubkey, Signature, Account , Instruction)
- `1.18` - Standard compatible release series matching modern Solan mainnet/devnet nodes


- In Solana , everything is an account . Solana smart contracts are 100% stateless. code accounts. All data (user balances, token mints, escrows vaults ) is stored in separate data accounts owned by those programs

- Before our indexer can decode or query anything , we need strongly typed domain models represnting what the solana ledger actually delivers . 

```
+--------------------------------------------------------------------+
|                         Indexer Pipeline                           |
|                                                                    |
|  Solana RPC / WS                                                   |
|        │ (Raw JSON-RPC / Bytes)                                    |
|        ▼                                                           |
|  [AccountSnapshot] ◄── Standardized Domain Envelope                |
|    ├── pubkey: Pubkey                                              |
|    ├── owner:  Pubkey                                              |
|    ├── lamports: u64                                               |
|    ├── data:   Vec<u8> ──► [Token / System Decoders in Module 1.7] |
|    └── slot:   u64                                                 |
|        │                                                           |
|        ▼                                                           |
|  In-Memory Storage / PostgreSQL / API                              |
+--------------------------------------------------------------------+

```

- By encapsulating on chain state into a generic `AccountSnapshot` our downstrema storage and indexing engines don't need to know which program created teh account 
- Encapsulation means bundling data and the function that operates on that data together while restricting direct access to the internal details


- On Solana blockchain , state is completely decoupled from executable code. Programs are stateless code accounts, mutable state lives entirely inside separate data accounts owned by those programs.
- When an indexer queries an RPC node (eg: via getAccountInfo) or receives WebSocket streaming notifications, the cluster transmits account metadata alongside a raw byte buffer. 
- BY separating generic account envelope metada from program-specific decoders, our downstream pipeline can store , sort and track accounts uniformly regardless of which program owns them

#### Represent a single point-in-time snapshot of an on-chain account. Store its 32 byte address its owning proram its lamport balance , the raw byte vector and the block slot. Provide a clean constructor to instantiate it and a helper to calculate its human readable SOL balance (diving lamports by 1,000,000,000)


