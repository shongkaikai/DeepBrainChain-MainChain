### 查询当前高度Hash

```bash
curl http://localhost:9933 -H "Content-Type:application/json;charset=utf-8" -d   '{
     "jsonrpc":"2.0",
      "id":1,
      "method":"chain_getBlockHash",
      "params": []
    }'
{"jsonrpc":"2.0","result":"0x987898b4b27051d13ee47f45eefe053c9f09a37393c9a601ac27acb27e6a265e","id":1}
```

### 查询在线奖励系统的信息

```bash
curl http://localhost:9933 -H "Content-Type:application/json;charset=utf-8" -d   '{
     "jsonrpc":"2.0",
      "id":1,
      "method":"onlineProfile_getOpInfo",
      "params": []
}'
```

+ 返回信息：

```json
{"jsonrpc":"2.0","result":{"totalCalcPoints":0,"totalGpuNum":0,"totalStake":"0","totalStaker":0},"id":1}
```


### 查询共有多少矿工

```json
curl http://localhost:9933 -H "Content-Type:application/json;charset=utf-8" -d   '{
     "jsonrpc":"2.0",
      "id":1,
      "method":"onlineProfile_getStakerNum",
      "params": []
}'
```

###  分页查询矿工质押详情

```json
{
     "jsonrpc":"2.0",
      "id":1,
      "method":"onlineProfile_getStakerListInfo",
      "params": ["0xe83d6e9b7c27c10a280b6544a8a04c81db946dd2fdf9c8fdea499e464c6d8306", 0, 7]
}
```

+ 参数说明：当前高度Hash，cur_page, per_page

返回结果：

```json
{
   "jsonrpc": "2.0",
   "result": [
      {
         "calcPoints": 0,
         "gpuNum": 0,
         "gpuRentRate": 0,
         "stakerAccount": "5CiPPseXPECbkjWCa6MnjNokrgYjMqmKndv2rSnekmSK2DjL",
         "stakerName": [],
         "totalReward": "0"
      },
      {
         "calcPoints": 0,
         "gpuNum": 0,
         "gpuRentRate": 0,
         "stakerAccount": "5HpG9w8EBLe5XCrbczpwq5TSXvedjrBGCwqxK1iQ7qUsSWFc",
         "stakerName": [],
         "totalReward": "0"
      },
      {
         "calcPoints": 0,
         "gpuNum": 0,
         "gpuRentRate": 0,
         "stakerAccount": "5FHneW46xGXgs5mUiveU4sbTyGBzmstUspZC92UhjJM694ty",
         "stakerName": [],
         "totalReward": "0"
      },
      {
         "calcPoints": 0,
         "gpuNum": 0,
         "gpuRentRate": 0,
         "stakerAccount": "5CiPPseXPECbkjWCa6MnjNokrgYjMqmKndv2rSnekmSK2DjL",
         "stakerName": [],
         "totalReward": "0"
      },
      {
         "calcPoints": 0,
         "gpuNum": 0,
         "gpuRentRate": 0,
         "stakerAccount": "5CiPPseXPECbkjWCa6MnjNokrgYjMqmKndv2rSnekmSK2DjL",
         "stakerName": [],
         "totalReward": "0"
      },
      {
         "calcPoints": 0,
         "gpuNum": 0,
         "gpuRentRate": 0,
         "stakerAccount": "5CiPPseXPECbkjWCa6MnjNokrgYjMqmKndv2rSnekmSK2DjL",
         "stakerName": [],
         "totalReward": "0"
      },
      {
         "calcPoints": 0,
         "gpuNum": 0,
         "gpuRentRate": 0,
         "stakerAccount": "5CiPPseXPECbkjWCa6MnjNokrgYjMqmKndv2rSnekmSK2DjL",
         "stakerName": [],
         "totalReward": "0"
      }
   ],
   "id": 1
}
```

---

---

### 查询机器列表

```bash
curl http://localhost:9933 -H "Content-Type:application/json;charset=utf-8" -d   '{
     "jsonrpc":"2.0",
      "id":1,
      "method":"onlineProfile_getMachineList",
      "params": []
    }'
```

+ 返回信息：

