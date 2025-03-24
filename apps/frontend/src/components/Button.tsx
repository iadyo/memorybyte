import clsx from "clsx";

function Button({
  color = "transparent",
  className,
  children,
  ...props
}: {
  color?: "black" | "purple" | "transparent";
  className?: string;
  children: React.ReactNode;
} & React.ButtonHTMLAttributes<HTMLButtonElement>) {
  const baseStyles = "font-bold rounded-2xl p-2 w-auto px-4 cursor-pointer";
  const colorStyles = {
    black: clsx(
      baseStyles,
      "bg-black text-white hover:bg-gray-800 transition-color duration-300",
    ),
    purple: clsx(
      baseStyles,
      "bg-purple-500 text-white hover:bg-purple-600 transition-color duration-300",
    ),
    transparent:
      "bg-transparent text-purple-500 hover:text-purple-600 transition-color duration-300",
  };

  return (
    <button className={clsx(colorStyles[color], className)} {...props}>
      {children}
    </button>
  );
}

export default Button;
