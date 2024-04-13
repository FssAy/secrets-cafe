import Latte from "@/assets/Latte.svg";

export default function Home() {

  return (
    <>
      <div className="flex flex-col">
        <main className="flex-grow">
          <div className="container mx-auto px-4">
            <div className="flex justify-center">
              <img src={Latte} alt="Secrets Cafe" className="w-40 h-25" />
            </div>
            <h1 className="mb-6 text-center text-6xl font-amatic font-medium">
              secrets.cafe
            </h1>
            <h2 className="mb-4 text-center text-2xl font-medium">
              Ever wanted to vent off about something you cannot share with anyone
              around? If so, this is a place for you!
            </h2>
            <p className="mb-4 text-center text-lg">
              Share your secrets, vent off, write your story, or see what other
              people posted with full anonymity. You don't have to sign-up, and
              there are no ads to track you. Each post is verified before getting
              public to check for misuse, spam, or harmful content, so browse
              without worries.
            </p>
            <p className="text-center text-sm text-red-500">
              This is a "pre-release" of this website, bugs and issues are to be
              expected.
            </p>
          </div>
        </main>
      </div>
    </>
  );
}
