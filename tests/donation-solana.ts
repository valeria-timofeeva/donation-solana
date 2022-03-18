import * as anchor from "@project-serum/anchor";
import { Program } from "@project-serum/anchor";
import { DonationSolana } from "../target/types/donation_solana";

describe("donation-solana", () => {
  // Configure the client to use the local cluster.
  anchor.setProvider(anchor.Provider.env());

  const program = anchor.workspace.DonationSolana as Program<DonationSolana>;

  it("Is initialized!", async () => {
    // Add your test here.
    const tx = await program.rpc.initialize({});
    console.log("Your transaction signature", tx);
  });
});
