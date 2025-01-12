import { AxiosError } from "axios";
import { useCallback, useEffect, useState } from "react";
import ContentLoader from "react-content-loader";
import { toast } from "sonner";
import { getQuestions } from "../../../api/question";
import useQuestions from "../../../hooks/use-questions";
import QuestionCard from "./Question";

export const ListQuestionsLoader = () => (
  <ContentLoader
    speed={2}
    width={330}
    backgroundColor="#3b3b3b"
    foregroundColor="#5e5e5e"
    viewBox="0 0 330 48"
  >
    <rect rx="3" ry="3" width="330" height="16" />
    <rect y="32" rx="3" ry="3" width="150" height="16" />
  </ContentLoader>
);

type ListQuestionsProps = {
  roomId: string | undefined;
};

const ListQuestions = ({ roomId }: ListQuestionsProps) => {
  const { connect, disconnect, questions, setQuestions } = useQuestions(roomId);

  const [loading, setLoading] = useState(true);

  const loadQuestions = useCallback(async () => {
    if (!roomId) return;

    try {
      const questions = await getQuestions(roomId);
      setQuestions(questions);
    } catch (err) {
      toast.error(
        (err as AxiosError)?.response?.data?.error ?? (err as AxiosError)
      );
    } finally {
      setLoading(false);
    }
  }, [roomId, setQuestions]);

  useEffect(() => {
    loadQuestions();
  }, [loadQuestions, disconnect]);

  if (loading) {
    return <ListQuestionsLoader />;
  }

  return (
    <ol className="list-decimal pl-6 space-y-8">
      {questions?.length
        ? questions?.map((question) => (
            <li key={question.id}>
              <QuestionCard question={question} />
            </li>
          ))
        : !loading &&
          !questions?.length && (
            <p className="text-zinc-100">No questions yet!</p>
          )}
    </ol>
  );
};

export default ListQuestions;
