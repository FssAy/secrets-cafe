import Navbar from "@/components/Layout/TopNav/navbar";
import Footer from "@/components/Layout/Footer/Footer";
import Rule from "@/components/Features/components/rule";
import { Button } from "@/components/ui/button";
import { Link } from "react-router-dom";

export default function HomePage() {
    return (
        <div className="flex flex-col min-h-screen">
            <Navbar />
            <main className="flex-grow flex flex-col items-center justify-center">
                <Rule />
                <div className="flex space-x-4 my-8">
                    <Link to="/write">
                        <Button size="lg">OK</Button>
                    </Link>
                </div>
            </main>
            <footer className="mt-auto">
                <Footer />
            </footer>
        </div>
    );
}