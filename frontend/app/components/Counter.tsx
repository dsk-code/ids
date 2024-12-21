import { useEffect, useState } from "react";


const Counter: React.FC = () => {
    const [count, setCount] = useState<number>(0);
          
    useEffect(() => {
      console.log(`count is ${count}`);
    }, [count]);
  
    const cricktest = () => {
      const logElement = document.querySelector('#log');
      if (logElement != null) {
        logElement.innerHTML = '新しい内容';
      }
    }
    const overtest = () => {
      const logElement = document.querySelector('#log');
      if (logElement != null) {
        logElement.innerHTML = '古い内容';
      }
    }
  
    const clickCounter = () => {
      setCount(c => (c < 30 ? c + 1 : c))
      
    }
  
    return (
      <>
        <button id="log" onClick={cricktest} onMouseOver={overtest}>現在の内容</button>
        <label>{count}</label>
        <button onClick={clickCounter}>count</button>
      </>
    );
  };
  export default Counter;