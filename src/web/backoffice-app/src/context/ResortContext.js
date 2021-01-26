import React from "react"
import ResortReducer from "../reducers/ResortReducer"

// Initial State
const initialState = {
	loading: true,
	error: "",
	resorts: [],
	page: 1,
	idSelected: 0,
}

// Create Context
export const ResortContext = React.createContext()

// Create Provider
export function ResortProvider({children}) {
	const [state, dispatch] = React.useReducer(ResortReducer, initialState)

	return (
		<ResortContext.Provider
			value={state}
		>
			{children}
		</ResortContext.Provider>
	)
}
