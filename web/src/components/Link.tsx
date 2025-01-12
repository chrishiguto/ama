import { AnchorHTMLAttributes } from "react";
import cn from "classnames";
import Slot from "./Slot";

type ButtonVariant = "orange" | "zinc";

type AsChildProps<DefaultElementProps> =
  | ({ asChild?: false } & DefaultElementProps)
  | { asChild: true; children: React.ReactNode };

type LinkProps = AsChildProps<AnchorHTMLAttributes<HTMLAnchorElement>> & {
  variant?: ButtonVariant;
  asChild?: boolean;
};

const Link = ({
  asChild,
  variant = "orange",
  children,
  ...props
}: LinkProps) => {
  const Comp = asChild ? Slot : "a";

  return (
    <Comp
      className={cn(
        "inline-flex items-center justify-center gap-x-1.5 transition-colors w-fit-content inline-flex rounded-lg text-sm font-medium",
        {
          "hover:text-orange-500 text-orange-400": variant === "orange",
          "hover:text-zinc-900 text-zinc-800": variant === "zinc",
        }
      )}
      {...props}
    >
      {children}
    </Comp>
  );
};

export default Link;
