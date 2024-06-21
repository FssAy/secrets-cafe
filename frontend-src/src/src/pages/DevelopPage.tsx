import Navbar from "@/components/Layout/TopNav/navbar";
import Develop from "@/components/Features/components/develop";
import Footer from "@/components/Layout/Footer/Footer";

export default function DevelopPage() {
  return (
    <div className="flex flex-col min-h-screen">
      <Navbar />
      <main className="flex-grow">
        <Develop />
      </main>
      <hr />
      <Footer />
    </div>
  );
}
