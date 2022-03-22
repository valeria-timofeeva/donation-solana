import * as anchor from "@project-serum/anchor";
import { Program } from "@project-serum/anchor";
import { publicKey } from "@project-serum/anchor/dist/cjs/utils";
import { assert, expect } from "chai";
import { DonationSolana } from "../target/types/donation_solana";
import { clusterApiUrl, Connection, PublicKey } from "@solana/web3.js";

const { SystemProgram } = anchor.web3;

describe("donation-solana", () => {
  // Configure the client to use the local cluster.
  const provider = anchor.Provider.env();
  anchor.setProvider(provider);

  const program = anchor.workspace.DonationSolana as Program<DonationSolana>;
  let donationAccount: anchor.web3.Keypair;
  let donator = anchor.web3.Keypair.generate();

  it("Should create donation account", async () => {
    donationAccount = anchor.web3.Keypair.generate();
    console.log("p", donationAccount.publicKey);

    const tx = await program.rpc.initialize({
      accounts: {
        donationAccount: donationAccount.publicKey,
        user: provider.wallet.publicKey,
        systemProgram: SystemProgram.programId,
      },
      signers: [donationAccount],
    });

    const account = await program.account.donationAccount.fetch(
      donationAccount.publicKey
    );

    assert.equal(account.owner, provider.wallet.publicKey);
  });

  it("Should make donation", async () => {
    await provider.connection.confirmTransaction(
      await provider.connection.requestAirdrop(donator.publicKey, 1),
      "confirmed"
    );

    const tx = await program.rpc.makeDonation(new anchor.BN(1), {
      accounts: {
        donationAccount: donationAccount.publicKey,
        user: donator.publicKey,
      },
      signers: [donator],
    });

    const donationBalance = await program.account.donationAccount.getAccountInfo(donator.publicKey)
    assert.equal(donationBalance.lamports.toString(), "1")
  });

  it("Should fail if not owner try to withdraw", async () => {
    let notOwner = anchor.web3.Keypair.generate();

    const tx = await program.rpc.withdraw({
      accounts: {
        donationAccount: donationAccount.publicKey,
        user: notOwner.publicKey,
      },
      signers: [notOwner],
    })
  });

  it("Should return all donators", async () => {
    let donator1 = anchor.web3.Keypair.generate();
    let donator2 = anchor.web3.Keypair.generate();

    await provider.connection.confirmTransaction(
      await provider.connection.requestAirdrop(donator1.publicKey, 10000000000),
      "confirmed"
    );
    await provider.connection.confirmTransaction(
      await provider.connection.requestAirdrop(donator2.publicKey, 10000000000),
      "confirmed"
    );

    const tx1 = await program.rpc.makeDonation(new anchor.BN(100), {
      accounts: {
        donationAccount: donationAccount.publicKey,
        user: donator1.publicKey,
      },
      signers: [donator1],
    });

    const tx2 = await program.rpc.makeDonation(new anchor.BN(100), {
      accounts: {
        donationAccount: donationAccount.publicKey,
        user: donator2.publicKey,
      },
      signers: [donator2],
    });

    const account = await program.account.donationAccount.fetch(
      donationAccount.publicKey
    );

    const connection = new Connection(clusterApiUrl("devnet"), "confirmed");
    const accounts = await connection.getProgramAccounts(program.programId);

    //to ask
    expect(accounts.find).to.eql([donator.publicKey, donator1, donator2]);
  });

  it("Should return all donation for selected donator", async () => {
    await provider.connection.confirmTransaction(
      await provider.connection.requestAirdrop(donator.publicKey, 1),
      "confirmed"
    );

    const tx = await program.rpc.makeDonation(new anchor.BN(1), {
      accounts: {
        donationAccount: donationAccount.publicKey,
        user: donator.publicKey,
      },
      signers: [donator],
    });

    //to check result fn getDonationsForAddress
  });
});

