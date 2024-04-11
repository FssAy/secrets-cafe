import { BrowserRouter as Router, Routes, Route } from "react-router-dom";
import HomeContent from "./components/Layout/HomeContent";
import LandingPage from "./pages/landingPage";

function App() {
  return (
    <Router>
      <Routes>
        <Route path="/" element={<LandingPage />} />
      </Routes>
    </Router>
  );
}

export default App;
