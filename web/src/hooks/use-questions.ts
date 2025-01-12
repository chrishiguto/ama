import { useRef, useCallback, useState, useEffect } from "react";
import { toast } from "sonner";
import { Question } from "../types";

type Message = {
  kind: "Create" | "Update";
  data: Question;
};

const useQuestions = (roomId: string | undefined) => {
  const socketRef = useRef<WebSocket | null>(null);

  const [questions, setQuestions] = useState<Question[] | null>(null);

  const connect = () => {
    if (socketRef.current && socketRef.current.readyState === WebSocket.OPEN) {
      return socketRef.current;
    }

    const socket = new WebSocket(
      `${import.meta.env.VITE_WS_BASE_URL}/room/subscribe/${roomId}`
    );

    socketRef.current = socket;

    socket.onerror = (error) => {
      toast.error("Room connection error: " + error);
    };

    socket.onclose = () => {
      socketRef.current = null;
    };

    socket.onmessage = (rawMessage) => {
      try {
        const message: Message = JSON.parse(rawMessage.data);

        switch (message.kind) {
          case "Create": {
            setQuestions((prevState) =>
              prevState?.length ? [...prevState, message.data] : [message.data]
            );
            break;
          }

          case "Update": {
            setQuestions((prevState) => {
              if (!prevState) {
                return [message.data];
              }

              const questionIndex = prevState.findIndex(
                (question) => question.id === message.data.id
              );

              if (questionIndex < 0) {
                return prevState;
              }

              const newState = [...prevState];
              newState[questionIndex] = message.data;

              return newState;
            });

            break;
          }

          default:
            break;
        }
      } catch (error) {
        console.error(" error: " + error);
      }
    };

    return socket;
  };

  const disconnect = useCallback(() => {
    if (socketRef.current) {
      socketRef.current.close();
      socketRef.current = null;
    }
  }, []);

  useEffect(() => {
    connect();

    return () => disconnect();
  }, []);

  return {
    connect,
    disconnect,
    questions,
    setQuestions,
    socket: socketRef.current,
  };
};

export default useQuestions;
