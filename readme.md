# elements rust json rpc client library

This is a new library aiming to be a useful dependency for apps and wallets that wish to interact with an [Elements](https://github.com/ElementsProject/elements) daemon, most often used in the [Liquid](https://liquid.net/) sidechain.

**NOTE: This is still a brand new Work In Progress crate**

If you need an existing crate you could take a look at [liquid-rpc](https://crates.io/crates/liquid-rpc)

## Design goals

- lower level calls that match the RPCs directly
- a higher level API to abstract common patterns
- ...

## RPCs

From: https://elementsproject.org/en/doc/22.0.0/rpc/

### blockchain

- [x] getbestblockhash
- [ ] getblock
- [ ] getblockchaininfo
- [x] getblockcount
- [ ] getblockfilter
- [ ] getblockhash
- [ ] getblockheader
- [ ] getblockstats
- [ ] getchaintips
- [ ] getchaintxstats
- [ ] getdifficulty
- [ ] getmempoolancestors
- [ ] getmempooldescendants
- [ ] getmempoolentry
- [ ] getmempoolinfo
- [ ] getrawmempool
- [ ] getsidechaininfo
- [ ] gettxout
- [ ] gettxoutproof
- [ ] gettxoutsetinfo
- [ ] preciousblock
- [ ] pruneblockchain
- [ ] savemempool
- [ ] scantxoutset
- [ ] verifychain
- [ ] verifytxoutproof

### control

- [ ] getmemoryinfo
- [ ] getrpcinfo
- [ ] help
- [ ] logging
- [ ] stop
- [ ] uptime

### generating

- [ ] combineblocksigs
- [ ] consumecompactsketch
- [ ] consumegetblocktxn
- [ ] finalizecompactblock
- [x] generateblock
- [ ] generatetoaddress
- [ ] generatetodescriptor
- [ ] getcompactsketch
- [ ] getnewblockhex

### mining

- [ ] getblocktemplate
- [ ] getmininginfo
- [ ] getnetworkhashps
- [ ] prioritisetransaction
- [ ] submitblock
- [ ] submitheader
- [ ] testproposedblock

### network

- [ ] addnode
- [ ] clearbanned
- [ ] disconnectnode
- [ ] getaddednodeinfo
- [ ] getconnectioncount
- [ ] getnettotals
- [ ] getnetworkinfo
- [ ] getnodeaddresses
- [ ] getpeerinfo
- [ ] listbanned
- [ ] ping
- [ ] setban
- [ ] setnetworkactive

### rawtransactions

- [ ] analyzepsbt
- [ ] calculateasset
- [ ] combinepsbt
- [ ] combinerawtransaction
- [ ] converttopsbt
- [ ] createpsbt
- [ ] createrawtransaction
- [ ] decodepsbt
- [ ] decoderawtransaction
- [ ] decodescript
- [ ] finalizepsbt
- [ ] fundrawtransaction
- [ ] getrawtransaction
- [ ] parsepsbt
- [ ] rawblindrawtransaction
- [ ] rawissueasset
- [ ] rawreissueasset
- [ ] sendrawtransaction
- [ ] signrawtransactionwithkey
- [ ] testmempoolaccept
- [ ] updatepsbtpegin
- [ ] utxoupdatepsbt

### signer

- [ ] enumeratesigners

### util

- [ ] createblindedaddress
- [ ] createmultisig
- [ ] deriveaddresses
- [ ] dumpassetlabels
- [x] estimatesmartfee
- [ ] getdescriptorinfo
- [ ] getindexinfo
- [ ] getpakinfo
- [ ] signmessagewithprivkey
- [ ] tweakfedpegscript
- [ ] validateaddress
- [ ] verifymessage

### wallet

- [ ] abandontransaction
- [ ] abortrescan
- [ ] addmultisigaddress
- [ ] backupwallet
- [ ] blindrawtransaction
- [ ] bumpfee
- [ ] claimpegin
- [ ] createrawpegin
- [x] createwallet
- [ ] destroyamount
- [ ] dumpblindingkey
- [ ] dumpissuanceblindingkey
- [ ] dumpmasterblindingkey
- [ ] dumpprivkey
- [ ] dumpwallet
- [ ] encryptwallet
- [ ] getaddressesbylabel
- [x] getaddressinfo
- [ ] getbalance
- [ ] getbalances
- [x] getnewaddress
- [ ] getpeginaddress
- [ ] getrawchangeaddress
- [ ] getreceivedbyaddress
- [ ] getreceivedbylabel
- [ ] gettransaction
- [ ] getunconfirmedbalance
- [x] getwalletinfo
- [ ] getwalletpakinfo
- [ ] importaddress
- [ ] importblindingkey
- [ ] importdescriptors
- [ ] importissuanceblindingkey
- [ ] importmasterblindingkey
- [ ] importmulti
- [ ] importprivkey
- [ ] importprunedfunds
- [ ] importpubkey
- [ ] importwallet
- [ ] initpegoutwallet
- [ ] issueasset
- [ ] keypoolrefill
- [ ] listaddressgroupings
- [ ] listdescriptors
- [ ] listissuances
- [ ] listlabels
- [ ] listlockunspent
- [ ] listreceivedbyaddress
- [ ] listreceivedbylabel
- [ ] listsinceblock
- [ ] listtransactions
- [ ] listunspent
- [ ] listwalletdir
- [ ] listwallets
- [ ] loadwallet
- [ ] lockunspent
- [ ] psbtbumpfee
- [ ] reissueasset
- [ ] removeprunedfunds
- [ ] rescanblockchain
- [ ] send
- [ ] sendmany
- [x] sendtoaddress
- [ ] sendtomainchain
- [ ] sethdseed
- [ ] setlabel
- [ ] settxfee
- [ ] setwalletflag
- [ ] signblock
- [ ] signmessage
- [ ] signrawtransactionwithwallet
- [ ] unblindrawtransaction
- [ ] unloadwallet
- [ ] upgradewallet
- [ ] walletcreatefundedpsbt
- [ ] walletdisplayaddress
- [ ] walletlock
- [ ] walletpassphrase
- [ ] walletpassphrasechange
- [ ] walletprocesspsbt

### zmq

- [ ] getzmqnotifications

---

## used in the tutorial

https://elementsproject.org/elements-code-tutorial/overview

- [x] getblockcount
- [x] createwallet
- [x] getwalletinfo
- [ ] rescanblockchain
- [x] getnewaddress
- [x] sendtoaddress
- [x] generatetoaddress
- [x] getaddressinfo
- [ ] getrawmempool
- [ ] gettransaction
- [ ] getrawtransaction
- [ ] importaddress
- [ ] importblindingkey
- [ ] dumpblindingkey
- [ ] dumpassetlabels
- [ ] listissuances
- [ ] importissuanceblindingkey
- [ ] decoderawtransaction
- [ ] reissueasset
- [ ] destroyamount
- [ ] dumpprivkey
- [ ] createmultisig
- [ ] getnewblockhex
- [ ] submitblock
- [ ] signblock
- [ ] combineblocksigs
- [ ] getpeginaddress
- [ ] gettxoutproof
- [ ] claimpegin
- [ ] issueasset
- [ ] reissueasset
- ...
