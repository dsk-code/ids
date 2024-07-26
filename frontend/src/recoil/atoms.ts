import { atom } from "recoil";
import { User } from "../types/userTypes";
import { Class } from "../types/classTypes";

export const userState = atom<User []>({
    key: "userState",
    default: [],
});

export const classState = atom<Class []>({
    key: "classState",
    default: [],
})