import { useAuth0 } from "@auth0/auth0-react"
import { useAccessToken } from "../accesstoken/useAccessToken";
import { getData } from "~/api/api";
import { GetRequestParts } from "~/types/request/requestPartsTypes";
import { ResponseEntity } from "~/types/response/responseTypes";
import { data } from "@remix-run/react";

export const useGetRequest = () => {
    const { isAuthenticated } = useAuth0();
    const { getAccessToken, isLoading, setIsLoading } = useAccessToken();

    const getRequest = async <T extends ResponseEntity>(parts: GetRequestParts) => {
        if (!isAuthenticated) {
            console.error("User is not authenticated");
            return { success: false, message: "User is not authenticated" };
        }

        setIsLoading(true);
        try {
            // アクセストークンの取得
            const accessToken = await getAccessToken();
        
            if (accessToken) {
                // APIにPOSTリクエスト
                console.log("リクエスト開始");
                const response = await getData<T>(
                    parts.apiPath,
                    accessToken,
                );
                if ( response ) {
                    console.log("Response:", response);
                    return { success: true, data: response };
                }
            }
        } catch (e) {
            const errorMessage = e instanceof Error ? e.message : "An unknown error occurred";
            console.error(errorMessage);
            return { success: false, message: errorMessage };
        } finally {
            setIsLoading(false);
        }
    }

    return { getRequest, isLoading, setIsLoading }
}
