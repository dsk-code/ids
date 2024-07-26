import React, { useState } from 'react'
import DatePicker from "react-datepicker";

import "react-datepicker/dist/react-datepicker.css";
import SearchBox from '../components/SeachBox';

const Test1: React.FC = () => {
    const [startDate, setStartDate] = useState(new Date());

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
        </>
    )
};
export default Test1;