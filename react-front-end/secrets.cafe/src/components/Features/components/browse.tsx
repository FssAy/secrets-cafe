import browseGuy from "@/assets/browse-page-logo.svg";
import { Button } from "@/components/ui/button";
import SearchIcon from '@mui/icons-material/Search';

export default function Browse() {
    return (
        <div className="grid grid-cols-1 md:grid-cols-2 gap-4 content-center p-4">
            <h1 className="text-center text-6xl font-bold mb-1 mr-[250px]">
                Browse
            </h1>
            <div className="md:col-start-1 md:col-end-2 md:row-start-2 flex justify-center md:pl-20">
                <img src={browseGuy} alt="Write your story" className="w-full md:w-[600px] h-auto" />
            </div>
            <div className="md:col-start-2 md:row-start-2 md:pl-10 md:justify-self-start md:ml-[-50px] flex flex-col justify-center">
                <div className="mb-8">
                    <p className="text-[30px] font-semibold mb-4">
                        Here you can read stories posted by other people.
                    </p>
                    <p className="text-[30px] font-normal text-center">
                        Get a random one or insert its unique code.
                    </p>
                </div>
                <div className="flex flex-col items-center space-y-1">
                    <div className="relative">
                        <input
                            className="border-[1px] p-input-code border-[#616161] rounded-base pr-12"
                            placeholder="Secret code"
                        />
                        <Button size="sm" className="absolute top-1/2 right-1 transform -translate-y-1/2">
                            <SearchIcon />
                        </Button>
                    </div>
                    <div className="flex items-center justify-center">
                        <p>or</p>
                    </div>
                    <div className="flex items-center justify-center">
                        <Button size="lg">
                            Get Random
                        </Button>
                    </div>
                </div>
            </div>
        </div>
    )
}