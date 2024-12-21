import { useState } from "react";

const SearchBox: React.FC = () => {
    const [inputValue, setInputValue] = useState('');

    const handleChange = (event: React.ChangeEvent<HTMLInputElement>) => {
      setInputValue(event.target.value);
    };
    return(
        <>
            <div className="max-w-sm">
            {/* <!-- SearchBox --> */}
            <div className="relative" data-hs-combo-box='{
                "groupingType": "default",
                "isOpenOnFocus": true,
                "apiUrl": "../assets/data/searchbox.json",
                "apiGroupField": "category",
                "outputItemTemplate": "<div data-hs-combo-box-output-item><span className=\"flex items-center cursor-pointer py-2 px-4 w-full text-sm text-gray-800 hover:bg-gray-100 focus:outline-none focus:bg-gray-100"><div className=\"flex items-center w-full\"><div className=\"flex items-center justify-center rounded-full bg-gray-200 size-6 overflow-hidden me-2.5\"><img className=\"flex-shrink-0\" data-hs-combo-box-output-item-attr=&apos;[{\"valueFrom\": \"image\", \"attr\": \"src\"}, {\"valueFrom\": \"name\", \"attr\": \"alt\"}]&apos; /></div><div data-hs-combo-box-output-item-field=\"name\" data-hs-combo-box-search-text data-hs-combo-box-value></div></div><span className=\"hidden hs-combo-box-selected:block\"><svg className=\"flex-shrink-0 size-3.5 text-blue-600" xmlns=\"http:.w3.org/2000/svg\" width=\"24\" height=\"24\" viewBox=\"0 0 24 24\" fill=\"none\" stroke=\"currentColor\" stroke-width=\"2\" stroke-linecap=\"round\" stroke-linejoin=\"round\"><polyline points=\"20 6 9 17 4 12\"></polyline></svg></span></span></div>",
                "groupingTitleTemplate": "<div className=\"text-xs uppercase text-gray-500 m-3 mb-1"></div>"
            }'>
                <div className="relative">
                <div className="absolute inset-y-0 start-0 flex items-center pointer-events-none z-20 ps-3.5">
                    <svg className="flex-shrink-0 size-4 text-gray-400" xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round">
                    <circle cx="11" cy="11" r="8"></circle>
                    <path d="m21 21-4.3-4.3"></path>
                    </svg>
                </div>
                <input className="py-3 ps-10 pe-4 block w-full border-gray-200 rounded-lg text-sm focus:border-blue-500 focus:ring-blue-500 disabled:opacity-50 disabled:pointer-events-none" type="text" placeholder="Type a name" value={inputValue} onChange={handleChange} data-hs-combo-box-input="" />
                </div>

                {/* <!-- SearchBox Dropdown --> */}
                <div className="absolute z-50 w-full bg-white border border-gray-200 rounded-lg" style={{display: 'none',}} data-hs-combo-box-output="">
                <div className="max-h-72 rounded-b-lg overflow-hidden overflow-y-auto [&::-webkit-scrollbar]:w-2 [&::-webkit-scrollbar-thumb]:rounded-full [&::-webkit-scrollbar-track]:bg-gray-100 [&::-webkit-scrollbar-thumb]:bg-gray-300" data-hs-combo-box-output-items-wrapper="">
                    <div data-hs-combo-box-output-item>
                        <span className="flex items-center cursor-pointer py-2 px-4 w-full text-sm text-gray-800 hover:bg-gray-100 focus:outline-none focus:bg-gray-100">
                            <div className="flex items-center w-full">
                                <div className="flex items-center justify-center rounded-full bg-gray-200 size-6 overflow-hidden me-2.5\">
                                    <img className="flex-shrink-0" data-hs-combo-box-output-item-attr='[{"valueFrom": "image", "attr": "src"}, {"valueFrom": "name", "attr": "alt"}]' />
                                </div>
                                <div data-hs-combo-box-output-item-field="name" data-hs-combo-box-search-text data-hs-combo-box-value>
                                    test
                                </div>
                            </div>
                            <span className="hidden hs-combo-box-selected:block">
                                <svg className="flex-shrink-0 size-3.5 text-blue-600" xmlns="http:.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round">
                                    <polyline points="20 6 9 17 4 12"></polyline>
                                </svg>
                            </span>
                        </span>
                    </div>
                </div>
                </div>
                {/* <!-- End SearchBox Dropdown --> */}
            </div>
            {/* <!-- End SearchBox --> */}
            </div>
        </>
    )
};
export default SearchBox;