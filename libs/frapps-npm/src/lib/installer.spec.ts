import get, { AxiosResponse } from 'axios';
import { mkdirSync, readFileSync, writeFileSync } from 'fs';
import { PassThrough, Readable, Stream } from 'stream';
import { x } from 'tar';
import { repository, version } from '../../package.json';
import { install, needsInstall } from './installer';

jest.mock('axios');
jest.mock('tar');
jest.mock('fs');

const getSpy = get as unknown as jest.SpyInstance;
const xSpy = x as unknown as jest.SpyInstance;
const readFileSyncSpy = readFileSync as unknown as jest.SpyInstance;
const writeFileSyncSpy = writeFileSync as unknown as jest.SpyInstance;

describe('needsInstall', () => {
	it('returns true if no version file found in the install dir', () => {
		readFileSyncSpy.mockImplementation(() => {
			throw new Error();
		});
		const needs = needsInstall('/some/dir');
		expect(needs).toBe(true);
	});

	it('returns false if version file in the install dir contains current version', () => {
		readFileSyncSpy.mockImplementation((path: string) =>
			path === '/some/dir/version' ? version : '???'
		);
		const needs = needsInstall('/some/dir');
		expect(needs).toBe(false);
	});

	it('returns true if version file in the install dir does not contain current version', () => {
		readFileSyncSpy.mockImplementation((path: string) =>
			path === '/some/dir/version' ? '0.0.0' : '???'
		);
		const needs = needsInstall('/some/dir');
		expect(needs).toBe(true);
	});
});

describe('install', () => {
	let response: AxiosResponse<Readable>;
	let xWriter: PassThrough;

	beforeEach(() => {
		const stream = Readable.from(['some-body']);
		response = {
			data: stream,
			status: 200,
			statusText: 'OK',
			headers: {},
			config: {},
		};
		getSpy.mockResolvedValue(response);
		xWriter = new PassThrough();
		xSpy.mockImplementation(() => {
			setTimeout(() => xWriter.emit('close'), 500);
			return xWriter;
		});
	});

	it('fails if fetch fails', async () => {
		getSpy.mockRejectedValue(new Error('Some error'));
		await expect(install('/some/dir', 'Linux')).rejects.toEqual(
			new Error('Some error')
		);
	});

	it('fails if response is not 200', async () => {
		response.status = 404;
		response.statusText = 'Not found';
		await expect(install('/some/dir', 'Linux')).rejects.toEqual(
			new Error('Unable to download package: 404 Not found.')
		);
	});

	it('downloads and installs correct linux archive', async () => {
		await install('/some/dir', 'Linux');
		expect(getSpy).toHaveBeenCalledWith(
			`${repository.url}/releases/download/${version}/frapps_${version}_x86_64-unknown-linux-musl.tar.gz`,
			{ responseType: 'stream' }
		);
	});

	it('extracts downloaded linux archive to correct destination (i.e. installs)', async () => {
		await install('/some/dir', 'Linux');
		expect(mkdirSync).toHaveBeenCalledWith('/some/dir', { recursive: true });
		expect(x).toHaveBeenCalledWith({ C: '/some/dir' });
		await expectStream(xWriter, 'some-body');
	});

	it('writes current version to the version file after successful install', async () => {
		await install('/some/dir', 'Linux');
		expect(writeFileSyncSpy).toHaveBeenCalledWith('/some/dir/version', version);
	});
});

async function expectStream(stream: Stream, expected: string) {
	return new Promise<void>((resolve, reject) => {
		let data = '';
		stream.on('data', (buffer: Buffer) => {
			data += buffer.toString('utf8');
		});

		stream.on('end', () => {
			try {
				expect(data).toEqual(expected);
				resolve();
			} catch (e) {
				reject(e);
			}
		});
	});
}
