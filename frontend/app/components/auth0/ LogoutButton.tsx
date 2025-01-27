import { useAuth0 } from "@auth0/auth0-react";
import React from "react";

export const LogoutButton: React.FC = () => {
  const { logout } = useAuth0();

  const handleLogout = () => {
    logout({
      logoutParams: {
        returnTo: window.location.origin,
      },
    });
  };

  return (
    <button className="px-1 py-2 bg-transparent font-bold rounded-lg hover:bg-gray-200 
        transition-all duration-300 ml-3 text-gray-600" onClick={handleLogout}>
      ログアウト
    </button>
  );
};