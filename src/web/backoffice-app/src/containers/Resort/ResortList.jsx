import React from "react"
import PropTypes from "prop-types"
import ResortListItem from "./ResortListItem"
import DataTable from "../../components/Datatable"
import {columnsName} from "./constants"
import {transformPaginations} from "../../utils/datatable"

const ResortList = ({data, onDeleteHandle}) => {
    const [page, setPage] = React.useState(1)
    const resortData = {
        data: data,
		...transformPaginations(data ? data.length : 0, page)
    }

	const showResortList = () => {
		if (!data) {
			return []
		} else {
			const rows = resortData.data.map((item, idx) => {
				const resort = (
					<ResortListItem key={idx} id={item.id} name={item.name} onDeleteHandle={onDeleteHandle}/>
				)
				return resort
			})
			return rows
		}
	}

	const onChangePage = (pageNumber) => {
		setPage(pageNumber)
	}

	return (
		<>
			<DataTable
				headers={columnsName}
				rows={showResortList()}
				totalRecords={resortData ? resortData.totalRecords : 0}
				currentPage={resortData ? resortData.currentPage : 0}
				initialRecords={resortData ? resortData.initialValue : 0}
				finishRecords={resortData ? resortData.finishValue : 0}
				onPageChange={onChangePage}
			/>
		</>
	)
}

ResortList.prototype = {
    resortDate: PropTypes.array.isRequired,
    onDeleteHandle: PropTypes.func,
}

export default ResortList
