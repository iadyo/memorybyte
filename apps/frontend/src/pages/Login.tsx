import axios from "axios";
import { useEffect, useState } from "react";

function Login() {
  const [data, setData] = useState(null);

  useEffect(() => {
    axios
      .get("http://127.0.0.1:8000/api/categories")
      .then((response) => {
        setData(response.data);
      })
      .catch((error) => {
        console.error("Error fetching data: ", error);
      });
  }, []);
  return (
    <>
      <p>{data}</p>
    </>
  );
}

export default Login;
