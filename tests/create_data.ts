import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { DataStore } from "../target/types/data_store";
import { Connection, Keypair, SystemProgram, PublicKey } from "@solana/web3.js";

describe("Create Data Account", () => {
  const provider = anchor.AnchorProvider.env();
  anchor.setProvider(provider);

  const payer = provider.wallet as anchor.Wallet;
  const program = anchor.workspace.DataStore as Program<DataStore>;
  const programId = program.programId;
  const dataStore = PublicKey.findProgramAddressSync([payer.publicKey.toBuffer()], programId)[0];

  it("Is initialized!", async () => {
    const tx = await program.methods
      .createDataStore("Content upload into the Solana blockchain!")
      .accounts({
        dataStore: dataStore,
        signer: payer.publicKey,
        systemProgram: SystemProgram.programId,
      })
      .rpc();
    console.log("Transaction signature", tx);
  });
});
