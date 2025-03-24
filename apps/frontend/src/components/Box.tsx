import clsx from "clsx";

function Box({
  children,
  className,
}: {
  className: string;
  children: React.ReactNode;
}) {
  const baseStyles = "p-5 bg-white rounded-xl";
  return <section className={clsx(baseStyles, className)}>{children}</section>;
}

export default Box;
