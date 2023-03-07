export function bytesToString(bytes: string): string {
  const outputNumber = bytes.substring(2).split('').map(x => parseInt(x as unknown as string, 16))

  const length = outputNumber.length
  let result = ''
  for (let i = 0; i < length; i += 2) {
    result += String.fromCharCode(outputNumber[i] * 16 + outputNumber[i + 1])
  }

  return result
}
