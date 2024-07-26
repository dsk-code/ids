import React, { useEffect } from 'react'
import { useSetRecoilState, useRecoilValue } from 'recoil';
import { userState } from '../recoil/atoms';
import { User } from '../types/userTypes';
import { useSearchParams } from "react-router-dom";
import { getUserByClass } from '../recoil/selectors';
import UsersListItem from '../components/UsersListItem';
import { fetchUsers } from '../api/dummy';

const UsersList: React.FC = () => {
    const setUsers = useSetRecoilState(userState);
    const [params] = useSearchParams();
    const className: string | null = params.get('name');
    const userslist: User[] = useRecoilValue(getUserByClass(params.get('name')))
    console.log(params);

    // バックエンドへのアクセス
    useEffect(() => {
        const fetchedUsers = () => {
        const fetch_users: User[] = fetchUsers;
        setUsers(fetch_users);
        }
        fetchedUsers()
    }, []);
    // console.log(users);

    return (
        <>
            <div className='container mx-auto max-w-screen-lg'>
                <h1 className='text-2xl font-extrabold mt-10 ml-10'>{className}</h1>
                <div className='flex justify-center items-center mt-10'>
                    <ul className='flex flex-row flex-wrap justify-center'>
                        {
                        userslist.map(user => {
                            return (
                                <li key={user.id}>
                                    <UsersListItem user={user}/>
                                </li>
                            )                    
                        })
                        }
                    </ul>
                </div>
            </div>
        </>
    );

};
export default UsersList;

// if (user.class) {
//     return <li key={user.id}><Link to={`/users/${user.id}`}>{user.name}{user.class}</Link></li>
// } else {
//     return <li key={user.id}><Link to={`/users/${user.id}`}>{user.name}</Link></li>
// }