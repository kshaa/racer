import * as t from 'io-ts'

export const EnvConfig = t.type({
  httpServer: t.string,
  wsServer: t.string
})
export type EnvConfigT = t.TypeOf<typeof EnvConfig>

export const envConfig: EnvConfigT = {
  httpServer: process.env.zoopHttpServer as string,
  wsServer: process.env.zoopWebsocketServer as string,
}