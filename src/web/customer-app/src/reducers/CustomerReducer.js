/**
 * Customer Reducer
 */
export default (state, action) => {
    switch (action.type) {
        case "ADD_CUSTOMER_NAME":  
            return {
                name: action.payload
            }
        default:
            return state
    }
}