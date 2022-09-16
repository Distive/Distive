import React, { ReactNode, useEffect, useState } from 'react';
import initDistiveHook, { ThreadState, } from '@distive/react'

import InternalContext, { DefaultInternalContext } from './context/internalContext'


import './index.css'




interface Config {
    mode: 'FLAT'
    canisterId: string,
    reactions: Array<{ display: string, value: string }>
    resolveProfileImageUrl: (userId: string) => string
    newPageForNestedThread: boolean //if true, a new page will be opened for nested threads, else threads will be rendered under the post
}

export interface Components {
    RootContainer: React.FC<{}>
    ChannelContainer: React.FC<{}>
    NavBar: React.FC<{ activeChannelTab: string }>
    NavBarChannelTab: React.FC<{ title: string }>
    InputContainer: React.FC<{}>
    TextBox: React.FC<{}>
    SubmitButton: React.FC<{}>
    PhotoButton: React.FC<{}>
    // ReactionsPopup: React.FC<{}>
    PostsContainer: React.FC<{isReply:boolean}>
    Post: React.FC<{ state: ThreadState[''] }>
    // PostPopup: React.FC<{}>
    ImageUploadButton: React.FC<{}>
    InputButtonsContainer: React.FC<{}>
    PostsLoader: React.FC<{}>
    ChannelTopBar: React.FC<{}>
}

interface Props {
    config: Config
    Components: Components
}

export interface PostControlsProps {
    postId: string
}

export const DefaultComponent: React.FC<{}> = ({ children }) => {
    return <>
        {children}
    </>
}

