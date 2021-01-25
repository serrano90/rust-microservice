import React from "react"
import { Link, useLocation } from 'react-router-dom';
import qs from "query-string"

const useQuery = () => {
    let location = useLocation()
    return React.useMemo(() => qs.parse(location.search), [location.search])
}
export default function ThankYouPage() {
    const {name} = useQuery()
    
	return (
		<>
			<div className="container-fluid mt-8 mb-4">
				<div className="row">
					<div className="d-flex justify-content-center">
						<div className="col-lg-8 col-md-10 col-xs-12 mb-4 mt-3 text-center">
							<h2 className="mt-5">
                                Thank you, {name}!
							</h2>
                            <Link to="/" className="btn btn btn-outline-info">Register</Link>
						</div>
					</div>
				</div>
			</div>
		</>
	)
}
