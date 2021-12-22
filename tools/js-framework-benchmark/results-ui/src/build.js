const fs = require('fs');
const rollup = require('rollup');
const buble = require('rollup-plugin-buble');
const terser = require('terser');
const CleanCSS = require('clean-css');
const path = require("path");
const yargs = require("yargs-parser");

const start = +new Date();

const RESULTS_PATH = path.resolve(__dirname + '/../../webdriver-ts/results');

const frameworks = yargs(process.argv, {array: ["framework"]}).framework || [];

function filterFramework(file) {
	return (
		frameworks.length === 0 ||
		frameworks.some(f => file.indexOf(f) > -1)
	);
}

function encodeBench(obj) {
    return [
        obj.benchmark.substr(0,2),
    //  +obj.min.toFixed(2),
    //  +obj.max.toFixed(2),
        +obj.mean.toFixed(2),
    //  +obj.median.toFixed(2),
    //  +obj.geometricMean.toFixed(2),
        +obj.standardDeviation.toFixed(2),
    //  obj.values.map(v => +v.toFixed(2)),
    ];
}

let libs = {
    keyed: {},
    unkeyed: {},
};

// grab result files, group by framework, bench types and encode benches into arrays
fs.readdirSync(RESULTS_PATH).filter(file => file.endsWith('.json') && filterFramework(file)).forEach(file => {
    var r = JSON.parse(fs.readFileSync(RESULTS_PATH + "/" + file, 'utf8'));
    var implGroup = r.keyed ? libs.keyed : libs.unkeyed;
    var libName = r.framework;

    if (implGroup[libName] == null) {
        implGroup[libName] = {
            name: libName,
            bench: {
                cpu: [],
                memory: [],
                startup: [],
            },
        };
    }

    implGroup[libName].bench[r.type].push(encodeBench(r));
});

// convert to arrays and sort
Object.keys(libs).forEach(implType => {
    libs[implType] = Object.values(libs[implType]).sort((a, b) => a.name.localeCompare(b.name));
});

fs.writeFileSync(path.resolve(__dirname + '/data.js'), 'export default ' + JSON.stringify(libs), 'utf8');

async function build() {
    const bundle = await rollup.rollup({
        input: __dirname + "/ui.js",
        plugins: [
            buble(),
        ],
    });

	// from docs (https://github.com/mishoo/UglifyJS2)
	const compressDefaults = {
		arguments: true,
		booleans: true,
		collapse_vars: true,
		comparisons: true,
		conditionals: true,
		dead_code: true,
		directives: true,
		drop_console: false,
		drop_debugger: true,
		evaluate: true,
		expression: false,
		global_defs: {},
		hoist_funs: false,
		hoist_props: true,
		hoist_vars: false,
		if_return: true,
		inline: 3,
		join_vars: true,
		keep_fargs: true,
		keep_fnames: false,
		keep_infinity: false,
		loops: true,
		negate_iife: true,
		passes: 1,
		properties: true,
		pure_funcs: null,
		pure_getters: "strict",
		reduce_funcs: true,
		reduce_vars: true,
		sequences: true,
		side_effects: true,
		switches: true,
		toplevel: false,
		top_retain: null,
		typeofs: true,
		unsafe: false,
		unsafe_comps: false,
		unsafe_Function: false,
		unsafe_math: false,
		unsafe_proto: false,
		unsafe_regexp: false,
		unsafe_undefined: false,
		unused: true,
	};

	const uglifyOpts = {
		compress: Object.assign({}, compressDefaults, {
			booleans: false,
			inline: 0,
			keep_fargs: false,
			hoist_props: false,
			loops: false,
			reduce_funcs: false,
			unsafe: true,
			unsafe_math: true,
		}),
	};

    const { output } = await bundle.generate({
        format: "iife",
    });

    const minJs = terser.minify(output[0].code, uglifyOpts).code;

    var css = fs.readFileSync(__dirname + '/bootstrap-reboot.css', 'utf8').replace(/\/\*[\s\S]+?\*\/\s*/gm, '');
    css += fs.readFileSync(__dirname + '/style.css', 'utf8');

    const minCss = new CleanCSS({level: 2}).minify(css).styles;

    const html = [
        '<!doctype html>',
        '<html>',
        '<head>',
        '<meta charset="utf-8">',
        '<meta http-equiv="x-ua-compatible" content="ie=edge">',
        '<title>Interactive Results</title>',
        '<meta name="viewport" content="width=device-width, initial-scale=1, shrink-to-fit=no">',
        '<style>',
        minCss,
        '</style>',
        '</head>',
        '<body>',
        '<script>',
        minJs.trim(),
        '</script>',
        '</body>',
        '</html>',
    ].join("");

    fs.writeFileSync(__dirname + "/../table.html", html, 'utf8');

    console.log("Built in " + (+new Date() - start) + "ms, (" + (html.length / 1024).toFixed(1) + "KB)");
}

build();