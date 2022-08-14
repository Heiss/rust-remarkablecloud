import axios from "axios";
import { defineStore } from "pinia";

import router from "~/router";
import { ElMessage } from "element-plus";
import { User } from "~/models";

let user: User | null = null;
const localUser = localStorage.getItem("user");
if (localUser !== null) {
  user = User.fromJSON(localUser);
}

const authenticated: boolean = false;

export const useAuthStore = defineStore({
  id: "auth",
  state: () => ({
    // initialize state from local storage to enable user to stay logged in
    user,
    authenticated,
    returnUrl: "",
  }),
  actions: {
    authenticated() {
      return this.user !== null;
    },
    login(code: string) {
      axios
        .post(`/login`, {
          code,
        })
        .then((user) => {
          // update pinia state
          this.user = User.fromJSON(user.data);
          // store user details and jwt in local storage to keep user logged in between page refreshes
          localStorage.setItem("user", user.data);

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
