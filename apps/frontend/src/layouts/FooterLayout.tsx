import { useNavigate } from "react-router";

export default function Footer() {
  const navigate = useNavigate();

  const sections = [
    {
      title: "Menu",
      links: [
        { name: "Eksploruj", path: "/explore" },
        { name: "FAQ", path: "/faq" },
        { name: "Wsparcie", path: "/support" },
        { name: "Regulamin", path: "/rules" },
      ],
    },
    {
      title: "Projekt",
      links: [
        { name: "Współpraca", path: "/partnership" },
        { name: "Polityka prywatności", path: "/privacy" },
      ],
    },
    {
      title: "MemoryByte",
      links: [
        { name: "Nasza historia", path: "/history" },
        { name: "Nasza misja", path: "/mission" },
      ],
    },
  ];

  return (
    <footer className="flex flex-wrap p-15 bg-white space-x-15 lg:space-x-10 space-y-5 text-sm lg:text-base dark:bg-gray-700 dark:text-white">
      {sections.map((section, index) => (
        <div key={index} className="w-1/4 space-y-2">
          <h2 className="font-bold">{section.title}</h2>
          <ul>
            {section.links.map((link, idx) => (
              <li
                key={idx}
                className="cursor-pointer hover:text-purple-500 transition-colors duration-300"
                onClick={() => navigate(link.path)}
              >
                {link.name}
              </li>
            ))}
          </ul>
        </div>
      ))}
      <hr className="w-full border-t border-gray-200 my-5" />
      <span>&copy;2025 MemoryByte</span>
    </footer>
  );
}
