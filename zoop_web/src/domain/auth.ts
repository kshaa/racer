import {Either} from "fp-ts/Either";
import * as E from "fp-ts/Either";
import {Option} from "fp-ts/Option";
import * as O from "fp-ts/Option";
import {AppError, UsernameEmptyError} from "@/domain/appError";
import * as t from 'io-ts'
import {pipe} from "fp-ts/function";

// Auth state
export const User = t.type({
  id: t.string,
  username: t.string,
  ticket: t.string
})
export type UserT = t.TypeOf<typeof User>

export interface AuthState {
  user: Option<UserT>
}

export const empty: AuthState = {
  user: O.none
}

export function extractAuthUsername(authState: AuthState): string {
  return pipe(
    authState.user,
    O.map<UserT, string>((u) => u.username),
    O.getOrElse<string>(() => "Unknown")
  )
}

// Auth actions
export interface Register {
  username: string
}

export function validateUsername(username: String): Either<AppError, Register> {
  let trimmed = username.trim()

  if (trimmed === "") return E.left(new UsernameEmptyError())
  else return E.right({ username: trimmed } as Register)
}