export default ({ config: {
    reactions,
    resolveProfileImageUrl,
    newPageForNestedThread
},
    Components: DI
}: Props) => {
   
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


    return <InternalContext.Provider
        value={{
            ...DefaultInternalContext,
            activateReply(postId) {
                setReplying({ postId })
            },
            currentUserID: ''
        }}
    >
        <DI.RootContainer>
            <NavBar DI={DI} />
            <DI.ChannelContainer>
                <DI.ChannelTopBar>
                </DI.ChannelTopBar>
                <DI.PostsContainer isReply={false}>
                    <DI.Post
                        state={{
                            content: '“Art is anything you can get away with.” Andy Warhol',
                            created_at: Date.now(),
                            id: '1',
                            metadata: [{
                                count: 4,
                                is_toggled: [false],
                                label: 'reaction:😃'
                            }, {
                                count: 1,
                                is_toggled: [true],
                                label: 'reaction:👍🏽'
                            }],
                            replies: { remainingCount: 0, thread: [] },
                            status: 'INITIAL',
                            toggledMetadataLabels: [],
                            userId: 'NOSFERATU_69'

                        }}
                    />
                    <DI.Post
                        state={{
                            content: '“Art is anything you can get away with.” Andy Warhol',
                            created_at: Date.now(),
                            id: '1',
                            metadata: [{
                                count: 4,
                                is_toggled: [false],
                                label: 'reaction:😃'
                            }, {
                                count: 1,
                                is_toggled: [true],
                                label: 'reaction:👍🏽'
                            }],
                            replies: { remainingCount: 0, thread: [] },
                            status: 'INITIAL',
                            toggledMetadataLabels: [],
                            userId: 'NOSFERATU_69'

                        }}
                    />
                    <DI.Post
                        state={{
                            content: '“Art is anything you can get away with.” Andy Warhol',
                            created_at: Date.now(),
                            id: '1',
                            metadata: [{
                                count: 4,
                                is_toggled: [false],
                                label: 'reaction:😃'
                            }, {
                                count: 1,
                                is_toggled: [true],
                                label: 'reaction:👍🏽'
                            }],
                            replies: { remainingCount: 0, thread: [] },
                            status: 'INITIAL',
                            toggledMetadataLabels: [],
                            userId: 'NOSFERATU_69'

                        }}
                    />
                    <DI.Post
                        state={{
                            content: `Exactly what I was thinking of and surprised this was so low. However we recognise animals as living individuals whine we don't with ai (understandably at this time)`,
                            created_at: Date.now(),
                            id: '1',
                            metadata: [{
                                count: 4,
                                is_toggled: [false],
                                label: 'reaction:😃'
                            }, {
                                count: 1,
                                is_toggled: [true],
                                label: 'reaction:👍🏽'
                            }],
                            replies: { remainingCount: 0, thread: [] },
                            status: 'INITIAL',
                            toggledMetadataLabels: [],
                            userId: 'the_chaco_kid'

                        }}
                    >
                        <DI.PostsContainer isReply>
                            <DI.Post
                                state={{
                                    content: `Exactly what I was thinking of and surprised this was so low. However we recognise animals as living individuals whine we don't with ai (understandably at this time)`,
                                    created_at: Date.now(),
                                    id: '1',
                                    metadata: [{
                                        count: 4,
                                        is_toggled: [false],
                                        label: 'reaction:😃'
                                    }, {
                                        count: 1,
                                        is_toggled: [true],
                                        label: 'reaction:👍🏽'
                                    }],
                                    replies: { remainingCount: 0, thread: [] },
                                    status: 'INITIAL',
                                    toggledMetadataLabels: [],
                                    userId: 'the_chaco_kid'

                                }}
                            >
                                <DI.PostsContainer isReply>
                                    <DI.Post
                                        state={{
                                            content: `Yeah it’s just a new/different kind of art. It has its own pros/cons

                                            Pros: it can make tons of beautiful ideas come to life without requiring years of practice, could break things like video game design wide open by cutting art costs
                                            
                                            Cons: people won’t respect it as much because it’s “too easy”, lacks the heart/soul of handcrafted art, currently limited to 2D digital images
                                            
                                            It’s best quality right now is that it’s provocative, gets the people going`,
                                            created_at: Date.now(),
                                            id: '1',
                                            metadata: [{
                                                count: 4,
                                                is_toggled: [false],
                                                label: 'reaction:😃'
                                            }, {
                                                count: 1,
                                                is_toggled: [true],
                                                label: 'reaction:👍🏽'
                                            }],
                                            replies: { remainingCount: 0, thread: [] },
                                            status: 'INITIAL',
                                            toggledMetadataLabels: [],
                                            userId: 'the_chaco_kid'

                                        }}
                                    >

                                    </DI.Post>
                                    <DI.Post
                                        state={{
                                            content: `I think what makes art interesting (at least to me) is the effort put into it.. as an artist, this scares me in this already competitive world.`,
                                            created_at: Date.now(),
                                            id: '1',
                                            metadata: [{
                                                count: 4,
                                                is_toggled: [false],
                                                label: 'reaction:😃'
                                            }, {
                                                count: 1,
                                                is_toggled: [true],
                                                label: 'reaction:👍🏽'
                                            }],
                                            replies: { remainingCount: 0, thread: [] },
                                            status: 'INITIAL',
                                            toggledMetadataLabels: [],
                                            userId: 'the_chaco_kid'

                                        }}
                                    >
                                        <DI.PostsContainer isReply>
                                    <DI.Post
                                        state={{
                                            content: `Yeah it’s just a new/different kind of art. It has its own pros/cons

                                            Pros: it can make tons of beautiful ideas come to life without requiring years of practice, could break things like video game design wide open by cutting art costs
                                            
                                            Cons: people won’t respect it as much because it’s “too easy”, lacks the heart/soul of handcrafted art, currently limited to 2D digital images
                                            
                                            It’s best quality right now is that it’s provocative, gets the people going`,
                                            created_at: Date.now(),
                                            id: '1',
                                            metadata: [{
                                                count: 4,
                                                is_toggled: [false],
                                                label: 'reaction:😃'
                                            }, {
                                                count: 1,
                                                is_toggled: [true],
                                                label: 'reaction:👍🏽'
                                            }],
                                            replies: { remainingCount: 0, thread: [] },
                                            status: 'INITIAL',
                                            toggledMetadataLabels: [],
                                            userId: 'the_chaco_kid'

                                        }}
                                    >

                                    </DI.Post>
                                    <DI.Post
                                        state={{
                                            content: `I think what makes art interesting (at least to me) is the effort put into it.. as an artist, this scares me in this already competitive world.`,
                                            created_at: Date.now(),
                                            id: '1',
                                            metadata: [{
                                                count: 4,
                                                is_toggled: [false],
                                                label: 'reaction:😃'
                                            }, {
                                                count: 1,
                                                is_toggled: [true],
                                                label: 'reaction:👍🏽'
                                            }],
                                            replies: { remainingCount: 0, thread: [] },
                                            status: 'INITIAL',
                                            toggledMetadataLabels: [],
                                            userId: 'the_chaco_kid'

                                        }}
                                    >

                                    </DI.Post>
                                </DI.PostsContainer> 
                                    </DI.Post>
                                </DI.PostsContainer> 
                            </DI.Post>
                        </DI.PostsContainer>
                    </DI.Post>

                </DI.PostsContainer>
                <DI.InputContainer>
                    <DI.TextBox />
                    <DI.InputButtonsContainer>
                        <DI.PhotoButton />
                        <DI.SubmitButton />
                    </DI.InputButtonsContainer>
                </DI.InputContainer>
            </DI.ChannelContainer>
        </DI.RootContainer>

    </InternalContext.Provider>
}

function NavBar({ DI }: { DI: Components }) {
    const [index, setIndex] = useState('NOOOM')
    return <DI.NavBar activeChannelTab={index}>
        <DI.NavBarChannelTab title='NOOOM' />
        <DI.NavBarChannelTab title='NOOOM 2' />
    </DI.NavBar>;
}


