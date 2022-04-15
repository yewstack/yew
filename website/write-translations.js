const {
    i18n: { locales },
} = require('./docusaurus.config.js')
const util = require('util')
const exec = util.promisify(require('child_process').exec)

/**
 * @param {string} locale
 */
async function writeTranslation(locale) {
    // exec rejects when the subprocess exits with non-zero code
    const { stdout, stderr } = await exec(
        `npm run docusaurus -- write-translations --locale ${locale}`
    )
    console.log(stdout)
    console.error(stderr)
}

async function writeTranslations() {
    for (const locale of locales.filter((locale) => locale !== 'en')) {
        await writeTranslation(locale)
    }
}

module.exports = writeTranslations

if (require.main === module) {
    writeTranslations().catch((e) => console.error(e))
}
