import * as Yup from "yup";

export const RegisterCustomer = Yup.object({
  firstName: Yup.string()
    .max(30, "Must be 15 characters or less")
    .required("Required"),
  lastName: Yup.string()
    .max(60, "Must be 20 characters or less")
    .required("Required"),
  email: Yup.string().email("Invalid email address").required("Required"),
  resortId: Yup.number().required("Required")
});
