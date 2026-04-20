# Sorolance

**Sorolance** - Milestone-Based Decentralized Freelance Marketplace

## Project Description

Sorolance is a decentralized smart contract solution built on the Stellar blockchain using the Soroban SDK. It provides a secure, trustless platform for managing freelance projects and payments. By utilizing milestone-based smart contracts, the system eliminates the need for expensive centralized middleman platforms, ensuring that clients' funds are securely locked in escrow and freelancers are guaranteed payment upon successful completion of defined tasks.

The application empowers both parties by bringing transparency to project workflows, securing budgets upfront, and executing automatic payouts through the speed and efficiency of the Stellar network.

## Project Vision

Our vision is to revolutionize the global gig economy by:

- **Removing Intermediaries**: Eliminating high platform fees (often 10-20%) charged by traditional freelance marketplaces.
- **Ensuring Fair Compensation**: Guaranteeing freelancers get paid for their hard work by locking client funds in an immutable smart contract before work begins.
- **Building Trustless Collaboration**: Providing clients with the security that their funds will only be released when specific, agreed-upon milestones are met.
- **Empowering Global Talent**: Leveraging Stellar's borderless, low-cost micro-transactions to connect clients and freelancers worldwide without banking friction.

We envision a future where independent professionals have complete autonomy and financial security, backed by the mathematical certainty of blockchain technology.

## Key Features

### 1. **Milestone-Based Escrow**
- Break down large projects into manageable milestones
- Lock funds securely upfront for the entire project
- Release funds sequentially as each phase is completed

### 2. **Trustless Fund Management**
- Client funds are held in the smart contract, not by a third party
- Neither party can unilaterally withdraw locked funds without mutual agreement or task completion
- Automated payouts triggered instantly upon client approval

### 3. **Transparent Project Tracking**
- Real-time on-chain state reflects project progress
- Immutable record of submitted work and approved milestones
- Clear visibility of locked vs. released funds for both parties

### 4. **Stellar Network Efficiency**
- Sub-cent transaction fees make micro-milestones economically viable
- Near-instant settlement ensures freelancers receive funds without delay
- Built with the modern, developer-friendly Soroban Rust SDK

### 5. **Secure Work Submission**
- On-chain logging of milestone submissions
- Cryptographically verified approvals by the client's wallet

## Contract Details

- Contract Address: CDDOUEGEMLIUJEGW2OCHZYLJT74BHOJICLRHJYQS3RLLZL3IPVZOEX7T

## Future Scope

### Short-Term Enhancements
1. **Multi-Asset Support**: Allow escrow contracts using stablecoins like USDC alongside XLM
2. **Dispute Resolution Mechanism**: Introduce a decentralized arbiter role to resolve conflicts
3. **Time-Locked Milestones**: Implement deadlines that auto-refund clients if freelancers ghost the project

### Medium-Term Development
4. **On-Chain Reputation System**: Build a rating and review system permanently tied to a user's wallet address
5. **Portfolio Integration**: Allow freelancers to mint completed milestones as verified "Proof of Work" NFTs
6. **Decentralized Chat**: Integrate encrypted wallet-to-wallet messaging for project communication

### Long-Term Vision
7. **DAO Governance**: Transition platform parameter control (like arbiter fees) to a community-governed DAO
8. **Cross-Chain Escrow**: Enable clients to fund projects from other blockchains
9. **Decentralized Identity (DID)**: Implement KYC-less verification systems using zero-knowledge proofs

### Enterprise Features
10. **Enterprise Payroll Pipelines**: Allow agencies to fund multiple escrow contracts in batches
11. **Tax & Invoicing Tools**: Auto-generate compliant PDF invoices triggered by smart contract events
12. **Custom Arbitration Workflows**: Support corporate-defined legal arbiters for high-value contracts

---

## Technical Requirements

- Soroban SDK
- Rust programming language
- Stellar blockchain network (Testnet/Mainnet)
- Compatible Stellar Wallet (e.g., Freighter)

## Getting Started

Deploy the smart contract to Stellar's Soroban network and interact with it using the core escrow functions:

- `create_project()` - Initialize a new escrow contract, define milestones, and lock client funds.
- `submit_milestone()` - Freelancer calls this to mark a specific phase as complete.
- `approve_and_pay()` - Client approves the work, triggering the contract to release milestone funds to the freelancer.
- `cancel_project()` - Mutual cancellation feature to safely refund the client before work begins.

---

**Sorolance** - Securing Trust in the Decentralized Gig Economy