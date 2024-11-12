import * as web3 from "@solana/web3.js";

await performChecks(pg);

console.log("ProgramID: ", pg.PROGRAM_ID.toString());

// Get the latest blockhash info
const blockhashInfo = await pg.connection.getLatestBlockhash();

// Helper function to serialize the instruction data
function serializeInstruction(
	instructionId: number,
	nodeId?: string,
	taskId?: string,
	ipAddress?: string,
	hardwareId?: string,
	answerData?: string,
): Buffer {
	const instructionBuffer = Buffer.alloc(4);
	instructionBuffer.writeUInt32LE(instructionId); // Write 4-byte instruction ID

	const buffers = [instructionBuffer];

	if (nodeId) {
		const nodeIdBuffer = Buffer.from(nodeId, "utf8");
		const nodeIdLength = Buffer.from([nodeIdBuffer.length]);
		buffers.push(nodeIdLength, nodeIdBuffer);
	}

	if (taskId) {
		const taskIdBuffer = Buffer.from(taskId, "utf8");
		const taskIdLength = Buffer.from([taskIdBuffer.length]);
		buffers.push(taskIdLength, taskIdBuffer);
	}

	if (ipAddress) {
		const ipAddressBuffer = Buffer.from(ipAddress, "utf8");
		const ipAddressLength = Buffer.from([ipAddressBuffer.length]);
		buffers.push(ipAddressLength, ipAddressBuffer);
	}

	if (hardwareId) {
		const hardwareIdBuffer = Buffer.from(hardwareId, "utf8");
		const hardwareIdLength = Buffer.from([hardwareIdBuffer.length]);
		buffers.push(hardwareIdLength, hardwareIdBuffer);
	}

	if (answerData) {
		const answerBuffer = Buffer.from(answerData, "utf8");
		const answerLength = Buffer.from([answerBuffer.length]);
		buffers.push(answerLength, answerBuffer);
	}

	return Buffer.concat(buffers);
}

// Function to send a transaction for a given instruction type
async function sendTransaction(instructionId: number, dataBuffer: Buffer) {
	const tx = new web3.Transaction({ ...blockhashInfo });

	tx.add(
		new web3.TransactionInstruction({
			programId: pg.PROGRAM_ID,
			keys: [
				{
					pubkey: pg.wallet.keypair.publicKey,
					isSigner: true,
					isWritable: true,
				},
			],
			data: dataBuffer,
		}),
	);

	tx.sign(pg.wallet.keypair);

	const txHash = await pg.connection.sendRawTransaction(tx.serialize());
	console.log("Transaction sent with hash:", txHash);

	await pg.connection.confirmTransaction({
		blockhash: blockhashInfo.blockhash,
		lastValidBlockHeight: blockhashInfo.lastValidBlockHeight,
		signature: txHash,
	});

	console.log(
		`View your transaction on Solana Explorer: 
    https://explorer.solana.com/tx/${txHash}?cluster=devnet`,
	);
}

// Register a new node
const registerNodeData = serializeInstruction(
	0,
	"123",
	undefined,
	"192.168.1.1",
	"hw_001",
);
await sendTransaction(0, registerNodeData);

// Remove a node
const removeNodeData = serializeInstruction(1, "123");
await sendTransaction(1, removeNodeData);

// Dispatch a task to a node
const dispatchTaskData = serializeInstruction(2, "123", "task_001");
await sendTransaction(2, dispatchTaskData);

// Return an answer from a node for a task
const returnAnswerData = serializeInstruction(
	3,
	"123",
	"task_001",
	undefined,
	undefined,
	"answer_data",
);
await sendTransaction(3, returnAnswerData);

// Start a session for a node
const startSessionData = serializeInstruction(4, "123");
await sendTransaction(4, startSessionData);

// End a session for a node
const endSessionData = serializeInstruction(5, "123");
await sendTransaction(5, endSessionData);

// Check to ensure the playground is set up correctly and there's enough devnet SOL
async function performChecks(pg: any) {
	const MINIMUM_BALANCE_REQUIRED = 1e9; // for example, 1 SOL

	if (!pg.wallet || !pg.wallet.keypair) {
		throw new Error(
			"You first need to connect your playground wallet at the bottom left",
		);
	}

	// Get the balance of the wallet and check if we have enough
	const walletBalance = await pg.connection.getBalance(
		pg.wallet.keypair.publicKey,
	);

	if (walletBalance < MINIMUM_BALANCE_REQUIRED) {
		console.log(
			`Your sol balance is low. To get some devnet SOL you can use this link:
      https://faucet.solana.com/?walletAddress=${pg.wallet.keypair.publicKey.toString()}&amount=1`,
		);
	}

	if (!pg.PROGRAM_ID) {
		throw new Error("You first need to ‘build‘ and ‘deploy‘ your program");
	}
}
