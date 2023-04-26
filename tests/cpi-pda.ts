import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { CpiPda } from "../target/types/cpi_pda";
import { Worker as WorkerTypes } from "../target/types/worker";
import { Keypair } from "@solana/web3.js";
import { assert } from "chai";

describe("cpi-pda", () => {
	// Configure the client to use the local cluster.
	anchor.setProvider(anchor.AnchorProvider.env());

	const managerProgram = anchor.workspace.CpiPda as Program<CpiPda>;
	const workerProgram = anchor.workspace.Worker as Program<WorkerTypes>;

	const { SystemProgram } = anchor.web3;

	const provider = anchor.AnchorProvider.local();
	const me = provider.wallet;
	anchor.setProvider(provider);

	let dataKP: Keypair;

	before(async () => {
		dataKP = Keypair.generate();
	});

	it("Is initialized!", async () => {
		await workerProgram.methods
			.initialize()
			.accounts({
				data: dataKP.publicKey,
				user: me.publicKey,
				systemProgram: SystemProgram.programId
			}).signers([dataKP])
			.rpc()
		const dataAccount = await workerProgram.account.data.fetch(dataKP.publicKey);
		assert(dataAccount.value.toNumber() === 0, 'value should be 0');
	});

	it("Is incremented!", async () => {
		await workerProgram.methods.increment().accounts({
			data: dataKP.publicKey
		}).rpc();

		const dataAccount = await workerProgram.account.data.fetch(dataKP.publicKey);
		assert(dataAccount.value.toNumber() === 1, "value should have been incremented");
	})

	it("Is incremented by the manager!", async() => {
		
		await managerProgram.methods.incThroughMe().accounts({
			workerProgram: workerProgram.programId,
			workerData: dataKP.publicKey
		}).rpc();

		const dataAccount = await workerProgram.account.data.fetch(dataKP.publicKey);
		assert(dataAccount.value.toNumber() === 2, "value should have been incremented twice");

	})

});
