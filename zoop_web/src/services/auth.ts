import * as E from "fp-ts/Either";
import {Either, isLeft, isRight} from "fp-ts/Either";
import {AppError, FetchError, ServerAppError, ServerError} from "@/domain/appError";
import {Register, User, UserT} from "@/domain/auth";
import {PathReporter} from "io-ts/PathReporter";
import {Errors} from "io-ts";

export function postRegistration(zoopHttpServer: string, register: Register): Promise<Either<AppError, UserT>> {
  return fetch(`${zoopHttpServer}/user/register/${encodeURIComponent(register.username)}`, { method: "POST"})
    .then((response) => response.json())
    .then((json) => {
      const decodedUser = User.decode(json);
      const decodedServerError = ServerError.decode(json);

      const userResult = E.mapLeft<Errors, FetchError>((e: Errors) => {
        const decodeReason = PathReporter.report(E.left(e)).join("\n")
        return new FetchError(`Failed to decode response: ${decodeReason}`)
      })(decodedUser)

      return isRight(decodedServerError)
          ? E.left(new ServerAppError(decodedServerError.right))
          : userResult
    })
    .catch((reason) => E.left(new FetchError(reason)))
}