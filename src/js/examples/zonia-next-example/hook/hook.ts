import { SDKConfig, Post, Thread, UpsertPostInput, Page, SDKResult, SDK } from '../../../sdk'
import { useState } from 'react'

type AddPostInput = Omit<UpsertPostInput, 'commentId' | 'channelId'>
type UpdatePostInput = { postId: string, content: string }

export interface ZoniaHook {
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

export interface ZoniaHookParam {
    channelID: string
    // cursor?: string
    limit?: number
    initialPage?: Page
}

export enum PostStatus {
    INITIAL,
    SENDING_REMOVE,
    SENDING_UPDATE,
    SENDING_ADD,
    SUCCESS_REMOVE,
    SUCCESS_UPDATE,
    SUCCESS_ADD,
    FAILURE_REMOVE,
    FAILURE_UPDATE,
    FAILURE_ADD,
} // INITIAL -> SENDING -> SUCCESS | FAILURE

export interface ThreadState {
    [postId: string]: PostThreadState
}

interface PostThreadState extends Post {
    status: PostStatus,
}


//sdk should be an argument to initUseZonia
export const useZonia = (SDK: SDK, params: ZoniaHookParam) => {

    const [loading, setLoading] = useState(false)
    const [error, setError] = useState("")

    const [thread, setThread] = useState<ThreadState>(marshallThread(params.initialPage?.thread ?? []))
    const initialThread = params.initialPage?.thread ?? []

    const lastPostId = initialThread.length ? initialThread[initialThread.length - 1].id : ''
    const [cursor, setCursor] = useState(lastPostId ?? '')

    const [remainingPostCount, setRemainingPostCount] = useState<number>(params.initialPage?.remainingCount ?? -1)

    function marshallThread(thread: Thread): ThreadState {
        return thread.reduce((prevThreadState, currPost): ThreadState => {
            return {
                ...prevThreadState,
                [currPost.id]: {
                    ...currPost,
                    status: PostStatus.INITIAL
                }
            }
        }, {} as ThreadState)
    }

    const loadMore = () => {
        setLoading(true)
        setError("")
        SDK.getThread({
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

    const addPost: ZoniaHook['addPost'] = ({ content, parentId }) => {
        const temporaryPostId = (Math.random() + 1).toString(36).substring(10)

        setThread(thread => ({
            ...thread,
            [temporaryPostId]: {
                content,
                id: temporaryPostId,
                created_at: Date.now(),
                replies: { remainingCount: 0, thread: [] },
                status: PostStatus.SENDING_ADD,
                userId: ''
            }
        }))

        SDK.upsertPost({
            channelId: params.channelID,
            content,
            parentId
        })
            .match(id => {
                setThread(prevThreadState => {
                    const { [temporaryPostId]: newPost, ...oldThread } = prevThreadState

                    return {
                        ...oldThread,
                        [id]: {
                            ...newPost,
                            status: PostStatus.SUCCESS_ADD
                        }
                    }

                })
            }, e => {
                setThread(prevThreadState => {
                    const { [temporaryPostId]: newPost, ...oldThread } = prevThreadState

                    return {
                        ...oldThread,
                        [temporaryPostId]: {
                            ...newPost,
                            status: PostStatus.FAILURE_ADD
                        }
                    }

                })
                // console.error(e) //perhaps call a callback supplied by the developer

            })

    }

    const updatePost: ZoniaHook['updatePost'] = ({ content, postId: postId }) => {
        if (!(postId in thread)) {
            return
        }

        const { [postId]: oldPost } = thread

        setThread({
            ...thread,
            [postId]: {
                ...oldPost,
                content,
                status: PostStatus.SENDING_UPDATE
            }
        })

        SDK.upsertPost({
            channelId: params.channelID,
            content,
            postId: postId,
        }).match(_ => {
            const { [postId]: newPost } = thread
            setThread({
                ...thread,
                [postId]: {
                    ...newPost,
                    status: PostStatus.SUCCESS_UPDATE
                }
            })
        }, e => {
            // console.error(e) //perhaps call a callback supplied by the developer
            const { [postId]: newPost } = thread
            setThread({
                ...thread,
                [postId]: {
                    ...newPost,
                    status: PostStatus.FAILURE_UPDATE
                }
            })
        })

    }



    const removePost: ZoniaHook['removePost'] = async (postId) => {
        if (!(postId in thread)) {
            return
        }

        const { [postId]: oldPost } = thread
        setThread({
            ...thread,
            [postId]: {
                ...oldPost,
                status: PostStatus.SENDING_REMOVE
            }
        })
        SDK.removePost({
            channelId: params.channelID,
            postId
        })
            .match(_ => {
                const { [postId]: newPost, ...newThread } = thread
                setThread({
                    ...newThread,
                    [postId]: {
                        ...newPost,
                        status: PostStatus.SUCCESS_REMOVE
                    }
                })
            }, e => {
                // console.error(e) //perhaps call a callback supplied by the developer
                const { [postId]: newPost } = thread
                setThread({
                    ...thread,
                    [postId]: {
                        ...newPost,
                        status: PostStatus.FAILURE_REMOVE
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

}




export default useZonia