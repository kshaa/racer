import {createSlice} from "@reduxjs/toolkit";
import {HYDRATE} from "next-redux-wrapper";
import {AppState} from "@/redux/store";
import {AuthState} from "@/domain/auth";
import * as Auth from "@/domain/auth";

// Actual Slice
const initialState: AuthState = Auth.empty
export const authSlice = createSlice({
  name: "auth",
  initialState,
  reducers: {
    // Action to set the authentication status
    setUser(state, action) {
      state.user = action.payload;
    },
  },

  // Special reducer for hydrating the state. Special case for next-redux-wrapper
  extraReducers: {
    [HYDRATE]: (state, action) => {
      return {
        ...state,
        ...action.payload.auth,
      };
    },
  },
});

export const { setUser } = authSlice.actions;

export const selectAuthState = (state: AppState): AuthState => state.auth;

export default authSlice.reducer;
