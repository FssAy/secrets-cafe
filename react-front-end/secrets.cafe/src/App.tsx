import { useState } from "react";
import Navbar from "./components/navbar";

function App() {
  const [count, setCount] = useState(0);

  return (
    <>
      <Navbar />
      <h1>Test</h1>
    </>
  );
}

export default App;
