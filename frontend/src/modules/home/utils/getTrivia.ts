import axios from "axios";

export const getTrivia = async () => {
    return axios({
        method: "get",
        url: "/api/trivia"
    })
    .then((res) => {
        return res.data;
    })
}