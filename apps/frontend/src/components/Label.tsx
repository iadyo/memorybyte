import clsx from "clsx";
import { NavLink } from "react-router";

function MainLabel({
  text,
  className,
  ...props
}: { text: string; className?: string } & React.ComponentProps<
  typeof NavLink
>) {
  const baseStyles = "text-2xl font-bold";

  return (
    <NavLink className={clsx(baseStyles, className)} {...props}>
      {text}
    </NavLink>
  );
}

export { MainLabel };
