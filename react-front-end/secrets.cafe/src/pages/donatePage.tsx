import Navbar from "@/components/Layout/TopNav/navbar";
import Footer from "@/components/Layout/Footer";
import Donate from "@/components/donate";

export default function DonatePage() {
  return (
    <div className="flex flex-col min-h-screen">
      <Navbar />
      <main className="flex-grow">
        <Donate />
      </main>
      <Footer />
    </div>
  );
}
