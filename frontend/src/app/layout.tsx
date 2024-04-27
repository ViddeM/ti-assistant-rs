import type { Metadata } from "next";
import "../resources/styles/globals.scss";
import "../resources/styles/vars.scss";
import localFont from "next/font/local";

const sliderFont = localFont({
  src: "../resources/fonts/slider/slider_regular.ttf",
  display: "swap",
});

export const metadata: Metadata = {
  title: "TI Helper",
  description: "A utility website for Twilight Imperium 4th ed",
};

export default function RootLayout({
  children,
}: {
  children: React.ReactNode;
}) {
  return (
    <html lang="en" className={sliderFont.className}>
      <body>
        <main className="main">{children}</main>
      </body>
    </html>
  );
}
