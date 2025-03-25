import {
  FaDiscord,
  FaGithub,
  FaInstagram,
  FaYoutube,
  FaMoon,
  FaSun,
} from "react-icons/fa";
import { useNavigate } from "react-router";
import useDarkMode from "../utils";

export default function Footer() {
  const [darkMode, toggleDarkMode] = useDarkMode();
  const navigate = useNavigate();

  const liStyle = "hover:text-gray-900 dark:hover:text-gray-200";

  return (
    <footer className="flex flex-wrap bg-white dark:bg-gray-700 dark:text-white p-13">
      <div className="w-full lg:w-1/2">
        <h2 className="font-bold text-xl">MemoryByte</h2>
        <p className="text-gray-600 dark:text-gray-300 pt-3">
          Ucz się z fiszek, testów i innych aktywności.
        </p>

        <div className="flex space-x-3 mt-3 text-xl">
          <a href="#" target="_blank" rel="noreferrer" aria-label="Zobacz nasze filmiki na YouTube">
            <FaYoutube />
          </a>
          <a href="#" target="_blank" rel="noreferrer" aria-label="Porozmawiaj z ludźmi">
            <FaDiscord />
          </a>
          <a href="#" target="_blank" rel="noreferrer" aria-label="Zobacz nasze zdjęcia na IG">
            <FaInstagram />
          </a>
          <a href="#" target="_blank" rel="noreferrer" aria-label="Spójrz na nasz projekt!">
            <FaGithub />
          </a>
        </div>
      </div>

      <div className="w-full lg:w-1/2 flex flex-wrap lg:flex-nowrap justify-evenly text-sm pt-10 lg:pt-0">
        <div className="w-full lg:w-1/3 space-y-3">
          <h2 className="font-semibold uppercase tracking-wider">Nawigacja</h2>
          <ul className="text-gray-600 dark:text-gray-300 cursor-pointer space-y-2">
            <li className={liStyle} onClick={() => navigate("#")}>Strona główna</li>
            <li className={liStyle} onClick={() => navigate("#")}>Fiszki</li>
            <li className={liStyle} onClick={() => navigate("#")}>Testy</li>
          </ul>
        </div>
        <div className="w-full lg:w-1/3 space-y-3 mt-5 lg:mt-0">
          <h2 className="font-semibold uppercase tracking-wider">Informacje</h2>
          <ul className="text-gray-600 dark:text-gray-300 cursor-pointer space-y-2">
            <li className={liStyle} onClick={() => navigate("#")}>Polityka prywatności</li>
            <li className={liStyle} onClick={() => navigate("#")}>Kontakt</li>
          </ul>
        </div>
        <div className="w-full lg:w-1/3 space-y-3 mt-5 lg:mt-0">
          <h2 className="font-semibold uppercase tracking-wider">Konto</h2>
          <ul className="text-gray-600 dark:text-gray-300 cursor-pointer space-y-2">
            <li className={liStyle} onClick={() => navigate("#")}>Logowanie</li>
            <li className={liStyle} onClick={() => navigate("#")}>Rejestracja</li>
          </ul>
        </div>
      </div>
      <hr className="w-full border-t border-gray-200 dark:border-gray-600 my-10" />
      <div className="w-full flex-col lg:flex-row flex items-center lg:justify-between dark:text-gray-300 text-gray-600">
        <span className="text-sm">
          &copy; 2025 MemoryByte. Wszelkie prawa zastrzeżone.
        </span>
        <button aria-label="Enable or disable dark mode" onClick={() => toggleDarkMode((prev) => !prev)}>
          {darkMode ? (
            <FaSun className="cursor-pointer" />
          ) : (
            <FaMoon className="cursor-pointer" />
          )}
        </button>
      </div>
    </footer>
  );
}
