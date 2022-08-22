import axios from "axios";
import { defineStore } from "pinia";

import router from "~/router";
import { ElMessage } from "element-plus";
import { User } from "~/models";

let user: User | null = null;
let cron: NodeJS.Timer;

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
    cron,
  }),
  actions: {
    authenticated() {
      return this.user !== null;
    },
    login(code: string, email: string) {
      axios
        .post(`/login`, {
          code,
          email,
        })
        .then((user) => {
          // update pinia state
          this.user = User.fromJSON(user.data);
          // store user details and jwt in local storage to keep user logged in between page refreshes
          console.log(this.user);

          localStorage.setItem("user", this.user.serialize());

          ElMessage.success("Your code was valid. You are logged in now.");
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
    check() {
      axios
        .post("/jwt", { jwt: this.user?.jwt })
        .then((user) => {
          this.user = User.fromJSON(user.data);
          localStorage.setItem("user", this.user.serialize());
        })
        .catch(() => {
          if (this.user != null) {
            ElMessage.error(
              "Your login session is not valid anymore. Please login again."
            );
          }
          clearInterval(this.cron);
          this.logout();
          return false;
        });
    },
    start_check() {
      this.check();
      this.cron = setInterval(() => {
        console.log("run");
        this.check();
      }, 5 * 60 * 1000);
    },
  },
});
