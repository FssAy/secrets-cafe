import Navbar from "@/components/Layout/TopNav/navbar";
import Develop from "@/components/develop";
import Footer from "@/components/Layout/Footer";

export default function DevelopPage() {
  return (
    <div className="flex flex-col min-h-screen">
      <Navbar />
      <main className="flex-grow">
        <Develop />
      </main>
      <Footer />
    </div>
  );
}
