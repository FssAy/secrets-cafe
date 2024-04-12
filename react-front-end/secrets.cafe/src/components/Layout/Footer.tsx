import IntrovertCat from "@/assets/github-mark.svg";

const Footer = () => {
    return (
        <footer className="py-4 bg-gray-100 ">
            <div className="container mx-auto px-4 flex items-center justify-center space-x-4">
                <a
                    href="https://creativecommons.org/licenses/by-nc/4.0/?ref=chooser-v1"
                    className="text-sm font-medium"
                >
                    secrets.cafe © 2024 is licensed under CC BY-NC 4.0
                </a>
                <a href="https://github.com/FssAy/secrets-cafe">
                    <img src={IntrovertCat} alt="Introvert Cat" className="w-6 h-6" />
                </a>
            </div>
        </footer>
    );
};

export default Footer;