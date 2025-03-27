import { FiArrowRight } from "react-icons/fi";
import { motion } from "framer-motion";
import Button from "../components/Button";
import Header from "../layouts/HeaderLayout";
import Footer from "../layouts/FooterLayout";

function Home() {
  return (
    <>
      <div className="bg-white dark:bg-gray-700">
        <div className="hidden lg:flex lg:items-center lg:justify-center lg:bg-purple-500 lg:h-10 lg:w-full lg:text-white">
          <p>Witamy na początku naszej strony!</p>
        </div>
        <div className="container mx-auto p-5">
          <Header />
        </div>
      </div>
      <motion.main
        className="flex flex-col items-center justify-center h-80 dark:bg-gray-600 dark:text-white"
        initial={{ opacity: 0, y: -20 }}
        animate={{ opacity: 1, y: 0 }}
        transition={{ duration: 0.5 }}
      >
        <h2 className="text-3xl text-center lg:text-4xl font-semibold">
          Let's learn english words together!
        </h2>
        <p className="text-center lg:text-lg">
          Opanuj cały materiał, którego chcesz się nauczyć, korzystając z
          interaktywnych fiszek, przykładowych testów i innych aktywności.
        </p>

        <Button color="black" className="mt-5" onClick={() => {}}>
          Zacznij naukę
          <FiArrowRight className="text-xl inline text-gray-300" />
        </Button>
      </motion.main>

      <Footer />
    </>
  );
}

export default Home;
