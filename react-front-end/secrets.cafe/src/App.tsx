import { BrowserRouter as Router, Routes, Route } from "react-router-dom";
import LandingPage from "./pages/landingPage";
import DonatePage from "./pages/donatePage";
import DevelopPage from "./pages/developPage";

function App() {
  return (
    <Router>
      <Routes>
        <Route path="/" element={<LandingPage />} />
        <Route path="/donate" element={<DonatePage />} />
        <Route path="/develop" element={<DevelopPage />} />
      </Routes>
    </Router>
  );
}

export default App;
