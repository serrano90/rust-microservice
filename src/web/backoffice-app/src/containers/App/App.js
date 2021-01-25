import React from "react"
import {Routes, Route} from "react-router-dom"
import Layout from "../../components/Layout"
import routers from "../../utils/routers"
import {ResortProvider} from "../../context/ResortContext"
// Pages
import DashboardPage from "../Dashboard"
import ResortPage from "../Resort"

function App() {
	return (
		<main>
			<Layout>
				<Routes>
					<Route path={routers.DASHBOARD.path} element={<DashboardPage />} />
					<Route path={routers.RESORT.path} element={<ResortPage />} />
				</Routes>
			</Layout>
		</main>
	)
}

export default App
