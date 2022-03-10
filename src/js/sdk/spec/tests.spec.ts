import * as fc from 'fast-check'
import { ok } from 'neverthrow'
import SDK, { GetThreadInput, SDKConfig, UpsertPostInput } from '..'

const DEMO_DATA: UpsertPostInput[] = []

const client = SDK({ serverId: 'rrkah-fqaaa-aaaaa-aaaaq-cai' })._unsafeUnwrap()

test('createThread', async () => {
    const result = await client.upsertPost({
        channelId: "channel_1",
        content: "content_1",
        commentId: "comment_1"
    })
    
    expect(result.isOk()).toBe(true)
    expect(result._unsafeUnwrap()).toBe("comment_1")



})

// created_at should be in unix time