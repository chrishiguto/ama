import { ButtonHTMLAttributes, PropsWithChildren, ReactNode } from "react";
import cn from "classnames";
import { LoaderCircle } from "lucide-react";

type ButtonVariant = "orange" | "zinc";

type ButtonProps = {
  variant?: ButtonVariant;
  loading?: boolean;
  endIcon?: ReactNode;
} & PropsWithChildren<ButtonHTMLAttributes<HTMLButtonElement>>;

const Button = ({
  variant = "orange",
  loading = false,
  endIcon,
  children,
  className,
  ...props
}: ButtonProps) => {
  return (
    <button
      className={cn(
        "inline-flex items-center justify-center gap-x-1.5 transition-colors focus-within:outline-none focus-within:ring-2 focus-within:ring-zinc-600 focus-within:ring-offset-2 ring-offset-zinc-950 w-fit-content inline-flex py-1.5 px-3 rounded-lg text-sm font-medium",
        className,
        {
          "hover:bg-orange-500 bg-orange-400 text-orange-950":
            variant === "orange",
          "hover:bg-zinc-900 bg-zinc-800 text-zinc-400": variant === "zinc",
          "opacity-75": loading || props.disabled,
          "cursor-not-allowed": loading || props.disabled,
        },
      )}
      {...props}
    >
      {children}
      {endIcon && !loading ? (
        <span>{endIcon}</span>
      ) : (
        <span>
          <LoaderCircle className="animate-spin" size={16} />
        </span>
      )}
    </button>
  );
};

export default Button;
