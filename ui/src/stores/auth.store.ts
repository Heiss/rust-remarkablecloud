import axios from "axios";
import { defineStore } from "pinia";

import router from "~/router";
import { ElMessage } from "element-plus";

const localUser = localStorage.getItem("user");
let user: object | null = null;
if (localUser !== null) {
  user = JSON.parse(localUser);
}

export const useAuthStore = defineStore({
  id: "auth",
  state: () => ({
    // initialize state from local storage to enable user to stay logged in
    user,
    returnUrl: "",
  }),
  actions: {
    login(code: string) {
      axios
        .post(`/login`, {
          code,
        })
        .then((user) => {
          // update pinia state
          this.user = user;
          // store user details and jwt in local storage to keep user logged in between page refreshes
          localStorage.setItem("user", JSON.stringify(user));

          ElMessage.error("Your code was valid. You are logged in now.");
          // redirect to previous url or default to home page
          router.push(this.returnUrl || "/");
          return true;
        })
        .catch((res) => {
          ElMessage.error("Login failed. Your code was invalid.");
          return false;
        });
    },
    logout() {
      this.user = null;
      localStorage.removeItem("user");
      router.push("/login");
    },
  },
});
