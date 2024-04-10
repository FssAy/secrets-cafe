import Latte from "../assets/Latte.svg";

export default function Landing() {
  return (
    <>
      <div className="grid grid-cols-3 items-center justify-center gap-5">
        <div className=" col-start-2 flex items-center justify-center">
          <img src={Latte} alt="Secrets Cafe" className="w-56 ml-5" />
        </div>
        <h1 className="col-start-1 col-end-5 justify-center items-center text-center text-6xl font-amatic font-medium">
          secrets.cafe
        </h1>
        <h2 className="col-start-1 col-end-5 justify-center items-center text-center text-2xl font-medium md:col-start-2 md:col-end-3">
          Ever wanted to vent off about something you cannot share with anyone
          around? If so, this is a place for you!
        </h2>
        <h4 className="col-start-1 col-end-5 justify-center items-center text-center text-sm font-medium md:col-start-2 md:col-end-3">
          Share your secrets, vent off, wrtie your story, or see what other
          people posted with full anonymity. You don't have to sign-up, and
          there are no ads to track you. Each post is verified before getting
          public to check for misuse, spam, or harmful content, so browse
          without worries.
        </h4>
        <h5 className="col-start-1 col-end-5 justify-center items-center text-red-500 text-center text-sm font-medium md:col-start-2 md:col-end-3">
          This is a "pre-release" of this website, bugs and issues are to be
          expected.
        </h5>
      </div>
    </>
  );
}
