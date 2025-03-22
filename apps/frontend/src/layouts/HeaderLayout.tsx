import { useNavigate } from "react-router";
import Button from "../components/Button";
import Input from "../components/Input";
import { MainLabel } from "../components/Label";
import { FiPlus } from "react-icons/fi";

export default function Header() {
  const navigate = useNavigate();

  return (
    <header className="flex items-center justify-between">
      <MainLabel text="MB" className="block md:hidden" to="/" />
      <MainLabel text="MemoryByte" className="hidden md:block" to="/" />
      <Input placeholder="Wyszukaj" className="hidden md:block" />

      <div className="flex items-center gap-10">
        <Button>
          Stwórz <FiPlus className="inline" />
        </Button>
        <Button color="purple" onClick={() => navigate("/login")}>
          Zaloguj się
        </Button>
      </div>
    </header>
  );
}
