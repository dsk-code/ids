import { useAuth0 } from "@auth0/auth0-react"
import { useAccessToken } from "../accesstoken/useAccessToken";
import { deleteData } from "~/api/api";
import { DeleteRequestParts, GetRequestParts } from "~/types/requestPartsTypes";

export const useDeleteRequest = () => {
    const { isAuthenticated } = useAuth0();
    const { getAccessToken, isLoading, setIsLoading } = useAccessToken();

    const deleteRequest = async (parts: DeleteRequestParts) => {
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
                const response = await deleteData(
                    parts.apiPath,
                    accessToken
                );
                console.log("Response:", response);
                if (response.status === 204) {
                    return { success: true, message: "削除されました。" };
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

    return { deleteRequest, isLoading, setIsLoading }
}
