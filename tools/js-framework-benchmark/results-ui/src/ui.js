import results from './data';
import {
    defineElement as el,
    defineView as vw,
    createView as cv,
} from '../node_modules/domvm/dist/pico/domvm.pico.es.js';

// https://github.com/darkskyapp/string-hash/blob/master/index.js
function hash(str) {
    var hash = 5381,
        i = str.length;

    while(i) {
        hash = (hash * 33) ^ str.charCodeAt(--i);
    }

    /* JavaScript does bitwise operations (like XOR, above) on 32-bit signed
    * integers. Since we want the results to be always positive, convert the
    * signed int to an unsigned by doing an unsigned bitshift. */
    return hash >>> 0;
}

function decodeBench(arr) {
    return {
        name: arr[0],
//      min: arr[1],
//      max: arr[2],
        mean: arr[1],
//      median: arr[4],
//      geoMean: arr[5],
        stdDev: arr[2],
//      values: arr[3],

        factor: 0,
    };
}

function inflateBenches(lib) {
    lib.factors = {
        cpu: 0,
        memory: 0,
        startup: 0,
    };

    Object.keys(lib.factors).forEach(type => {
        var obj = {};
        lib.bench[type].map(b => obj[b[0]] = decodeBench(b));
        lib.bench[type] = obj;
    });
}

const benchDescr = {
    '01': '1k create',
    '02': '1k replace',
    '03': '10k update 1k',
    '04': '1k select 1',
    '05': '1k swap 2',
    '06': '1k remove 1',
    '07': '10k create',
    '08': '10k append 1k',
    '09': '10k clear',
    '21': 'loaded',
    '22': '1k create (5x)',
    '23': '1k update 100 (5x)',
    '24': '1k replace (5x)',            // default sort for memory?
    '25': '1k clear (5x)',
    '31': 'script bootup time',
    '32': 'consistently interactive',
    '33': 'main thread work cost',      // default sort for startup?
    '34': 'total byte weight',
};

function benchStruct(val) {
    return {
        cpu: {
            '01': val,
            '02': val,
            '03': val,
            '04': val,
            '05': val,
            '06': val,
            '07': val,
            '08': val,
            '09': val,
        },
        memory: {
            '21': val,
            '22': val,
            '23': val,
            '24': val,
            '25': val,
        },
        startup: {
            '31': val,
            '32': val,
            '33': val,
            '34': val,
        },
    };
}

const colorSteps = [
    1.000, 1.125, 1.250, 1.375,
    1.500, 1.625, 1.750, 1.875,
    2.000, 2.250, 2.500, 2.750,
    3.000, 3.250, 3.500, 4.000,
];

function colorClass(factor) {
    const last = colorSteps.length - 1;

    for (var i = 0; i < colorSteps.length; i++) {
        if (i == last || factor < colorSteps[i+1])
            break;
    }

    return "c" + (i < 10 ? "0" : "") + i;
}

function frameworkEnabled(type, name) {
    return STORE.enabled.frameworks[type][name];
}

function benchEnabled(type, name) {
    return STORE.enabled.benchmarks[type][name];
}

function eachEnabledLib(cb) {
    Object.keys(STORE.results).forEach(implType => {
        results[implType].forEach(lib => {
            if (frameworkEnabled(implType, lib.name))
                cb(implType, lib);
        });
    });
}

function eachEnabledBench(lib, cb) {
    Object.keys(lib.bench).forEach(benchType => {
        Object.keys(lib.bench[benchType]).forEach(num => {
            if (benchEnabled(benchType, num))
                cb(benchType, lib.bench[benchType][num]);
        });
    });
}

function anyEnabled(what, type) {
    var all = STORE.enabled[what][type];

    for (var name in all) {
        if (all[name])
            return true;
    }

    return false;
}

function clampMin(val, min) {
    return Math.max(val, min);
}

function setFactors() {
    var mins = {
        keyed: benchStruct(1e10),
        unkeyed: benchStruct(1e10),
    };

    // calc minimums
    eachEnabledLib((implType, lib) => {
        eachEnabledBench(lib, (benchType, res) => {
            var typeMins = mins[implType][benchType];
            typeMins[res.name] = clampMin(Math.min(typeMins[res.name], res[STORE.metric]), STORE.clamp[benchType]);
        });
    });

    // set factors
    eachEnabledLib((implType, lib) => {
        eachEnabledBench(lib, (benchType, res) => {
            var typeMins = mins[implType][benchType];
            res.factor = clampMin(res[STORE.metric], STORE.clamp[benchType]) / typeMins[res.name];
        });
    });

    eachEnabledLib((implType, lib) => {
        for (var benchType in lib.bench) {
            lib.factors[benchType] = 0;

            var i = 0;
            for (var num in lib.bench[benchType]) {
                if (benchEnabled(benchType, num)) {
                    i++;
                    lib.factors[benchType] += lib.bench[benchType][num].factor;
                }
            }
            lib.factors[benchType] /= i;
        }
    });
}

