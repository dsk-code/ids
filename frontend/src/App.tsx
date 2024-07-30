import {  useEffect } from "react";
// import { useLocation } from 'react-router-dom';
import "./styles/input.css";
import "preline";
import { IStaticMethods } from "preline";
import { Route, Routes } from "react-router-dom";
import { Home } from "./pages/Home";
import Layout from "./pages/Layout";
import Root from "./pages/Root";
import UsersList from "./pages/UsersList";
import User from "./pages/User";
import Test1 from "./pages/Test1";
import NotFound from "./pages/NotFound";
import { useAuth0 } from "@auth0/auth0-react";
import PageLoader from "./components/common/PageLoader";
import { AuthenticationGuard } from "./components/auth/AuthenticationGuard";

// prelineが全体で使えるように
declare global {
  interface Window {
    HSStaticMethods: IStaticMethods;
  }
}

const App: React.FC = () => {
  const { isLoading } = useAuth0();
  // const location = useLocation();

  // useEffect(() => {
  //   window.HSStaticMethods.autoInit();
  // }, [location.pathname]);

  // prelineの初期化（重要）
  useEffect(() => {
    if (window.HSStaticMethods && typeof window.HSStaticMethods.autoInit === 'function') {
      window.HSStaticMethods.autoInit();
    }
  }, []);

  if (isLoading) {
    return (
      <div className="page-layout">
        <PageLoader />
      </div>
    );
  }

  return (
    <>
      <Routes>
        <Route path="/" element={<Home />} />
        <Route path="/user/" element={<Layout />}>
            <Route path="" element={<Root />} />
            <Route path="users" element={<AuthenticationGuard component={UsersList} />} />
            <Route path="users/:id" element={<User />} />
            <Route path="test1" element={<AuthenticationGuard component={Test1} />} />
            <Route path="*" element={<NotFound />} />
        </Route>

      </Routes>
      
    </>
  );
};
export default App;

{/* <RouterProvider router={router} /> */}