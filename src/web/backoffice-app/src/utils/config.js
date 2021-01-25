/**
 * Config
 */
export const config = {
  api: {
    url: process.env.REACT_APP_SERVICE_BASE_URL,
    timeout: process.env.REACT_APP_SERVICE_TIMEOUT * 60,
  },
};