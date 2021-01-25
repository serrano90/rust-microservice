import React from "react"
import {Routes, Route} from "react-router-dom"
import {CustomerProvider} from "../../context/CustomerContext"
// Page
import RegisterPage from "../Register"
import ThankYouPage from "../ThankYou"

function App() {
	return (
		<main>
			<CustomerProvider>
				<Routes>
					<Route path="/" element={<RegisterPage />} />
					<Route path="thankyou" element={<ThankYouPage />} />
				</Routes>
			</CustomerProvider>
		</main>
	)
}

export default App
