import { Button } from "@/components/ui/button";
import developMark from "@/assets/develop.svg";

export default function Develop() {
  function develop() {
    window.open("https://github.com/FssAy/secrets-cafe", "_blank");
  }

  return (
    <div className="grid grid-cols-3 items-center justify-center gap-5 mt-20">
      <div className="col-start-2 flex items-center justify-center">
        <img className=" w-[180px]" src={developMark} alt="heart emoji" />
      </div>
      <h1 className="col-span-3 justify-center items-center text-center font-['Open_Sans'] font-bold text-[48px] leading-[60px]">
        Develop
      </h1>
      <h2 className="col-span-3 text-center p-5 md:col-span-3 font-['Open_Sans'] text-base font-normal leading-[27.2px] whitespace-pre-line">
        {`Using our open API, you can create your own projects based on 
          secrets.cafe, just visit the API documentation for more details.`}
      </h2>
      <div className="col-start-2 justify-center items-center text-center">
        <Button onClick={develop} size="lg">
          Docs
        </Button>
      </div>
    </div>
  );
}