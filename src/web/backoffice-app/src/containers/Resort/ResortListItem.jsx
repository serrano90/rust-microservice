/**
 * Resort Item
 */
import React from "react"
import PropTypes from "prop-types"
import Button from "../../components/Button"

const ResortListItem = ({id, name, onDeleteHandle}) => {
	return (
		<tr key={id}>
			<td>
				<div className="d-block">
					<span className="font-weight-bold">{id}</span>
				</div>
			</td>
			<td>
				<div className="d-block">
					<span className="font-weight-bold">{name}</span>
				</div>
			</td>
			<td>
				<span className="font-weight-normal">
					<Button 
					text="Delete"
					variant="danger"
					type="button"
					size="sm"
					onClick={() => {onDeleteHandle(id)}} />
				</span>
			</td>
		</tr>
	)
}

ResortListItem.prototype = {
	id: PropTypes.string.isRequired,
	name: PropTypes.string.isRequired,
	onDeleteHandle: PropTypes.func.isRequired
}

export default ResortListItem
