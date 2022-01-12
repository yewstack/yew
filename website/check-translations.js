const {i18n: {locales}} = require('./docusaurus.config.js');
const util = require('util');
const exec = util.promisify(require('child_process').exec);
const path = require('path');
const fs = require('fs');
const os = require('os');
const dircompare = require('dir-compare');
const writeTranslations = require('./write-translations.js')


const temp = fs.mkdtempSync(path.join(os.tmpdir(), 'yew-website-'));


async function main() {
    await new Promise(resolve => {
        fs.cp('i18n', temp, {recursive: true}, () => {
            resolve()
        })
    })

    await writeTranslations()

    const result = await dircompare.compare(temp, 'i18n', {compareContent: true});
    if (result.same) {
        console.log("Translations unchanged");
    } else {
        console.error("Translations changed, please run `npm run write-translations` to generate the stubs");
        process.exitCode = 1;
    }
}

main()
    .catch(e => console.error(e))




