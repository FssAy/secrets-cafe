import { BrowserRouter as Router, Routes, Route } from "react-router-dom";
import HomeContent from "./components/Layout/HomeContent";
import LandingPage from "./pages/landingPage";
import DonatePage from "./pages/donatePage";

function App() {
  return (
    <Router>
      <Routes>
        <Route path="/" element={<LandingPage />} />
        <Route path="/donate" element={<DonatePage />} />
      </Routes>
    </Router>
  );
}

export default App;
