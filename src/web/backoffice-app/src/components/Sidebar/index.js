/**
 * Sidebar
 */

import React from "react"
import routers from "../../utils/routers"
import NavItem from "./NavItem"

const menu = [
	{
		name: "Dashboard",
		url: routers.DASHBOARD.path,
		icon: "chart-pie"
	},
	{
		name: "Resorts",
		url: routers.RESORT.path,
		icon: "users"
	},
	{
		name: "Customers",
		url: routers.CUSTOMER.path,
		icon: "users"
	}
]

function Sidebar() {
	return (
		<>
			<nav
				id="sidebarMenu"
				className="sidebar d-md-block bg-primary text-white collapse"
				data-simplebar
			>
				<div className="sidebar-inner px-4 pt-3">
					<ul className="nav flex-column">
						{menu.map((item, idx) => (
							<NavItem
								title={item.name}
                                url={item.url}
                                icon={item.icon}
								active={false}
								key={idx}
							/>
						))}
					</ul>
				</div>
			</nav>
		</>
	)
}

export default Sidebar
