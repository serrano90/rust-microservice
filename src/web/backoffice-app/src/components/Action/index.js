import React from "react"
import PropTypes from "prop-types"
import {FontAwesomeIcon} from "@fortawesome/react-fontawesome"

const Action = ({variant, icon, onClick}) => {
	return (
		<FontAwesomeIcon icon={["fas", icon]} className={`text-${variant} mr-2`} onClick={onClick}/>
	)
}

Action.prototype = {
	variant: PropTypes.string.isRequired,
	icon: PropTypes.string.isRequired,
	onClick: PropTypes.func
}

export default Action
