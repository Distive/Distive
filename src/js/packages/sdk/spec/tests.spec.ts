import * as fc from 'fast-check'
import { ok } from 'neverthrow'
import SDK, { GetThreadInput, SDKConfig, UpsertPostInput } from '..'


const client = SDK({ serverId: 'rrkah-fqaaa-aaaaa-aaaaq-cai', host: 'http://127.0.0.1:8000' })
    ._unsafeUnwrap()

const CHANNEL_ID = `channel_${Math.round(Math.random() * 1000)}`
console.log('CHANNEL_ID',CHANNEL_ID)
test('createThread', async () => {
    const result = await client.upsertPost({
        channelId: CHANNEL_ID,
        content: "content_1",
        postId: "comment_1"
    })

    expect(result.isOk()).toBe(true)
    expect(result._unsafeUnwrap()).toBe("comment_1")
})

test('getThread', async () => {
    const result = await client.getThread({
        channelId: CHANNEL_ID,
        limit: 10,
        cursor:"",
        metadataUserIds:[]
    })
    expect(result.isOk()).toBe(true)
    expect(result._unsafeUnwrap().thread.length).toBeGreaterThan(0)
})

test('testThread', async () => {
    const result =( await client.getThread({
        channelId: CHANNEL_ID,
    }))._unsafeUnwrap()

    expect(Object.keys(result).length > 0).toBeTruthy()
})

test('toggleMetadata', async () => {
    const result = await client.toggleMetadata({
        channelId: CHANNEL_ID,
        label: "label_1",
        postId: "comment_1"
    })
    // Anonymous SDK doesn't support toggle_metadata
    expect(result.isOk()).toBeTruthy()
    expect(result._unsafeUnwrap()).toBeFalsy()
})
