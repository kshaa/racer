import {Uuid, uuidFromString} from "./uuid"
import {Either} from "fp-ts/Either";
import {pipe} from "fp-ts/function";
import * as E from "fp-ts/Either";
import * as O from "fp-ts/Option";
import {urlFromString} from "@/domain/url";
import {AppError, ParseError} from "@/domain/appError";
import {RoomConfig, RoomConfigT} from "@/domain/lobby";
import * as t from "io-ts";
import {jsonFromString} from "@/domain/json";

export type RoomConnect = {
  httpBaseurl: URL,
  wsBaseurl: URL,
  userId: Uuid,
  userTicket: string,
  roomId: Uuid,
  roomConfig: RoomConfigT
}

export function validateRoomConnect(
  httpBaseurl: string,
  httpBaseurlKey: string,
  wsBaseurl: string,
  wsBaseurlKey: string,
  userId: string,
  userIdKey: string,
  userTicket: string,
  userTicketKey: string,
  roomId: string,
  roomIdKey: string,
  roomConfigJson: string,
  roomConfigJsonKey: string,
): Either<Map<string, AppError>, RoomConnect> {
  const successes = new Map<string, any>()
  const failures = new Map<string, AppError>()

  pipe(urlFromString(httpBaseurl), O.match(
    () => failures.set(httpBaseurlKey, new ParseError(httpBaseurlKey, "Not a valid URL")),
    (s: URL) => successes.set(httpBaseurlKey, s)
  ))

  pipe(urlFromString(wsBaseurl), O.match(
    () => failures.set(wsBaseurlKey, new ParseError(wsBaseurlKey, "Not a valid URL")),
    (s: URL) => successes.set(wsBaseurlKey, s)
  ))

  pipe(uuidFromString(userId), O.match(
    () => failures.set(userIdKey, new ParseError(userIdKey, "Not a valid UUID")),
    (s: Uuid) => successes.set(userIdKey, s)))

  pipe(uuidFromString(roomId), O.match(
    () => failures.set(roomIdKey, new ParseError(roomIdKey, "Not a valid UUID")),
    (s: Uuid) => successes.set(roomIdKey, s)))

  pipe(
    jsonFromString(roomConfigJson),
    O.map((s: any) => O.fromEither(RoomConfig.decode(s))),
    O.flatten,
    O.match(
    () => failures.set(roomConfigJsonKey, new ParseError(roomConfigJsonKey, "Not a valid RoomConfig")),
    (s: any) => successes.set(roomConfigJsonKey, s)
  ))

  if (failures.size !== 0) return E.left(failures)
  else {
    const obj = Object.fromEntries(successes)
    const _httpBaseurl: URL = obj[httpBaseurlKey]
    const _wsBaseurl: URL = obj[wsBaseurlKey]
    const _userId: Uuid = obj[userIdKey]
    const _roomId: Uuid = obj[roomIdKey]
    const _roomConfig: RoomConfigT = obj[roomConfigJsonKey]

    const rooomJoin: RoomConnect = {
      httpBaseurl: _httpBaseurl,
      wsBaseurl: _wsBaseurl,
      userId: _userId,
      userTicket: userTicket,
      roomId: _roomId,
      roomConfig: _roomConfig
    }
    return E.right(rooomJoin)
  }
}
