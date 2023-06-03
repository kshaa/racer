import * as t from "io-ts";

export enum AppErrorKind {
  RegistrationEmptyError,
  FetchError,
  ServerAppError
}

export interface AppError {
  kind: AppErrorKind
  message: String
}

export class FetchError implements AppError {
  kind = AppErrorKind.FetchError
  message: String
  error: any
  constructor(error: any) {
    this.error = error
    this.message = `Failed to contact server: ${error}`
  }
}

export class UsernameEmptyError implements AppError {
  kind = AppErrorKind.RegistrationEmptyError
  message = `Username can't be empty`
}

export const ServerError = t.type({
  error: t.any,
  message: t.string
})
export type ServerErrorT = t.TypeOf<typeof ServerError>

export class ServerAppError implements AppError {
  kind = AppErrorKind.ServerAppError
  error: ServerErrorT
  message: String
  constructor(error: ServerErrorT) {
    this.error = error
    this.message = `Problem: ${error.message}`
  }
}