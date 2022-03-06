import * as fc from 'fast-check'
import { ok } from 'neverthrow'
import SDK, { GetThreadInput, SDKConfig, UpsertPostInput } from '..'

const DEMO_DATA: UpsertPostInput[] = []

const client = SDK({ serverId: 'rrkah-fqaaa-aaaaa-aaaaq-cai' })._unsafeUnwrap()

test('createThread', async() => {
    // fc.assert(
    //     fc.property(
    //         fc.string(), fc.string(),
    //         (channelId, content) => {
    //              client
    //                 .upsertPost({
    //                     channelId,
    //                     content
    //                 })
    //                 .andThen(id => client.getThread({ channelId, cursor: id }))
    //                 .map(page => {
    //                     expect(page.thread.length).toBe(1)
    //                     expect(page.thread[0].content).toBe(content)
    //                     console.log(channelId)
    //                     console.dir(page,{depth:null})
    //                     client.removePost({channelId,postId: page.thread[0].id})
    //                 })
                    
    //         }
    //     )
    // )
})