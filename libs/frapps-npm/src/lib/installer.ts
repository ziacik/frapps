import get, { AxiosResponse } from 'axios';
import { mkdirSync, readFileSync, writeFileSync } from 'fs';
import { join } from 'path';
import { x } from 'tar';
import { repository, version } from '../../package.json';
import { getPlatformSpec, PlatformType } from './platform';

export function needsInstall(installDir: string): boolean {
	try {
		const installedVersion = readFileSync(join(installDir, 'version'), 'utf8');
		return installedVersion !== version;
	} catch {
		return true;
	}
}

export async function install(
	installDir: string,
	platformType: PlatformType
): Promise<void> {
	const { target } = getPlatformSpec(platformType);

	const response = await get(
		`${repository.url}/releases/download/${version}/frapps_${version}_${target}.tar.gz`,
		{ responseType: 'stream' }
	);

	if (response.status !== 200) {
		throw new Error(
			`Unable to download package: ${response.status} ${response.statusText}.`
		);
	}

	mkdirSync(installDir, { recursive: true });

	await extractResponseTo(installDir, response);
	writeVersion(installDir);
}

async function extractResponseTo(
	installDir: string,
	response: AxiosResponse
): Promise<void> {
	return new Promise((resolve, reject) => {
		try {
			const writer = x({ C: installDir });
			writer.on('close', () => {
				resolve();
			});
			writer.on('error', (err: unknown) => {
				reject(err);
			});
			response.data.pipe(writer);
		} catch (err: unknown) {
			reject(err);
		}
	});
}

function writeVersion(installDir: string): void {
	const versionFile = join(installDir, 'version');
	writeFileSync(versionFile, version);
}
