interface EnvVariables {
    VITE_AUTH0_DOMAIN: string;
    VITE_AUTH0_CLIENT_ID: string;
    VITE_AUTH0_CALLBACK_URL: string;
    VITE_AUTH0_AUDIENCE: string;
    VITE_AUTH0_SCOPE: string;
    VITE_AUTH0_APP_DOMEIN: string;
    VITE_BACKEND_API_URL: string;
    VITE_ZIPCLOUD_URL: string;
}
  
const useEnv = () => {
    // unknown経由で型変換
    const env = import.meta.env as unknown as EnvVariables ;
    
    return {
        domain: env.VITE_AUTH0_DOMAIN || "",
        clientId: env.VITE_AUTH0_CLIENT_ID || "",
        callbackUrl: env.VITE_AUTH0_CALLBACK_URL || "",
        audience: env.VITE_AUTH0_AUDIENCE || "",
        scope: env.VITE_AUTH0_SCOPE || "",
        appDmain: env.VITE_AUTH0_APP_DOMEIN || "",
        backendApiUrl: env.VITE_BACKEND_API_URL || "",
        zipcloudUrl: env.VITE_ZIPCLOUD_URL || "",
    };
};
  
export default useEnv;
  