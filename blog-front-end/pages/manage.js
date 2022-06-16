import React from 'react'
import { Button, Input, IconButton } from '@chakra-ui/react'
import { SearchIcon } from '@chakra-ui/icons'

import Logo from './components/logo'
import Test, { content_info } from './test/test'

import style from '../styles/Manage.module.scss'

class ArticleTable extends React.Component {
    constructor(props) {
        super(props)
        this.state = {
            content_info: ''
        }
    }

    async componentDidMount() {
        let content_info
        try {
            // article = await axios_get(url.article)      // deployment version
            content_info = Test.content_info                // test version
        } catch (err) {
            console.log(err)
        }

        console.log(content_info)

        this.setState({
            content_info: content_info
        })
    }

    render() {
        return (
            <div className={style.articleTable}>
                {content_info.map((category, i) => {
                    let element =
                        <div className={style.item}>
                            <div className={style.category}>
                                <div className={style.categoryName}>{category.name}</div>
                                <div className={style.categoryTimestamp}>1996-08-30 16:24:12</div>
                                <div className={style.categoryBtn}>
                                    <IconButton aria-label='Search database' icon={<SearchIcon />} size="xs" />
                                    <IconButton aria-label='Search database' icon={<SearchIcon />} size="xs" />
                                    <IconButton aria-label='Search database' icon={<SearchIcon />} size="xs" />
                                </div>
                            </div>
                            {category.articles.map((article, j) => {
                                return (
                                    <div className={style.article}>
                                        <div className={style.articleName}>{article.name}</div>
                                        <div className={style.articleTimestamp}>1996-08-30 16:24:12</div>
                                        <div className={style.articleBtn}>
                                            <IconButton aria-label='Search database' icon={<SearchIcon />} size="xs" />
                                        </div>
                                    </div>
                                )
                            })}
                        </div>
                    return element
                })}
            </div>
        )
    }
}

export default class extends React.Component {
    render() {
        return (
            <div className={style.all}>
                <div className={style.header}>
                    {/* avatar */}
                    <div className={style.logo}>
                        <Logo style={{ color: 'white' }} />
                    </div>
                    <div className={style.search}>
                        <Input className={style.searchInput} color='white' placeholder='search' size='sm' />
                        <IconButton aria-label='Search database' icon={<SearchIcon />} size="sm" />
                    </div>
                    <div className={style.add_btn}>
                        <Button className={style.btn} size='sm'>Add Article</Button>
                    </div>
                </div>
                <div className={style.main}>
                    <ArticleTable />
                </div>
            </div>
        )
    }
}