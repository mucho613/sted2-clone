type Props = {
  label: string;
  state: "normal" | "selected" | "active";
  onClick?: () => void;
};

function MenuItem(props: Props) {
  const { label, state, onClick } = props;

  return (
    <button
      className={`
        pl-px pt-px text-left h-[17px] leading-none
        ${state === "active" ?
          "bg-sted-cyan text-sted-black" :
        state === "selected" ?
          "bg-sted-white text-sted-black" :
          "bg-sted-blue text-sted-white"}
      `}
      type="button"
      onClick={onClick}
    >
      {label}
    </button>
  );
}

export default MenuItem;
