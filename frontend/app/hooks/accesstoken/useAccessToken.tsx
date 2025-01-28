import { useAuth0 } from "@auth0/auth0-react"
import { useState } from "react";
import useEnv from "../useEnv";

export const useAccessToken = () => {
    const {isAuthenticated, getAccessTokenSilently} = useAuth0();
    const [isLoading, setIsLoading] = useState(false);
    const { audience } = useEnv();

    const getAccessToken = async () => {
        if (isAuthenticated) {
            setIsLoading(true);
            try {
                return await getAccessTokenSilently({
                    authorizationParams: { audience },
                });
            } catch (error) {
                console.error("Failed to get access token:", error);
                throw error;
            } finally {
                setIsLoading(false);
            }
        }
    };

    return { getAccessToken, isLoading, setIsLoading };
}