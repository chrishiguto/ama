import { PropsWithChildren } from "react";
import cn from "classnames";

type ContainerSize = "small" | "medium" | "large" | "xlarge";

type ContainerProps = {
  size?: ContainerSize;
};

const Container = ({
  children,
  size = "small",
}: PropsWithChildren<ContainerProps>) => {
  return (
    <div
      className={cn(
        "flex items-center justify-center min-h-screen mx-auto p-4 md:p-0",
        {
          "max-w-sm": size === "small",
          "max-w-md": size === "medium",
          "max-w-lg": size === "large",
          "max-w-2xl": size === "xlarge",
        },
      )}
    >
      {children}
    </div>
  );
};

export default Container;
