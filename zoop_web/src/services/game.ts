import {invoke} from "@tauri-apps/api/tauri";
import {stringify as uuidStringify} from "uuid";
import init, {networked_game_raw} from "@/services/zoop_engine";
import {isTauriClient} from "@/services/tauri";
import {RoomJoin} from "@/domain/roomJoin";

export function joinRoom(
  roomDetails: RoomJoin,
  canvasSelector: string
) {
  if (isTauriClient) {
    joinRoomNative(roomDetails)
  } else {
    joinRoomWasm(roomDetails, canvasSelector)
  }
}

export function joinRoomWasm(roomDetails: RoomJoin, canvasSelector: string) {
  fetch("/zoop_engine_bg.wasm")
    .then((response) => response.arrayBuffer())
    .then((bytes) => init(bytes).catch((error) => {
      if (!error.message.startsWith("Using exceptions for control flow,")) {
        throw error;
      }
    }))
    .then((_) => {
      try {
        console.log("Is main", roomDetails.isMainPlayer)
        networked_game_raw(
          roomDetails.isMainPlayer,
          uuidStringify(roomDetails.player0.value),
          uuidStringify(roomDetails.player1.value),
          uuidStringify(roomDetails.room.value),
          canvasSelector,
        )
      } catch (error) {
        if (!error.message.startsWith("Using exceptions for control flow,")) {
          throw error;
        }
      }
    })
}

export function joinRoomNative(roomDetails: RoomJoin) {
  invoke(
    'join_game',
    {
      isMainPlayer: roomDetails.isMainPlayer,
      player0Uuid: uuidStringify(roomDetails.player0.value),
      player1Uuid: uuidStringify(roomDetails.player1.value),
      roomUuid: uuidStringify(roomDetails.room.value)
    }
  ).then(console.log).catch(console.error)
}