var _ = require('lodash');
var exec = require('child_process').execSync;
var fs = require('fs-extra');
var path = require('path');
var yargs = require ('yargs');

let args = yargs(process.argv)
.usage("$0 [--bootstrap --minimal]")
.help('help')
.boolean('bootstrap')
.boolean('minimal')
.argv;

if (args.bootstrap ^ args.minimal == false) {
	console.log("ERROR: You must either choose bootstrap or minimal");
} else {
	if (args.bootstrap) {
		fs.copySync("css/useOriginalBootstrap.css","css/currentStyle.css");
		let bootstrap = fs.readFileSync("css/bootstrap/dist/css/bootstrap.min.css")
		let mainCss = fs.readFileSync("css/main.css")
		let str = `<dom-module id="shared-styles"><template><style>${bootstrap}\n${mainCss}</style></template></dom-module>`;
		fs.writeFileSync("polymer-v2.0.0-non-keyed/src/shared-styles.html", str);
	} else {
		fs.copySync("css/useMinimalCss.css","css/currentStyle.css");
		let minCss = fs.readFileSync("css/useMinimalCss.css")
		let str = `<dom-module id="shared-styles"><template><style>${minCss}</style></template></dom-module>`;
		fs.writeFileSync("polymer-v2.0.0-non-keyed/src/shared-styles.html", str);
	}
}


/*
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
				if (name.indexOf("/target")>-1) {
					return name.indexOf("/target/web")>-1;
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
			if (include(file)) {
				var curSource = path.join( source, file );
				if ( fs.lstatSync( curSource ).isDirectory() ) {
					// console.log("copy dir "+curSource);
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

	}
});

*/