import { selectorFamily } from "recoil";
import { userState } from "./atoms";
import { User } from "../types/userTypes";

export const getUserById = selectorFamily({
    key: "getUserById",
    get: (userId) => ({ get }) => {
        const users = get(userState);
        const user =  users.find(user => user.id === userId);
        return user ? user : "user not found";
    },
});

// 引数のクラスの生徒の配列を返す
export const getUserByClass = selectorFamily({
    key: "getUserByClass",
    get: (class_name: string | null) => ({ get }) => {
        const users: User[] = get(userState);
        if (!class_name) {
            return users;
        } else if (class_name === "全園児") {
            return users;
        } else {
            const find_users: User[]= users.filter(
                user => user.class === class_name
            );
            return find_users;
        }
        
    },
});
