import React from 'react'
import { useParams } from 'react-router-dom';
import { useRecoilValue } from 'recoil';
import { getUserById } from '../recoil/selectors';


const User: React.FC = () => {
    const params = useParams();
    const user = useRecoilValue(getUserById(params.id));

    if (typeof user === 'string') {
        return <div>{user}</div>;
    }

    return(
        <>
            <div className='container mx-auto max-w-screen-lg'>
                <div className='flex items-center justify-center'>
                    <p>{params.id}</p>
                    <p>こんにちは{user.name}さん</p>
                </div>
            </div>
        </>
    )
};
export default User;