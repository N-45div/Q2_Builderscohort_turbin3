# SciVault: Decentralized Research Vault (PoC)

## Overview

**Project Name:** SciVault

**Value Proposition:**
SciVault is a decentralized platform built on Solana, leveraging high-speed blockchain, decentralized physical infrastructure (DePIN), and real-world asset (RWA) tokenization to securely store, share, and monetize scientific research data. It empowers researchers to collaborate, publish, and earn rewards in a trustless, low-cost environment.

### Product-Market Fit:

Current research systems are plagued by paywalls, data silos, and funding inefficiencies. SciVault addresses these challenges by providing:

* Decentralized, encrypted data storage via IPFS/Filecoin.
* Tokenized research assets (NFTs) using Metaplex standards.
* A peer-review marketplace optimized for speed and cost-efficiency on Solana.

---

## Target User Profiles

### 1. Independent Researcher

* **Demographics:** Academics and scientists, aged 25-50, globally distributed.
* **Interests:** Open science, decentralized collaboration, data monetization.
* **Motivations:** Securely publish research, gain recognition, and earn rewards without costly intermediaries.

### 2. Biotech/Pharma Research Manager

* **Demographics:** Professionals in biotech/pharma firms.
* **Interests:** Licensing high-quality, peer-reviewed research.
* **Motivations:** Access trustworthy data to accelerate R\&D and reduce costs.

### 3. Grant Provider

* **Demographics:** Funding organizations and grant agencies.
* **Interests:** Supporting impactful research with transparent reward mechanisms.
* **Motivations:** Efficiently identify and fund promising projects based on peer-review scores.

---

## User Stories

**SV-001: Independent Researcher (Dr. Maya Patel)**

* **Goal:** Securely publish and monetize research data.
* **Acceptance Criteria:**

  * Upload encrypted research data and metadata.
  * Mint NFTs for tokenized research assets.
  * View peer-review feedback, trade/license assets, and receive rewards.

**SV-002: Biotech Firm Research Manager (Alex Chen)**

* **Goal:** License verifiable, peer-reviewed research data.
* **Acceptance Criteria:**

  * Browse, filter, and license NFTs with peer-review scores.
  * Initiate licensing via escrow and track data usage.

**SV-003: Grant Provider (Sarah Lopez)**

* **Goal:** Fund promising research based on peer-review scores.
* **Acceptance Criteria:**

  * View peer-reviewed assets and trigger reward payouts.
  * Monitor payout history and adjust reward criteria.

---

## MVP Features

1. Researcher Initialization: Register as a verified researcher.
2. Research Upload: Store research metadata and encrypted data on-chain.
3. NFT Minting: Tokenize research assets as NFTs.
4. Escrow Marketplace: Facilitate research asset sales via escrow.
5. Peer-Review Integration: Mocked peer-review scores for MVP.

---

## Technical Architecture

* **Blockchain:** Solana (high throughput, low fees).
* **Framework:** Anchor 0.31.1 for smart contract development.
* **Storage:** IPFS/Filecoin (mocked with CIDs for MVP).
* **NFTs:** Metaplex standards (mocked for MVP).
* **Oracles:** Switchboard for peer-review scores (planned).

### Solana Programs

* `initialize_researcher`: Register a researcher.
* `upload_research`: Store research metadata on-chain.
* `mint_research_nft`: Mark research as tokenized.
* `create_escrow`: List research for sale via escrow.
* `buy_research`: Transfer lamports and complete escrow.

---

## Prerequisites

* **Rust:** Install Rust and Cargo (`rustup` recommended).
* **Solana CLI:** Install the Solana CLI.
* **Anchor:** Install Anchor CLI.
* **Node.js:** Install Node.js (v18 or later) and npm.
* **solana-test-validator:** For local testing.

---

## Testing

* Researcher initialization.
* Research upload and NFT minting (mocked).
* Escrow creation and purchase.

---

## Future Roadmap

* Full Metaplex Integration.
* IPFS/Filecoin Integration.
* Switchboard Oracle Integration.
* Frontend UI Development.
* Security Audits.
* Scalability Improvements.

---

## License

This project is licensed under the MIT License. See the LICENSE file for details.

---

SciVault: Empowering researchers to share, monetize, and collaborate on scientific discoveries in a decentralized world.
