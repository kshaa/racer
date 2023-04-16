import {invoke} from "@tauri-apps/api/tauri";
import {stringify as uuidStringify} from "uuid";
import init, {networked_game_raw} from "@/services/zoop_engine";
import {isTauriClient} from "@/services/tauri";
import {RoomJoin} from "@/domain/roomJoin";

export function joinRoom(
  roomDetails: RoomJoin
) {
  if (isTauriClient) {
    joinRoomNative(roomDetails)
  } else {
    joinRoomWasm(roomDetails)
  }
}

export function joinRoomWasm(roomDetails: RoomJoin) {
  fetch("/zoop_engine_bg.wasm")
    .then((response) => response.arrayBuffer())
    .then((bytes) => init(bytes))
    .then((_) => {
      networked_game_raw(
        roomDetails.isMainPlayer,
        uuidStringify(roomDetails.player0.value),
        uuidStringify(roomDetails.player1.value),
        uuidStringify(roomDetails.room.value)
      )
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