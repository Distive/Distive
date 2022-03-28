//types should be standalone and not depend on sdk
import { Post, Thread, UpsertPostInput, Page, SDK } from 'zomia'
import { useEffect, useState } from 'react'

type AddPostInput = Omit<UpsertPostInput, 'commentId' | 'channelId'>
// interface AddPostInput {}
type UpdatePostInput = { postId: string; content: string }

export interface ZoniaHook {
    loading: boolean
    thread: ThreadState
    error: string

    addPost: (input: AddPostInput) => void
    updatePost: (input: UpdatePostInput) => void
    removePost: (postId: string) => void
    remainingPostCount: number
    loadMore: () => void
}

interface PostStatusCallback {
    id: string
    status: 'SUCCESS' | 'FAILURE' | 'SENDING'
    type: 'REMOVE' | 'UPDATE' | 'ADD' | 'REPLY'
}

export interface ZoniaHookParam {
    channelID: string
    // cursor?: string
    limit?: number
    initialPage?: Page
    onPostStatusChange?: (payload: PostStatusCallback) => void
}



// INITIAL -> SENDING -> SUCCESS | FAILURE
export type PostStatus = `${PostStatusCallback['status']}_${PostStatusCallback['type']}` | 'INITIAL'

export interface ThreadState {
    [postId: string]: PostThreadState
}

interface PostThreadState extends Post {
    status: PostStatus,

}

