import { useZonia } from '../src'
import { Thread, Page, ZoniaResult } from '../../sdk'
import React, { useContext } from 'react'


const Component = () => {
    const zoniaHookResult = useZonia({ serverId: "rrkah-fqaaa-aaaaa-aaaaq-cai" })
    return zoniaHookResult.match(({ getThread, removePost, upsertPost }) => {
        return getThread({ channelId: "", cursor: "", limit: 10 })
            .

    }, err => <div>{err} </div>)
}


interface RenderThreadProps {
    thread: Thread
}

const RenderThread = ({ thread }: RenderThreadProps) => {
    const zoniaHookResult = useZonia({ serverId: "rrkah-fqaaa-aaaaa-aaaaq-cai" })
    return zoniaHookResult.match(({ page, removePost, upsertPost, loading, loadMore }) => {
        return <div>
            {renderThread(thread)}
            <div style={{ marginTop: 10 }}>
                {
                    loading ? <div>Loading...</div> :
                        page.match((page) =>
                            <div>  {renderThread(page.thread)} </div>
                            , err => <div>{err}</div>)

                }
            </div>
            <div style={{ marginTop: 10 }}>
                <button onClick={loadMore}>Load more</button>
            </div>
        </div>

    }, err => <div>{err} </div>)

}

function renderThread(thread: Thread): React.ReactNode {
    return thread.map(comment => {
        return <div style={{ marginTop: 10 }}>
            <div>{comment.content}</div>
            <div>{comment.userId}</div>
            <div style={{ marginLeft: 20 }}>
                <RenderThread thread={comment.replies.thread} />
            </div>
        </div>
    })
}
