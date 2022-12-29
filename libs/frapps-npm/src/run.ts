#!/usr/bin/env node

import { spawn } from 'child_process';
import { join } from 'path';
import { install, needsInstall } from './lib/installer';
import { getCurrentPlatform, getPlatformSpec } from './lib/platform';

const installDir = join(__dirname, '../node_modules', '.bin');

installIfNeeded()
	.then(() => setTimeout(() => run(), 1000))
	.catch(() => console.error('kurv'));

function run() {
	const { binaryName } = getPlatformSpec(getCurrentPlatform());

	const binaryPath = join(installDir, binaryName);

	const child = spawn(binaryPath, process.argv.slice(2));

	child.stdout.on('data', (data: unknown) => {
		console.log(`${data}`);
	});

	child.stderr.on('data', (data: unknown) => {
		console.error(`${data}`);
	});

	child.on('close', (code: number) => {
		process.exit(code);
	});
}

async function installIfNeeded() {
	if (needsInstall(installDir)) {
		await install(installDir, getCurrentPlatform());
	}
}
