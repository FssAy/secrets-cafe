import { BrowserRouter as Router, Routes, Route } from "react-router-dom";
import HomeContent from "./pages/HomePage";
import DonatePage from "@/pages/DonatePage";
import DevelopPage from "@/pages/DevelopPage";
import DiscoverPage from "./pages/DiscoverPage";

function App() {
  return (
    <Router>
      <Routes>
        <Route path="/" element={<HomeContent />} />
        <Route path="/discover" element={<DiscoverPage />} />
        <Route path="/donate" element={<DonatePage />} />
        <Route path="/develop" element={<DevelopPage />} />
        <Route path="/contribute" element={<DevelopPage />} />
      </Routes>
    </Router>
  );
}

export default App;
