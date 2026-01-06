import axios from "axios";

export const getCards = async (id: number) => {
    return axios({
        method: "get",
        url: "/api/print/" + id
    })
    .then((res) => {
        return res.data;
    })
}

export const getBoards = async () => {
    return axios({
        method: "get",
        url: "/api/boards"
    })
    .then((res) => {
        return res.data
    })
}