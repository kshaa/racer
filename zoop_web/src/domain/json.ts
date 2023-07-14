import { tryCatch, Option } from "fp-ts/Option"
import { pipe } from "fp-ts/function"

export function jsonFromString(value: string): Option<any> {
  return pipe(
    tryCatch(() => JSON.parse(value)),
  )
}