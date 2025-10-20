import icon from "@/assets/AHQ AI.png";

export default function Splash() {
  return <div className="w-screen h-screen flex flex-col justify-center text-center items-center">
    <img src={icon} />

    <span className="mt-5 dui-loading-xl dui-loading-spinner dui-loading" />
  </div>;
}