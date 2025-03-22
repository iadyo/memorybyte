import clsx from "clsx";

function Input({
  className,
  ...props
}: { className?: string } & React.ComponentProps<"input">) {
  const baseStyles = "rounded-3xl bg-gray-100 p-2 w-2xl px-4";

  return <input className={clsx(baseStyles, className)} {...props} />;
}

export default Input;
