import { Button } from "@/components/ui/button";
import discoverMark from "@/assets/discover.svg";
import { Link } from "react-router-dom";

export default function Develop() {

  return (
    <div className="grid grid-cols-3 items-center justify-center gap-5 mt-[130px]">
      <div className="col-start-2 flex items-center justify-center">
        <img className=" w-[180px]" src={discoverMark} alt="heart emoji" />
      </div>
      <h2 className="col-span-3 text-center p-5 md:col-span-3 font-['Open_Sans'] text-base font-normal leading-[27.2px] whitespace-pre-line">
        {`Sorry, this page is still under construction, come back later :)`}
      </h2>
      <div className="col-start-2 justify-center items-center text-center">
        <Link to="/">
          <Button size="lg">
            Docs
          </Button>
        </Link>
      </div>
    </div>
  );
}