import { ArrowUp } from "lucide-react";
import { reactQuestion } from "../../../api/question";
import Link from "../../../components/Link";
import { Question } from "../../../types";

type QuestionProps = {
  question: Question;
};

const QuestionCard = ({ question }: QuestionProps) => {
  const handleReact = async () => {
    try {
      await reactQuestion(question.id);
    } catch (err) {
      console.log(err);
    }
  };

  return (
    <div className="text-zinc-100 leading-relaxed">
      <span>{question.value}</span>
      <div></div>
      <Link asChild>
        <button className="mt-3" onClick={handleReact}>
          <ArrowUp size={16} />
          <span>Like question ({question.reaction_count})</span>
        </button>
      </Link>
    </div>
  );
};

export default QuestionCard;
