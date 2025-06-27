"use client";
import { isImage } from "@/utils/utils";

interface Props {
  id: string,
}
export default function FileFullSize({ id }: Props) {
  return <section className="w-full h-full">
    {/* {id} */}
    {isImage(id) ?
      <img src={`/u/${id}`} alt="" className="w-full h-full object-contain" />
      : <a href={`/u/${id}`} target="_blank" rel="noopener noreferrer">Open in new tab</a>
    }
  </section>;
}
