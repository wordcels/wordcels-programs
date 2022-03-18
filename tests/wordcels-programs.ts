import * as anchor from '@project-serum/anchor';
import { Program } from '@project-serum/anchor';
import { WordcelsPrograms } from '../target/types/wordcels_programs';

describe('wordcels-programs', () => {

  // Configure the client to use the local cluster.
  anchor.setProvider(anchor.Provider.env());

  const program = anchor.workspace.WordcelsPrograms as Program<WordcelsPrograms>;

  it('Is initialized!', async () => {
    // Add your test here.
    const tx = await program.rpc.initialize({});
    console.log("Your transaction signature", tx);
  });
});
