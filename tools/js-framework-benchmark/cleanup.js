var _ = require('lodash');
var exec = require('child_process').execSync;
var fs = require('fs');
var commandExists = require('command-exists');
const path = require('path');
const rimraf = require('rimraf');

function rmIfExists(base, name) {
	let dir = path.join(base, name);
	if(fs.existsSync(dir)) {
		console.log("Clean ",dir);
        rimraf.sync(dir);
	}
}

for (let keyedType of ['keyed', 'non-keyed']) {
    let dir = path.resolve('frameworks', keyedType);
    let directories = fs.readdirSync(dir);

    for (let name of directories) {
        let fd = path.resolve(dir, name);
        console.log('cleaning ', fd);
		if(fs.existsSync(fd+"/node_modules")) {
			rimraf.sync(fd+"/node_modules");
		}
		rmIfExists(fd, "package-lock.json");
		rmIfExists(fd, "yarn.lock");
		rmIfExists(fd, "dist");
		rmIfExists(fd, "elm-stuff");
		rmIfExists(fd, "bower_components");
		rmIfExists(fd, "node_modules");
	}
}
