import * as E from "fp-ts/Either";
import * as O from "fp-ts/Option";
import {Either} from "fp-ts/Either";
import {AppError, FetchError} from "@/domain/appError";
import {RoomConfig, RoomConfigT, RoomId, RoomIdT} from "@/domain/lobby";
import {Option} from "fp-ts/Option";
import {parsedServerError, parsedServerJson} from "@/services/fetch";

export function postCreateRoom(zoopHttpServer: string, playerId: string, ticket: string, playerCount: number): Promise<Either<AppError, RoomIdT>> {
  return fetch(`${zoopHttpServer}/api/game/new/by/${playerId}/ticket/${encodeURIComponent(ticket)}/player_count/${playerCount}`, { method: "POST"})
    .then((response) => response.json())
    .then((json) =>  parsedServerJson(json, "RoomIdT", RoomId.decode))
    .catch((reason) => E.left(new FetchError(reason)))
}

export function postJoinRoom(zoopHttpServer: string, roomId: string, playerId: string, ticket: string): Promise<Option<AppError>> {
  return fetch(`${zoopHttpServer}/api/game/join/${roomId}/by/${playerId}/ticket/${encodeURIComponent(ticket)}`, { method: "POST"})
    .then((response) => response.json())
    .then(parsedServerError)
    .catch((reason) => O.some(new FetchError(reason)))
}

export function getRoomReady(zoopHttpServer: string, roomId: string, playerId: string, ticket: string): Promise<Either<AppError, RoomConfigT>> {
  return fetch(`${zoopHttpServer}/api/game/ready/${roomId}/for/${playerId}/ticket/${encodeURIComponent(ticket)}`, { method: "GET"})
    .then((response) => response.json())
    .then((json) =>  parsedServerJson(json, "RoomConfigT", RoomConfig.decode))
    .catch((reason) => E.left(new FetchError(reason)))
}
