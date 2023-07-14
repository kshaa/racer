import {Option} from "fp-ts/Option";
import {AppError} from "@/domain/appError";
import * as O from "fp-ts/Option";

export interface FormErrors {
   values: Map<Option<string>, AppError[]>
}

export function cloneErrors(errors: FormErrors): FormErrors {
  const valueCopy = JSON.parse(JSON.stringify(Array.from(errors.values)))
  return {
    values: new Map(valueCopy)
  }
}

export function newFormErrors(): FormErrors {
  return {
    values: new Map<Option<string>, AppError[]>()
  }
}

export function flushErrors(
  setter: (transform: ((prevState: FormErrors) => FormErrors)) => void,
  key: Option<string>
): void {
  setter((prevState: FormErrors) => {
    const copy = cloneErrors(prevState)
    copy.values.set(key, [])
    return copy
  })
}

export function addFormKeyError(
  setter: (transform: ((prevState: FormErrors) => FormErrors)) => void,
  key: Option<string>,
  newError: AppError
): void {
  addFormKeyErrors(setter, key, [newError])
}

export function addFormKeyErrors(
  setter: (transform: ((prevState: FormErrors) => FormErrors)) => void,
  key: Option<string>,
  newErrors: [AppError]
): void {
  setter((prevState: FormErrors) => {
    const copy = cloneErrors(prevState)
    const oldErrors = copy.values.get(key) || []
    copy.values.set(key, oldErrors.concat(newErrors))
    return copy
  })
}

export function keyErrorMessage(errors: FormErrors, key: Option<string>): Option<string> {
  const keyErrors = errors.values.get(key) || []
  const keyErrorMessages = keyErrors.map(e => e.message)

  if (keyErrorMessages.length > 0) return O.some(keyErrorMessages.join(". "))
  else return O.none
}
