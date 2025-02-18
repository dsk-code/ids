import { useAuth0 } from "@auth0/auth0-react"
import { useAccessToken } from "../accesstoken/useAccessToken";
import { putData } from "~/api/api";
import { ResponseEntity } from "~/types/response/responseTypes";
import { PutRequestParts } from "~/types/request/requestPartsTypes";

export const usePutRequest = () => {
    const { isAuthenticated } = useAuth0();
    const { getAccessToken, isLoading, setIsLoading } = useAccessToken();

    const putRequest = async <T extends ResponseEntity>(parts: PutRequestParts) => {
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
                const response = await putData<T>(
                    parts.apiPath,
                    parts.payload,
                    accessToken
                );
                console.log("Response:", response);
                return { success: true, data: response };
            }
        } catch (e) {
            const errorMessage = e instanceof Error ? e.message : "An unknown error occurred";
            console.error(errorMessage);
            return { success: false, message: errorMessage };
        } finally {
            setIsLoading(false);
        }
    }

    return { putRequest, isLoading, setIsLoading }
}