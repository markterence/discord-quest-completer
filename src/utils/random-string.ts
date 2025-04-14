import { nanoid } from 'nanoid'

export const randomString = (length: number = 16): string => {
    return nanoid(length)
}