import axios from "axios";

export const getCards = async () => {
    return axios({
        method: "get",
        url: "/api/print"
    })
    .then((res) => {
        return res.data;
    })
}