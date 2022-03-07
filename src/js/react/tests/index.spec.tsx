import initZoniaHook from '../src'
import { Thread, Page, ZoniaResult } from '../../sdk'
import React, { useContext } from 'react'
import { ThreadState } from '../src/hook'

const useZonia = initZoniaHook({ serverId: "rrkah-fqaaa-aaaaa-aaaaq-cai" })

const Component = () => {
    return <RenderThread />
}


interface RenderThreadProps {
    thread?: Thread
}

const RenderThread = ({ thread }: RenderThreadProps) => {
    const zoniaHookResult = useZonia({ channelID: "", initialThread: thread })
    return zoniaHookResult.match(({ thread, removePost, addPost, updatePost, loading, loadMore, remainingPostCount }) => {
        return <div>
            {
                loading ?
                    <div>Loading...</div> :
                    <div style={{ marginTop: 10 }}>
                        {renderThread(thread)}
                    </div>
            }
            {
                (remainingPostCount > 0 ||
                    remainingPostCount === -1)
                && !loading && <div style={{ marginTop: 10 }}>
                    <button onClick={loadMore}>Load more</button>
                </div>
            }
        </div>

    }, err => <div>{err} </div>)

}

function renderThread(thread: ThreadState): React.ReactNode {
    return Object.entries(thread).map(([commentId, comment]) => {
        return <div style={{ marginTop: 10 }}>
            <div>{comment.content}</div>
            <div>{comment.userId}</div>
            <div style={{ marginLeft: 20 }}>
                <RenderThread thread={comment.replies.thread} />
            </div>
        </div>
    })
}
