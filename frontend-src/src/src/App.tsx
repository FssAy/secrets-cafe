import { BrowserRouter as Router, Routes, Route } from "react-router-dom";
import HomePage from "./pages/HomePage";
import DonatePage from "@/pages/DonatePage";
import DevelopPage from "@/pages/DevelopPage";
import DiscoverPage from "@/pages/DiscoverPage";
import ContributePage from "@/pages/ContributePage";
import WritePage from "@/pages/WritePage";
import RulesPage from "@/pages/RulesPage";
import BrowsePage from "@/pages/BrowsePage";

function App() {
  return (
    <Router>
      <Routes>
        <Route path="/" element={<HomePage />} />
        <Route path="/discover" element={<DiscoverPage />} />
        <Route path="/donate" element={<DonatePage />} />
        <Route path="/develop" element={<DevelopPage />} />
        <Route path="/contribute" element={<ContributePage />} />
        <Route path="/write" element={<WritePage />} />
        <Route path="/browse" element={<BrowsePage />} />
        <Route path="/rules" element={<RulesPage />} />
      </Routes>
    </Router>
  );
}

export default App;
