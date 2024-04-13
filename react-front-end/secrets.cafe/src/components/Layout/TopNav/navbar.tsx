import Latte from "@/assets/Latte.svg";
("use client");


export default function Navbar() {
  return (
    <>
      <nav className="flex justify-between items-center p-4">
        <div className="flex items-center">
          <img
            src={Latte}
            alt="Secrets Cafe"
            className="gap-2 w-13 h-11 mr-2"
          />
          <a href="/" className="text-xl font-bold ">secrets.cafe</a>
        </div>
        <div className="flex gap-4">
          <a href="/discover" className="text-xl hover:underline">Discover</a>
          <a href="/donate" className="text-xl  hover:underline">Donate</a>
          <a href="/develop" className="text-xl hover:underline">Develop</a>
          <a href="/contribute" className="text-xl hover:underline">Contribute</a>
        </div>
      </nav>
    </>
  );
}
