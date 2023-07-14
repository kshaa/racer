import Head from 'next/head'
import styles from '@/styles/Home.module.css'
import Button from '@mui/material/Button';
import TextField from "@mui/material/TextField";
import Stack from "@mui/material/Stack";
import {Typography} from "@mui/material";
import {useState} from "react";
import {pipe} from "fp-ts/function";
import * as E from "fp-ts/Either";
import {useRouter} from "next/router";
import {validateUsername} from "@/domain/auth";
import * as O from "fp-ts/Option";
import {addFormKeyError, flushErrors, keyErrorMessage, newFormErrors} from "@/domain/formError";
import {selectAuthState, setUser} from "@/redux/auth";
import {useDispatch, useSelector } from "react-redux";
import {postRegistration} from "@/services/auth";
import {getOrElse, isNone, isSome} from "fp-ts/Option";
import Alert from "@mui/material/Alert";
import {envConfig} from "@/services/config";

export default function Auth() {
  const router = useRouter()
  const authState = useSelector(selectAuthState);
  const dispatch = useDispatch();

  const [errors, setErrors] = useState(newFormErrors())

  const [username, setUsername] = useState("")
  const usernameErrors = keyErrorMessage(errors, O.some("username"))
  const onUserNameChange = (e: any) => {
    flushErrors(setErrors, O.some("username"))
    setUsername(e.target.value)
  }

  const formMetaErrors = keyErrorMessage(errors, O.none)

  const onSubmit = () => {
    pipe(
      validateUsername(username),
      E.match(
        // This would be cleaner (more functional and would not be as nested)
        // with TaskEither, Validation & effects at the end
        (error) => addFormKeyError(setErrors, O.some("username"), error),
        (register) =>
          postRegistration(envConfig.httpServer, register).then((result) =>
            pipe(
              result,
              E.match(
              (error) => addFormKeyError(setErrors, O.none, error),
              (user) => {
                  dispatch(setUser(O.some(user)))
                  router.push("/")
                }
              )
            )
          )
      )
    )
  }

  return (
    <>
      <Head>
        <title>Town Racer - Auth</title>
        <meta name="viewport" content="width=device-width, initial-scale=1" />
        <link rel="icon" href="/favicon.ico" />
      </Head>
      <main className={styles.main}>
        <Stack spacing={2} sx={{ width: "100%", maxWidth: "500px", marginTop: "5rem" }}>
          <Typography variant="h3">Register</Typography>
          <TextField
            error={isSome(usernameErrors)}
            helperText={getOrElse<string>(() => "")(usernameErrors)}
            value={username} onChange={onUserNameChange}
            required={true} id="username" label="Username" variant="outlined" />
          {isSome(formMetaErrors) &&
              <Alert severity="error">{getOrElse<string>(() => "")(formMetaErrors)}</Alert>
          }
          <Button onClick={onSubmit} variant="contained">Start</Button>
        </Stack>
      </main>
    </>
  )
}

