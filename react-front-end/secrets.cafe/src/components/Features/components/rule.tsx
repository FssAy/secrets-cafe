import AccountBalanceIcon from '@mui/icons-material/AccountBalance';
export default function Home() {

    return (
        <>
            <div className="flex flex-col">
                <main className="flex-grow">
                    <div className="container mx-auto px-4">
                        <div className="flex justify-center">
                            <AccountBalanceIcon sx={{ fontSize: 120 }} />
                        </div>
                        <h1 className="mb-6 text-center text-6xl font-amatic font-medium">
                            Rules
                        </h1>
                        <div className="flex flex-col w-[600px]">
                            <main className="flex flex-col justify-center">
                                <h2 className="mb-4 text-2xl font-medium w-[700px]">
                                    Please get to know these rules and follow them when posting.
                                </h2>
                                <ul className="mb-4 mr-5 text-xl list-decimal list-inside">
                                    <li>
                                        <span>
                                            No NSFW content.
                                        </span>
                                    </li>
                                    <li>
                                        <span>
                                            Don't include any URLs. This could impact the user's privacy.
                                        </span>
                                    </li>
                                    <li>
                                        <span>
                                            For your own safety don't include any personal information.
                                        </span>
                                    </li>
                                    <li>
                                        <span>
                                            If you don't agree with someone's views, don't report the post.
                                        </span>
                                    </li>
                                    <li>
                                        <span>
                                            Any harmful, illegal or malicious content will be rejected.
                                        </span>
                                    </li>
                                </ul>
                                <p className="text-left text-md text-red-500 w-full">
                                    Keep in mind that these rules might change in the future,
                                    if you disagree with something please visit our GitHub repository and make an issue.
                                </p>
                            </main>
                        </div>

                    </div>
                </main>
            </div>
        </>
    );
}
