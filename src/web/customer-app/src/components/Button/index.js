import React from "react";
import PropType from "prop-types";

const Button = ({ text, variant, type, size }) => {
  return (
    <>
      <button className={`btn btn-${variant} btn-${size}`} type={type}>
        {text}
      </button>
    </>
  );
};

Button.prototype = {
  text: PropType.string.isRequired,
  variant: PropType.string.isRequired,
  type: PropType.string.isRequired,
  size: PropType.string,
};

export default Button;
