import React from 'react'
import { User } from '../types/userTypes';
import { Link } from 'react-router-dom';

type UsersListItemProps = {
    user: User;
}
const UsersListItem: React.FC<UsersListItemProps> = ({user}) => {
    
    return(
        <>
            <Link to={`/user/users/${user.id}`} className="flex-shrink-0 group block">
                <div className="flex items-center border-2 rounded-md w-96 m-2 p-3 gap-3 border-gray-300 transition translate-y-4 hover:shadow-lg hover:bg-gray-50">
                    <img className="inline-block flex-shrink-0 size-12 rounded-full" src="https://images.unsplash.com/photo-1568602471122-7832951cc4c5?ixlib=rb-4.0.3&ixid=MnwxMjA3fDB8MHxwaG90by1wYWdlfHx8fGVufDB8fHx8&auto=format&fit=facearea&facepad=2&w=300&h=300&q=80" alt="Image Description"/>
                    <div className="ms-3 ml-8">
                    <ruby className="font-semibold text-gray-800">{user.name}<rt className="text-xs font-medium text-gray-400 dark:text-neutral-500">{user.furigana}</rt></ruby>
                    </div>
                </div>
            </Link>
        </>
    )
};
export default UsersListItem;