import { atom } from "recoil";
import { User, ResponseAuthUser } from "../types/userTypes";
import { Class, RequestPostClass } from "../types/classTypes";
import { Teacher } from "~/types/teachersTypes";

export const authUserState = atom<ResponseAuthUser>({
    key: "authUserState",
    default: {
        id: "",
        auth0UserName: "",
    },
})
export const classState = atom<Class | undefined>({
    key: "classState",
    default: undefined,
})

export const classListState = atom<Class[]>({
    key: "classListState",
    default: [],
})

export const teachersListState = atom<Teacher[]>({
    key: "teachersListState",
    default: [],
})


// export const userState = atom<User []>({
//     key: "userState",
//     default: [],
// });
// export const RequestPostClassState = atom<RequestPostClass>({
//     key: "RequestPostClassState",
//     default: {
//         className: "",
//         age: 0
//     },
// })

// export const accessTokenState = atom<string>({
//     key: 'accessTokenState',  // ユニークなID
//     default: "",  // 初期値
// });