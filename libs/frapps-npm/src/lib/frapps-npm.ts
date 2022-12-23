import { Binary } from "binary-install";
import * as os from "os";
import { name, repository, version } from "../../package.json";

const SUPPORTED_PLATFORMS = [
	{
		TYPE: "Windows_NT",
		ARCHITECTURE: "x64",
		RUST_TARGET: "x86_64-pc-windows-msvc",
		BINARY_NAME: "frapps.exe"
	},
	{
		TYPE: "Linux",
		ARCHITECTURE: "x64",
		RUST_TARGET: "x86_64-unknown-linux-musl",
		BINARY_NAME: "frapps"
	},
	{
		TYPE: "Darwin",
		ARCHITECTURE: "x64",
		RUST_TARGET: "x86_64-apple-darwin",
		BINARY_NAME: "frapps"
	}
];

function getPlatformMetadata() {
	const type = os.type();
	const architecture = os.arch();

	for (const supportedPlatform of SUPPORTED_PLATFORMS) {
		if (
			type === supportedPlatform.TYPE &&
			architecture === supportedPlatform.ARCHITECTURE
		) {
			return supportedPlatform;
		}
	}

	throw new Error(`Platform with type "${type}" and architecture "${architecture}" is not supported by ${name}.\nYour system must be one of the following:\n\n${SUPPORTED_PLATFORMS}`);
};

function getBinary() {
	const platformMetadata = getPlatformMetadata();
	// the url for this binary is constructed from values in `package.json`
	// https://github.com/EverlastingBugstopper/binary-install/releases/download/v1.0.0/binary-install-example-v1.0.0-x86_64-apple-darwin.tar.gz
	// const url = `${repository.url}/releases/download/v${version}/${name}-v${version}-${platformMetadata.RUST_TARGET}.tar.gz`;
	const url = 'https://github.com/ziacik/frapps/releases/download/0.0.1/frapps_0.0.1_x86_64-unknown-linux-musl.tar.gz';
	return new Binary(platformMetadata.BINARY_NAME, url);
};

export function run() {
	const binary = getBinary();
	binary.run();
};

export function install() {
	const binary = getBinary();
	binary.install();
};

