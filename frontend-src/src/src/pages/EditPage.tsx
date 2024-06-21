import Edit from "@/components/Features/components/edit";
import { Button } from "@/components/ui/button";
import { Link } from "react-router-dom";

export default function HomePage() {
    return (
        <div className="flex flex-col items-center justify-center">
            <Edit />
            <Link to="/">
                <Button size="lg">Back</Button>
            </Link>
        </div>
    );
}
