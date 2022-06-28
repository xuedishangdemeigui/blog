import React from 'react'
import { Avatar, Input, IconButton } from '@chakra-ui/react'
import { SearchIcon } from '@chakra-ui/icons'
import ChakraUIRenderer from 'chakra-ui-markdown-renderer';
import ReactMarkdown from 'react-markdown'
import remarkGfm from 'remark-gfm'
import remarkMath from 'remark-math'
import rehypeKatex from 'rehype-katex'
import rehypeRaw from 'rehype-raw'

import { url } from '../conf'
import { axios_get } from '../utils/axios'
import { theme } from './themes/markdown'
import Test from './test/test'
import Logo from './components/logo';

import style from '../styles/Test.module.scss'
import 'katex/dist/katex.min.css'

/* article component */
class ArticleContent extends React.Component {
    constructor(props) {
        super(props)
        this.state = {
            article: ''
        }
    }

    async componentDidMount() {
        let article
        try {
            // article = await axios_get(url.article)      // deployment version
            article = Test.article                      // test version
        } catch (err) {
            console.log(err)
        }

        this.setState({
            article: article
        })
    }

    render() {
        return (
            <ReactMarkdown
                className={style.markdown}
                components={ChakraUIRenderer(theme)}
                remarkPlugins={[remarkMath, remarkGfm]}
                rehypePlugins={[rehypeKatex, rehypeRaw]}
                children={this.state.article}
            />
        )
    }
}

/* content component */
class Content extends React.Component {
    constructor(props) {
        super(props)
    }

    contentRender(content) {
        return (
            <div className={style.article}>
                <ul>
                    {content.map((category, i) => {
                        let element =
                            <li key={i}>
                                {category.name}
                                <ul>
                                    {category.children.map((article, j) => {
                                        return <li key={j}><a href='#'>{article.name}</a></li>
                                    })}
                                </ul>
                            </li>
                        return element
                    })}
                </ul>
            </div>
        )
    }

    render() {
        let content = this.props.content

        return (
            // this.contentRender(content)
            <ArticleContent />
        );
    }
}

/* index component */
class Index extends React.Component {
    constructor(props) {
        super(props)
    }

    render() {
        let content = this.props.content

        return (
            <div className={style.articleList}>
                <ul>
                    {content.map((category, i) => {
                        let element =
                            <li tabIndex="0" key={i}>
                                <div>{category.name}</div>
                                <ul>
                                    {category.children.map((article, j) => {
                                        return <li key={j}><a href='#'>{article.name}</a></li>
                                    })}
                                </ul>
                            </li>
                        return element
                    })}
                </ul>
            </div>
        );
    }
}

export default class extends React.Component {
    static async getInitialProps({ req }) {
        // let content_info = await axios_get(url.content)
        let content_info = Test.content_info
        console.log(content_info)
        return { content_info }
    }

    render() {
        return (
            <div className={style.all}>
                {/* index part in the left */}
                <div className={style.index}>
                    {/* avatar and search */}
                    <div className={style.indexHeader}>
                        {/* avatar */}
                        {/* <div className={style.indexTitle}>
                            <Avatar name='ZhangChi' src='/images/avator.jpeg'></Avatar>
                            <span className={style.titleText}>ZhangChi's Blog</span>
                        </div> */}
                        <Logo style={{ color: 'white' }} />
                        {/* search */}
                        <div className={style.indexSearch}>
                            <Input className={style.searchInput} color='white' placeholder='search' size='sm' />
                            <IconButton aria-label='Search database' icon={<SearchIcon />} size="sm" />
                        </div>
                    </div>

                    {/* content part */}
                    <div className={style.contentIndex}>Content Index</div>
                    <Index content={this.props.content_info} />
                </div>

                {/* article part in the right */}
                <div className={style.articleBody}>
                    {/* personal information */}
                    <div className={style.articleHeader}>
                        <div className={style.intro}><a href='#'>About Me</a></div>
                    </div>
                    {/* article */}
                    <Content content={this.props.content_info} />
                </div>
            </div>
        )
    }
}