import {invoke} from "@tauri-apps/api/tauri";
import {stringify as uuidStringify} from "uuid";
import init, {networked_game_raw} from "@/services/zoop_engine";
import {isTauriClient} from "@/services/tauri";
import {RoomConnect, validateRoomConnect} from "@/domain/roomConnect";
import {UserT} from "@/domain/auth";
import {RoomConfigT, RoomIdT} from "@/domain/lobby";
import {pipe} from "fp-ts/function";
import {envConfig} from "@/services/config";
import * as E from "fp-ts/Either";
import {addFormKeyError, FormErrors} from "@/domain/formError";
import * as O from "fp-ts/Option";
import {AppError} from "@/domain/appError";
import {NextRouter} from "next/router";

export function routerPushRoomConnect(
  router: NextRouter,
  setErrors: (transform: ((prevState: FormErrors) => FormErrors)) => void,
  user: UserT,
  roomId: RoomIdT,
  roomConfig: RoomConfigT
) {
  pipe(
    validateRoomConnect(
      envConfig.httpServer,
      "httpBaseUrl",
      envConfig.wsServer,
      "wsBaseurlKey",
      user.id,
      "userId",
      user.ticket,
      "userTicket",
      roomId,
      "roomId",
      JSON.stringify(roomConfig),
      "roomConfig"
    ),
    E.match(
      (errors) => {
        for (const [key, value] of Object.entries(errors)) {
          addFormKeyError(setErrors, O.some(key), value as AppError)
        }
      },
      ({ httpBaseurl,  wsBaseurl,  userId,  userTicket,  roomId, roomConfig }) =>
        router.push({
          pathname: '/room/[roomId]',
          query: {
            httpBaseurl: httpBaseurl.toString(),
            wsBaseurl: wsBaseurl.toString(),
            userId: uuidStringify(userId.value),
            userTicket: userTicket,
            roomId: uuidStringify(roomId.value),
            roomConfigJson: JSON.stringify(roomConfig)
          }
        })
    )
  )
}

export function connectRoom(
  roomDetails: RoomConnect,
  canvasSelector: string
) {
  if (isTauriClient) {
    connectRoomNative(roomDetails)
  } else {
    connectRoomWasm(roomDetails, canvasSelector)
  }
}

export function connectRoomWasm(roomDetails: RoomConnect, canvasSelector: string) {
  fetch("/zoop_engine_bg.wasm")
    .then((response) => response.arrayBuffer())
    .then((bytes) => init(bytes).catch((error: any) => {
      if (!error.message.startsWith("Using exceptions for control flow,")) {
        throw error;
      }
    }))
    .then((_) => {
      try {
        networked_game_raw(
          roomDetails.httpBaseurl.toString(),
          roomDetails.wsBaseurl.toString(),
          uuidStringify(roomDetails.userId.value),
          roomDetails.userTicket,
          uuidStringify(roomDetails.roomId.value),
          JSON.stringify(roomDetails.roomConfig),
          canvasSelector,
        )
      } catch (error: any) {
        if (!error.message.startsWith("Using exceptions for control flow,")) {
          throw error;
        }
      }
    })
}

export function connectRoomNative(roomDetails: RoomConnect) {
  invoke(
    'connect_game',
    {
      httpBaseurl: roomDetails.httpBaseurl.toString(),
      wsBaseurl: roomDetails.wsBaseurl.toString(),
      userUuid: uuidStringify(roomDetails.userId.value),
      userTicket: roomDetails.userTicket,
      roomUuid: uuidStringify(roomDetails.roomId.value),
      roomConfigJson: JSON.stringify(roomDetails)
    }
  ).then(console.log).catch(console.error)
}