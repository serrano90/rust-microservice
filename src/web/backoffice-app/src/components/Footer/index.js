/**
 * Footer
 */
import React from "react"

export default function () {
	return (
		<>
			<footer className="py-5 footer section">
				<div className="row">
					<div className="mb-4 col-12 col-lg-6 mb-lg-0">
						<p className="mb-0 text-center text-xl-left">
							Copyright © <span className="current-year">{new Date().getFullYear()}</span>{" "}
						</p>
					</div>
				</div>
			</footer>
		</>
	)
}
