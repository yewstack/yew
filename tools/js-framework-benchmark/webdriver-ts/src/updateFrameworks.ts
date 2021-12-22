import * as fs from 'fs';
import * as path from 'path';
const ncu = require('npm-check-updates');
import * as semver from 'semver';
import * as yargs from 'yargs';
import {loadFrameworkVersionInformation, determineInstalledVersions, FrameworkVersionInformation, FrameworkVersionInformationStatic, FrameworkVersionInformationDynamic, FrameworkVersionInformationError,
    PackageVersionInformation, PackageVersionInformationValid, PackageVersionInformationErrorUnknownPackage, PackageVersionInformationErrorNoPackageJSONLock, PackageVersionInformationResult} from './common';
var exec = require('child_process').execSync;

let args = yargs(process.argv)
    .usage("$0 --updade true|false --dir")
    .default('update', 'true')
    .array('dir')
    .boolean('update').argv;

let updatePackages = args.update;
console.log("ARGS", args._.slice(2, args._.length));
let directories = args._.slice(2, args._.length);
let checkDirectory = (keyedType:string, folderName: string) => directories.length===0 || args._.some(a => path.join(keyedType, folderName).startsWith(a));

async function ncuReportsUpdatedVersion(packageVersionInfo: PackageVersionInformationResult) {
    let ncuInfo = await ncu.run({
        packageFile: path.resolve('..', 'frameworks', packageVersionInfo.framework.keyedType, packageVersionInfo.framework.directory, 'package.json'),
        silent: true,
        jsonUpgraded: true,
        loglevel: 'silent'
    });
    if (ncuInfo) {
        // console.log(ncuInfo);
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

    let frameworkVersionInformations = await loadFrameworkVersionInformation();

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


    let packageLockInformations : PackageVersionInformationResult[] = await Promise.all(automatically.map(frameworkVersionInformation => determineInstalledVersions(frameworkVersionInformation)));

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

        // console.log(unknownPackages.map(val => val.framework.keyedType +'/' + val.framework.directory + ' for package ' + unknownPackagesStr(val)).join('\n') + '\n');
    }

    let checkVersionsFor = packageLockInformations
        .filter(pli => pli.versions.every((packageVersionInfo: PackageVersionInformation) => packageVersionInfo instanceof PackageVersionInformationValid))
        .filter(f => checkDirectory(f.framework.keyedType,f.framework.directory));

    console.log("checkVersionsFor", checkVersionsFor.map(v => v.getFrameworkData().uri));

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


