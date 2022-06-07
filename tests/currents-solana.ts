import * as anchor from "@project-serum/anchor";
import { Program } from "@project-serum/anchor";
import { PublicKey } from '@solana/web3.js';
import { CurrentsSolana } from "../target/types/currents_solana";
import { assert } from 'chai';
import bs58 from "bs58";

describe("currents-solana", () => {
  // Configure the client to use the local cluster.
  anchor.setProvider(anchor.AnchorProvider.env());

  const program = anchor.workspace.CurrentsSolana as Program<CurrentsSolana>;
  const provider = program.provider as anchor.AnchorProvider;

  it("Create a user", async () => {
    const userName = "Bob";
    const [userMetaPDA, _] = await PublicKey
      .findProgramAddress(
        [
          anchor.utils.bytes.utf8.encode("nodes/users"),
          anchor.utils.bytes.utf8.encode(userName),
        ],
        program.programId
      );
    // Add your test here.
    const tx = await program.methods.addUser("stripeId", userName).accounts({
      user: provider.wallet.publicKey,
      userMeta: userMetaPDA,
    }).rpc();
    const accs = await program.account.userMeta.all([{
      memcmp: {
        bytes: bs58.encode(Buffer.from(userName)),
        offset: 8/*Discriminator*/ + 4/*string prefix*/,
      }
    }]);
    assert.equal(1, accs.length);
  });
  it("Create a user, a blog, link them", async () => {
    const userName = "Bob2";
    const [userMetaPDA] = await PublicKey
      .findProgramAddress(
        [
          anchor.utils.bytes.utf8.encode("nodes/users"),
          anchor.utils.bytes.utf8.encode(userName),
        ],
        program.programId
      );
    // Add your test here.
    await program.methods.addUser("stripeId", userName).accounts({
      user: provider.wallet.publicKey,
      userMeta: userMetaPDA,
    }).rpc();
    const blogName = "The Blog2";
    const [blogMetaPDA] = await PublicKey
      .findProgramAddress(
        [
          anchor.utils.bytes.utf8.encode("nodes/blogs"),
          anchor.utils.bytes.utf8.encode(blogName),
        ],
        program.programId
      );
    await program.methods.addBlog(blogName).accounts({
      user: provider.wallet.publicKey,
      blogMeta: blogMetaPDA,
    }).rpc();

    const [edgePda] = await PublicKey
      .findProgramAddress(
        [
          anchor.utils.bytes.utf8.encode("edges/subscriptions"),
          userMetaPDA.toBuffer(),
          blogMetaPDA.toBuffer(),
        ],
        program.programId
      );

    await program.methods.addSubscription().accounts({
      user: provider.wallet.publicKey,
      userMeta: userMetaPDA,
      blogMeta: blogMetaPDA,
      edge: edgePda,
    }).rpc();
    // search edges by user
    const edgesUser = await program.account.subscriptionEdgeMeta.all([{
      memcmp: {
        bytes: userMetaPDA.toBase58(),
        offset: 8/*Discriminator*/,
      }
    }]);
    // search edges by blog
    const edgesBlog = await program.account.subscriptionEdgeMeta.all([{
      memcmp: {
        bytes: blogMetaPDA.toBase58(),
        offset: 8/*Discriminator*/ + 32,
      }
    }]);
    // const edges = await program.account.subscriptionEdgeMeta.all();

    assert.equal(userMetaPDA.toBase58(), edgesUser[0].account.from.toBase58());
    assert.equal(userMetaPDA.toBase58(), edgesBlog[0].account.from.toBase58());
    assert.equal(blogMetaPDA.toBase58(), edgesUser[0].account.to.toBase58());
    assert.equal(blogMetaPDA.toBase58(), edgesBlog[0].account.to.toBase58());
  });
});
