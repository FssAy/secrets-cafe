import people from "@/assets/home-page-logo.svg";
import { Button } from "@/components/ui/button";
import { Link } from "react-router-dom";

export default function Home() {
  return (
    <div className="grid grid-col-3 gap-1 content-center">
      <h1 className="col-start-1 col-end-3 text-center text-[28px] font-bold mb-10">
        Ever wanted to vent off about something you cannot share with anyone
        around? <br />If so, this is a place for you!
      </h1>
      <div className="col-start-1 row-start-2 justify-self-center">
        <img src={people} alt="Secrets Cafe" className="w-[500px] h-auto" />
      </div>
      <div className="gap-2 col-start-2 flex flex-col justify-center items-center justify-self-start">
        <p className="text-center text-base font-sans font-semibold">
          Share your secrets, vent off, write your story, or see what <br />other
          people posted with full anonymity.
        </p>
        <p className="text-center text-base font-sans font-normal">
          You don't have to sign-up, and
          there are no ads to track you.<br /> Each post is verified before getting
          public to check for misuse,<br /> spam, or harmful content, so browse
          without worries.
        </p>
        <div className="flex space-x-4 my-8">
          <Link to="/write">
            <Button size="lg">Write</Button>
          </Link>
          <Link to="/browse">
            <Button size="lg">Browse</Button>
          </Link>
        </div>
      </div>
    </div>
  );
}