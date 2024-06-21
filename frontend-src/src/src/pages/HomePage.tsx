import Navbar from "@/components/Layout/TopNav/navbar";
import Footer from "@/components/Layout/Footer/Footer";
import Home from "@/components/Features/components/home";
import { useEffect } from "react";

export default function HomePage() {
    useEffect(() => {
        localStorage.removeItem('isEditOpen');
    }, []);

    return (
        <div className="flex flex-col min-h-screen">
            <Navbar />
            <main className="flex-grow mt-[50px]">
                <Home />
            </main>
            <hr />
            <Footer />
        </div>
    );
}