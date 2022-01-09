const {i18n: {locales}} = require('./docusaurus.config.js');
const util = require('util');
const exec = util.promisify(require('child_process').exec);

/**
 * @param {string} locale
 */
async function writeTranslation(locale) {
    const {stdout, stderr} = await exec(`npm run docusaurus -- write-translations --locale ${locale}`)
    console.log(stdout)
    console.error(stderr)
}


async function writeTranslations() {
    for (const locale1 of locales
        .filter(locale => locale !== 'en')) {
        await writeTranslation(locale1);
    }
}

module.exports = writeTranslations

if (require.main === module) {
    writeTranslations().then(() => {
    })
}




