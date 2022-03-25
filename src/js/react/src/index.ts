import { Result, ok } from 'neverthrow'
import _SDK, { SDK } from 'zomia'
import { ZoniaHookParam, useZonia, ZoniaHook } from './hook'

// export { useZonia } from './hook'
export type { ThreadState, ZoniaHook, ZoniaHookParam, PostStatus } from './hook'

interface Config {
    serverId: string
    host?: string
    sdk?: SDK
}

const initZoniaHookWithDefault = ({ serverId, sdk, host }: Config): Result<(params: ZoniaHookParam) => ZoniaHook, string> => {
    if (sdk) {
        return ok((params) => useZonia(sdk, params))
    } else {
        return _SDK({ serverId, host })
            .map(sdk => (params: ZoniaHookParam) => useZonia(sdk, params))
            .mapErr(err => err.message)
    }
}

export default initZoniaHookWithDefault