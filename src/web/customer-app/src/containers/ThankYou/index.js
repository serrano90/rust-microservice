import React from "react"
import { Link, useLocation } from 'react-router-dom';
import qs from "query-string"
import {CustomerContext} from "../../context/CustomerContext"

const useQuery = () => {
    let location = useLocation()
    return React.useMemo(() => qs.parse(location.search), [location.search])
}
export default function ThankYouPage() {
	const { state } = React.useContext(CustomerContext);
	console.log(state)
    //const {name} = useQuery()
    
	return (
		<>
			<div className="mt-8 mb-4 container-fluid">
				<div className="row">
					<div className="d-flex justify-content-center">
						<div className="mt-3 mb-4 text-center col-lg-8 col-md-10 col-xs-12">
							<h2 className="mt-5">
                                Thank you, {state.name}!
							</h2>
                            <Link to="/" className="btn btn-outline-info">Register</Link>
						</div>
					</div>
				</div>
			</div>
		</>
	)
}
