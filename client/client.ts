import * as web3 from "@solana/web3.js";
import * as anchor from "@coral-xyz/anchor";
import * as anchor from "@coral-xyz/anchor";
import * as web3 from "@solana/web3.js";
import { PublicKey } from "@solana/web3.js";
import { getAssociatedTokenAddressSync } from "@solana/spl-token";
import type { Dedns } from "../target/types/dedns";
import type { Dedns } from "../target/types/dedns";

// Configure the client to use the local cluster
anchor.setProvider(anchor.AnchorProvider.env());

const program = anchor.workspace.Dedns as anchor.Program<Dedns>;


// Configure the client to use the local cluster
anchor.setProvider(anchor.AnchorProvider.env());

const program = anchor.workspace.Dedns as anchor.Program<Dedns>;


// Define the token mint address for wSOL and the owner
const wSOLMint = new PublicKey("So11111111111111111111111111111111111111112"); // wSOL mint address
const ownerPublicKey = new PublicKey(
  "J6TyK5tehKXVuZZWPw1oNYxRJiYa2Wiwj4iJsE1b1DEA"
);

const testMint = new PublicKey("8RzuQp7pNwykMnqBhmurWPe7JNSiApTnQWa5N2SVkktg");
const testAuthority = new PublicKey("GzaLzMNN5qLqM39ZcAKepM68gCmD6jyL37wQDCcxPxdr");

// Get the associated token address
const ata = getAssociatedTokenAddressSync(testMint, testAuthority);
console.log("Associated Token Address:", ata.toBase58());
