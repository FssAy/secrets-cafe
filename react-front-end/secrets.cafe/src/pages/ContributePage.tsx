import Navbar from "@/components/Layout/TopNav/navbar";
import Footer from "@/components/Layout/Footer";
import Contribute from "@/components/contribute";

export default function ContributePage() {
    return (
        <div className="flex flex-col min-h-screen">
            <Navbar />
            <main className="flex-grow">
                <Contribute />
            </main>
            <Footer />
        </div>
    );
}
