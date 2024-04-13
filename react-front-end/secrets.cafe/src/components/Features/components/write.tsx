import HeartEmoji from "@/assets/heart-emoji.svg";

export default function Write() {

  return (
    <div className="grid grid-cols-3 items-center justify-center gap-5 mt-7">
      <div className="col-start-2 flex items-center justify-center">
        <img className=" w-64" src={HeartEmoji} alt="heart emoji" />
      </div>
      <h1 className="col-span-3 justify-center items-center text-center text-5xl font-amatic font-medium">
        Share your story, secret, or anything you wish to.
        Remember it will go public and could never be removed.
      </h1>
      <h2 className="col-span-3 text-center p-3 md:col-start-2 md:col-end-3">
        Please keep it respectful, follow the rules and guidelines.
        If your post is considered as spam, or if it's harmful it will be rejected.
        If you understand and agree to our terms click "I Agree" button.
      </h2>
    </div>
  );
}
