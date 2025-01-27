// src/api.ts
// export interface ApiResponse<T> {
//     success: boolean;
//     data: T;
//     message?: string;
//   }
  
export const getData = async <T>(
    url: string,
    token: string
): Promise<T> => {
    const response = await fetch(url, {
        method: "GET",
        headers: {
            "Content-Type": "application/json",
            Authorization: `Bearer ${token}`, // 認証トークン
        },
    });
  
    if (!response.ok) {
      throw new Error(`Error: ${response.status} ${response.statusText}`);
    }
  
    return response.json();
};

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

export const deleteData = async (
    url: string,
    token: string
) => {
    const response = await fetch(url, {
        method: "DELETE",
        headers: {
            "Content-Type": "application/json",
            Authorization: `Bearer ${token}`, // 認証トークン
        },
    });
  
    if (response.status === 204) {
        return response;
    } else {
        throw new Error(`Error: ${response.status} ${response.statusText}`);
    }  
};

  