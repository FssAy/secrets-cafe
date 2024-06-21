import HeartEmoji from "@/assets/donate.svg";
import { Button } from "@/components/ui/button";
export default function Donate() {
    function donate() {
        window.open("https://ko-fi.com/secrets_cafe", "_blank");
    }
    return (
        <div className="grid grid-cols-3 items-center justify-center gap-5 mt-20">
            <div className="col-start-2 flex items-center justify-center">
                <img className="w-[180px]" src={HeartEmoji} alt="heart emoji" />
            </div>
            <h1 className="col-span-3 justify-center items-center text-center font-['Open_Sans'] font-bold text-[48px] leading-[60px]">
                Donate
            </h1>
            <h2 className="col-span-3 text-center p-3 md:col-start-2 md:col-end-3 font-['Open_Sans'] text-base font-normal leading-[27.2px]">
                Secrets.cafe is a freely available open-source project that operates
                without any income. If you value its existence and would like to support
                its continuation, please consider making a donation of any amount. Your
                contributions will go towards covering upkeep costs.
            </h2>
            <div className="col-start-2 justify-center items-center text-center">
                <Button onClick={donate} size="lg" >
                    Donate
                </Button>
            </div>
        </div>
    );
}