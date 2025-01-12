import { axios } from "./axios";

export const getRoom = async (id: string) => {
  const response = await axios.get(`/room/${id}`);
  return response.data;
};

export const createRoom = async (name: string) => {
  const response = await axios.post("/room", { name });
  return response.data;
};
