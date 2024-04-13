import { Button } from "../../ui/button";
import Cog from "@/assets/Cog.svg";

export default function Develop() {
  function develop() {
    window.open("https://github.com/FssAy/secrets-cafe", "_blank");
  }

  return (
    <div className="grid grid-cols-3 items-center justify-center gap-5 mt-7">
      <div className="col-start-2 flex items-center justify-center">
        <img className=" w-64" src={Cog} alt="heart emoji" />
      </div>
      <h1 className="col-span-3 justify-center items-center text-center text-7xl font-amatic font-medium">
        Develop
      </h1>
      <h2 className="col-span-3 text-center p-3 md:col-start-2 md:col-end-3">
        Using our open API, you can create your own projects based on
        secrets.cafe, just visit the API documentation for more details.
      </h2>
      <Button
        onClick={develop}
        className="col-start-2 bg-white font-amatic text-black rounded-base border-2 border-black justify-center items-center text-center text-2xl font-medium hover:bg-slate-300"
      >
        Docs
      </Button>
    </div>
  );
}
