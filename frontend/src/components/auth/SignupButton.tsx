import { useAuth0 } from "@auth0/auth0-react";
import React from "react";

export const SignupButton: React.FC = () => {
  const { loginWithRedirect } = useAuth0();

  const handleSignUp = async () => {
    await loginWithRedirect({
      appState: {
        returnTo: "/user/users",
      },
      authorizationParams: {
        screen_hint: "signup",
      },
    });
  };

  return (
    <button className="px-6 py-2 bg-theme font-bold rounded-lg hover:bg-purple-600 
        transition-all duration-300" onClick={handleSignUp}>
      Sign Up
    </button>
  );
};