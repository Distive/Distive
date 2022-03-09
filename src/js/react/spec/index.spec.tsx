import { initZoniaHook } from '../src'
import { Thread, Page, ZoniaResult } from '../../sdk/dist'
import React, { useContext } from 'react'
import { renderHook, act } from '@testing-library/react-hooks'
import { PostStatus } from '../src/hook'

const useZonia = initZoniaHook({ serverId: 'rrkah-fqaaa-aaaaa-aaaaq-cai' })

test('actions should set the loading state',async () => {
    const { result, waitForNextUpdate } = renderHook(() => useZonia
        ({
            channelID: 'test_channel',
            initialPage: { remainingCount: 0, thread: [] },
        }))
    // const { thread, addPost,loading, loadMore,removePost } = result.current._unsafeUnwrap()

    expect(result.current._unsafeUnwrap(). loading).toBe(false)
    act(() => {
       result.current._unsafeUnwrap(). addPost({
            channelId: 'test_channel',
            content: 'hello',
            parentId: '',
        })
    })

    const commentId = ()=>  Object.entries(result.current._unsafeUnwrap().thread)[0][1].id


    expect(result.current._unsafeUnwrap().thread[commentId()].status).toBe(PostStatus.SENDING)
    await waitForNextUpdate()
    expect(result.current._unsafeUnwrap().thread[commentId()].status).toBe(PostStatus.SUCCESS)

    act(()=>{
       result.current._unsafeUnwrap(). loadMore()
    })

    expect(result.current._unsafeUnwrap().loading).toBe(true)
    await waitForNextUpdate()
    expect(result.current._unsafeUnwrap().loading).toBe(false)
    expect(Object.keys(result.current._unsafeUnwrap().thread).length).toBe(1)
    
    act(()=>{
     result.current._unsafeUnwrap(). removePost(commentId())
    })

    expect(  result.current._unsafeUnwrap().thread[commentId()].status).toBe(PostStatus.SENDING)
    await waitForNextUpdate()
    expect(  result.current._unsafeUnwrap().thread[commentId()].status).toBe(PostStatus.SUCCESS)
    
})