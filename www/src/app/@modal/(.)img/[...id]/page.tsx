import FileFullSize from "@/components/ImageFullSize";
import Modal from "@/components/Modal";
import { use } from "react";

type PageProps = Promise<{ id: Array<string> }>;

export default function Page(props: { params: PageProps }) {
  const { id } = use(props.params);
  return <Modal>
    <FileFullSize id={id.join("/")} />
  </Modal>;
}
