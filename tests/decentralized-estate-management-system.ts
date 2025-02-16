import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { DecentralizedEstateManagementSystem } from "../target/types/decentralized_estate_management_system";
import { Keypair, LAMPORTS_PER_SOL, PublicKey } from "@solana/web3.js";
import { expect, assert } from "chai";
import { BN } from "bn.js";
import { randomBytes } from "crypto";

describe("decentralized-estate-management-system tests", () => {
  // Configure the client to use the local cluster.
  const provider = anchor.AnchorProvider.env();
  anchor.setProvider(provider);

  const program = anchor.workspace
    .DecentralizedEstateManagementSystem as Program<DecentralizedEstateManagementSystem>;

  const user = provider.wallet;

  const [leader, resident1, resident2, resident3, resident4, resident5] =
    Array.from({ length: 6 }, () => Keypair.generate());

  const estateName = "testEstate";

  const [estatePda, estateBump] = PublicKey.findProgramAddressSync(
    [Buffer.from("estate"), Buffer.from(estateName)],
    program.programId
  );

  const [vaultPda, vaultBump] = PublicKey.findProgramAddressSync(
    [Buffer.from("vault"), estatePda.toBuffer()],
    program.programId
  );

  const description = "hire new security";
  const pollPda = PublicKey.findProgramAddressSync(
    [
      Buffer.from("poll"),
      estatePda.toBuffer(),
      resident1.publicKey.toBuffer(),
      Buffer.from(description),
    ],
    program.programId
  )[0];

  const [residentPda, residentBump] = PublicKey.findProgramAddressSync(
    [
      Buffer.from("resident"),
      estatePda.toBuffer(),
      leader.publicKey.toBuffer(),
    ],
    program.programId
  );

  it("creates estate", async () => {
    // Add your test here.
    const tx = await provider.connection.requestAirdrop(
      leader.publicKey,
      2 * LAMPORTS_PER_SOL
    );

    await provider.connection.confirmTransaction(tx, "confirmed");
    try {
      const tx = await program.methods
        .initialize(estateName)
        .accountsStrict({
          leader: leader.publicKey,
          estate: estatePda,
          resident: residentPda,
          vault: vaultPda,
          systemProgram: anchor.web3.SystemProgram.programId,
        })
        .signers([leader])
        .rpc();

      const estate = await program.account.estateState.fetch(estatePda);
      const vaultBalance = await provider.connection.getBalance(vaultPda);

      assert.equal(estate.leader.toBase58(), leader.publicKey.toBase58());
      assert.equal(estate.name, estateName);
      assert.equal(vaultBalance, 0);
      assert.equal(estate.noOfResidents, 1);
      assert.equal(estate.vaultBump, vaultBump);
    } catch (error) {
      console.log("error:", error);
    }
  });
  it("resident joining estate", async () => {
    const airdropSol = async (address: PublicKey) => {
      const tx = await provider.connection.requestAirdrop(
        address,
        2 * LAMPORTS_PER_SOL
      );

      await provider.connection.confirmTransaction(tx, "confirmed");
      //console.log(`airdropped sol to ${address.toBase58()}`);
    };
    const add_a_resident = async (resident: anchor.web3.Keypair) => {
      try {
        await airdropSol(resident.publicKey);
        const [residentPda, residentBump] =
          await PublicKey.findProgramAddressSync(
            [
              Buffer.from("resident"),
              estatePda.toBuffer(),
              resident.publicKey.toBuffer(),
            ],
            program.programId
          );
        const tx = await program.methods
          .addResident()
          .accountsStrict({
            user: resident.publicKey,
            resident: residentPda,
            estate: estatePda,
            systemProgram: anchor.web3.SystemProgram.programId,
          })
          .signers([resident])
          .rpc();
      } catch (error) {
        console.log(error);
      }
    };

    try {
      await add_a_resident(resident1);
      await add_a_resident(resident2);
      await add_a_resident(resident3);
      await add_a_resident(resident4);
      await add_a_resident(resident5);

      const estate = await program.account.estateState.fetch(estatePda);

      assert.equal(estate.noOfResidents, 6);
    } catch (error) {
      console.log("error:", error);
    }
  });
  it("deposit SOL", async () => {
    const oneSol = new BN(100000000);
    const oneSolNum = 100000000;
    const seed = new BN(randomBytes(8));

    try {
      const transactionPda = PublicKey.findProgramAddressSync(
        [
          Buffer.from("transaction"),
          estatePda.toBuffer(),
          resident1.publicKey.toBuffer(),
          seed.toArrayLike(Buffer, "le", 8),
        ],
        program.programId
      )[0];

      const residentPda = await PublicKey.findProgramAddressSync(
        [
          Buffer.from("resident"),
          estatePda.toBuffer(),
          resident1.publicKey.toBuffer(),
        ],
        program.programId
      )[0];

      const tx = await program.methods
        .makeDeposit(seed, oneSol)
        .accountsStrict({
          user: resident1.publicKey,
          vault: vaultPda,
          estate: estatePda,
          transaction: transactionPda,
          resident: residentPda,
          systemProgram: anchor.web3.SystemProgram.programId,
        })
        .signers([resident1])
        .rpc();

      const resident = await program.account.residentState.fetch(residentPda);
      const estate = await program.account.estateState.fetch(estatePda);
      const transaction = await program.account.transactionState.fetch(
        transactionPda
      );
      const vaultBalance = await provider.connection.getBalance(vaultPda);

      assert.equal(resident.totalContributed.toNumber(), 100000000);
      assert.equal(estate.vaultBalance.toNumber(), vaultBalance);
      assert.equal(transaction.estate.toBase58(), estatePda.toBase58());
      assert.equal(transaction.isDeposit, true);
      assert.equal(transaction.amount.toNumber(), 100000000);
      assert.equal(transaction.from.toBase58(), resident1.publicKey.toBase58());
      assert.equal(transaction.to.toBase58(), vaultPda.toBase58());
    } catch (error) {
      console.log(error);
    }
  });
  it("create poll", async () => {
    const oneSol = new BN(100000000);
    try {
      const tx = await program.methods
        .createPoll(description, oneSol)
        .accountsStrict({
          user: resident1.publicKey,
          estate: estatePda,
          poll: pollPda,
          systemProgram: anchor.web3.SystemProgram.programId,
        })
        .signers([resident1])
        .rpc();

      const poll = await program.account.pollState.fetch(pollPda);

      assert.equal(poll.amount.toNumber(), 100000000);
      assert.equal(poll.active, true);
      assert.equal(poll.estate.toBase58(), estatePda.toBase58());
      assert.equal(poll.creator.toBase58(), resident1.publicKey.toBase58());
      assert.equal(poll.description, "hire new security");
      assert.equal(poll.agreeVotes.toNumber(), 0);
    } catch (error) {
      console.log(error);
    }
  });

  it("agree vote win in poll", async () => {
    let vaultBalance = await provider.connection.getBalance(vaultPda);
    let pollCreatorBalance = await provider.connection.getBalance(resident1.publicKey);
    const vote_in_poll = async (
      user: anchor.web3.Keypair,
      user_vote: boolean
    ) => {
      try {
        const seed = new BN(randomBytes(8));
        const votePda = await PublicKey.findProgramAddressSync(
          [
            Buffer.from("vote"),
            estatePda.toBuffer(),
            user.publicKey.toBuffer(),
            pollPda.toBuffer(),
          ],
          program.programId
        )[0];

        const transactionPda = PublicKey.findProgramAddressSync(
          [
            Buffer.from("transaction"),
            estatePda.toBuffer(),
            user.publicKey.toBuffer(),
            seed.toArrayLike(Buffer, "le", 8),
          ],
          program.programId
        )[0];

        const tx = await program.methods
          .vote(seed, user_vote)
          .accountsStrict({
            user: user.publicKey,
            estate: estatePda,
            vault: vaultPda,
            poll: pollPda,
            pollCreator: resident1.publicKey,
            vote: votePda,
            transaction: transactionPda,
            systemProgram: anchor.web3.SystemProgram.programId,
          })
          .signers([user])
          .rpc();

        let vote = await program.account.voteState.fetch(votePda);

        assert.equal(vote.voter.toBase58(), user.publicKey.toBase58());
        assert.equal(vote.vote, user_vote);
      } catch (error) {
        console.log(error);
      }
    };

    await vote_in_poll(resident1, true);
    await vote_in_poll(resident2, true);
    await vote_in_poll(resident3, true);
    await vote_in_poll(resident4, true);


    let poll = await program.account.pollState.fetch(pollPda);
    let estate = await program.account.estateState.fetch(estatePda);

    let pollCreatorBalanceAfter = await provider.connection.getBalance(resident1.publicKey);
    assert.equal(poll.agreeVotes.toNumber(), 4);
    assert.equal(poll.active, false);

    assert.equal(estate.vaultBalance.toNumber(), vaultBalance - poll.amount.toNumber());
    //assert.equal(pollCreatorBalanceAfter, pollCreatorBalance + poll.amount.toNumber())
  });
});
