import React from "react";
import PropType from "prop-types";

const Alert = ({ variant, message }) => {
  return (
    <div className={`alert alert-${variant}`} role="alert">
      <div className="text-white">{message}</div>
    </div>
  );
}

Alert.prototype = {
    variant: PropType.string.isRequired,
    message: PropType.string.isRequired
}

export default Alert
