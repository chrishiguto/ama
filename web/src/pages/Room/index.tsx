import { Share2 } from "lucide-react";
import { useCallback, useEffect, useState } from "react";
import { useParams, Link as RRLink } from "react-router";

import { getRoom } from "../../api/room";
import Button from "../../components/Button";
import Container from "../../components/Container";
import Logo from "../../components/Logo";
import FormQuestion from "./components/FormQuestion";
import ListQuestions from "./components/ListQuestions";
import ContentLoader from "react-content-loader";
import Link from "../../components/Link";
import { AxiosError } from "axios";
import { toast } from "sonner";

const Room = () => {
  const params = useParams();

  const [loading, setLoading] = useState(true);
  const [error, setError] = useState<null | string>(null);

  const loadRoom = useCallback(async () => {
    if (!params?.id) return;

    try {
      setLoading(true);

      await getRoom(params.id);
    } catch (err: unknown) {
      if (err instanceof AxiosError) {
        setError(err.response?.data.error ?? err.message);
        return;
      }

      setError("An unexpected error happened.");
    } finally {
      setLoading(false);
    }
  }, [params.id]);

  const handleShareRoom = async () => {
    await navigator.clipboard.writeText(window.location.href);
    toast.success(
      "Room link copied to clipboard! (I know, I know... not the best UX, but hey, simplicity wins for now! 😜)"
    );
  };

  useEffect(() => {
    loadRoom();
  }, [loadRoom]);

  return (
    <Container size="xlarge">
      <div className="flex flex-col self-stretch w-full">
        <header className="flex flex-col sm:flex-row gap-y-6 items-center border-b border-zinc-900 justify-between pt-4 sm:pt-10 pb-4 sm:pb-6">
          <section className="flex items-center gap-x-3">
            <RRLink to="/">
              <Logo size={26} />
            </RRLink>
            <h3 className="text-sm">
              <span className="text-zinc-500">Room code:</span>{" "}
              {!loading ? params.id : ""}
            </h3>

            {loading && (
              <ContentLoader
                speed={2}
                width={300}
                backgroundColor="#ababab"
                foregroundColor="#fafafa"
                viewBox="0 0 300 30"
              >
                <rect y="9" rx="3" ry="3" width="300" height="12" />
              </ContentLoader>
            )}
          </section>

          <Button
            className="self-end sm:self-auto"
            variant="zinc"
            endIcon={<Share2 size={16} />}
            onClick={handleShareRoom}
          >
            Share
          </Button>
        </header>

        {error && (
          <div className="text-center">
            <p className="text-zinc-300 mt-6 mb-2">{error}</p>
            <Link asChild>
              <RRLink to="/">Go home</RRLink>
            </Link>
          </div>
        )}

        <main className="flex flex-1 flex-col py-6">
          {!error && !loading && (
            <>
              <FormQuestion roomId={params.id!} />

              <section className="py-6">
                <ListQuestions roomId={params.id} />
              </section>
            </>
          )}
        </main>
      </div>
    </Container>
  );
};

export default Room;
