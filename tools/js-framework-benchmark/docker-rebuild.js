var _ = require('lodash');
var exec = require('child_process').execSync;
var fs = require('fs');
var path = require('path');
var yargs = require('yargs');
const rimraf = require('rimraf');

let args = process.argv.length<=2 ? [] : process.argv.slice(2,process.argv.length);

let frameworks = args.filter(a => !a.startsWith("--"));
let justCopyAndBuild = args.some(f => f=="--justCopyAndBuild")

console.log("justCopyAndBuild", justCopyAndBuild);

if (frameworks.length==0) {
    console.log("ERROR: Missing arguments. Command: docker-rebuild keyed/framework1 non-keyed/framework2 ...");
    process.exit(1);
}

let rsync =  (keyed,name) => exec(`rsync -avC --exclude elm-stuff --exclude dist --exclude output --exclude package-lock.json --exclude tmp --exclude node_modules --exclude bower_components /src/frameworks/${keyed}/${name} /build/frameworks/${keyed}/`,    
{
    stdio: 'inherit'
});   


for (f of frameworks) {
    let components = f.split("/");
    if (components.length!=2) {
        console.log(`ERROR: invalid name ${f}. It must contain exactly one /.`)
        process.exit(1);
    }
    let [keyed,name] = components;
    let path = `frameworks/${keyed}/${name}`
    if (justCopyAndBuild) {
        rsync(keyed,name);
        exec('npm run build-prod', {
            cwd: path,
            stdio: 'inherit'
        });
    } else {
        if (fs.existsSync(path)) {
            console.log("deleting folder ",path);
            exec(`rm -r ${path}`);
        }
        rsync(keyed,name);
        exec('rm -rf package-lock.json yarn.lock dist elm-stuff bower_components node_modules', {
            cwd: path,
            stdio: 'inherit'
        });
        console.log("running npm install && npm run build-prod");
        exec('npm install && npm run build-prod', {
            cwd: path,
            stdio: 'inherit'
        });

    }
}

let frameworkNames = frameworks.join(" ");
console.log('npm run bench -- --headless --noResults --exitOnError true --count 1  '+frameworkNames);
exec('npm run bench -- --headless --noResults --exitOnError true --count 1 '+frameworkNames, {
    cwd: 'webdriver-ts',
    stdio: 'inherit'
});
console.log('npm run isKeyed -- --headless '+frameworkNames);
exec('npm run isKeyed -- --headless '+frameworkNames, {
    cwd: 'webdriver-ts',
    stdio: 'inherit'
});

exec('npm run index', {
    cwd: 'webdriver-ts',
    stdio: 'inherit'
});

console.log("All checks are fine!");
console.log("======> Please rerun the benchmark: npm run bench ", frameworkNames);
