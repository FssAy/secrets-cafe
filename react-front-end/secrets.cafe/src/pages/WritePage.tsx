import Navbar from "@/components/Layout/TopNav/navbar";
import Footer from "@/components/Layout/Footer";
import Write from "@/components/write";
import EditPage from "@/pages/EditPage";
import { Button } from "@/components/ui/button";
import { useState } from "react";

export default function HomePage() {
    const [isEditOpen, setIsEditOpen] = useState(false);

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
                    <div className="flex space-x-4 my-8">
                        <Button size="lg">Rule</Button>
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