import Navbar from "@/components/Layout/TopNav/navbar";
import Footer from "@/components/Layout/Footer/Footer";
import Write from "@/components/Features/components/write";
import EditPage from "@/pages/EditPage";
import { Button } from "@/components/ui/button";
import { useState, useEffect } from "react";
import { Link } from "react-router-dom";

export default function HomePage() {
    const [isEditOpen, setIsEditOpen] = useState(() => {
        const storedIsEditOpen = localStorage.getItem('isEditOpen');
        return storedIsEditOpen ? JSON.parse(storedIsEditOpen) : false;
    });

    useEffect(() => {
        localStorage.setItem('isEditOpen', JSON.stringify(isEditOpen));
    }, [isEditOpen]);

    const handleOpenEdit = () => {
        setIsEditOpen(true);
    }

    return (
        isEditOpen ? (
            <EditPage />
        ) : (
            <div className="flex flex-col min-h-screen">
                <Navbar />
                <main className="flex-grow flex flex-col items-center justify-center">
                    <Write />
                    <div className="flex space-x-4 my-8 justify-center">
                        <Link to="/rules">
                            <Button size="lg">Rule</Button>
                        </Link>
                        <Button size="lg" onClick={handleOpenEdit}>I Agree</Button>
                    </div>
                </main>
                <footer className="mt-auto">
                    <Footer />
                </footer>
            </div>
        )
    );
}