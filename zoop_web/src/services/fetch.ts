import {Errors, Validation} from "io-ts";
import * as E from "fp-ts/Either";
import {ParseError, ServerAppError, ServerError} from "@/domain/appError";
import {PathReporter} from "io-ts/PathReporter";
import {isRight} from "fp-ts/Either";
import * as O from "fp-ts/Option";

export function parsedOrFetchError<A>(parseKind: string, parsed: Validation<A>) {
  return E.mapLeft<Errors, ParseError>((e: Errors) => {
    const decodeReason = PathReporter.report(E.left(e)).join("\n")
    return new ParseError(parseKind, decodeReason)
  })(parsed)
}

export function parsedServerJson<A>(
  json: any,
  parseKind: string,
  parse: (json: any) => Validation<A>) {
  const decodedDomain = parse(json);
  const decodedServerError = ServerError.decode(json);

  const roomConfigResult = parsedOrFetchError(parseKind, decodedDomain)

  return isRight(decodedServerError)
    ? E.left(new ServerAppError(decodedServerError.right))
    : roomConfigResult
}

export function parsedServerError(json: any) {
  const decodedServerError = ServerError.decode(json);

  return isRight(decodedServerError)
    ? O.some(new ServerAppError(decodedServerError.right))
    : O.none
}
