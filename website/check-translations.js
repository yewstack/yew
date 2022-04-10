const {
    i18n: { defaultLocale, locales },
} = require('./docusaurus.config.js')
const util = require('util')
const exec = util.promisify(require('child_process').exec)
const path = require('path')
const fs = require('fs')
const os = require('os')
const dircompare = require('dir-compare')
const writeTranslations = require('./write-translations.js')

const VERSION_NAME_CURRENT = 'current'
const VERSIONS = (async () => {
    const listedFiles = await fs.promises.readdir('versioned_docs', {
        withFileTypes: true,
    })
    return [VERSION_NAME_CURRENT].concat(
        listedFiles.filter((e) => e.isDirectory()).map((e) => e.name)
    )
})()

async function checkSuperfluousTranslations() {
    const versions = await VERSIONS
    let success = true
    for (const locale of locales) {
        if (locale === defaultLocale) {
            continue
        }
        for (const version of versions) {
            let isCurrentVersion = version == VERSION_NAME_CURRENT
            const originDir = isCurrentVersion
                ? 'docs'
                : path.join('versioned_docs', version)
            const localeDir = path.join(
                'i18n',
                locale,
                'docusaurus-plugin-content-docs',
                version
            )
            if (
                !(await fs.promises.access(localeDir, fs.constants.F_OK).then(
                    (_) => true,
                    (_) => false
                ))
            ) {
                console.warn(
                    `Missing translations for locale ${locale}, version ${version}.`
                )
                continue
            }

            const result = await dircompare.compare(originDir, localeDir)
            if (!result.diffSet) {
                throw new Error('Expected diff set')
            }
            const superfluous = result.diffSet
                .filter((e) => e.state === 'right')
                .map((e) => path.join(e.path2, e.name2))
            if (superfluous.length > 0) {
                let severity = isCurrentVersion ? console.error : console.warn
                severity(
                    `Found superfluous translations for locale ${locale}, version ${version}:`,
                    superfluous
                )
                if (isCurrentVersion) success = false
            }
        }
    }
    return success
}

async function checkWriteTranslations() {
    const temp = await fs.promises.mkdtemp(
        path.join(os.tmpdir(), 'yew-website-')
    )
    await new Promise((resolve) => {
        fs.cp('i18n', temp, { recursive: true }, () => {
            resolve()
        })
    })

    await writeTranslations()

    const result = await dircompare.compare(temp, 'i18n', {
        compareContent: true,
    })
    if (result.same) {
        console.log('Translations unchanged')
        return true
    } else {
        console.error(
            'Translations changed, please run `npm run write-translations` to generate the stubs'
        )
        return false
    }
}

async function main() {
    let okay = true
    okay &= await checkSuperfluousTranslations()
    okay &= await checkWriteTranslations()

    if (!okay) {
        process.exitCode = 1
    }
}

main().catch((e) => {
    console.error(e)
    process.exitCode = 1
})
