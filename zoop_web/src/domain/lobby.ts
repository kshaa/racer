import * as t from 'io-ts'

export const RoomId = t.string
export type RoomIdT = t.TypeOf<typeof RoomId>

export const RoomConfig = t.type({
  players: t.array(t.string)
})
export type RoomConfigT = t.TypeOf<typeof RoomConfig>