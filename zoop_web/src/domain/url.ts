import { tryCatch, Option } from "fp-ts/Option"
import * as O from "fp-ts/Option"
import { pipe } from "fp-ts/function"
import { parse as parseUuid } from "uuid"
import {Either} from "fp-ts/Either";
import * as E from "fp-ts/Either";
import {AppError} from "@/domain/appError";

export function urlFromString(value: string): Option<URL> {
  return tryCatch(() => new URL(value))
}