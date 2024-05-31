const anchor = require('@project-serum/anchor');
const { Connection, PublicKey } = require('@solana/web3.js');

async function main() {
    // Configure the client to use the local cluster.
    anchor.setProvider(anchor.AnchorProvider.env());

    // Address of the deployed program.
    const programId = new anchor.web3.PublicKey('Fg6PaFpoGXkYsidMpWxqSWpmyoMaMdHk4Zr1Dz4V7gAa');

    // Generate the program's interface.
    const program = new anchor.Program(idl, programId);

    // The address of the account to verify.
    const nftPublicKey = new PublicKey('Your NFT PublicKey Here');

    // Fetch proof from oracle service (simplified example)
    const response = await fetch('http://localhost:5000/get_proof');
    const { pubkey, signature, tx_hash } = await response.json();

    // Verify and update the NFT.
    const tx = await program.rpc.verifyAndUpdate(pubkey, signature, tx_hash, {
        accounts: {
            nft: nftPublicKey,
        },
    });

    console.log('Transaction signature', tx);
}

main().catch(err => {
    console.error(err);
});
