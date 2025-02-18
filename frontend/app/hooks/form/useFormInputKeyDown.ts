// form入力時にenterキーを押すとsubmitされるのを防ぐ為にenterでフォーカスを消しています
export const useFormInputKeyDown = () => {
    const handleFormInputKeyDown = (event: React.KeyboardEvent<HTMLInputElement>) => {
        if (event.key === "Enter" && event.nativeEvent.isComposing === false) {
            event.preventDefault();
            const activeElement = document.activeElement as HTMLElement;
            if (activeElement) {
                activeElement.blur();
            }
        }
    };

    return { handleFormInputKeyDown }
}