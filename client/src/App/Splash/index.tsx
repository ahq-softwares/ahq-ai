import icon from "@/assets/AHQ Progress Logo.png";

export default function Splash() {
  return <div className="w-screen h-screen flex flex-col justify-center text-center items-center">
    <img src={icon} className="w-[40%] md:w-[20%]" />

    <span className="mt-5 dui-loading-xl dui-loading-spinner dui-loading" />
  </div>;
}