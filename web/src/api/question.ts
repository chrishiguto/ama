import { axios } from "./axios";

export const createQuestion = async (roomId: string, question: string) => {
  const response = await axios.post("/question", {
    room_id: roomId,
    value: question,
  });
  return response.data;
};

export const getQuestions = async (roomId: string) => {
  const response = await axios.get(`/room/${roomId}/questions`);
  return response.data;
};

export const reactQuestion = async (questionId: string) => {
  const response = await axios.patch(`/question/${questionId}/react`);
  return response.data;
};
