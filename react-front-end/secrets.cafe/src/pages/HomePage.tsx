import Navbar from "@/components/Layout/TopNav/navbar";
import Footer from "@/components/Layout/Footer";
import Home from "@/components/home";

export default function HomePage() {
    return (
        <div className="flex flex-col min-h-screen">
            <Navbar />
            <main className="flex-grow">
                <Home />
            </main>
            <Footer />
        </div>
    );
}
