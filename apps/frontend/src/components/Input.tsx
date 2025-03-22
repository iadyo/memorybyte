import clsx from "clsx";

function Input({
  className,
  ...props
}: { className?: string } & React.ComponentProps<"input">) {
  const baseStyles =
    "rounded-2xl bg-gray-100 p-2 px-4 border border-gray-200 focus:outline-none text-gray-500";

  return <input className={clsx(baseStyles, className)} {...props} />;
}

export default Input;