```json
{"jsonrpc":"2.0","result":{"bonded_machine":[],"bonding_machine":[],"booked_machine":[],"ocw_confirmed_machine":[],"waiting_hash":[]},"id":1}
```

### 查询机器信息

```
curl http://localhost:9933 -H "Content-Type:application/json;charset=utf-8" -d   '{
     "jsonrpc":"2.0",
      "id":1,
      "method":"onlineProfile_getMachineInfo",
      "params": ["0x3267667070334d4142344171325a50455537326e655a5456635a6b627a447a5839366f7039643366766933"]
    }'
```

注： machine_id应该转为hex

```python
a = "2gfpp3MAB4Aq2ZPEU72neZTVcZkbzDzX96op9d3fvi3"
>>> b = bytes(a, 'utf-8')
>>> for c in b:
     print(c，end=',')

[50,103,102,112,112,51,77,65,66,52,65,113,50,90,80,69,85,55,50,110,101,90,84,86,99,90,107,98,122,68,122,88,57,54,111,112,57,100,51,102,118,105,51]

>>> a = "2gfpp3MAB4Aq2ZPEU72neZTVcZkbzDzX96op9d3fvi3"
>>> b = str.encode(a)
>>> b
b'2gfpp3MAB4Aq2ZPEU72neZTVcZkbzDzX96op9d3fvi3'
>>> b.hex()
'3267667070334d4142344171325a50455537326e655a5456635a6b627a447a5839366f7039643366766933'

返回结果：

```json
{"jsonrpc":"2.0","result":{"bondingHeight":"139","machineInfoDetail":{"committee_upload_info":{"calc_point":0,"cpu_core_num":0,"cpu_rate":0,"cpu_type":[],"cuda_core":0,"gpu_mem":0,"gpu_num":0,"gpu_type":[],"hard_disk":0,"is_support":false,"machine_id":[],"mem_num":0,"rand_str":[]},"staker_customize_info":{"download_net":0,"images":[],"latitude":0,"left_change_time":3,"longitude":0,"upload_net":0}},"machineOwner":"5FHneW46xGXgs5mUiveU4sbTyGBzmstUspZC92UhjJM694ty","machinePrice":0,"machineStatus":"OcwConfirming","rewardDeadline":"0","stakeAmount":"0"},"id":1}
```

### 查询委员会被分配的机器ID
```bash
 curl http://localhost:9933 -H "Content-Type:application/json;charset=utf-8" -d   '{
     "jsonrpc":"2.0",
      "id":1,
      "method":"leaseCommittee_getCommitteeMachineList",
      "params": ["0x2b7eca733ed4e538339c41cb454092261d722c0ee355ce1744daaee22388c044", "5GrwvaEF5zXb26Fz9rcQpDWS57CtERHpNehXCPcNoHGKutQY"]
    }'
```

参数说明： blockHash, committeeAccountId

返回结果:

```json
{"jsonrpc":"2.0","result":{"booked_machine":[],"confirmed_machine":[],"hashed_machine":[],"online_machine":[]},"id":1}

```

### 查询委员会验证时间
```bash
curl http://localhost:9933 -H "Content-Type:application/json;charset=utf-8" -d   '{
     "jsonrpc":"2.0",
      "id":1,
      "method":"leaseCommittee_getCommitteeOps",
      "params": ["0x0fa478725b4dead40343a9a2b543ca6d3c1f52692493b99e6350be1b539d4e52", "5GrwvaEF5zXb26Fz9rcQpDWS57CtERHpNehXCPcNoHGKutQY", [50,103,102,112,112,51,77,65,66,52,65,113,50,90,80,69,85,55,50,110,101,90,84,86,99,90,107,98,122,68,122,88,57,54,111,112,57,100,51,102,118,105,51]]
    }'
