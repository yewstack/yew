const { visit } = require('unist-util-visit')

function rustDocHiddenLines() {
    return (tree) => {
        visit(tree, 'code', (node) => {
            if (!node.lang || !node.lang.startsWith('rust')) return

            const lines = node.value.split('\n')
            const result = []

            for (const line of lines) {
                if (line === '#') {
                    continue
                }
                if (line.startsWith('# ')) {
                    continue
                }
                if (line.startsWith('## ')) {
                    // escape
                    result.push(line.slice(2))
                    continue
                }
                result.push(line)
            }

            node.value = result.join('\n')
        })
    }
}

module.exports = rustDocHiddenLines
