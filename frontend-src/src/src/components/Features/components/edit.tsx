import TextEditor from "../../utils/TextEditor";

export default function Edit() {
  return (
    <>
      <div className="flex flex-col">
          <div className="container mx-auto px-4 py-8">
            <div className="bg-white shadow-md rounded-lg p-6">
              <TextEditor />
            </div>
          </div>
      </div>
    </>
  );
}
