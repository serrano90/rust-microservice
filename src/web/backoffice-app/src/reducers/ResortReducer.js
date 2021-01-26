/**
 * ResortState
 */
export default (state, action) => {
	switch (action.type) {
		case "FETCH_SUCCESS":
			return {
				loading: false,
				error: "",
				reducers: action.payload
			}
		case "FETCH_ERROR":
			return {
				loading: false,
				error: "Does not possible load resorts",
				reducers: [],
				page: 1
			}
		case "CHANGE_PAGE": {
			return {
				page: action.payload,
				...state
			}
		}
		case "ADD_RESORT": {
			return {
				loading: true,
				error: '',
				...state
			}
		}
		case "REMOVE_RESORT": {
			return {
				loading: true,
				error: '',
				...state
			}
		}
		case "UPDATE_RESORT": {
			return {
				loading: true,
				error: '',
				...state
			}
		}
		default:
			return state
	}
}
