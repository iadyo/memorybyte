import { useNavigate } from "react-router";
import Button from "../components/Button";
import Input from "../components/Input";
import { MainLabel } from "../components/Label";
import { FiPlus, FiSearch } from "react-icons/fi";
import { FaMoon, FaSun } from "react-icons/fa";
import useDarkMode from "../utils";

export default function Header() {
  const navigate = useNavigate();
  const [darkMode, toggleDarkMode] = useDarkMode();

  return (
    <header className="flex items-center justify-between">
      <MainLabel text="MB" className="block lg:hidden dark:text-white" to="/" />
      <MainLabel
        text="MemoryByte"
        className="hidden lg:block dark:text-white"
        to="/"
      />

      <div className="hidden lg:relative lg:flex lg:items-center">
        <Input placeholder="Szukaj fiszek" className="block w-xl" id="search" />
        <Button
          color="black"
          className="absolute h-full right-0"
          onClick={() => alert("you are nigger")}
        >
          <FiSearch />
        </Button>
      </div>

      <nav className="flex items-center gap-7">
        <Button onClick={() => toggleDarkMode((prev) => !prev)}>
          {darkMode ? (
            <FaSun className="text-xl" />
          ) : (
            <FaMoon className="text-xl" />
          )}
        </Button>
        <Button onClick={() => navigate("/create")}>
          Stwórz <FiPlus className="text-xl inline" />
        </Button>
        <Button color="purple" onClick={() => navigate("/login")}>
          Zaloguj się
        </Button>
      </nav>
    </header>
  );
}
