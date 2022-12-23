#!/usr/bin/env node

import { install } from './lib/installer';
install()
	.then(() => console.log('Done'))
	.catch((e) => {
		console.error('Sorry', e);
		process.exit(1);
	});
