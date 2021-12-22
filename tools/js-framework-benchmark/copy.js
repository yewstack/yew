var _ = require('lodash');
var exec = require('child_process').execSync;
var fs = require('fs-extra');
var path = require('path');

if (fs.existsSync("dist")) fs.removeSync("dist");
fs.mkdirSync("dist");
fs.mkdirSync("dist"+path.sep+"webdriver-ts");
fs.copySync("webdriver-ts"+path.sep+"table.html", "dist"+path.sep+"webdriver-ts"+path.sep+"table.html");

fs.copySync("index.html", "dist"+path.sep+"index.html");
fs.copySync("css", "dist"+path.sep+"css");

var excludes = ["node_modules","elm-stuff","project",".DS_Store"]
var excludedDirectories = ['css', 'dist','node_modules','webdriver-ts'];

// http://stackoverflow.com/questions/13786160/copy-folder-recursively-in-node-js
function copyFileSync( source, target ) {

    var targetFile = target;

    //if target is a directory a new file with the same name will be created
    if ( fs.existsSync( target ) ) {
        if ( fs.lstatSync( target ).isDirectory() ) {
            targetFile = path.join( target, path.basename( source ) );
        }
    }

    fs.writeFileSync(targetFile, fs.readFileSync(source));
}

function include(name) {
		if (name.indexOf("binding.scala")>-1) {
			console.log('name.indexOf("binding.scala")>-1', name.indexOf("/target")>-1, name.indexOf("/target/web")>-1, name);
				if (name.indexOf("/target")>-1) {
					return name.endsWith('/target') || name.indexOf("/target/web")>-1;
				}
		}
		if (excludes.every(ex => name.indexOf(ex)==-1)) {
			// console.log("<- filter", name);
			return true;
		} else {
			return false;
		}
}

function copyFolderRecursiveSync( source, target ) {
    var files = [];

    //check if folder needs to be created or integrated
    var targetFolder = path.join( target, path.basename( source ) );
    if ( !fs.existsSync( targetFolder ) ) {
        fs.mkdirSync( targetFolder );
    }

    //copy
    if ( fs.lstatSync( source ).isDirectory() ) {
        files = fs.readdirSync( source );
        files.forEach( function ( file ) {
			var curSource = path.join( source, file );
			if (include(curSource)) {
				if ( fs.lstatSync( curSource ).isDirectory() ) {
					console.log("copy dir "+curSource);
					copyFolderRecursiveSync( curSource, targetFolder );
				} else if ( fs.lstatSync( curSource ).isSymbolicLink() ) {
					console.log("**** LINK");
				} else {
					// console.log("copy file "+curSource);
					copyFileSync( curSource, targetFolder );
				}
			}
        } );
    }
}

_.each(fs.readdirSync('.'), function(name) {
	if(fs.statSync(name).isDirectory() && name[0] !== '.' && excludedDirectories.indexOf(name)==-1) {
		console.log("dist"+path.sep+name);
		fs.mkdirSync("dist"+path.sep+name);
		copyFolderRecursiveSync(name, "dist");

/*		fs.mkdirSync("dist"+path.sep+name);
		if (fs.existsSync(name+path.sep+"dist")) {
			fs.mkdirSync("dist"+path.sep+name+path.sep+"dist");
			fs.copySync(name+path.sep+"dist", "dist"+path.sep+name+path.sep+"dist");
			if (fs.existsSync(name+path.sep+"index.html")) {
				fs.copySync(name+path.sep+"index.html", "dist"+path.sep+name+path.sep+"index.html");
			}
		} else {
			if (fs.existsSync(name+path.sep+"index.html")) {
				fs.copySync(name+path.sep+"index.html", "dist"+path.sep+name+path.sep+"index.html");
			}
		} */
	}
});

fs.copySync("stem-v0.2.70-non-keyed/node_modules/babel-polyfill/dist/polyfill.min.js","dist/stem-v0.2.70/node_modules/babel-polyfill/dist");
fs.copySync("slim-js-v3.3.0-non-keyed/node_modules/slim-js/src/Slim.js","dist/slim-js-v3.3.0/node_modules/slim-js/src/Slim.js");

