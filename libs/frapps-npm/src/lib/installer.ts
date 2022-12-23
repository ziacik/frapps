import get from 'axios';
import { x } from 'tar';
import { version, repository } from '../../package.json';

export async function install(): Promise<void> {
	console.log(
		`${repository.url}/releases/download/${version}/frapps_${version}_x86_64-unknown-linux-musl.tar.gz`
	);
	const response = await get(
		`${repository.url}/releases/download/${version}/frapps_${version}_x86_64-unknown-linux-musl.tar.gz`,
		{ responseType: 'stream' }
	);

	if (response.status !== 200) {
		throw new Error(
			`Unable to download package: ${response.status} ${response.statusText}.`
		);
	}

	response.data.pipe(x({ C: './install/dir' }));
}
