#!/usr/bin/env node

import { spawn } from 'child_process';
import { join } from 'path';
import { install, needsInstall } from './lib/installer';
import { getCurrentPlatform, getPlatformSpec } from './lib/platform';

const installDir = join(__dirname, '../node_modules', '.bin');

installIfNeeded()
	.then(() => run())
	.catch((err: unknown) => console.error(`Error ${err}`));

async function run() {
	const { binaryName } = getPlatformSpec(getCurrentPlatform());
	const binaryPath = join(installDir, binaryName);

	console.log('Running', binaryPath);

	const child = spawn(binaryPath, process.argv.slice(2), {
		shell: true,
		stdio: 'inherit',
	});

	child.stdout?.on('data', (data: unknown) => {
		console.log(`${data}`);
	});

	child.stderr?.on('data', (data: unknown) => {
		console.error(`${data}`);
	});

	child.on('close', (code: number | null) => {
		if (code == null) {
			console.error('Null return code from binary. (SEGFAULT?)');
			process.exit(1);
		} else {
			process.exit(code);
		}
	});

	child.on('error', (error: unknown) => {
		console.log('Error', error);
		process.exit(1);
	});
}

async function installIfNeeded() {
	if (needsInstall(installDir)) {
		console.log('Downloading current binary...');
		await install(installDir, getCurrentPlatform());
		console.log('Done installing.');
	}
}
