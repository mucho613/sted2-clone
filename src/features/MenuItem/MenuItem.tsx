type Props = {
  label: string;
  isSelected?: boolean;
  onClick?: () => void;
};

function MenuItem(props: Props) {
  const { label, isSelected, onClick } = props;

  return (
    <button
      className={`
        pl-px text-left h-[17px] leading-none
        ${isSelected ? "bg-sted-white text-sted-black" : "bg-sted-blue text-sted-white"}
      `}
      type="button"
      onClick={onClick}
    >
      {label}
    </button>
  );
}

export default MenuItem;
