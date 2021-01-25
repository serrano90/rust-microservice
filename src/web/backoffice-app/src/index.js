import React from "react"
import ReactDOM from "react-dom"
import {BrowserRouter as Router} from "react-router-dom"
import "./index.scss"
import App from "./containers/App/App"
import reportWebVitals from "./reportWebVitals"
import {createBrowserHistory} from "history"

import {library} from "@fortawesome/fontawesome-svg-core"
import {fas} from "@fortawesome/free-solid-svg-icons"
import {fab} from "@fortawesome/free-brands-svg-icons"
import {
	faCheck,
	faCoffee,
	faCreditCard,
	faUsers,
	faSignOutAlt,
	faTimes,
	faPlus,
	faMoneyCheckAlt
} from "@fortawesome/free-solid-svg-icons"

library.add(
	fab,
	fas,
	faCheck,
	faCoffee,
	faCreditCard,
	faUsers,
	faSignOutAlt,
	faTimes,
	faPlus,
	faMoneyCheckAlt
)

const history = createBrowserHistory()

ReactDOM.render(
	<React.StrictMode>
		<Router history={history}>
			<App />
		</Router>
	</React.StrictMode>,
	document.getElementById("root")
)

// If you want to start measuring performance in your app, pass a function
// to log results (for example: reportWebVitals(console.log))
// or send to an analytics endpoint. Learn more: https://bit.ly/CRA-vitals
reportWebVitals()
