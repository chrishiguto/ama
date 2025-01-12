import { forwardRef, InputHTMLAttributes, PropsWithChildren } from "react";
import cn from "classnames";

type TextInputProps = {
  error?: boolean;
  errorText?: string | null;
} & InputHTMLAttributes<HTMLInputElement>;

const TextInput = forwardRef<
  HTMLInputElement,
  PropsWithChildren<TextInputProps>
>(({ children, placeholder, error, errorText, ...props }, ref) => {
  return (
    <div className="text-left">
      <div
        className={cn(
          "flex items-center h-fit ring-offset-zinc-950 transition duration-200 ease-in-out rounded-xl w-full bg-zinc-900 border border-zinc-800",
          {
            "focus-within:outline-none focus-within:ring-2 focus-within:ring-orange-500 focus-within:ring-offset-2":
              !error,
            "ring-2 ring-offset-2 ring-red-500": !!error,
          }
        )}
      >
        <input
          ref={ref}
          placeholder={placeholder}
          className="bg-transparent border-none outline-none placeholder:text-zinc-500 text-sm w-full py-3 px-5"
          {...props}
        />

        <div className="shrink-0 inline-flex items-center pr-2">{children}</div>
      </div>
      {errorText && (
        <p className="mt-3 ml-2 text-red-500 text-sm">{errorText}</p>
      )}
    </div>
  );
});

export default TextInput;