function marshallThread(thread: Thread): ThreadState {
    return thread.reduce((prevThreadState, currPost): ThreadState => {
        return {
            ...prevThreadState,
            [currPost.id]: {
                ...currPost,
                status: 'INITIAL',
            },
        }
    }, {} as ThreadState)
}
//sdk should be an argument to initUseZonia
export const useZonia = (SDK: SDK, params: ZoniaHookParam) => {
    const onPostStatusChange = params.onPostStatusChange ?? (() => { })

    const [loading, setLoading] = useState(false)
    const [error, setError] = useState('')

    const [thread, setThread] = useState<ThreadState>(
        marshallThread(params.initialPage?.thread ?? [])
    )

    const initialThread = params.initialPage?.thread ?? []

    const lastPostId = initialThread.length ? initialThread[initialThread.length - 1].id : ''
    const [cursor, setCursor] = useState(lastPostId ?? '')

    const [remainingPostCount, setRemainingPostCount] = useState<number>(
        params.initialPage?.remainingCount ?? -1
    )

    useEffect(() => {
        setThread({
            ...thread,
            ...marshallThread(params.initialPage?.thread ?? [])
        })
    }, [params.initialPage])



    const loadMore = () => {
        setLoading(true)
        setError('')
        SDK.getThread({
            channelId: params.channelID,
            limit: params?.limit,
            cursor,
        })
            .map((page) => {
                setThread({
                    ...thread,
                    ...marshallThread(page.thread),
                })

                setRemainingPostCount(page.remainingCount)

                if (page.thread.length > 0) {
                    const lastPostId = page.thread[page.thread.length - 1].id
                    setCursor(lastPostId)
                }

                setLoading(false)
            })
            .mapErr((err) => {
                setError(err.message)
                setLoading(false)
            })
    }

    const addPost: ZoniaHook['addPost'] = ({ content, parentId }) => {
        const temporaryPostId = (Math.random() + 1).toString(36).substring(10)

        onPostStatusChange({
            id: parentId ?? '',
            status: 'SENDING',
            type: parentId ? 'REPLY' : 'ADD'
        })

        setThread((prevThreadState) => ({
            ...prevThreadState,
            ...(parentId ? {
                [parentId]: {
                    ...prevThreadState[parentId],
                    status: 'SENDING_REPLY',
                }
            } : {
                [temporaryPostId]: {
                    content,
                    id: temporaryPostId,
                    created_at: Date.now(),
                    replies: { remainingCount: 0, thread: [] },
                    status: 'SENDING_ADD',
                    userId: '',
                }
            })
        }))


        SDK.upsertPost({
            channelId: params.channelID,
            content,
            parentId,
        }).match(
            (id) => {

                onPostStatusChange({
                    id,
                    status: 'SUCCESS',
                    type: parentId ? 'REPLY' : 'ADD'
                })

                setThread((prevThreadState) => {
                    const { [temporaryPostId]: newPost, ...oldThread } = prevThreadState

                    return {
                        ...oldThread,
                        ...(parentId ? {
                            [parentId]: {
                                ...oldThread[parentId],
                                status: 'SUCCESS_REPLY',
                                replies: {
                                    ...oldThread[parentId].replies,
                                    thread: [
                                        ...oldThread[parentId].replies.thread,
                                        {
                                            content,
                                            created_at: Date.now(),
                                            id,
                                            replies: { remainingCount: 0, thread: [] },
                                            userId: ''
                                        }
                                    ]
                                }
                            }
                        } : {
                            [id]: {
                                ...newPost,
                                id, //replace temporary id with real id
                                status: 'SUCCESS_ADD',
                            }
                        })
                    }
                })
            },
            (e) => {
                onPostStatusChange({
                    id: parentId ?? '',
                    status: 'FAILURE',
                    type: parentId ? 'REPLY' : 'ADD'
                })

                setThread((prevThreadState) => {
                    const { [temporaryPostId]: newPost, ...oldThread } = prevThreadState
                    return {
                        ...oldThread,
                        ...(parentId ? {
                            [parentId]: {
                                ...oldThread[parentId],
                                status: 'FAILURE_REPLY',
                            }
                        } : {
                            [temporaryPostId]: {
                                ...newPost,
                                status: 'FAILURE_ADD',
                            },
                        })
                    }
                })
                // console.error(e) //perhaps call a callback supplied by the developer
            }
        )
    }

    const updatePost: ZoniaHook['updatePost'] = ({ content, postId: postId }) => {
        if (!(postId in thread)) {
            return
        }

        onPostStatusChange({
            id: postId,
            status: 'SENDING',
            type: 'UPDATE'
        })

        setThread(prevThreadState => {
            const { [postId]: oldPost } = prevThreadState
            return {
                ...prevThreadState,
                [postId]: {
                    ...oldPost,
                    content,
                    status: 'SENDING_UPDATE',
                },
            }

        })

        SDK.upsertPost({
            channelId: params.channelID,
            content,
            postId: postId,
        })
            .match(
                (_) => {

                    onPostStatusChange({
                        id: postId,
                        status: 'SUCCESS',
                        type: 'UPDATE'
                    })

                    setThread(prevThreadState => {
                        const { [postId]: post } = prevThreadState
                        return {
                            ...prevThreadState,
                            [postId]: {
                                ...post,
                                status: 'SUCCESS_UPDATE',
                            },
                        }
                    })
                },
                (e) => {

                    onPostStatusChange({
                        id: postId,
                        status: 'FAILURE',
                        type: 'UPDATE'
                    })

                    setThread(prevThreadState => {
                        const { [postId]: post } = prevThreadState
                        return {
                            ...prevThreadState,
                            [postId]: {
                                ...post,
                                status: 'FAILURE_UPDATE',
                            },
                        }
                    })
                }
            )
    }

    const removePost: ZoniaHook['removePost'] = async (postId) => {
        if (!(postId in thread)) {
            return
        }

        onPostStatusChange({
            id: postId,
            status: 'SENDING',
            type: 'REMOVE'
        })

        setThread(prevThreadState => {
            const { [postId]: oldPost } = prevThreadState
            return {
                ...prevThreadState,
                [postId]: {
                    ...oldPost,
                    status: 'SENDING_REMOVE',
                },
            }
        })

        SDK.removePost({
            channelId: params.channelID,
            postId,
        }).match(
            (_) => {
                setThread(prevThreadState => {
                    const { [postId]: newPost, ...newThread } = prevThreadState

                    return {
                        ...newThread,
                        [postId]: {
                            ...newPost,
                            status: 'SUCCESS_REMOVE',
                        },
                    }
                })
            },
            (e) => {
                // console.error(e) //perhaps call a callback supplied by the developer
                setThread(prevThreadState => {
                    const { [postId]: newPost } = prevThreadState
                    return {
                        ...thread,
                        [postId]: {
                            ...newPost,
                            status: 'FAILURE_REMOVE',
                        },
                    }
                })
            }
        )
    }

    return {
        error,
        loadMore,
        loading,
        thread,
        addPost,
        updatePost,
        remainingPostCount,
        removePost,
    }
}

export default useZonia
