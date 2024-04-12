import Navbar from "@/components/Layout/TopNav/navbar";
import Footer from "@/components/Layout/Footer";
import Discover from "@/components/discover";

export default function DiscoverPage() {
    return (
        <div className="flex flex-col min-h-screen">
            <Navbar />
            <main className="flex-grow">
                <Discover />
            </main>
            <Footer />
        </div>
    );
}
