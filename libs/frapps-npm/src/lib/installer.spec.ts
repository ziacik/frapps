import get, { AxiosResponse } from 'axios';
import { PassThrough, Readable, Stream } from 'stream';
import { x } from 'tar';
import { repository, version } from '../../package.json';
import { install } from './installer';

jest.mock('axios');
jest.mock('tar');

const getSpy = get as unknown as jest.SpyInstance;
const xSpy = x as unknown as jest.SpyInstance;

describe('installer', () => {
	let response: AxiosResponse<Readable>;
	let xStream: PassThrough;

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
		xStream = new PassThrough();
		xSpy.mockReturnValue(xStream);
	});

	it('fails if fetch fails', async () => {
		getSpy.mockRejectedValue(new Error('Some error'));
		await expect(install()).rejects.toEqual(new Error('Some error'));
	});

	it('fails if response is not 200', async () => {
		response.status = 404;
		response.statusText = 'Not found';
		await expect(install()).rejects.toEqual(
			new Error('Unable to download package: 404 Not found.')
		);
	});

	it('downloads and installs correct linux archive', async () => {
		await install();
		expect(getSpy).toHaveBeenCalledWith(
			`${repository.url}/releases/download/${version}/frapps_${version}_x86_64-unknown-linux-musl.tar.gz`,
			{ responseType: 'stream' }
		);
	});

	it('extracts downloaded linux archive to correct destination (i.e. installs)', async () => {
		await install();
		expect(x).toHaveBeenCalledWith({ C: '/install/dir' });
		await expectStream(xStream, 'some-body');
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
