import React, { useState } from 'react'
import DatePicker from "react-datepicker";

import "react-datepicker/dist/react-datepicker.css";
import SearchBox from '../components/SeachBox';
import { useAuth0 } from '@auth0/auth0-react';

const Test1: React.FC = () => {
    const [startDate, setStartDate] = useState(new Date());
    const { isAuthenticated, user } = useAuth0();

    if (!user) {
        return null;
    }

    if (isAuthenticated) {
        return(
            <>
                <div className='flex flex-col'>
                    <div>
                        <SearchBox />
                    </div>
                    
                    <DatePicker
                        selected={startDate}
                        onChange={(date) => date && setStartDate(date)}
                        showTimeSelect
                        timeFormat="HH:mm"
                        timeIntervals={15}
                        timeCaption="time"
                        dateFormat="yyyy-MM-dd HH:mm"
                    />
                </div>
                <h2>{user.name}</h2>
                <h2>{user.email}</h2>
            </>
        )
    }
};
export default Test1;