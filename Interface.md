# Uni-arts Network 

我们的区块链技术主要是基于substrat的技术框架进行的开发。

## 接口host

1.wss
```bash
wss://mainnet.jiamashexian.com
```
2. https
```bash
https://mainnet.jiamashexian.com:9933
```

## 接口

### 创建collection

pallet: nft  
function: create_collection  
参数:
|  collection_name   | collection_description  | token_prefix  | mode  |
|  ----  | ----  | ----  | ----  |
| Vec<U16>  | Vec<U16> | Vec<U8>  | CollectionMode |
| 名称  | 描述 | 暂时没有用  | 模式(一般可以是ReFungible，ReFungible第一个参数是最多多少NFT，第二个参数是每个NFT最多多少份) |

Log  
Created(u64, u8, AccountId)  

### 增加collection的admin

pallet: nft  
function: add_collection_admin  
参数:
|  collection_id   | new_admin_id  |
|  ----  | ----  |
| u64 | AccountId |
| collection_id  | 新的admin的id |

### 创建item(NFT)

pallet: nft  
function: create_item  
参数:
|  collection_id   | properties  |  owner   | royalty_rate  | royalty_expired_at  |
|  ----  | ----  | ----  | ----  | ----  |
| u64 | Vec<u8> | AccountId | u64 | BlockNumber |
| collection_id  | 属性(metadata) | 所有者  | 版税 | 版税有效期  |

Log  
ItemCreated(collection_id, item_id)  

### 流转(NFT)

pallet: nft  
function: transfer  
参数:
|  recipient   | collection_id  |  item_id   | value  |
|  ----  | ----  | ----  | ----  |
| AccountId | u64 | u64 | u64 |
| 接收者  | collection_id | item_id  | 数量 

Log  
ItemTransfer(collection_id, item_id, value, sender, recipient)


## 参考文档

 [Substrate Doc](https://docs.substrate.io/v3/getting-started/overview/)

[链的编码](https://docs.substrate.io/v3/advanced/scale-codec/)

[php的编码](https://github.com/gmajor-encrypt/php-scale-codec)

[php的rpc](https://github.com/gmajor-encrypt/php-substrate-api)

[链的JSON文件](https://github.com/uni-arts-chain/uni-arts-network/blob/master/runtime/fuxi/types.json)


