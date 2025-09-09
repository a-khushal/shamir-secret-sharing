import { p, Share } from './constants'

function randomBigInt(max: bigint): bigint {
	const bytes = Math.ceil(max.toString(2).length / 8);
	while (true) {
		let array = new Uint8Array(bytes);
		crypto.getRandomValues(array);

		let r = 0n;
		for (let b of array) {
			r = (r << 8n) + BigInt(b);
		}

		if (r <= max) return r;
	}
}

function generateShares(coeff: bigint[], n: number, p: bigint): Share[] {
	let shares = [];

	for (let x = 1; x <= n; x++) {
		let y = 0n;
		
		// horner's method of evaluating polynomials
		for (let j = coeff.length - 1; j >= 0; j--) {
			y = (y * BigInt(x) + coeff[j]) % p;
		}

		shares.push({ x: x.toString(), y: y.toString(16).padStart(64, '0') });
	}

	return shares;
}

function split(secret: string, n: number, k: number) {
	if (!(1 < k && k <= n)) throw new Error("Invalid threshold");
	if (n > 255) throw new Error("Maximum number of shares (n) is 255");

	const s = BigInt(secret) % p;
	let coeff = [s];

	for (let i = 1; i < k; i++) {
		coeff[i] = randomBigInt(p - 1n);
	}

	const shares = generateShares(coeff, n, p);
	return { shares };
}

console.log(split('0x9f3c0d2e1a4b5c6d7e8f9a0b1c2d3e4f5a6b7c8d9e0f1a2b3c4d5e6f7a8b9c0d', 5, 3));
