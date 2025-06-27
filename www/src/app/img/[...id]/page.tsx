import FileFullSize from "@/components/ImageFullSize";
import { use } from "react";

type PageProps = Promise<{ id: Array<string> }>;

export default function Page(props: { params: PageProps }) {
  const { id } = use(props.params);
  return <section>
    <FileFullSize id={id.join("/")} />
  </section>;
}
