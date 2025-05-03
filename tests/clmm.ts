import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { Clmm } from "../target/types/clmm";

describe("clmm", () => {
  // Configure the client to use the local cluster.
  anchor.setProvider(anchor.AnchorProvider.env());

  const program = anchor.workspace.clmm as Program<Clmm>;

  it("InitializePool", async () => {
    // Add your test here.
    // const tx = await program.methods.initializePool().rpc();
    // console.log("Your transaction signature", tx);
  });
});
