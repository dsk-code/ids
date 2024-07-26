import React from 'react'
import InfoList from '../components/InfoList';
import { format } from 'date-fns';

export interface Information {
    id: number;
    name: string;
    summary: string;
    manager: string;
    memo?: string;
    date_time: Date; // Date型にする予定
}

const Root: React.FC = () => {

    const infos: Information[] = [
        {
            id: 1,
            name: "川原 陸",
            summary: "体調不良の為",
            manager: "北野 良子",
            memo: "頭痛",
            date_time: new Date(2024, 7, 2, 15, 19),
        },
        {
            id: 2,
            name: "相良 喜朗",
            summary: "体調不良の為",
            manager: "倉田 涼子",
            memo: "頭痛",
            date_time: new Date(2024, 7, 2, 15, 30),
        },
    ]
    
    const now: Date = new Date();
    const formattedMonth: string = format(now, "yyyy年MM月dd日");


    return(
        <>
            <div className='container mx-auto max-w-screen-lg'>
                <div className='flex flex-col justify-start items-center mt-3 ml-3'>
                    <h1 className='font-extrabold text-5xl text-gray-500'>インフォメーション</h1>
                    <h2 className='text-lg text-gray-400'>今日</h2>
                    <div className='pt-5'>
                        <p className='text-2xl'>{formattedMonth}</p>
                        <InfoList infos={infos}/>
                    </div>
                </div>
            </div>
        </>
    )
};
export default Root;