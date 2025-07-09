export function generateSearchRegex (text: string) {
  return text && new RegExp(text.trim().replace(/\s+/g, '.{0,5}'), 'i')
}