import * as anchor from "@project-serum/anchor";
import { Program } from "@project-serum/anchor";
import { RustTaskBackend } from "../target/types/rust_task_backend";

describe("rust-task-backend", () => {
  // Configure the client to use the local cluster.
  anchor.setProvider(anchor.Provider.env());

  const program = anchor.workspace.RustTaskBackend as Program<RustTaskBackend>;

  it("Is initialized!", async () => {
    // Add your test here.
    const tx = await program.rpc.initialize({});
    console.log("Your transaction signature", tx);
  });
});
