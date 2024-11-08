export async function postData(url: string, data: unknown) {
  return fetch(`${url}`, {
    method: "POST",
    headers: {
      "Content-Type": "application/json",
    },
    body: JSON.stringify(data),
  })
    .then((response) => response.json())
    .then((res) => {
      if (res.error_msg) {
        throw new Error(res.error_msg);
      }
      return res;
    });
}

export async function getData(url: string) {
  return fetch(url)
    .then((response) => response.json())
    .then((res) => {
      if (res.error_msg) {
        throw new Error(res.error_msg);
      }
      return res;
    });
}

export async function deleteData(url: string) {
  return fetch(url, { method: "DELETE" })
    .then((response) => response.json())
    .then((res) => {
      if (res.error_msg) {
        throw new Error(res.error_msg);
      }
      return res;
    });
}

export function getCookie(name: string) {
  const value = `; ${document.cookie}`;
  const parts = value.split(`; ${name}=`);
  if (parts.length === 2) return parts.pop().split(";").shift();
}
