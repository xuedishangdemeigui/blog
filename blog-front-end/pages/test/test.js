const article = `
# Header 1
## Header 2

*React-Markdown* is **Awesome**

\`$> ls -al\`

$C_L$

_italic_

**bold**

<h1>Hello world!</h1>
    `

const content_info = [
    {
        name: "zk-SNARK",
        articles: [
            {
                name: "算术电路"
            },
            {
                name: "R1CS"
            }
        ]
    },
    {
        name: "Compiler",
        articles: [
            {
                name: "词法分析"
            },
            {
                name: "语法分析"
            }
        ]
    },
]

module.exports = {
    article,
    content_info
}