

export type ApiPath =
    | `${string}/me`
    | `${string}/classes`
    | `${string}/classes/${string}`
    | `${string}/teachers`
    | `${string}/teachers?offset=${string}&limit=${string}&status=${string}`
    | `${string}/teachers/${string}`


