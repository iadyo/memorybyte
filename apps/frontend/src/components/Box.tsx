import clsx from "clsx";

function Box({
  children,
  className,
}: {
  className: string;
  children: React.ReactNode;
}) {
  const baseStyles = "mt-10 p-5 bg-white rounded-xl";
  return <section className={clsx(baseStyles, className)}>{children}</section>;
}

export default Box;
