import { NavLink } from "react-router-dom";
import BackButtonArrow from "./BackButtonArrow";
import ForwardButtonArrow from "./ForwardButtonArrow";
// import { LogoutButton } from "../auth/ LogoutButton";
import Auth0NavButtons from "../auth/Auth0NavButtons";

export default function Header() {
    return (
      <>
        <header className="flex flex-wrap sm:justify-start sm:flex-nowrap w-full bg-gray-100 text-sm py-4 backdrop-opacity-10">
        <nav className="max-w-[85rem] w-full mx-auto px-4 sm:flex sm:items-center sm:justify-between" aria-label="Global">
            <div className="flex flex-row items-center gap-0">
              <BackButtonArrow />
              <ForwardButtonArrow />
            </div>
            <div className="flex flex-row items-center gap-5 mt-5 sm:justify-end sm:mt-0 sm:ps-5">
              <NavLink to="/user/test1" className="font-medium text-blue-500 select-none">Test1</NavLink>
              <a className="font-medium text-gray-600 hover:text-gray-400 dark:text-neutral-400 dark:hover:text-neutral-500 select-none" href="#">Account</a>
              <a className="font-medium text-gray-600 hover:text-gray-400 dark:text-neutral-400 dark:hover:text-neutral-500 select-none" href="#">Work</a>
              <Auth0NavButtons />
            </div>
        </nav>
        </header>
      </>
    );
  }

