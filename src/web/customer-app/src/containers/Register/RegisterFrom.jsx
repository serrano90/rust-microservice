import React from "react"
import {useFormik} from "formik"
import {RegisterCustomer} from "../../utils/validate"
import Button from "../../components/Button"
import PropType from "prop-types"

const RegisterForm = ({submitHandle, resorts}) => {
	const formik = useFormik({
		initialValues: {
			firstName: "",
			lastName: "",
			email: "",
			resortId: 0
		},
		validationSchema: RegisterCustomer,
		onSubmit: submitHandle
	})

	return (
		<form onSubmit={formik.handleSubmit}>
			<div className="form-group mb-4">
				<label htmlFor="firstName">First Name</label>
				<div className="input-group">
					<input
						id="firstName"
						name="firstName"
						placeholder="John"
						className={`form-control ${
							formik.touched.firstName && formik.errors.firstName
								? "is-invalid"
								: ""
						}`}
						type="text"
						onChange={formik.handleChange}
						value={formik.values.firstName}
					/>
					{formik.touched.firstName && formik.errors.firstName ? (
						<div className="invalid-feedback">{formik.errors.firstName}</div>
					) : null}
				</div>
			</div>

			<div className="form-group mb-4">
				<label htmlFor="lastName">Last Name</label>
				<div className="input-group">
					<input
						id="lastName"
						name="lastName"
						placeholder="Nash"
						className={`form-control ${
							formik.touched.lastName && formik.errors.lastName
								? "is-invalid"
								: ""
						}`}
						type="text"
						onChange={formik.handleChange}
						value={formik.values.lastName}
					/>
					{formik.touched.lastName && formik.errors.lastName ? (
						<div className="invalid-feedback">{formik.errors.lastName}</div>
					) : null}
				</div>
			</div>

			<div className="form-group mb-4">
				<label htmlFor="email">Email Address</label>
				<div className="input-group">
					<input
						id="email"
						name="email"
						placeholder="test@example.com"
						className={`form-control ${
							formik.touched.email && formik.errors.email ? "is-invalid" : ""
						}`}
						type="text"
						onChange={formik.handleChange}
						value={formik.values.email}
					/>
					{formik.touched.email && formik.errors.email ? (
						<div className="invalid-feedback">{formik.errors.email}</div>
					) : null}
				</div>
			</div>

			<div className="form-group mb-4">
				<label htmlFor="resortId">Resort</label>
				<div className="input-group">
					<select
						id="resortId"
						name="resortId"
						className={`form-control ${
							formik.touched.resortId && formik.errors.resortId
								? "is-invalid"
								: ""
						}`}
						placeholder="Seleccione un Plan"
						onChange={formik.handleChange}
						value={formik.values.resortId}
					>
						<option value="">Select Resort... </option>
						{resorts.map((item) => {
							return <option key={item.id} value={item.id}>{item.name}</option>
						})}
					</select>
					{formik.touched.resortId && formik.errors.resortId ? (
						<div className="invalid-feedback">{formik.errors.resortId}</div>
					) : null}
				</div>
			</div>

			<Button variant="info" type="submit" text="Register" disabled={formik.isSubmitting || !formik.isValid}/>
		</form>
	)
}

RegisterForm.prototype = {
	submitHandle: PropType.func.isRequired,
	resorts: PropType.array.isRequired
}

export default RegisterForm
