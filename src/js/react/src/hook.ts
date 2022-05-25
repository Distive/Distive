import { Post, Thread, UpsertPostInput, Page, SDK, ToggleMetadataInput as _ToggleMetadataInput } from '@distive/sdk'
import { useEffect, useState } from 'react'

type AddPostInput = Omit<UpsertPostInput, 'commentId' | 'channelId'>
type UpdatePostInput = { postId: string; content: string; parentId?: string }
type ToggleMetadataInput = Omit<_ToggleMetadataInput, 'channelId'>

export interface DistiveHook {
    loading: boolean
    thread: ThreadState
    error: string

    addPost: (input: AddPostInput) => void
    updatePost: (input: UpdatePostInput) => void
    removePost: (postId: string) => void
    toggleMetadata: (input: ToggleMetadataInput) => void

    remainingPostCount: number
    loadMore: () => void
}

interface PostStatusCallback {
    id: string
    status: 'SUCCESS' | 'FAILURE' | 'SENDING'
    type: 'REMOVE' | 'UPDATE' | 'ADD' | 'REPLY' | 'METADATA'
    message: string
}

export interface DistiveHookParam {
    channelID: string
    // cursor?: string
    limit?: number
    initialPage?: Page
    onPostStatusChange?: (payload: PostStatusCallback) => void
}



// INITIAL -> SENDING -> SUCCESS | FAILURE
export type PostStatus = `${PostStatusCallback['status']}_${PostStatusCallback['type']}` | 'INITIAL'

export interface ThreadState {
    [postId: string]: PostThreadState,
}

interface PostThreadState extends Post {
    status: PostStatus,
    toggledMetadataLabels: string[],
}

function marshallThread(thread: Thread): ThreadState {
    return thread.reduce((prevThreadState, currPost): ThreadState => {
        return {
            ...prevThreadState,
            [currPost.id]: {
                ...currPost,
                status: 'INITIAL',
                toggledMetadataLabels: currPost.metadata.filter(m => m.is_toggled).map(m => m.label),
            },
        }
    }, {} as ThreadState)
}
//sdk should be an argument to initUseDistive
export const useDistive = (SDK: SDK, params: DistiveHookParam): DistiveHook => {
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

    const addPost: DistiveHook['addPost'] = ({ content, parentId }) => {
        const temporaryPostId = (Math.random() + 1).toString(36).substring(10)

        onPostStatusChange({
            id: parentId ?? '',
            status: 'SENDING',
            type: parentId ? 'REPLY' : 'ADD',
            message: '',
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
                    metadata: [],
                    toggledMetadataLabels: [],
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
                    type: parentId ? 'REPLY' : 'ADD',
                    message: '',
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
                                            userId: '',
                                            metadata: []
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
                    type: parentId ? 'REPLY' : 'ADD',
                    message: '',
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

    const updatePost: DistiveHook['updatePost'] = ({ content, postId, parentId }) => {
        if (!(postId in thread)) {
            return
        }

        onPostStatusChange({
            id: postId,
            status: 'SENDING',
            type: 'UPDATE',
            message: '',
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
            parentId
        })
            .match(
                (_) => {

                    onPostStatusChange({
                        id: postId,
                        status: 'SUCCESS',
                        type: 'UPDATE',
                        message: '',
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
                        type: 'UPDATE',
                        message: e.message,
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

    const removePost: DistiveHook['removePost'] = async (postId) => {
        if (!(postId in thread)) {
            return
        }

        onPostStatusChange({
            id: postId,
            status: 'SENDING',
            type: 'REMOVE',
            message: ''
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
                onPostStatusChange({
                    id: postId,
                    status: 'SUCCESS',
                    type: 'REMOVE',
                    message: ''
                })
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
                onPostStatusChange({
                    id: postId,
                    status: 'FAILURE',
                    type: 'REMOVE',
                    message: e.message,
                })
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

    const toggleMetadata: DistiveHook['toggleMetadata'] = (input: ToggleMetadataInput) => {

        onPostStatusChange({
            id: input.postId,
            status: 'SENDING',
            type: 'METADATA',
            message: '',
        })

        setThread(prevThreadState => {
            const { [input.postId]: oldPost } = prevThreadState
            return {
                ...prevThreadState,
                [input.postId]: {
                    ...oldPost,
                    status: 'SENDING_METADATA',
                },
            }
        })

        SDK.toggleMetadata({
            channelId: params.channelID,
            postId: input.postId,
            label: input.label,
        }).match(
            (result) => {


                if (result) {
                    onPostStatusChange({
                        id: input.postId,
                        status: 'SUCCESS',
                        type: 'METADATA',
                        message: '',
                    })

                    setThread(prevThreadState => {
                        const { [input.postId]: oldPost } = prevThreadState
                        return {
                            ...prevThreadState,
                            [input.postId]: {
                                ...oldPost,
                                status: 'SUCCESS_METADATA',
                                toggledMetadataLabels:
                                    (() => {
                                        const { toggledMetadataLabels } = oldPost
                                        if (toggledMetadataLabels.includes(input.label)) {
                                            return toggledMetadataLabels.filter(l => l !== input.label)
                                        } else {
                                            return [...toggledMetadataLabels, input.label]
                                        }
                                    })(),
                            },
                        }
                    })
                } else {
                    onPostStatusChange({
                        id: input.postId,
                        status: 'FAILURE',
                        type: 'METADATA',
                        message: 'User must be authenticated to toggle metadata',
                    })

                    setThread(prevThreadState => {
                        const { [input.postId]: oldPost } = prevThreadState
                        return {
                            ...prevThreadState,
                            [input.postId]: {
                                ...oldPost,
                                status: 'FAILURE_METADATA',
                            },
                        }
                    })
                }
            },
            (e) => {
                onPostStatusChange({
                    id: input.postId,
                    status: 'FAILURE',
                    type: 'METADATA',
                    message: e.message,
                })

                setThread(prevThreadState => {
                    const { [input.postId]: oldPost } = prevThreadState
                    return {
                        ...prevThreadState,
                        [input.postId]: {
                            ...oldPost,
                            status: 'FAILURE_METADATA',
                        },
                    }
                })
            })
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
        toggleMetadata
    }
}

export default useDistive
