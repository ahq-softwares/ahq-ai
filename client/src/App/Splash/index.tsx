import icon from "@/assets/AHQ Progress Logo.png";

export default function Splash() {
  return <div className="w-screen h-screen flex flex-col justify-center text-center items-center">
    <img src={icon} className="w-[40%] md:w-[20%] md:max-w-[200px]" />

    <span className="mt-10 dui-loading-spinner dui-loading dui-loading-xl md:!w-[calc(var(--size-selector,0.25rem)*10)]" />
  </div>;
}