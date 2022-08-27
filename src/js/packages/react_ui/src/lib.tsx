import React, { useEffect, useState } from 'react';
import initDistiveHook, { ThreadState } from '@distive/react'
import InternalContext, { DefaultInternalContext } from './context/internalContext'
import { createEditor } from 'slate'
import { Slate, Editable, withReact } from 'slate-react'
import { BaseEditor, } from 'slate'
import { ReactEditor } from 'slate-react'
import { Post } from './Post';

type CustomElement = { type: 'paragraph'; children: CustomText[] }
type CustomText = { text: string }

declare module 'slate' {
    interface CustomTypes {
        Editor: BaseEditor & ReactEditor
        Element: CustomElement
        Text: CustomText
    }
}

export type PostThreadState = ThreadState['']

interface Config {
    canisterId: string,
    reactions: Array<{ display: string, value: string }>
    resolveProfileImageUrl: (userId: string) => string
    newPageForNestedThread: boolean //if true, a new page will be opened for nested threads, else threads will be rendered under the post
}

interface Props {
    config: Config
}

export interface PostControlsProps {
    postId: string
}

const newLocal = ({ userId }: { userId: string; }): JSX.Element => {
    return <div
        className='c_user-info'
    >
        <div className='c_user-info-details'>
            <img className='c_user-info-image' src='https://www.redditstatic.com/avatars/defaults/v2/avatar_default_6.png' />
            <div>
                <h1 className='c_user-info-name'>
                    Origyn
                </h1>
                <h2 className='c_user-info-id'>
                    {userId}
                </h2>
            </div>
        </div>
        <div className='c_item-count'>
            <span className='c_item-count-no'>4</span>
            <span className='c_item-text'>Items</span>
        </div>

        <div>
            {/* <h2>Items</h2> */}
            <div className='c_user-items'>
                <div className='c_user-item'>
                    <img className='c_user-item-preview' src='https://nitter.nl/pic/orig/media%2FFaWgfiLWYAI9C6v.jpg' />
                    <div className='c_user-item-details'>
                        <span className='c_user-item-name'>Pineapple Punk #1</span>
                        <div className='c_offer-button'>Make Offer</div>
                    </div>
                </div>
                <div className='c_user-item'>
                    <img className='c_user-item-preview' src='https://nitter.nl/pic/orig/media%2FFaWgfiLWYAI9C6v.jpg' />
                    <div className='c_user-item-details'>
                        <span className='c_user-item-name'>Pineapple Punk #2</span>
                        <div className='c_offer-button'>Make Offer</div>
                    </div>
                </div>
                <div className='c_user-item'>
                    <img className='c_user-item-preview' src='https://nitter.nl/pic/orig/media%2FFaWgfiLWYAI9C6v.jpg' />
                    <div className='c_user-item-details'>
                        <span className='c_user-item-name'>Pineapple Punk #3</span>
                        <div className='c_offer-button'>Make Offer</div>
                    </div>
                </div>
                <div className='c_user-item'>
                    <img className='c_user-item-preview' src='https://nitter.nl/pic/orig/media%2FFaWgfiLWYAI9C6v.jpg' />
                    <div className='c_user-item-details'>
                        <span className='c_user-item-name'>Pineapple Punk #4</span>
                        <div className='c_offer-button'>Make Offer</div>
                    </div>
                </div>

            </div>
        </div>
    </div>;
};
export default ({ config: { reactions, resolveProfileImageUrl, newPageForNestedThread } }: Props) => {
    //@ts-ignore
    const [editor] = useState(() => withReact(createEditor()))
    const [replying, setReplying] = useState({ postId: '' })

    const useDistive = initDistiveHook({
        serverId: 'vlxpi-eqaaa-aaaag-aajoq-cai'
    })._unsafeUnwrap()

    const { thread, loadMore, loading } = useDistive({
        channelID: 'books',
        limit: 20,
        onPostStatusChange: function ({ id, status, type, message }): void {
            console.log(`Post ${id} changed status to ${status}`);
        }
    })

    useEffect(() => {
        loadMore()
    }, [])

    const PostInput = () => <div className='input-container'>
        <Slate
            editor={editor}
            value={[]}
        >
            <Editable />
        </Slate>
        <div className='input-buttons'>
            <div className="icon input-button icon-photo" />
            <div className='icon input-button icon-submit' />
        </div>
    </div>;
    return <InternalContext.Provider
        value={{
            ...DefaultInternalContext,
            activateReply(postId) {
                setReplying({ postId })
            },
            currentUserID: '',
            UserInfoComponent: newLocal,
        }}
    >
        <div className='distive-root'>
            {!replying.postId ?
                <div className='thread-container'>
                    <PostInput />
                    {
                        loading ? 'Loading' : Object.values(thread).map(post => <Post {...post} />)
                    }
                </div> :
                <div className='reply-container'>
                    <div className='back-button' onClick={() => {
                        setReplying({ postId: '' })
                    }}>
                        <div className='icon icon-back' />
                        <span className='back-text'>Back</span>
                    </div>
                    <div className="reply-description">Replying to <span className='reply-username'>{
                        thread[replying.postId].userId
                    }</span></div>
                    <div className='thread-container'>
                        <PostInput />
                        {
                            Object.values(thread[replying.postId].replies.thread).map(post => <Post
                                status='INITIAL'
                                toggledMetadataLabels={[]}
                                {...post} />
                            )
                        }
                    </div>
                </div>}
        </div>
    </InternalContext.Provider>
}