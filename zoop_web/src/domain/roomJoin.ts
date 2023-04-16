import {Uuid, uuidFromString} from "./uuid"
import {Either} from "fp-ts/Either";
import {pipe} from "fp-ts/function";
import * as E from "fp-ts/Either";

export type RoomJoin = {
  isMainPlayer: boolean,
  player0: Uuid,
  player1: Uuid,
  room: Uuid,
}

export function validateRoomJoin(
  isMainPlayer: boolean,
  player0: string,
  player1: string,
  roomId: string,
  player0Key: string,
  player1Key: string,
  roomKey: string,
): Either<Map<string, string>, RoomJoin> {
  const successes = new Map<string, any>()
  const failures = new Map<string, string>()

  pipe(uuidFromString(player0), E.match(
    (e: string) => failures.set(player0Key, e),
    (s: Uuid) => successes.set(player0Key, s)))

  pipe(uuidFromString(player1), E.match(
    (e: string) => failures.set(player1Key, e),
    (s: Uuid) => successes.set(player1Key, s)))

  pipe(uuidFromString(roomId), E.match(
    (e: string) => failures.set(roomKey, e),
    (s: Uuid) => successes.set(roomKey, s)))

  if (failures.size !== 0) return E.left(failures)
  else {
    const obj = Object.fromEntries(successes)
    const player0: Uuid = obj[player1Key]
    const player1: Uuid = obj[player1Key]
    const roomId: Uuid = obj[roomKey]
    const rooomJoin: RoomJoin = {
      room: roomId,
      player0,
      player1,
      isMainPlayer
    }
    return E.right(rooomJoin)
  }
}
