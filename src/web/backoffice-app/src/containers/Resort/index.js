/**
 * Resort Page
 */
import React from "react"
import {config} from "../../utils/config"
import ResortList from "./ResortList";

export default function ResortPage() {
	const [loading, setLoading] = React.useState(true)
	const [error, setError] = React.useState()
	const [resortData, setResortData] = React.useState(null)
	

	React.useEffect(() => {
		loadResort()
	}, [])

	const loadResort = () => {
		fetch(`${config.api.url}/resorts`, {
			headers: {
				"Content-Type": "application/json"
			}
		})
			.then((resp) => resp.json())
			.then(
				(result) => {
					setLoading(false)
					setResortData(result)
				},
				(error) => {
					setResortData(null)
					setLoading(false)
					setError(error)
				}
			)
			.catch((error) => {
				setResortData(null)
				setLoading(false)
				console.log(error)
			})
	}

	const deleteHandle = (id) => {
		fetch(`${config.api.url}/resorts/${id}`, {
			method: "DELETE",
		})
		.then(
			(resp) => {
				setLoading(true)
				setError()
				loadResort()
			},
			(error) => {
				setLoading(false)
				setError(error)
			}
		)
		.catch((error) => {
			setLoading(false)
			console.log(error)
		})
	}

	return (
		<>
			<div className="flex-wrap py-4 d-flex justify-content-between flex-md-nowrap align-items-center">
				<div className="mb-4 d-block mb-md-0">
					<h2 className="h4">Resort List</h2>
				</div>
			</div>
			<ResortList data={resortData} onDeleteHandle={deleteHandle}/>
		</>
	)
}
