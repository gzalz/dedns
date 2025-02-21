import * as web3 from "@solana/web3.js";
import * as anchor from "@coral-xyz/anchor";
import * as anchor from "@coral-xyz/anchor";
import * as web3 from "@solana/web3.js";
import { getAssociatedTokenAddressSync } from "@solana/spl-token";
import type { Dedns } from "../target/types/dedns";
import type { Dedns } from "../target/types/dedns";

describe("Test", () => {
  // Configure the client to use the local cluster
  anchor.setProvider(anchor.AnchorProvider.env());

  const program = anchor.workspace.Dedns as anchor.Program<Dedns>;
  
  // Configure the client to use the local cluster
  anchor.setProvider(anchor.AnchorProvider.env());

  const program = anchor.workspace.Dedns as anchor.Program<Dedns>;
  
  it("initialize", async () => {
    const seed = Buffer.from("leases");
    const leaseOwner = new web3.PublicKey(
      "Sp5wv9Tyb9P3NiWLAohbxHVUDq2APRd9tdT2sLxbLZ3"
    ).toBuffer();
    const pda = web3.PublicKey.findProgramAddressSync(
      [seed, leaseOwner],
      new web3.PublicKey("2oTp3tETacPzdrj4r3QJdpY5x9QugzHa21sJd2Q6AWav")
    );
    console.log(pda.toString());
  });
});
