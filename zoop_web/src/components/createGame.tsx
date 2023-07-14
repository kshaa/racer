import Button from '@mui/material/Button';
import Stack from "@mui/material/Stack";
import * as O from "fp-ts/Option"
import {
  FormControl,
  FormGroup,
  InputLabel, MenuItem,
  Select, SelectChangeEvent,
} from "@mui/material";
import {useState} from "react";
import {pipe} from "fp-ts/function";
import * as E from "fp-ts/Either";
import {getOrElse, isSome, Option} from "fp-ts/Option";
import {UserT} from "@/domain/auth";
import {addFormKeyError, flushErrors, keyErrorMessage, newFormErrors} from "@/domain/formError";
import {getRoomReady, postCreateRoom} from "@/services/lobby";
import {RoomConfigT, RoomIdT} from "@/domain/lobby";
import Alert from "@mui/material/Alert";
import {envConfig} from "@/services/config";
import {useRouter} from "next/router";
import {routerPushRoomConnect} from "@/services/game";

export interface CreateGameProps {
  user: UserT
}

export default function CreateGame(props: CreateGameProps) {
  const router = useRouter()

  const [errors, setErrors] = useState(newFormErrors())
  const formMetaErrors = keyErrorMessage(errors, O.none)

  const [roomId, setRoomId] = useState<Option<RoomIdT>>(O.none)

  const [playerCount, setPlayerCount] = useState(2)

  const [isRoomReady, setIsRoomReady] = useState(false)

  const onPlayerChange = (e: SelectChangeEvent<number>) => {
    flushErrors(setErrors, O.none)
    setPlayerCount(Number(e.target.value))
  }

  const onRoomReady = (user: UserT, roomId: RoomIdT, roomConfig: RoomConfigT) => {
    setIsRoomReady(true)
    routerPushRoomConnect(router, setErrors, user, roomId, roomConfig)
  }

  const onWaitRoomReady = (roomId: RoomIdT) => {
    getRoomReady(envConfig.httpServer, roomId, props.user.id, props.user.ticket).then((result) =>
      pipe(
        result,
        E.match(
          (error) => addFormKeyError(setErrors, O.none, error),
          (config) => onRoomReady(props.user, roomId, config)
        ),
      )
    )
  }

  const onSubmit = () => {
    postCreateRoom(envConfig.httpServer, props.user.id, props.user.ticket, playerCount).then((result) =>
      pipe(
        result,
        E.match(
          (error) => addFormKeyError(setErrors, O.none, error),
          (room) => {
            setRoomId(O.some(room))
            onWaitRoomReady(room)
          }
        )
      )
    )
  }

  const onCancel = () => {
    flushErrors(setErrors, O.none)
    setRoomId(O.none)
    setIsRoomReady(false)
  }

  return (
    <Stack spacing={2} sx={{ width: "100%", maxWidth: "500px", marginTop: "1rem" }}>
      <FormControl component="fieldset" variant="standard">
        <FormGroup sx={{ marginBottom: 2 }}>
          <FormControl fullWidth>
            <InputLabel id="player-count-select-label">Player count</InputLabel>
            <Select
              labelId="player-count-select-label"
              id="player-count-select"
              value={playerCount}
              label="Player count"
              onChange={onPlayerChange}
              disabled={isSome(roomId)}
            >
              {[...Array(20)].map((_, i) => i + 2).map(i =>
                <MenuItem key={i} value={i}>{i}</MenuItem>
              )}
            </Select>
          </FormControl>
        </FormGroup>
      </FormControl>
      {isSome(formMetaErrors) &&
          <Alert severity="error">{getOrElse<string>(() => "")(formMetaErrors)}</Alert>
      }
      {O.match(
        () => <Button variant="contained" onClick={onSubmit}>Create</Button>,
        (knownRoomId: string) => {
          return (
            !isRoomReady
              ? <Stack spacing={2} sx={{width: "100%", maxWidth: "500px", marginTop: "1rem"}}>
                <Alert severity="info">Share this room code with friends to join:<br></br>{knownRoomId}</Alert>
                <Button variant="contained" onClick={onCancel}>Waiting for players... (Cancel)</Button>
              </Stack>
              : <Stack spacing={2} sx={{width: "100%", maxWidth: "500px", marginTop: "1rem"}}>
                <Button variant="contained" onClick={onCancel}>Launching game... (Cancel)</Button>
              </Stack>
          )
        }
      )(roomId)}
    </Stack>
  )
}

