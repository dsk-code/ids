import { Class } from "../types/classTypes";
import { User } from "../types/userTypes";

export const fetchUsers: User[] = [
    {
        id: "1",
        name: "北村 隆氏",
        furigana: "きたむら たかし",
        class: "たまご"
    },
    {
        id: "2",
        name: "北村 隆氏",
        furigana: "きたむら たかし",
        class: "ひよこ"
    }
]

export const fetchClass: Class[] = [
    {
        class_id: 1,
        class_name: "たまご",
        age: 0,
    },
    {
        class_id: 2,
        class_name: "ひよこ",
        age: 1,
    }
]