import * as fs from 'fs';
import {JSONResult, config, initializeFrameworks} from './common'
import {BenchmarkType, Benchmark, benchmarks} from './benchmarks'

async function main() {
	let frameworks = await initializeFrameworks();
	
	frameworks.sort( (a,b) => a.fullNameWithKeyedAndVersion.localeCompare(b.fullNameWithKeyedAndVersion));
	
	const dots = require('dot').process({
		path: './'
	});
	
	fs.writeFileSync('../index.html', dots.index({
		frameworks
	}), {
		encoding: 'utf8'
	})
}

main();