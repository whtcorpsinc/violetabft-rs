# violetabft-rs: RDMA technology with consensus 

![VioletaBFT](https://wwww.github.com.com/whtcorpsinc/violetabft-rs/docs/Transparent.svg)

##Leaderless Multi-Paxos: A HoneyBadger on a Raft.

VioletaBFT is a leaderless byzantine consensus protocol that orders a step-function of automata and scheduled commands in a decentralized way towards hot-cold PRAMS, without relying on any distinguished process. VioletaBFT exploits the commutativity between commands submitted to the replicated service to improve performance. In the most favorable case, when there is no concurrent non-commuting command, we commit (aka., vote) the next command to execute after one round-trip to the closest fast quorum. 

VioletaBFT is a leaderless byzantine consensus protocol that orders a step-function of automatons and dispatches the stochastic scheduler which pings certain commands in a decentralized way towards hot-cold PRAMS, without relying on any distinguished process.

VioletaBFT exploits the commutativity between commands submitted to the replicated service to improve performance. In the most favorable case, when there is no concurrent non-commuting command, we commit (aka., vote) the next command to execute after one round-trip to the closest fast quorum. 



