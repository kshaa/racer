import * as t from "io-ts";

export enum AppErrorKind {
  RegistrationEmptyError,
  FetchError,
  ParseError,
  ServerAppError
}

export interface AppError {
  kind: AppErrorKind
  message: string
}

export class FetchError implements AppError {
  kind = AppErrorKind.FetchError
  message: string
  error: any
  constructor(error: any) {
    this.error = error
    this.message = `Failed to contact server: ${error}`
  }
}

export class ParseError implements AppError {
  kind = AppErrorKind.ParseError
  message: string
  parseKind: string
  reason: string
  constructor(parseKind: string, reason: string) {
    this.parseKind = parseKind
    this.reason = reason
    this.message = `Failed to decode '${parseKind}': ${reason}`
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
  message: string
  constructor(error: ServerErrorT) {
    this.error = error
    this.message = `Request refused: ${error.message}`
  }
}