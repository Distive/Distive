import { Page, SDK as ISDK, SDKConfig, Thread, default as _SDK } from '../../sdk'
import { err, Result, ok } from 'neverthrow'
import { useEffect, useState } from 'react'

interface ZoniaHook {
    loading: boolean
    thread: Thread
    error: string
    upsertPost: ISDK['upsertPost']
    removePost: ISDK['removePost']
    loadMore: () => void
}

type ErrorMessage = String

export const useZonia = (args: SDKConfig): Result<ZoniaHook, ErrorMessage> => {


    const SDKResult = _SDK(args)
    if (SDKResult.isErr()) {
        return err("")
    }
    const SDK = SDKResult._unsafeUnwrap()

    const [loading, setLoading] = useState(false)
    const [thread, setThread] = useState<Thread>([])
    const [error, setError] = useState("")

    useEffect(() => {
        // setLoading(true)
        // _SDK(args)
        //     .map(sdk => {
        //         setLoading(false)
        //         setSDK(sdk)
        //     })
        //     .mapErr(err => {
        //         setLoading(false)
        //         setError(err.message)
        //     })

    }, [])

    const loadMore = async () => {
        setError("")
        await SDK.getThread({
            channelId: args.serverId,
            limit: 10,
            cursor: thread[thread.length - 1]?.id ?? ''
        }).map(page => {
            setThread([...thread, ...page.thread])
        }).mapErr(err => {
            setError(err.message)
        })
    }

    const upsertPost: ZoniaHook['upsertPost'] = async (input) => {

    }

    const removePost: ZoniaHook['removePost'] = async (input) => {

    }

    return ok({
        error,
        loadMore,
        loading,
        removePost,
        thread,
        upsertPost
    })

}




export default useZonia