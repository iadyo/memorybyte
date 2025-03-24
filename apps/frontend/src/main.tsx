import { StrictMode } from "react";
import { createRoot } from "react-dom/client";
import "./index.css";
import { BrowserRouter, Route, Routes } from "react-router";
import Home from "./pages/Home";
import Login from "./pages/Login";
import NoSite from "./pages/NoSite";

createRoot(document.getElementById("main")!).render(
  <StrictMode>
    <BrowserRouter>
      <Routes>
        <Route index element={<Home />} />
        <Route path="/login" element={<Login />} />
        <Route path="*" element={<NoSite />} />
      </Routes>
    </BrowserRouter>
  </StrictMode>,
);
