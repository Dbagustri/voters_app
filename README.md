🗳️ Stellar Voting DApp

Stellar Voting DApp – Blockchain-Based Decentralized Voting System

📌 Project Description

Stellar Voting DApp is a decentralized smart contract application built on the Stellar blockchain using the Soroban SDK. This system enables users to create polls, cast votes, and view results in a transparent and tamper-proof environment.

Unlike traditional voting systems that rely on centralized authorities, this application ensures that all voting data is securely stored on-chain, making it immutable, verifiable, and resistant to manipulation.

Each poll contains multiple options, and users can vote only once per poll, ensuring fairness and integrity in the voting process.

This project was developed as part of a learning exploration in blockchain and smart contract development using Soroban on Stellar.

🎯 Project Vision

Our vision is to transform digital decision-making by:

Decentralizing Voting Systems: Eliminating reliance on centralized authorities
Ensuring Transparency: Making all voting results publicly verifiable
Preventing Manipulation: Using blockchain immutability to secure votes
Empowering Users: Allowing individuals to participate in trustless governance
Building Fair Systems: Guaranteeing one vote per user per poll

We envision a future where voting systems—whether for organizations, communities, or governance—are secure, transparent, and fully decentralized.

🚀 Key Features
1. Poll Creation
Create polls with custom titles
Add multiple voting options
Automatic poll ID generation
Stored securely on blockchain
2. Secure Voting System
One user = one vote per poll
Prevents double voting
Real-time vote updates
Fair and transparent process
3. Result Retrieval
View total votes for each option
Instant access to poll results
Structured data for frontend integration
4. Blockchain Transparency
All actions recorded on-chain
Verifiable voting results
Immutable data storage
No hidden manipulation
5. Stellar Network Integration
Built with Soroban Smart Contract SDK
Fast and low-cost transactions
Scalable and efficient architecture
📜 Contract Details
Network: Stellar Testnet
Contract Address:
CBTTPXQKEZSCE722RRIV4XWDAFFKXUXQPXRJFJDTBUMROTEKVOZLRJPK
📸 Deployment Proof

Below is the successful deployment of the smart contract on Stellar Testnet:

🧪 Demo Flow
Create Poll → "Pemilihan Ketua"
Add options → A, B, C
Vote as user1 → option A
Vote as user2 → option B
Get Result → [1,1,0]
🛠️ Smart Contract Functions
🔹 create_poll()

Create a new voting poll

Parameters:

title: String
options: Vec<String>
🔹 vote()

Vote for a specific option in a poll

Parameters:

poll_id: u64
option_index: u32
voter: String
🔹 get_polls()

Retrieve all available polls

🔹 get_result()

Get voting results for a poll

Parameters:

poll_id: u64
🔄 How It Works
User creates a poll with multiple options
Poll is stored on the blockchain
Users vote using their identifier
Smart contract prevents double voting
Results can be retrieved anytime
🔮 Future Scope
Short-Term Enhancements
Wallet-Based Voting
Replace String voter with blockchain Address
Improve security and authenticity
Voting Deadline
Add start and end time for polls
Percentage Results
Display results in percentage format
Improved Validation
Prevent invalid inputs and edge cases
Medium-Term Development
Frontend Interface
Web UI for easier interaction
No need for Stellar Lab
User Authentication
Integrate wallet-based identity
Poll Categories
Organize polls by topic or purpose
Event Logging
Track voting activities on-chain
Long-Term Vision
DAO Governance Integration
Use voting for decentralized decision-making
Cross-Chain Voting
Expand compatibility beyond Stellar
Decentralized UI Hosting
Deploy frontend on IPFS
Privacy Voting
Use zero-knowledge proofs for anonymous voting
🧰 Technical Requirements
Rust Programming Language
Soroban SDK
Stellar Testnet
Soroban Studio / Stellar Lab
🚀 Getting Started
Deploy the smart contract to Stellar Soroban network
Use Stellar Lab or CLI to interact with functions
Execute:
create_poll() → create a poll
vote() → cast a vote
get_polls() → view polls
get_result() → see results
🧠 Key Learning Outcomes
Smart contract development using Soroban
Blockchain-based data storage
Handling state in decentralized systems
Building trustless applications
👤 Author
Diandra Bagustri
🏁 Conclusion

<img width="1920" height="1200" alt="image" src="https://github.com/user-attachments/assets/c02ec5a9-d5e1-4635-b309-e32e420a5b6d" />


Stellar Voting DApp demonstrates how blockchain technology can be used to build transparent, secure, and decentralized voting systems. By removing central authority and leveraging smart contracts, this project ensures fairness and trust in every vote.

✨ Decentralized Voting for a Transparent Future
