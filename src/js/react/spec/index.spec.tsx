import { Thread, Page, ZoniaError, ErrorKind, Post, SDK } from 'zomia'
import { renderHook, act } from '@testing-library/react-hooks'
import useZonia, { PostStatus } from '../src/hook'
import { errAsync, fromSafePromise, ResultAsync } from 'neverthrow'

const stall = (ms: number) => new Promise(resolve => setTimeout(resolve, ms))

interface MockStorage {
    [key: string]: Post
}

let mockStorage: MockStorage = {}

const successSdk: SDK = {
    getThread({ channelId, cursor, limit }) {
        return fromSafePromise(stall(5))
            .map(() => {
                const thread: Thread = Object.values(mockStorage).map(i => ({
                    ...i,
                }))
                //find index of cursor in thread
                const cursorIndex = thread.findIndex(i => i.id === cursor)
                //use cursor and limit to paginate the thread
                const paginatedThread: Thread = thread.slice(cursorIndex + 1, cursorIndex + 1 + (limit ?? 10));

                return {
                    thread: paginatedThread,
                    remainingCount: thread.length - paginatedThread.length,
                }

            })
            .mapErr(e => ({ kind: ErrorKind.Internal, message: 'internal error' }))
    },
    removePost({ channelId, postId }) {
        return fromSafePromise(stall(5))
            .map(() => {
                const { [postId]: _, ...rest } = mockStorage
                mockStorage = rest
                return postId
            })
            .mapErr(e => ({ kind: ErrorKind.Internal, message: 'internal error' }))
    },
    upsertPost({ channelId, content, postId: commentId, parentId }) {
        return fromSafePromise(stall(5))
            .map(() => {
                const _commentId = commentId ?? [channelId, parentId, content].join('-')
                mockStorage = {
                    ...mockStorage,
                    [_commentId]: {
                        id: _commentId,
                        content,
                        created_at: 0,
                        replies: { remainingCount: 0, thread: [] },
                        userId: ''
                    }
                }
                return _commentId
            })
            .mapErr(e => ({ kind: ErrorKind.Internal, message: 'internal error' }))
    }
}

const failureSdk: SDK = {
    getThread() {
        return fromSafePromise(stall(5))
            .andThen(() => {
                return errAsync({ kind: ErrorKind.Internal, message: 'internal error' })
            }) as ResultAsync<Page, ZoniaError>
    },
    removePost() {
        return fromSafePromise(stall(5))
            .andThen(() => {
                return errAsync({ kind: ErrorKind.Internal, message: 'internal error' })
            }) as ResultAsync<string, ZoniaError>
    },
    upsertPost() {
        return fromSafePromise(stall(5))
            .andThen(() => {
                return errAsync({ kind: ErrorKind.Internal, message: 'internal error' })
            }) as ResultAsync<string, ZoniaError>
    }
}

test('actions should set the proper states (success)', async () => {

    const { result, waitForNextUpdate } = renderHook(() => useZonia(successSdk, {
        channelID: 'test_channel',
        initialPage: { remainingCount: 0, thread: [] },
    }))

    expect(result.current.loading).toBe(false)
    act(() => {
        result.current.loadMore()
    })
    expect(result.current.loading).toBe(true)
    await waitForNextUpdate()
    expect(result.current.loading).toBe(false)



    act(() => {
        result.current.addPost({
            content: 'test_content',
        })
    })

    for (const postId in result.current.thread) {
        expect(result.current.thread[postId].status).toBe('SENDING_ADD')
    }
    await waitForNextUpdate()

    for (const postId in result.current.thread) {
        expect(result.current.thread[postId].status).toBe('SUCCESS_ADD')
    }

    act(() => {
        for (const postId in result.current.thread) {
            result.current.removePost(postId)
        }
    })


    for (const postId in result.current.thread) {
        expect(result.current.thread[postId].status).toBe('SENDING_REMOVE')
    }

    await waitForNextUpdate()

    for (const postId in result.current.thread) {
        expect(result.current.thread[postId].status).toBe('SUCCESS_REMOVE')
    }




})

test('replies should set proper states (success)', async () => {
    const { result, waitForNextUpdate } = renderHook(() => useZonia(successSdk, {
        channelID: 'test_channel',
        initialPage: { remainingCount: 0, thread: [] },
    }))



    act(() => {
        result.current.addPost({
            content: 'test_content',
        })
    })

    await waitForNextUpdate()

    act(() => {
        for (const postId in result.current.thread) {
            result.current.addPost({
                content: '',
                parentId: postId,
            })
        }
    })

    expect(Object.values(result.current.thread).map(post =>
        post.status).some(status => status === 'SENDING_REPLY')).toBeTruthy()

    await waitForNextUpdate()


    expect(Object.values(result.current.thread).map(post =>
        post.status).some(status => status === 'SUCCESS_REPLY')).toBeTruthy()
})

test('replies should set proper states (failure)', async () => {
    const { result, waitForNextUpdate } = renderHook(() => useZonia(failureSdk, {
        channelID: 'test_channel',
        initialPage: { remainingCount: 0, thread: [] },
    }))



    act(() => {
        result.current.addPost({
            content: 'test_content',
        })
    })

    await waitForNextUpdate()

    act(() => {
        for (const postId in result.current.thread) {
            result.current.addPost({
                content: '',
                parentId: postId,
            })
        }
    })

    expect(Object.values(result.current.thread).map(post =>
        post.status).some(status => status === 'SENDING_REPLY')).toBeTruthy()

    await waitForNextUpdate()


    expect(Object.values(result.current.thread).map(post =>
        post.status).some(status => status === 'FAILURE_REPLY')).toBeTruthy()
})


test('actions should set the proper states (failure)', async () => {

    const { result, waitForNextUpdate } = renderHook(() => useZonia
        (failureSdk, {
            channelID: 'test_channel',
            initialPage: { remainingCount: 0, thread: [] },
        }))

    expect(result.current.loading).toBe(false)
    act(() => {
        result.current.loadMore()
    })
    expect(result.current.loading).toBe(true)
    await waitForNextUpdate()
    expect(result.current.loading).toBe(false)

    act(() => {
        result.current.addPost({
            content: 'test_content',
        })
    })

    for (const postId in result.current.thread) {
        expect(result.current.thread[postId].status).toBe('SENDING_ADD')
    }
    await waitForNextUpdate()

    for (const postId in result.current.thread) {
        expect(result.current.thread[postId].status).toBe('FAILURE_ADD')
    }

    act(() => {
        for (const postId in result.current.thread) {
            result.current.removePost(postId)
        }
    })

    for (const postId in result.current.thread) {
        expect(result.current.thread[postId].status).toBe('SENDING_REMOVE')
    }
    await waitForNextUpdate()

    for (const postId in result.current.thread) {
        expect(result.current.thread[postId].status).toBe('FAILURE_REMOVE')
    }


})


test('pagination', () => { })
test('error', () => { })
test('remaining post count', () => { })