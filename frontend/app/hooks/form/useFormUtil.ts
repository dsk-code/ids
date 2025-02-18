import moji from "moji";

export const useFormUtil = () => {
    const postCodeWithHyphen: (input:string) => string = (input: string) => {
        const trimedInput = input.trim();
        const halfWidthInput = moji(trimedInput).convert('ZE', 'HE').toString();
        const rmHyphen = halfWidthInput.replace(/[^0-9]/g, "");
        const postCode = rmHyphen.slice(0, 3) + "-" + rmHyphen.slice(3);

        return postCode
    }

    return { postCodeWithHyphen }
}