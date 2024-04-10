import { useState } from "react";
import Navbar from "./components/navbar";
import Landing from "./components/landing";
import Footer from "./components/footer";

function App() {
  const [count, setCount] = useState(0);

  return (
    <>
      <Navbar />
      <Landing />
      <Footer />
    </>
  );
}

export default App;
