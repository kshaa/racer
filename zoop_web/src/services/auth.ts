import * as E from "fp-ts/Either";
import {Either} from "fp-ts/Either";
import {AppError, FetchError} from "@/domain/appError";
import {Register, User, UserT} from "@/domain/auth";
import {parsedServerJson} from "@/services/fetch";

export function postRegistration(zoopHttpServer: string, register: Register): Promise<Either<AppError, UserT>> {
  return fetch(`${zoopHttpServer}/api/user/register/${encodeURIComponent(register.username)}`, { method: "POST"})
    .then((response) => response.json())
    .then((json) =>  parsedServerJson(json, "UserT", User.decode))
    .catch((reason) => E.left(new FetchError(reason)))
}