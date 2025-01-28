import { useAuth0 } from "@auth0/auth0-react"
import useEnv from "../useEnv";
import { useAccessToken } from "../accesstoken/useAccessToken";
import { Class, RequestPutClass } from "~/types/classTypes";
import { putData } from "~/api/api";

export const usePutClass = () => {
    const { isAuthenticated } = useAuth0();
    const { getAccessToken, isLoading, setIsLoading } = useAccessToken();
    const { backendApiUrl } = useEnv();

    const putClass = async (classId: string, payload: RequestPutClass) => {
        if (!isAuthenticated) {
            console.error("User is not authenticated");
            return { success: false, message: "User is not authenticated" };
        }

        setIsLoading(true);
        try {
            // アクセストークンの取得
            const accessToken = await getAccessToken();

            console.log(payload);
            if (accessToken) {
                // APIにPOSTリクエスト
                console.log("リクエスト開始");
                const response = await putData<Class>(
                    `${backendApiUrl}/classes/${classId}`,
                    payload,
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

    return { putClass, isLoading };
}