export interface RequestGetAddress {
    postCode: string;
}

export interface ResponseGetAddress {
    status: number;
    message?: string;
    results: Address[]
}

export interface Address {
    address1: string;
    address2: string;
    address3: string;
    kana1: string;
    kana2: string;
    kana3: string;
    prefcode: string;
    zipcode: string;

}
