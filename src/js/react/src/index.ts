import { Result, ok } from 'neverthrow'
import _SDK, { SDK, SDKConfig, ZoniaResult } from '../../sdk/dist'

import { default as _initZoniaHook, ZoniaHook, ZoniaHookParam } from './hook'

const initZoniaHook = (sdk: SDK) => {
    return _initZoniaHook(sdk)
}

interface Config {
    serverId: string
    sdk?: SDK
}

const initZoniaHookWithDefault = ({ serverId, sdk }: Config): Result<(params: ZoniaHookParam) => ZoniaHook, string> => {
    if (sdk) {
        return ok(initZoniaHook(sdk))
    } else {
        return _SDK({ serverId })
            .map(sdk => initZoniaHook(sdk))
            .mapErr(err => err.message)
    }
}

export default initZoniaHookWithDefault