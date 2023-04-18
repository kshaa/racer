import Head from 'next/head'
import styles from '@/styles/Home.module.css'
import Alert from "@mui/material/Alert";
import {useRouter} from 'next/router'
import {RoomJoin, validateRoomJoin} from "@/domain/roomJoin"
import * as O from 'fp-ts/Option'
import {joinRoom} from "@/services/game";
import {useEffect} from "react";
import {pipe} from "fp-ts/function";
import {isTauriClient} from "@/services/tauri";
import Link from "next/link";

export default function Room() {
  const router = useRouter()
  const isRouterReady = Object.keys(router.query).length > 0
  const isMainPlayer = (router.query.isMainPlayer as string) === "true"
  const player0 = router.query.player0 as string
  const player1 = router.query.player1 as string
  const roomId = router.query.roomId as string
  const validation = validateRoomJoin(
    isMainPlayer,
    player0,
    player1,
    roomId,
    "player0",
    "player1",
    "roomId"
  )
  let errors =
    pipe(
      O.getLeft(validation),
      O.filter((_: Map<string, string>) => isRouterReady),
      O.getOrElse(() => new Map())
    )

  let roomJoin = O.getRight(validation)
  useEffect(() => {
    O.map<RoomJoin, void>((details) => {
      console.log("Joining room", details, performance.timing.navigationStart)
      joinRoom(details, "#game")
    })(roomJoin)
  }, [roomId]);

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
