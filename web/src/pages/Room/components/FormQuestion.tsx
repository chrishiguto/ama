import { AxiosError } from "axios";
import { ArrowRight } from "lucide-react";
import { useRef, useState } from "react";
import { toast } from "sonner";
import { createQuestion } from "../../../api/question";
import Button from "../../../components/Button";
import TextInput from "../../../components/TextInput";

type FormQuestionProps = {
  roomId: string;
};

const FormQuestion = ({ roomId }: FormQuestionProps) => {
  const inputRef = useRef<HTMLInputElement | null>(null);

  const [question, setQuestion] = useState("");
  const [loading, setLoading] = useState(false);
  const [questionFieldError, setQuestionFieldError] = useState<boolean>(false);

  const handleCreateQuestion = async (event: React.FormEvent) => {
    event.preventDefault();

    if (!question) {
      setQuestionFieldError(true);
      inputRef.current?.focus();
      return;
    }

    try {
      setLoading(true);
      setQuestionFieldError(false);

      await createQuestion(roomId, question);
      setQuestion("");

      toast(`You've successfully created a question!`);
    } catch (err: unknown) {
      console.log(err);

      if (err instanceof AxiosError) {
        toast.error(err.response?.data.error ?? err.message);
        return;
      }

      toast.error("An unexpected error happened.");
    } finally {
      setLoading(false);
    }
  };

  return (
    <form className="w-full" onSubmit={handleCreateQuestion}>
      <TextInput
        ref={inputRef}
        placeholder="What's your question?"
        value={question}
        onChange={(e) => setQuestion(e.target.value)}
        error={questionFieldError}
      >
        <Button
          type="submit"
          loading={loading}
          endIcon={<ArrowRight size={16} />}
        >
          Create question
        </Button>
      </TextInput>
    </form>
  );
};

export default FormQuestion;
