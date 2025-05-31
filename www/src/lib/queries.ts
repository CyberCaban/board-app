import { postFormData } from "@/utils/utils";

export function sentFileToBackend(
  file: File,
  filename: string,
  isPrivate: boolean,
  onSuccess: (refName: string) => void,
  onFailure: (err: unknown) => void,
) {
  const formData = new FormData();
  formData.append("file", file);
  formData.append("filename", filename);
  formData.append("is_private", isPrivate.toString());
  postFormData("/api/file/create", formData).then(onSuccess).catch(onFailure);
}