// TODO: must reset if needed row is disabled
function sortBy(metric, sortDir) {
    STORE.sortDir = sortDir;
    STORE.sortBy = metric;

    var typeNum = metric.split(".");

    // cpu, memory, startup
    var benchType = typeNum[0];
    var num = typeNum[1];

    if (benchType == "name")
        var cmp = (a, b) => STORE.sortDir * a.name.localeCompare(b.name);
    else if (num == null)
        var cmp = (a, b) => STORE.sortDir * (a.factors[benchType] - b.factors[benchType]);
    else {
        var cmp = (a, b) => {
            var aVals = a.bench[benchType][num],
                bVals = b.bench[benchType][num];

            return STORE.sortDir * (
                (aVals ? aVals[STORE.metric] : 1e9) -
                (bVals ? bVals[STORE.metric] : 1e9)
            );
        };
    }

    STORE.results.keyed.sort(cmp);
    STORE.results.unkeyed.sort(cmp);

    VIEW && VIEW.redraw();

    return false;
}

function nextSortDir(metric) {
    if (STORE.sortBy == metric)
        return STORE.sortDir * -1;
    else
        return 1;
}

function render(implType, benchType, aggr) {
    if (!anyEnabled("benchmarks", benchType))
        return null;

    var enabledFrameworks = STORE.results[implType].filter(lib => frameworkEnabled(implType, lib.name)),
        enabledBenches = Object.keys(benchStruct()[benchType]).filter(num => benchEnabled(benchType, num));

    return el("table", [
        el("caption", benchType),
        el("tr", [
            el("th", {class: "benchname"}, [
                el("a", {href: "#", onclick: [sortBy, "name", nextSortDir("name")]}, "Name"),
            ]),
            enabledFrameworks.map(lib =>
                el("th", lib.name)
            )
        ]),
        enabledBenches.map(num =>
            el("tr", [
                el("th", {class: "benchname"}, [
                    el("a", {href: "#", onclick: [sortBy, benchType + "." + num, nextSortDir(benchType + "." + num)]}, benchDescr[num]),
                ]),
                enabledFrameworks.map(lib => {
                    var bench = lib.bench[benchType][num];

                    if (bench == null)
                        return el("td");
                    else {
                        return el("td", {class: colorClass(bench.factor)}, [
                            el("div", {class: "value"}, bench[STORE.metric].toFixed(1)),
                            el("div", {class: "stddev"}, bench.stdDev.toFixed(1)),
                            el("div", {class: "factor"}, bench.factor.toFixed(1)),
                        ]);
                    }
                }),
            ])
        ),
        aggr && el("tr", [
            el("th", [
                el("a", {href: "#", onclick: [sortBy, benchType, nextSortDir(benchType)]}, "slowdown"),
            ]),
            enabledFrameworks.map(lib =>
                el("th", {class: colorClass(lib.factors[benchType])}, lib.factors[benchType].toFixed(2))
            )
        ]),
    ]);
}

// TODO?: DRY
function toggle(what, type, num) {
    var isEnabled = STORE.enabled[what][type][num] = !STORE.enabled[what][type][num];

    setFactors();

    if (what == "benchmarks" && !isEnabled && STORE.sortBy == type + "." + num)
        sortBy("cpu", 1);
    else
        sortBy(STORE.sortBy, STORE.sortDir);

    VIEW.redraw();

    return false;
}

function setAll(what, type, val) {
    var all = STORE.enabled[what][type];

    for (var name in all)
        all[name] = val;

    setFactors();
    sortBy(STORE.sortBy, STORE.sortDir);

    VIEW.redraw();

    return false;
}

function selectorTpl(what, fmtName) {
    var struct = STORE.enabled[what];

    return el("div", {id: what}, Object.keys(struct).map(type => [
        el("div", {class: "group"}, [
            el("h4", [
                type,
                el("div", {class: "toggle_all"}, [
                    el("a", {href: "#", onclick: [setAll, what, type, true]}, "All"),
                    " / ",
                    el("a", {href: "#", onclick: [setAll, what, type, false]}, "None"),
                ])
            ]),
            Object.keys(struct[type]).map(key =>
                el("label", [
                    el("input", {
                        type: "checkbox",
                        checked: STORE.enabled[what][type][key],
                        onchange: [toggle, what, type, key]
                    }),
                    el("strong", fmtName(key)),
                ])
            )
        ])
    ]));
}

function tablesTpl(implType) {
    if (!anyEnabled("frameworks", implType))
        return null;

    return el("div", {id: implType}, [
        el("h1", implType),
        render(implType, "cpu", true),
        render(implType, "memory", false),
        render(implType, "startup", false),
    ]);
}

const AppView = {
    render() {
        return el("div", {id: "results"}, [
            selectorTpl("benchmarks", name => benchDescr[name]),
            selectorTpl("frameworks", name => name),
            tablesTpl("keyed"),
            tablesTpl("unkeyed"),
        ]);
    }
};

const STORE = {
    metric: "mean",
    enabled: {
        benchmarks: benchStruct(true),
        frameworks: {
            keyed: {},
            unkeyed: {},
        },
    },
    clamp: {
        cpu: 16.66666,
        memory: 0,
        startup: 0,
    },
    sortBy: null,
    sortDir: null,
    results: results,
};

Object.keys(results).forEach(implType => {
    results[implType].forEach(lib => {
        inflateBenches(lib);
        // todo: use this hash in url to show only specific libs (e.g. #libs=abcd,3745)
        lib.hash = hash(lib.name).toString(16).substr(0, 4);
        STORE.enabled.frameworks[implType][lib.name] = true;
    });
});

setFactors();
sortBy("cpu", 1);

const VIEW = cv(AppView).mount(document.body);