import IntrovertCat from "@/assets/github-mark.svg";

const Footer = () => {
    return (
        <div className="flex justify-between items-center h-[50px]">
            <a
                href="https://creativecommons.org/licenses/by-nc/4.0/?ref=chooser-v1"
                className="text-sm font-medium ml-10"
            >
                secrets.cafe © 2024 is licensed under CC BY-NC 4.0
            </a>
            <a href="https://github.com/FssAy/secrets-cafe">
                <img src={IntrovertCat} alt="Introvert Cat" className="w-6 h-6 mr-10" />
            </a>
        </div>
    );
};

export default Footer;