import { useNavigate } from "react-router";
import Button from "../components/Button";
import Input from "../components/Input";
import { MainLabel } from "../components/Label";
import { FiPlus } from "react-icons/fi";

export default function Header() {
  const navigate = useNavigate();

  return (
    <header className="flex items-center justify-between">
      <MainLabel text="MB" className="block lg:hidden" to="/" />
      <MainLabel text="MemoryByte" className="hidden lg:block" to="/" />
      <Input placeholder="Wyszukaj" className="hidden lg:block w-xl" />

      <nav className="flex items-center gap-7">
        <Button onClick={() => navigate("/create")}>
          Stwórz <FiPlus className="inline" />
        </Button>
        <Button color="purple" onClick={() => navigate("/login")}>
          Zaloguj się
        </Button>
      </nav>
    </header>
  );
}
