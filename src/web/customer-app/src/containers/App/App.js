import React from "react";
import { Routes, Route, Link } from "react-router-dom";
// Page
import RegisterPage from "../Register";
import ThankYouPage from "../ThankYou";

function App() {
  return (
    <main>
      <Routes>
        <Route path="/" element={<RegisterPage />} />
        <Route path="thankyou" element={<ThankYouPage />} />
      </Routes>
    </main>
  );
}

export default App;
