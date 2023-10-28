import { FC, SelectHTMLAttributes } from "react";

export type DropdownProps = SelectHTMLAttributes<HTMLSelectElement>;

export const Dropdown: FC<DropdownProps> = ({ className, ...props }) => {
  const style = `${className}`;

  return <select className={style} {...props} />;
};
