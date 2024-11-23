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

export async function postFormData(url: string, data: FormData) {
  return fetch(`${url}`, {
    method: "POST",
    body: data,
  })
    .then((response) => response.json())
    .then((res) => {
      if (res.error_msg) {
        throw new Error(res.error_msg);
      }
      return res;
    });
}

export async function putData(url: string, data: unknown) {
  return fetch(`${url}`, {
    method: "PUT",
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

export async function getData(url: string, fetchOptions?: RequestInit) {
  return fetch(url, fetchOptions)
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
  if (parts.length === 2) return parts.pop()?.split(";").shift();
}

export function reorder<T>(list: T[], startIndex: number, endIndex: number) {
  const _reorderForward = (list: T[], startIndex: number, endIndex: number) => {
    const tmp = list[startIndex];
    for (let i = startIndex; i < endIndex; i++) {
      list[i] = list[i + 1];
    }
    list[endIndex - 1] = tmp;
    return list;
  };
  const _reorderBackward = (
    list: T[],
    startIndex: number,
    endIndex: number,
  ) => {
    for (let i = startIndex; i > endIndex; i--) {
      list[i] = list[i - 1];
    }
    return list;
  };

  if (startIndex < endIndex) _reorderForward(list, startIndex, endIndex);
  else if (startIndex > endIndex) _reorderBackward(list, startIndex, endIndex);
  return list;
}
