import React from "react"

const Showing = ({initialRecords, finishRecords, totalRecords}) => (
	<div className="font-weight-bold small">
		Showing{" "}
		<b>
			{initialRecords} {" to "}
			{finishRecords > totalRecords ? totalRecords : finishRecords}
		</b>
		{" of "}
		<b>{totalRecords}</b> entries
	</div>
)

export default Showing
