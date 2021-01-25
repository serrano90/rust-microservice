import React from "react"
import CustomerReducer from "../reducers/CustomerReducer"

// Initial State
const initialState = {
	name: ""
}

// Create Context
export const CustomerContext = React.createContext()

// Create Provider
export function CustomerProvider({children}) {
	const [state, dispatch] = React.useReducer(CustomerReducer, initialState)

	return (
		<CustomerContext.Provider value={{state: state, dispatch: dispatch}}>
			{children}
		</CustomerContext.Provider>
	)
}
