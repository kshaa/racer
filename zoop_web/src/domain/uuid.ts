import { tryCatch, Option } from "fp-ts/Option"
import * as O from "fp-ts/Option"
import { pipe } from "fp-ts/function"
import { parse as parseUuid } from "uuid"

export type Uuid = { value: Uint8Array }
export function uuidFromString(value: string): Option<Uuid> {
  return pipe(
    tryCatch(() => parseUuid(value)),
    O.map<Uint8Array, Uuid>(parsed => ({ value: parsed })),
  )
}