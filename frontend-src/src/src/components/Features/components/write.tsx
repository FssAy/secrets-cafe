import girl from "@/assets/write-page-logo.svg";
import { Button } from "@/components/ui/button";
import { useState, useEffect } from "react";
import EditPage from "@/pages/EditPage";
import { cn } from "@/lib/utils";

export default function Home() {
  const [isEditOpen, setIsEditOpen] = useState(() => {
    const storedIsEditOpen = localStorage.getItem('isEditOpen');
    return storedIsEditOpen ? JSON.parse(storedIsEditOpen) : false;
  });
  const [isChecked, setIsChecked] = useState(false);

  useEffect(() => {
    localStorage.setItem('isEditOpen', JSON.stringify(isEditOpen));
  }, [isEditOpen]);

  const handleOpenEdit = () => {
    setIsEditOpen(true);
  }

  const handleCheckboxChange = () => {
    setIsChecked(!isChecked);
  }

  return (
    isEditOpen ? (
      <EditPage />
    ) : (
      <div className="grid grid-cols-1 md:grid-cols-2 gap-4 content-center p-4">
        <h1 className="text-center text-2xl md:text-3xl font-bold mb-8 md:mb-10">
          Share your story, secret, or anything you wish to!
        </h1>
        <div className="md:col-start-1 md:col-end-2 md:row-start-2 flex justify-center md:pl-20">
          <img src={girl} alt="Write your story" className="w-full md:w-[750px] h-auto" />
        </div>
        <div className="md:col-start-2 md:row-start-2 md:pl-20 md:justify-self-start md:ml-[-50px] flex flex-col justify-center">
          <div className="mb-8">
            <p className="text-lg font-semibold mb-4">
              Remember it will go public and could never be removed.
            </p>
            <p className="text-lg font-normal">
              Please keep it respectful, follow the rules and guidelines.<br />
              If your post is considered as spam, or if it's harmful it will be rejected.<br />
              If you understand and agree to our terms click "I Agree" button.
            </p>
          </div>
          <div className="flex flex-col items-center">
            <Button
              size="lg"
              onClick={handleOpenEdit}
              className={cn(
                "mb-4",
                !isChecked && "bg-gray-300 cursor-not-allowed hover:bg-gray-300"
              )}
              disabled={!isChecked}
            >
              Write
            </Button>
            <div className="flex items-center justify-center">
              <input
                type="checkbox"
                id="agree"
                checked={isChecked}
                onChange={handleCheckboxChange}
                className="form-checkbox h-4 w-4 text-blue-500 mr-2"
              />
              <label htmlFor="agree" className="text-sm">
                I agree to the rules and guidelines
              </label>
            </div>
          </div>
        </div>
      </div>
    )
  )
}