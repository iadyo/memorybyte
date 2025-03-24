import { FaDiscord, FaGithub, FaInstagram, FaYoutube } from "react-icons/fa";
import { useNavigate } from "react-router";

export default function Footer() {
  const navigate = useNavigate();

  return (
    <footer className="flex flex-wrap bg-white dark:bg-gray-700 dark:text-white p-13">
      <div className="w-full lg:w-1/2">
        <h2 className="font-bold text-xl">MemoryByte</h2>
        <p>&copy;2025 MemoryByte</p>

        <div className="flex space-x-3 mt-3">
          <a href="#" target="_blank" rel="noreferrer">
            <FaYoutube />
          </a>
          <a href="#" target="_blank" rel="noreferrer">
            <FaDiscord />
          </a>
          <a href="#" target="_blank" rel="noreferrer">
            <FaInstagram />
          </a>
          <a href="#" target="_blank" rel="noreferrer">
            <FaGithub />
          </a>
        </div>

        <hr className="w-full border-t border-gray-200 dark:border-gray-600 my-5" />
      </div>

      <div className="w-full lg:w-1/2 flex flex-wrap lg:flex-nowrap justify-evenly lg:text-right">
        <div className="w-full lg:w-1/4 space-y-3">
          <h2 className="font-bold text-2xl">Nawigacja</h2>
          <ul className="text-gray-600 dark:text-gray-300 cursor-pointer">
            <li onClick={() => navigate("#")}>Lorem ipsum1</li>
            <li onClick={() => navigate("#")}>Lorem ipsum2</li>
            <li onClick={() => navigate("#")}>Lorem ipsum3</li>
            <li onClick={() => navigate("#")}>Lorem ipsum4</li>
          </ul>
        </div>
        <div className="w-full lg:w-1/4 space-y-3 mt-5 lg:mt-0">
          <h2 className="font-bold text-2xl">Inne gówna</h2>
          <ul className="text-gray-600 dark:text-gray-300 cursor-pointer">
            <li onClick={() => navigate("#")}>Lorem ipsum1</li>
            <li onClick={() => navigate("#")}>Lorem ipsum2</li>
            <li onClick={() => navigate("#")}>Lorem ipsum3</li>
          </ul>
        </div>
      </div>
    </footer>
  );
}
