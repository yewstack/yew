import * as fs from 'fs';
import * as path from 'path';
const ncu = require('npm-check-updates');
import * as semver from 'semver';
import * as yargs from 'yargs';

var exec = require('child_process').execSync;

async function prepareDockerVolume() {
    // Check if docker volume js-framework-benchmark exists and create it if not
    try {
        let r: string[] = [];
        exec('docker volume inspect js-framework-benchmark', {
            stdio: r
        });
        console.log("docker volume js-framework-benchmark exists already");
    } catch (e) {
        let r: string[] = [];
        if (e.message.indexOf("No such volume: js-framework-benchmark")>-1) {
            console.log("Volume not found. Creating volume: docker volume create js-framework-benchmark");
            exec('docker volume create js-framework-benchmark', {
                stdio: r
            });
        } else {
            console.log("Unknown error checking volume ", e);
        }
    }
}

async function clearDockerVolume() {
    try {
        let r: string[] = [];
        exec('docker volume inspect js-framework-benchmark', {
            stdio: r
        });
    } catch (e) {
        console.log("docker volume js-framework-benchmark not found");
        return;
    }
    console.log("Remove docker volume js-framework-benchmark");
    let r: string[] = [];
    exec('docker volume rm js-framework-benchmark', {
        stdio: r
    });
}

async function stopContainerIfRunnning() {
    console.log("checking if js-framework-benchmark container runs.");
    let r : string[] = [];
    let res = exec('docker ps', {
        stdio: r
    });
    if (res.indexOf('js-framework-benchmark')>-1) {
        console.log("js-framework-benchmark container runs. Stopping this container.");
        let res = exec('docker stop js-framework-benchmark', {
            stdio: r
        });    
    }
}

async function startDocker() {
    console.log("starting docker");
    exec('docker run --rm -d -i --name js-framework-benchmark -p 8080:8080 --volume js-framework-benchmark:/build js-framework-benchmark-centos', {
        stdio: 'inherit'
    });
}

function copyFileToBuild(file: string) {
    exec(`docker cp ${file} js-framework-benchmark:/build`, {
        stdio: 'inherit'
    });
}
function dockerRootExec(cmd: string) {
    return exec(`docker exec -it -u root js-framework-benchmark ${cmd}`, {
        stdio: 'inherit'
    });
}

async function copyFilesToDocker() {
    try {
        console.log('copying build files to docker volume');
        copyFileToBuild("../build.js");
        copyFileToBuild("../css");
        copyFileToBuild("../package.json");
        copyFileToBuild("../frameworks");
        dockerRootExec('npm install');
        dockerRootExec('chown -R user:user /build');
    } catch (e) {
        console.log("copy files to docker failed. Trying to stop container js-framework-benchmark");
        stopContainerIfRunnning();
        throw e;
    }
}

async function runBuildInDocker() {
    console.log("executing npm install and node build.js in docker container");
    exec('docker exec -it -w /build js-framework-benchmark npm install', {
        stdio: 'inherit'
    });

    exec('docker exec -it -w /build js-framework-benchmark node build.js --benchmarks_only', {
        stdio: 'inherit'
    });
}

async function main() {
    stopContainerIfRunnning();
    // clearDockerVolume();
    // prepareDockerVolume();
    startDocker();
    copyFilesToDocker();
    runBuildInDocker();
}

main()
    .then(text => {
    })
    .catch(err => {
        console.log('error', err);
    });

