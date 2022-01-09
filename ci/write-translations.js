const {i18n: {locales}} = require('../website/docusaurus.config.js');
const util = require('util');
const exec = util.promisify(require('child_process').exec);

/**
 * @param {string} locale
 */
async function writeTranslations(locale) {
    const {stdout, stderr} = await exec(`npm run docusaurus -- write-translations --locale ${locale}`)
    console.log(stdout)
    console.error(stderr)
}


locales
    .filter(locale => locale !== 'en')
    .forEach(async locale => await writeTranslations(locale))



