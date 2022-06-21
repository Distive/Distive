!!!warning
Distive is still early alpha software
!!!

# Introduction

#### What is Distive ?
Distive is a decentralized communication channel that can be used to build software that requires mostly text input from an audience. Some examples of what you can build:
1. A comment section
2. An online poll
3. A forum
4. A chat room.

#### Why does Distive Exist ?
Because it's finally possible to build free to use software, that has close to zero maintainance cost, is open-source and difficult to censor, while guaranteeing data privacy.

#### How does Distive Work ?
Distive is hosted on the [Internet Computer Blockchain](https://internetcomputer.org), which allows building efficient and unstoppable decentralized software. Public services like Distive on the IC unlike traditional software have alot less moving parts; there are no ports, databases, external caching services, firewalls, etc. This makes Distive very simple and trivial to deploy.

Distive's pricing model works like this: services (canisters in Internet Computer terms) are given free credits (cycles) to run each month. When a service runs out of cycles, it freezes and can't be queried. You can then add more cycles to it. A small percentage of these cycles (currently 5%) are taxed and distributed to other services to keep running for free.

Every Distive service (canister) deployed is standalone and can only be controlled by its owner. It is not a single database with multiple users stored in it. This means nobody, including me, can control what happens to your deployed service (canister). 

Distive is written in Rust, and the source code is available on [Github](https://github.com/scroobius-pip/Distive).

#### How can I use Distive ?
1. Head over to the [Dashboard](https://dashboard.distive.com) and create a new service(canister)
2. Then use the Canister ID to integrate or build your own software following the [Integration Guide](https://docs.distive.com/integrations)

