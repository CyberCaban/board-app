interface ButtonProps extends React.ButtonHTMLAttributes<HTMLButtonElement> {
  onClick?: () => void;
}

export default function Button({
  children,
  ...props
}: ButtonProps) {
  return (
    <button
      className="bg-blue-500 hover:bg-blue-700 text-white py-2 px-4 rounded"
      {...props}
    >
      {children}
    </button>
  );
}
