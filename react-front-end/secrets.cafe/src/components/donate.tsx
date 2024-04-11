import HeartEmoji from "@/assets/heart-emoji.svg";
import { Button } from "./ui/button";

export default function Donate() {
  function donate() {
    window.open("https://ko-fi.com/secrets_cafe", "_blank");
  }

  return (
    <div className="grid grid-cols-3 items-center justify-center gap-5 mt-7">
      <div className="col-start-2 flex items-center justify-center">
        <img className=" w-64" src={HeartEmoji} alt="heart emoji" />
      </div>
      <h1 className="col-span-3 justify-center items-center text-center text-7xl font-amatic font-medium">
        Donate
      </h1>
      <h2 className="col-span-3 text-center p-3 md:col-start-2 md:col-end-3">
        Secrets.cafe is a freely available open-source project that operates
        without any income. If you value its existence and would like to support
        its continuation, please consider making a donation of any amount. Your
        contributions will go towards covering upkeep costs.
      </h2>
      <Button
        onClick={donate}
        className="col-start-2 bg-white font-amatic text-black rounded-base border-2 border-black justify-center items-center text-center text-2xl font-medium hover:bg-slate-300"
      >
        Donate
      </Button>
    </div>
  );
}
