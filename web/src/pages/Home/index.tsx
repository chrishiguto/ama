import { AxiosError } from "axios";
import { ArrowRight } from "lucide-react";
import { useRef, useState } from "react";
import { useNavigate } from "react-router";
import { toast } from "sonner";
import { createRoom } from "../../api/room";
import Button from "../../components/Button";
import Container from "../../components/Container";
import Logo from "../../components/Logo";
import TextInput from "../../components/TextInput";

const Home = () => {
  const navigate = useNavigate();
  const inputRef = useRef<HTMLInputElement | null>(null);

  const [roomName, setRoomName] = useState("");
  const [roomNameFieldError, setRoomNameFieldError] = useState<string | null>(
    null
  );
  const [loading, setLoading] = useState(false);

  const handleCreateRoom = async (event: React.FormEvent) => {
    event.preventDefault();

    if (!roomName) {
      setRoomNameFieldError("Room name is required.");
      inputRef.current?.focus();
      return;
    }

    try {
      setLoading(true);
      setRoomNameFieldError(null);
      const room = await createRoom(roomName);

      navigate(`/room/${room.id}`);
      toast("Success! Your room has been created! 🎉");
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
    <Container size="medium">
      <div className="flex flex-col items-center gap-y-6 text-center">
        <Logo />
        <p>
          Create a public AMA (Ask Me Anything) room and prioritize the most
          important questions for the community.
        </p>

        <form className="w-full" onSubmit={handleCreateRoom}>
          <TextInput
            ref={inputRef}
            placeholder="Room name"
            value={roomName}
            onChange={(e) => setRoomName(e.target.value)}
            error={!!roomNameFieldError}
            errorText={roomNameFieldError}
          >
            <Button
              type="submit"
              loading={loading}
              endIcon={<ArrowRight size={16} />}
            >
              Create room
            </Button>
          </TextInput>
        </form>
      </div>
    </Container>
  );
};

export default Home;
