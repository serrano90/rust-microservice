/**
 * Admin Layout
 */
import React from "react"
import Navbar from "components/Navbar"
import Sidebar from "components/Sidebar"
import Footer from "components/Footer"

export default function Layout({children}) {
	return (
		<>
			<div>
				<div className="container-fluid bg-soft">
					<div className="row">
						<div className="col-12">
							<Sidebar />
							<main className="content">
								<Navbar />
								{children}
								<Footer />
							</main>
						</div>
					</div>
				</div>
			</div>
		</>
	)
}
