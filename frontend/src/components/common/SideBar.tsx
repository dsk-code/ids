import React, { useEffect, useState } from 'react'
import {  
    BsSearch,  
    BsPersonLinesFill, 
    BsChevronDown 
} from "react-icons/bs";
import { LuPanelLeftOpen } from "react-icons/lu";
import { useRecoilState } from 'recoil';
import { classState } from '../../recoil/atoms';
import { NavLink } from 'react-router-dom';
import { fetchClass } from '../../api/dummy';

interface Menu {
    id: number;
    title: string;
    spacing: boolean;
    submenu: boolean;
    submenuItems?: SubmenuItem[];
    link: string;
}

interface SubmenuItem {
    id: number;
    title?: string;
}

const SideBar: React.FC = () => {
    const [open, setOpen] = useState(false);
    const [submenuOpen, setSubmenuOpen] = useState(false);
    const [classList, setClassList] =  useRecoilState(classState);

    useEffect(() => {
        const fetchedClassList = async () => {
            try {
                const fetch_class = await fetchClass;
                setClassList(fetch_class);
            } catch (error) {
                console.error('Failed to fetch class:', error)
            }
        }
        fetchedClassList();
    }, []);
    
    const Menus: Menu[] = [];
    const menu1 = {
        id: 0,
        title: "Home",
        spacing: false,
        submenu: false,
        link: "/"
    }
    const menu2_submenuItems: SubmenuItem[] = [{id: 0, title: "全園児"}];
    classList.forEach(cls => {
        if (cls.class_name) {
            const sumenuItem: SubmenuItem = {
                id: cls.class_id,
                title: cls.class_name
            };
            menu2_submenuItems.push(sumenuItem);
        }
    })
    const menu2 = {
        id: 1,
        title: "園児",
        spacing: true,
        submenu: menu2_submenuItems.length === 0 ? false : true,
        submenuItems: menu2_submenuItems,
        link: "/user/users?name=",
    }
    const menu3 = {
        id: 2,
        title: "教諭",
        spacing: false,
        submenu: false,
        link: "/user/test1"
    }
    Menus.push(menu1, menu2, menu3);
    
    // submenuがあるときにNavLinkを使うと親メニューを押すと全リストが出てしまう
    const ConditionalLink = (menu: Menu) => {
        if (!menu.submenu) {
            return (
                <li key={menu.id}>
                    <NavLink to={`${menu.link}`}  className={`text-gray-300 text-sm flex
                    items-center gap-x-4 cursor-pointer p-2 hover:bg-light-white
                    rounded-md select-none ${menu.spacing ? "mt-9" : "mt-2"}`}
                    onClick={() => menu.submenu && setSubmenuOpen(!submenuOpen)}
                    >
                        <span className='text-2xl block float-left'>
                            <BsPersonLinesFill />
                        </span>
                        <span className={`text-base font-medium flex-1
                        ${!open && "hidden"}`
                        }>
                            {menu.title}
                        </span>
                        {
                            menu.submenu && open && (
                                <BsChevronDown className={`${submenuOpen && "rotate-180"}`}
                                />
                            )
                        }
                    </NavLink>
                </li>
            )
        } else {
            return (
                <li key={menu.id} className={`text-gray-300 text-sm flex
                items-center gap-x-4 cursor-pointer p-2 hover:bg-light-white
                rounded-md select-none ${menu.spacing ? "mt-9" : "mt-2"}`}
                onClick={() => menu.submenu && setSubmenuOpen(!submenuOpen)}
                >
                    <span className='text-2xl block float-left'>
                        <BsPersonLinesFill />
                    </span>
                    <span className={`text-base font-medium flex-1
                    ${!open && "hidden"}`
                    }>
                        {menu.title}
                    </span>
                    {
                        menu.submenu && open && (
                            <BsChevronDown className={`${submenuOpen && "rotate-180"}`}
                            />
                        )
                    }
                </li>
            )
            
        }
    }

    const openClick = () => {
        setOpen(!open);
        setSubmenuOpen(!submenuOpen);
    };

    return(
        <>
            <div className={`bg-gray-800 h-screen p-5 pt-8 m-0
                ${open ? "w-64" : "w-20"} duration-100 relative flex-none
                `}
            >
                <div className='inline-flex'>
                    <LuPanelLeftOpen className={`bg-amber-300 text-4xl
                        rounded cursor-pointer block float-left mr-2 duration-500 
                        ${open && "rotate-180"}`} onClick={openClick}/>
                    <h1 className={`text-white origin-left font-medium
                        text-2xl duration-200 select-none ${!open && "scale-0"}`}>
                        KidsMng
                    </h1>
                </div>

                <div className={`flex items-center rounded-md bg-light-white 
                mt-6 ${!open ? "px-2.5 py-2" : "px-4"} py-2`}>
                    <BsSearch className={`text-white text-lg block
                    float-left cursor-pointer m-0 p-0 ${open && "mr-3 text-lg"}`}/>

                    <input type={"search"} name="search" placeholder="Search" 
                    className={`text-base bg-transparent w-full text-white
                    focus:outline-0 foucus:outline-none border-0 focus:border-0 py-0 my-0 select-none ${!open && "hidden"}`} />
                </div>

                <ul className='pt-2'>
                    {
                        Menus.map((menu) => (
                            <>
                                {ConditionalLink(menu)}

                                {menu.submenu && submenuOpen && open && menu.submenuItems &&(
                                    <ul className='duration-100'>
                                        {
                                            menu.submenuItems.map((submenuItem) => (
                                                <li key={submenuItem.id}>
                                                    <NavLink to={`${menu.link}${submenuItem.title}`} className='text-gray-300 text-sm flex
                                                    items-center gap-x-4 cursor-pointer p-2 px-5 hover:bg-light-white
                                                    rounded-md select-none'>
                                                        {submenuItem.title}
                                                    </NavLink>
                                                </li>
                                            ))
                                        }
                                    </ul>
                                )}
                            </>
                        ))
                    }
                </ul>


            </div>
        </>
    )
};
export default SideBar;