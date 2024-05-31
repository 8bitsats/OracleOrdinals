import * as anchor from '@project-serum/anchor';
import { Program } from '@project-serum/anchor';
import { CrossChainOracle } from '../target/types/cross_chain_oracle';

describe('cross_chain_oracle', () => {

  // Configure the client to use the local cluster.
  anchor.setProvider(anchor.Provider.env());

  const program = anchor.workspace.CrossChainOracle as Program<CrossChainOracle>;

  it('Is initialized!', async () => {
    // Add your test here.
    const tx = await program.rpc.initialize({});
    console.log("Your transaction signature", tx);
  });
});
