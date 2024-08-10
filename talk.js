const anchor = require('@project-serum/anchor');

async function main() {
    // Configure the client to use the local validator.
    const provider = anchor.AnchorProvider.env();
    anchor.setProvider(provider);

    // Address of the deployed program
    const programId = new anchor.web3.PublicKey("6iGpXVq6yoCRJG97RxDTpwkAoJnRPiiNzZEyRyJJNskt");

    // Generate an instance of the program
    const idl = await anchor.Program.fetchIdl(programId, provider);
    const program = new anchor.Program(idl, programId, provider);

    // Example: Call the create_proposal function
    try {
        const description = "Example proposal";
        const expiration = new anchor.BN(Date.now() / 1000 + 3600); // 1 hour from now

        // You need to pass the correct accounts and their keys
        const proposal = anchor.web3.Keypair.generate();
        await program.rpc.createProposal(description, expiration, {
            accounts: {
                proposal: proposal.publicKey,
                authority: provider.wallet.publicKey,
                systemProgram: anchor.web3.SystemProgram.programId,
            },
            signers: [proposal],
        });

        console.log("Proposal created successfully!");
    } catch (error) {
        console.error("Failed to create proposal:", error);
    }
}

main();
