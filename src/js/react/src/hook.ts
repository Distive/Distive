import _SDK, { SDKConfig, Post, Thread, UpsertPostInput, Page } from '../../sdk'
import { Result } from 'neverthrow'
import { useState } from 'react'

type AddPostInput = Omit<UpsertPostInput, 'commentId'>
type UpdatePostInput = { commentId: string, content: string }

interface ZoniaHook {
    loading: boolean
    thread: ThreadState
    error: string
    addPost: (input: AddPostInput) => void
    updatePost: (input: UpdatePostInput) => void
    removePost: (postId: string) => void
    remainingPostCount: number
    loadMore: () => void
    // onPostFailure?: (error: string) => void
    // onPostSuccess?: () => void
}

interface ZoniaHookParam {
    channelID: string
    // cursor?: string
    limit?: number
    initialPage?: Page
}

export enum PostStatus {
    INITIAL,
    SENDING,
    SUCCESS,
    FAILURE
} // INITIAL -> SENDING -> SUCCESS | FAILURE

export interface ThreadState {
    [postId: string]: PostThreadState
}

interface PostThreadState extends Post {
    status: PostStatus,
}

type ErrorMessage = String


interface State {
    thread: ThreadState
    error: ErrorMessage
    loading: boolean
    remainingPostsCount: number //-1 initial state, 0 no more posts, >0 has more posts
    cursor: string
}

//sdk should be an argument to initUseZonia
export const initUseZonia = (args: SDKConfig) => {
    const marshallThread = (thread: Thread): ThreadState => {
        return thread.reduce((prevThreadState, currPost): ThreadState => {
            return {
                ...prevThreadState,
                [currPost.id]: {
                    ...currPost,
                    status: PostStatus.SUCCESS
                }
            }
        }, {} as ThreadState)
    }

    return (params: ZoniaHookParam): Result<ZoniaHook, ErrorMessage> => {
        const [loading, setLoading] = useState(false)
        const [thread, setThread] = useState<ThreadState>(marshallThread(params.initialPage?.thread ?? []))
        const [error, setError] = useState("")

        const initialThread = params.initialPage?.thread ?? []
        const lastPostId = initialThread.length ? initialThread[initialThread.length - 1].id : ''
        const [cursor, setCursor] = useState(lastPostId ?? '')

        const [remainingPostCount, setRemainingPostCount] = useState<number>(params.initialPage?.remainingCount ?? -1)

        return _SDK(args)
            .map(SDK => {
                const loadMore = async () => {
                    setError("")
                    setLoading(true)
                    await SDK.getThread({
                        channelId: params.channelID,
                        limit: params?.limit,
                        cursor
                    }).map(page => {

                        setThread({
                            ...thread,
                            ...marshallThread(page.thread)
                        })

                        setRemainingPostCount(page.remainingCount)

                        if (page.thread.length > 0) {
                            const lastPostId = page.thread[page.thread.length - 1].id
                            setCursor(lastPostId)
                        }

                        setLoading(false)

                    }).mapErr(err => {
                        setError(err.message)
                        setLoading(false)
                    })
                }

                const addPost: ZoniaHook['addPost'] = ({ channelId, content, parentId }) => {
                    const temporaryPostId = (Math.random() + 1).toString(36).substring(10)
                    setThread({
                        ...thread,
                        [temporaryPostId]: {
                            content,
                            id: temporaryPostId,
                            created_at: Date.now(),
                            replies: { remainingCount: 0, thread: [] },
                            status: PostStatus.SENDING,
                            userId: ''
                        }
                    })
                    SDK.upsertPost({
                        channelId,
                        content,
                        parentId
                    }).match(id => {
                        const { [temporaryPostId]: newPost, ...oldThread } = thread
                        setThread({
                            ...oldThread,
                            [id]: {
                                ...newPost,
                                status: PostStatus.SUCCESS
                            }
                        })
                    }, e => {
                        console.error(e) //perhaps call a callback supplied by the developer
                        const { [temporaryPostId]: newPost, ...oldThread } = thread
                        setThread({
                            ...oldThread,
                            [temporaryPostId]: {
                                ...newPost,
                                status: PostStatus.FAILURE
                            }
                        })
                    })

                }

                const updatePost: ZoniaHook['updatePost'] = ({ content, commentId }) => {
                    const { [commentId]: oldPost } = thread

                    setThread({
                        ...thread,
                        [commentId]: {
                            ...oldPost,
                            content,
                            status: PostStatus.SENDING
                        }
                    })

                    SDK.upsertPost({
                        channelId: params.channelID,
                        content,
                        commentId,
                    }).match(_ => {
                        const { [commentId]: newPost } = thread
                        setThread({
                            ...thread,
                            [commentId]: {
                                ...newPost,
                                status: PostStatus.SUCCESS
                            }
                        })
                    }, e => {
                        console.error(e) //perhaps call a callback supplied by the developer
                        const { [commentId]: newPost } = thread
                        setThread({
                            ...thread,
                            [commentId]: {
                                ...newPost,
                                status: PostStatus.FAILURE
                            }
                        })
                    })

                }



                const removePost: ZoniaHook['removePost'] = async (postId) => {
                    const { [postId]: oldPost } = thread
                    setThread({
                        ...thread,
                        [postId]: {
                            ...oldPost,
                            status: PostStatus.SENDING
                        }
                    })
                    SDK.removePost({
                        channelId: params.channelID,
                        postId
                    })
                        .match(_ => {
                            const { [postId]: newPost } = thread
                            setThread({
                                ...thread,
                                [postId]: {
                                    ...newPost,
                                    status: PostStatus.SUCCESS
                                }
                            })
                        }, e => {
                            console.error(e) //perhaps call a callback supplied by the developer
                            const { [postId]: newPost } = thread
                            setThread({
                                ...thread,
                                [postId]: {
                                    ...newPost,
                                    status: PostStatus.FAILURE
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
                    removePost
                }
            })
            .mapErr(({ message }) => message)


    }
}




export default initUseZonia