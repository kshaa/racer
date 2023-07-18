import Head from 'next/head'
import styles from '@/styles/Home.module.css'
import Alert from "@mui/material/Alert";
import {useRouter} from 'next/router'
import {RoomConnect, validateRoomConnect} from "@/domain/roomConnect"
import * as O from 'fp-ts/Option'
import {connectRoom} from "@/services/game";
import {useEffect} from "react";
import {pipe} from "fp-ts/function";
import {isTauriClient} from "@/services/tauri";
import Link from "next/link";
import {AppError} from "@/domain/appError";

export default function Room() {
  const router = useRouter()
  const isRouterReady = Object.keys(router.query).length > 0
  const httpBaseurl = router.query.httpBaseurl as string
  const wsBaseurl = router.query.wsBaseurl as string
  const userId = router.query.userId as string
  const userTicket = router.query.userTicket as string
  const roomId = router.query.roomId as string
  const roomConfigJson = router.query.roomConfigJson as string

  const validation = validateRoomConnect(
    httpBaseurl,
    "httpBaseUrl",
    wsBaseurl,
    "wsBaseUrl",
    userId,
    "userId",
    userTicket,
    "userTicket",
    roomId,
    "roomId",
    roomConfigJson,
    "roomConfigJson"
  )
  let errors =
    pipe(
      O.getLeft(validation),
      O.filter((_: Map<string, AppError>) => isRouterReady),
      O.map((validation) => {
        const next = new Map<string, string>();
        for (let [key, error] of Object.entries(validation)) {
          next.set(key, error.message)
        }
        return next
      }),
      O.getOrElse(() => new Map())
    )

  let roomJoin = O.getRight(validation)
  useEffect(() => {
    O.map<RoomConnect, void>((details) => {
      console.log("Connecting to room", details, performance.timing.navigationStart)
      connectRoom(details, "#game")
    })(roomJoin)
  }, [roomId, roomJoin]);

  return (
    <>
      <Head>
        <title>Town Racer</title>
        <meta name="description" content="Top-down view racing game" />
        <meta name="viewport" content="width=device-width, initial-scale=1" />
        <link rel="icon" href="/favicon.ico" />
      </Head>
      <main className={styles.main}>
        {Array.from(errors.entries()).map(([key, error]) => {
          return (<Alert key={key} severity="error">{error}</Alert>)
        })}
        {!isTauriClient &&
            <canvas id="game"></canvas>
        }
        {isTauriClient &&
            <Link href="/">Back to lobby</Link>
        }
      </main>
    </>
  )
}
