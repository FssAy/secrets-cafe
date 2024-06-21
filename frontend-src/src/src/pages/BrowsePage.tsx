import Navbar from "@/components/Layout/TopNav/navbar";
import Footer from "@/components/Layout/Footer/Footer";
import Browse from "@/components/Features/components/browse";

export default function HomePage() {
    return (
        <div className="flex flex-col min-h-screen">
            <Navbar />
            <main className="flex-grow mt-[50px]">
                <Browse />
            </main>
            <hr/>
            <Footer />
        </div>
    )
}