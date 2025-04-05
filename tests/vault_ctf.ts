import * as anchor from "@project-serum/anchor";
import { Program } from "@project-serum/anchor";
import { VaultCtf } from "../target/types/vault_ctf";
import { assert } from "chai";

describe("🔓 CTF - Account Injection", () => {
  const provider = anchor.AnchorProvider.env();
  anchor.setProvider(provider);
  const program = anchor.workspace.VaultCtf as Program<VaultCtf>;

  const user = anchor.web3.Keypair.generate();
  const [vaultPDA, bump] = anchor.web3.PublicKey.findProgramAddressSync(
    [Buffer.from("vault"), user.publicKey.toBuffer()],
    program.programId
  );

  const flag = Buffer.from("CTF{injected-vault-access}".padEnd(32));

  it("Initialise le coffre de la victime", async () => {
    const tx = await program.methods
      .initializeVault(bump, [...flag])
      .accounts({
        vault: vaultPDA,
        user: user.publicKey,
        systemProgram: anchor.web3.SystemProgram.programId,
      })
      .signers([user])
      .rpc();

    console.log("✅ Coffre initialisé :", tx);
  });

  it("🚩 L'attaquant lit le flag sans vérification", async () => {
    const vault = await program.account.vault.fetch(vaultPDA);
    console.log("🚩 FLAG:", Buffer.from(vault.flag).toString());
    assert(Buffer.from(vault.flag).toString().startsWith("CTF{"));
  });
});
