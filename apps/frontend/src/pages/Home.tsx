import Button from "../components/Button";
import Header from "../layouts/HeaderLayout";

function Home() {
  return (
    <>
      <div className="bg-white">
        <div className="hidden lg:flex lg:items-center lg:justify-center lg:bg-purple-500 lg:h-10 lg:w-full lg:text-white">
          <p>Witamy na początku naszej strony! NiggersTeam</p>
        </div>
        <div className="container mx-auto p-5">
          <Header />
        </div>
      </div>
      <main className="flex flex-col items-center justify-center h-80">
        <h2 className="text-3xl text-center lg:text-4xl font-semibold">
          Let's learn english words together!
        </h2>
        <p className="text-center lg:text-lg">
          Opanuj cały materiał, którego chcesz się nauczyć, korzystając z
          interaktywnych fiszek, przykładowych testów i innych aktywności.
        </p>

        <Button color="black" className="mt-5" onClick={() => {}}>Zacznij naukę!</Button>
      </main>
    </>
  );
}

export default Home;
