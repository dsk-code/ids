// src/api.ts
// export interface ApiResponse<T> {
//     success: boolean;
//     data: T;
//     message?: string;
//   }
  
export const postData = async <T>(
    url: string,
    data: object,
    token: string
): Promise<T> => {
    const response = await fetch(url, {
        method: "POST",
        headers: {
            "Content-Type": "application/json",
            Authorization: `Bearer ${token}`, // 認証トークン
        },
        body: JSON.stringify(data),
    });
  
    if (!response.ok) {
      throw new Error(`Error: ${response.status} ${response.statusText}`);
    }
  
    return response.json();
};
  