/*let args = yargs(process.argv)
    .usage("$0 --updade true|false --dir")
    .default('update', 'true')
    .array('dir')
    .boolean('update').argv;

let updatePackages = args.update;
console.log("ARGS", args._.slice(2, args._.length));
let directories = args._.slice(2, args._.length);
let checkDirectory = (keyedType:string, folderName: string) => directories.length===0 || args._.includes(path.join(keyedType, folderName));

async function ncuReportsUpdatedVersion(packageVersionInfo: PackageVersionInformationResult) {
    let ncuInfo = await ncu.run({
        packageFile: path.resolve('..', 'frameworks', packageVersionInfo.framework.keyedType, packageVersionInfo.framework.directory, 'package.json'),
        silent: true,
        jsonUpgraded: true,
        loglevel: 'silent'
    });
    if (ncuInfo) {
        console.log(ncuInfo);
        return packageVersionInfo.versions.filter((pi: PackageVersionInformationValid) => ncuInfo[pi.packageName])
            .some((pi: PackageVersionInformationValid) => {
                let newVersion = ncuInfo[pi.packageName];
                if (newVersion.startsWith('^')) newVersion = newVersion.substring(1);
                if (newVersion.startsWith('~')) newVersion = newVersion.substring(1);
                if (newVersion) {
                    return !semver.satisfies(newVersion, '~'+pi.version);
                } else {
                    return false;
                }
        });
    } else {
        return false;
    }
}

async function ncuRunUpdate(packageVersionInfo: PackageVersionInformationResult) {
    console.log("Update "+packageVersionInfo.framework.keyedType +'/' + packageVersionInfo.framework.directory);
    await ncu.run({
        packageFile: path.resolve('..', 'frameworks', packageVersionInfo.framework.keyedType, packageVersionInfo.framework.directory, 'package.json'),
        upgrade: true
    });
}


async function main() {

    let frameworkVersionInformations = loadFrameworkVersionInformation();

    let errors = frameworkVersionInformations.filter(frameworkVersionInformation => frameworkVersionInformation instanceof FrameworkVersionInformationError);

    if (errors.length > 0) {
        console.log("ERROR: The following frameworks do not include valid version info and must be fixed");
        console.log(errors.map(val => val.keyedType +'/' + val.directory).join('\n') + '\n');
    }

    let manually = frameworkVersionInformations.filter(frameworkVersionInformation => frameworkVersionInformation instanceof FrameworkVersionInformationStatic);

    if (manually.length > 0) {
        console.log("WARNING: The following frameworks must be updated manually: ");
        console.log(manually.map(val => val.keyedType + '/' + val.directory).join('\n') + '\n');
    }

    let automatically = frameworkVersionInformations
            .filter(frameworkVersionInformation => frameworkVersionInformation instanceof FrameworkVersionInformationDynamic)
            .map(frameworkVersionInformation => frameworkVersionInformation as FrameworkVersionInformationDynamic);

    let packageLockInformations : PackageVersionInformationResult[] = automatically.map(frameworkVersionInformation => determineInstalledVersions(frameworkVersionInformation));

    let noPackageLock = packageLockInformations.filter(pli => pli.versions.some((packageVersionInfo: PackageVersionInformation) => packageVersionInfo instanceof PackageVersionInformationErrorNoPackageJSONLock));

    if (noPackageLock.length > 0) {
        console.log("WARNING: The following frameworks do not yet have a package-lock.json file (maybe you must 'npm install' it): ");
        console.log(noPackageLock.map(val => val.framework.keyedType +'/' + val.framework.directory).join('\n') + '\n');
    }

    let unknownPackages = packageLockInformations.filter(pli => pli.versions.some((packageVersionInfo: PackageVersionInformation) => packageVersionInfo instanceof PackageVersionInformationErrorUnknownPackage));

    if (unknownPackages.length > 0) {
        console.log("WARNING: The following frameworks do not have a version for the specified packages in package-lock.json file (maybe you misspelled the package name): ");
        let unknownPackagesStr = (packageVersionInfo: PackageVersionInformationResult) => packageVersionInfo.versions.filter(pvi => pvi instanceof PackageVersionInformationErrorUnknownPackage).
            map((packageVersionInfo: PackageVersionInformationErrorUnknownPackage) => packageVersionInfo.packageName).join(', ');

        console.log(unknownPackages.map(val => val.framework.keyedType +'/' + val.framework.directory + ' for package ' + unknownPackagesStr(val)).join('\n') + '\n');
    }

    let checkVersionsFor = packageLockInformations
        .filter(pli => pli.versions.every((packageVersionInfo: PackageVersionInformation) => packageVersionInfo instanceof PackageVersionInformationValid))
        .filter(f => checkDirectory(f.framework.keyedType,f.framework.directory));

    console.log("checkVersionsFor", checkVersionsFor);

    let toBeUpdated = new Array<PackageVersionInformationResult>();
    for (let f of checkVersionsFor) {
        if (await ncuReportsUpdatedVersion(f)) toBeUpdated.push(f);
    }
    console.log("The following frameworks can be updated");

    if (toBeUpdated.length > 0) {
        console.log(toBeUpdated.map(val => val.framework.keyedType +'/' + val.framework.directory).join('\n') + '\n');

        if (updatePackages) {
            let rebuild = "";
            for (let val of toBeUpdated) {
                console.log("ACTION: Updating package.json for " +  val.framework.keyedType +'/' + val.framework.directory);
                await ncuRunUpdate(val);
                let prefix = `${val.framework.keyedType}/${val.framework.directory}`;
                rebuild = rebuild + "'"+prefix+"' ";
            }
            console.log("\nTODO: Rebuilding is required:");

            console.log(`npm run rebuild -- ${rebuild}`);
            exec('npm run rebuild -- '+rebuild, {
                stdio: 'inherit'
            });

        }
    }
}

main()
    .then(text => {
    })
    .catch(err => {
        console.log('error', err);
    });


*/