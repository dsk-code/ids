import { useAuth0 } from "@auth0/auth0-react"
import { useAccessToken } from "../accesstoken/useAccessToken";
import { getData, getDataWithoutAccessToken } from "~/api/api";
import { GetRequestParts } from "~/types/requestPartsTypes";
import { ResponseEntity } from "~/types/responseTypes";
import { useState } from "react";
import useEnv from "../useEnv";

export const useZipcloud = () => {
    const { isAuthenticated } = useAuth0();
    const [isLoading, setIsLoading] = useState(false);
    const { zipcloudUrl } = useEnv();

    const zipcloud = async (postCode: string) => {
        if (!isAuthenticated) {
            console.error("User is not authenticated");
            return { success: false, message: "User is not authenticated" };
        }

        setIsLoading(true);
        try {
            console.log(zipcloudUrl);
            console.log("リクエスト開始");
            const response = await getDataWithoutAccessToken(
                `${zipcloudUrl}/search?zipcode=${postCode}`
            );
            if ( response ) {
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

    return { zipcloud }
}
