import Head from 'next/head'
import styles from '@/styles/Home.module.css'
import Button from '@mui/material/Button';
import TextField from "@mui/material/TextField";
import Stack from "@mui/material/Stack";
import * as O from "fp-ts/Option"
import Checkbox from "@mui/material/Checkbox";
import {
  FormControl,
  FormControlLabel,
  FormGroup,
  InputLabel,
  MenuItem,
  Select,
  Tab,
  Tabs,
  Typography
} from "@mui/material";
import {ReactNode, useEffect, useState} from "react";
import {pipe} from "fp-ts/function";
import {RoomConnect, validateRoomConnect} from "@/domain/roomConnect";
import * as E from "fp-ts/Either";
import {stringify as uuidStringify} from "uuid";
import {useRouter} from "next/router";
import Link from 'next/link';
import {useSelector} from "react-redux";
import {selectAuthState} from "@/redux/auth";
import {getOrElse, isNone, isSome} from "fp-ts/Option";
import {extractAuthUsername, UserIdT, UserT} from "@/domain/auth";
import {Box} from "@mui/system";
import {TabPanel} from "@/components/tabPanel";
import Alert from "@mui/material/Alert";
import {addFormKeyError, flushErrors, keyErrorMessage, newFormErrors} from "@/domain/formError";
import {getRoomReady, postCreateRoom, postJoinRoom} from "@/services/lobby";
import {AppError} from "@/domain/appError";
import {RoomConfigT, RoomIdT} from "@/domain/lobby";
import {envConfig} from "@/services/config";
import {set} from "fp-ts";
import {routerPushRoomConnect} from "@/services/game";

export interface JoinGameProps {
  user: UserT
}

export default function JoinGame(props: JoinGameProps) {
  const router = useRouter()

  const [roomId, setRoomId] = useState("")
  const onRoomIdChange = (e: any) => {
    flushErrors(setErrors, O.some("roomId"))
    flushErrors(setErrors, O.none)
    setRoomId(e.target.value)
  }

  const [isRoomReady, setIsRoomReady] = useState(false)
  const [isWaitingRoomReady, setIsWaitingRoomReady] = useState(false)

  const [errors, setErrors] = useState(newFormErrors())
  const roomIdErrors = keyErrorMessage(errors, O.some("roomId"))
  const formMetaErrors = keyErrorMessage(errors, O.none)


  const onRoomReady = (user: UserT, roomId: RoomIdT, roomConfig: RoomConfigT) => {
    setIsRoomReady(true)
    routerPushRoomConnect(router, setErrors, user, roomId, roomConfig)
  }

  const onWaitRoomReady = (user: UserT, roomId: RoomIdT) => {
    setIsWaitingRoomReady(true)
    getRoomReady(envConfig.httpServer, roomId, props.user.id, props.user.ticket).then((result) =>
      pipe(
        result,
        E.match(
          (error) => addFormKeyError(setErrors, O.none, error),
          (config) => onRoomReady(user, roomId, config)
        )
      )
    )
  }

  const onSubmit = (user: UserT, roomId: RoomIdT) => {
    postJoinRoom(envConfig.httpServer, roomId, props.user.id, props.user.ticket).then((result) =>
      pipe(
        result,
        O.match(
          () => onWaitRoomReady(user, roomId),
          (error) => addFormKeyError(setErrors, O.none, error)
        )
      )
    )
  }

  const onCancel = () => {
    setIsRoomReady(false)
    setIsWaitingRoomReady(false)
    flushErrors(setErrors, O.none)
  }

  return (
    <Stack spacing={2} sx={{ width: "100%", maxWidth: "500px", marginTop: "1rem" }}>
      <TextField
        error={isSome(roomIdErrors)}
        helperText={getOrElse<string>(() => "")(roomIdErrors)}
        value={roomId} onChange={onRoomIdChange} disabled={isWaitingRoomReady || isRoomReady}
        required={true} id="roomId" label="Room code" variant="outlined" />
      {isSome(formMetaErrors) &&
          <Alert severity="error">{getOrElse<string>(() => "")(formMetaErrors)}</Alert>
      }
      {isRoomReady
        ? <Button variant="contained" onClick={onCancel}>Launching game... (Cancel)</Button>
        : isWaitingRoomReady
          ? <Button variant="contained" onClick={onCancel}>Waiting for players... (Cancel)</Button>
          : <Button variant="contained" onClick={() => onSubmit(props.user, roomId)}>Join</Button>
      }
    </Stack>
  )
}

