import Head from 'next/head'
import styles from '@/styles/Home.module.css'
import {useRouter} from 'next/router'
import {RoomJoin, validateRoomJoin} from "@/domain/roomJoin"
import * as O from 'fp-ts/Option'
import { useWhenInit } from "@seamusleahy/init-hooks";
import {joinRoom} from "@/services/game";

export default function Room() {
  const router = useRouter()
  const isMainPlayer = (router.query.roomId as string) === "true"
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
  let errors = O.getOrElse(() => new Map())(O.getLeft(validation))
  let roomJoin = O.getRight(validation)
  useWhenInit(() => {
    O.map<RoomJoin, void>((details) => {
      joinRoom(details)
    })(roomJoin)
  })

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
        <canvas id="game"></canvas>
      </main>
    </>
  )
}
