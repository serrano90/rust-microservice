/**
 * useResort Hooks
 */
import React from "react"
import {ResortContext} from "../context/ResortContext"

export default function useResort() {
	const contextValue = React.useContext(ResortContext)
	return contextValue
}
