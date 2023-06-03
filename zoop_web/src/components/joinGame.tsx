import Head from 'next/head'
import styles from '@/styles/Home.module.css'
import Button from '@mui/material/Button';
import TextField from "@mui/material/TextField";
import Stack from "@mui/material/Stack";
import * as O from "fp-ts/Option"
import Checkbox from "@mui/material/Checkbox";
import {FormControlLabel, Tab, Tabs, Typography} from "@mui/material";
import {ReactNode, useEffect, useState} from "react";
import {pipe} from "fp-ts/function";
import {RoomJoin, validateRoomJoin} from "@/domain/roomJoin";
import * as E from "fp-ts/Either";
import {stringify as uuidStringify} from "uuid";
import {useRouter} from "next/router";
import Link from 'next/link';
import {useSelector} from "react-redux";
import {selectAuthState} from "@/redux/auth";
import {isNone} from "fp-ts/Option";
import {extractAuthUsername} from "@/domain/auth";
import {Box} from "@mui/system";
import {TabPanel} from "@/components/tabPanel";

export interface JoinGameProps {
}

export default function JoinGame(props: JoinGameProps) {
  const router = useRouter()

  // const onSubmit = () => {
  //   pipe(
  //     validateRoomJoin(
  //       isMainPlayer,
  //       player0,
  //       player1,
  //       roomId,
  //       "player0",
  //       "player1",
  //       "roomId",
  //     ),
  //     E.match<Map<string, string>, RoomJoin>(
  //       (errors) => setErrors(errors),
  //       ({ room, isMainPlayer, player0, player1 }) =>
  //         router.push({
  //           pathname: '/room/[roomId]',
  //           query: {
  //             isMainPlayer,
  //             player0: uuidStringify(player0.value),
  //             player1: uuidStringify(player1.value),
  //             roomId: uuidStringify(room.value),
  //           }
  //         })
  //     )
  //   )
  // }

  return (
    <Stack spacing={2} sx={{ width: "100%", maxWidth: "500px", marginTop: "1rem" }}>
      <Button variant="contained">Join</Button>
    </Stack>
  )
}

