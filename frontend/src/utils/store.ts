import { defineStore } from "pinia";
import axios from "axios";

interface User {
    name: string,
    role: "editor" | "admin"
}

interface AuthState {
    user: User | null,
    token: string | null
}

export const useAuthStore = defineStore('auth', {
    state: (): AuthState => ({
        user: null,
        token: null
    }),
    getters: {
        isLoggedIn: (state): boolean => !!state.token,
        isAdmin: (state): boolean => state.user?.role === "admin"
    },
    actions: {
        async login(username: string, password: string) {
            try {
                await axios({
                    method: "post",
                    url: "/api/auth/login",
                    data: {
                        username: username, 
                        password: password
                    }
                }).then((res) => {
                    if (res.status !== 200) {
                        return;
                    }

                    if (res.status === 200) {
                        this.user = {
                            name: res.data.name,
                            role: res.data.role
                        }
                        this.token = res.data.token
                    }
                })
            } catch (error) {
                console.log(error)
                return error
            }
        },


        async logout() {
            await axios({
                method: "post",
                url: "/api/auth/logout",
            })           
            this.token = null
            this.user = null
        },


        async checkAuth() {
            const token = this.token
            if (token) {
                axios.get("/api/auth/check")
                .then(res => {
                    if (res.data.invalid) {

                    }
                })
            }
        },
        async refreshToken() {

        }
    },
})