import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { Scivault } from "../target/types/scivault";
import assert from "assert";

describe("scivault", () => {
  anchor.setProvider(anchor.AnchorProvider.env());
  const program = anchor.workspace.Scivault as Program<Scivault>;
  const provider = anchor.getProvider();
  const wallet = provider.wallet as anchor.Wallet;

  // Helper function to fund the wallet
  async function ensureWalletFunded() {
    const balance = await provider.connection.getBalance(wallet.publicKey);
    if (balance < 1_000_000_000) { // Less than 1 SOL
      await provider.connection.requestAirdrop(wallet.publicKey, 1_000_000_000);
      await new Promise(resolve => setTimeout(resolve, 1000)); // Wait for airdrop
    }
  }

  before(async () => {
    await ensureWalletFunded();
  });

  it("Initializes a researcher", async () => {
    const researcher = anchor.web3.Keypair.generate();

    await program.methods
      .initializeResearcher("Dr. Alice")
      .accounts({
        researcher: researcher.publicKey,
        signer: wallet.publicKey,
        systemProgram: anchor.web3.SystemProgram.programId,
      } as any) // Workaround for TypeScript error
      .signers([researcher])
      .rpc();

    const researcherAccount = await program.account.researcher.fetch(researcher.publicKey);
    assert.equal(researcherAccount.name, "Dr. Alice");
    assert.equal(researcherAccount.verified, true);
    assert.equal(researcherAccount.authority.toString(), wallet.publicKey.toString());
  });

  it("Uploads research", async () => {
    const researcher = anchor.web3.Keypair.generate();
    const research = anchor.web3.Keypair.generate();

    await program.methods
      .initializeResearcher("Dr. Alice")
      .accounts({
        researcher: researcher.publicKey,
        signer: wallet.publicKey,
        systemProgram: anchor.web3.SystemProgram.programId,
      } as any)
      .signers([researcher])
      .rpc();

    await program.methods
      .uploadResearch("My Research", "QmDummyHash")
      .accounts({
        research: research.publicKey,
        researcher: researcher.publicKey,
        signer: wallet.publicKey,
        systemProgram: anchor.web3.SystemProgram.programId,
      } as any)
      .signers([research])
      .rpc();

    const researchAccount = await program.account.research.fetch(research.publicKey);
    assert.equal(researchAccount.title, "My Research");
    assert.equal(researchAccount.ipfsHash, "QmDummyHash");
    assert.equal(researchAccount.minted, false);
    assert.equal(researchAccount.author.toString(), researcher.publicKey.toString());
  });

  it("Mints research as NFT", async () => {
    const researcher = anchor.web3.Keypair.generate();
    const research = anchor.web3.Keypair.generate();

    await program.methods
      .initializeResearcher("Dr. Alice")
      .accounts({
        researcher: researcher.publicKey,
        signer: wallet.publicKey,
        systemProgram: anchor.web3.SystemProgram.programId,
      } as any)
      .signers([researcher])
      .rpc();

    await program.methods
      .uploadResearch("My Research", "QmDummyHash")
      .accounts({
        research: research.publicKey,
        researcher: researcher.publicKey,
        signer: wallet.publicKey,
        systemProgram: anchor.web3.SystemProgram.programId,
      } as any)
      .signers([research])
      .rpc();

    await program.methods
      .mintResearchNft("https://ipfs.io/ipfs/QmDummyHash")
      .accounts({
        research: research.publicKey,
        researcher: researcher.publicKey,
        signer: wallet.publicKey,
        systemProgram: anchor.web3.SystemProgram.programId,
      } as any)
      .rpc();

    const researchAccount = await program.account.research.fetch(research.publicKey);
    assert.equal(researchAccount.minted, true);
  });

  it("Fails to mint already minted research", async () => {
    const researcher = anchor.web3.Keypair.generate();
    const research = anchor.web3.Keypair.generate();

    await program.methods
      .initializeResearcher("Dr. Alice")
      .accounts({
        researcher: researcher.publicKey,
        signer: wallet.publicKey,
        systemProgram: anchor.web3.SystemProgram.programId,
      } as any)
      .signers([researcher])
      .rpc();

    await program.methods
      .uploadResearch("My Research", "QmDummyHash")
      .accounts({
        research: research.publicKey,
        researcher: researcher.publicKey,
        signer: wallet.publicKey,
        systemProgram: anchor.web3.SystemProgram.programId,
      } as any)
      .signers([research])
      .rpc();

    await program.methods
      .mintResearchNft("https://ipfs.io/ipfs/QmDummyHash")
      .accounts({
        research: research.publicKey,
        researcher: researcher.publicKey,
        signer: wallet.publicKey,
        systemProgram: anchor.web3.SystemProgram.programId,
      } as any)
      .rpc();

    try {
      await program.methods
        .mintResearchNft("https://ipfs.io/ipfs/QmDummyHash")
        .accounts({
          research: research.publicKey,
          researcher: researcher.publicKey,
          signer: wallet.publicKey,
          systemProgram: anchor.web3.SystemProgram.programId,
        } as any)
        .rpc();
      assert.fail("Should have failed to mint already minted research");
    } catch (err) {
      assert.equal(err.error.errorCode.code, "AlreadyMinted");
    }
  });

  it("Creates and buys escrow", async () => {
    const researcher = anchor.web3.Keypair.generate();
    const research = anchor.web3.Keypair.generate();
    const escrow = anchor.web3.Keypair.generate();

    await program.methods
      .initializeResearcher("Dr. Alice")
      .accounts({
        researcher: researcher.publicKey,
        signer: wallet.publicKey,
        systemProgram: anchor.web3.SystemProgram.programId,
      } as any)
      .signers([researcher])
      .rpc();

    await program.methods
      .uploadResearch("My Research", "QmDummyHash")
      .accounts({
        research: research.publicKey,
        researcher: researcher.publicKey,
        signer: wallet.publicKey,
        systemProgram: anchor.web3.SystemProgram.programId,
      } as any)
      .signers([research])
      .rpc();

    const price = 1_000_000; // 0.001 SOL
    await program.methods
      .createEscrow(new anchor.BN(price))
      .accounts({
        escrow: escrow.publicKey,
        research: research.publicKey,
        researcher: researcher.publicKey,
        seller: wallet.publicKey,
        systemProgram: anchor.web3.SystemProgram.programId,
      } as any)
      .signers([escrow])
      .rpc();

    const buyerBalanceBefore = await provider.connection.getBalance(wallet.publicKey);

    await program.methods
      .buyResearch()
      .accounts({
        escrow: escrow.publicKey,
        research: research.publicKey,
        buyer: wallet.publicKey,
        seller: wallet.publicKey,
        systemProgram: anchor.web3.SystemProgram.programId,
      } as any)
      .rpc();

    const escrowAccount = await program.account.escrow.fetch(escrow.publicKey);
    assert.equal(escrowAccount.active, false);
    assert.equal(escrowAccount.price.toNumber(), price);
    assert.equal(escrowAccount.seller.toString(), wallet.publicKey.toString());
    assert.equal(escrowAccount.research.toString(), research.publicKey.toString());

    const buyerBalanceAfter = await provider.connection.getBalance(wallet.publicKey);
    assert(buyerBalanceBefore > buyerBalanceAfter, "Buyer balance should decrease");
  });

  it("Fails to buy from inactive escrow", async () => {
    const researcher = anchor.web3.Keypair.generate();
    const research = anchor.web3.Keypair.generate();
    const escrow = anchor.web3.Keypair.generate();

    await program.methods
      .initializeResearcher("Dr. Alice")
      .accounts({
        researcher: researcher.publicKey,
        signer: wallet.publicKey,
        systemProgram: anchor.web3.SystemProgram.programId,
      } as any)
      .signers([researcher])
      .rpc();

    await program.methods
      .uploadResearch("My Research", "QmDummyHash")
      .accounts({
        research: research.publicKey,
        researcher: researcher.publicKey,
        signer: wallet.publicKey,
        systemProgram: anchor.web3.SystemProgram.programId,
      } as any)
      .signers([research])
      .rpc();

    await program.methods
      .createEscrow(new anchor.BN(1_000_000))
      .accounts({
        escrow: escrow.publicKey,
        research: research.publicKey,
        researcher: researcher.publicKey,
        seller: wallet.publicKey,
        systemProgram: anchor.web3.SystemProgram.programId,
      } as any)
      .signers([escrow])
      .rpc();

    await program.methods
      .buyResearch()
      .accounts({
        escrow: escrow.publicKey,
        research: research.publicKey,
        buyer: wallet.publicKey,
        seller: wallet.publicKey,
        systemProgram: anchor.web3.SystemProgram.programId,
      } as any)
      .rpc();

    try {
      await program.methods
        .buyResearch()
        .accounts({
          escrow: escrow.publicKey,
          research: research.publicKey,
          buyer: wallet.publicKey,
          seller: wallet.publicKey,
          systemProgram: anchor.web3.SystemProgram.programId,
        } as any)
        .rpc();
      assert.fail("Should have failed to buy from inactive escrow");
    } catch (err) {
      assert.equal(err.error.errorCode.code, "EscrowInactive");
    }
  });

  it("Fails to buy with insufficient funds", async () => {
    const researcher = anchor.web3.Keypair.generate();
    const research = anchor.web3.Keypair.generate();
    const escrow = anchor.web3.Keypair.generate();
    const poorBuyer = anchor.web3.Keypair.generate();

    await program.methods
      .initializeResearcher("Dr. Alice")
      .accounts({
        researcher: researcher.publicKey,
        signer: wallet.publicKey,
        systemProgram: anchor.web3.SystemProgram.programId,
      } as any)
      .signers([researcher])
      .rpc();

    await program.methods
      .uploadResearch("My Research", "QmDummyHash")
      .accounts({
        research: research.publicKey,
        researcher: researcher.publicKey,
        signer: wallet.publicKey,
        systemProgram: anchor.web3.SystemProgram.programId,
      } as any)
      .signers([research])
      .rpc();

    const highPrice = 10_000_000_000; // 10 SOL
    await program.methods
      .createEscrow(new anchor.BN(highPrice))
      .accounts({
        escrow: escrow.publicKey,
        research: research.publicKey,
        researcher: researcher.publicKey,
        seller: wallet.publicKey,
        systemProgram: anchor.web3.SystemProgram.programId,
      } as any)
      .signers([escrow])
      .rpc();

    try {
      await program.methods
        .buyResearch()
        .accounts({
          escrow: escrow.publicKey,
          research: research.publicKey,
          buyer: poorBuyer.publicKey,
          seller: wallet.publicKey,
          systemProgram: anchor.web3.SystemProgram.programId,
        } as any)
        .signers([poorBuyer])
        .rpc();
      assert.fail("Should have failed to buy with insufficient funds");
    } catch (err) {
      assert.equal(err.error.errorCode.code, "InsufficientFunds");
    }
  });
});