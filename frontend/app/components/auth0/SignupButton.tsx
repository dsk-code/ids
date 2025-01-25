import { useAuth0 } from "@auth0/auth0-react";
import React from "react";

export const SignupButton: React.FC = () => {
  const { loginWithRedirect } = useAuth0();

  const handleSignUp = async () => {
    await loginWithRedirect({
      authorizationParams: {
        screen_hint: "signup",
      },
    });
  };

  return (
    <button className="px-1 py-2 bg-black font-bold rounded-lg hover:bg-gray-500
        transition-all duration-300" onClick={handleSignUp}>
      サインアップ
    </button>
  );
};