```

参数说明： blockHash, committeeAccountId, machineId

返回结果：

```json
{"jsonrpc":"2.0","result":{"bookedTime":"0","confirmHash":[0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0],"confirmTime":"0","hashTime":"0","machineInfo":{"calc_point":0,"cpu_core_num":0,"cpu_rate":0,"cpu_type":[],"cuda_core":0,"gpu_mem":0,"gpu_num":0,"gpu_type":[],"hard_disk":0,"is_support":false,"machine_id":[],"mem_num":0,"rand_str":[]},"machineStatus":"Booked","stakedDbc":"0"},"id":1}
```

### 查询用户租用机器订单

```bash
curl http://localhost:9933 -H "Content-Type:application/json;charset=utf-8" -d   '{
     "jsonrpc":"2.0",
      "id":1,
      "method":"rentMachine_getRentOrder",
      "params": ["0xe602f649549317c389f9f02dea97d552a026c193c0a2584bc1f668efd1134de1","5GrwvaEF5zXb26Fz9rcQpDWS57CtERHpNehXCPcNoHGKutQY", [50,103,102,112,112,51,77,65,66,52,65,113,50,90,80,69,85,55,50,110,101,90,84,86,99,90,107,98,122,68,122,88,57,54,111,112,57,100,51,102,118,105,51]]
    }'
```

参数说明： BlockHash, AccountId, MachineId

返回结果：
```json
{"jsonrpc":"2.0","result":{"confirmRent":"0","rentEnd":"0","rentStart":"0","renter":"5C4hrfjw9DjXZTzV3MwzrrAr9P1MJhSrvWGWqi1eSuyUpnhM","stakeAmount":"0"},"id":1}
```

### 查询用户的租用列表

```
curl http://localhost:9933 -H "Content-Type:application/json;charset=utf-8" -d   '{
     "jsonrpc":"2.0",
      "id":1,
      "method":"rentMachine_getRentList",
      "params": ["0xe602f649549317c389f9f02dea97d552a026c193c0a2584bc1f668efd1134de1","5GrwvaEF5zXb26Fz9rcQpDWS57CtERHpNehXCPcNoHGKutQY"]
    }'
```

参数说明：BlockHash, AccountId

返回结果： ["machine_id1", "machine_id2"]


---

---

### 查询矿工的质押数量

```bash
curl http://localhost:9933 -H "Content-Type:application/json;charset=utf-8" -d   '{
     "jsonrpc":"2.0",
      "id":1,
      "method":"onlineProfile_getStakerInfo",
      "params": ["0x1a6cf8f12ea4d3ac5c4ac2d8a0e91a08fc1e416917e4f16a5328bd775a0f1919","5GjrZ4iQdxZhAKKjNooMruGqBwH5CwbJ6Un6Cinc7j45zToE"]
    }'
```

+ "params"为：["Block_hash", "AccountId"]

+ 返回信息：

```json
{"jsonrpc":"2.0","result":{"calcPoints":0,"gpuNum":0,"totalReward":"0"},"id":1}
```

### 分页查询矿工账户ID
```bash
curl http://localhost:9933 -H "Content-Type:application/json;charset=utf-8" -d   '{
     "jsonrpc":"2.0",
      "id":1,
      "method":"onlineProfile_getStakerList",
      "params": ["0xbd7e44182d643e9da10402ceaa4bcb17c5995550da73bb9187f73081903cb567", 7, 7]
    }'
```

+ 返回信息：
```json
{"jsonrpc":"2.0","result":["5DhR2dxiPZquPhFjfPzFg5jZENdr375hbX643kr9FBXMVa2z", "5FEmxL86rj2av2X1p7bVvLWZx7CSdFDUmhmWMF1EjUeoB9wg", "5Ebn8seCXL3cj2PDpsgTpXAnuvH24RbSgpxnCmKGxcwANFQ8"],"id":1}
```

### 查询地址对应的账户名称
```bash
curl http://localhost:9933 -H "Content-Type:application/json;charset=utf-8" -d   '{
     "jsonrpc":"2.0",
      "id":1,
      "method":"onlineProfile_getStakerIdentity",
      "params": ["0xbc5d40d87d829a76eb987bb388d05e3d848eec7d91009f3efd30de67a229f116", "5FHneW46xGXgs5mUiveU4sbTyGBzmstUspZC92UhjJM694ty"]
    }'
```

+ 返回信息：
```json
{"jsonrpc":"2.0","result":[98,111,98],"id":1}
```

python 中decode得到账户名称：
```
>>> bytes([98,111,98]).decode('utf-8')
'bob'
```