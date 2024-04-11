import { BrowserRouter as Router, Routes, Route } from "react-router-dom";
import HomeContent from './components/Layout/HomeContent';

function App() {

  return (
    <Router>
      <Routes>
        <Route path="/" element={<HomeContent />} />
      </Routes>
    </Router>
  );
}

export default App;