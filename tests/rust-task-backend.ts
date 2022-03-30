import * as anchor from "@project-serum/anchor";
import assert from 'assert';
import { Program } from "@project-serum/anchor";
import { RustTaskBackend } from "../target/types/rust_task_backend";

describe('rust_task', () => {
  const provider = anchor.Provider.env();
  anchor.setProvider(provider);
  const program = anchor.workspace.RustTaskBackend as Program<RustTaskBackend>;
  const { SystemProgram } = anchor.web3;
  const donators = anchor.web3.Keypair.generate();

  it("Is initialized!", async () => {
    await program.rpc.initialize({ 
      accounts: {
        donators: donators.publicKey,
        user: provider.wallet.publicKey,
        systemProgram: SystemProgram.programId,
      },
      signers: [donators],
    });
    const account = await program.account.donators.fetch(donators.publicKey);
    assert.ok(Number(account.totalDonators) === 0);
  });

  it("Is should send transaction", async () => {
    await program.rpc.sendTransaction(new anchor.BN(30000),{ 
      accounts: {
        donators: donators.publicKey,
        user: provider.wallet.publicKey,
      },
    });
    const account = await program.account.donators.fetch(donators.publicKey);
    assert.ok(Number(account.donatorsList[0].amount) === 30000);
  })

  it('Is should receive money', async () => {
    await program.rpc.receiveTransaction({accounts: {donators: donators.publicKey, user: provider.wallet.publicKey}});
    const account = await program.account.donators.fetch(donators.publicKey);
    console.log(account);
  })

});