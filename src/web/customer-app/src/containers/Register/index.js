import React from "react"
import {useNavigate} from "react-router-dom"
import RegisterForm from "./RegisterFrom"
import Container from "../../components/Container"
import Alert from "../../components/Alert"
import {config} from "../../utils/config"
import {CustomerContext} from "../../context/CustomerContext"

export default function RegisterPage() {
	const { dispatch } = React.useContext(CustomerContext);
	let navigate = useNavigate()
	const [loading, setLoading] = React.useState(true)
	const [error, setError] = React.useState()
	const [resortData, setResortData] = React.useState([])

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
					console.log(result)
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

	const submitHandle = (values, {setSubmitting}) => {
		const body = JSON.stringify(
			{
				name: values.firstName,
				last_name: values.lastName,
				email: values.email,
				hotel_id: parseInt(values.resortId)
			},
			null,
			2
		)

		fetch(`${config.api.url}/customers`, {
			method: "POST",
			headers: {
				Accept: "application/json",
				"Content-Type": "application/json"
			},
			body: body
		})
			.then((resp) => resp.json())
			.then(
				(resp) => {
					dispatch({
						type: "ADD_CUSTOMER_NAME",
						payload: resp.first_name
					})
					navigate("/thankyou", {replace: true})
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

		setSubmitting(false)
	}

	return (
		<Container>
			<div className="row justify-content-center">
				<div className="col-12 d-flex align-items-center justify-content-center">
					<div className="p-4 mb-4 bg-white border rounded mb-lg-0 shadow-soft border-light p-lg-5 w-100 fmxw-500">
						<div className="mb-4 text-center text-md-center mt-md-0">
							<h1 className="mb-0 h3">Customer Register</h1>
						</div>
						{error ? <Alert message={error.message} variant="danger" /> : ""}
						{!loading ? (
							<RegisterForm submitHandle={submitHandle} resorts={resortData} />
						) : (
							""
						)}
					</div>
				</div>
			</div>
		</Container>
	)
}
