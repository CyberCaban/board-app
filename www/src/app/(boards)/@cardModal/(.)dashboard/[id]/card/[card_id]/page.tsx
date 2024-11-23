import EditCard from "@/app/(boards)/_components/EditCard";
import Modal from "@/components/Modal";
import { use } from "react";

type Params = Promise<{ id: string; card_id: string }>;

export default function CardPage(props: { params: Params }) {
  const { id, card_id } = use(props.params);

  return (
    <Modal>
      <EditCard board_id={id} card_id={card_id} />
    </Modal>
  );
}
