import IntrovertCat from "@/assets/github-mark.svg";

export default function Footer() {
  return (
    <>
      <div className="grid grid-cols-3 items-center justify-center mt-10 md:grid-cols-3">
        <a
          href="https://creativecommons.org/licenses/by-nc/4.0/?ref=chooser-v1"
          className="col-start-1 col-end-4 justify-center items-center text-center text-sm font-medium md:col-start-1 md:col-end-2"
        >
          secrets.cafe © 2024 is licensed under CC BY-NC 4.0
        </a>
        <div className="col-start-2 flex justify-center items-center mt-5 md:col-start-3 md:mt-0">
          <a href="https://github.com/FssAy/secrets-cafe">
            <img src={IntrovertCat} alt="Introvert Cat" className="w-10" />
          </a>
        </div>
      </div>
    </>
  );
}
