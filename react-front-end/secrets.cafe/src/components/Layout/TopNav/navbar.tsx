import Coffee from "@/assets/coffee.svg";
("use client");


export default function Navbar() {
  return (
    <>
      <nav className="flex justify-between items-center p-4">
        <div className="flex items-center">
          <img
            src={Coffee}
            alt="Secrets Cafe"
            className="gap-2 w-6 h-6 mr-1"
          />
          <a href="/" className="text-xl font-bold ">secrets.cafe</a>
        </div>
        <div className="flex gap-8 font-sans font-semibold text-base">
          <a href="/" className="hover:text-[#6C63FF]">Home</a>
          <a href="/discover" className="hover:text-[#6C63FF]">Discover</a>
          <a href="/donate" className="hover:text-[#6C63FF]">Donate</a>
          <a href="/develop" className="hover:text-[#6C63FF]">Develop</a>
          <a href="/contribute" className="hover:text-[#6C63FF]">Contribute</a>
        </div>
      </nav>
      <hr />
    </>
  );
}
