import * as os from 'os';

export type PlatformType = 'Windows_NT' | 'Linux' | 'Darwin';

type RustTarget =
	| 'x86_64-pc-windows-msvc'
	| 'x86_64-unknown-linux-musl'
	| 'x86_64-apple-darwin';

type PlatformSpec = {
	target: RustTarget;
	binaryName: string;
};

const SUPPORTED_PLATFORMS: Map<PlatformType, PlatformSpec> = new Map();

SUPPORTED_PLATFORMS.set('Windows_NT', {
	target: 'x86_64-pc-windows-msvc',
	binaryName: 'frapps.exe',
});

SUPPORTED_PLATFORMS.set('Linux', {
	target: 'x86_64-unknown-linux-musl',
	binaryName: 'frapps',
});

SUPPORTED_PLATFORMS.set('Darwin', {
	target: 'x86_64-apple-darwin',
	binaryName: 'frapps',
});

export function getCurrentPlatform(): PlatformType {
	const type = os.type() as PlatformType;
	const architecture = os.arch();

	if (architecture !== 'x64') {
		throw new Error('Unsupported architecture ' + architecture);
	}

	return type;
}

export function getPlatformSpec(platformType: PlatformType): PlatformSpec {
	const platformSpec = SUPPORTED_PLATFORMS.get(platformType);

	if (!platformSpec) {
		throw new Error('Unsupported platform type ' + platformType);
	}

	return platformSpec;
}
