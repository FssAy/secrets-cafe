import Latte from "../assets/Latte.svg";
("use client");

import * as React from "react";

import { cn } from "@/lib/utils";
import {
  NavigationMenu,
  NavigationMenuContent,
  NavigationMenuItem,
  NavigationMenuLink,
  NavigationMenuList,
  NavigationMenuTrigger,
  navigationMenuTriggerStyle,
} from "@/components/ui/navigation-menu";

import IntrovertCat from "@/assets/github-mark-white.svg";

const components: {
  title: string;
  href: string;
  description: string;
  visual: string;
}[] = [
  {
    title: "About us",
    href: "/about-us",
    description:
      "A place where you can share your secrets, stories, and anything you wish to.",
    visual: "",
  },
  {
    title: "Donate",
    href: "/donate",
    description:
      "Your donation will help us to keep the service running and improve it.",
    visual: "",
  },
  {
    title: "Develop",
    href: "/develop",
    description: "Check out the API documentation.",
    visual: "",
  },
  {
    title: "Contribute",
    href: "/contribute",
    description: "secrets.cafe is open-source. Contribute to the project.",
    visual: IntrovertCat,
  },
];

export default function Navbar() {
  return (
    <>
      <div className="flex items-center justify-around p-4 bg-main">
        <a href="/" className="flex items-center space-x-2">
          <img src={Latte} alt="Secrets Cafe" className="w-14 h-14" />
          <span className="text-5xl font-amatic font-medium">secrets.cafe</span>
        </a>

        <NavigationMenu>
          <NavigationMenuList>
            <NavigationMenuItem>
              <NavigationMenuTrigger>Getting started</NavigationMenuTrigger>
              <NavigationMenuContent>
                <ul className="grid gap-3 p-4 md:w-[400px] lg:w-[500px] lg:grid-cols-[.75fr_1fr]">
                  <li className="row-span-3">
                    <NavigationMenuLink asChild>
                      <a
                        className="flex h-full w-full select-none flex-col justify-end rounded-md bg-gradient-to-b from-muted/50 to-muted p-6 no-underline outline-none focus:shadow-md"
                        href="/"
                      >
                        <div className="mb-2 mt-4 text-6xl font-medium mx-auto font-amatic">
                          secrets.cafe
                          <img
                            src={Latte}
                            alt="Secrets Cafe"
                            className="w-16 h-16 justify-center mx-auto mt-4 mb-2"
                          />
                        </div>
                        <p className="text-sm mx-auto text-center">
                          A place where you can share your secrets...
                        </p>
                      </a>
                    </NavigationMenuLink>
                  </li>
                  <ListItem href="/docs" title="Write">
                    Share your story, secret, or anything you wish to.
                  </ListItem>
                  <ListItem href="/docs/installation" title="Browse">
                    Read stories posted by other people.
                  </ListItem>
                  <ListItem href="/docs/primitives/typography" title="Discover">
                    Still under construction.
                  </ListItem>
                </ul>
              </NavigationMenuContent>
            </NavigationMenuItem>
            <NavigationMenuItem>
              <NavigationMenuTrigger>About</NavigationMenuTrigger>
              <NavigationMenuContent>
                <ul className="grid w-[400px] gap-3 p-4 md:w-[500px] md:grid-cols-2 lg:w-[600px] ">
                  {components.map((component) => (
                    <ListItem
                      key={component.title}
                      title={component.title}
                      href={component.href}
                    >
                      {component.description}
                    </ListItem>
                  ))}
                </ul>
              </NavigationMenuContent>
            </NavigationMenuItem>
            <NavigationMenuItem></NavigationMenuItem>
          </NavigationMenuList>
        </NavigationMenu>
      </div>
    </>
  );
}

const ListItem = React.forwardRef<
  React.ElementRef<"a">,
  React.ComponentPropsWithoutRef<"a">
>(({ className, title, children, ...props }, ref) => {
  return (
    <li>
      <NavigationMenuLink asChild>
        <a
          ref={ref}
          className={cn(
            "block select-none space-y-1 rounded-md p-3 leading-none no-underline outline-none transition-colors hover:bg-accent hover:text-accent-foreground focus:bg-accent focus:text-accent-foreground",
            className
          )}
          {...props}
        >
          <div className="text-sm font-medium leading-none">{title}</div>
          <p className="line-clamp-2 text-sm leading-snug text-muted-foreground">
            {children}
          </p>
        </a>
      </NavigationMenuLink>
    </li>
  );
});
ListItem.displayName = "ListItem";
