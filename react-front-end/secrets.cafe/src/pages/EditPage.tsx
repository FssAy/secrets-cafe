import Navbar from "@/components/Layout/TopNav/navbar";
import Footer from "@/components/Layout/Footer/Footer";
import Edit from "@/components/Features/components/edit";
import { Button } from "@/components/ui/button";
import { Link } from "react-router-dom";


export default function HomePage() {
    return (
        <div className="flex flex-col min-h-screen">
            <Navbar />
            <main className="flex-grow flex flex-col items-center justify-center">
                <Edit />
                <div className="flex justify-center my-8">
                    <Link to="/">
                        <Button size="lg">Back</Button>
                    </Link>
                </div>
            </main>
            <footer className="mt-auto">
                <Footer />
            </footer>
        </div>
    );
}