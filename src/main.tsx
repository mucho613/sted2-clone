import React from "react";
import ReactDOM from "react-dom/client";
import { BrowserRouter, Route, Routes } from "react-router";
import "./input.css";
import MainScreen from "./pages/MainScreen";

ReactDOM.createRoot(document.getElementById("root") as HTMLElement).render(
  <React.StrictMode>
    <BrowserRouter>
      <Routes>
        <Route index element={<MainScreen />} />
      </Routes>
    </BrowserRouter>
  </React.StrictMode>,
);
