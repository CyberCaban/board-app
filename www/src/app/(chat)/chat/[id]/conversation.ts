import { getData, postData } from "@/utils/utils";

const findConversation = (member_one: string, member_two: string) => {
  return postData(`/chat_source/conversation/${member_one}/${member_two}`);
};

const getLastMessages = (conversation_id: string) => {
  return getData(`/chat_source/last_messages/${conversation_id}`);
};

export { findConversation, getLastMessages };
