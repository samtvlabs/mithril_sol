# Verifying Mithril Aggregate Signatures

This repository contains a proof of concept for verifying mitril aggregated signature.

## Running POC

In order to run the test , install the [kurtosis-cli](https://docs.kurtosis.com/install/) and run

```shell
make testnet
```

If succesful , you should see the following output on your screen

```shell
========================================== User Services ==========================================
UUID           Name                                             Ports                                         Status
d9b9c576dc4a   cl-1-lighthouse-reth                             http: 4000/tcp -> http://127.0.0.1:32822      RUNNING
                                                                metrics: 5054/tcp -> http://127.0.0.1:32821   
                                                                tcp-discovery: 9000/tcp -> 127.0.0.1:32820    
                                                                udp-discovery: 9000/udp -> 127.0.0.1:32776    
61e75fe87f18   cl-1-lighthouse-reth-validator                   http: 5042/tcp -> 127.0.0.1:32824             RUNNING
                                                                metrics: 5064/tcp -> http://127.0.0.1:32823   
68ec7b13f59e   cl-2-lighthouse-reth                             http: 4000/tcp -> http://127.0.0.1:32827      RUNNING
                                                                metrics: 5054/tcp -> http://127.0.0.1:32826   
                                                                tcp-discovery: 9000/tcp -> 127.0.0.1:32825    
                                                                udp-discovery: 9000/udp -> 127.0.0.1:32777    
a0e3afe3cbb6   cl-2-lighthouse-reth-validator                   http: 5042/tcp -> 127.0.0.1:32829             RUNNING
                                                                metrics: 5064/tcp -> http://127.0.0.1:32828   
6e3e39061916   el-1-reth-lighthouse                             engine-rpc: 8551/tcp -> 127.0.0.1:32812       RUNNING
                                                                metrics: 9001/tcp -> 127.0.0.1:32811          
                                                                rpc: 8545/tcp -> 127.0.0.1:32814              
                                                                tcp-discovery: 30303/tcp -> 127.0.0.1:32810   
                                                                udp-discovery: 30303/udp -> 127.0.0.1:32774   
                                                                ws: 8546/tcp -> 127.0.0.1:32813               
d8f029936103   el-2-reth-lighthouse                             engine-rpc: 8551/tcp -> 127.0.0.1:32817       RUNNING
                                                                metrics: 9001/tcp -> 127.0.0.1:32816          
                                                                rpc: 8545/tcp -> 127.0.0.1:32819              
                                                                tcp-discovery: 30303/tcp -> 127.0.0.1:32815   
                                                                udp-discovery: 30303/udp -> 127.0.0.1:32775   
                                                                ws: 8546/tcp -> 127.0.0.1:32818               
e20f9e2f31e4   prelaunch-data-generator-cl-genesis-data         <none>                                        RUNNING
a6336b4aa44b   prelaunch-data-generator-cl-validator-keystore   <none>                                        RUNNING
f872959ac93d   prelaunch-data-generator-el-genesis-data         <none>                                        RUNNING
43bd0fa8c237   task-5009164c-d412-4708-a382-29b1106bc497        <none>                                        RUNNING
```

- Copy the local Execution Client (EL) port (here it is  rpc: 8545/tcp -> `127.0.0.1:32814`)
- Run the forge tests using the rpc port as the `fork-url`

```shell
forge test --fork-url http://127.0.0.1:32814
```


## Deleting the Kurtosis instance

- List all the running Kurtosis enclaves

```shell

kurtosis enclave ls
UUID           Name               Status     Creation Time
d81add16b7ab   verdant-geyser     STOPPED    Wed, 13 Sep 2023 11:35:13 UTC
55567e20d948   dry-steppe         STOPPED    Wed, 13 Sep 2023 12:27:15 UTC
ac32bca10ef2   crumbling-desert   RUNNING    Wed, 13 Sep 2023 12:36:26 UTC
41e1cfadf280   sleeping-harbor    RUNNING    Wed, 13 Sep 2023 12:38:24 UTC
b59a10957352   patient-orchard    RUNNING    Wed, 13 Sep 2023 12:41:39 UTC
```

- Select the name or UUID of the instance you want to kill, and then run

```shell
 kurtosis enclave stop b59a10957352
INFO[2023-09-13T12:49:30Z] Stopping enclaves...                         
INFO[2023-09-13T12:49:34Z] Enclaves stopped successfull
```

- Remove the artifacts

```shell
kurtosis clean
INFO[2023-09-13T12:50:51Z] Cleaning old Kurtosis engine containers...   
INFO[2023-09-13T12:50:51Z] Successfully cleaned old Kurtosis engine containers 
INFO[2023-09-13T12:50:51Z] Cleaning enclaves...                         
INFO[2023-09-13T12:50:53Z] Successfully removed the following enclaves: 
41e1cfadf280415b9e13a4edc164d9e0        sleeping-harbor
55567e20d9484a2bb6c348172fe43870        dry-steppe
ac32bca10ef2405684db714ad188a168        crumbling-desert
b59a109573524512aefa0679814ff877        patient-orchard
d81add16b7ab403ea2c57150b449e859        verdant-geyser
INFO[2023-09-13T12:50:53Z] Successfully cleaned enclaves 
````
