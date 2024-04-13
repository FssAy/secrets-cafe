import TextEditor from "../../utils/TextEditor";

export default function Edit() {

  return (
    <>
      <div className="flex flex-col">
        <main className="flex-grow">
          <div className="container mx-auto px-4">
            <div className="mt-8">
              <TextEditor />
            </div>
          </div>
        </main>
      </div>
    </>
  );
}
